use std::fs;

fn main() {
    let dir_path = ".";//path to directory

    let dir_contents = match fs::read_dir(dir_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    for entry in dir_contents {
        match entry {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }
}
