use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = match args.get(1) {
        Some(val) => val,
        None => {
            println!("Num players was not provided!");
            return;
        }
    };

    let num_players = match arg.parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            println!("Unable to parse number of players from argument: {}", e);
            return;
        }
    };

    coup_engine::run_game(num_players);
}
