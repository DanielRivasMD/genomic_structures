////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  MEChimericRead,
  Sequence,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// // load & update
// macro_rules! me_chimeric_read {
//   ( $function: ident;
//     $sequence: expr, $expect: expr
//   ) => {
//     #[test]
//     fn $function() {
//       let mut me_chimeric_read = MEChimericRead::new();
//       me_chimeric_read.sequence = $sequence;
//       let reversed = me_chimeric_read.reverse_sequence();
//       assert_eq!(
//         reversed, $expect,
//         "\n\nInput sequence: {:?}.\nReversed sequence: {:?}.\nExpected:
// {:?}.\n\n",         $sequence, reversed, $expect,
//       );
//     }
//   };
// }

////////////////////////////////////////////////////////////////////////////////////////////////////

// test

////////////////////////////////////////////////////////////////////////////////////////////////////

// reverse sequence
macro_rules! reverse_sequence {
  ( $function: ident;
    $sequence: expr, $expect: expr
  ) => {
    #[test]
    fn $function() {
      let mut me_chimeric_read = MEChimericRead::new();
      me_chimeric_read.sequence = $sequence;
      let reversed = me_chimeric_read.reverse_sequence();
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
reverse_sequence!(rev01; "AAAAAAA".to_string(), "TTTTTTT");
reverse_sequence!(rev02; "MACTHAA".to_string(), "TTHAGTM");
reverse_sequence!(rev03; "CAAGAAC".to_string(), "GTTCTTG");
reverse_sequence!(rev04; "GATTACA".to_string(), "TGTAATC");

////////////////////////////////////////////////////////////////////////////////////////////////////
