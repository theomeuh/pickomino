use std::{io, str::FromStr};

use crate::server::actions::{Action, Actions};
use crate::server::server::Server;

pub struct Client {
    server: Server,
}

impl Client {
    pub fn new(server: Server) -> Client {
        Client { server }
    }

    pub fn list_actions(&self) -> Vec<Action> {
        println!("Listing Actions\n");
        self.server.list_actions()
    }

    pub fn select_action(&self) {
        let action = Client::parse_action();

        match action {
            Action::ListActions => println!("{}", Actions(self.list_actions())),
        }
    }

    // parsers

    fn parse_action() -> Action {
        println!("Please input the next action");
        loop {
            let mut action_input = String::new();

            io::stdin()
                .read_line(&mut action_input)
                .expect("Failed to read line");

            match Action::from_str(&action_input.trim()) {
                Ok(action) => return action,
                Err(_) => {
                    println!("Invalid action");
                    continue;
                }
            }
        }
    }
}
