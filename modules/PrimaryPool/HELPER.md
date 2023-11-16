Note: Helper function/module named *map_subscriptions_check*.
*map_subscriptions_check* used to help us log the store module of subscription event.
to run *map_subscriptions_check*
***run-*** 

substreams run -e goerli.eth.streamingfast.io:443 substreams.yaml map_subscriptions_check --start-block 9561662 --stop-block +21 --debug-modules-output=map_pools,store_pools_created,map_subscriptions,store_subscription_created,map_subscriptions_check

***Run***
substreams run -e goerli.eth.streamingfast.io:443 substreams.yaml map_subscriptions --start-block 9561662 --stop-block +21 --debug-modules-output=map_pools,store_pools_created,map_subscriptions
*for Primary module*
*And Run*


substreams run -e goerli.eth.streamingfast.io:443 substreams.yaml map_trades --start-block 9014502  --debug-modules-output=map_pools,store_pools_created,map_trades
*for Secondary Module*


------------------
Installation error in wasmEdge
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- --version 0.11.2

Link

https://github.com/streamingfast/substreams-sink-kv#wasmedge


