////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEChimericRead;

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
macro_rules! me_chimeric_read {
  ( $function: ident;
    $sequence: expr, $expected: expr
  ) => {
    #[test]
    fn $function() {
      let mut to_reverse = MEChimericRead::new();
      to_reverse.sequence = $sequence;
      let reversed = to_reverse.reverse_sequence();
      assert_eq!(
        reversed, $expected,
        "\n\nInput sequence: {:?}.\nReversed sequence: {:?}.\nExpected: {:?}.\n\n",
        $sequence, reversed, $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
me_chimeric_read!(test01; "AAAAAAA".to_string(), "TTTTTTT".to_string());
me_chimeric_read!(test02; "MACTHAA".to_string(), "TTHAGTM".to_string());
me_chimeric_read!(test03; "CAAGAAC".to_string(), "GTTCTTG".to_string());
me_chimeric_read!(test04; "GATTACA".to_string(), "TGTAATC".to_string());

////////////////////////////////////////////////////////////////////////////////////////////////////

// reverse sequence
macro_rules! reverse_sequence {
  ( $function: ident;
    $sequence: expr, $expected: expr
  ) => {
    #[test]
    fn $function() {
      let mut me_chimeric_read = MEChimericRead::new();
      me_chimeric_read.sequence = $sequence;
      let reversed = me_chimeric_read.reverse_sequence();
      assert_eq!(
        reversed, $expected,
        "\n\nValue: {:?}.\nExpected: {:?}.\n\n",
        reversed, $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
reverse_sequence!(rev01; "GATTACA".to_string(), "TGTAATC".to_string());

////////////////////////////////////////////////////////////////////////////////////////////////////
