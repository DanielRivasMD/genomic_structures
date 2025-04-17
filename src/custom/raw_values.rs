////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  error::common_error::CommonError,
  functions::flag_interpretor::SAMFlag,
  structures::{
    cigar::CIGAR,
    extra_values_enum::ExtraValuesEnum,
    me_anchor::TagME,
    orientation_enum::OrientationEnum,
    read_control::ReadControl,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: raw parser straigth from record line
// TODO: think about a way to use raw values only with reference to strings
// TODO: write an error catcher with better messages for parsing
// values ordered as SAM format
/// Structural representation of SAM file records.
#[derive(Debug, new, Default, PartialEq)]
pub struct RawValues {
  /// Read ID with memory.
  #[new(default)]
  pub read_id: ReadControl,

  // Alignement flag.
  #[new(default)]
  pub flag: i32,

  /// Scaffold alignment allocation.
  #[new(default)]
  pub scaffold: String,

  /// Alignment position.
  #[new(default)]
  pub position: i32,

  /// Mapping quality (MAPQ).
  #[new(default)]
  pub quality: i32,

  /// CIGAR string.
  #[new(value = "CIGAR::new()")]
  pub cigar: CIGAR,

  /// Template length (TLEN).
  #[new(default)]
  pub tlen: i32,

  /// Read sequence.
  #[new(default)]
  pub sequence: String,

  // TODO: expand to other annotations?
  /// Orientation annotation.
  #[new(default)]
  pub orientation: OrientationEnum,

  /// Additional annotations.
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

  /// Update records.
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
  /// Retrieve additional annotations.
  pub fn get_extra(&self) -> f64 {
    match self.extra {
      ExtraValuesEnum::MobelSize(value) => value,
      // zero value on mobile element size is indicative of not registered
      // further logic invalidates not registered values => mobile element tag
      ExtraValuesEnum::None => 0.,
    }
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

// implement tag mobile element
impl TagME for RawValues {
  // tag mobile element orientation as upstream
  fn upstream(&mut self) {
    self.orientation = OrientationEnum::Upstream;
  }

  // tag mobile element orientation as downstream
  fn downstream(&mut self) {
    self.orientation = OrientationEnum::Downstream;
  }

  // read orientation
  // assign true when read is aligned on
  // reversed strand in relation to assembly
  // otherwise false
  fn read_orientation(&self) -> bool {
    self.interpret(5)
  }

  // reset
  fn reset_orientation(&mut self) {
    self.orientation = OrientationEnum::None;
  }

  // get size
  fn get_size(&self) -> f64 {
    self.get_extra()
  }

  // cigar left boundry
  fn get_cigar_left_boundry(&self) -> i32 {
    self.cigar.left_boundry
  }

  // cigar right boundry
  fn get_cigar_rigth_boundry(&self) -> i32 {
    self.cigar.right_boundry
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
