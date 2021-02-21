use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]//impl Copy for Team -- also works
pub enum Team {
    O,
    X,
    E,
    T,
}
impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Team::O => "\x1b[32;1mO\x1b[0m",
            Team::X => "\x1b[31;1mX\x1b[0m",
            Team::E => " ",
            Team::T => "T",
            /*
             * TODO: Make colors not static as in say you are client and your team color is red and oponent is green, and then say on server side your team color is also red and your oponent is green,
            */
        };
        write!(f, "{}", printable)
    }
}


#[derive(Serialize, Deserialize)]
pub struct GridnRend {
    // grid_data: [Team; 9],
    pub grid_data: [[Team; 3]; 3],
    pub active_team: Team,
    pub winner: Option<Team>,
}

impl GridnRend {
    pub fn new() -> Result<GridnRend, std::io::Error> {
        let grid_chan: [[Team; 3]; 3] = [[Team::E; 3]; 3];
        Ok(
            GridnRend {
                grid_data: grid_chan,
                active_team: Team::X,
                winner: None,
            }
        )
    }
    pub fn print_grid(&self) { // Prints the grid with whats already stored in struct's data.
        /*let grid_format = format!("    0   1   2 \n  -------------\n0 | {} | {} | {} |\n  -------------\n1 | {} | {} | {} |\n  -------------\n2 | {} | {} | {} |\n  -------------",
                                  self.grid_data[0],
                                  self.grid_data[1],
                                  self.grid_data[2],
                                  self.grid_data[3],
                                  self.grid_data[4],
                                  self.grid_data[5],
                                  self.grid_data[6],
                                  self.grid_data[7],
                                  self.grid_data[8]
        );*/
        // let grid_format = format!("")
        // println!("{}", grid_format);
        /*
         * TODO: Make this color ansi mess go away, figure out way to implement this into the Enum Display impl, also make it so its red no matter what team you are on and make it variable or something like that (Client side has different veiw of colors depending on team then server)
        */
        println!("    \x1b[34;1m0   1   2\x1b[0m");
        for (i, row) in self.grid_data.iter().enumerate() {
            println!("  \x1b[1m-------------\x1b[0m");
            print!("\x1b[34;1m{}\x1b[0m", i);
            for (_j, col) in row.iter().enumerate() {
                match col {
                    Team::X => {
                        print!(" \x1b[1m|\x1b[0m {}", col);
                    },
                    Team::O => {
                        print!(" \x1b[1m|\x1b[0m {}", col);
                    },
                    _ => {
                        print!(" \x1b[1m|\x1b[0m {}", col);
                    },
                }
                // print!(" | {}", col);
            }
            println!(" \x1b[1m|\x1b[0m")
        }
        println!("  \x1b[1m-------------\x1b[0m");

    }
    pub fn inputn_update(&mut self) {
        // todo: make it take input and update with new info
        println!("Which position to plot? ({})", self.active_team);
        let (row, col) = loop {
            let mut input = String::new();
            let _ = std::io::stdout().flush();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            if input.len() != 3 { // 3 because \n gets counted as a character???
                println!("Format is 02 (0 is first horizontal row and 2 is 3 on verticle row)");
                continue;
            }
            input.pop().unwrap();
            let charray: Vec<char> = input.chars().collect();
            let row_val = match charray[0].to_digit(10) {
                Some(y) => y,
                None => {
                    println!("Must be number.");
                    continue;
                },
            };
            let col_val = match charray[1].to_digit(10) {
                Some(y) => y,
                None => {
                    println!("Must be a number.");
                    continue;
                },
            };
            if row_val > 2 || col_val > 2 {
                println!("Not valid number on grid");
                continue;
            }
            if self.grid_data[row_val as usize][col_val as usize] != Team::E {
                println!("{}:{} is already taken.", row_val, col_val);
                continue;
            }
            break (row_val, col_val);
        };
        self.grid_data[row as usize][col as usize] = self.active_team;
        /*match self.active_team {
            Team::X => self.active_team = Team::O,
            Team::O => self.active_team = Team::X,
            Team::E => println!("What the?"),
        }*/
    }

    pub fn checkn_assert(&mut self) -> Option<Team> { // Some value is team winner, none value means no winner yet
        if self.winner != None {
            return Some(self.winner.unwrap());
        }
        for i in 0..3{
            // if self.grid_data[i][0] == self.active_team {
                // println!("hiii");
            // }
            if self.grid_data[i][0] == self.active_team && self.grid_data[i][1] == self.active_team && self.grid_data[i][2] == self.active_team {
                self.winner = Some(self.active_team);
                return Some(self.active_team)
            }
            if self.grid_data[0][i] == self.active_team && self.grid_data[1][i] == self.active_team && self.grid_data[2][i] == self.active_team {
                self.winner = Some(self.active_team);
                return Some(self.active_team)
            }
        }
        if self.grid_data[0][0] == self.active_team && self.grid_data[1][1] == self.active_team && self.grid_data[2][2] == self.active_team {
            self.winner = Some(self.active_team);
            return Some(self.active_team)
        }
        if self.grid_data[2][0] == self.active_team && self.grid_data[1][1] == self.active_team && self.grid_data[0][2] == self.active_team {
            self.winner = Some(self.active_team);
            return Some(self.active_team)
        }
        let mut vextor = Vec::new(); // vector and loop to transform self_grid into a 1d array from a 2d array so i can analyze all the data at once
        for (_i, row) in self.grid_data.iter().enumerate() {
            for (_j, col) in row.iter().enumerate() {
                vextor.push(col);
            }
        }
        if vextor.iter().all(|&i| i != &Team::E) {
            self.winner = Some(Team::T);
            return Some(self.winner.unwrap())
        }
        // switch teams if doesn't win
        match self.active_team {
            Team::X => self.active_team = Team::O,
            Team::O => self.active_team = Team::X,
            Team::E => println!("You have an unfortinate future..."),
            Team::T => println!("Tie"),
        }
        None
    }

}
