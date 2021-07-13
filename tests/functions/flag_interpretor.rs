////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::interpretor;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_interpretor(flag, digit, expected) => {
    assert_eq!(super::interpretor(flag, digit), expected);
  }
  - r_p_01 (1, 1, true)
  - r_p_02 (177, 1, true)
  - r_p_03 (72, 1, false)

  - r_mp_01 (2, 2, true)
  - r_mp_02 (135, 2, true)
  - r_mp_03 (177, 2, false)

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

}

////////////////////////////////////////////////////////////////////////////////////////////////////
