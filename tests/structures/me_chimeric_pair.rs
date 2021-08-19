////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEChimericPair;

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update

////////////////////////////////////////////////////////////////////////////////////////////////////

// get
macro_rules! get_chr_anchor {
  ( $function: ident;
    $sequence: expr, $expect: expr
  ) => {
    #[test]
    fn $function() {
      let mut me_chimeric_pair = MEChimericPair::new();
      me_chimeric_pair.read1.sequence = $sequence;
      let retrieved = me_chimeric_pair.get_chr_anchor();
      assert_eq!(
        retrieved.sequence, $expect,
        "\n\nInput sequence: {:?}.\nRetrieved sequence: {:?}.\nExpected: {:?}.\n\n",
        $sequence, retrieved.sequence, $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
get_chr_anchor!(get01; "AAAAAAA".to_string(), "AAAAAAA".to_string());
get_chr_anchor!(get02; "MACTHAA".to_string(), "MACTHAA".to_string());
get_chr_anchor!(get03; "CAAGAAC".to_string(), "CAAGAAC".to_string());
get_chr_anchor!(get04; "GATTACA".to_string(), "GATTACA".to_string());

////////////////////////////////////////////////////////////////////////////////////////////////////
