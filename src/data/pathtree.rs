use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
   value: String
}

pub struct PathTree {
   pub tree: HashMap<String, Option<Node>>
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

   pub fn set_by_path(&mut self, value: String, path: Vec<String>) {
      let mut the_hierarchy = Self::get_path_hierarchy(&path);
      the_hierarchy.iter_mut().enumerate().for_each(|(index, one_path)| {
         if index == path.len() - 1 {
            self.tree.insert(one_path.to_string(), Some(Node { value: value.to_owned() }));
         }
         else {
            if !self.tree.contains_key(one_path) {
               self.tree.insert(one_path.to_string(), None);
            }
         }
      })
   }


   fn get_path_hierarchy(path: &Vec<String>) -> Vec<String> {
      let mut current_path = String::new();
      let mut hierarchy: Vec<String> = vec![];

      path.iter().for_each(|path_node| {
         current_path.push_str(path_node.clone().as_str());
         current_path.push_str(" ");
         hierarchy.push(current_path.trim().to_string());
      });

      hierarchy
   }
}

#[test]
fn test_path_hierarchy() {
   let test_vec: Vec<String> = vec![
      "one".to_string(),
      "one two".to_string(),
      "one two three".to_string()
   ];
   let expected: Vec<String> = "one two three"
      .split(" ")
      .map(|elem| { elem.to_string() })
      .collect::<Vec<String>>();
   assert_eq!(test_vec, PathTree::get_path_hierarchy(&expected));

   let test_vec: Vec<String> = vec![
      "one".to_string()
   ];
   let expected: Vec<String> = "one"
      .split(" ")
      .map(|elem| { elem.to_string() })
      .collect::<Vec<String>>();
   assert_eq!(test_vec, PathTree::get_path_hierarchy(&expected));

   let test_vec: Vec<String> = vec![
      "そっか".to_string(),
      "そっか おふの".to_string(),
      "そっか おふの $%?рашин".to_string(),
      "そっか おふの $%?рашин /fourth".to_string(),
      "そっか おふの $%?рашин /fourth .fifth".to_string(),
      "そっか おふの $%?рашин /fourth .fifth \\sixth".to_string(),
   ];
   let expected: Vec<String> = "そっか おふの $%?рашин /fourth .fifth \\sixth"
      .split(" ")
      .map(|elem| { elem.to_string() })
      .collect::<Vec<String>>();
   assert_eq!(test_vec, PathTree::get_path_hierarchy(&expected));
}