use std::collections::HashMap;
use std::sync::Arc;

use futures::channel::oneshot;
use futures::lock::Mutex;
use futures::stream::StreamExt;
use http::HeaderValue;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::HttpConnector;
use hyper::{header, Body, Client, HeaderMap, Method, Request, Response, StatusCode};
use tokio::sync::mpsc;


#[derive(Debug)]
enum PersistenceOperationType{
    Store,
    Retrieve
    
}

use crate::utils::structs::{BackendStatusCodes, ClientSubscribeRequest, CustomerInterfaceType, Job, MessagingResult, PublishRequest, RestToMessagingContext,RestToPersistenceContext,StoreRequest, RetrieveRequest, PersistenceJobType,PersistenceResult};
use crate::utils::structs::{MessagingToRestContext, SubscribeRequest};

static X_CORRRELATION_ID_HEADER_NAME:& str = "x-correlation-id";
static X_DATABASE_NAME_HEADER_NAME:& str = "x-database-name";
static X_DATABASE_KEY_HEADER_NAME:& str = "x-database-key";
static X_TOPIC_HEADER_NAME:& str = "topic";


fn extract_value_from_headers(header_name:String, headers: &HeaderMap<HeaderValue>) -> Option<String> {
    let header = header::HeaderName::from_lowercase(header_name.as_bytes()).unwrap();
    let maybe_header = headers.get(header);
    if let Some(value) = maybe_header {
        Some(String::from_utf8_lossy(value.as_bytes()).to_string())
    } else {
        None
    }
}

fn extract_topic_from_headers(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    extract_value_from_headers(String::from(X_TOPIC_HEADER_NAME),headers)
}

fn extract_correlation_id_from_headers(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    extract_value_from_headers(String::from(X_CORRRELATION_ID_HEADER_NAME),headers)
}

fn extract_database_name_from_headers(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    extract_value_from_headers(String::from(X_DATABASE_NAME_HEADER_NAME),headers)
}

fn extract_database_key_from_headers(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    extract_value_from_headers(String::from(X_DATABASE_KEY_HEADER_NAME),headers)
}


fn find_channel_by_topic<'a>(
    client_topic: &'a str,
    from_client_to_backend_channel_sender: &'a HashMap<String, mpsc::Sender<RestToMessagingContext>>,
) -> Option<&'a mpsc::Sender<RestToMessagingContext>> {
    from_client_to_backend_channel_sender.get(client_topic)
}

fn find_channel_by_database_name<'a>(
    database_name: &'a str,
    from_client_to_persistence_sender: &'a HashMap<String, mpsc::Sender<RestToPersistenceContext>>,
) -> Option<&'a mpsc::Sender<RestToPersistenceContext>> {
    from_client_to_persistence_sender.get(database_name)
}

fn find_to_client_sender<'a>(client_topic: &'a str,to_client_sender: &'a HashMap<String, mpsc::Sender<MessagingToRestContext>>)->Option<&'a mpsc::Sender<MessagingToRestContext>>{
    to_client_sender.get(client_topic)
	
}

fn validate_content_type(headers: &HeaderMap<HeaderValue>) -> Option<bool> {
    match headers.get(http::header::CONTENT_TYPE) {
        Some(header) => {
            if header == HeaderValue::from_static("application/json") {
		debug!{"Found header {:?}",header}
                Some(true)
            } else {
                None
            }
        }
        None => None,
    }
}
    

fn set_http_response(backend_status: BackendStatusCodes, response: &mut Response<Body>) {
    match backend_status {
        BackendStatusCodes::Ok(msg) => {
            *response.status_mut() = StatusCode::OK;
            *response.body_mut() = Body::from(msg);
        }
        BackendStatusCodes::Error(msg) => {
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *response.body_mut() = Body::from(msg);
        }
        BackendStatusCodes::NoTopic(msg) => {
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *response.body_mut() = Body::from(msg);
        }
    }
}

