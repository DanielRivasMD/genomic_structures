////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::interpret;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {
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

  fn test_interpret(flag, digit, expected) => {
    assert_eq!(super::interpret(flag, digit), expected);
  }
  - r_p_01 (1, 1, true)
  - r_p_02 (177, 1, true)
  - r_p_03 (72, 1, false)
////////////////////////////////////////////////////////////////////////////////////////////////////

  - r_mp_01 (2, 2, true)
  - r_mp_02 (135, 2, true)
  - r_mp_03 (177, 2, false)
// test
  - r_unmp_01 (4, 3, true)
  - r_unmp_02 (2165, 3, true)
  - r_unmp_03 (121, 3, false)
  - m_unmp_01 (8, 4, true)
  - m_unmp_02 (185, 4, true)
  - m_unmp_03 (177, 4, false)
  - r_rev_st_01 (16, 5, true)
  - r_rev_st_02 (157, 5, true)
  - r_rev_st_03 (169, 5, false)
  - m_rev_st_01 (32, 6, true)
  - m_rev_st_02 (99, 6, true)
  - m_rev_st_03 (147, 6, false)
  - f_pair_01 (64, 7, true)
  - f_pair_02 (73, 7, true)
  - f_pair_03 (133, 7, false)
  - s_pair_01 (128, 8, true)
  - s_pair_02 (457, 8, true)
  - s_pair_03 (73, 8, false)
  - not_pr_align_01 (256, 9, true)
  - not_pr_align_02 (389, 9, true)
  - not_pr_align_03 (133, 9, false)
  - r_fail_q_01 (512, 10, true)
  - r_fail_q_02 (901, 10, true)
  - r_fail_q_03 (329, 10, false)
  - r_pcr_op_dup_01 (1024, 11, true)
  - r_pcr_op_dup_02 (1157, 11, true)
  - r_pcr_op_dup_03 (133, 11, false)
  - suppl_alig_01 (2048, 12, true)
  - suppl_alig_02 (3095, 12, true)
  - suppl_alig_03 (99, 12, false)
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
