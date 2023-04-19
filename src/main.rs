use std::{env, fs, process::exit};

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!("Entries in {:?}:", current_dir);

    for entry in fs::read_dir(current_dir).unwrap_or_else(|err| {
        eprintln!("error reading dir: {err}");
        exit(1);
    }) {
        let entry = entry.unwrap_or_else(|err| {
            eprintln!("error reading entry: {err}");
            exit(1);
        });
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap_or_else(|err| {
            eprintln!("error getting metadata: {err}");
            exit(1);
        });
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();

        if metadata.is_file() {
            println!(
                "filename: {:?}, Last modified: {:?} seconds,  size: {:?} bytes, ",
                path.file_name().ok_or("No filename").unwrap(),
                last_modified,
                metadata.len(),
            );
        }
    }
}
