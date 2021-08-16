////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEChimericPair;

////////////////////////////////////////////////////////////////////////////////////////////////////

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
me_chimeric_pair!(test01; String::from("AAAAAAA"), String::from("AAAAAAA"));
me_chimeric_pair!(test02; String::from("MACTHAA"), String::from("MACTHAA"));
me_chimeric_pair!(test03; String::from("CAAGAAC"), String::from("CAAGAAC"));
me_chimeric_pair!(test04; String::from("GATTACA"), String::from("GATTACA"));

////////////////////////////////////////////////////////////////////////////////////////////////////
