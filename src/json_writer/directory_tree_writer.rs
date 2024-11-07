use crate::model::directory_node::DirectoryNode;
use std::fs::File;
use std::io::Write;
use serde_json;

/*
* Function to create a file for specified path, populates it with JSON
* from helper function.
*
* @param root_dir: contains reference to directory node to be written to a file
* @param output_path: reference to string value containing specified path for file creation
*/
pub fn write_tree_to_file(root_dir: &DirectoryNode, output_path: &str) -> std::io::Result<()> { 
    let json = convert_tree_to_json(&root_dir);
    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/*
* Helper function to convert directory tree to JSON
*
* @param root_dir: reference to directory node
* @return json: returns json string, transfering ownership to calling function
*/
fn convert_tree_to_json(root_dir: &DirectoryNode) -> String {
    let json = serde_json::to_string_pretty(&root_dir)
        .expect("Failed to serialize directory tree to JSON");
    json
}
