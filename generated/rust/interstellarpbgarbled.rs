// @generated
/// Rougly matches <https://github.com/GaloisInc/swanky/blob/master/scuttlebutt/src/block.rs#L18>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScuttlebuttBlock {
    #[prost(sfixed64, tag="1")]
    pub high: i64,
    #[prost(sfixed64, tag="2")]
    pub low: i64,
}
/// <https://github.com/GaloisInc/swanky/blob/master/fancy-garbling/src/wire.rs#L15>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FancyGarblingWireMod2 {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<ScuttlebuttBlock>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FancyGarblingWire {
    #[prost(oneof="fancy_garbling_wire::Block", tags="1")]
    pub block: ::core::option::Option<fancy_garbling_wire::Block>,
}
/// Nested message and enum types in `FancyGarblingWire`.
pub mod fancy_garbling_wire {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Block {
        #[prost(message, tag="1")]
        Wire(super::FancyGarblingWireMod2),
    }
}
/// <https://github.com/GaloisInc/swanky/blob/master/fancy-garbling/src/classic.rs#L21>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FancyGarblingClassicGarbledCircuit {
}
/// <https://github.com/GaloisInc/swanky/blob/master/fancy-garbling/src/classic.rs#L97>
/// But WITHOUT "garbler_inputs" field b/c that is NOT needed client-side.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FancyGarblingClassicEncoder {
    #[prost(message, repeated, tag="1")]
    pub evaluator_inputs: ::prost::alloc::vec::Vec<FancyGarblingWire>,
    #[prost(btree_map="uint32, message", tag="2")]
    pub deltas: ::prost::alloc::collections::BTreeMap<u32, FancyGarblingWire>,
}
/// Mirrors: <https://github.com/Interstellar-Network/lib-garble-rs/blob/6c16289e9233007abcd18e5d3e713fda2fe9483c/src/circuit.rs>
/// DO NOT re-use "message DisplayConfig" in deps/protos/skcd/skcd.proto
/// it SHOULD be close to it but ONLY contains what the client NEEDS; DO NOT expose server-side data to the clients!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatorConfig {
    #[prost(uint32, tag="20")]
    pub width: u32,
    #[prost(uint32, tag="21")]
    pub height: u32,
}
/// This should be as close as possible from the CLIENT runtime structure
/// ie it is an EVALUABLE Garbled Circuit
/// Basically it contains:
/// - cf <https://github.com/Interstellar-Network/lib-garble-rs/blob/main/src/garble.rs> "pub struct InterstellarGarbledCircuit"
///       Thhis is essentially:
///       - a (Garbled)Circuit that mirrors Swanky/Fancy-Garbling's Circuit(=a list of Gate)
///       - the Blocks corresponding to the Garbled version of the clear text circuit
/// - what is neeeded to encode the Evaluator inputs
/// - the ENCODED Garbler inputs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterstellarGarbledCircuit {
    #[prost(message, optional, tag="1")]
    pub garbled_circuit: ::core::option::Option<FancyGarblingClassicGarbledCircuit>,
    #[prost(message, optional, tag="2")]
    pub encoder: ::core::option::Option<FancyGarblingClassicEncoder>,
    #[prost(message, optional, tag="3")]
    pub config: ::core::option::Option<EvaluatorConfig>,
}
// @@protoc_insertion_point(module)
