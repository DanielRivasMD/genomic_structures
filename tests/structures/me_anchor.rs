////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  MEAnchor,
  OrientationEnum,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// test update & load macro
macro_rules! me_anchor {
  ( $function: ident; $assertion: ident;
    loaded |> $loaded_cigar: expr, $loaded_flag: expr, $loaded_mobel: expr, $loaded_orientation: expr, $loaded_position: expr, $loaded_size: expr;
    manual |> $manual_cigar: expr, $manual_flag: expr, $manual_mobel: expr, $manual_orientation: expr, $manual_position: expr, $manual_size: expr;
  ) => {
    #[test]
    fn $function() {
      let loaded = MEAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position).unwrap(),
        $loaded_flag,
        $loaded_mobel.clone(),
        $loaded_orientation,
        $loaded_position,
        $loaded_size,
      );

      let manual = MEAnchor {
        cigar:       CIGAR::load($manual_cigar, $manual_position).unwrap(),
        flag:        $manual_flag,
        mobel:       $manual_mobel.clone(),
        orientation: $manual_orientation,
        position:    $manual_position,
        size:        $manual_size,
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
me_anchor!(test01; assert_eq;
  loaded |> "100M", 75, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
  manual |> "100M", 75, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
);

// fail
me_anchor!(fail01; assert_ne;
  loaded |> "100M", 75, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
  manual |> "100M", 95, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
