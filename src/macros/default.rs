////////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculate effective genome / chromosome / scaffold length
/// with default parameters: `BIN_SIZE` and `BIN_OVERLAP`.
#[macro_export]
macro_rules! calculate_effective_len {
  // explicit values
  ( $gl: expr, $bs: expr, $bo: expr ) => {
    calculate_effective_len($gl, $bs, $bo)
  };

  // default values
  ( $gl: expr ) => {
    calculate_effective_len($gl, BIN_SIZE as f64, BIN_OVERLAP as f64)
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculate Poisson's lambda (λ)
/// with default parameters: `BIN_SIZE`.
#[macro_export]
macro_rules! calculate_lambda {
  // explicit values
  ( $pr: expr, $egl: expr, $bs: expr ) => {
    calculate_lambda($pr, $egl, $bs)
  };

  // default values
  ( $pr: expr, $egl: expr ) => {
    calculate_lambda($pr, $egl, BIN_SIZE as f64)
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
