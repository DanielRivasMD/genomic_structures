////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::interpret;

////////////////////////////////////////////////////////////////////////////////////////////////////

// public function
macro_rules! interpret {
  ( $function: ident;
    params |> $flag: expr, $digit: expr;
    expected |> $expected: expr;
  ) => {
    #[test]
    fn $function() {
      let interpretation = interpret($flag, $digit);
      assert_eq!(
        interpretation,
        $expected,
        "\n\nFlag: {:?}.\nDigit: {:?}.\nInterpretation {:?}.\nExpected: {:?}.\n\n",
        $flag,
        $digit,
        interpretation,
        $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
interpret!(read_pair00; params |> 72, 1; expected |> false;);
interpret!(read_pair01; params |> 1, 1; expected |> true;);
interpret!(read_pair02; params |> 177, 1; expected |> true;);

interpret!(mate_pair00; params |> 177, 2; expected |> false;);
interpret!(mate_pair01; params |> 2, 2; expected |> true;);
interpret!(mate_pair02; params |> 135, 2; expected |> true;);

interpret!(read_unpair00; params |> 121, 3; expected |> false;);
interpret!(read_unpair01; params |> 4, 3; expected |> true;);
interpret!(read_unpair02; params |> 2165, 3; expected |> true;);

interpret!(mate_unpair00; params |> 177, 4; expected |> false;);
interpret!(mate_unpair01; params |> 8, 4; expected |> true;);
interpret!(mate_unpair02; params |> 185, 4; expected |> true;);

interpret!(read_rev_st00; params |> 169, 5; expected |> false;);
interpret!(read_rev_st01; params |> 16, 5; expected |> true;);
interpret!(read_rev_st02; params |> 157, 5; expected |> true;);

interpret!(mate_rev_st00; params |> 147, 6; expected |> false;);
interpret!(mate_rev_st01; params |> 32, 6; expected |> true;);
interpret!(mate_rev_st02; params |> 99, 6; expected |> true;);

interpret!(first_pair00; params |> 133, 7; expected |> false;);
interpret!(first_pair01; params |> 64, 7; expected |> true;);
interpret!(first_pair02; params |> 73, 7; expected |> true;);

interpret!(second_pair00; params |> 73, 8; expected |> false;);
interpret!(second_pair01; params |> 128, 8; expected |> true;);
interpret!(second_pair02; params |> 457, 8; expected |> true;);

interpret!(not_primary00; params |> 133, 9; expected |> false;);
interpret!(not_primary01; params |> 256, 9; expected |> true;);
interpret!(not_primary02; params |> 389, 9; expected |> true;);

interpret!(fail_quality00; params |> 329, 10; expected |> false;);
interpret!(fail_quality01; params |> 512, 10; expected |> true;);
interpret!(fail_quality02; params |> 901, 10; expected |> true;);

interpret!(pcr_optical_dup00; params |> 133, 11; expected |> false;);
interpret!(pcr_optical_dup01; params |> 1024, 11; expected |> true;);
interpret!(pcr_optical_dup02; params |> 1157, 11; expected |> true;);

interpret!(supplementary00; params |> 99, 12; expected |> false;);
interpret!(supplementary01; params |> 2048, 12; expected |> true;);
interpret!(supplementary02; params |> 3095, 12; expected |> true;);

////////////////////////////////////////////////////////////////////////////////////////////////////
