////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_strcutures::ChrAnchor;

////////////////////////////////////////////////////////////////////////////////////////////////////


data_test! {

  fn test_chr_anchor_loader(chr, flag, pos, cigar, mapq, tlen, expected) => {
    let fline = vec!["", flag, chr, pos, mapq, cigar, "", "", tlen];
    let test_anchor = super::ChrAnchor::loader(&fline);
    assert_eq!(test_anchor.chr, expected.chr);
    assert_eq!(test_anchor.flag, expected.flag);
    assert_eq!(test_anchor.pos, expected.pos);
    assert_eq!(test_anchor.cigar, expected.cigar);
    assert_eq!(test_anchor.mapq, expected.mapq);
    assert_eq!(test_anchor.tlen, expected.tlen);
  }

  - _00 ("chr7", "56", "2099", "100M", "100", "100",
    super::ChrAnchor{
      chr: "chr7".to_string(),
      flag: 56,
      pos: 2099,
      cigar: "100M".to_string(),
      mapq: 100,
      tlen: 100,
    }
  )
}

////////////////////////////////////////////////////////////////////////////////////////////////////
