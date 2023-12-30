mod tree;
use std::env;
fn main() {
    match env::current_dir() {
        Ok(p) => tree::parse_tree(p.as_os_str()),
        Err(e) => eprintln!("{}", e.kind().to_string()),
    }
}
