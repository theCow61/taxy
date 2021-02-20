use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]//impl Copy for Team -- also works
pub enum Team {
    O,
    X,
    E,
}
impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            Team::O => 'O',
            Team::X => 'X',
            Team::E => ' ',
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
        println!("    0   1   2");
        for (i, row) in self.grid_data.iter().enumerate() {
            println!("  -------------");
            print!("{}", i);
            for (_j, col) in row.iter().enumerate() {
                print!(" | {}", col);
            }
            println!(" |")
        }
        println!("  -------------");

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
        // switch teams if doesn't win
        match self.active_team {
            Team::X => self.active_team = Team::O,
            Team::O => self.active_team = Team::X,
            Team::E => println!("You have an unfortinate future..."),
        }
        None
    }

}
