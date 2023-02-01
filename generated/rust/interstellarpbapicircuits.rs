// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkcdDisplayRequest {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
    /// MUST match what lib_circuits's interstellar::circuits::GenerateDisplaySkcd expects
    /// It will be converted from Vec<float> -> Vec<tuple<float>> by "generate_skcd_display"
    #[prost(float, repeated, tag="3")]
    pub digits_bboxes: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkcdDisplayReply {
    #[prost(string, tag="1")]
    pub skcd_cid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkcdGenericFromIpfsRequest {
    #[prost(string, tag="1")]
    pub verilog_cid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkcdGenericFromIpfsReply {
    #[prost(string, tag="1")]
    pub skcd_cid: ::prost::alloc::string::String,
}
/// Using an independent top-level well_knowns.proto does not work...
/// <https://github.com/stepancheg/rust-protobuf/issues/409>
/// same as google.protobuf.Empty, but directly included to avoid well-knowns
/// [well-knowns are a pain to use with Android]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
// @@protoc_insertion_point(module)
