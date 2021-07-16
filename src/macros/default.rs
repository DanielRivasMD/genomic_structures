////////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculate effective genome / chromosome / scaffold length
/// with default parameters: `BIN_SIZE` and `BIN_OVERLAP`.
#[macro_export]
macro_rules! effective_genome_length_calculator {
  // explicit values
  ( $gl: expr, $bs: expr, $bo: expr ) => {
    effective_genome_length_calculator($gl, $bs, $bo)
  };

  // default values
  ( $gl: expr ) => {
    effective_genome_length_calculator($gl, BIN_SIZE as f64, BIN_OVERLAP as f64)
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculate Poisson's lambda (λ)
/// with default parameters: `BIN_SIZE`.
#[macro_export]
macro_rules! lambda_calculator {
  // explicit values
  ( $pr: expr, $egl: expr, $bs: expr ) => {
    lambda_calculator($pr, $egl, $bs)
  };

  // default values
  ( $pr: expr, $egl: expr ) => {
    lambda_calculator($pr, $egl, BIN_SIZE as f64)
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
