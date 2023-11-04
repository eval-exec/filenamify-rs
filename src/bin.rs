fn main() {
    // this program should only accept only one parameter from cli
    // it can read param from stdin too
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = args[1].clone();
    let safe_filename = filenamify::filenamify(filename);

    // replace safe_filename's space into '_'
    let safe_filename = safe_filename.replace(" ", "_");
    let safe_filename = safe_filename.replace("/", "_");
    let safe_filename = safe_filename.replace("(", "_");
    let safe_filename = safe_filename.replace(")", "_");

    println!("{}", safe_filename);
}
