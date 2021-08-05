////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::ChrAnchor;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  // fn test_chr_anchor_load(chr, flag, pos, cigar, mapq, tlen, expected) => {
  //   let fline = vec!["", flag, chr, pos, mapq, cigar, "", "", tlen];
  //   let test_anchor = super::ChrAnchor::load(

  //   );
  //   assert_eq!(test_anchor.chr, expected.chr);
  //   assert_eq!(test_anchor.flag, expected.flag);
  //   assert_eq!(test_anchor.position, expected.position);
  //   assert_eq!(test_anchor.cigar, expected.cigar);
  //   assert_eq!(test_anchor.mapq, expected.mapq);
  //   assert_eq!(test_anchor.tlen, expected.tlen);
  // }

  // - _00 ("chr7", "56", "2099", "100M", "100", "100",
  //   super::ChrAnchor{
  //     chr: "chr7".to_string(),
  //     flag: 56,
  //     position: 2099,
  //     cigar: super::CIGAR::new(),
  //     mapq: 100,
  //     tlen: 100,
  //   }
  // )

  fn test_chr_anchor_binner(chr_anchor, expected) => {
    assert_eq!(chr_anchor.binner(), expected);
  }

  - _00 (
    super::ChrAnchor{
      chr: "chr7".to_string(),
      flag: 56,
      pos: 2099,
      cigar: "100M".to_string(),
      mapq: 100,
      tlen: 100,
    },
    2000
  )

  // fn test_chr_interpretor(chr_anchor, value, expected) => {
  //   assert_eq!(chr_anchor.interpretor(value), expected);
  // }

  // - _00 (
  //   super::ChrAnchor {
  //     chr:   "chr7".to_string(),
  //     flag:  177,
  //     pos:   2099,
  //     cigar: "100M".to_string(),
  //     mapq:  100,
  //     tlen:  100,
  //   },
  //   1,
  //   true
  // )

}

////////////////////////////////////////////////////////////////////////////////////////////////////
