////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::{
  ExtraValuesEnum,
  OrientationEnum,
  RawValues,
  ReadControl,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// load & update
macro_rules! test_raw_values {
  ( $function: ident;
    params |> $fline: expr;
    expect |> $expect: expr;
  ) => {
    #[test]
    fn $function() {
      let loaded = RawValues::load($fline).expect("RawValues loading failed!");
      assert_eq!(
        loaded, $expect,
        "\n\nLoaded RawValues:\n{:#?}.\n\nExpected:\n{:#?}.\n\n",
        loaded, $expect,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_raw_values!(test01;
  params |> vec!["ID", "16", "scaffold", "1", "60", "100M", "", "", "100", "GATTACA", ""];
  expect |> RawValues{
    read_id: ReadControl{
      current: "ID".to_string(),
      previous: "".to_string(),
    },
    flag: 16,
    scaffold: "scaffold".to_string(),
    position: 1,
    quality: 60,
    cigar: CIGAR::load("100M", 1).expect("CIGAR loading failed!"),
    tlen: 100,
    sequence: "GATTACA".to_string(),
    orientation: OrientationEnum::None,
    extra: ExtraValuesEnum::None,
  };
);

////////////////////////////////////////////////////////////////////////////////////////////////////
