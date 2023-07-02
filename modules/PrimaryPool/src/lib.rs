mod abi;
mod pb;
#[path = "kv_out.rs"]
mod kv;

use hex_literal::hex;
use pb::verified::{Pool, Pools, Subscription, Subscriptions};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_sink_kv::pb::kv::KvOperations;

const FACTORY_CONTRACT: [u8; 20] = hex!("0x4823be69546f9e1Ab8a87f315108c19dDC8E48b4");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_pools(blk: eth::Block) -> Result<Pools, substreams::errors::Error> {
    Ok(Pools {
        pools: blk
            .events::<abi::factory::events::PoolCreated>(&[&FACTORY_CONTRACT])
            .map(|(pool_created, _log)| {
                log::info!("PoolCreated event seen");

                Pool {
                    pool_address: Hex(pool_created.pool).to_string(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn map_subscriptions(pool_created:Pools) {
    log::info!("Detecting subscriptions from Primary pools");
    for Pool in Pools.Pool {
        Ok(Subscriptions {
            subscriptions: blk
                .events::<abi::pool::events::Subscriptions>(&[&Pool])
                .map(|(order_created, _log)| {
                    log::info!("Subscriptions event seen");
    
                    Subscription {
                        assetIn_address: order_created.assetIn,
                        assetOut_address: order_created.assetOut,
                        subscription_amount: order_created.subscription.low_u64(),
                        investor_address: order_created.subscription,
                        price: order_created.price.low_u64(),
                        execution_date: order_created.executionDate,
                    }
                })
                .collect(),
        })
    }
}

#[substreams::handlers::map]
pub fn kv_out(
    order_created: Subscriptions,
    deltas: store::Deltas<DeltaProto<BlockMeta>>,
) -> Result<KvOperations, Error> {

    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();

    // Call a function that will push key-value operations from the deltas
    kv::process_deltas(&mut kv_ops, deltas);

    // Here, we could add more operations to the kv_ops
    kv_ops.push_new(assetIn, Subscription.assetIn_address);
    kv_ops.push_new(assetOut, Subscription.assetOut_address);
    kv_ops.push_new(subscription, Subscription.subscription_amount);
    kv_ops.push_new(investor, Subscription.investor_address);
    kv_ops.push_new(price, Subscription.price);
    kv_ops.push_new(executionDate, Subscription.execution_date);

    Ok(kv_ops)
}