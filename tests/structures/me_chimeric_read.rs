////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  MEAnchor,
  MEChimericRead,
  OrientationEnum,
  Sequence,
};

// crate utilities
use crate::load_me_anchor;

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

// tag
macro_rules! test_tag {
  ( $function: ident;
    expect |> $expect: tt;
    vargs ... $($variadic_orientation: tt),+;
  ) => {
    #[test]
    fn $function() {
      let mut me_chimeric_read = MEChimericRead::new();
      // variadic loading
      $( load_me_anchor!(me_chimeric_read, $variadic_orientation); )+
      // tag
      me_chimeric_read.tag();
      // assert
      assert_eq!(me_chimeric_read.orientation, OrientationEnum::$expect);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_tag!(tag00;
  expect |> None;
  vargs ... None;
);

test_tag!(tag01;
  expect |> Upstream;
  vargs ... Upstream;
);

test_tag!(tag02;
  expect |> Downstream;
  vargs ... Downstream;
);

test_tag!(tag03;
  expect |> Palindromic;
  vargs ... Upstream, Downstream;
);

test_tag!(tag04;
  expect |> Upstream;
  vargs ... Upstream, Downstream, Upstream;
);

test_tag!(tag05;
  expect |> Downstream;
  vargs ... Upstream, Downstream, Downstream;
);

test_tag!(tag06;
  expect |> Palindromic;
  vargs ... Upstream, Downstream, Downstream, Upstream;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// count tag
macro_rules! test_edge {
  ( $function: ident;
    expect |> $expect: expr;
    vargs ... $($variadic_position: expr => $variadic_orientation: tt),+;
  ) => {
    #[test]
    fn $function() {
      let mut me_chimeric_read = MEChimericRead::new();
      // variadic loading
      $( load_me_anchor!(me_chimeric_read, $variadic_position, $variadic_orientation); )+
      // tag
      me_chimeric_read.tag();
      // determine edge
      let edge = me_chimeric_read.edge();
      // assert
      assert_eq!(edge, $expect);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
// none
test_edge!(edge00;
  expect |> 0;
  vargs ... 0 => None, 0 => None;
);

// upstream
test_edge!(edge01;
  expect |> 100;
  vargs ... 100 => Upstream;
);

// upstream <- minimum
test_edge!(edge02;
  expect |> 50;
  vargs ... 50 => Upstream, 75 => Upstream;
);

// upstream
test_edge!(edge03;
  expect |> 10000;
  vargs ... 10000 => Downstream;
);

// downstream <- maximum
test_edge!(edge04;
  expect |> 10075;
  vargs ... 10050 => Downstream, 10075 => Downstream;
);

// palindromic
test_edge!(edge05;
  expect |> 0;
  vargs ... 50 => Downstream, 75 => Upstream;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// reverse sequence
macro_rules! test_reverse_sequence {
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
test_reverse_sequence!(rev01; "AAAAAAA".to_string(), "TTTTTTT");
test_reverse_sequence!(rev02; "MACTHAA".to_string(), "TTHAGTM");
test_reverse_sequence!(rev03; "CAAGAAC".to_string(), "GTTCTTG");
test_reverse_sequence!(rev04; "GATTACA".to_string(), "TGTAATC");

////////////////////////////////////////////////////////////////////////////////////////////////////
