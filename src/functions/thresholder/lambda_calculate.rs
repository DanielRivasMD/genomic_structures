////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::BIN_SIZE;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
use super::lambda_calculate;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
macro_rules! lambda_calculate {
  ( $function: ident;
    params |> $noreads: expr, $eflen: expr;
    expected |> $expected: expr;
  ) => {
    #[test]
    fn $function() {
      let lambda = lambda_calculate($noreads, $eflen, BIN_SIZE as f64);
      assert_eq!(
        lambda,
        $expected,
        "\n\nNumber of reads: {:?}.\nEffective scaffold length: {:?}.\nCalculated lambda (λ): {:?}.\nExpected: {:?}.\n\n",
        $noreads,
        $eflen,
        lambda,
        $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
lambda_calculate!(test01; params |> 100., 400.; expected |> 25.;);
lambda_calculate!(test02; params |> 5050., 75400000.; expected |> 0.006_697_612_732_095_490_5;);

////////////////////////////////////////////////////////////////////////////////////////////////////
