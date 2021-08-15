////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::threshold;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_threshold(read_count, chr_size, fdr, hm_keys, hm_vals, expected) => {
    let mut bined_hm = std::collections::HashMap::new();
    for ix in 0..hm_keys.len() {
      bined_hm.insert(hm_keys[ix].clone(), hm_vals[ix].clone());
macro_rules! threshold {
  ( $function: ident;
  params |> $read_count: expr, $scaffold_size: expr, $fdr: expr;
  keys |> $keys: expr;
  values |> $values: expr;
  expected |> $expected: expr;
) => {
    #[test]
    fn $function() {
      let mut bined_hm = std::collections::HashMap::new();
      for ix in 0..$keys.len() {
        bined_hm.insert($keys[ix].clone(), $values[ix].clone());
      }

      let threshold25 = threshold(
        $read_count,
        $scaffold_size,
        $fdr,
        &bined_hm,
        25
      );
      assert_eq!(
        threshold25,
        $expected,
        "\n\nRead count: {:?}.\nScaffold size: {:?}.\nFalse discovery rate (FDR): {:?}.\nBinned HashMap:\n{:#?}.\n\nExpected: {:?}.\n\n",
        $read_count,
        $scaffold_size,
        $fdr,
        &bined_hm,
        $expected,
      );
    }
    assert_eq!(super::threshold(read_count, chr_size, fdr, &bined_hm, 25), expected);
  }
  };
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
////////////////////////////////////////////////////////////////////////////////////////////////////

}
// test
threshold!(test01;
  params |> 6., 1000., 0.001;
  keys |> vec![ String::from("100"), String::from("200"), String::from("300"), String::from("400"), String::from("500"), String::from("600") ];
  values |> [
    vec![String::from("100.1"), String::from("100.2")],
    vec![String::from("200.1"), String::from("200.2"), String::from("200.3")],
    vec![String::from("300.1"), String::from("300.2")],
    vec![String::from("400.1"), String::from("400.2")],
    vec![String::from("500.1"), String::from("500.2"), String::from("500.3")],
    vec![String::from("600.1"), String::from("600.2")],
  ];
  expected |> 5;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
