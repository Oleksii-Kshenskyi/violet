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
}