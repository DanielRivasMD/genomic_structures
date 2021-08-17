////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::BreakPoint;

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
macro_rules! break_point {
  ( $function: ident; $assertion: ident;
    params |> $sequence: expr, $offset: expr;
    expected |> $expected: expr;
  ) => {
    #[test]
    fn $function() {
      let break_point = BreakPoint::load($sequence, $offset);

      $assertion!(
        break_point,
        $expected,
        "\n\nCalculated BreakPoint:\n{:#?}.\n\nExpected:\n{:#?}.\n\n",
        break_point,
        $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
break_point!(test01; assert_eq;
  params |> "B1234567890OOOOO", 0.;
  expected |> BreakPoint{
    sequence: "B1234567890".to_string(),
    coordinate: 1.,
  };
);

break_point!(test02; assert_eq;
  params |> "OOOOO0987654321B", 1.;
  expected |> BreakPoint{
    sequence: "0987654321B".to_string(),
    coordinate: 0.,
  };
);

break_point!(test03; assert_eq;
  params |> "MMMMMMMMM0987654321B1234567890OOOOO", -19.;
  expected |> BreakPoint{
    sequence: "MMMMMMMMM0987654321B1234567890".to_string(),
    coordinate: 20.,
  };
);

break_point!(test04; assert_eq;
  params |> "OOOOO0987654321B1234567890MMMMMMMMM", 20.;
  expected |> BreakPoint{
    sequence: "0987654321B1234567890MMMMMMMMM".to_string(),
    coordinate: -19.,
  };
);

// fail
break_point!(fail01; assert_ne;
  params |> "MRRRRRRRRRROOOOO", 0.;
  expected |> BreakPoint{
    sequence: "MRRRRRRRRRROOOOO".to_string(),
    coordinate: 1.,
  };
);

break_point!(fail02; assert_ne;
  params |> "OOOOO0987654321B1234567890MMMMMMMMM", 20.;
  expected |> BreakPoint{
    sequence: "OOOOO0987654321B1234567890MMMMMMMMM".to_string(),
    coordinate: -19.,
  };
);

////////////////////////////////////////////////////////////////////////////////////////////////////
