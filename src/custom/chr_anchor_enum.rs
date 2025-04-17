////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: format table documentation

// |---------------------------------------------|---------------------------------------------|
// | C anchor [unmpped] (position) {orientation} | ME anchor [mapped] (position) {orientation} |
// |---------------------------------------------|---------------------------------------------|
// | complete [*] (mate) {inwards}               | complete [100M] (ME limit) {outwards}       |
// | complete [*] (mate) {inwards}               | partial [50S50M] (ME limit) {outwards}      |
// | partial [50S50M] (ME limit) {inwards}       | complete [100M]  (ME limit) {outwards}      |
// | partial [50S50M] (ME limit) {inwards}       | partial [50S50M]  (ME limit) {outwards}     |
// |---------------------------------------------|---------------------------------------------|
/// Define chromosomal anchor.
#[derive(Debug, new, PartialEq)]
pub enum ChrAnchorEnum {
  Read1,
  Read2,
  None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// manual default trait implementation
impl Default for ChrAnchorEnum {
  fn default() -> Self {
    Self::None
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
