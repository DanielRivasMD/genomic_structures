////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::structures::extra_values_enum::ExtraValuesEnum;
use crate::structures::cigar::CIGAR;
use crate::structures::orientation_enum::OrientationEnum;
use crate::structures::read_control::ReadControl;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: think about a way to use raw values only with reference to strings
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
  pub read_id: ReadControl,

  #[new(default)]
  pub sequence: String,

  #[new(default)]
  pub tlen: i32,
  // TODO: expand to other annotations?

  #[new(default)]
  pub orientation: OrientationEnum,

  #[new(default)]
  pub extra: ExtraValuesEnum,

}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl RawValues {
  // raw SAM alignment
  pub fn load(flines: Vec<&str>) -> Self
  {
    // TODO: add other fields
    // create empty struct
    let mut raw_values = RawValues::new();

    // read id
    raw_values.read_id.current = flines[0].to_string();

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

  pub fn extra_get(&self) -> f64 {
    match self.extra {
      ExtraValuesEnum::MobelSize(value) => value,
      ExtraValuesEnum::None => { println!("No annotation to retrive");
        0.
      }
    }
  }

  pub fn orientation_get(&self) -> String {
    match self.orientation {
      // redesigned the mobile element chimeric read to accept enum
      OrientationEnum::Downstream => String::from("downstream"), //println!("Downstream"),
      OrientationEnum::Upstream => String::from("upstream"), // println!("Upstream"),
      OrientationEnum::None => String::new(), // print!("No values"),
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
