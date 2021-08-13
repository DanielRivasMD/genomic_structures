////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! load {
  // mobile element on hashmap
  ( mobile element => $record: expr; $values: expr; $switch: expr; $read_me: tt; $read_chr: tt ) => {
    // record data on primary alignment
    if $values.flag <= 255 {
      $record.$read_me.sequence = $values.sequence.clone();
      $record.$read_me.quality = $values.quality;
    }

    // record mobile element data
    $record.$read_me.me_read.push(MEAnchor::load(
      $values.cigar.clone(),
      $values.flag,
      $values.scaffold.clone(),
      $values.orientation.clone(),
      $values.position,
      $values.get_extra(),
    ));

    if $switch.mobel {
      $record.chranch = ChrAnchorEnum::$read_chr;
    }
  };

  // chromosomal loci
  ( chromosomal => $record: expr; $values: expr; $read_no: tt ) => {
    if $record.$read_no.sequence == $values.sequence ||
      $record.$read_no.sequence_reverser() == $values.sequence
    {
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

/// Evaluate mapping quality (MAPQ).
#[macro_export]
macro_rules! mapq {
  // mapq
  ( $record: expr, $read_no: tt ) => {
    $record.$read_no.chr_read.is_empty() ||
      $record.$read_no.chr_read[0].mapq < MAPQ
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Count reads that align at a certain bin along chromosome.
#[macro_export]
macro_rules! chr_count {
  // mobile element chromosomal counter
  ( $read_id: expr, $read_count: expr, $chr_pair: expr, $position_hm: expr ) => {
    let binned_position = format!("{}", $chr_pair.bin());
    chr_count($read_id, $position_hm, binned_position);
    $read_count += 1;
  };

  // structural variant chromosomal counter
  ( $read_id: expr, $sv_pair: expr, $position_hm: expr ) => {
    let binned_pos = (
      $sv_pair.read1.chr_read[0].bin(),
      $sv_pair.read2.chr_read[0].bin(),
    );
    let binned_position = format!(
      "{}-{}",
      std::cmp::min(binned_pos.0, binned_pos.1),
      std::cmp::max(binned_pos.0, binned_pos.1)
    );
    chr_count($read_id, $position_hm, binned_position);
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: write test for macros
