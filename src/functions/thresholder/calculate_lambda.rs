////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::BIN_SIZE;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
use super::calculate_lambda;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
macro_rules! test_calculate_lambda {
  ( $function: ident;
    params |> $noreads: expr, $eflen: expr;
    expect |> $expect: expr;
  ) => {
    #[test]
    fn $function() {
      let lambda = calculate_lambda($noreads, $eflen, BIN_SIZE as f64);
      assert_eq!(
        lambda,
        $expect,
        "\n\nNumber of reads: {:?}.\nEffective scaffold length: {:?}.\nCalculated lambda (λ): {:?}.\nExpected: {:?}.\n\n",
        $noreads,
        $eflen,
        lambda,
        $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_calculate_lambda!(test01;
  params |> 100., 400.;
  expect |> 25.;
);

test_calculate_lambda!(test02;
  params |> 5050., 75400000.;
  expect |> 0.006_697_612_732_095_490_5;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
