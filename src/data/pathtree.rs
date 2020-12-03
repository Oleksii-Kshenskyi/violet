use std::collections::HashMap;

pub struct PathTree {
   pub tree: HashMap<String, Option<HashMap<String, String>>>
}

impl PathTree {
   pub fn new() -> Self {
      PathTree {
         tree: HashMap::new()
      }
   }

   pub fn create_path(pathify_this: &String) -> Vec<&str> {
      pathify_this.trim().split_whitespace().collect::<Vec<&str>>()
   }

   pub fn set_by_path(&mut self, value: String, path: Vec<&str>) {
      path.iter().enumerate().for_each(|(index, elem)| {
         match self.tree.get_mut(elem.to_owned()) {
            Some(inner_elem) => {
               if index == path.len() - 1 {
                  inner_elem.as_mut().unwrap().insert(elem.to_string(), value.clone());
               }
            } ,
            None => { self.tree.insert(elem.to_string(), Some(HashMap::new())); }
         }
      });
   }
}