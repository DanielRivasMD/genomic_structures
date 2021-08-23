////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  BreakPoint,
  MEAnchor,
  OrientationEnum,
  TagME,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
macro_rules! test_me_anchor {
  ( $function: ident;
    loaded |> $loaded_cigar: expr, $loaded_flag: expr, $loaded_mobel: expr, $loaded_orientation: expr, $loaded_position: expr, $loaded_size: expr;
    manual |> $manual_cigar: expr, $manual_flag: expr, $manual_mobel: expr, $manual_orientation: expr, $manual_position: expr, $manual_size: expr;
  ) => {
    #[test]
    fn $function() {
      let loaded = MEAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position)
          .expect("CIGAR loading failed!"),
        $loaded_flag,
        $loaded_mobel.clone(),
        $loaded_orientation,
        $loaded_position,
        $loaded_size,
      );

      let manual = MEAnchor {
        breakpoint:  BreakPoint::new(),
        cigar:       CIGAR::load($manual_cigar, $manual_position)
          .expect("CIGAR loading failed!"),
        flag:        $manual_flag,
        mobel:       $manual_mobel.clone(),
        orientation: $manual_orientation,
        position:    $manual_position,
        size:        $manual_size,
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
test_me_anchor!(test01;
  loaded |> "100M", 75, "mobel77".to_string(), OrientationEnum::None, 2099, 11000.;
  manual |> "100M", 75, "mobel77".to_string(), OrientationEnum::None, 2099, 11000.;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// calculate break point
macro_rules! test_calculate_break_point {
  ( $function: ident;
    loaded |> $loaded_sequence: expr, $loaded_cigar: expr, $loaded_flag: expr, $loaded_mobel: expr, $loaded_orientation: expr, $loaded_position: expr, $loaded_size: expr;
    manual |> $manual_sequence: expr, $manual_coordinate: expr;
  ) => {
    #[test]
    fn $function() {
      let mut loaded = MEAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position)
          .expect("CIGAR loading failed!"),
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
test_calculate_break_point!(bp01;
  loaded |> "MMMM0987654321B1234567890OOOOO", "15S15M", 83, "mobel77".to_string(), OrientationEnum::None, 1, 11000.;
  manual |> "MMMM0987654321B1234567890".to_string(), 15.;
);

test_calculate_break_point!(bp02;
  loaded |> "B1234567890OOOOO", "1S15M", 83, "mobel77".to_string(), OrientationEnum::None, 1, 11000.;
  manual |> "B1234567890".to_string(), 1.;
);

test_calculate_break_point!(bp03;
  loaded |> "OOOOO0987654321B", "30M1S", 75, "mobel77".to_string(), OrientationEnum::None, 10971, 11000.;
  manual |> "0987654321B".to_string(), 0.;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// tag
macro_rules! test_tag {
  ( $function: ident;
    loaded |> $loaded_cigar: expr, $loaded_flag: expr, $loaded_mobel: expr, $loaded_orientation: expr, $loaded_position: expr, $loaded_size: expr;
    expect |> $manual_orientation: expr;
  ) => {
    #[test]
    fn $function() {
      let mut loaded = MEAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position)
          .expect("CIGAR loading failed!"),
        $loaded_flag,
        $loaded_mobel.clone(),
        $loaded_orientation,
        $loaded_position,
        $loaded_size,
      );
      loaded.tag();

      let mut manual = MEAnchor::new();
      manual.orientation = $manual_orientation;

      assert_eq!(
        loaded.orientation, manual.orientation,
        "\n\nLoaded value:\n{:#?}.\n\nManual value:\n{:#?}.\n\n",
        loaded.orientation, manual.orientation,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_tag!(tag00;
  loaded |> "30M1S", 83, "mobel77".to_string(), OrientationEnum::None, 10871, 11000.;
  expect |> OrientationEnum::None;
);

test_tag!(tag01;
  loaded |> "15S15M", 75, "mobel77".to_string(), OrientationEnum::None, 1, 11000.;
  expect |> OrientationEnum::Upstream;
);

test_tag!(tag02;
  loaded |> "15S15M", 83, "mobel77".to_string(), OrientationEnum::None, 1, 11000.;
  expect |> OrientationEnum::Upstream;
);

test_tag!(tag03;
  loaded |> "30M1S", 75, "mobel77".to_string(), OrientationEnum::None, 10971, 11000.;
  expect |> OrientationEnum::Downstream;
);

test_tag!(tag04;
  loaded |> "30M1S", 83, "mobel77".to_string(), OrientationEnum::None, 10971, 11000.;
  expect |> OrientationEnum::Downstream;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
