////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  BreakPoint,
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
        breakpoint:  BreakPoint::new(),
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

macro_rules! calculate_break_point {
  ( $function: ident;
    loaded |> $loaded_sequence: expr, $loaded_cigar: expr, $loaded_flag: expr, $loaded_mobel: expr, $loaded_orientation: expr, $loaded_position: expr, $loaded_size: expr;
    manual |> $manual_sequence: expr, $manual_coordinate: expr;
  ) => {
    #[test]
    fn $function() {
      let mut loaded = MEAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position).unwrap(),
        $loaded_flag,
        $loaded_mobel.clone(),
        $loaded_orientation,
        $loaded_position,
        $loaded_size,
      );

      loaded.calculate_break_point($loaded_sequence);

      let mut manual = MEAnchor::new();
      manual.breakpoint = BreakPoint {
        sequence:   $manual_sequence,
        coordinate: $manual_coordinate,
      };

      assert_eq!(
        loaded.breakpoint, manual.breakpoint,
        "\n\nLoaded value:\n{:#?}.\n\nManual value:\n{:#?}.\n\n",
        loaded.breakpoint, manual.breakpoint,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
calculate_break_point!(bp01;
  loaded |> "MMMM0987654321B1234567890OOOOO", "15S15M", 83, String::from("mobel77"), OrientationEnum::None, 1, 11000.;
  manual |> String::from("MMMM0987654321B1234567890"), 15.;
);

calculate_break_point!(bp02;
  loaded |> "B1234567890OOOOO", "1S15M", 83, String::from("mobel77"), OrientationEnum::None, 1, 11000.;
  manual |> String::from("B1234567890"), 1.;
);

calculate_break_point!(bp03;
  loaded |> "OOOOO0987654321B", "30M1S", 75, String::from("mobel77"), OrientationEnum::None, 10971, 11000.;
  manual |> String::from("0987654321B"), 0.;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
