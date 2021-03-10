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
        if !self.does_path_exist(&path) {
            false
        } else {
            self.get_by_path(&path).is_some()
        }
    }

    fn attempt_multiword_parsing(&self, path: &str) -> Option<(String, Vec<String>)> {
        None
    }

    fn attempt_single_word_parsing(&self, path: &str) -> Option<(String, Vec<String>)> {
        let pathvec = TreePath::create_path(path);
        let mut args: Vec<String> = vec![];
        let mut argumented: Vec<String> = vec![];
        for path in TreePath::get_path_hierarchy(&pathvec.join(" ")) {
            let previous_state = argumented.clone();
            argumented = TreePath::create_path(&TreePath::append_path_node(
                &argumented,
                TreePath::get_last_node(&path).unwrap().as_str(),
            ));
            if self.does_path_exist(&argumented.join(" ")) {
                continue;
            } else {
                let argified_from_previous = TreePath::append_path_node(&previous_state, "<ARG>");
                if self.does_path_exist(&argified_from_previous) {
                    argumented = TreePath::create_path(&argified_from_previous);
                    args.push(TreePath::get_last_node(&path).unwrap());
                } else {
                    return None;
                }
            }
        }

        Some((argumented.join(" "), args))
    }

    pub fn get_command_and_args_from_path(&self, path: &str) -> Option<(String, Vec<String>)> {
        if let Some((mw_command, mw_args)) = self.attempt_multiword_parsing(path) {
            Some((mw_command, mw_args))
        }
        else {
            self.attempt_single_word_parsing(path)
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
    assert_eq!(
        expected,
        TreePath::get_path_hierarchy("そっか おふの $%?рашин /fourth .fifth \\sixth")
    );
}

#[test]
fn test_tree_setters_and_getters() {
    let mut test_tree = PathTree::new();

    test_tree.set_by_path(
        "test garbage val".to_string(),
        "そっか おふの $%?рашин /fourth .fifth \\sixth",
    );
    assert_eq!(false, test_tree.does_node_exist("そっか"));
    assert_eq!(true, test_tree.does_path_exist("そっか"));
    assert_eq!(false, test_tree.does_node_exist("そっか おふの"));
    assert_eq!(true, test_tree.does_path_exist("そっか おふの"));
    assert_eq!(false, test_tree.does_node_exist("そっか おふの $%?рашин"));
    assert_eq!(true, test_tree.does_path_exist("そっか おふの $%?рашин"));
    assert_eq!(
        false,
        test_tree.does_node_exist("そっか おふの $%?рашин /fourth")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの $%?рашин /fourth")
    );
    assert_eq!(
        false,
        test_tree.does_node_exist("そっか おふの $%?рашин /fourth .fifth")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの $%?рашин /fourth .fifth")
    );
    assert_eq!(
        true,
        test_tree.does_node_exist("そっか おふの $%?рашин /fourth .fifth \\sixth")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの $%?рашин /fourth .fifth \\sixth")
    );
    assert_eq!(None, test_tree.get_by_path("そっか"));
    assert_eq!(None, test_tree.get_by_path("そっか おふの"));
    assert_eq!(None, test_tree.get_by_path("そっか おふの $%?рашин"));
    assert_eq!(
        None,
        test_tree.get_by_path("そっか おふの $%?рашин /fourth")
    );
    assert_eq!(
        None,
        test_tree.get_by_path("そっか おふの $%?рашин /fourth .fifth")
    );
    assert_eq!(
        Some(&Node {
            value: String::from("test garbage val")
        }),
        test_tree.get_by_path("そっか おふの $%?рашин /fourth .fifth \\sixth")
    );
}

#[test]
fn check_empty_path_creation() {
    let mut test_tree = PathTree::new();
    test_tree.set_by_path(
        "test garbage val".to_string(),
        "そっか おふの $%?рашин /fourth .fifth \\sixth",
    );

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

    assert_eq!(false, test_tree.does_node_exist("something"));
    assert_eq!(true, test_tree.does_path_exist("something"));
    assert_eq!(false, test_tree.does_node_exist("something completely"));
    assert_eq!(true, test_tree.does_path_exist("something completely"));
    assert_eq!(
        true,
        test_tree.does_node_exist("something completely bonkers")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("something completely bonkers")
    );

    assert_eq!(None, test_tree.get_by_path("something"));
    assert_eq!(None, test_tree.get_by_path("something completely"));
    assert_eq!(
        Some(&Node {
            value: String::from("test garbage val")
        }),
        test_tree.get_by_path("something completely bonkers")
    );

    assert_eq!(true, test_tree.tree.contains_key("something"));
    assert_eq!(true, test_tree.tree.contains_key("something completely"));
    assert_eq!(
        true,
        test_tree.tree.contains_key("something completely bonkers")
    );
}

#[test]
fn check_argumented_paths() {
    let mut test_tree = PathTree::new();
    test_tree.set_by_path("garbage", "this is <ARG> another <ARG>");
    test_tree.set_by_path("garbage", "oh my god its <ARG> working <ARG> !!!");
    test_tree.set_by_path("garbage", "<ARG> I cant believe <ARG>");
    test_tree.set_by_path(
        "garbage",
        "<ARG> and <ARG> and <ARG> and <ARG> and <ARG> and KEKW",
    );

    assert_eq!(
        Some((
            String::from("this is <ARG> another <ARG>"),
            vec![String::from("just"), String::from("test")]
        )),
        test_tree.get_command_and_args_from_path("this is just another test")
    );
    assert_eq!(
        Some((
            String::from("this is <ARG> another <ARG>"),
            vec![String::from("pooka"), String::from("kooka")]
        )),
        test_tree.get_command_and_args_from_path("this is pooka another kooka")
    );
    assert_eq!(
        Some((
            String::from("this is <ARG> another <ARG>"),
            vec![String::from("$$"), String::from("---")]
        )),
        test_tree.get_command_and_args_from_path("this is $$ another ---")
    );
    assert_eq!(
        None,
        test_tree.get_command_and_args_from_path("this is not just another test")
    );

    assert_eq!(
        Some((
            String::from("oh my god its <ARG> working <ARG> !!!"),
            vec![String::from("actually"), String::from("OMG")]
        )),
        test_tree.get_command_and_args_from_path("oh my god its actually working OMG !!!")
    );
    assert_eq!(
        Some((
            String::from("oh my god its <ARG> working <ARG> !!!"),
            vec![String::from("kekek"), String::from("lulul")]
        )),
        test_tree.get_command_and_args_from_path("oh my god its kekek working lulul !!!")
    );
    assert_eq!(
        None,
        test_tree.get_command_and_args_from_path("oh my god its not actually working lulul ? !!!")
    );

    assert_eq!(
        Some((
            String::from("<ARG> I cant believe <ARG>"),
            vec![String::from("Jesus"), String::from("it!!!")]
        )),
        test_tree.get_command_and_args_from_path("Jesus I cant believe it!!!")
    );
    assert_eq!(
        Some((
            String::from("<ARG> I cant believe <ARG>"),
            vec![String::from("123"), String::from("584")]
        )),
        test_tree.get_command_and_args_from_path("123 I cant believe 584")
    );
    assert_eq!(
        None,
        test_tree.get_command_and_args_from_path("OMG I can believe this")
    );

    assert_eq!(
        Some((
            String::from("<ARG> and <ARG> and <ARG> and <ARG> and <ARG> and KEKW"),
            vec![
                String::from("one"),
                String::from("two"),
                String::from("three"),
                String::from("four"),
                String::from("five")
            ]
        )),
        test_tree
            .get_command_and_args_from_path("one and two and three and four and five and KEKW")
    );
}
