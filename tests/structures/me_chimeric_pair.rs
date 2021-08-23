////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  ChrAnchorEnum,
  MEAnchor,
  MEChimericPair,
  OrientationEnum,
};

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
      // read1
      $(
        let mut me_anchor = MEAnchor::new();
        me_anchor.orientation = OrientationEnum::$variadic_orientation1;
        me_anchor.position = $variadic_position1;
        me_chimeric_pair.read1.me_read.push(me_anchor);
      )+
      // read2
      $(
        let mut me_anchor = MEAnchor::new();
        me_anchor.orientation = OrientationEnum::$variadic_orientation2;
        me_anchor.position = $variadic_position2;
        me_chimeric_pair.read2.me_read.push(me_anchor);
      )+
      // tag
      me_chimeric_pair.tag();
      // assert
      assert_eq!(me_chimeric_pair.chranch, ChrAnchorEnum::$expect);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_tag!(tag00;
  expect |> None;
  vargs ... 50 => Upstream, 75 => Upstream;
  vargs ... 50 => Upstream, 75 => Upstream;
);

test_tag!(tag01;
  expect |> Read1;
  vargs ... 100 => Upstream, 50 => Upstream;
  vargs ... 150 => Upstream, 75 => Upstream;
);

test_tag!(tag02;
  expect |> Read2;
  vargs ... 100 => Upstream, 150 => Upstream;
  vargs ... 150 => Upstream, 75 => Upstream;
);

test_tag!(tag03;
  expect |> Read1;
  vargs ... 10995 => Downstream, 10500 => Downstream;
  vargs ... 10950 => Downstream, 10900 => Downstream;
);

test_tag!(tag04;
  expect |> Read2;
  vargs ... 10100 => Downstream, 10050 => Downstream;
  vargs ... 10150 => Downstream, 10075 => Downstream;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
