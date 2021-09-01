////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::threshold;

////////////////////////////////////////////////////////////////////////////////////////////////////

// public function
macro_rules! test_threshold {
  ( $function: ident;
    params |> $read_count: expr, $scaffold_size: expr, $fdr: expr;
    values |> k >>> $keys: expr; v >>> $values: expr;
    expect |> $expect: expr;
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
        $expect,
        "\n\nRead count: {:?}.\nScaffold size: {:?}.\nFalse discovery rate (FDR): {:?}.\nBinned HashMap:\n{:#?}.\n\nExpected: {:?}.\n\n",
        $read_count,
        $scaffold_size,
        $fdr,
        &bined_hm,
        $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_threshold!(test01;
  params |> 6., 1000., 0.001;
  values |>
    k >>> vec![ 100, 200, 300, 400, 500, 600, ];
    v >>> [
      vec!["100.1".to_string(), "100.2".to_string(), ],
      vec!["200.1".to_string(), "200.2".to_string(), "200.3".to_string(), ],
      vec!["300.1".to_string(), "300.2".to_string(), ],
      vec!["400.1".to_string(), "400.2".to_string(), ],
      vec!["500.1".to_string(), "500.2".to_string(), "500.3".to_string(), ],
      vec!["600.1".to_string(), "600.2".to_string(), ],
    ];
  expect |> 5;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
