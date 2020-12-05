pub struct TreePath;

impl TreePath {
    pub fn create_path(pathify_this: &str) -> Vec<String> {
        pathify_this
            .trim()
            .split_whitespace()
            .map(|elem| elem.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_path_hierarchy(path: &[String]) -> Vec<String> {
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
