mod abi;
mod pb;
// #[path = "kv_out.rs"]
// mod kv;

// use std::error::Error;
use hex_literal::hex;
use pb::{
    block_meta::primary::v1::BlockMeta,
    verified::{
        self,
        primary::v1::{Pool, Pools, Subscription, Subscriptions},
        // primary::v1::{Pool, Pools, Subscription, Subscriptions},
    },
};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use substreams::{errors::Error, log, store, store::DeltaProto, Hex};
use substreams_ethereum::pb::eth::v2 as eth;
// use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

// const FACTORY_CONTRACT: [u8; 20] = hex!("2081d59917ee6b58d92a19174c158354359187bc");
const FACTORY_CONTRACT: [u8; 20] = hex!("4823be69546f9e1Ab8a87f315108c19dDC8E48b4");
substreams_ethereum::init!();
#[substreams::handlers::map]
fn map_pools(blk: eth::Block) -> Result<Pools, substreams::errors::Error> {
    log::info!("Detecting Pools from Primary pools");

    Ok(Pools {
        pools: blk
            .events::<abi::factory::events::PoolCreated>(&[&FACTORY_CONTRACT])
            .map(|(pool_created, _log)| {
                log::info!("PoolCreated event seen");

                Pool {
                    pool_address: (pool_created.pool),
                    // pool_address: Hex(pool_created.pool).to_string(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn map_subscriptionsCheck(blk: eth::Block) -> Result<Subscriptions, substreams::errors::Error> {
    log::info!("Detecting Subscriptions from Primary pools");

    Ok(Subscriptions {
        subscriptions: blk
            .events::<abi::pool::events::Subscription>(&[&FACTORY_CONTRACT])
            .map(|(subscription, _log)| {
                log::info!("Subscription event seen");

                Subscription {
                    // Map the fields from the detected event to the Subscription struct
                    // For example:
                    asset_in_address: subscription.asset_in,
                    asset_out_address: subscription.asset_out,
                    subscription_amount: subscription.subscription.to_u64(),
                    investor_address: subscription.investor,
                    price: subscription.price.to_u64(),
                    execution_date: subscription.execution_date.to_u64(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn map_subscriptions(
    blk: eth::Block,
    pool_created: Pools,
) -> Result<verified::primary::v1::Subscriptions, substreams::errors::Error> {
    log::info!("Detecting subscriptions from Primary pools");
    log::info!("{:?}", pool_created);
    let mut all_subscriptions = Vec::new();
    // let blk:eth::Block;
    for pool in pool_created.pools {
        log::info!("{:?}", Hex(&pool.pool_address));
        let subscriptions_for_pool: Vec<_> = blk
            // .events::<abi::pi::v1::Subscription>(&[&pool.pool_address])
            .events::<abi::pool::events::Subscription>(&[&pool.pool_address])
            .map(|(order_created, _log)| {
                log::info!("Subscriptions event seen");
                Subscription {
                    asset_in_address: order_created.asset_in,
                    asset_out_address: order_created.asset_out,
                    subscription_amount: order_created.subscription.to_u64(),
                    investor_address: order_created.investor,
                    price: order_created.price.to_u64(),
                    execution_date: order_created.execution_date.to_u64(),
                }
            })
            .collect();

        all_subscriptions.extend(subscriptions_for_pool);
    }

    Ok(verified::primary::v1::Subscriptions {
        subscriptions: all_subscriptions,
    })
}

#[substreams::handlers::map]
pub fn kv_out(order_created: Subscriptions) -> Result<KvOperations, Error> {
    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();

    // Here, we could add more operations to the kv_ops
    // kv_ops.push_new(assetIn, Subscription.assetIn_address);
    // kv_ops.push_new(assetOut, Subscription.assetOut_address);
    // kv_ops.push_new(subscription, Subscription.subscription_amount);
    // kv_ops.push_new(investor, Subscription.investor_address);
    // kv_ops.push_new(price, Subscription.price);
    // kv_ops.push_new(executionDate, Subscription.execution_date);

    Ok(kv_ops)
}
