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
pub struct Trades {
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(bytes="vec", tag="1")]
    pub security_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub order_type: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub price: u64,
    #[prost(bytes="vec", tag="4")]
    pub currency_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="5")]
    pub traded_amount: u64,
    #[prost(uint64, tag="6")]
    pub execution_date: u64,
}
// @@protoc_insertion_point(module)
