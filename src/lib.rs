////////////////////////////////////////////////////////////////////////////////////////////////////

//! This crate provides a
//! collection of structures, types and functions for mobile elment
//! and structural variant identification from genomic data.
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

// functions
mod functions;

pub use crate::functions::{
  counter::strand_count,
  flag_interpretor::interpret,
  identificator::identify,
  thresholder::threshold,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// structures
mod structures;

pub use crate::structures::{
  anchor_enum::AnchorEnum,
  break_point::BreakPoint,
  chr_anchor::ChrAnchor,
  chr_anchor_enum::ChrAnchorEnum,
  cigar::CIGAR,
  erv_annotations::ERVAnnotations,
  extra_values_enum::ExtraValuesEnum,
  me_anchor::MEAnchor,
  me_chimeric_pair::MEChimericPair,
  me_chimeric_read::MEChimericRead,
  me_library::MELibrary,
  orientation_enum::OrientationEnum,
  raw_values::RawValues,
  read_control::ReadControl,
  sv_chimeric_pair::SVChimericPair,
  sv_chimeric_read::SVChimericRead,
  sv_type::SVType,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: handle constants
pub const BIN_OVERLAP: i32 = 50;
pub const BIN_SIZE: i32 = 100;
pub const TRANSLOCATION_DISTANCE: i32 = 1000;

////////////////////////////////////////////////////////////////////////////////////////////////////
