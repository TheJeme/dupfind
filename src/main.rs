mod indentprint;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_path: &str = &env::current_dir().unwrap().as_path().display().to_string();

    match args.len() {
        1 => run(current_path),
        2 => run(&args[1]),
        _ => {
            println!("Invalid number of arguments. Usage: `dupfind [path]");
            std::process::exit(0);
        }
    }
    println!("Done!");
}

fn run(target_path: &str) {
    let dir_path = std::fs::read_dir(target_path).unwrap();
    let mut files: HashMap<String, Vec<String>> = HashMap::new();

    for path in dir_path
        .filter_map(|dir_entry| dir_entry.ok())
        .filter(|e| e.metadata().unwrap().is_file())
    {
        let hash = sha256::try_digest(Path::new(&path.path())).unwrap();
        files
            .entry(hash)
            .or_default()
            .push(path.path().to_str().unwrap().to_string());
    }
    let mut auto_delete_duplicates = false;

    let mut files_vec: Vec<String> = Vec::new();
    for (_, v) in files.iter() {
        for s in v {
            files_vec.push(s.to_string());
        }
    }
    if files_vec.len() == files.keys().len() {
        println!("No duplicate files found!");
        std::process::exit(0);
    }
    for (_key, mut duplicate_files) in files {
        if duplicate_files.len() > 1 {
            if auto_delete_duplicates {
                // Delete all except first file
                for (i, _) in duplicate_files[1..].iter().enumerate() {
                    fs::remove_file(&duplicate_files[i + 1]).unwrap();
                }
                continue;
            }

            loop {
                println!(" N  File");
                for (index, file) in duplicate_files.iter().enumerate() {
                    let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
                    println!("[{}] {:}", index + 1, file_name);
                }
                print_help();
                let mut command = String::new();
                std::io::stdin().read_line(&mut command).unwrap();
                let command_args: Vec<&str> = command.split_whitespace().collect();

                // If no arguments given, continue.
                if command_args.is_empty() || command_args.len() > 2 {
                    println!("Invalid command");
                    continue;
                }

                match command_args[0] {
                    "n" => break,
                    "o" => match command_args[1].parse::<usize>() {
                        Err(_) => println!("Invalid file number!"),
                        Ok(file_index) => {
                            if file_index > duplicate_files.len() {
                                println!("Invalid file number!");
                                continue;
                            }
                            let opener_result = opener::open(std::path::Path::new(
                                &duplicate_files[file_index - 1],
                            ));
                            match opener_result {
                                Ok(_) => println!("Opened the file!"),
                                Err(_) => {
                                    println!("Can not open the file!");
                                }
                            }
                        }
                    },
                    "d" => match command_args[1].parse::<usize>() {
                        Err(_) => println!("Invalid file number!"),
                        Ok(file_index) => {
                            if file_index > duplicate_files.len() {
                                println!("Invalid file number!");
                                continue;
                            }
                            fs::remove_file(&duplicate_files[file_index - 1]).unwrap();
                            duplicate_files.remove(file_index - 1);
                            println!("File deleted succesfully!");
                            if duplicate_files.len() == 1 {
                                break;
                            }
                        }
                    },
                    "a" => {
                        // Delete all except first file
                        for (i, _) in duplicate_files[1..].iter().enumerate() {
                            fs::remove_file(&duplicate_files[i + 1]).unwrap();
                        }
                        println!("Deleted all duplicate files!");
                        break;
                    }
                    "A" => {
                        // Delete all except first file
                        for (i, _) in duplicate_files[1..].iter().enumerate() {
                            fs::remove_file(&duplicate_files[i + 1]).unwrap();
                        }
                        auto_delete_duplicates = true;
                        break;
                    }
                    "e" => {
                        println!("Exiting the program!");
                        std::process::exit(0);
                    }
                    _ => println!("Invalid command"),
                }

                println!("---------------------")
            }
        }
    }
}

fn print_help() {
    println!("Please enter command:");
    const HELP: &[[&str; 2]] = &[
        ["'o N'", "- Open specific file"],
        ["'d N'", "- Delete specific file"],
        ["'a'", "- Delete all except [1]"],
        [
            "'A'",
            "- Delete all except [1] and do this to all next files",
        ],
        ["'n'", "- Next files"],
        ["'e'", "- Exit"],
    ];
    for item in HELP {
        indentprint::print(item[0], "", 1);
        indentprint::println(item[1], item[0], 8);
    }
}
