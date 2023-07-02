# Verified Substreams

## Introduction to Substreams

Substreams is a powerful blockchain indexing technology, developed for The Graph Network.

Substreams enables developers to write Rust modules, composing data streams alongside the community, and provides extremely high performance indexing by virtue of parallelization, in a streaming-first fashion.

Substreams has all the benefits of StreamingFast Firehose, like low-cost caching and archiving of blockchain data, high throughput processing, and cursor-based reorgs handling.

## Documentation

Full documentation for installing, running and working with Substreams is available at: https://substreams.streamingfast.io.

## Streaming orders and trade data from the Verified Network

The modules in this repo stream subscription/pricing data from primary issue pools, and trade/pricing data from secondary issue pools.

There are two modules - PrimaryPool and SecondaryPool. And the consumers are for Kafka. 

The Verified substreams are for Ethereum endpoints for Mainnet, Polygon, BNB (for production) and Goerli (for testing). 

## License

[BUSL 1.1](LICENSE)
