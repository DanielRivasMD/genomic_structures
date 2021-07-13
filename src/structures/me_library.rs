////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::erv_annotations::ERVAnnotations;

////////////////////////////////////////////////////////////////////////////////////////////////////

// to load onto => hashmap for mobile elements library entries
/// Construc and collect ERV library.
#[derive(Debug, new, Default)]
pub struct MELibrary {
  /// Mobile element sequence.
  #[new(default)]
  pub me_seq: String,

  /// Mobile element size.
  #[new(default)]
  pub me_size: i32,

  /// ERV annotations.
  #[new(default)]
  pub annotations_erv: ERVAnnotations,
  // TODO: potentially expandable to other types of mobile elements
}

////////////////////////////////////////////////////////////////////////////////////////////////////
