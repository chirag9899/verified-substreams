# Required only on MacOS to properly instruct the 'substreams-sink-kv' where to find the WasmEdge library
export DYLD_LIBRARY_PATH=$LIBRARY_PATH

substreams-sink-kv inject mainnet.eth.streamingfast.io:443 "badger3://$(pwd)/badger_data.db" substreams.yaml