async fn get_whole_body(mut req: Request<Body>) -> Vec<u8> {
    let mut whole_body = Vec::new();
    while let Some(maybe_chunk) = req.body_mut().next().await {
        if let Ok(chunk) = &maybe_chunk {
            whole_body.extend_from_slice(chunk);
        }
    }
    whole_body
}

async fn sub_unsubscribe_handler(is_subscribe: bool, whole_body: Vec<u8>,correlation_id:String, from_client_to_backend_channel_sender: &HashMap<String, mpsc::Sender<RestToMessagingContext>>, to_client_sender_for_rest:&HashMap<String, mpsc::Sender<MessagingToRestContext>>)->Response<Body>{
    let wb = String::from_utf8_lossy(&whole_body);
    if is_subscribe{
	info!("Subscribe {} ",wb);
    }else{
	info!("Unsubscribe {} ",wb);
    }
    let maybe_json = serde_json::from_slice(&whole_body);
    match maybe_json {
        Ok(json) => {
	    sub_unsubscribe_processor(is_subscribe, json,correlation_id,&from_client_to_backend_channel_sender,&to_client_sender_for_rest).await
	},
	Err(e) => {
            warn!("Unable to parse body {:?}", e);
	    let mut response = Response::new(Body::from(e.to_string()));
            *response.status_mut() = StatusCode::BAD_REQUEST;
	    response
        }
    }
    
}
    

async fn store_retrieve_processor(op_type:PersistenceOperationType, correlation_id: String,  headers:&HeaderMap<HeaderValue>,  req: Request<Body> , from_client_to_persistence_sender: &HashMap<String, mpsc::Sender<RestToPersistenceContext>>)->Response<Body>{
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
    let database_name = extract_database_name_from_headers(headers).unwrap();
    let key = extract_database_key_from_headers(headers).unwrap();
    let whole_body = get_whole_body(req).await;

    info!("{:?} start {} {}", op_type, database_name, key);
    let maybe_channel = find_channel_by_database_name(&database_name, from_client_to_persistence_sender);
    let mut sender = if let Some(channel) = maybe_channel {
        channel.clone()
    } else {
        set_http_response(BackendStatusCodes::NoTopic("No mapping for this topic".to_string()), &mut response);
        return response;
    };       
    
    let (local_tx, local_rx): (oneshot::Sender<PersistenceResult>, oneshot::Receiver<PersistenceResult>) = oneshot::channel();
    
    
    let res = match op_type{
	PersistenceOperationType::Store=>{
	    let sr = StoreRequest {
		correlation_id,
		payload: whole_body,
		key:key.clone()
	    };
    
	    let job = RestToPersistenceContext {
		job: PersistenceJobType::Store(sr.clone()),
		sender: local_tx,
	    };
	    sender.try_send(job)
	},
	PersistenceOperationType::Retrieve=>{
	    let rr = RetrieveRequest {
		correlation_id,
		key:key.clone()
	    };
    
	    let job = RestToPersistenceContext {
		job: PersistenceJobType::Retrieve(rr.clone()),
		sender: local_tx,
	    };
	    sender.try_send(job)
	}
    };

    if let Err(e) = res{
        warn!("Channel is dead {:?}", e);
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        *response.body_mut() = Body::empty();
    }else{    
	debug!("Waiting for store");
	let response_from_store: Result<PersistenceResult, oneshot::Canceled> = local_rx.await;
	debug!("Got result {:?}", response_from_store);
	if let Ok(res) = response_from_store {
            set_http_response(res.status, &mut response);
	    if !res.payload.is_empty() {
		*response.body_mut() = Body::from(res.payload);
	    }
	} else {
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *response.body_mut() = Body::empty();
	}
	info!("{:?} end {} {}", op_type, database_name, key);
    }
    response
}

