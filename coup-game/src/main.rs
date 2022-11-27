use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must provide player controllers as command line arguments");
        return;
    };

    coup_engine::run_game(&args.get(1..).unwrap().to_vec());
}
