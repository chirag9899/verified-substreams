mod abi;
mod pb;
#[path = "kv_out.rs"]
mod kv;

use hex_literal::hex;
use pb::verified::{
        self,
        primary::v1::{Pool, Pools, Subscription, Subscriptions},
    };
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use substreams::{errors::Error, log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;

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
pub fn kv_out(order_created: Subscription) -> Result<KvOperations, Error> {
    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();

    // Here, we could add more operations to the kv_ops
    kv_ops.push_new("assetInAddress", order_created.asset_in_address,1);
    kv_ops.push_new("assetOutAddress", order_created.asset_out_address,2);
    kv_ops.push_new("subscriptionAmount", order_created.subscription_amount.to_be_bytes(),3);
    kv_ops.push_new("investorAddress", order_created.investor_address,4);
    kv_ops.push_new("price", order_created.price.to_be_bytes(),5);
    kv_ops.push_new("executionDate", order_created.execution_date.to_be_bytes(),6);

    Ok(kv_ops)
}
