# Simple web app that connects to KV sink

This app uses Connect-Web to create a gRPC connection to badgerdb where kv sink dumps data.

## Start Connect-Web server to launch API defined in protobuf at ../proto

```export DYLD_LIBRARY_PATH=$LIBRARY_PATH substreams-sink-kv serve "badger3://$(pwd)/badger_data.db" substreams.yaml --listen-addr=":8080"```

## Generate the connectivity code

```npm run buf:generate```

## Run the app

```npm run dev```