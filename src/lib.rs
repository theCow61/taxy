/*
 *
 * TODO: Add a mouse click verification by maybe highlighting by hovering over with mouse
 *
 * TODO: instaid of using get_cursor_pos, track it instaid with the Selection struct so certain terminals dont brake with it.
 *
*/
use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};
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
        let format_o = format!(
            "{}{}O{}",
            termion::style::Bold,
            termion::color::Fg(termion::color::Green),
            termion::style::Reset
        );
        let format_x = format!(
            "{}{}X{}",
            termion::style::Bold,
            termion::color::Fg(termion::color::Red),
            termion::style::Reset
        );
        let printable = match *self {
            Team::O => &format_o,
            Team::X => &format_x,
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
    //stdin: std::io::Stdin,
    //stdout: termion::raw::RawTerminal<std::io::StdoutLock<'a>>,
}

impl GridnRend {
    pub fn new() -> Result<GridnRend, std::io::Error> {
        let grid_chan: [[Team; 3]; 3] = [[Team::E; 3]; 3];
        // let stdout = std::io::stdout();
        Ok(GridnRend {
            grid_data: grid_chan,
            active_team: Team::X,
            winner: None,
            //       stdin: std::io::stdin(),
            //      stdout: stdout.lock().into_raw_mode().unwrap(),
        })
    }
    pub fn print_grid(&self, screen: &mut termion::raw::RawTerminal<std::io::Stdout>) {
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

        // Idek, on the right track with that \n\r stuff tho 😏
        // let stdout = std::io::stdout();
        // let mut stdout = stdout.lock().into_raw_mode().unwrap();
        write!(
            screen,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        screen.flush().unwrap();
        //for (i, row) in self.grid_data.iter().enumerate() {
        //   write!(stdout, "{}---------{}\n\r", termion::style::Bold, termion::style::Reset).unwrap();
        // for (j, col) in row.iter().enumerate() {
        //   write!(stdout, "\n\r{}|{} {}", termion::style::Bold, termion::style::Reset, col).unwrap();
        //}
        // }
        write!(screen, " {}┏━━━┳━━━┳━━━┓{}\n\r", termion::style::Bold, termion::style::Reset).unwrap();
        for (i, row) in self.grid_data.iter() .enumerate(){
            if i != 0 {
            write!(
                screen,
                " {}┣━━━╋━━━╋━━━┫{}\n\r",
                termion::style::Bold,
                termion::style::Reset
            )
            .unwrap();
            }
            for col in row.iter() {
                write!(
                    screen,
                    " {}┃{} {}",
                    termion::style::Bold,
                    termion::style::Reset,
                    col
                )
                .unwrap();
                // print!(" | {}", col);
            }
            write!(
                screen,
                " {}┃{}\n\r",
                termion::style::Bold,
                termion::style::Reset
            )
            .unwrap();
        }
        write!(
            screen,
            " {}┗━━━┻━━━┻━━━┛{}\n\r",
            termion::style::Bold,
            termion::style::Reset
        )
        .unwrap();
    }
    pub fn inputn_update(&mut self, screen: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        // todo: make it take input and update with new info
        println!("Which position to plot? ({})", self.active_team);
        println!(
            "\r{}{}{}Hint: {}{}Use h,j,k,l keys to naviage grid and Enter key to select spot. You can also click with your mouse to select your spot.{}",
            termion::cursor::Hide,
            termion::style::Bold,
            termion::color::Fg(termion::color::LightMagenta),
            termion::style::Reset,
            termion::style::Faint,
            termion::style::Reset
        );
        let stdin = std::io::stdin();
        let tup: (u8, u8);
        if termion::is_tty(&std::io::stdin()) {
            //tup = validate_input_tty(&self, stdin); // Maybe create stdin variable before this if statement and pass it as an argument to this function and the other non_tty function?
            tup = prodjection(stdin, screen, &self.grid_data);
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

fn _validate_input_tty(bruh: &GridnRend, stdin: std::io::Stdin) -> (u32, u32) {
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

fn prodjection(
    stdin: std::io::Stdin,
    screen: &mut termion::raw::RawTerminal<std::io::Stdout>,
    team_grid: &[[Team; 3]; 3],
) -> (u8, u8) {
    write!(screen, "{}", termion::cursor::Goto(1, 2)).unwrap();
    let mut selection = Selection {
        is_selected: false,
        // selected_pos: (1, 2),
    };
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => {
                write!(screen, "{}", termion::clear::All).unwrap();
                panic!();
            }
            Event::Key(Key::Char('j')) => {
                if let Ok(pos) = termion::cursor::DetectCursorPos::cursor_pos(screen) {
                    if pos == (1, 2) {
                        // selection.old_pos = Some((4, 4));
                        selection.is_selected = true;
                        write!(
                            screen,
                            "{}{}{}*{}",
                            termion::cursor::Goto(4, 4),
                            termion::cursor::BlinkingUnderline,
                            termion::style::Blink,
                            termion::style::Reset
                        )
                        .unwrap();
                    } else if pos.1 < 6 {
                        selection.unhighlight(screen, team_grid, pos);
                        // selection.old_pos = Some((pos.0 - 1, pos.1 - 2));
                        selection.is_selected = true;
                        write!(
                            screen,
                            "{}{}{}{}*{}",
                            termion::cursor::Left(1),
                            termion::cursor::Down(2),
                            termion::cursor::BlinkingUnderline,
                            termion::style::Blink,
                            termion::style::Reset
                        )
                        .unwrap();
                    }
                }
            }
            Event::Key(Key::Char('k')) => {
                if let Ok(pos) = termion::cursor::DetectCursorPos::cursor_pos(screen) {
                    if pos.1 > 2 {
                        selection.unhighlight(screen, team_grid, pos);
                        selection.is_selected = true;
                        write!(
                            screen,
                            "{}{}{}{}*{}",
                            termion::cursor::Left(1),
                            termion::cursor::Up(2),
                            termion::cursor::BlinkingUnderline,
                            termion::style::Blink,
                            termion::style::Reset
                        )
                        .unwrap();
                    }
                }
            }
            Event::Key(Key::Char('h')) => {
                // Cant tell if this works or not with the 'h' key until i get those * sorted out
                if let Ok(pos) = termion::cursor::DetectCursorPos::cursor_pos(screen) {
                    if pos.0 > 5 {
                        selection.unhighlight(screen, team_grid, pos);
                        selection.is_selected = true;
                        write!(
                            screen,
                            "{}{}{}*{}",
                            termion::cursor::Left(5),
                            termion::cursor::BlinkingUnderline,
                            termion::style::Blink,
                            termion::style::Reset
                        )
                        .unwrap();
                    }
                }
            }
            Event::Key(Key::Char('l')) => {
                /* TODO: Create a type that displays the * thing when selected and do it in the scope of these events, implement Drop for it so when it gets out of scope you can make the symbol * disapeer... */
                if let Ok(pos) = termion::cursor::DetectCursorPos::cursor_pos(screen) {
                    if pos.0 < 10 {
                        // if let Some(old_pos) = selection {
                        //     write!(screen, "{} ", termion::cursor::Goto(old_pos.0, old_pos.1)).unwrap();
                        // }
                        // selection = Some((pos.0 + 3, pos.1));
                        selection.unhighlight(screen, team_grid, pos);
                        selection.is_selected = true;
                        write!(
                            screen,
                            "{}{}{}*{}",
                            termion::cursor::Right(3),
                            termion::cursor::BlinkingUnderline,
                            termion::style::Blink,
                            termion::style::Reset
                        )
                        .unwrap();
                    }
                }
                //write!(stdout, "{}{}*", termion::cursor::Right(3), termion::cursor::BlinkingUnderline).unwrap();
            }
            Event::Mouse(me) => {
                if let MouseEvent::Press(_, x, y) = me {
                    let gridified = (((x as i32 / 4) - 1), ((y as i32 / 2) - 1));
                    if (0..3).contains(&gridified.1)
                        && (0..3).contains(&gridified.0)
                        && team_grid[gridified.1 as usize][gridified.0 as usize] == Team::E
                    {
                        // return (gridified.0 as u8, gridified.1 as u8)
                        return (gridified.1 as u8, gridified.0 as u8);
                    }
                }
                // TODO: Make it so when your mouse hovers over a spot it highights it and when u click it, it registers it and enables it...
            }
            Event::Key(Key::Char('\n')) => {
                if let Ok(pros) = termion::cursor::DetectCursorPos::cursor_pos(screen) {
                    if pros != (1, 2) {
                        let pos = (pros.0 - 1, pros.1);
                        // let mut alt_screen = termion::screen::AlternateScreen::from(std::io::stdout());
                        // write!(alt_screen, "test").unwrap();
                        // alt_screen.flush().unwrap();
                        // let pos_gridformat = (((pos.0 / 4) - 1) as u8, ((pos.1 / 2) - 1) as u8);
                        let pos_gridformat = ((((pos.1 / 2) - 1) as u8), (((pos.0 / 4) - 1) as u8));
                        if team_grid[pos_gridformat.0 as usize][pos_gridformat.1 as usize]
                            == Team::E
                        {
                            return pos_gridformat;
                        } else {
                            let mut alt_screen = termion::screen::AlternateScreen::from(stdout());
                            write!(
                                alt_screen,
                                "{}{}{}Spot already taken...",
                                termion::cursor::Goto(1, 9),
                                termion::style::Bold,
                                termion::color::Fg(termion::color::LightMagenta)
                            )
                            .unwrap();
                            alt_screen.flush().unwrap();
                        }
                    }
                }
            }
            _ => {}
        }
        screen.flush().unwrap();
    }

    screen.flush().unwrap();
    // TODO: GET ALT SCREEN TO WORK. ADD MOUSE SUPPORT.
    (1, 1)
}

fn validate_input_non_tty(stdin: std::io::Stdin) -> (u8, u8) {
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

struct Selection {
    // old_pos: Option<(u16, u16)>, // To delete
    is_selected: bool,
    // selected_pos: (u16, u16),
}
impl Selection {
    fn unhighlight(
        &self,
        screen: &mut termion::raw::RawTerminal<std::io::Stdout>,
        team_grid: &[[Team; 3]; 3],
        pos: (u16, u16),
    ) {
        if self.is_selected {
            let real_pos = (pos.0 - 1, pos.1);
            let team = team_grid[((real_pos.1 / 2) - 1) as usize][((real_pos.0 / 4) - 1) as usize];
            write!(screen, "{}{}", termion::cursor::Left(1), team).unwrap();
        }
    }
}

impl Drop for GridnRend {
    fn drop(&mut self) {
        println!(
            "\r{}{}Thanks for playing.{}",
            termion::cursor::Down(3),
            termion::style::Faint,
            termion::cursor::Show
        );
    }
}
