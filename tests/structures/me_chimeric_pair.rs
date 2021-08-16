////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEChimericPair;
use genomic_structures::MEChimericRead;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_me_chimeric_read(sequence, expected) => {
    let mut toretrieve = super::MEChimericPair::new();
    toretrieve.read1 = super::MEChimericRead::new();
    toretrieve.read1.sequence = sequence;
    let retrieved = toretrieve.chr_anchor_retriever();

    assert_eq!(retrieved.sequence, expected);
  }
macro_rules! me_chimeric_pair {
  ( $function: ident;
    $sequence: expr, $expected: expr
  ) => {
    #[test]
    fn $function() {
      let mut to_retrieve = MEChimericPair::new();
      to_retrieve.read1.sequence = $sequence;
      let retrieved = to_retrieve.get_chr_anchor();
      assert_eq!(
        retrieved.sequence, $expected,
        "\n\nInput sequence: {:?}.\nRetrieved sequence: {:?}.\nExpected: {:?}.\n\n",
        $sequence, retrieved.sequence, $expected,
      );
    }
  };
}

  - _00 ("GATTACA".to_string(), "GATTACA".to_string())
  - _01 ("AAAAAAA".to_string(), "AAAAAAA".to_string())
  - _02 ("MACTHAA".to_string(), "MACTHAA".to_string())
  - _03 ("CAAGAAC".to_string(), "CAAGAAC".to_string())
////////////////////////////////////////////////////////////////////////////////////////////////////

}
// test
me_chimeric_pair!(test01; String::from("AAAAAAA"), String::from("AAAAAAA"));
me_chimeric_pair!(test02; String::from("MACTHAA"), String::from("MACTHAA"));
me_chimeric_pair!(test03; String::from("CAAGAAC"), String::from("CAAGAAC"));
me_chimeric_pair!(test04; String::from("GATTACA"), String::from("GATTACA"));

////////////////////////////////////////////////////////////////////////////////////////////////////
