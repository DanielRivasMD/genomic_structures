////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use genomic_structures::MEChimericPair;
use genomic_structures::MEChimericRead;
use genomic_structures::ChrAnchorEnum;

////////////////////////////////////////////////////////////////////////////////////////////////////


data_test! {

  fn test_me_chimeric_read(sequence, expected) => {
    let mut toretrieve = super::MEChimericPair::new(super::ChrAnchorEnum::None);
    toretrieve.read1 = super::MEChimericRead::new();
    toretrieve.read1.sequence = sequence;
    let retrieved = toretrieve.chr_anchor_retriever();

    assert_eq!(retrieved.sequence, expected);
  }

  - _00 ("GATTACA".to_string(), "GATTACA".to_string())
  - _01 ("AAAAAAA".to_string(), "AAAAAAA".to_string())
  - _02 ("MACTHAA".to_string(), "MACTHAA".to_string())
  - _03 ("CAAGAAC".to_string(), "CAAGAAC".to_string())

}

////////////////////////////////////////////////////////////////////////////////////////////////////
