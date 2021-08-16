////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  error::common_error::CommonError,
  functions::sam_flag::SAMFlag,
  structures::{
    cigar::CIGAR,
    extra_values_enum::ExtraValuesEnum,
    orientation_enum::OrientationEnum,
    read_control::ReadControl,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: raw parser straigth from record line
// TODO: add test
// TODO: think about a way to use raw values only with reference to strings
// TODO: update field documentation
// values ordered as SAM format
#[derive(Debug, new, Clone, Default)]
pub struct RawValues {
  #[new(default)]
  pub read_id: ReadControl,

  #[new(default)]
  pub flag: i32,

  #[new(default)]
  pub scaffold: String,

  #[new(default)]
  pub position: i32,

  #[new(default)]
  pub quality: i32,

  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  #[new(default)]
  pub tlen: i32,

  #[new(default)]
  pub sequence: String,

  // TODO: expand to other annotations?
  #[new(default)]
  pub orientation: OrientationEnum,

  #[new(default)]
  pub extra: ExtraValuesEnum,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
impl RawValues {
  ///
  ///   - Load records contained in SAM file for processing on mobile element
  ///     alignment:
  ///     - Read ID.
  ///     - Alignment flag and interprets orientation.
  ///     - Mobile element aligned.
  ///     - CIGAR calculating alignment coordinates and boundries.
  ///   - Load mobile element features.
  ///   - Load structural variant features.
  pub fn load(flines: Vec<&str>) -> anyResult<Self> {
    // create empty struct
    let mut raw_values = RawValues::new();

    // update values
    raw_values.update(flines)?;

    return Ok(raw_values);
  }

  pub fn update(
    &mut self,
    flines: Vec<&str>,
  ) -> anyResult<()> {
    // read id
    self.read_id.current = flines[0].to_string();

    // flag & read orientation
    self.flag = flines[1].parse::<i32>().context(CommonError::Parsing)?;

    // scaffold
    self.scaffold = flines[2].to_string();

    // position
    self.position = flines[3].parse::<i32>().context(CommonError::Parsing)?;

    //  quality
    self.quality = flines[4].parse::<i32>().context(CommonError::Parsing)?;

    // cigar
    self.cigar = CIGAR::load(&flines[5].to_string(), self.position)?;

    // flines[6]

    // flines[7]

    // alignment length
    self.tlen = flines[8].parse::<i32>().context(CommonError::Parsing)?;

    // sequence
    self.sequence = flines[9].to_string();

    // flines[10]

    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// get & reset
impl RawValues {
  pub fn get_extra(&self) -> f64 {
    match self.extra {
      ExtraValuesEnum::MobelSize(value) => value,
      ExtraValuesEnum::None => {
        println!("No annotation to retrive");
        0.
      }
    }
  }

  pub fn get_orientation(&self) -> String {
    match self.orientation {
      // redesigned the mobile element chimeric read to accept enum
      OrientationEnum::Downstream => String::from("downstream"),
      OrientationEnum::Upstream => String::from("upstream"),
      OrientationEnum::None => String::new(),
    }
  }

  pub fn reset_orientation(&mut self) {
    self.orientation = OrientationEnum::None;
  }

  pub fn get_read_orientation(&self) -> bool {
    self.interpret(5)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// implement SAM flag
impl SAMFlag for RawValues {
  fn get_flag(&self) -> i32 {
    self.flag
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
