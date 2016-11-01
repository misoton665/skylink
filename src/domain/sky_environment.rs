use domain::link::Link;

#[derive(Clone)]
pub struct SkyEnvironment {
  links: Vec<Link>,
}

impl SkyEnvironment {
  pub fn empty_env() -> SkyEnvironment {
    SkyEnvironment{links: vec![]}
  }

  pub fn add_link(&mut self, link: Link) {
    self.links.push(link);
  }

  pub fn delete_link(&mut self, dropping_link_id: &'static str) {
    let links_iter = self.clone().links.into_iter();
    let mut new_links = vec![];
    new_links.extend(links_iter.filter(|l| l.id != dropping_link_id));
    self.links = new_links;
  }

  pub fn find_link(&self, id: &'static str) -> Option<&Link> {
    for l in self.links.iter() {
      if l.id == id {
        return Some(l)
      }
    }
    None
  }
}
