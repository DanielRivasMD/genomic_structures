////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::cigar::CIGAR;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new, Clone, Default)]
pub struct RawValues {
  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  #[new(default)]
  pub scaffold: String,

  #[new(default)]
  pub flag: i32,

  #[new(default)]
  pub position: i32,

  #[new(default)]
  pub quality: i32,

  #[new(default)]
  pub read_id: String,

  #[new(default)]
  pub sequence: String,

  #[new(default)]
  pub tlen: i32,
  // TODO: expand to other annotations?
////////////////////////////////////////////////////////////////////////////////////////////////////

impl RawValues {
  // raw SAM alignment
  pub fn load(flines: Vec<&str>) -> Self
  {
    // TODO: add other fields
    // create empty struct
    let mut raw_values = RawValues::new();

    // read id
    raw_values.read_id = flines[0].to_string();

    // flag & read orientation
    raw_values.flag = flines[1].parse::<i32>().unwrap();

    // scaffold
    raw_values.scaffold = flines[2].to_string();

    // position
    raw_values.position = flines[3].parse::<i32>().unwrap();

    //  quality
    raw_values.quality = flines[4].parse::<i32>().unwrap();

    // cigar
    raw_values
      .cigar
      .update(&flines[5].to_string(), raw_values.position);

    // flines[6]

    // flines[7]

    // alignment length
    raw_values.tlen = flines[8].parse::<i32>().unwrap();

    // sequence
    raw_values.sequence = flines[9].to_string();

    // flines[10]

    return raw_values
  }

  pub fn mobel_tag(
    &self,
    // me_anchor: MEAnchor,
    me_limit: i32,
    orientation: bool
  ) -> String {
    if self.cigar.left_boundry <= me_limit && orientation {
      return String::from("upstream");
    } else if self.cigar.right_boundry as f64 <= me_limit.into() && !orientation {
      return String::from("downstream");
    } else {
      return String::new();
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
