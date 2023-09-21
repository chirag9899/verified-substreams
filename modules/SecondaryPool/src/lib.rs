mod abi;
mod pb;

use hex_literal::hex;
use pb::verified::secondary::v1::{Pool, Pools, Trade, Trades};
use substreams::{errors::Error, log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;
// use substreams_sink_kv::pb::kv::KvOperations;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use crate::pb::verified;
const FACTORY_CONTRACT: [u8; 20] = hex!("e3e79e4106327e6eAeFBD03C1fD3A4A531c59b10");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_pools(blk: eth::Block) -> Result<Pools, substreams::errors::Error> {
    Ok(Pools {
        pools: blk
            .events::<abi::factory::events::PoolCreated>(&[&FACTORY_CONTRACT])
            .map(|(pool_created, _log)| {
                log::info!("PoolCreated event seen");

                Pool {
                    // pool_address: Hex(pool_created.pool).to_string(),
                    pool_address: Hex(pool_created.pool).0,
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn map_trades(
    blk: eth::Block,
    pool_created: Pools,
) -> Result<verified::secondary::v1::Trades, substreams::errors::Error> {
    log::info!("Detecting trades from Secondary pools");
    let mut all_trades = Vec::new();
    for pool in pool_created.pools {
        let trades_for_pool: Vec<_> = blk
            .events::<abi::pool::events::TradeReport>(&[&pool.pool_address])
            .map(|(trade_reported, _log)| {
                log::info!("TradeReport event seen");

                Trade {
                    security_address: trade_reported.security,
                    order_type: trade_reported.order_type.to_vec(),
                    price: trade_reported.price.to_u64(),
                    currency_address: trade_reported.currency,
                    traded_amount: trade_reported.amount.to_u64(),
                    execution_date: trade_reported.execution_date.to_u64(),
                }
            })
            .collect();
        all_trades.extend(trades_for_pool);
    }
    Ok(verified::secondary::v1::Trades { trades: all_trades })
}

// #[substreams::handlers::map]
pub fn kv_out(trade_reported: Trades) -> Result<KvOperations, Error> {
    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();

    // Here, we could add more operations to the kv_ops
    // kv_ops.push_new(security, Trades.security_address);
    // kv_ops.push_new(orderType, Trades.order_type);
    // kv_ops.push_new(price, Trades.price);
    // kv_ops.push_new(currency, Trades.currency_address);
    // kv_ops.push_new(amount, Trades.traded_amount);
    // kv_ops.push_new(executionDate, Trades.execution_date);

    Ok(kv_ops)
}
