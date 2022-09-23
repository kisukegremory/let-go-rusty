use regex::Regex;
mod io_handler;

fn replace_string(file: &String, find: &String, replace: &String) -> String {
    let re = Regex::new(find).unwrap();
    return re.replace_all(file, replace).to_string();
}

fn main() {
    let args = io_handler::read_args();
    let mut file = io_handler::read_file(&args);
    file = replace_string(&file, &args.find, &args.replace);
    io_handler::write_file(&args.output_file, &file);
}
