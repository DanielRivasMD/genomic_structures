////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
use super::cumsum;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
macro_rules! test_cumsum {
  ( $function: ident;
      params |> $cum_vec: expr;
      expect |> $expect: expr;
    ) => {
    #[test]
    fn $function() {
      let cummulative = cumsum($cum_vec);
      assert_eq!(
        cummulative, $expect,
        "\n\nOriginal vector:\n{:#?}.\n\nCummulative:\n{:#?}.\n\nExpected value:\n{:#?}.\n\n",
        $cum_vec, cummulative, $expect,
      )
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_cumsum!(test01;
  params |> vec![0., 1., 2., 3., 4., ];
  expect |> vec![0., 1., 3., 6., 10., ];
);

test_cumsum!(test02;
  params |> vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ];
  expect |> vec![0.,  1., 3., 6., 10., 15., 21., 28., 36., 45., 55., ];
);

test_cumsum!(test03;
  params |> vec![10., 9., 8., 7., 6., 5., 4., 3., 2., 1., 0., ];
  expect |> vec![10., 19., 27., 34., 40., 45., 49., 52., 54., 55., 55., ];
);

////////////////////////////////////////////////////////////////////////////////////////////////////
