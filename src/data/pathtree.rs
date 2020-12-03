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

   pub fn create_path(pathify_this: &String) -> Vec<String> {
      pathify_this.trim().split_whitespace().map(|elem| {
         elem.to_string()
      }).collect::<Vec<String>>()
   }

   pub fn set_by_path(&mut self, element: String, path: Vec<String>) {
      path.iter().enumerate().for_each(|(index, elem)| {
         match self.tree.get_mut(elem) {
            Some(inner_elem) => {
               if index == path.len() - 1 {
                  inner_elem.as_mut().unwrap().insert(elem.to_string(), element.clone());
               }
            } ,
            None => { self.tree.insert(elem.to_string(), Some(HashMap::new())); }
         }
      });
   }
}