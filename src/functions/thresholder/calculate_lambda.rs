////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::BIN_SIZE;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
use super::calculate_lambda;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
macro_rules! calculate_lambda {
  ( $function: ident;
    params |> $noreads: expr, $eflen: expr;
    expected |> $expected: expr;
  ) => {
    #[test]
    fn $function() {
      let lambda = calculate_lambda($noreads, $eflen, BIN_SIZE as f64);
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
calculate_lambda!(test01; params |> 100., 400.; expected |> 25.;);
calculate_lambda!(test02; params |> 5050., 75400000.; expected |> 0.006_697_612_732_095_490_5;);

////////////////////////////////////////////////////////////////////////////////////////////////////
