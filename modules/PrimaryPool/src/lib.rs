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
use substreams::{log, store, store::DeltaProto, Hex, errors::Error};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

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

// Yakshesh:: added return type because map_ func should return Result
#[substreams::handlers::map]
fn map_subscriptions(
    blk: eth::Block,
    pool_created: Pools,
) -> Result<verified::primary::v1::Subscriptions, substreams::errors::Error> {
    log::info!("Detecting subscriptions from Primary pools");
    
    let mut all_subscriptions = Vec::new();
    // let blk:eth::Block;
    for pool in pool_created.pools {
        log::info!("{:?}",Hex(&pool.pool_address)); 
        let subscriptions_for_pool: Vec<_> = blk
        .events::<abi::pi::v1::Subscription>(&[&pool.pool_address])
        // .events::<abi::pool::events::Subscription>(&[&pool.pool_address])
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

// Ok(verified::primary::v1::Subscription {
//     asset_in_address: all_subscriptions[0].asset_in_address.clone(),
//     asset_out_address: all_subscriptions[0].asset_out_address.clone(),
//     subscription_amount: all_subscriptions[0].subscription_amount,
//     investor_address: all_subscriptions[0].investor_address.clone(),
//     price: all_subscriptions[0].price,
//     execution_date: all_subscriptions[0].execution_date,
// })
