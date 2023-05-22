use rand::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub number: u8,
}

pub struct Solution {
    pub correct_player: Player,
    pub option_list: Vec<Player>,
}

impl Player {
    pub fn generate_new_player(roster: &mut Vec<Player>, num_options: usize) -> Solution {
        // If we want to be a bit more explicit (and a little more efficient) we can
        // make a handle to the thread-local generator.
        let mut rng = thread_rng();

        /* Generate random player*/
        roster.shuffle(&mut rng);
        // println!("I shuffled my roster: {:?}", roster);

        // Takes random player as first afer shuffle
        let random_player: Player = roster[0].clone();
        // println!("Random player: {:?}", random_player);

        /* Generate n player options, including correct player */
        let mut options: Vec<Player> = Vec::with_capacity(num_options);

        for player in roster.iter().take(num_options) {
            options.push(player.clone());
        }

        /* Shuffle again to mix in random player */
        options.shuffle(&mut rng);
        // println!("I shuffled my options: {:?}", options);

        Solution {
            correct_player: random_player,
            option_list: options,
        }
    }

    pub fn parse_player_line(player_line: String) -> Result<Player, &'static str> {
        let data: Vec<&str> = player_line.split(',').collect();

        if data.len() < 2 {
            // Two data pieces: player name, jersey number
            return Err("Data line in roster csv is empty or too short.");
        }

        let mut data_iter = data.iter();
        let name = match data_iter.next() {
            Some(name) => *name,
            None => return Err("Cannot parse player name entry in roster csv."),
        };

        let number = match data_iter.next() {
            Some(number) => match number.parse::<u8>() {
                Ok(number) => number,
                Err(_e) => return Err("Cannot parse jersey number entry in roster csv."),
            },
            None => return Err("BCannot parse jersey number entry in roster csv."),
        };

        Ok(Player {
            name: name.to_string(),
            number,
        })
    }
}

// fn test_roster() {
//     let mut roster: Vec<Player> = Vec::from([
//         Player {
//             name: "josh".to_string(),
//             number: 11,
//         },
//         Player {
//             name: "claudia".to_string(),
//             number: 17,
//         },
//         Player {
//             name: "lucas".to_string(),
//             number: 1,
//         },
//         Player {
//             name: "izzy".to_string(),
//             number: 12,
//         },
//     ]);

//     let roster_size = roster.len();
//     let num_options: usize = 3;
// }
