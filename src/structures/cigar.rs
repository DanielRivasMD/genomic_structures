////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::error::common_error::CommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contain read's CIGAR information.
#[derive(Debug, new, Clone, Default, PartialEq)]
pub struct CIGAR {
  /// Alignment as a vector of coordinates.
  #[new(default)]
  pub align: Vec<i32>,

  /// Deletion as a vector of coordinates.
  #[new(default)]
  pub deletion: Vec<i32>,

  /// Insertion as a vector of coordinates.
  #[new(default)]
  pub insertion: Vec<i32>,

  /// Left boundry.
  #[new(default)]
  pub left_boundry: i32,

  /// Left clip position coordinate.
  #[new(default)]
  pub left_clip: i32,

  /// Right boundry.
  #[new(default)]
  pub right_boundry: i32,

  /// Right clip position coordinate.
  #[new(default)]
  pub rigth_clip: i32,

  /// String.
  #[new(default)]
  pub signature: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
impl CIGAR {
  ///
  /// Load string into CIGAR struct.
  ///
  /// # Parameters
  ///
  /// * `to_interpret` - String to parse.
  ///
  /// * `position` - 1-based coordinate.
  ///
  /// # Returns
  ///
  /// Return CIGAR object.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::CIGAR;
  ///
  /// let cigar =
  ///   CIGAR::load("10H1I2M2D80M5H", 101).expect("CIGAR loading failed!");
  /// assert_eq!(cigar, CIGAR {
  ///   align:         vec![2, 80],
  ///   deletion:      vec![2],
  ///   insertion:     vec![1],
  ///   left_boundry:  91,
  ///   left_clip:     10,
  ///   right_boundry: 190,
  ///   rigth_clip:    5,
  ///   signature:     "10H1I2M2D80M5H".to_string(),
  /// });
  /// ```
  pub fn load(
    to_interpret: &str,
    position: i32,
  ) -> anyResult<Self> {
    let mut cigar_out = Self::new();
    cigar_out.update(to_interpret, position)?;
    Ok(cigar_out)
  }

  ///
  /// Update CIGAR values.
  ///
  /// # Parameters
  ///
  /// * `to_interpret` - String to parse.
  ///
  /// * `position` - 1-based coordinate.
  ///
  /// # Returns
  ///
  /// Return CIGAR object.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::CIGAR;
  ///
  /// let mut cigar = CIGAR::new();
  /// cigar
  ///   .update("10H1I2M2D80M5H", 101)
  ///   .expect("CIGAR loading failed!");
  ///
  /// assert_eq!(cigar, CIGAR {
  ///   align:         vec![2, 80],
  ///   deletion:      vec![2],
  ///   insertion:     vec![1],
  ///   left_boundry:  91,
  ///   left_clip:     10,
  ///   right_boundry: 190,
  ///   rigth_clip:    5,
  ///   signature:     "10H1I2M2D80M5H".to_string(),
  /// });
  /// ```
  pub fn update(
    &mut self,
    to_interpret: &str,
    position: i32,
  ) -> anyResult<()> {
    self.signature = to_interpret.to_string().clone();
    // identify no CIGAR annotation
    if to_interpret == "*" {
      self.align.push(0);
    } else {
      // iterate to find annotations
      let mut char_vec = vec![];
      for ix in to_interpret.char_indices() {
        match ix.1 {
          'H' | 'S' | 'M' | 'D' | 'I' => {
            char_vec.push(ix.0);
          }
          _ => (),
        }
      }

      let mut j = 0;

      // iterate on identified characters
      for i in char_vec.iter() {
        match &to_interpret[*i..*i + 1] {
          // TODO: change soft & hard clip interpretation. perhaps add enum
          "H" | "S" => {
            // indicate whether aligned bases have been recorded
            if self.align.iter().sum::<i32>() == 0 {
              self.left_clip = (&to_interpret[j..*i])
                .parse::<i32>()
                .context(CommonError::Parsing)?;
            } else {
              self.rigth_clip = (&to_interpret[j..*i])
                .parse::<i32>()
                .context(CommonError::Parsing)?;
            }
          }
          "M" => {
            self.align.push(
              (&to_interpret[j..*i])
                .parse::<i32>()
                .context(CommonError::Parsing)?,
            );
          }
          "I" => {
            self.insertion.push(
              (&to_interpret[j..*i])
                .parse::<i32>()
                .context(CommonError::Parsing)?,
            );
          }
          "D" => {
            self.deletion.push(
              (&to_interpret[j..*i])
                .parse::<i32>()
                .context(CommonError::Parsing)?,
            );
          }
          _ => {}
        }
        j = i + 1;
      }

      // calculate boundries
      self.calculate_boundries(position);
    }
    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// calculate boundry & total alignment
// boundries are inherently tested on load & update
impl CIGAR {
  // both boundries
  fn calculate_boundries(
    &mut self,
    position: i32,
  ) {
    self.left_boundry = self.calculate_left_boundry(position);
    self.right_boundry = self.calculate_right_boundry(position);
  }

  // adjust alignment coordinates according to CIGAR interpretation
  // return tupple ( adjusted position, total aligned length )
  fn total_alignment(&self) -> i32 {
    let align_sum: i32 = self.align.iter().sum();
    let ins_sum: i32 = self.insertion.iter().sum();
    let del_sum: i32 = self.deletion.iter().sum();
    align_sum + ins_sum + del_sum
  }

  // left boundry
  fn calculate_left_boundry(
    &self,
    position: i32,
  ) -> i32 {
    position - self.left_clip
  }

  // right boundry
  fn calculate_right_boundry(
    &self,
    position: i32,
  ) -> i32 {
    let leftmost = self.calculate_left_boundry(position);
    // accomodate 1-based coordinate system
    leftmost + (self.left_clip + self.total_alignment() + self.rigth_clip) - 1
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// display trait implementation
impl fmt::Display for CIGAR {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(
      f,
      "{}\t{}\t{}\t",
      self.signature, self.left_boundry, self.right_boundry
    )
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
