mod abi;
mod pb;

use hex_literal::hex;
use pb::uniswap::{Pool, Pools, Trade, Trades};
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;

const FACTORY_CONTRACT: [u8; 20] = hex!("0xe3e79e4106327e6eAeFBD03C1fD3A4A531c59b10");

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
    log::info!("Detecting trades from Secondary pools");
    for Pool in Pools.Pool {
        Ok(Trades {
            trades: blk
                .events::<abi::pool::events::Trades>(&[&Pool])
                .map(|(trade_reported, _log)| {
                    log::info!("TradeReport event seen");
    
                    Trade {
                        security_address: trade_reported.security,
                        order_type: trade_reported.orderType,
                        price: trade_reported.price.low_u64(),
                        currency_address: trade_reported.currency,
                        traded_amount: trade_reported.amount.low_u64(),
                        execution_date: trade_reported.executionDate,
                    }
                })
                .collect(),
        })
    }
}