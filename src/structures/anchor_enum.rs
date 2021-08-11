////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new, Clone)]
pub enum AnchorEnum {
  Foward5,
  Forward3,
  Reverse5,
  Reverse3,
  None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for AnchorEnum {
  fn default() -> Self {
    Self::None
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
