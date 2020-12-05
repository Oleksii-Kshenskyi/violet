use std::collections::HashMap;
use crate::util::treepath::TreePath;

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub value: T,
}

pub struct PathTree<T> {
    pub tree: HashMap<String, Option<Node<T>>>,
}

impl<T> PathTree<T> where 
    T: Clone {
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    pub fn set_by_path(&mut self, value: T, path: Vec<String>) {
        if path.len() == 0 {
            panic!("ERROR: path to a node cannot be empty!");
        }

        let the_hierarchy = TreePath::get_path_hierarchy(&path);
        the_hierarchy
            .into_iter()
            .enumerate()
            .for_each(|(index, one_path)| {
                if index == path.len() - 1 {
                    self.tree.insert(
                        one_path.to_string(),
                        Some(Node {
                            value: value.to_owned(),
                        }),
                    );
                } else {
                    if !self.tree.contains_key(&one_path) {
                        self.tree.insert(one_path.to_string(), None);
                    }
                }
            })
    }

    pub fn get_by_path(&self, path: Vec<String>) -> Option<&Node<T>> {
        if let Some(node_option) = self.tree.get(&path.join(" ")) {
            node_option.as_ref()
        } else {
            None
        }
    }

    pub fn does_node_exist(&self, path: Vec<String>) -> bool {
        self.tree.contains_key(&path.join(" "))
    }

}

#[test]
fn test_path_hierarchy() {
    let test_vec: Vec<String> = vec![
        "one".to_string(),
        "one two".to_string(),
        "one two three".to_string(),
    ];
    let expected: Vec<String> = "one two three"
        .split(" ")
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>();
    assert_eq!(test_vec, TreePath::get_path_hierarchy(&expected));

    let test_vec: Vec<String> = vec!["one".to_string()];
    let expected: Vec<String> = "one"
        .split(" ")
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>();
    assert_eq!(test_vec, TreePath::get_path_hierarchy(&expected));

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
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>();
    assert_eq!(test_vec, TreePath::get_path_hierarchy(&expected));
}

#[test]
fn test_tree_setters_and_getters() {
    let mut test_tree = PathTree::new();
    let path = TreePath::create_path(&"そっか おふの $%?рашин /fourth .fifth \\sixth".to_string());

    test_tree.set_by_path("test garbage val".to_string(), path);
    assert_eq!(
        true,
        test_tree.does_node_exist(TreePath::create_path(&"そっか".to_string()))
    );
    assert_eq!(
        true,
        test_tree.does_node_exist(TreePath::create_path(&"そっか おふの".to_string()))
    );
    assert_eq!(
        true,
        test_tree.does_node_exist(TreePath::create_path(&"そっか おふの $%?рашин".to_string()))
    );
    assert_eq!(
        true,
        test_tree.does_node_exist(TreePath::create_path(
            &"そっか おふの $%?рашин /fourth".to_string()
        ))
    );
    assert_eq!(
        true,
        test_tree.does_node_exist(TreePath::create_path(
            &"そっか おふの $%?рашин /fourth .fifth".to_string()
        ))
    );
    assert_eq!(
        true,
        test_tree.does_node_exist(TreePath::create_path(
            &"そっか おふの $%?рашин /fourth .fifth \\sixth".to_string()
        ))
    );
    assert_eq!(
        None,
        test_tree.get_by_path(TreePath::create_path(&"そっか".to_string()))
    );
    assert_eq!(
        None,
        test_tree.get_by_path(TreePath::create_path(&"そっか おふの".to_string()))
    );
    assert_eq!(
        None,
        test_tree.get_by_path(TreePath::create_path(&"そっか おふの $%?рашин".to_string()))
    );
    assert_eq!(
        None,
        test_tree.get_by_path(TreePath::create_path(
            &"そっか おふの $%?рашин /fourth".to_string()
        ))
    );
    assert_eq!(
        None,
        test_tree.get_by_path(TreePath::create_path(
            &"そっか おふの $%?рашин /fourth .fifth".to_string()
        ))
    );
    assert_eq!(
        Some(&Node {
            value: String::from("test garbage val")
        }),
        test_tree.get_by_path(TreePath::create_path(
            &"そっか おふの $%?рашин /fourth .fifth \\sixth".to_string()
        ))
    );
}

#[test]
fn check_empty_path_creation() {
    let mut test_tree = PathTree::new();
    let path = TreePath::create_path(&"そっか おふの $%?рашин /fourth .fifth \\sixth".to_string());
    test_tree.set_by_path("test garbage val".to_string(), path.clone());

    assert_eq!(
        Vec::<String>::new(),
        TreePath::create_path(&String::from(""))
    );
    assert_eq!(Vec::<String>::new(), TreePath::get_path_hierarchy(&vec![]));

    assert_eq!(false, test_tree.does_node_exist(vec![]));
    assert_eq!(None, test_tree.get_by_path(vec![]));
}

#[test]
#[should_panic(expected = "ERROR: path to a node cannot be empty!")]
fn test_setting_node_with_empty_path_panics() {
    let mut test_tree = PathTree::new();
    test_tree.set_by_path("value".to_string(), vec![]);
}