async fn sub_unsubscribe_processor(is_subscribe: bool, csr: ClientSubscribeRequest,correlation_id:String, from_client_to_backend_channel_sender: &HashMap<String, mpsc::Sender<RestToMessagingContext>>, to_client_sender_for_rest:&HashMap<String, mpsc::Sender<MessagingToRestContext>>)->Response<Body>{

    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
    
    let json: ClientSubscribeRequest = csr;
    let (local_tx, local_rx): (oneshot::Sender<MessagingResult>, oneshot::Receiver<MessagingResult>) = oneshot::channel();
    let client_topic =  json.client_topic.clone();
    
    if let Some(to_client_sender) = find_to_client_sender(&client_topic,&to_client_sender_for_rest){
	let endpoint = json.endpoint.clone();			
	let sb = SubscribeRequest {
	    correlation_id,
            client_interface_type: CustomerInterfaceType::REST,
            client_topic: json.client_topic.clone(),
            endpoint,
	    tx:Box::new(to_client_sender.clone())
	};
	
	let maybe_channel = find_channel_by_topic(&sb.client_topic, &from_client_to_backend_channel_sender);
	
	let mut sender = if let Some(channel) = maybe_channel {
            channel.clone()
	} else {
            set_http_response(BackendStatusCodes::NoTopic("No channel for this topic".to_string()), &mut response);
	    return response;
	};
	let job = if is_subscribe {
	    RestToMessagingContext {
		job: Job::Subscribe(sb),
		sender: local_tx,
	    }
	}else{
	    RestToMessagingContext {
		job: Job::Unsubscribe(sb),
		sender: local_tx,
	    }
	};
	
	debug!("Waiting for broker");
	if let Err(e) = sender.try_send(job) {
            warn!("Channel is dead {:?}", e);
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *response.body_mut() = Body::empty();

	}

	let response_from_broker: Result<MessagingResult, oneshot::Canceled> = local_rx.await;
	debug!("Got result {:?}", response_from_broker);
	if let Ok(res) = response_from_broker {
            set_http_response(res.status, &mut response);
	} else {
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *response.body_mut() = Body::empty();
	}	
    }else{
	set_http_response(BackendStatusCodes::NoTopic("No mapping for this topic".to_string()), &mut response);			
    }
    response
}

pub async fn handler(req: Request<Body>, from_client_to_backend_channel_sender: HashMap<String, mpsc::Sender<RestToMessagingContext>>,to_client_sender_for_rest:HashMap<String, mpsc::Sender<MessagingToRestContext>>,from_client_to_persistence_sender:HashMap<String, mpsc::Sender<RestToPersistenceContext>>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::NOT_ACCEPTABLE;

    let headers = req.headers().clone();
    debug!("Headers {:?}", headers);

    let correlation_id = if let Some(correlation_id) = extract_correlation_id_from_headers(&headers){
	correlation_id
    }else{
	*response.status_mut() = StatusCode::BAD_REQUEST;
        *response.body_mut() = Body::empty();
        return Ok(response);
    };
	
    debug!("Correlation id {}", correlation_id);
    
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/publish") => {
            let whole_body = get_whole_body(req).await;
            let wb = whole_body.clone();
            let wb = String::from_utf8_lossy(&wb);
            info!("Publish start {}", wb);
            let client_topic = extract_topic_from_headers(&headers).unwrap();
            let maybe_channel = find_channel_by_topic(&client_topic, &from_client_to_backend_channel_sender);
            let mut sender = if let Some(channel) = maybe_channel {
                channel.clone()
            } else {
                set_http_response(BackendStatusCodes::NoTopic("No mapping for this topic".to_string()), &mut response);
                return Ok(response);
            };

            let p = PublishRequest {
		correlation_id,
                payload: whole_body,
                client_topic,
            };

            let (local_tx, local_rx): (oneshot::Sender<MessagingResult>, oneshot::Receiver<MessagingResult>) = oneshot::channel();
            let job = RestToMessagingContext {
                job: Job::Publish(p),
                sender: local_tx,
            };

            if let Err(e) = sender.try_send(job) {
                warn!("Channel is dead {:?}", e);
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *response.body_mut() = Body::empty();
            }

            debug!("Waiting for broker");
            let response_from_broker: Result<MessagingResult, oneshot::Canceled> = local_rx.await;
            debug!("Got result {:?}", response_from_broker);
            if let Ok(res) = response_from_broker {
                set_http_response(res.status, &mut response);
            } else {
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *response.body_mut() = Body::empty();
            }
            info!("Publish end {}", wb);
            Ok(response)
        }

        (&Method::POST, "/subscribe") => {
	    if validate_content_type(&headers).is_none() {
                return Ok(response);
            }	 
            let whole_body = get_whole_body(req).await;
	    response = sub_unsubscribe_handler(true, whole_body,correlation_id,&from_client_to_backend_channel_sender,&to_client_sender_for_rest).await;
	    Ok(response)
	},
	
	(&Method::POST, "/unsubscribe") => {
	    if validate_content_type(&headers).is_none() {
                return Ok(response);
            }	 
            let whole_body = get_whole_body(req).await;
	    response = sub_unsubscribe_handler(false, whole_body,correlation_id,&from_client_to_backend_channel_sender,&to_client_sender_for_rest).await;
	    Ok(response)
	}

	(&Method::POST, "/store") => {	  
	    let response = store_retrieve_processor(PersistenceOperationType::Store, correlation_id, &headers, req, &from_client_to_persistence_sender).await;
            Ok(response)
	},
	
	(&Method::POST, "/retrieve") => {
	    let response = store_retrieve_processor(PersistenceOperationType::Retrieve, correlation_id, &headers, req, &from_client_to_persistence_sender).await;
            return Ok(response);

	},
