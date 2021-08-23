////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  anchor_count,
  MEAnchor,
  MEChimericRead,
  OrientationEnum,
};

// crate utilities
use crate::load_me_anchor;

////////////////////////////////////////////////////////////////////////////////////////////////////

// anchor count
macro_rules! test_anchor_count {
  ( $function: ident;
    params |> $anchor: tt;
    expect |> $expect: expr;
    vargs ... $($variadic_orientation: tt),+;
  ) => {
    #[test]
    fn $function() {
      let mut me_chimeric_read = MEChimericRead::new();
      // variadic loading
      $( load_me_anchor!(me_chimeric_read, $variadic_orientation); )+
      // count anchors
      let count = anchor_count!(me_chimeric_read, $anchor);
      // assert
      assert_eq!(count, $expect);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_anchor_count!(none00;
  params |> None;
  expect |> 0;
  vargs ... Downstream;
);

test_anchor_count!(none01;
  params |> None;
  expect |> 1;
  vargs ... Upstream, None, Downstream;
);

test_anchor_count!(upstream00;
  params |> Upstream;
  expect |> 0;
  vargs ... Downstream;
);

test_anchor_count!(upstream01;
  params |> Upstream;
  expect |> 1;
  vargs ... Upstream, None, Downstream;
);

test_anchor_count!(upstream02;
  params |> Upstream;
  expect |> 2;
  vargs ... Upstream, None, Downstream, Upstream, None;
);

test_anchor_count!(downstream00;
  params |> Downstream;
  expect |> 0;
  vargs ... Upstream;
);

test_anchor_count!(downstream01;
  params |> Downstream;
  expect |> 1;
  vargs ... Upstream, None, Downstream, Upstream;
);

test_anchor_count!(downstream02;
  params |> Downstream;
  expect |> 2;
  vargs ... Downstream, Downstream, None, Upstream;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
