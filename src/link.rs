//type SStr = &'static str;

use common::SStr;

pub struct Link {
  pub id: SStr,
  pub path: SStr,
  pub has_gitrep: bool,
}

impl Link {
  pub fn new(id: SStr, path: SStr, has_gitrep: bool) -> Link {
    Link{id: id, path: path, has_gitrep: has_gitrep}
  }
}

impl PartialEq for Link {
  fn eq(&self, other: &Link) -> bool {
    self.id == other.id
  }
}

//pub trait IdEq<T>{
//  fn eq_id(&self, that: &IdEq<T>) -> bool;
//
//  fn get_id(&self) -> T;
//}
//
//impl<'a> IdEq<&'static str> for Link<'a> {
//  fn eq_id(&self, that: &IdEq<&'static str>) -> bool {
//    self.get_id() == that.get_id()
//  }
//
//  fn get_id(&self) -> &'static str {
//    self.id
//  }
//}
