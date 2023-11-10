// src/kv_out.rs

use substreams::proto;
use substreams::store::{self, DeltaProto};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
// use substreams_sink_kv::pb::kv::KvOperations;
use crate::pb;
use crate::pb::verified::primary::v1::{Pool,Subscription, Subscriptions};

// use crate::pb::eth::block_meta::v1::BlockMeta;

pub fn process_deltas(ops: &mut KvOperations, deltas: store::Deltas<DeltaProto<Subscription>>) {
    use substreams::pb::substreams::store_delta::Operation;

    for delta in deltas.deltas {
        match delta.operation {
            // KV Operations do not distinguish between Create and Update.
            Operation::Create | Operation::Update => {
                let val = proto::encode(&delta.new_value).unwrap();
                ops.push_new(delta.key, val, delta.ordinal);
            }
            Operation::Delete => ops.push_delete(&delta.key, delta.ordinal),
            x => panic!("unsupported opeation {:?}", x),
        }
    }
}