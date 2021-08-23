////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  ChrAnchorEnum,
  MEAnchor,
  MEChimericPair,
  OrientationEnum,
};

// crate utilities
use crate::load_me_anchor;

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

// tag
macro_rules! test_tag {
  ( $function: ident;
    // $me_chimeric_pair: expr;
    expect |> $expect: tt;
    vargs ... $($variadic_position1: expr => $variadic_orientation1: tt),+;
    vargs ... $($variadic_position2: expr => $variadic_orientation2: tt),+;
  ) => {
    #[test]
    fn $function() {
      let mut me_chimeric_pair = MEChimericPair::new();
      // variadic loading read1
      $( load_me_anchor!(me_chimeric_pair.read1, $variadic_position1, $variadic_orientation1); )+
      // variadic loading read2
      $( load_me_anchor!(me_chimeric_pair.read2, $variadic_position2, $variadic_orientation2); )+
      // tag
      me_chimeric_pair.tag();
      // assert
      assert_eq!(me_chimeric_pair.chranch, ChrAnchorEnum::$expect);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// fail
test_tag!(tag00; // none
  expect |> None;
  vargs ... 0 => None, 0 => None;
  vargs ... 0 => None, 0 => None;
);

test_tag!(tag01; // ambigous ends
  expect |> None;
  vargs ... 50 => Upstream, 75 => Upstream;
  vargs ... 1000 => Downstream;
);

test_tag!(tag02; // palindromic reads
  expect |> None;
  vargs ... 0 => None, 75 => Upstream;
  vargs ... 1050 => Downstream, 75 => Upstream;
);

test_tag!(tag03; // palindromic reads
  expect |> None;
  vargs ... 50 => Downstream, 75 => Upstream;
  vargs ... 50 => Upstream, 75 => Upstream;
);

// test upstream
test_tag!(tag10; // upstream same position both reads
  expect |> None;
  vargs ... 100 => Upstream, 50 => Upstream;
  vargs ... 50 => Upstream, 75 => Upstream;
);

test_tag!(tag11; // minimum read1
  expect |> Read1;
  vargs ... 100 => Upstream, 50 => Upstream;
  vargs ... 150 => Upstream, 75 => Upstream;
);

test_tag!(tag12; // minimum read2
  expect |> Read2;
  vargs ... 100 => Upstream, 150 => Upstream;
  vargs ... 150 => Upstream, 75 => Upstream;
);

test_tag!(tag13; // resilient secondary alignments
  expect |> Read1;
  vargs ... 100 => Upstream, 50 => Upstream, 10000 => Downstream;
  vargs ... 150 => Upstream, 75 => Upstream, 0 => None;
);

test_tag!(tag14;
  expect |> Read2; // alined same position physical read
  vargs ... 100 => Upstream, 150 => Upstream, 0 => None, 0 => Palindromic;
  vargs ... 150 => Upstream, 75 => Upstream, 75 => Upstream;
);

test_tag!(tag15; // umapped pair read2
  expect |> Read2;
  vargs ... 150 => Upstream, 75 => Upstream, 75 => Upstream;
  vargs ... 0 => None, 0 => None;
);

test_tag!(tag16;
  expect |> Read1; // umapped pair read1
  vargs ... 0 => None, 0 => None;
  vargs ... 150 => Upstream, 75 => Upstream, 75 => Upstream;
);

// test downstream
test_tag!(tag20; // downstream same position both reads
  expect |> None;
  vargs ... 10100 => Downstream, 10050 => Downstream;
  vargs ... 10100 => Downstream, 10075 => Downstream;
);

test_tag!(tag21; // maximum read1
  expect |> Read1;
  vargs ... 10150 => Downstream, 10075 => Downstream;
  vargs ... 10100 => Downstream, 10050 => Downstream;
);

test_tag!(tag22; // maximum read2
  expect |> Read2;
  vargs ... 10150 => Downstream, 10075 => Downstream;
  vargs ... 10100 => Downstream, 10250 => Downstream;
);

test_tag!(tag23; // resilient secondary alignments
  expect |> Read1;
  vargs ... 10150 => Downstream, 10075 => Downstream, 0 => None;
  vargs ... 10100 => Downstream, 10050 => Downstream, 100 => Upstream;
);

test_tag!(tag24;
  expect |> Read2; // alined same position physical read
  vargs ... 10100 => Downstream, 10050 => Downstream, 0 => None, 0 => Palindromic;
  vargs ... 10150 => Downstream, 10075 => Downstream, 10075 => Downstream;
);

test_tag!(tag25; // umapped pair read2
  expect |> Read2;
  vargs ... 10150 => Downstream, 1075 => Downstream, 1075 => Downstream;
  vargs ... 0 => None, 0 => None;
);

test_tag!(tag26;
  expect |> Read1; // umapped pair read1
  vargs ... 0 => None, 0 => None;
  vargs ... 10150 => Downstream, 1075 => Downstream, 1075 => Downstream;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
