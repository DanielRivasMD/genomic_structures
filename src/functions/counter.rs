////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::{
  chr_anchor::ChrAnchor,
  me_anchor::MEAnchor,
  orientation_enum::OrientationEnum,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Count reads per strand and orientation.
pub fn strand_count(
  read_id: String,
  str: &str,
  mut read_count: i32,
  chr_pair: &ChrAnchor,
  me_pair: &[MEAnchor],
  position_hm: &mut HashMap<String, Vec<String>>,
) -> i32 {
  let mut mobel_counter = ElementCount::new();

  for i in me_pair.iter() {
    mobel_counter.count(&i.orientation);
  }

  match str {
    "F5" => {
      if chr_pair.flag == 0 &&
        mobel_counter.upstream >= mobel_counter.downstream
      {
        chr_count!(read_id, read_count, chr_pair, position_hm);
      }
    }
    "F3" => {
      if chr_pair.flag == 16 &&
        mobel_counter.upstream <= mobel_counter.downstream
      {
        chr_count!(read_id, read_count, chr_pair, position_hm);
      }
    }
    "R5" => {
      if chr_pair.flag == 16 &&
        mobel_counter.upstream >= mobel_counter.downstream
      {
        chr_count!(read_id, read_count, chr_pair, position_hm);
      }
    }
    "R3" => {
      if chr_pair.flag == 0 &&
        mobel_counter.upstream <= mobel_counter.downstream
      {
        chr_count!(read_id, read_count, chr_pair, position_hm);
      }
    }
    // TODO: add non-compatible count?
    _ => {}
  }
  read_count
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// hold count upstream & downstream
#[derive(Debug, new)]
struct ElementCount {
  #[new(default)]
  upstream:   i32,
  #[new(default)]
  downstream: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// count on element
impl ElementCount {
  fn count(
    &mut self,
    orientation: &OrientationEnum,
  ) {
    match orientation {
      OrientationEnum::Upstream => self.upstream += 1,
      OrientationEnum::Downstream => self.downstream += 1,
      OrientationEnum::None => (),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// count reads at bin
fn chr_count(
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
