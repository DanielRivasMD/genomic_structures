////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  Anchor,
  AnchorEnum,
  ChrAnchor,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
macro_rules! test_chr_anchor {
  ( $function: ident;
    loaded |> $loaded_cigar: expr, $loaded_chr: expr, $loaded_flag: expr, $loaded_position: expr, $loaded_mapq: expr, $loaded_tlen: expr;
    manual |> $manual_cigar: expr, $manual_chr: expr, $manual_flag: expr, $manual_position: expr, $manual_mapq: expr, $manual_tlen: expr;
  ) => {
    #[test]
    fn $function() {
      let loaded = ChrAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position)
          .expect("CIGAR loading failed!"),
        $loaded_chr.clone(),
        $loaded_flag,
        $loaded_mapq,
        $loaded_position,
        $loaded_tlen,
      );

      let manual = ChrAnchor {
        anchor:   AnchorEnum::None,
        cigar:    CIGAR::load($manual_cigar, $manual_position)
          .expect("CIGAR loading failed!"),
        chr:      $manual_chr.clone(),
        flag:     $manual_flag,
        position: $manual_position,
        mapq:     $manual_mapq,
        tlen:     $manual_tlen,
      };

      assert_eq!(
        loaded, manual,
        "\n\nLoaded value:\n{:#?}.\n\nManual value:\n{:#?}.\n\n",
        loaded, manual,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_chr_anchor!(test01;
  loaded |> "100M", "chr7".to_string(), 56, 2099, 60, 100;
  manual |> "100M", "chr7".to_string(), 56, 2099, 60, 100;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// bin
macro_rules! test_bin {
  ( $function: ident;
    $position: expr, $expect: expr
  ) => {
    #[test]
    fn $function() {
      let mut chr_anchor = ChrAnchor::new();
      chr_anchor.position = $position;
      let binned = chr_anchor.bin();
      assert_eq!(
        binned, $expect,
        "\n\nValue: {:?}.\nExpected: {:?}.\n\n",
        binned, $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_bin!(bin01; 2099, 2000);

////////////////////////////////////////////////////////////////////////////////////////////////////
