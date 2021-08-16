////////////////////////////////////////////////////////////////////////////////////////////////////

/// Determine breaking point.
#[derive(Debug, new, Default, PartialEq)]
pub struct BreakPoint {
  /// Sequence.
  #[new(default)]
  pub sequence: String,

  /// Coordinates.
  #[new(default)]
  pub coordinate: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
impl BreakPoint {
  ///
  /// Load values onto `BreakPoint`.
  ///
  /// # Parameters
  ///
  /// * `sequence` - Original read sequence.
  ///
  /// * `offset` - Mobile element estimated boundry to offset sequence.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::BreakPoint;
  ///
  /// let breakpoint = BreakPoint::load("GATTACAAAAA", 0.);
  ///
  /// assert_eq!(breakpoint, BreakPoint {
  ///   sequence:   String::from("GATTACAAAAA"),
  ///   coordinate: 1.,
  /// })
  /// ```
  pub fn load(
    sequence: &str,
    offset: f64,
  ) -> Self {
    let mut breakpoint = BreakPoint::new();
    breakpoint.update(sequence, offset);
    breakpoint
  }

  ///
  /// Update  values of `BreakPoint`.
  ///
  /// # Parameters
  ///
  /// * `sequence` - Original read sequence.
  ///
  /// * `offset` - Mobile element estimated boundry to offset sequence.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::BreakPoint;
  ///
  /// let mut breakpoint = BreakPoint::new();
  /// breakpoint.update("GATTACAAAAA", 0.);
  ///
  /// assert_eq!(breakpoint, BreakPoint {
  ///   sequence:   String::from("GATTACAAAAA"),
  ///   coordinate: 1.,
  /// })
  /// ```
  pub fn update(
    &mut self,
    sequence: &str,
    offset: f64,
  ) {
    // TODO: be aware of too large cleavage
    let cleave = 10.;
    // determine coordinate
    self.coordinate = (offset * -1.) + 1.;
    // left break point. upstream from mobile element
    if offset <= 0. {
      self.sequence =
        sequence[..(self.coordinate + cleave) as usize].to_string();
    // right break point. downstream from mobile element
    } else {
      self.sequence = sequence
        [(sequence.len() as f64 - offset - cleave) as usize..]
        .to_string();
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
