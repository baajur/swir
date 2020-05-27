rm client_api_pb2*.py
virtualenv build
. build/bin/activate
python -m pip install --upgrade pip
python -m pip install grpcio grpcio-tools
python -m grpc_tools.protoc -I../../../grpc_api --python_out=. --grpc_python_out=. ../../../grpc_api/common_structs.proto
python -m grpc_tools.protoc -I../../../grpc_api --python_out=. --grpc_python_out=. ../../../grpc_api/client_api.proto

#rm -rf build
