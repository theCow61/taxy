/*
 *
 * TODO: Make output and rendering of grid go through termion so you know where the empty spot
 * locatations are to track the clicking of it.
 *
*/
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)] //impl Copy for Team -- also works
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
        Ok(GridnRend {
            grid_data: grid_chan,
            active_team: Team::X,
            winner: None,
        })
    }
    pub fn print_grid(&self) {
        // Prints the grid with whats already stored in struct's data.
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
                    }
                    Team::O => {
                        print!(" \x1b[1m|\x1b[0m {}", col);
                    }
                    _ => {
                        print!(" \x1b[1m|\x1b[0m {}", col);
                    }
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
        let stdin = std::io::stdin();
        let tup: (u32, u32);
        if termion::is_tty(&std::io::stdin()) {
            tup = validate_input_tty(&self, stdin); // Maybe create stdin variable before this if statement and pass it as an argument to this function and the other non_tty function?
        }
        // End of user input and processing
        else {
            tup = validate_input_non_tty(stdin);
        }

        // self.grid_data[row as usize][col as usize] = self.active_team;

        self.grid_data[tup.0 as usize][tup.1 as usize] = self.active_team;

        /*match self.active_team {
            Team::X => self.active_team = Team::O,
            Team::O => self.active_team = Team::X,
            Team::E => println!("What the?"),
        }*/
    }

    pub fn checkn_assert(&mut self) -> Option<Team> {
        // Some value is team winner, none value means no winner yet
        if self.winner != None {
            return Some(self.winner.unwrap());
        }
        /*for i in 0..3{
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
            if self.grid_data[i][0..3].iter().all(|&i| i == self.active_team) {

            }
        }*/
        for i in 0..3 {
            /*println!("L: {:?}", &self.grid_data[i][..3]);
            if &self.grid_data[i][..3] == &[self.active_team; 3] {
                println!("L");
                self.winner = Some(self.active_team);
                return Some(self.active_team)
            }
            println!("H: {:?}", &self.grid_data[..3][i]);
            if &self.grid_data[..3][i] == &[self.active_team; 3] {
                println!("H");
                self.winner = Some(self.active_team);
                return Some(self.active_team)
            }*/
            if self.grid_data[i][0..3]
                .iter()
                .all(|&i| i == self.active_team)
            {
                println!("L");
                self.winner = Some(self.active_team);
                return Some(self.active_team);
            }
            if self.grid_data[0][i] == self.active_team
                && self.grid_data[1][i] == self.active_team
                && self.grid_data[2][i] == self.active_team
            {
                // I DON'T KNOW WHY THIS WONT BEHAVE
                self.winner = Some(self.active_team);
                return Some(self.active_team);
            }
        }

        if self.grid_data[0][0] == self.active_team
            && self.grid_data[1][1] == self.active_team
            && self.grid_data[2][2] == self.active_team
        {
            self.winner = Some(self.active_team);
            return Some(self.active_team);
        }
        if self.grid_data[2][0] == self.active_team
            && self.grid_data[1][1] == self.active_team
            && self.grid_data[0][2] == self.active_team
        {
            self.winner = Some(self.active_team);
            return Some(self.active_team);
        }
        let mut vextor = Vec::new(); // vector and loop to transform self_grid into a 1d array from a 2d array so i can analyze all the data at once
        for (_i, row) in self.grid_data.iter().enumerate() {
            for (_j, col) in row.iter().enumerate() {
                vextor.push(col);
            }
        }
        if vextor.iter().all(|&i| i != &Team::E) {
            self.winner = Some(Team::T);
            return Some(self.winner.unwrap());
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

fn validate_input_tty(bruh: &GridnRend, stdin: std::io::Stdin) -> (u32, u32) {
    let (row, col) = loop {
        let mut input = String::new();
        let _ = std::io::stdout().flush();
        stdin.read_line(&mut input).expect("Failed to read input");
        if input.len() != 3 {
            // 3 because \n gets counted as a character???
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
            }
        };
        let col_val = match charray[1].to_digit(10) {
            Some(y) => y,
            None => {
                println!("Must be a number.");
                continue;
            }
        };
        if row_val > 2 || col_val > 2 {
            println!("Not valid number on grid");
            continue;
        }
        if bruh.grid_data[row_val as usize][col_val as usize] != Team::E {
            println!("{}:{} is already taken.", row_val, col_val);
            continue;
        }
        break (row_val, col_val);
    };
    (row, col)
}

fn validate_input_non_tty(stdin: std::io::Stdin) -> (u32, u32) {
    // let stdin = stdin();
    let mut stdout = termion::input::MouseTerminal::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}Click where you want it.", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => panic!(),
            Event::Mouse(me) => {
                if let MouseEvent::Press(_, x, y) = me {
                    println!("{}{}", x, y);
                } else {
                }
            }
            _ => {}
        }
    }

    (1, 1)
}
