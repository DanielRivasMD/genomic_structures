
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_strcutures::CIGAR;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_alignment(to_cigar, expected) => {
    let cigar = super::CIGAR::loader(to_cigar);
    assert_eq!(cigar.align, expected);
  }

  - align_00 ("100M", vec![100])
  - align_01 ("3H67M20H", vec![67])
  - align_02 ("54S46M", vec![46])
  - align_03 ("10H3M2I80M5H", vec![3, 80])

  fn test_deletion(to_cigar, expected) => {
    let cigar = super::CIGAR::loader(to_cigar);
    assert_eq!(cigar.del, expected);
  }

  - deletion_00 ("100M", vec![])
  - deletion_01 ("3H60D7M20H", vec![60])
  - deletion_02 ("50S4D6I4M", vec![4])
  - deletion_03 ("1H10D2M2D80M5H", vec![10, 2])

  fn test_insertion(to_cigar, expected) => {
    let cigar = super::CIGAR::loader(to_cigar);
    assert_eq!(cigar.ins, expected);
  }

  - insertion_00 ("100M", vec![])
  - insertion_01 ("3H60I7M20H", vec![60])
  - insertion_02 ("59S4I46M", vec![4])
  - insertion_03 ("10H1I2M2I80M5H", vec![1, 2])

  fn test_left_clip(to_cigar, expected) => {
    let cigar = super::CIGAR::loader(to_cigar);
    assert_eq!(cigar.lclip, expected);
  }

  - left_clip_00 ("100M", 0)
  - left_clip_01 ("23H67M", 23)
  - left_clip_02 ("54S46M", 54)
  - left_clip_03 ("10H3D2I80M5H", 10)

  fn test_right_clip(to_cigar, expected) => {
    let cigar = super::CIGAR::loader(to_cigar);
    assert_eq!(cigar.rclip, expected);
  }

  - right_clip_00 ("100M", 0)
  - right_clip_01 ("23H57M10H", 10)
  - right_clip_02 ("54M46S", 46)
  - right_clip_03 ("10H3D2I80M5H", 5)

}

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_boundries(to_cigar, position, expected) => {
    let cigar = super::CIGAR::loader(to_cigar);
    assert_eq!(cigar.boundries(position), expected);
  }

  - boundries_00 ("100M", 0, (0, 100))
  - boundries_01 ("54H40M6H", 198, (144, 238))
  - boundries_02 ("57S2M4I46M", 698, (641, 739))
  - boundries_03 ("10H3M2I80M5H", 748, (738, 833))

}

////////////////////////////////////////////////////////////////////////////////////////////////////
