use std::env;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
pub struct Arguments {
    pub input_file: String,
    pub output_file: String,
    pub find: String,
    pub replace: String,
}

pub fn read_args() -> Arguments {
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

pub fn read_file(args: &Arguments) -> String {
    let file = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            println!("Reading Error {}", e);
            std::process::exit(1)
        }
    };

    return file;
}

pub fn write_file(path: &String, contents: &String) {
    match fs::write(path, contents) {
        Ok(_) => {}
        Err(e) => {
            println!("Write Error {}", e);
            std::process::exit(1)
        }
    }
}
