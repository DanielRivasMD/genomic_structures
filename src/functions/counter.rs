////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::{
  chr_anchor::ChrAnchor,
  me_anchor::MEAnchor,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Count reads per strand and orientation.
///
/// # Examples
///
/// ```
/// TODO: add example
/// ```
pub fn strand_counter(
  read_id: String,
  str: &str,
  mut read_count: i32,
  chr_pair: &ChrAnchor,
  me_pair: &[MEAnchor],
  position_hm: &mut HashMap<String, Vec<String>>,
) -> i32 {
  let mut mobel_counter = ElementCounter::new();

  for i in me_pair.iter() {
    mobel_counter.counter(&i.orientation);
  }

  match str {
    "F5" => {
      if chr_pair.flag == 0 && mobel_counter.upstream >= mobel_counter.downstream {
        chr_counter!(read_id, read_count, chr_pair, position_hm);
      }
    }
    "F3" => {
      if chr_pair.flag == 16 && mobel_counter.upstream <= mobel_counter.downstream {
        chr_counter!(read_id, read_count, chr_pair, position_hm);
      }
    }
    "R5" => {
      if chr_pair.flag == 16 && mobel_counter.upstream >= mobel_counter.downstream {
        chr_counter!(read_id, read_count, chr_pair, position_hm);
      }
    }
    "R3" => {
      if chr_pair.flag == 0 && mobel_counter.upstream <= mobel_counter.downstream {
        chr_counter!(read_id, read_count, chr_pair, position_hm);
      }
    }
    // TODO: add non-compatible count?
    _ => {}
  }
  read_count
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// count reads at bin
fn chr_counter(
  read_id: String,
  position_hm: &mut HashMap<String, Vec<String>>,
  binned_position: String,
) {
  position_hm
    .entry(binned_position.clone())
    .or_insert_with(Vec::new);
  if let Some(id_vector) = position_hm.get_mut(&binned_position) {
    id_vector.push(read_id);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// hold count upstream & downstream
#[derive(Debug, new)]
struct ElementCounter {
  #[new(default)]
  pub upstream:   i32,
  #[new(default)]
  pub downstream: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// count on element
impl ElementCounter {
  fn counter(
    &mut self,
    orientation: &str,
  ) {
    if orientation == "upstream" {
      self.upstream += 1;
    } else if orientation == "downstream" {
      self.downstream += 1;
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
