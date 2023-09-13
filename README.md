# Verified Substreams

## Introduction to Substreams

Substreams is a powerful blockchain indexing technology, developed for The Graph Network.

Substreams enables developers to write Rust modules, composing data streams alongside the community, and provides extremely high performance indexing by virtue of parallelization, in a streaming-first fashion.
Substreams has all the benefits of StreamingFast Firehose, like low-cost caching and archiving of blockchain data, high throughput processing, and cursor-based reorgs handling.

## Documentation

Full documentation for installing, running and working with Substreams is available at: https://substreams.streamingfast.io.

## Streaming orders and trade data from the Verified Network

The modules in this repo stream subscription/pricing data from primary issue pools, and trade/pricing data from secondary issue pools.

There are two modules - PrimaryPool and SecondaryPool. The data mapped by substreams is persisted in key/value stores and can be served using gRPC. More on the [client interfaces here](https://github.com/streamingfast/substreams-sink-kv/tree/develop/examples/generic-service) with an example.

The Verified substreams are for Ethereum endpoints for Mainnet, Polygon, BNB (for production) and Goerli (for testing). 

To generate rust code to connect to protobuf, run the following command in the module folder
```substreams protogen substreams.yaml --exclude-paths="sf/substreams,google"```

To build the rust code, run the following command in the module folder
```cargo build --target wasm32-unknown-unknown --release```

To package the modules, run the following command in the module folder
```substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_block --start-block 10000001 --stop-block +1```

## License

[BUSL 1.1](LICENSE)