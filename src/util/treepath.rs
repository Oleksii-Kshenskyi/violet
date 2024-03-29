pub struct TreePath;

impl TreePath {
    pub fn create_path(pathify_this: &str) -> Vec<String> {
        pathify_this
            .trim()
            .split_whitespace()
            .map(|elem| elem.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_path_hierarchy(path: &str) -> Vec<String> {
        let path = TreePath::create_path(path);
        let mut current_path = String::new();
        let mut hierarchy: Vec<String> = vec![];

        path.iter().for_each(|path_node| {
            current_path.push_str(path_node.clone().as_str());
            current_path.push(' ');
            hierarchy.push(current_path.trim().to_string());
        });

        hierarchy
    }

    pub fn prettify(path: &str) -> String {
        TreePath::create_path(path).join(" ")
    }

    pub fn append_path_node(path: &[String], node_to_append: &str) -> String {
        let mut newpathvec = path.to_owned();
        newpathvec.push(node_to_append.to_owned());
        newpathvec.join(" ")
    }

    pub fn get_last_node(of_path: &str) -> Option<String> {
        let path = TreePath::create_path(of_path);
        match path.last() {
            Some(node) => Some(node.to_owned()),
            None => None,
        }
    }

    pub fn reconstruct_argumented_path(path_to_reconstruct: &str, args: Vec<String>) -> String {
        let mut pathvec = TreePath::create_path(path_to_reconstruct);

        let mut arg_index: usize = 0;
        for node in pathvec.iter_mut() {
            if node.as_str() == "<ARG>" {
                let new_arg = format!("\"{}\"", args[arg_index]).to_string();
                *node = new_arg;

                arg_index += 1;
            }
        }

        pathvec.join(" ")
    }

    pub fn count_x_nodes_for_path(path: &str, x_node: &str) -> usize {
        TreePath::create_path(path)
            .iter()
            .filter(|node| node.as_str() == x_node)
            .count()
    }

    pub fn create_shortcut(path: &str, serial: usize) -> String {
        if path.is_empty() {
            panic!("PANIC: TreePath::create_shortcut(): couldn't create the shortcut, source path is empty!");
        }

        let pathvec = TreePath::create_path(path);
        let mut shortcut: String = String::from('[');
        let mut arg_count: usize = 0;

        let serial_str = serial.to_string();
        let alias_serial = if serial_str.as_str() == "1" {
            ""
        } else {
            serial_str.as_str()
        };

        for node in pathvec {
            if node.as_str() == "<ARG>" {
                shortcut.push('a');
                arg_count += 1;
            } else {
                shortcut.push(node.chars().next().unwrap());
            }
        }

        if !alias_serial.is_empty() {
            shortcut.push_str(alias_serial);
        }
        shortcut.push(']');
        if arg_count != 0 {
            shortcut.push(' ');
            shortcut.push_str(vec!["<ARG>"; arg_count].join(" ").as_str());
        }
        shortcut
    }

    pub fn is_path_a_shortcut(path: &str) -> bool {
        let first_node = TreePath::create_path(path)[0].to_owned();

        first_node.starts_with('[') && first_node.ends_with(']')
    }
}
