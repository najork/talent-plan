#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampResponse {
    #[prost(uint64, tag="1")]
    pub timestamp: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    #[prost(bool, tag="1")]
    pub is_primary: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitResponse {
}
