use crate::util::treepath::TreePath;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub value: T,
}

pub struct PathTree<T> {
    pub tree: HashMap<String, Option<Node<T>>>,
}

impl<T> PathTree<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    pub fn set_by_path(&mut self, value: T, path: &str) {
        if path.is_empty() {
            panic!("ERROR: path to a node cannot be empty!");
        }

        let the_hierarchy = TreePath::get_path_hierarchy(&path);
        let path = TreePath::create_path(path);
        the_hierarchy
            .into_iter()
            .enumerate()
            .for_each(|(index, one_path)| {
                if index == path.len() - 1 {
                    self.tree.insert(
                        one_path,
                        Some(Node {
                            value: value.to_owned(),
                        }),
                    );
                } else {
                    self.tree.entry(one_path).or_insert(None);
                }
            })
    }

    pub fn get_by_path(&self, path: &str) -> Option<&Node<T>> {
        let path = TreePath::create_path(&path);
        if let Some(node_option) = self.tree.get(&path.join(" ")) {
            node_option.as_ref()
        } else {
            None
        }
    }

    pub fn does_path_exist(&self, path: &str) -> bool {
        self.tree.contains_key(&TreePath::prettify(path))
    }

    pub fn does_node_exist(&self, path: &str) -> bool {
        let path = TreePath::prettify(path);
        if !self.tree.contains_key(&path) {
            false
        }
        else {
            match self.get_by_path(&path) {
                Some(_) => true,
                None => false
            }
        }
    }
}

#[test]
fn test_path_hierarchy() {
    let expected: Vec<String> = vec![
        "one".to_string(),
        "one two".to_string(),
        "one two three".to_string(),
    ];
    assert_eq!(expected, TreePath::get_path_hierarchy("one two three"));

    let expected: Vec<String> = vec!["one".to_string()];
    assert_eq!(expected, TreePath::get_path_hierarchy("one"));

    let expected: Vec<String> = vec![
        "そっか".to_string(),
        "そっか おふの".to_string(),
        "そっか おふの $%?рашин".to_string(),
        "そっか おふの $%?рашин /fourth".to_string(),
        "そっか おふの $%?рашин /fourth .fifth".to_string(),
        "そっか おふの $%?рашин /fourth .fifth \\sixth".to_string(),
    ];
    assert_eq!(expected, TreePath::get_path_hierarchy("そっか おふの $%?рашин /fourth .fifth \\sixth"));
}

#[test]
fn test_tree_setters_and_getters() {
    let mut test_tree = PathTree::new();

    test_tree.set_by_path("test garbage val".to_string(), "そっか おふの $%?рашин /fourth .fifth \\sixth");
    assert_eq!(
        false,
        test_tree.does_node_exist("そっか")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか")
    );
    assert_eq!(
        false,
        test_tree.does_node_exist("そっか おふの")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの")
    );
    assert_eq!(
        false,
        test_tree.does_node_exist("そっか おふの $%?рашин")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの $%?рашин")
    );
    assert_eq!(
        false,
        test_tree.does_node_exist(            
            "そっか おふの $%?рашин /fourth"
        )
    );
    assert_eq!(
        true,
        test_tree.does_path_exist(            
            "そっか おふの $%?рашин /fourth"
        )
    );
    assert_eq!(
        false,
        test_tree.does_node_exist(
            "そっか おふの $%?рашин /fourth .fifth"
        )
    );
    assert_eq!(
        true,
        test_tree.does_path_exist(
            "そっか おふの $%?рашин /fourth .fifth"
        )
    );
    assert_eq!(
        true,
        test_tree.does_node_exist(
            "そっか おふの $%?рашин /fourth .fifth \\sixth"
        )
    );
    assert_eq!(
        true,
        test_tree.does_path_exist(
            "そっか おふの $%?рашин /fourth .fifth \\sixth"
        )
    );
    assert_eq!(
        None,
        test_tree.get_by_path("そっか")
    );
    assert_eq!(
        None,
        test_tree.get_by_path("そっか おふの")
    );
    assert_eq!(
        None,
        test_tree.get_by_path("そっか おふの $%?рашин")
    );
    assert_eq!(
        None,
        test_tree.get_by_path(
            "そっか おふの $%?рашин /fourth"
        )
    );
    assert_eq!(
        None,
        test_tree.get_by_path(
            "そっか おふの $%?рашин /fourth .fifth"
        )
    );
    assert_eq!(
        Some(&Node {
            value: String::from("test garbage val")
        }),
        test_tree.get_by_path(
            "そっか おふの $%?рашин /fourth .fifth \\sixth"
        )
    );
}

#[test]
fn check_empty_path_creation() {
    let mut test_tree = PathTree::new();
    test_tree.set_by_path("test garbage val".to_string(), "そっか おふの $%?рашин /fourth .fifth \\sixth");

    assert_eq!(
        Vec::<String>::new(),
        TreePath::create_path(&String::from(""))
    );
    assert_eq!(Vec::<String>::new(), TreePath::get_path_hierarchy(""));

    assert_eq!(false, test_tree.does_node_exist(""));
    assert_eq!(None, test_tree.get_by_path(""));
}

#[test]
#[should_panic(expected = "ERROR: path to a node cannot be empty!")]
fn test_setting_node_with_empty_path_panics() {
    let mut test_tree = PathTree::new();
    test_tree.set_by_path("value".to_string(), "");
}

#[test]
fn test_pathing_works_with_untrimmed_paths() {
    let mut test_tree = PathTree::new();
    let path = "        something           completely      bonkers       \n";

    test_tree.set_by_path("test garbage val".to_string(), path);

    assert_eq!(
        false,
        test_tree.does_node_exist("something")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("something")
    );
    assert_eq!(
        false,
        test_tree.does_node_exist("something completely")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("something completely")
    );
    assert_eq!(
        true,
        test_tree.does_node_exist("something completely bonkers")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("something completely bonkers")
    );

    assert_eq!(
        None,
        test_tree.get_by_path("something")
    );
    assert_eq!(
        None,
        test_tree.get_by_path("something completely")
    );
    assert_eq!(
        Some(&Node {
            value: String::from("test garbage val")
        }),
        test_tree.get_by_path(
            "something completely bonkers"
        )
    );

    assert_eq!(true, test_tree.tree.contains_key("something"));
    assert_eq!(true, test_tree.tree.contains_key("something completely"));
    assert_eq!(true, test_tree.tree.contains_key("something completely bonkers"));
}