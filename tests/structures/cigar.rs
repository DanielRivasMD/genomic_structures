////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::CIGAR;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_alignment(to_cigar, position, expected) => {
    let mut cigar = super::CIGAR::new();
    cigar.update(to_cigar, position);
    assert_eq!(cigar.align, expected);
  }

  - _00 ("100M", 0, vec![100])
  - _01 ("3H67M20H", 0, vec![67])
  - _02 ("54S46M", 0, vec![46])
  - _03 ("10H3M2I80M5H", 0, vec![3, 80])

  fn test_deletion(to_cigar, position, expected) => {
    let mut cigar = super::CIGAR::new();
    cigar.update(to_cigar, position);
    assert_eq!(cigar.del, expected);
  }

  - _00 ("100M", 0, vec![])
  - _01 ("3H60D7M20H", 0, vec![60])
  - _02 ("50S4D6I4M", 0, vec![4])
  - _03 ("1H10D2M2D80M5H", 0, vec![10, 2])

  fn test_insertion(to_cigar, position, expected) => {
    let mut cigar = super::CIGAR::new();
    cigar.update(to_cigar, position);
    assert_eq!(cigar.ins, expected);
  }

  - _00 ("100M", 0, vec![])
  - _01 ("3H60I7M20H", 0, vec![60])
  - _02 ("59S4I46M", 0, vec![4])
  - _03 ("10H1I2M2I80M5H", 0, vec![1, 2])

  fn test_left_clip(to_cigar, position, expected) => {
    let mut cigar = super::CIGAR::new();
    cigar.update(to_cigar, position);
    assert_eq!(cigar.lclip, expected);
  }

  - _00 ("100M", 0, 0)
  - _01 ("23H67M", 0, 23)
  - _02 ("54S46M", 0, 54)
  - _03 ("10H3D2I80M5H", 0, 10)

  fn test_right_clip(to_cigar, position, expected) => {
    let mut cigar = super::CIGAR::new();
    cigar.update(to_cigar, position);
    assert_eq!(cigar.rclip, expected);
  }

  - _00 ("100M", 0, 0)
  - _01 ("23H57M10H", 0, 10)
  - _02 ("54M46S", 0, 46)
  - _03 ("10H3D2I80M5H", 0, 5)

  // TODO: add test boundries

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

// data_test! {

//   fn test_boundries(to_cigar, position, expected) => {
//     let mut cigar = super::CIGAR::loader(to_cigar);
//     assert_eq!(cigar.boundries(position), expected);
//   }

//   - _00 ("100M", 0, (0, 100))
//   - _01 ("54H40M6H", 198, (144, 238))
//   - _02 ("57S2M4I46M", 698, (641, 739))
//   - _03 ("10H3M2I80M5H", 748, (738, 833))

// }
// test
// BUG: fix boundries
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
    signature: String::from("100M"),
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
    signature: String::from("54H46M"),
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
    signature: String::from("54S46M"),
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
    signature: String::from("3H67M30H"),
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
    signature: String::from("10H3M2I80M5H"),
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
    signature: String::from("13H60D7M20H"),
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
    signature: String::from("50S4D6I40M"),
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
    signature: String::from("1H10D2M2D80M5H"),
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
    signature: String::from("1H10D2M2D80M5H"),
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
    signature: String::from("50S4D6I40M"),
  };
);

////////////////////////////////////////////////////////////////////////////////////////////////////
