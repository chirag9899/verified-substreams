mod abi;
mod pb;

use hex_literal::hex;
use pb::uniswap::{Pool, Pools, Subscription, Subscriptions};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;

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
fn map_subscriptions(Pools) {
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