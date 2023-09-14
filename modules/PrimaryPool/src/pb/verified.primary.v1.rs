// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(bytes="vec", tag="1")]
    pub pool_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscriptions {
    #[prost(message, repeated, tag="1")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    #[prost(bytes="vec", tag="1")]
    pub asset_in_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub asset_out_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub subscription_amount: u64,
    #[prost(bytes="vec", tag="4")]
    pub investor_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="5")]
    pub price: u64,
    #[prost(uint64, tag="6")]
    pub execution_date: u64,
}
// @@protoc_insertion_point(module)
