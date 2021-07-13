////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEAnchor;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_me_anchor_loader(mobel, flag, pos, cigar, size, orient, expected) => {
    let fline = vec!["", flag, mobel, pos, "", cigar, "", "", ""];
    let test_anchor = super::MEAnchor::loader(&fline, size, orient);
    assert_eq!(test_anchor.mobel, expected.mobel);
    assert_eq!(test_anchor.size, expected.size);
    assert_eq!(test_anchor.flag, expected.flag);
    assert_eq!(test_anchor.pos, expected.pos);
    assert_eq!(test_anchor.cigar, expected.cigar);
    assert_eq!(test_anchor.orientation, expected.orientation);
  }

  - _00 ("me11", "75", "650", "100M", 1000., "FS5",
    super::MEAnchor{
      mobel: "me11".to_string(),
      size: 1000.,
      flag: 75,
      pos: 650,
      cigar: "100M".to_string(),
      orientation: "FS5".to_string(),
    }
  )

}

////////////////////////////////////////////////////////////////////////////////////////////////////
