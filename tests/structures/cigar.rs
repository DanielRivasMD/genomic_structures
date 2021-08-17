////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::CIGAR;

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
macro_rules! cigar {
  ( $function: ident;
    params |> $cigar: expr, $position: expr;
    expected |> $expected: expr;
  ) => {
    #[test]
    fn $function() {
      let cigar =
        CIGAR::load($cigar, $position).expect("CIGAR loading failed!");
      assert_eq!(
        cigar, $expected,
        "\n\nCalculated CIGAR:\n{:#?}.\n\nExpected:\n{:#?}.\n\n",
        cigar, $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
cigar!(test01;
  params |> "100M", 101;
  expected |> CIGAR{
    align: vec![100],
    deletion: vec![],
    insertion: vec![],
    left_boundry: 101,
    left_clip: 0,
    right_boundry: 200,
    rigth_clip: 0,
    signature: "100M".to_string(),
  };
);

cigar!(test02;
  params |> "54H46M", 101;
  expected |> CIGAR{
    align: vec![46],
    deletion: vec![],
    insertion: vec![],
    left_boundry: 47,
    left_clip: 54,
    right_boundry: 146,
    rigth_clip: 0,
    signature: "54H46M".to_string(),
  };
);

cigar!(test03;
  params |> "54S46M", 101;
  expected |> CIGAR{
    align: vec![46],
    deletion: vec![],
    insertion: vec![],
    left_boundry: 47,
    left_clip: 54,
    right_boundry: 146,
    rigth_clip: 0,
    signature: "54S46M".to_string(),
  };
);

cigar!(test04;
  params |> "3H67M30H", 101;
  expected |> CIGAR{
    align: vec![67],
    deletion: vec![],
    insertion: vec![],
    left_boundry: 98,
    left_clip: 3,
    right_boundry: 197,
    rigth_clip: 30,
    signature: "3H67M30H".to_string(),
  };
);

cigar!(test05;
  params |> "10H3M2I80M5H", 101;
  expected |> CIGAR{
    align: vec![3, 80],
    deletion: vec![],
    insertion: vec![2],
    left_boundry: 91,
    left_clip: 10,
    right_boundry: 190,
    rigth_clip: 5,
    signature: "10H3M2I80M5H".to_string(),
  };
);

cigar!(test06;
  params |> "13H60D7M20H", 101;
  expected |> CIGAR{
    align: vec![7],
    deletion: vec![60],
    insertion: vec![],
    left_boundry: 88,
    left_clip: 13,
    right_boundry: 187,
    rigth_clip: 20,
    signature: "13H60D7M20H".to_string(),
  };
);

cigar!(test07;
  params |> "50S4D6I40M", 101;
  expected |> CIGAR{
    align: vec![40],
    deletion: vec![4],
    insertion: vec![6],
    left_boundry: 51,
    left_clip: 50,
    right_boundry: 150,
    rigth_clip: 0,
    signature: "50S4D6I40M".to_string(),
  };
);

cigar!(test08;
  params |> "1H10D2M2D80M5H", 101;
  expected |> CIGAR{
    align: vec![2, 80],
    deletion: vec![10, 2],
    insertion: vec![],
    left_boundry: 100,
    left_clip: 1,
    right_boundry: 199,
    rigth_clip: 5,
    signature: "1H10D2M2D80M5H".to_string(),
  };
);

cigar!(test09;
  params |> "1H10D2M2D80M5H", 1;
  expected |> CIGAR{
    align: vec![2, 80],
    deletion: vec![10, 2],
    insertion: vec![],
    left_boundry: 0,
    left_clip: 1,
    right_boundry: 99,
    rigth_clip: 5,
    signature: "1H10D2M2D80M5H".to_string(),
  };
);

cigar!(test10;
  params |> "50S4D6I40M", 1;
  expected |> CIGAR{
    align: vec![40],
    deletion: vec![4],
    insertion: vec![6],
    left_boundry: -49,
    left_clip: 50,
    right_boundry: 50,
    rigth_clip: 0,
    signature: "50S4D6I40M".to_string(),
  };
);

////////////////////////////////////////////////////////////////////////////////////////////////////
