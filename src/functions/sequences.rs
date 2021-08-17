////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn reverse_sequence(sequence: &str) -> String {
  sequence
    .chars()
    .map(|nucleotide| {
      match nucleotide {
        '!' => '?',
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => nucleotide,
      }
    })
    .rev()
    .collect()
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Sequence {
  ///
  /// Obtain reverse complement sequence.
  ///
  /// # Examples
  ///
  /// ```
  /// use genomic_structures::reverse_sequence;
  ///
  /// let to_reverse = "GATTACA";
  /// let reversed = "TGTAATC".to_string();
  ///
  /// assert_eq!(reverse_sequence(to_reverse), reversed);
  /// ```
  fn reverse_sequence(&self) -> String {
    reverse_sequence(self.get_sequence())
  }

  fn get_sequence(&self) -> &str;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
