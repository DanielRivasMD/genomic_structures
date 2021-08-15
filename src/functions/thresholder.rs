////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  BIN_OVERLAP,
  BIN_SIZE,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// C bindings
use libc::{
  c_double,
  c_int,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Obtain a threshold according to parameters.
///
/// Parameters:
/// 1) Number of reads.
/// 2) Scaffold / chromosome size.
/// 3) False discovery tolerance.
/// 4) Read positioning, i.e., pill up.
///
/// # Examples
///
/// ```
/// use genomic_structures::threshold;
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
/// assert_eq!(threshold(6., 1000., 0.001, &hm, 25), 5);
/// ```
pub fn threshold(
  pop_reads: f64,
  chromosome_size: f64,
  false_discovery_tolerance: f64,
  read_hm: &HashMap<String, Vec<String>>,
  psize: usize,
) -> usize {
  let eff_genome_length = effective_genome_length_calculate!(chromosome_size);
  let lambda = lambda_calculate!(pop_reads, eff_genome_length);
  let p_values = r_ppoisson(lambda, psize);

  let mut peak_prob = vec![0.; psize];
  for (ix, p_val) in p_values.iter().enumerate() {
    peak_prob[ix] = p_val * chromosome_size;
  }

  let bin_tb = table(read_hm, psize);
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
      break;
    }
  }
  threshold
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// R function signature declaration
#[link(name = "Rmath")]
extern "C" {
  fn ppois(
    x: c_double,
    lambda: c_double,
    lower_tail: c_int,
    log_p: c_int,
  ) -> c_double;
}

// TODO: rename
// effective genome / chromosome / scaffold length
fn effective_genome_length_calculate(
  genome_length: f64,
  bin_size: f64,
  bin_overlap: f64,
) -> f64 {
  genome_length * bin_size / bin_overlap
}

// Poisson lambda λ
fn lambda_calculate(
  pop_reads: f64,
  eff_genome_length: f64,
  bin_size: f64,
) -> f64 {
  pop_reads * bin_size / eff_genome_length
}

// Poisson distribution
fn r_ppoisson(
  lambda: f64,
  psize: usize,
) -> Vec<f64> {
  let mut ppois_vec = vec![0.; psize];
  for ppois_index in 1..=psize {
    // fixed lower_tail = TRUE & log_p = FALSE
    unsafe {
      ppois_vec[ppois_index - 1] = 1. - ppois(ppois_index as f64, lambda, 1, 0);
    }
  }
  ppois_vec
}

// table convertor
fn table(
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

// cumulative sum
fn cumsum(mut cum_vec: Vec<f64>) -> Vec<f64> {
  let mut cumulus = 0.;
  for cix in &mut cum_vec {
    cumulus += *cix;
    *cix = cumulus;
  }
  cum_vec
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test private functions

// test R ppois bindings
#[cfg(test)]
mod ppois;

#[cfg(test)]
mod effective_genome_length_calculate;

#[cfg(test)]
mod lambda_calculate;

// test inverted probability poisson function
#[cfg(test)]
mod r_ppoisson;

#[cfg(test)]
mod table;

#[cfg(test)]
mod cumsum;

////////////////////////////////////////////////////////////////////////////////////////////////////
