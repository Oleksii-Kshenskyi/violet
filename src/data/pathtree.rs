use std::collections::HashMap;

pub struct PathTree<'a> {
   tree: HashMap<&'a str, Option<HashMap<&'a str, &'a str>>>
}

impl<'a> PathTree<'a> {
   pub fn new() -> Self {
      PathTree {
         tree: HashMap::new()
      }
   }

   pub fn create_path(pathify_this: &'a str) -> Vec<&'a str> {
      pathify_this.trim().split_whitespace().collect::<Vec<&str>>()
   }
}