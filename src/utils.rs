use dirs::home_dir;
use std::fs;

pub fn set_workspace() {
    let mut workspace = home_dir().unwrap();
    workspace.push(".pymanager");

    match fs::create_dir_all(workspace) {
        Ok(_) => println!("hi mom"),
        Err(e) => println!("Error creating workspace: {}", e),
    }
}
