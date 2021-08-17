////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEChimericPair;

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
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

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
me_chimeric_pair!(test01; "AAAAAAA".to_string(), "AAAAAAA".to_string());
me_chimeric_pair!(test02; "MACTHAA".to_string(), "MACTHAA".to_string());
me_chimeric_pair!(test03; "CAAGAAC".to_string(), "CAAGAAC".to_string());
me_chimeric_pair!(test04; "GATTACA".to_string(), "GATTACA".to_string());

////////////////////////////////////////////////////////////////////////////////////////////////////
