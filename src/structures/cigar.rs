////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contain read's CIGAR information.
#[derive(Debug, new)]
pub struct CIGAR {
  /// Left clip.
  #[new(default)]
  pub lclip: i32,

  /// Alignment.
  #[new(default)]
  pub align: Vec<i32>,

  /// Right clip.
  #[new(default)]
  pub rclip: i32,

  /// Insertion.
  #[new(default)]
  pub ins: Vec<i32>,

  /// Deletion.
  #[new(default)]
  pub del: Vec<i32>,

  /// Left boundry.
  #[new(default)]
  pub left_boundry: i32,

  /// Right boundry.
  #[new(default)]
  pub right_boundry: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl CIGAR {
  // TODO: update documentation
  /// Load string into CIGAR struct.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::CIGAR;
  ///
  /// let cigar = CIGAR::loader("10H1I2M2D80M5H");
  /// assert_eq!(cigar.align, vec![2, 80]);
  /// assert_eq!(cigar.del, vec![2]);
  /// assert_eq!(cigar.ins, vec![1]);
  /// assert_eq!(cigar.lclip, 10);
  /// assert_eq!(cigar.rclip, 5);
  /// ```
  pub fn loader(&mut self, to_interpret: &str, position: i32) {
    if to_interpret == "*" {
      self.align.push(0);
    } else {
      // iterate to find annotations
      let mut char_vec = vec![];
      for i in to_interpret.char_indices() {
        match i.1 {
          'H' | 'S' | 'M' | 'D' | 'I' => {
            char_vec.push(i.0);
          }
          _ => (),
        }
      }

      let mut j = 0;

      for i in char_vec.iter() {
        match &to_interpret[*i..*i + 1] {
          "H" | "S" => {
            if self.align.iter().sum::<i32>() == 0 {
              self.lclip = (&to_interpret[j..*i]).parse::<i32>().unwrap();
            } else {
              self.rclip = (&to_interpret[j..*i]).parse::<i32>().unwrap();
            }
          }
          "M" => {
            self
              .align
              .push((&to_interpret[j..*i]).parse::<i32>().unwrap());
          }
          "I" => {
            self
              .ins
              .push((&to_interpret[j..*i]).parse::<i32>().unwrap());
          }
          "D" => {
            self
              .del
              .push((&to_interpret[j..*i]).parse::<i32>().unwrap());
          }
          _ => {}
        }
        j = i + 1;
      }
    }

    // calculate boundries
    self.boundries(position);

  }

  // TODO: verify & rewrite boundries

  /// Define left and right boundries.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::CIGAR;
  ///
  /// let mut cigar = CIGAR::loader("10H84M6H");
  /// assert_eq!((90, 190), cigar.boundries(100));
  /// ```
  fn boundries(
    &mut self,
    position: i32,
  ) {
    self.left_boundry = self.left_boundry_calculator(position);
    self.right_boundry = self.right_boundry_calculator(position);
  }

  // adjust alignment coordinates according to CIGAR interpretation
  // return tupple ( adjusted position, total aligned length )
  fn adjuster(
    &self,
    position: i32,
  ) -> (i32, i32) {
    let align_sum: i32 = self.align.iter().sum();
    let ins_sum: i32 = self.ins.iter().sum();
    let del_sum: i32 = self.del.iter().sum();
    (self.lclip + position, align_sum + ins_sum + del_sum)
  }

  // left boundry
  fn left_boundry_calculator(
    &self,
    position: i32,
  ) -> i32 {
    position - self.lclip
  }

  // right boundry
  fn right_boundry_calculator(
    &self,
    position: i32,
  ) -> i32 {
    let lpos = self.left_boundry_calculator(position);
    lpos + (self.lclip + self.adjuster(position).1 + self.rclip)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
