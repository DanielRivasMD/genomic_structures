////////////////////////////////////////////////////////////////////////////////////////////////////

/// Load records:
///   - Load records contained in SAM file for processing on mobile element
///     alignment:
///     - Read ID.
///     - Alignment flag and interprets orientation.
///     - Mobile element aligned.
///     - CIGAR calculating alignment coordinates and boundries.
///   - Load mobile element features.
///   - Load structural variant features.
#[macro_export]
macro_rules! load {
  // raw SAM alignment
  ( $values: expr, $flines: expr, $err: expr ) => {
    // read id
    $values.read_id = $flines[0].to_string();

    // flag & read orientation
    $values.flag = $flines[1].parse::<i32>().context($err)?;

    // mobile element
    $values.mobel = $flines[2].to_string();

    // alignment interpretation
    $values.position = $flines[3].parse::<i32>().context($err)?;

    // cigar
    $values
      .cigar
      .loader(&$flines[5].to_string(), $values.position);

    // sequence
    $values.sequence = $flines[9].to_string();
  };

  // mobile element
  ( $record: expr, $read_no: tt, $values: expr, $err: expr ) => {
    if $values.pv_flag <= 255 {
      $record.$read_no.sequence = $values.sequence.clone();
    }
    $record.$read_no.me_read.push(MEAnchor::loader(
      $values.cigar.clone(),
      $values.pv_flag,
      $values.mobel.clone(),
      $values.mobel_orientation.clone(),
      $values.pv_position,
      $values.mobel_size,
    ));
  };

  // structural variant
  ( $record: expr, $read_no: tt, $flines: expr ) => {
    $record.$read_no.sequence = $flines[9].to_string();
    $record.$read_no.chr_read = ChrAnchor::loader(&$flines);
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Load record onto hashmap.
#[macro_export]
macro_rules! reload {
  // chromosomal loci
  ( $record: expr, $read_no: tt, $flines: expr ) => {
    if ($record.$read_no.sequence == $flines[9])
      || ($record.$read_no.sequence_reverser() == $flines[9])
    {
      $record.$read_no.chr_read.push(ChrAnchor::loader(&$flines))
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Evaluate mapping quality (MAPQ).
#[macro_export]
macro_rules! mapq {
  // mapq
  ( $record: expr, $read_no: tt ) => {
    $record.$read_no.chr_read.is_empty()
      || $record.$read_no.chr_read[0].mapq < MAPQ
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Count reads that align at a certain bin along chromosome.
#[macro_export]
macro_rules! chr_counter {
  // mobile element chromosomal counter
  ( $read_id: expr, $read_count: expr, $chr_pair: expr, $position_hm: expr ) => {
    let binned_position = format!("{}", $chr_pair.binner());
    chr_counter($read_id, $position_hm, binned_position);
    $read_count += 1;
  };

  // structural variant chromosomal counter
  ( $read_id: expr, $sv_pair: expr, $position_hm: expr ) => {
    let binned_pos = (
      $sv_pair.read1.chr_read[0].binner(),
      $sv_pair.read2.chr_read[0].binner(),
    );
    let binned_position = format!(
      "{}-{}",
      std::cmp::min(binned_pos.0, binned_pos.1),
      std::cmp::max(binned_pos.0, binned_pos.1)
    );
    chr_counter($read_id, $position_hm, binned_position);
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
