use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry, ReadDir},
};

const HORIZONTAL: &str = "─";
const VERTICAL: &str = "├";
const VERTICAL_END: &str = "└";
const VERTICAL_NOBAR: &str = "│";

enum TreeElement {
    Dir((OsString, Vec<TreeElement>)),
    File(OsString),
}
fn filter_unwrap(dirs: ReadDir) -> Vec<DirEntry> {
    dirs.filter(|el| el.is_ok()).map(|el| el.unwrap()).collect()
}

pub fn parse_tree(cwd: &OsStr) {
    match tree(cwd) {
        Ok(dir) => {
            println!(".");
            print_tree(&dir, "");
        }
        _ => {}
    };
}

fn tree(cwd: &OsStr) -> Result<Vec<TreeElement>, String> {
    let current_dir = match fs::read_dir(cwd) {
        Ok(dirs) => dirs,
        Err(err) => return Err(err.kind().to_string()),
    };
    let mut elements: Vec<TreeElement> = vec![];
    for el in filter_unwrap(current_dir) {
        if el.path().is_file() {
            elements.push(TreeElement::File(el.path().file_name().unwrap().to_owned()))
        } else {
            let dirname: OsString = el.path().file_name().unwrap().to_owned();
            let sub_elements = match tree(el.path().as_os_str()) {
                Ok(dirs) => TreeElement::Dir((dirname, dirs)),
                _ => TreeElement::Dir((dirname, Vec::with_capacity(0))),
            };
            elements.push(sub_elements)
        }
    }
    return Ok(elements);
}

fn format_indentation(name: &str, last_child: bool) -> String {
    //println!("{depth}");
    let format = match last_child {
        true => format!("{}{}", VERTICAL_END, HORIZONTAL.repeat(2)),
        false => format!("{}{}", VERTICAL, HORIZONTAL.repeat(2)),
    };
    format!("{} {}", format, name)
}

fn make_prefix(is_last: bool) -> String {
    let spacing = " ".repeat(3);
    match is_last {
        false => format!("{}{}", VERTICAL_NOBAR, spacing),
        true => format!("{}", spacing.repeat(2)),
    }
}

fn print_tree(tree: &Vec<TreeElement>, prefix: &str) {
    let len = tree.len();
    for i in 0..len {
        let el = &tree[i];
        let last_child = i + 1 == len;
        match el {
            TreeElement::File(file) => {
                if let Some(path) = file.to_str() {
                    println!("{prefix}{}", format_indentation(path, last_child));
                }
            }
            TreeElement::Dir(dir) => {
                if let Some(path) = dir.0.to_str() {
                    println!("{prefix}{}", format_indentation(path, last_child));
                    print_tree(&dir.1, &format!("{prefix}{}", make_prefix(last_child)))
                }
            }
        }
    }
}
