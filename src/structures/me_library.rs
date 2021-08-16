////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::erv_annotations::ERVAnnotations;

////////////////////////////////////////////////////////////////////////////////////////////////////

// to load onto => hashmap for mobile elements library entries
/// Construc and collect endogenous retrovirus (ERV) library.
#[derive(Debug, new, Default)]
pub struct MELibrary {
  /// Mobile element sequence.
  #[new(default)]
  pub sequence: String,

  /// Mobile element size.
  #[new(default)]
  pub size: i32,

  /// ERV annotations.
  #[new(default)]
  pub annotations_erv: ERVAnnotations,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
