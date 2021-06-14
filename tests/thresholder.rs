////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_strcutures::thresholder;

////////////////////////////////////////////////////////////////////////////////////////////////////


data_test! {

  fn test_thresholder(read_count, chr_size, fdr, hm_keys, hm_vals, expected) => {
    let mut bined_hm = std::collections::HashMap::new();
    for ix in 0..hm_keys.len() {
      bined_hm.insert(hm_keys[ix].clone(), hm_vals[ix].clone());
    }
    assert_eq!(super::thresholder(read_count, chr_size, fdr, &bined_hm, 25), expected)
  }

  - _00 (6., 1000., 0.001,
      [
        "100".to_string(),
        "200".to_string(),
        "300".to_string(),
        "400".to_string(),
        "500".to_string(),
        "600".to_string(),
      ],
      [
        vec!["100.1".to_string(), "100.2".to_string(), ],
        vec!["200.1".to_string(), "200.2".to_string(), "200.3".to_string(), ],
        vec!["300.1".to_string(), ],
        vec!["400.1".to_string(), "400.2".to_string(), ],
        vec!["500.1".to_string(), "500.2".to_string(), "500.3".to_string(), ],
        vec!["600.1".to_string(), "600.2".to_string(), ],
      ],
      5
      )

}

////////////////////////////////////////////////////////////////////////////////////////////////////
