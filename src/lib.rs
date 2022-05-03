////////////////////////////////////////////////////////////////////////////////////////////////////

//! This crate provides a collection of structures, types and functions
//! for structural variant and mobile elment identification from genomic data.
//! # Basic usage

////////////////////////////////////////////////////////////////////////////////////////////////////

// macros
#[macro_use]
extern crate derive_new;

////////////////////////////////////////////////////////////////////////////////////////////////////

// macros
#[macro_use]
mod macros;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error
mod error;

pub use crate::error::common_error::CommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// functions
mod functions;

// functions
pub use crate::functions::{
  flag_interpretor::interpret,
  identificator::identify,
  position_binner::bin,
  sequences::reverse_sequence,
};

// traits
pub use crate::functions::{
  flag_interpretor::SAMFlag,
  position_binner::Anchor,
  sequences::Sequence,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// structs
mod structures;

// enums
pub use crate::structures::{
  anchor_enum::AnchorEnum,
  chr_anchor_enum::ChrAnchorEnum,
  extra_values_enum::ExtraValuesEnum,
  orientation_enum::OrientationEnum,
};

// structs
pub use crate::structures::{
  bin_position::BinPosition,
  break_point::BreakPoint,
  chr_anchor::ChrAnchor,
  cigar::CIGAR,
  erv_annotations::ERVAnnotations,
  me_anchor::MEAnchor,
  me_chimeric_pair::MEChimericPair,
  me_chimeric_read::MEChimericRead,
  me_library::MELibrary,
  orientation_enum::OrientationPair,
  raw_values::RawValues,
  read_control::ReadControl,
  strand_direction::StrandDirection,
  sv_chimeric_pair::SVChimericPair,
  sv_chimeric_read::SVChimericRead,
  sv_type::SVType,
};

// traits
pub use crate::structures::{
  activate::ActivateExt,
  me_anchor::TagME,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: handle constants
pub const BIN_OVERLAP: i32 = 50;
pub const BIN_SIZE: i32 = 100;
pub const TRANSLOCATION_DISTANCE: i32 = 1000;
pub const ME_LIMIT: i32 = 200;
pub const ANCHOR_LIMIT: i32 = 50;

////////////////////////////////////////////////////////////////////////////////////////////////////
