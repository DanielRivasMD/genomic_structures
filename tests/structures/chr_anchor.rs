////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  AnchorEnum,
  ChrAnchor,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

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
