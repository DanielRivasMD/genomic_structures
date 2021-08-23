////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::reverse_sequence;

////////////////////////////////////////////////////////////////////////////////////////////////////

// reverse sequence
macro_rules! test_reverse_sequence {
  ( $function: ident;
    $sequence: expr, $expect: expr
  ) => {
    #[test]
    fn $function() {
      let reversed = reverse_sequence($sequence);
      assert_eq!(
        reversed, $expect,
        "\n\nInput sequence: {:?}.\nReversed sequence: {:?}.\nExpected: {:?}.\n\n",
        $sequence, reversed, $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_reverse_sequence!(rev01; "AAAAAAA", "TTTTTTT".to_string());
test_reverse_sequence!(rev02; "MACTHAA", "TTHAGTM".to_string());
test_reverse_sequence!(rev03; "CAAGAAC", "GTTCTTG".to_string());
test_reverse_sequence!(rev04; "GATTACA", "TGTAATC".to_string());

////////////////////////////////////////////////////////////////////////////////////////////////////
