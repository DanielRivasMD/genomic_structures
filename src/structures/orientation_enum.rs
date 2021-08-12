////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new, Clone, PartialEq)]
pub enum OrientationEnum {
  Downstream,
  Upstream,
  None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// manual default trait implementation
impl Default for OrientationEnum {
  fn default() -> Self {
    Self::None
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
