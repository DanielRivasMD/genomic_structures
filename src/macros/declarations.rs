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

    // scaffold
    $values.scaffold = $flines[2].to_string();

    // position
    $values.position = $flines[3].parse::<i32>().context($err)?;

    //  quality
    $values.quality = $flines[4].parse::<i32>().context($err)?;

    // cigar
    $values
      .cigar
      .update(&$flines[5].to_string(), $values.position);

    // alignment length
    $values.tlen = $flines[8].parse::<i32>().context($err)?;

    // sequence
    $values.sequence = $flines[9].to_string();
  };

  // mobile element on hash map
  ( $record: expr, $read_no: tt, $values: expr, $switches: expr, $err: expr ) => {
    if $values.flag <= 255 {
      $record.$read_no.sequence = $values.sequence.clone();
      $record.$read_no.quality = $values.quality;
    }
    $record.$read_no.me_read.push(MEAnchor::load(
      $values.cigar.clone(),
      $values.flag,
      $values.scaffold.clone(),
      $switches.mobel_anchor.orientation.clone(),
      $values.position,
      $switches.mobel_anchor.size,
    ));
  };

  // structural variant
  ( $record: expr, $read_no: tt, $flines: expr ) => {
    $record.$read_no.sequence = $flines[9].to_string();
    $record.$read_no.chr_read = ChrAnchor::load(&$flines);
    // TODO: update ChrAnchor::loader
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Load record onto hashmap.
#[macro_export]
macro_rules! reload {
  // chromosomal loci
  ( $record: expr, $read_no: tt, $values: expr ) => {
    if $record.$read_no.sequence == $values.sequence
      || $record.$read_no.sequence_reverser() == $values.sequence
    {
      $record.$read_no.chr_read.push(ChrAnchor::load(
        $values.cigar.clone(),
        $values.flag,
        $values.scaffold.clone(),
        $values.quality,
        $values.position,
        $values.tlen,
      ))
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
