////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::interpret;

////////////////////////////////////////////////////////////////////////////////////////////////////

// public function
macro_rules! interpret {
  ( $function: ident;
    params |> $flag: expr, $digit: expr;
    expect |> $expect: expr;
  ) => {
    #[test]
    fn $function() {
      let interpretation = interpret($flag, $digit);
      assert_eq!(
        interpretation,
        $expect,
        "\n\nFlag: {:?}.\nDigit: {:?}.\nInterpretation {:?}.\nExpected: {:?}.\n\n",
        $flag,
        $digit,
        interpretation,
        $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
interpret!(read_pair00;
  params |> 72, 1;
  expect |> false;
);

interpret!(read_pair01;
  params |> 1, 1;
  expect |> true;
);

interpret!(read_pair02;
  params |> 177, 1;
  expect |> true;
);

interpret!(mate_pair00;
  params |> 177, 2;
  expect |> false;
);

interpret!(mate_pair01;
  params |> 2, 2;
  expect |> true;
);

interpret!(mate_pair02;
  params |> 135, 2;
  expect |> true;
);

interpret!(read_unpair00;
  params |> 121, 3;
  expect |> false;
);

interpret!(read_unpair01;
  params |> 4, 3;
  expect |> true;
);

interpret!(read_unpair02;
  params |> 2165, 3;
  expect |> true;
);

interpret!(mate_unpair00;
  params |> 177, 4;
  expect |> false;
);

interpret!(mate_unpair01;
  params |> 8, 4;
  expect |> true;
);

interpret!(mate_unpair02;
  params |> 185, 4;
  expect |> true;
);

interpret!(read_rev_st00;
  params |> 169, 5;
  expect |> false;
);

interpret!(read_rev_st01;
  params |> 16, 5;
  expect |> true;
);

interpret!(read_rev_st02;
  params |> 157, 5;
  expect |> true;
);

interpret!(mate_rev_st00;
  params |> 147, 6;
  expect |> false;
);

interpret!(mate_rev_st01;
  params |> 32, 6;
  expect |> true;
);

interpret!(mate_rev_st02;
  params |> 99, 6;
  expect |> true;
);

interpret!(first_pair00;
  params |> 133, 7;
  expect |> false;
);

interpret!(first_pair01;
  params |> 64, 7;
  expect |> true;
);

interpret!(first_pair02;
  params |> 73, 7;
  expect |> true;
);

interpret!(second_pair00;
  params |> 73, 8;
  expect |> false;
);

interpret!(second_pair01;
  params |> 128, 8;
  expect |> true;
);

interpret!(second_pair02;
  params |> 457, 8;
  expect |> true;
);

interpret!(not_primary00;
  params |> 133, 9;
  expect |> false;
);

interpret!(not_primary01;
  params |> 256, 9;
  expect |> true;
);

interpret!(not_primary02;
  params |> 389, 9;
  expect |> true;
);

interpret!(fail_quality00;
  params |> 329, 10;
  expect |> false;
);

interpret!(fail_quality01;
  params |> 512, 10;
  expect |> true;
);

interpret!(fail_quality02;
  params |> 901, 10;
  expect |> true;
);

interpret!(pcr_optical_dup00;
  params |> 133, 11;
  expect |> false;
);

interpret!(pcr_optical_dup01;
  params |> 1024, 11;
  expect |> true;
);

interpret!(pcr_optical_dup02;
  params |> 1157, 11;
  expect |> true;
);

interpret!(supplementary00;
  params |> 99, 12;
  expect |> false;
);

interpret!(supplementary01;
  params |> 2048, 12;
  expect |> true;
);

interpret!(supplementary02;
  params |> 3095, 12;
  expect |> true;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
