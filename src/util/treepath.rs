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

    pub fn reconstruct_argumented_path(path_to_reconstruct: String, args: Vec<String>) -> Option<String> {
        let mut pathvec = TreePath::create_path(path_to_reconstruct.as_str());
        if pathvec.iter().filter(|node| node.as_str() == "<ARG>").count() != args.len() {
            None
        }
        else {
            let mut arg_index: usize = 0;
            for node in pathvec.iter_mut() {
                if node.as_str() == "<ARG>" {
                    let new_arg: String;
                    if TreePath::create_path(args[arg_index].as_str()).len() > 1 {
                        new_arg = format!("\"{}\"", args[arg_index]).to_string();
                    }
                    else {
                        new_arg = args[arg_index].clone();
                    }
                    *node = new_arg;
                    arg_index += 1;
                }
            }

            Some(pathvec.join(" "))
        }
    }
}
