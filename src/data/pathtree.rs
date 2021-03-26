use crate::util::treepath::TreePath;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub enum PathTreeOk {
    DropOk,
}

pub enum PathTreeErr {
    DropNodeDoesNotExist,
    DropNodeIsNull,
}



#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Node<T> {
    pub share_count: usize,
    pub value: Option<T>,
}

enum DropType {
    RemoveNode(String),
    ActiveToNull(String),
}

#[derive(Serialize, Deserialize)]
pub struct PathTree<T> {
    pub tree: HashMap<String, Node<T>>,
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
                    let inserted_node = self.tree.entry(
                        one_path).or_insert(
                        Node {
                            share_count: 0,
                            value: Some(value.to_owned()),
                        });
                    inserted_node.share_count += 1;
                } else {
                    let null_node = self
                                                    .tree
                                                    .entry(one_path)
                                                    .or_insert(Node {
                                                        share_count: 0,
                                                        value: None,
                                                    });
                    null_node.share_count += 1;
                }
            })
    }

    pub fn set_by_path_with_shortcut(&mut self, value: T, path: &str) {
        self.set_by_path(value.clone(), path);

        let mut shortcut_name = TreePath::create_shortcut(path, 1);
        if !self.does_node_contain_value(&shortcut_name) {
            self.set_by_path(value.clone(), &shortcut_name);
        } else {
            let mut alias_serial_number: usize = 2;
            shortcut_name = loop {
                let current_alias = TreePath::create_shortcut(path, alias_serial_number);
                if !self.does_node_contain_value(&current_alias) {
                    break current_alias;
                } else {
                    alias_serial_number += 1;
                }
            }
        }
        self.set_by_path(value, &shortcut_name);
    }

    pub fn get_by_path(&self, path: &str) -> Option<&Node<T>> {
        let path = TreePath::create_path(&path);

        self.tree.get(&path.join(" "))
    }




    fn flag_hierarchy_for_dropping(&mut self, path: &str) -> Vec<DropType> {
        let hierarchy = TreePath::get_path_hierarchy(path);
        let mut drops: Vec<DropType> = vec![];

        for (path_index, single_path) in hierarchy.iter().enumerate() {
            match self.tree.get_mut(single_path) {
                None => unreachable!("!!! PathTree::drop_hierarchy: node does not exist !!! Most likely the aliases JSON file is corrupted."),
                Some(node) => {
                    node.share_count -= 1;
                    if node.share_count == 0 {
                        drops.push(DropType::RemoveNode(single_path.to_owned()));
                    } else if node.value.is_some() && path_index == hierarchy.len() - 1 {
                        drops.push(DropType::ActiveToNull(single_path.to_owned()));
                    }
                },
            }
        }

        drops
    }

    fn drop_hierarchy(&mut self, path: &str) {
        let drops = self.flag_hierarchy_for_dropping(path);

        for drop in drops {
            match drop {
                DropType::RemoveNode(path) => {
                    self.tree.remove(&path).expect("ERROR: drop_hierarchy(): remove node failed, something is wrong with PathTree::flag_hierarchy() logic.");
                },
                DropType::ActiveToNull(path) => {
                    let node = self.tree.get_mut(&path).expect("ERROR: drop_hierarchy(): setting active node to null failed, something is wrong with PathTree::flag_hierarchy() logic.");
                    if !node.value.is_some() {
                        panic!("ERROR: PathTree::drop_hierarchy(): tried to set an active node to null, but it's already a null node.");
                    }
                    node.value = None;
                },
            }
        }
    }

    pub fn drop_by_path(&mut self, path: &str) -> Result<PathTreeOk, PathTreeErr> {
        match self.get_by_path(path) {
            None => Err(PathTreeErr::DropNodeDoesNotExist),
            Some(node) => {
                if let Some(_) = node.value {
                    self.drop_hierarchy(path);
                    Ok(PathTreeOk::DropOk)
                } else {
                    Err(PathTreeErr::DropNodeIsNull)
                }
            },
        }
    }

    pub fn does_path_exist(&self, path: &str) -> bool {
        self.tree.contains_key(&TreePath::prettify(path))
    }

    pub fn does_node_contain_value(&self, path: &str) -> bool {
        if !self.does_path_exist(&path) {
            false
        } else {
            self.get_by_path(&path).unwrap().value.is_some()
        }
    }

    fn attempt_multiword_parsing(&self, path: &str) -> Option<(String, Vec<String>)> {
        let nodes = TreePath::create_path(path);
        let mut slice_indices: (Vec<u32>, Vec<u32>) = (vec![], vec![]);
        let mut args: Vec<String> = vec![];
        let mut resulting_pathvec: Vec<String> = vec![];

        let mut loop_validation = true;
        nodes.iter().enumerate().for_each(|(index, node)| {
            if node.starts_with('\"') && node.len() == 1 {
                loop_validation = false;
                return;
            }

            if node.starts_with('\"') {
                slice_indices.0.push(index as u32);
            }
            if node.ends_with('\"') {
                slice_indices.1.push(index as u32);
            }
        });
        if !loop_validation {
            return None;
        }

        if slice_indices.0.is_empty() || slice_indices.1.is_empty() {
            return None;
        }
        if slice_indices.0.len() != slice_indices.1.len() {
            return None;
        }

        let mut previous_end_index: u32 = 0;
        let mut loop_validation = true;
        slice_indices
            .0
            .iter()
            .enumerate()
            .for_each(|(index, start)| {
                let end = slice_indices.1.get(index).unwrap();
                if *start > *end || *start < previous_end_index {
                    loop_validation = false;
                }

                previous_end_index = *end;
            });
        if !loop_validation {
            return None;
        }

        for arg_number in 0..slice_indices.0.len() {
            let lower_index = slice_indices.0[arg_number] as usize;
            let upper_index = slice_indices.1[arg_number] as usize;

            let mut new_arg: Vec<String> = vec![];
            new_arg.extend_from_slice(&nodes[lower_index..=upper_index]);
            let mut new_arg = new_arg.join(" ");

            new_arg.remove(0);
            new_arg.remove(new_arg.len() - 1);

            args.push(new_arg);
        }

        let mut started: bool = false;
        for node in &nodes {
            if node.starts_with('\"') {
                started = true;
                resulting_pathvec.push("<ARG>".to_owned());
            }

            if !node.starts_with('\"') && !node.ends_with('\"') && !started {
                resulting_pathvec.push(node.clone());
            }

            if node.ends_with('\"') {
                started = false;
            }
        }

        let resulting_path = resulting_pathvec.join(" ");
        if self.does_node_contain_value(resulting_path.as_str()) {
            Some((resulting_path, args))
        } else {
            None
        }
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

        if args.iter().any(|arg| arg == "\"") {
            return None;
        }

        Some((argumented.join(" "), args))
    }

    pub fn get_command_and_args_from_path(&self, path: &str) -> Option<(String, Vec<String>)> {
        if let Some((mw_command, mw_args)) = self.attempt_multiword_parsing(path) {
            Some((mw_command, mw_args))
        } else {
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
    assert_eq!(false, test_tree.does_node_contain_value("そっか"));
    assert_eq!(true, test_tree.does_path_exist("そっか"));
    assert_eq!(false, test_tree.does_node_contain_value("そっか おふの"));
    assert_eq!(true, test_tree.does_path_exist("そっか おふの"));
    assert_eq!(
        false,
        test_tree.does_node_contain_value("そっか おふの $%?рашин")
    );
    assert_eq!(true, test_tree.does_path_exist("そっか おふの $%?рашин"));
    assert_eq!(
        false,
        test_tree.does_node_contain_value("そっか おふの $%?рашин /fourth")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの $%?рашин /fourth")
    );
    assert_eq!(
        false,
        test_tree.does_node_contain_value("そっか おふの $%?рашин /fourth .fifth")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの $%?рашин /fourth .fifth")
    );
    assert_eq!(
        true,
        test_tree.does_node_contain_value("そっか おふの $%?рашин /fourth .fifth \\sixth")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("そっか おふの $%?рашин /fourth .fifth \\sixth")
    );
    assert_eq!(None, test_tree.get_by_path("そっか").unwrap().value);
    assert_eq!(None, test_tree.get_by_path("そっか おふの").unwrap().value);
    assert_eq!(
        None,
        test_tree
            .get_by_path("そっか おふの $%?рашин")
            .unwrap()
            .value
    );
    assert_eq!(
        None,
        test_tree
            .get_by_path("そっか おふの $%?рашин /fourth")
            .unwrap()
            .value
    );
    assert_eq!(
        None,
        test_tree
            .get_by_path("そっか おふの $%?рашин /fourth .fifth")
            .unwrap()
            .value
    );
    assert_eq!(
        Some(&Node {
            share_count: 1,
            value: Some(String::from("test garbage val"))
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

    assert_eq!(false, test_tree.does_node_contain_value(""));
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

    assert_eq!(false, test_tree.does_node_contain_value("something"));
    assert_eq!(true, test_tree.does_path_exist("something"));
    assert_eq!(
        false,
        test_tree.does_node_contain_value("something completely")
    );
    assert_eq!(true, test_tree.does_path_exist("something completely"));
    assert_eq!(
        true,
        test_tree.does_node_contain_value("something completely bonkers")
    );
    assert_eq!(
        true,
        test_tree.does_path_exist("something completely bonkers")
    );

    assert_eq!(None, test_tree.get_by_path("something").unwrap().value);
    assert_eq!(
        None,
        test_tree.get_by_path("something completely").unwrap().value
    );
    assert_eq!(
        Some(&Node {
            share_count: 1,
            value: Some(String::from("test garbage val"))
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
