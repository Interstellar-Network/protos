// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GarblerInputs {
    #[prost(enumeration="GarblerInputsType", tag="1")]
    pub r#type: i32,
    /// length in BITS
    /// eg a typical "diplay circuits" with 2 digits will have length = 2 * 7 = 14
    /// and the pinpad length = 10 * 7 = 70
    #[prost(uint32, tag="2")]
    pub length: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluatorInputs {
    #[prost(enumeration="EvaluatorInputsType", tag="1")]
    pub r#type: i32,
    /// length in BITS
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
/// <https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.retain_enum_prefix>
/// NOTE: careful if using prefix else rust(clippy) will complain:
/// "error: all variants have the same prefix: `GarblerInputs`"
/// But we MUST use a prefix else PROTOC will not compile:
/// "skcd/skcd.proto:26:3: "BUF" is already defined in "interstellarpbskcd".
/// skcd/skcd.proto:26:3: Note that enum values use C++ scoping rules, meaning that enum values are siblings of their type, not children of it.
/// Therefore, "BUF" must be unique within "interstellarpbskcd", not just within "GarblerInputsType"."
/// option retain_enum_prefix = false;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GarblerInputsType {
    /// MUST be set to 0!
    Buf = 0,
    /// Part of 7 segments display; so 7 bits
    SevenSegments = 1,
    /// Part of the watermark; typically width*height nb pixels
    Watermark = 2,
}
impl GarblerInputsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GarblerInputsType::Buf => "GARBLER_INPUTS_TYPE_BUF",
            GarblerInputsType::SevenSegments => "GARBLER_INPUTS_TYPE_SEVEN_SEGMENTS",
            GarblerInputsType::Watermark => "GARBLER_INPUTS_TYPE_WATERMARK",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvaluatorInputsType {
    /// The "display circuit" standard input type: SHOULD be randomized during each eval loop
    Rnd = 0,
    /// The "generic circuit" standard input type: SHOULD be choosen by the evaluator
    /// eg for the adder circuit
    ChoosenByEvaluator = 1,
    /// Same as previous, but for the garbler
    ChoosenByGarbler = 2,
}
impl EvaluatorInputsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EvaluatorInputsType::Rnd => "EVALUATOR_INPUTS_TYPE_RND",
            EvaluatorInputsType::ChoosenByEvaluator => "EVALUATOR_INPUTS_TYPE_CHOOSEN_BY_EVALUATOR",
            EvaluatorInputsType::ChoosenByGarbler => "EVALUATOR_INPUTS_TYPE_CHOOSEN_BY_GARBLER",
        }
    }
}
// @@protoc_insertion_point(module)
