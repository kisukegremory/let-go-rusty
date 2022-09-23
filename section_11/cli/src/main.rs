use regex::Regex;
use std::env;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Arguments {
    input_file: String,
    output_file: String,
    find: String,
    replace: String,
}

fn replace_string(file: &String, find: &String, replace: &String) -> String {
    let re = Regex::new(find).unwrap();
    return re.replace_all(file, replace).to_string();
}

fn read_args() -> Arguments {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.len() != 4 {
        println!("not 4 arguments - {:?}", arguments);
        std::process::exit(1);
    }

    return Arguments {
        input_file: arguments[0].clone(),
        output_file: arguments[1].clone(),
        find: arguments[2].clone(),
        replace: arguments[3].clone(),
    };
}

fn read_file(args: &Arguments) -> String {
    let file = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            println!("Reading Error {}", e);
            std::process::exit(1)
        }
    };

    return file;
}

fn write_file(path: &String, contents: &String) {
    match fs::write(path, contents) {
        Ok(_) => {}
        Err(e) => {
            println!("Write Error {}", e);
            std::process::exit(1)
        }
    }
}

fn main() {
    let args = read_args();
    let mut file = read_file(&args);
    file = replace_string(&file, &args.find, &args.replace);
    write_file(&args.output_file, &file);
}
