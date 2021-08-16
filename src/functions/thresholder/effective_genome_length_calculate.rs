////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  BIN_OVERLAP,
  BIN_SIZE,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
use super::effective_genome_length_calculate;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
macro_rules! effective_genome_length_calculate {
  ( $function: ident;
    $glen: expr, $expected: expr
  ) => {
    #[test]
    fn $function() {
      let effective_length = effective_genome_length_calculate(
        $glen,
        BIN_SIZE as f64,
        BIN_OVERLAP as f64,
      );
      assert_eq!(
        effective_length, $expected,
        "\n\nScaffold length: {:?}.\nEffective scaffold length: {:?}.\nExpected: {:?}.\n\n",
        $glen, effective_length, $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
effective_genome_length_calculate!(test01; 2000., 4000.);
effective_genome_length_calculate!(test02; 3243556456., 6487112912.);

////////////////////////////////////////////////////////////////////////////////////////////////////
