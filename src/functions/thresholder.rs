////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use std::collections::{HashMap};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{BIN_SIZE, BIN_OVERLAP};

////////////////////////////////////////////////////////////////////////////////////////////////////

// C bindings
use libc::{
  c_int,
  c_double,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


#[link(name="Rmath")]
extern {
// R function signature declaration
  fn ppois(
    x: c_double,
    lambda: c_double,
    lower_tail: c_int,
    log_p: c_int
  ) -> c_double;
}


fn effective_genome_length_calculator(
  genome_length: f64,
  bin_size: f64,
  bin_overlap: f64,
) -> f64 {
  genome_length * bin_size / bin_overlap
}

fn lambda_calculator(
  pop_reads: f64,
  eff_genome_length: f64,
  bin_size: f64,
) -> f64 {
  pop_reads * bin_size / eff_genome_length
}

fn r_ppoisson(
  lambda: f64,
  psize: usize,
) -> Vec<f64> {
  let mut ppois_vec = vec![0.; psize];
  for ppois_index in 1..=psize {
    // fix lower_tail = TRUE & log_p = FALSE
    unsafe {
      ppois_vec[ppois_index - 1] = 1. - ppois(ppois_index as f64, lambda, 1, 0);
    }
  }
  ppois_vec
}

fn tabler(
  bined_hm: &HashMap<String, Vec<String>>,
  psize: usize,
) -> Vec<f64> {
  let mut out_vec = vec![0.; psize];
  for (_, i) in bined_hm.iter() {
    let length_count = i.len();
    if length_count < psize {
      out_vec[length_count - 1] += 1.;
    }
  }
  out_vec
}

fn cumsum(
  mut cum_vec: Vec<f64>,
) -> Vec<f64> {
  let mut cumulus = 0.;
  for cix in &mut cum_vec {
    cumulus += *cix;
    *cix = cumulus;
  }
  cum_vec
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Obtain a threshold according to parameters.
///
/// Parameters:
/// 1) Number of reads
/// 2) Scaffold / chromosome size
/// 3) False discovery tolerance
/// 4) Read positioning, i.e., pill up
///
/// # Examples
///
/// ```
/// use genomic_strcutures::thresholder;
/// let ks = vec![
///   "100".to_string(),
///   "200".to_string(),
///   "300".to_string(),
///   "400".to_string(),
///   "500".to_string(),
///   "600".to_string(),
/// ];
/// let vs = vec![
///   vec!["100.1".to_string(), "100.2".to_string()],
///   vec![
///     "200.1".to_string(),
///     "200.2".to_string(),
///     "200.3".to_string(),
///   ],
///   vec!["300.1".to_string(), "300.".to_string()],
///   vec!["400.1".to_string(), "400.2".to_string()],
///   vec![
///     "500.1".to_string(),
///     "500.2".to_string(),
///     "500.3".to_string(),
///   ],
///   vec!["600.1".to_string(), "600.2".to_string()],
/// ];
/// let mut hm = std::collections::HashMap::new();
/// for ix in 0..ks.len() {
///   hm.insert(ks[ix].clone(), vs[ix].clone());
/// }
/// assert_eq!(thresholder(6., 1000., 0.001, &hm, 25), 5)
/// ```
pub fn thresholder(
  pop_reads: f64,
  chromosome_size: f64,
  false_discovery_tolerance: f64,
  read_hm: &HashMap<String, Vec<String>>,
  psize: usize,
) -> usize {
  let eff_genome_length = effective_genome_length_calculator!(chromosome_size);
  let lambda = lambda_calculator!(pop_reads, eff_genome_length);
  let p_values = r_ppoisson(lambda, psize);

  let mut peak_prob = vec![0.; psize];
  for (ix, p_val) in p_values.iter().enumerate() {
    peak_prob[ix] = p_val * chromosome_size;
  }

  let bin_tb = tabler(read_hm, psize);
  let cum_bin_tb = cumsum(bin_tb);
  let mut false_disc_values = vec![0.; psize];
  for ix in 0..psize {
    if cum_bin_tb[ix] == 0. {
      false_disc_values[ix] = 1.;
    } else {
      false_disc_values[ix] = peak_prob[ix] / cum_bin_tb[ix];
    }
  }

  let mut threshold = 0;
  for (ix, fd_val) in false_disc_values.iter().enumerate() {
    if *fd_val < false_discovery_tolerance {
      threshold = ix + 1;
      break
    }
  }
  threshold
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test private functions
#[cfg(test)]
mod priv_tests {
  use data_test::data_test;
  use crate::{BIN_SIZE, BIN_OVERLAP};
  use super::{
    ppois,
    effective_genome_length_calculator,
    lambda_calculator,
    r_ppoisson,
    tabler,
    cumsum,
  };

  data_test! {
    // precission point => 20

    // test R ppois bindings
    fn test_ppois(sample, lambda, expected) => {
      let pois_vec: Vec<f64> = sample
        .iter()
        .map(|ix| unsafe {
            super::ppois(*ix, lambda, 1, 0)
          } )
        .collect();
      assert_eq!(pois_vec, expected)
    }

    - _00 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 0., vec![1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, ])
    - _01 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 1., vec![0.367_879_441_171_442_33, 0.735_758_882_342_884_7, 0.919_698_602_928_605_8, 0.981_011_843_123_846_2, 0.996_340_153_172_656_3, 0.999_405_815_182_418_3, 0.999_916_758_850_712, 0.999_989_750_803_325_3, 0.999_998_874_797_402, 0.999_999_888_574_521_7, 0.999_999_989_952_233_6, ])
    - _02 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 2., vec![0.135_335_283_236_612_7, 0.406_005_849_709_838_1, 0.676_676_416_183_063_4, 0.857_123_460_498_546_9, 0.947_346_982_656_288_9, 0.983_436_391_519_385_6, 0.995_466_194_473_751_2, 0.998_903_281_032_141_3, 0.999_762_552_671_738_9, 0.999_953_501_924_982_8, 0.999_991_691_775_631_5, ])
    - _03 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 3., vec![0.049_787_068_367_863_944, 0.199_148_273_471_455_8, 0.423_190_081_126_843_53, 0.647_231_888_782_231_1, 0.815_263_244_523_772, 0.916_082_057_968_696_6, 0.966_491_464_691_158_8, 0.988_095_496_143_642_7, 0.996_197_007_938_324, 0.998_897_511_869_884_5, 0.999_707_663_049_352_7, ])
    - _04 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 4., vec![0.018_315_638_888_734_18, 0.091_578_194_443_670_93, 0.238_103_305_553_544_36, 0.433_470_120_366_709, 0.628_836_935_179_873_5, 0.785_130_387_030_405_2, 0.889_326_021_597_426_2, 0.948_866_384_207_152_6, 0.978_636_565_512_015_8, 0.991_867_757_203_066_1, 0.997_160_233_879_486_3, ])
    - _05 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 5., vec![0.006_737_946_999_085_467, 0.040_427_681_994_512_805, 0.124_652_019_483_081_16, 0.265_025_915_297_361_8, 0.440_493_285_065_212_3, 0.615_960_654_833_063_2, 0.762_183_462_972_938_7, 0.866_628_325_929_992_6, 0.931_906_365_278_151_5, 0.968_171_942_693_795_1, 0.986_304_731_401_617_1, ])
    - _06 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 6., vec![0.002_478_752_176_666_358_5, 0.017_351_265_236_664_505, 0.061_968_804_416_658_974, 0.151_203_882_776_647_84, 0.285_056_500_316_631_27, 0.445_679_641_364_611_3, 0.606_302_782_412_591_2, 0.743_979_760_453_717, 0.847_237_493_984_561, 0.916_075_983_005_124_3, 0.957_379_076_417_461_9, ])
    - _07 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 7., vec![0.000_911_881_965_554_516_2, 0.007_295_055_724_436_133, 0.029_636_163_880_521_784, 0.081_765_416_244_721_65, 0.172_991_607_882_071_35, 0.300_708_276_174_360_97, 0.449_711_055_848_698_8, 0.598_713_835_523_036_8, 0.729_091_267_738_082_3, 0.830_495_937_238_673_5, 0.901_479_205_889_087_3, ])
    - _08 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 8., vec![0.000_335_462_627_902_511_85, 0.003_019_163_651_122_608_6, 0.013_753_967_744_002_994, 0.042_380_111_991_684_004, 0.099_632_400_487_046_05, 0.191_236_062_079_625_2, 0.313_374_277_536_397_57, 0.452_960_809_486_994_6, 0.592_547_341_437_591_2, 0.716_624_258_727_010_9, 0.815_885_792_558_546_3, ])
    - _09 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 9., vec![0.000_123_409_804_086_679_56, 0.001_234_098_040_866_797, 0.006_232_195_106_377_316, 0.021_226_486_302_908_888, 0.054_963_641_495_104_916, 0.115_690_520_841_057_73, 0.206_780_839_859_987_08, 0.323_896_964_312_895_94, 0.455_652_604_322_418_8, 0.587_408_244_331_941_3, 0.705_988_320_340_511_7, ])
    - _10 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 10., vec![4.539_992_976_248_485_4e-5, 0.000_499_399_227_387_333_3, 0.002_769_395_715_511_576, 0.010_336_050_675_925_728, 0.029_252_688_076_961_082, 0.067_085_962_879_031_8, 0.130_141_420_882_483_07, 0.220_220_646_601_699_07, 0.332_819_678_750_718_8, 0.457_929_714_471_852_27, 0.583_039_750_192_985_4, ])

    fn test_effective_genome_length_calculator(glen, expected) => {
      assert_eq!(super::effective_genome_length_calculator(glen, super::BIN_SIZE as f64, super::BIN_OVERLAP as f64), expected)
    }

    - _00 (2000., 4000., )
    - _01 (3243556456., 6487112912., )

    fn test_lambda_calculator(preads, eflen, expected) => {
      assert_eq!(super::lambda_calculator(preads, eflen, super::BIN_SIZE as f64), expected)
    }

    - _00 (100., 400., 25., )
    - _01 (5050., 75400000., 0.006_697_612_732_095_490_5, )

    // test inverted probability poisson function
    fn test_r_poisson(lambda, psize, expected) => {
      assert_eq!(super::r_ppoisson(lambda, psize), expected)
    }

    - _00 (0., 20, vec![0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, ])
    - _01 (1., 20, vec![0.264_241_117_657_115_33, 0.080_301_397_071_394_16, 0.018_988_156_876_153_85, 0.003_659_846_827_343_66, 0.000_594_184_817_581_666_6, 8.324_114_928_803_805e-5, 1.024_919_667_469_426_1e-5, 1.125_202_597_984_653_6e-6, 1.114_254_782_885_737_4e-7, 1.004_776_639_668_136_8e-8, 8.316_107_802_386_341e-10, 6.359_779_369_802_254e-11, 4.519_828_955_551_475e-12, 2.999_822_612_537_173e-13, 1.865_174_681_370_263e-14, 1.110_223_024_625_156_5e-15, 1.110_223_024_625_156_5e-16, 0.0000000000000000000, 0.0000000000000000000, 0.0000000000000000000, ])
    - _02 (2., 20, vec![0.593_994_150_290_161_9, 0.323_323_583_816_936_6, 0.142_876_539_501_453_07, 0.052_653_017_343_711_084, 0.016_563_608_480_614_445, 0.004_533_805_526_248_824, 0.001_096_718_967_858_678_6, 0.000_237_447_328_261_142_23, 4.649_807_501_722_058_6e-5, 8.308_224_368_480_666e-6, 1.364_615_159_649_140_7e-6, 2.073_469_581_587_161_8e-7, 2.930_569_642_511_926_5e-8, 3.871_230_447_316_520_4e-9, 4.799_682_873_368_738e-10, 5.606_048_958_384_235_4e-11, 6.189_049_273_075_398e-12, 6.477_041_125_663_163e-13, 6.439_293_542_825_908e-14, 6.106_226_635_438_361e-15, ])
    - _03 (3., 20, vec![0.800_851_726_528_544_2, 0.576_809_918_873_156_5, 0.352_768_111_217_768_85, 0.184_736_755_476_228_03, 0.083_917_942_031_303_43, 0.033_508_535_308_841_236, 0.011_904_503_856_357_329, 0.003_802_992_061_676_002_7, 0.001_102_488_130_115_486_5, 0.000_292_336_950_647_342_8, 7.138_662_897_421_266e-5, 1.614_904_855_595_789e-5, 3.401_914_613_232_470_7e-6, 6.703_859_112_278_11e-7, 1.240_801_708_046_746e-7, 2.164_784_451_696_988_3e-8, 3.571_551_610_015_433e-9, 5.588_361_995_378_932e-10, 8.314_426_924_727_059e-11, 1.179_045_749_921_67e-11, ])
    - _04 (4., 20, vec![0.908_421_805_556_329_1, 0.761_896_694_446_455_6, 0.566_529_879_633_290_9, 0.371_163_064_820_126_5, 0.214_869_612_969_594_84, 0.110_673_978_402_573_76, 0.051_133_615_792_847_364, 0.021_363_434_487_984_168, 0.008_132_242_796_933_92, 0.002_839_766_120_513_731_5, 0.000_915_229_147_270_046_9, 0.000_273_716_822_855_485_4, 7.632_841_534_332_968e-5, 1.993_172_748_271_376_8e-5, 4.892_610_719_897_661e-6, 1.132_831_529_138_123e-6, 2.481_776_019_136_461_3e-7, 5.158_784_033_287_844e-8, 1.020_052_209_366_184_6e-8, 1.923_058_490_227_447_2e-9, ])
    - _05 (5., 20, vec![0.959_572_318_005_487_1, 0.875_347_980_516_918_9, 0.734_974_084_702_638_3, 0.559_506_714_934_787_7, 0.384_039_345_166_936_83, 0.237_816_537_027_061_3, 0.133_371_674_070_007_38, 0.068_093_634_721_848_48, 0.031_828_057_306_204_86, 0.013_695_268_598_382_881, 0.005_453_091_913_009_356, 0.002_018_851_627_437_090_5, 0.000_697_989_979_139_945_8, 0.000_226_253_676_176_790_82, 6.900_824_185_562_815e-5, 1.986_904_363_038_277_7e-5, 5.416_338_270_003_429e-6, 1.401_697_892_089_437_4e-6, 3.452_135_820_536_384e-7, 8.109_250_460_019_979e-8, ])
    - _06 (6., 20, vec![0.982_648_734_763_335_5, 0.938_031_195_583_341, 0.848_796_117_223_352_1, 0.714_943_499_683_368_8, 0.554_320_358_635_388_6, 0.393_697_217_587_408_83, 0.256_020_239_546_282_95, 0.152_762_506_015_438_95, 0.083_924_016_994_875_73, 0.042_620_923_582_538_106, 0.020_091_963_539_444_757, 0.008_827_483_517_898_194, 0.003_628_492_738_722_788_3, 0.001_400_353_833_361_900_3, 0.000_509_098_271_217_589_5, 0.000_174_877_435_413_445_25, 5.691_714_042_377_338e-5, 1.759_704_209_380_874_6e-5, 5.180_168_937_024_554e-6, 1.455_106_990_011_501e-6, ])
    - _07 (7., 20, vec![0.992_704_944_275_563_9, 0.970_363_836_119_478_2, 0.918_234_583_755_278_3, 0.827_008_392_117_928_6, 0.699_291_723_825_639_1, 0.550_288_944_151_301_3, 0.401_286_164_476_963_2, 0.270_908_732_261_917_7, 0.169_504_062_761_326_46, 0.098_520_794_110_912_75, 0.053_349_623_151_558_67, 0.026_999_773_425_268_714, 0.012_811_392_803_420_252, 0.005_717_202_492_496_076, 0.002_406_580_347_398_046_3, 0.000_958_183_158_917_713_7, 0.000_361_784_316_602_276_06, 0.000_129_851_433_479_544_2, 4.440_247_653_958_451e-5, 1.449_534_161_068_744e-5, ])
    - _08 (8., 20, vec![0.996_980_836_348_877_4, 0.986_246_032_255_997, 0.957_619_888_008_316, 0.900_367_599_512_954, 0.808_763_937_920_374_8, 0.686_625_722_463_602_4, 0.547_039_190_513_005_4, 0.407_452_658_562_408_75, 0.283_375_741_272_989_15, 0.184_114_207_441_453_68, 0.111_924_001_018_518_52, 0.063_797_196_736_561_92, 0.034_180_701_793_819_38, 0.017_256_990_397_966_465, 0.008_231_010_986_844_867, 0.003_718_021_281_284_178_4, 0.001_594_261_419_843_756_6, 0.000_650_368_148_092_495, 0.000_252_939_402_091_922_9, 9.396_790_369_176_067e-5, ])
    - _09 (9., 20, vec![0.998_765_901_959_133_2, 0.993_767_804_893_622_7, 0.978_773_513_697_091_1, 0.945_036_358_504_895_1, 0.884_309_479_158_942_3, 0.793_219_160_140_012_9, 0.676_103_035_687_104_1, 0.544_347_395_677_581_3, 0.412_591_755_668_058_7, 0.294_011_679_659_488_27, 0.196_991_617_470_657_4, 0.124_226_570_829_035_32, 0.073_850_769_307_911_79, 0.041_466_325_472_903_74, 0.022_035_659_171_898_98, 0.011_105_909_377_583_822, 0.005_319_571_251_181_654, 0.002_426_402_187_980_514, 0.001_055_953_684_358_956_8, 0.000_439_251_857_729_305_86, ])
    - _10 (10., 20, vec![0.999_500_600_772_612_7, 0.997_230_604_284_488_4, 0.989_663_949_324_074_3, 0.970_747_311_923_038_9, 0.932_914_037_120_968_2, 0.869_858_579_117_517, 0.779_779_353_398_301, 0.667_180_321_249_281_3, 0.542_070_285_528_147_7, 0.416_960_249_807_014_6, 0.303_223_853_696_893_4, 0.208_443_523_605_125_75, 0.135_535_577_380_688_78, 0.083_458_472_934_663_02, 0.048_740_403_303_978_66, 0.027_041_609_784_801_08, 0.014_277_613_597_049_599, 0.007_186_504_603_854_282, 0.003_454_341_975_856_811_7, 0.001_588_260_661_858_020_8, ])

    fn test_tabler(hm_keys, hm_vals, psize, expected) => {
      let mut bined_hm = std::collections::HashMap::new();
      for ix in 0..hm_keys.len() {
        bined_hm.insert(hm_keys[ix].clone(), hm_vals[ix].clone());
      }
      assert_eq!(super::tabler(&bined_hm, psize), expected)
    }

    - _00 (
      [
        "100".to_string(),
        "200".to_string(),
        "300".to_string(),
        "400".to_string(),
        "500".to_string(),
        "600".to_string(),
      ],
      [
        vec!["100.1".to_string(), "100.2".to_string(), ],
        vec!["200.1".to_string(), "200.2".to_string(), "200.3".to_string(), ],
        vec!["300.1".to_string(), ],
        vec!["400.1".to_string(), "400.2".to_string(), ],
        vec!["500.1".to_string(), "500.2".to_string(), "500.3".to_string(), ],
        vec!["600.1".to_string(), "600.2".to_string(), ],
      ],
      5,
      vec![1., 3., 2., 0., 0., ]
    )

    fn test_cumsum(cum_vec, expected) => {
      assert_eq!(super::cumsum(cum_vec), expected)
    }

    - _00 (vec![0., 1., 2., 3., 4., ], vec![0., 1., 3., 6., 10., ])
    - _01 (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], vec![0., 1., 3., 6., 10., 15., 21., 28., 36., 45., 55., ])
    - _02 (vec![10., 9., 8., 7., 6., 5., 4., 3., 2., 1., 0., ], vec![10., 19., 27., 34., 40., 45., 49., 52., 54., 55., 55., ])

  }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
