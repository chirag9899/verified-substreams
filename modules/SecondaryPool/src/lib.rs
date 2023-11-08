mod abi;
mod pb;

use hex_literal::hex;
use pb::verified::secondary::v1::{Pool, Pools, Trade};
use substreams::{
    errors::Error,
    log,
    // pb::substreams::module::input::store,
    store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto},
    Hex,
};
use substreams_ethereum::{pb::eth::v2 as eth, Event};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

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

#[substreams::handlers::store]
pub fn store_pools_created(pools: Pools, store: StoreSetProto<Pool>) {
    let mut ordinal = 0;
    for pool in pools.pools {
        ordinal = ordinal + 1;
        let pool_address = &pool.pool_address;
        store.set(ordinal, &Hex::encode(pool_address), &pool);
    }
}

#[substreams::handlers::map]
fn map_trades(blk: eth::Block, pools_store: StoreGetProto<Pool>) -> Result<Trade, Error> {
    log::info!("Detecting subscriptions from Secondry module");
    let mut pool_events = Trade::default();
    for trx in blk.transactions() {
        for (log, call_view) in trx.logs_with_calls() {
            let pool_address = &Hex(&log.address).to_string();

            let pool_opt = pools_store.get_last(&pool_address);
            if pool_opt.is_none() {
                continue;
            }
            if let Some(trade) = abi::pool::events::TradeReport::match_and_decode(log) {
                log::info!("TradeReport event seen");
                pool_events.security_address = trade.security;
                pool_events.order_ref = trade.order_ref.to_vec();
                pool_events.party = trade.party;
                pool_events.counterparty = trade.counterparty;
                pool_events.order_type = trade.order_type.to_vec();
                pool_events.price = trade.price.to_u64();
                pool_events.currency_address = trade.currency;
                pool_events.traded_amount = trade.amount.to_u64();
                pool_events.execution_date = trade.execution_date.to_u64();
            }
            // use the pool information from the store
        }
    }

    Ok(pool_events)
}

// #[substreams::handlers::map]
// fn map_trades(
//     blk: eth::Block,
//     pool_created: Pools,
// ) -> Result<verified::secondary::v1::Trades, substreams::errors::Error> {
//     log::info!("Detecting trades from Secondary pools");
//     let mut all_trades = Vec::new();
//     for pool in pool_created.pools {
//         let trades_for_pool: Vec<_> = blk
//             .events::<abi::pool::events::TradeReport>(&[&pool.pool_address])
//             .map(|(trade_reported, _log)| {
//                 log::info!("TradeReport event seen");

//                 Trade {
//                     security_address: trade_reported.security,
//                     order_ref:trade_reported.order_ref.to_vec(),
//                     party:trade_reported.party,
//                     counterparty:trade_reported.counterparty,
//                     order_type: trade_reported.order_type.to_vec(),
//                     price: trade_reported.price.to_u64(),
//                     currency_address: trade_reported.currency,
//                     traded_amount: trade_reported.amount.to_u64(),
//                     execution_date: trade_reported.execution_date.to_u64(),
//                 }
//             })
//             .collect();
//         all_trades.extend(trades_for_pool);
//     }
//     Ok(verified::secondary::v1::Trades { trades: all_trades })
// }

// #[substreams::handlers::map]
pub fn kv_out(trade_reported: Trade) -> Result<KvOperations, Error> {
    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();

    // Here, we could add more operations to the kv_ops
    kv_ops.push_new("security", trade_reported.security_address, 1);
    kv_ops.push_new("order_ref", trade_reported.order_ref, 2);
    kv_ops.push_new("party", trade_reported.party, 3);
    kv_ops.push_new("counterparty", trade_reported.counterparty, 4);
    kv_ops.push_new("order_type", trade_reported.order_type, 5);
    kv_ops.push_new("price", trade_reported.price.to_be_bytes(), 6);
    kv_ops.push_new("currency_address", trade_reported.currency_address, 7);
    kv_ops.push_new(
        "traded_amount",
        trade_reported.traded_amount.to_be_bytes(),
        8,
    );
    kv_ops.push_new(
        "execution_date",
        trade_reported.execution_date.to_be_bytes(),
        9,
    );

    Ok(kv_ops)
}
