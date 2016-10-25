pub struct Link {
  pub id: &'static str,
  pub path: &'static str,
  pub has_gitrep: bool,
}

impl Link {
  pub fn new(id: &'static str, path: &'static str, has_gitrep: bool) -> Link {
    Link{id: id, path: path, has_gitrep: has_gitrep}
  }
}

impl PartialEq for Link {
  fn eq(&self, other: &Link) -> bool {
    self.id == other.id
  }
}