// The 404 Not Found route...
	_ => {
	    warn!("Don't know what to do {} {}",&req.method(), &req.uri());
	    let mut not_found = Response::default();
	    *not_found.status_mut() = StatusCode::NOT_FOUND;
	    Ok(not_found)
	}
    }
}

async fn send_request(client: Client<HttpConnector<GaiResolver>>, payload: MessagingToRestContext) {
    let uri = payload.uri;
    //TODO: this will drump stack trace. probably just an error. otherwise validate url in http_handler
    let uri = uri.parse::<hyper::Uri>().unwrap();

    let p = payload.payload.clone();
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header(hyper::header::CONTENT_TYPE, HeaderValue::from_static("application/octet-stream"))
//	.header(hyper::header::HeaderName::from_lowercase(X_CORRRELATION_ID_HEADER_NAME.as_bytes()), HeaderValue::from_string(payload."application/octet-stream"))
        .body(Body::from(payload.payload))
        .expect("request builder");

    //    let sender = payload.sender.clone();
    //    let err_sender = payload.sender.clone();

    let p = String::from_utf8_lossy(&p);
    info!("Making request for {}", p);
    let resp = client.request(req).await;
    let _res = resp
        .map(move |res| {
            info!("Status POST to the client: {} {} ", res.status(), p);
            //                let mut status = "All good".to_string();
            if res.status() != hyper::StatusCode::OK {
                warn!("Error from the client {}", res.status());
                //                    status = "Invalid response from the client".to_string();
            }
            //                if let Err(e) = sender.send(MessagingResult { status: u32::from(res.status().as_u16()), result: status }) {
            //                    warn!("Problem with an internal communication {:?}", e);
            //                }
            //                res.into_body().concat2()
            res.into_body()
        })
        .map(|_| {})
        .map_err(move |err| {
            eprintln!("Error {}", err);
            //                    if let Err(e) = err_sender.send(MessagingResult { status: 1, result: "Something is wrong".to_string() }) {
            //                        warn!("Problem with an internal communication {:?}", e);
            //                    }
        });
}

pub async fn client_handler(rx: Arc<Mutex<mpsc::Receiver<MessagingToRestContext>>>) {
    let client = hyper::Client::builder().build_http();
    info!("Client done");
    let mut rx = rx.lock().await;
    while let Some(payload) = rx.next().await {
        let client = client.clone();
        tokio::spawn(async move { send_request(client, payload).await });
    }
}
