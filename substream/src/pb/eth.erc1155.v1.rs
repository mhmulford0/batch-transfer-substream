// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchTransfers {
    #[prost(message, repeated, tag="1")]
    pub batch_transfers: ::prost::alloc::vec::Vec<BatchTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchTransfer {
    #[prost(string, tag="1")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="4")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="5")]
    pub values: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="6")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub ordinal: u64,
}
// @@protoc_insertion_point(module)
