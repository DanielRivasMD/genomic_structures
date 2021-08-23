////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::BreakPoint;

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
macro_rules! test_break_point {
  ( $function: ident; $assertion: ident;
    params |> $sequence: expr, $offset: expr;
    expect |> $expect: expr;
  ) => {
    #[test]
    fn $function() {
      let break_point = BreakPoint::load($sequence, $offset);

      $assertion!(
        break_point,
        $expect,
        "\n\nCalculated BreakPoint:\n{:#?}.\n\nExpected:\n{:#?}.\n\n",
        break_point,
        $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_break_point!(test01; assert_eq;
  params |> "B1234567890OOOOO", 0.;
  expect |> BreakPoint{
    sequence: "B1234567890".to_string(),
    coordinate: 1.,
  };
);

test_break_point!(test02; assert_eq;
  params |> "OOOOO0987654321B", 1.;
  expect |> BreakPoint{
    sequence: "0987654321B".to_string(),
    coordinate: 0.,
  };
);

test_break_point!(test03; assert_eq;
  params |> "MMMMMMMMM0987654321B1234567890OOOOO", -19.;
  expect |> BreakPoint{
    sequence: "MMMMMMMMM0987654321B1234567890".to_string(),
    coordinate: 20.,
  };
);

test_break_point!(test04; assert_eq;
  params |> "OOOOO0987654321B1234567890MMMMMMMMM", 20.;
  expect |> BreakPoint{
    sequence: "0987654321B1234567890MMMMMMMMM".to_string(),
    coordinate: -19.,
  };
);

// fail
test_break_point!(fail01; assert_ne;
  params |> "MRRRRRRRRRROOOOO", 0.;
  expect |> BreakPoint{
    sequence: "MRRRRRRRRRROOOOO".to_string(),
    coordinate: 1.,
  };
);

test_break_point!(fail02; assert_ne;
  params |> "OOOOO0987654321B1234567890MMMMMMMMM", 20.;
  expect |> BreakPoint{
    sequence: "OOOOO0987654321B1234567890MMMMMMMMM".to_string(),
    coordinate: -19.,
  };
);

////////////////////////////////////////////////////////////////////////////////////////////////////
