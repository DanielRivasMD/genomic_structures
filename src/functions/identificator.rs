////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::TRANSLOCATION_DISTANCE;
use crate::{
  functions::flag_interpretor::interpret,
  structures::{
    sv_chimeric_pair::SVChimericPair,
    sv_type::SVType,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Identify structural variant type.
///
/// # Examples
///
/// ```
/// ```
pub fn identify(
  pair: &mut SVChimericPair,
  expected_tlen: i32,
) -> bool {
  let mut psw = vec![];

  // evaluate read pairs
  psw.push(sv_deletion(pair, expected_tlen));
  psw.push(sv_duplication(pair));
  psw.push(sv_inversion(pair));
  psw.push(sv_insertion(pair));
  psw.push(sv_translocation(pair));
  // BUG: variant are called simultaneous

  psw.contains(&true)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait SVIdentify {
  fn identify(
    self,
    expected_tlen: i32,
  ) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// deletion
fn sv_deletion(
  pair: &mut SVChimericPair,
  expected_tlen: i32,
) -> bool {
  let tlen = pair.read1.chr_read[0].position - pair.read2.chr_read[0].position;
  if tlen.abs() >= expected_tlen {
    pair.svtag = SVType::Deletion;
    true
  } else {
    false
  }
}

// duplication
fn sv_duplication(pair: &mut SVChimericPair) -> bool {
  if pair.read1.chr_read[0].tlen > 0 &&
    !interpret(pair.read1.chr_read[0].flag, 5) &&
    !interpret(pair.read2.chr_read[0].flag, 5)
  {
    pair.svtag = SVType::Duplication;
    true
  } else {
    false
  }
}

// inversion
fn sv_inversion(pair: &mut SVChimericPair) -> bool {
  if interpret(pair.read1.chr_read[0].flag, 5) ==
    interpret(pair.read2.chr_read[0].flag, 5) &&
    (pair.read1.chr_read[0].chr == pair.read2.chr_read[0].chr)
  {
    pair.svtag = SVType::Inversion;
    true
  } else {
    false
  }
}

// insertion
fn sv_insertion(pair: &mut SVChimericPair) -> bool {
  if interpret(pair.read1.chr_read[0].flag, 3) ||
    interpret(pair.read2.chr_read[0].flag, 3)
  {
    pair.svtag = SVType::Insertion;
    true
  } else {
    false
  }
}

// translocation
fn sv_translocation(pair: &mut SVChimericPair) -> bool {
  let tlen = pair.read1.chr_read[0].position - pair.read2.chr_read[0].position;
  if tlen.abs() > TRANSLOCATION_DISTANCE ||
    pair.read1.chr_read[0].chr != pair.read2.chr_read[0].chr
  {
    pair.svtag = SVType::Translocation;
    true
  } else {
    false
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test private functions
#[cfg(test)]
mod priv_tests {
  use super::{
    sv_deletion,
    sv_duplication,
    sv_insertion,
    sv_inversion,
    sv_translocation,
  };
  use crate::structures::sv_chimeric_pair::SVChimericPair;
  use crate::structures::sv_type::SVType;
  use data_test::data_test;

  data_test! {

    fn test_deletion(pos1, pos2, exlen, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read[0].pos = pos1;
      svchim.read2.chr_read.pos = pos2;

      assert!(super::sv_deletion(&mut svchim, exlen), expected);
    }
    // TODO test unit not working as intended
    // - un_del (1000, 30000, 500, false)
    // - dos_del (30000, 1000, 500, true)
    // - tres_del (1900, 1400, 500, true)

    fn test_duplication(tl, fl1, fl2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.tlen = tl;
      svchim.read1.chr_read.flag = fl1;
      svchim.read2.chr_read.flag = fl2;

      assert!(super::sv_duplication(&mut svchim), expected);
    }
    // - un_dup (1, 123, 324, true)

    fn test_inversion(fl1, ch1, fl2, ch2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.flag = fl1;
      svchim.read1.chr_read.chr = ch1.to_string();
      svchim.read2.chr_read.flag = fl2;
      svchim.read2.chr_read.chr = ch2.to_string();

      assert!(super::sv_inversion(&mut svchim), expected);
    }
    // - un_inv (177, 1, 177, 1, false)

    fn test_insertion(fl1, fl2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.flag = fl1;
      svchim.read2.chr_read.flag = fl2;

      assert!(super::sv_insertion(&mut svchim), expected);
    }
    // - un_ins (123, 123, false)

    fn test_translocation(pos1, ch1, pos2, ch2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.pos = pos1;
      svchim.read1.chr_read.chr = ch1.to_string();
      svchim.read2.chr_read.pos = pos2;
      svchim.read2.chr_read.chr = ch2.to_string();

      assert!(super::sv_translocation(&mut svchim), expected);
    }
    // - un_trans (10, 1, 809, 7, false)

  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
