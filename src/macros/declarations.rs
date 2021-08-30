////////////////////////////////////////////////////////////////////////////////////////////////////

/// Load features onto mobile element chimeric pair.
#[macro_export]
macro_rules! load {
  // mobile element on hashmap
  ( mobile element |> $record: expr; $values: expr; $read_no: tt ) => {
    // record data on primary alignment
    if $values.flag <= 255 {
      $record.$read_no.sequence = $values.sequence.clone();
    }

    // record mobile element data
    $record.$read_no.me_read.push(MEAnchor::load(
      $values.cigar.clone(),
      $values.flag,
      $values.scaffold.clone(),
      $values.orientation.clone(),
      $values.position,
      $values.get_extra(),
    ));

    // calculate break point
    $record
      .$read_no
      .me_read
      .iter_mut()
      .last()
      .unwrap()
      .calculate_break_point(&$record.$read_no.sequence.clone());
  };

  // chromosomal loci
  ( chromosomal |> $record: expr; $values: expr; $read_no: tt ) => {
    // check whether read id & sequence coincide
    if $record.$read_no.sequence == $values.sequence ||
      $record.$read_no.reverse_sequence() == $values.sequence
    {
      // assign chromosomal anchor mapping quality
      $record.$read_no.quality = $values.quality;

      // load chromosomal anchor
      $record.$read_no.chr_read.push(ChrAnchor::load(
        $values.cigar.clone(),
        $values.scaffold.clone(),
        $values.flag,
        $values.quality,
        $values.position,
        $values.tlen,
      ))
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// // structural variant
// ( $record: expr, $read_no: tt, $flines: expr ) => {
//   $record.$read_no.sequence = $flines[9].to_string();
//   $record.$read_no.chr_read = ChrAnchor::load(&$flines);
//   // TODO: update ChrAnchor::loader
// };

////////////////////////////////////////////////////////////////////////////////////////////////////

///
#[macro_export]
macro_rules! anchor_count {
  ( $me_chimeric_read: expr, $anchor: tt ) => {
    $me_chimeric_read
      .me_read
      .iter()
      .filter(|me_anchor| me_anchor.orientation == OrientationEnum::$anchor)
      .count()
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: write test for macros
