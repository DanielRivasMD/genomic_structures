////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::AnchorEnum;
use genomic_structures::ChrAnchor;
use genomic_structures::CIGAR;

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
    assert_eq!(chr_anchor.bin(), expected);
  }

  - _00 (
    super::ChrAnchor{
      chr: "chr7".to_string(),
      flag: 56,
      position: 2099,
      cigar: super::CIGAR::new(),
      mapq: 100,
      tlen: 100,
    },
    2000
  )

  // fn test_chr_interpretor(chr_anchor, value, expected) => {
  //   assert_eq!(chr_anchor.interpretor(value), expected);
  // }
// test update & load macro
macro_rules! chr_anchor {
  ( $function: ident; $assertion: ident;
    loaded |> $loaded_cigar: expr, $loaded_chr: expr, $loaded_flag: expr, $loaded_position: expr, $loaded_mapq: expr, $loaded_tlen: expr;
    manual |> $manual_cigar: expr, $manual_chr: expr, $manual_flag: expr, $manual_position: expr, $manual_mapq: expr, $manual_tlen: expr;
  ) => {
    #[test]
    fn $function() {
      let loaded = ChrAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position).unwrap(),
        $loaded_chr.clone(),
        $loaded_flag,
        $loaded_mapq,
        $loaded_position,
        $loaded_tlen,
      );

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
      let manual = ChrAnchor {
        anchor:   AnchorEnum::None,
        cigar:    CIGAR::load($manual_cigar, $manual_position).unwrap(),
        chr:      $manual_chr.clone(),
        flag:     $manual_flag,
        position: $manual_position,
        mapq:     $manual_mapq,
        tlen:     $manual_tlen,
      };

      $assertion!(
        loaded,
        manual,
        "\n\nLoaded value:\n{:#?}.\n\nManual value:\n{:#?}.\n\n",
        loaded,
        manual,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
chr_anchor!(test01; assert_eq;
  loaded |> "100M", String::from("chr7"), 56, 2099, 60, 100;
  manual |> "100M", String::from("chr7"), 56, 2099, 60, 100;
);

// fail
chr_anchor!(fail01; assert_ne;
  loaded |> "100M", String::from("chr5"), 56, 2099, 60, 100;
  manual |> "100M", String::from("chr7"), 56, 2099, 60, 100;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
