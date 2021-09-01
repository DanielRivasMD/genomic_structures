////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
use super::table;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
macro_rules! test_table {
  ( $function: ident;
      params |> $psize: expr;
      values |> k >>> $keys: expr; v >>> $values: expr;
      expect |> $expect: expr;
    ) => {
    #[test]
    fn $function() {
      let mut bined_hm = std::collections::HashMap::new();
      for ix in 0..$keys.len() {
        bined_hm.insert($keys[ix], $values[ix].clone());
      }
      let table = table(&bined_hm, $psize);

      assert_eq!(
        table, $expect,
        "\n\nCalculated table:\n{:#?}.\n\nExpected value:\n{:#?}.\n\n",
        table, $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_table!(test01;
  params |> 5;
  values |>
    k >>> [
      100,
      200,
      300,
      400,
      500,
      600,
    ];
    v >>> [
      vec!["100.1".to_string(), "100.2".to_string(), ],
      vec!["200.1".to_string(), "200.2".to_string(), "200.3".to_string(), ],
      vec!["300.1".to_string(), ],
      vec!["400.1".to_string(), "400.2".to_string(), ],
      vec!["500.1".to_string(), "500.2".to_string(), "500.3".to_string(), ],
      vec!["600.1".to_string(), "600.2".to_string(), ],
    ];
  expect |> vec![1., 3., 2., 0., 0., ];
);

////////////////////////////////////////////////////////////////////////////////////////////////////
