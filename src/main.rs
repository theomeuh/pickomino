use crate::server::actions::Actions;

mod client;
mod constant;
mod server;

fn main() {
    let server = server::server::Server::new();
    println!("Server started");
    let client = client::client::Client::new(server);
    println!("Client started\n");

    loop {
        println!("Select an Action");
        println!("{}", Actions(client.list_actions()));

        client.select_action();
    }
}
