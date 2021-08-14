////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;
// crate utilities
use genomic_structures::MEAnchor;
use genomic_structures::OrientationEnum;
use genomic_structures::CIGAR;

////////////////////////////////////////////////////////////////////////////////////////////////////

// test update & load macro
macro_rules! me_anchor {
  ( $function: ident; $assertion: ident;
    loaded |> $loaded_cigar: expr, $loaded_flag: expr, $loaded_mobel: expr, $loaded_orientation: expr, $loaded_position: expr, $loaded_size: expr;
    manual |> $manual_cigar: expr, $manual_flag: expr, $manual_mobel: expr, $manual_orientation: expr, $manual_position: expr, $manual_size: expr;
  ) => {
    #[test]
    fn $function() {
      let loaded = MEAnchor::load(
        CIGAR::load($loaded_cigar, $loaded_position).unwrap(),
        $loaded_flag,
        $loaded_mobel.clone(),
        $loaded_orientation,
        $loaded_position,
        $loaded_size,
      );

      let manual = MEAnchor {
        cigar:       CIGAR::load($manual_cigar, $manual_position).unwrap(),
        flag:        $manual_flag,
        mobel:       $manual_mobel.clone(),
        orientation: $manual_orientation,
        position:    $manual_position,
        size:        $manual_size,
      };

data_test! {

  // fn test_me_anchor_load(mobel, flag, pos, cigar, size, orient, expected) => {
  //   let fline = vec!["", flag, mobel, pos, "", cigar, "", "", ""];
  //   let test_anchor = super::MEAnchor::load(&fline, size, orient);
  //   assert_eq!(test_anchor.mobel, expected.mobel);
  //   assert_eq!(test_anchor.size, expected.size);
  //   assert_eq!(test_anchor.flag, expected.flag);
  //   assert_eq!(test_anchor.position, expected.position);
  //   assert_eq!(test_anchor.cigar, expected.cigar);
  //   assert_eq!(test_anchor.orientation, expected.orientation);
  // }

  // - _00 ("me11", "75", "650", "100M", 1000., "FS5",
  //   super::MEAnchor{
  //     mobel: "me11".to_string(),
  //     size: 1000.,
  //     flag: 75,
  //     position: 650,
  //     cigar: super::CIGAR::new(), //"100M".to_string(),
  //     orientation: "FS5".to_string(),
  //   }
  // )

      $assertion!(
        loaded,
        manual,
        "\n\nLoaded value:\n{:#?}.\n\nManual value:\n{:#?}.\n\n",
        loaded,
        manual,
      );
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
me_anchor!(test01; assert_eq;
  loaded |> "100M", 75, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
  manual |> "100M", 75, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
);

// fail
me_anchor!(fail01; assert_ne;
  loaded |> "100M", 75, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
  manual |> "100M", 95, String::from("mobel77"), OrientationEnum::None, 2099, 11000.;
);

////////////////////////////////////////////////////////////////////////////////////////////////////
