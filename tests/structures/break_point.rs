////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::BreakPoint;

////////////////////////////////////////////////////////////////////////////////////////////////////

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
    sequence: String::from("B1234567890"),
    coordinate: 1.,
  };
);

break_point!(test02; assert_eq;
  params |> "OOOOO0987654321B", 1.;
  expected |> BreakPoint{
    sequence: String::from("0987654321B"),
    coordinate: 0.,
  };
);

break_point!(test03; assert_eq;
  params |> "MMMMMMMMM0987654321B1234567890OOOOO", -19.;
  expected |> BreakPoint{
    sequence: String::from("MMMMMMMMM0987654321B1234567890"),
    coordinate: 20.,
  };
);

break_point!(test04; assert_eq;
  params |> "OOOOO0987654321B1234567890MMMMMMMMM", 20.;
  expected |> BreakPoint{
    sequence: String::from("0987654321B1234567890MMMMMMMMM"),
    coordinate: -19.,
  };
);

// fail
break_point!(fail01; assert_ne;
  params |> "MRRRRRRRRRROOOOO", 0.;
  expected |> BreakPoint{
    sequence: String::from("MRRRRRRRRRROOOOO"),
    coordinate: 1.,
  };
);

break_point!(fail02; assert_ne;
  params |> "OOOOO0987654321B1234567890MMMMMMMMM", 20.;
  expected |> BreakPoint{
    sequence: String::from("OOOOO0987654321B1234567890MMMMMMMMM"),
    coordinate: -19.,
  };
);

////////////////////////////////////////////////////////////////////////////////////////////////////
