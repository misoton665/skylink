use domain::link::Link;

#[derive(Clone, Debug)]
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

#[test]
fn test_find_empty_env() {
  let sky_env = SkyEnvironment::empty_env();

  let some_link: Option<&Link> = sky_env.find_link("some");

  debug_assert_eq!(some_link, None);
}

#[test]
fn test_find_and_add_link() {
  let mut sky_env = SkyEnvironment::empty_env();

  let test_link_name = "test";

  let test_link = Link::new(test_link_name, "/test/path", false);

  sky_env.add_link(test_link);

  let some_link = sky_env.find_link(test_link_name);

  debug_assert_eq!(some_link.is_some(), true);
}

#[test]
fn test_add_and_delete_link() {
  let mut sky_env = SkyEnvironment::empty_env();

  let test_link_name = "test";

  let test_link = Link::new(test_link_name, "/test/path", false);

  sky_env.add_link(test_link);

  sky_env.delete_link(test_link_name);

  debug_assert_eq!(sky_env.find_link(test_link_name).is_none(), true);
}