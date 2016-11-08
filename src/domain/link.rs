#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Link {
  pub id: String,
  pub path: String,
  pub has_gitrep: bool,
}

impl Link {
  pub fn new(id: &'static str, path: &'static str, has_gitrep: bool) -> Link {
    Link{id: id.to_string(), path: path.to_string(), has_gitrep: has_gitrep}
  }
}

impl PartialEq for Link {
  fn eq(&self, other: &Link) -> bool {
    self.id == other.id
  }
}
