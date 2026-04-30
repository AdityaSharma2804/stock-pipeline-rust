//! Generated Protobuf stock message from prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
/// Stock data record with symbol, price, and timestamp.
pub struct Stock {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(double, tag = "2")]
    pub price: f64,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
