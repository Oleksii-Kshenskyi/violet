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
            current_path.push_str(" ");
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
}
