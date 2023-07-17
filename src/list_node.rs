#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNodei32 {
  pub val: i32,
  pub next: Option<Box<ListNodei32>>
}

impl ListNodei32 {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNodei32 {
      next: None,
      val
    }
  }
}