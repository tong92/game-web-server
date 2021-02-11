use game_web_server;

fn main() {
    println!("{} is call_one", game_web_server::call_one());
    game_web_server::create_server();
}