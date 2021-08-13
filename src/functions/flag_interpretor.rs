////////////////////////////////////////////////////////////////////////////////////////////////////

/// Binary interpretor for SAM flags.
///
/// Interpretation is carried out as follows:
/// 1) read paired.
/// 2) read mapped in proper pair.
/// 3) read unmapped.
/// 4) mate unmapped.
/// 5) read reverse strand.
/// 6) mate reverse strand.
/// 7) first in pair.
/// 8) second in pair.
/// 9) not primary alignment.
/// 10) read fails platform/vendor quality checks.
/// 11) read is PCR or optical duplicate.
/// 12) supplementary alignment.
///
/// # Examples
///
/// ```
/// use genomic_structures::interpret;
///
/// assert_eq!(interpret(177, 1), true);
/// assert_eq!(interpret(177, 2), false);
/// assert_eq!(interpret(2165, 3), true);
/// assert_eq!(interpret(133, 7), false);
/// assert_eq!(interpret(157, 5), true);
/// ```
pub fn interpret(
  n: i32,
  p: usize,
) -> bool {
  let bin_n: String = format!("{:b}", n).chars().rev().collect();
  let mut stat_array = ['0'; 12];
  for i in bin_n.char_indices() {
    stat_array[i.0] = i.1;
  }
  stat_array[p - 1] == '1'
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// // test private functions
// #[cfg(test)]
// mod priv_tests {
//   use crate::structures::chr_anchor::ChrAnchor;
//   use super::SAMFlag;

//   use data_test::data_test;

//   data_test! {

//     fn test_chr_interpret(chr_anchor, value, expected) => {
//       assert_eq!(chr_anchor.interpret(value), expected);
//     }

//
//     - _00 ( super::ChrAnchor { chr:   "chr7".to_string(), flag:  177, pos:
//       2099, cigar: "100M".to_string(), mapq:  100, tlen:  100, }, 1, true
//     )
//   }
// }
