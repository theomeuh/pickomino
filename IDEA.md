- Have an event bus to split logical business to display / communication business
- recode the logic as an automate and on each transition, emit an event / display somethin


while no winner:
    - read_game_state + available options
    - send action

two things: server + client

define clear interface between server and client


game_state --> cf   pub struct GameState {
                        pub players: Vec<Player>,
                        pub index_current_player: usize, // index in players Vector of the current player
                        pub dominos: Vec<Domino>,
                    }
client actions (game action + game engine action) --> TBD:
    - pick dominos
    - steal Domino
    - draw dice
    - get game_state
    - start game
    - create user
