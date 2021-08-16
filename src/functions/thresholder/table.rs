////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
use super::table;

////////////////////////////////////////////////////////////////////////////////////////////////////

// private function
macro_rules! table {
  ( $function: ident;
      params |> $psize: expr;
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
      let table = table(&bined_hm, $psize);

      assert_eq!(
        table, $expected,
        "\n\nCalculated table:\n{:#?}.\n\nExpected value:\n{:#?}.\n\n",
        table, $expected,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
table!(test01;
  params |> 5;
  keys |> [
    "100".to_string(),
    "200".to_string(),
    "300".to_string(),
    "400".to_string(),
    "500".to_string(),
    "600".to_string(),
  ];
  values |> [
    vec!["100.1".to_string(), "100.2".to_string(), ],
    vec!["200.1".to_string(), "200.2".to_string(), "200.3".to_string(), ],
    vec!["300.1".to_string(), ],
    vec!["400.1".to_string(), "400.2".to_string(), ],
    vec!["500.1".to_string(), "500.2".to_string(), "500.3".to_string(), ],
    vec!["600.1".to_string(), "600.2".to_string(), ],
  ];
  expected |> vec![1., 3., 2., 0., 0., ];
);

////////////////////////////////////////////////////////////////////////////////////////////////////
