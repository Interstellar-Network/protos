// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarbleIpfsRequest {
    #[prost(string, tag="1")]
    pub skcd_cid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarbleIpfsReply {
    #[prost(string, tag="1")]
    pub pgarbled_cid: ::prost::alloc::string::String,
}
/// For now it only serves to pass around the OTP/digits/permutation set during "strip+packmsg"
/// It SHOULD NOT be sent to the client, it SHOULD be kept server-side!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitServerMetadata {
    #[prost(bytes="vec", tag="1")]
    pub digits: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarbleAndStripIpfsRequest {
    #[prost(string, tag="1")]
    pub skcd_cid: ::prost::alloc::string::String,
    /// NOTE: for now it does the 2 steps "strip" + "packmsg" in the same route but that is a bit pointless
    /// The goal is to have a 2 steps process: garble+strip(store in DB/etc), then later packmsg using strip result from DB
    #[prost(string, tag="2")]
    pub tx_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub server_metadata: ::core::option::Option<CircuitServerMetadata>,
}
/// For now we have an all-in-route that garbles then strips the circuits
/// so we return both a Packmsg and a stripped PGC.
/// This avoids storing a PGC somewhere then pulling it for garbling and restoring it before
/// sending it to the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarbleAndStripIpfsReply {
    #[prost(string, tag="1")]
    pub pgarbled_cid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub packmsg_cid: ::prost::alloc::string::String,
}
/// Using an independent top-level well_knowns.proto does not work...
/// <https://github.com/stepancheg/rust-protobuf/issues/409>
/// same as google.protobuf.Empty, but directly included to avoid well-knowns
/// [well-knowns are a pain to use with Android]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
// @@protoc_insertion_point(module)
