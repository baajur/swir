# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: client_api.proto

from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='client_api.proto',
  package='swir',
  syntax='proto3',
  serialized_options=b'\n\022rs.swir.api.clientB\022SwirClientApiProtoP\001',
  serialized_pb=b'\n\x10\x63lient_api.proto\x12\x04swir\"H\n\x0ePublishRequest\x12\x16\n\x0e\x63orrelation_id\x18\x01 \x01(\t\x12\r\n\x05topic\x18\x02 \x01(\t\x12\x0f\n\x07payload\x18\x03 \x01(\x0c\"9\n\x0fPublishResponse\x12\x16\n\x0e\x63orrelation_id\x18\x01 \x01(\t\x12\x0e\n\x06status\x18\x02 \x01(\t\"9\n\x10SubscribeRequest\x12\x16\n\x0e\x63orrelation_id\x18\x01 \x01(\t\x12\r\n\x05topic\x18\x02 \x01(\t\"<\n\x11SubscribeResponse\x12\x16\n\x0e\x63orrelation_id\x18\x01 \x01(\t\x12\x0f\n\x07payload\x18\x02 \x01(\x0c\x32\xcd\x01\n\tClientApi\x12\x38\n\x07Publish\x12\x14.swir.PublishRequest\x1a\x15.swir.PublishResponse\"\x00\x12\x44\n\x0fPublishBiStream\x12\x14.swir.PublishRequest\x1a\x15.swir.PublishResponse\"\x00(\x01\x30\x01\x12@\n\tSubscribe\x12\x16.swir.SubscribeRequest\x1a\x17.swir.SubscribeResponse\"\x00\x30\x01\x42*\n\x12rs.swir.api.clientB\x12SwirClientApiProtoP\x01\x62\x06proto3'
)




_PUBLISHREQUEST = _descriptor.Descriptor(
  name='PublishRequest',
  full_name='swir.PublishRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='correlation_id', full_name='swir.PublishRequest.correlation_id', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='topic', full_name='swir.PublishRequest.topic', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='payload', full_name='swir.PublishRequest.payload', index=2,
      number=3, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=26,
  serialized_end=98,
)


_PUBLISHRESPONSE = _descriptor.Descriptor(
  name='PublishResponse',
  full_name='swir.PublishResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='correlation_id', full_name='swir.PublishResponse.correlation_id', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='status', full_name='swir.PublishResponse.status', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=100,
  serialized_end=157,
)


_SUBSCRIBEREQUEST = _descriptor.Descriptor(
  name='SubscribeRequest',
  full_name='swir.SubscribeRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='correlation_id', full_name='swir.SubscribeRequest.correlation_id', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='topic', full_name='swir.SubscribeRequest.topic', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=159,
  serialized_end=216,
)


_SUBSCRIBERESPONSE = _descriptor.Descriptor(
  name='SubscribeResponse',
  full_name='swir.SubscribeResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='correlation_id', full_name='swir.SubscribeResponse.correlation_id', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='payload', full_name='swir.SubscribeResponse.payload', index=1,
      number=2, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=218,
  serialized_end=278,
)

DESCRIPTOR.message_types_by_name['PublishRequest'] = _PUBLISHREQUEST
DESCRIPTOR.message_types_by_name['PublishResponse'] = _PUBLISHRESPONSE
DESCRIPTOR.message_types_by_name['SubscribeRequest'] = _SUBSCRIBEREQUEST
DESCRIPTOR.message_types_by_name['SubscribeResponse'] = _SUBSCRIBERESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

PublishRequest = _reflection.GeneratedProtocolMessageType('PublishRequest', (_message.Message,), {
  'DESCRIPTOR' : _PUBLISHREQUEST,
  '__module__' : 'client_api_pb2'
  # @@protoc_insertion_point(class_scope:swir.PublishRequest)
  })
_sym_db.RegisterMessage(PublishRequest)

PublishResponse = _reflection.GeneratedProtocolMessageType('PublishResponse', (_message.Message,), {
  'DESCRIPTOR' : _PUBLISHRESPONSE,
  '__module__' : 'client_api_pb2'
  # @@protoc_insertion_point(class_scope:swir.PublishResponse)
  })
_sym_db.RegisterMessage(PublishResponse)

SubscribeRequest = _reflection.GeneratedProtocolMessageType('SubscribeRequest', (_message.Message,), {
  'DESCRIPTOR' : _SUBSCRIBEREQUEST,
  '__module__' : 'client_api_pb2'
  # @@protoc_insertion_point(class_scope:swir.SubscribeRequest)
  })
_sym_db.RegisterMessage(SubscribeRequest)

SubscribeResponse = _reflection.GeneratedProtocolMessageType('SubscribeResponse', (_message.Message,), {
  'DESCRIPTOR' : _SUBSCRIBERESPONSE,
  '__module__' : 'client_api_pb2'
  # @@protoc_insertion_point(class_scope:swir.SubscribeResponse)
  })
_sym_db.RegisterMessage(SubscribeResponse)


DESCRIPTOR._options = None

_CLIENTAPI = _descriptor.ServiceDescriptor(
  name='ClientApi',
  full_name='swir.ClientApi',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  serialized_start=281,
  serialized_end=486,
  methods=[
  _descriptor.MethodDescriptor(
    name='Publish',
    full_name='swir.ClientApi.Publish',
    index=0,
    containing_service=None,
    input_type=_PUBLISHREQUEST,
    output_type=_PUBLISHRESPONSE,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='PublishBiStream',
    full_name='swir.ClientApi.PublishBiStream',
    index=1,
    containing_service=None,
    input_type=_PUBLISHREQUEST,
    output_type=_PUBLISHRESPONSE,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='Subscribe',
    full_name='swir.ClientApi.Subscribe',
    index=2,
    containing_service=None,
    input_type=_SUBSCRIBEREQUEST,
    output_type=_SUBSCRIBERESPONSE,
    serialized_options=None,
  ),
])
_sym_db.RegisterServiceDescriptor(_CLIENTAPI)

DESCRIPTOR.services_by_name['ClientApi'] = _CLIENTAPI

# @@protoc_insertion_point(module_scope)