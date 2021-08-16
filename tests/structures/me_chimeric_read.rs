////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEChimericRead;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_me_chimeric_read(sequence, expected) => {
    let mut toreverse = super::MEChimericRead::new();
    toreverse.sequence = sequence;
    assert_eq!(toreverse.sequence_reverser(), expected);
  }
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

  - _00 ("GATTACA".to_string(), "TGTAATC".to_string())
  - _01 ("AAAAAAA".to_string(), "TTTTTTT".to_string())
  - _02 ("MACTHAA".to_string(), "TTHAGTM".to_string())
  - _03 ("CAAGAAC".to_string(), "GTTCTTG".to_string())
////////////////////////////////////////////////////////////////////////////////////////////////////

}
// test
me_chimeric_read!(test01; String::from("AAAAAAA"), String::from("TTTTTTT"));
me_chimeric_read!(test02; String::from("MACTHAA"), String::from("TTHAGTM"));
me_chimeric_read!(test03; String::from("CAAGAAC"), String::from("GTTCTTG"));
me_chimeric_read!(test04; String::from("GATTACA"), String::from("TGTAATC"));

////////////////////////////////////////////////////////////////////////////////////////////////////
