// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarblerInputs {
    #[prost(enumeration="GarblerInputsType", tag="1")]
    pub r#type: i32,
    #[prost(uint32, tag="2")]
    pub length: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatorInputs {
    #[prost(enumeration="EvaluatorInputsType", tag="1")]
    pub r#type: i32,
    #[prost(uint32, tag="2")]
    pub length: u32,
}
/// Only for display circuits
/// We could alternatively use a map<> for added flexibility but those fields
/// basically never change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayConfig {
    #[prost(uint32, tag="20")]
    pub width: u32,
    #[prost(uint32, tag="21")]
    pub height: u32,
    /// cf drawable::DigitSegmentsType
    #[prost(uint32, tag="22")]
    pub segments_type: u32,
}
/// IMPORTANT it MUST esentially match /lib_circuits/src/blif/skcd_config.h
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkcdConfig {
    /// INPUTS
    /// Those are splitted into GARBLER:
    #[prost(message, repeated, tag="1")]
    pub garbler_inputs: ::prost::alloc::vec::Vec<GarblerInputs>,
    /// Then evaluator
    #[prost(message, repeated, tag="2")]
    pub evaluator_inputs: ::prost::alloc::vec::Vec<EvaluatorInputs>,
    #[prost(message, optional, tag="10")]
    pub display_config: ::core::option::Option<DisplayConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkcdGate {
    #[prost(enumeration="SkcdGateType", tag="1")]
    pub r#type: i32,
    /// "a" and "b" are optional
    /// eg a INV gate only has "a", etc
    #[prost(string, tag="2")]
    pub a: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub b: ::prost::alloc::string::String,
    /// "o" SHOULD always be set
    #[prost(string, tag="4")]
    pub o: ::prost::alloc::string::String,
}
///
/// [BACKEND ONLY]
/// It is used to link using "circuit generation" and "garbling".
///
/// The fields SHOULD be about the same as the class BlifParser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Skcd {
    #[prost(message, repeated, tag="1")]
    pub gates: ::prost::alloc::vec::Vec<SkcdGate>,
    /// outputs
    #[prost(string, repeated, tag="2")]
    pub outputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// TODO NOTE: inputs(and possibly outputs too) COULD be deduced from SkcdConfig and the gates
    /// themselves, but it is easier to map them directly b/c that way we avoid
    /// having to mess converting "gate ID in .blif" with "gate ID in .skcd" and also in skcd_parser.rs
    ///
    /// inputs
    #[prost(string, repeated, tag="3")]
    pub inputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="15")]
    pub config: ::core::option::Option<SkcdConfig>,
}
/// cf gate_types.h
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SkcdGateType {
    Zero = 0,
    Nor = 1,
    Aanb = 2,
    Invb = 3,
    Naab = 4,
    Inv = 5,
    Xor = 6,
    Nand = 7,
    And = 8,
    Xnor = 9,
    Buf = 10,
    Aonb = 11,
    Bufb = 12,
    Naob = 13,
    Or = 14,
    One = 15,
}
impl SkcdGateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SkcdGateType::Zero => "ZERO",
            SkcdGateType::Nor => "NOR",
            SkcdGateType::Aanb => "AANB",
            SkcdGateType::Invb => "INVB",
            SkcdGateType::Naab => "NAAB",
            SkcdGateType::Inv => "INV",
            SkcdGateType::Xor => "XOR",
            SkcdGateType::Nand => "NAND",
            SkcdGateType::And => "AND",
            SkcdGateType::Xnor => "XNOR",
            SkcdGateType::Buf => "BUF",
            SkcdGateType::Aonb => "AONB",
            SkcdGateType::Bufb => "BUFB",
            SkcdGateType::Naob => "NAOB",
            SkcdGateType::Or => "OR",
            SkcdGateType::One => "ONE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GarblerInputsType {
    GarblerInputsBuf = 0,
    GarblerInputsSevenSegments = 1,
    GarblerInputsWatermark = 2,
}
impl GarblerInputsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GarblerInputsType::GarblerInputsBuf => "GARBLER_INPUTS_BUF",
            GarblerInputsType::GarblerInputsSevenSegments => "GARBLER_INPUTS_SEVEN_SEGMENTS",
            GarblerInputsType::GarblerInputsWatermark => "GARBLER_INPUTS_WATERMARK",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvaluatorInputsType {
    EvaluatorInputsRnd = 0,
}
impl EvaluatorInputsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EvaluatorInputsType::EvaluatorInputsRnd => "EVALUATOR_INPUTS_RND",
        }
    }
}
// @@protoc_insertion_point(module)
