use std::io::prelude::*;
use taxy::GridnRend;
use taxy::Team;
use termion::raw::IntoRawMode;

pub fn run(hostip: &str) {
    // let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut screen =
        termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    let mut stream = std::net::TcpStream::connect(hostip).expect("Could not connect...");
    println!("Has connected to {}", hostip);
    let mut gridnrend: GridnRend;
    let cli_team = MyTeam { my_team: Team::O }; // Will have to be mut in future so you can change and flip sides if restarting game
    loop {
        // let mut toDec: Vec<u8> = Vec::new();
        // let mut dex = loop {
        // match stream.read(&mut toDec) {
        // Ok(_) => { break toDec; },
        // Err(_) => { continue; },
        // }
        // };
        // stream.read(&mut toDec).unwrap();
        // println!("{:?}", toDec);
        if let Ok(temprend) = bincode::deserialize_from::<_, GridnRend>(&mut stream) {
            gridnrend = temprend;
        } else {
            println!("Lost connection...");
            return;
        }
        if let Some(winner) = gridnrend.winner {
            gridnrend.print_grid(&mut screen);
            /*if gridnrend.winner == Some(Team::T) {
                println!("Tied game.");
                break;
            }*/
            // TODO: FLIP TEAM AND MAKE IT DETECT WHO WIN WITHOUT HARDCODING SO WHEN REPEAT GAME
            // FEATURE IS ADDED YOU CAN FLIP WHO GOES FIRST
            /*match gridnrend.winner {
                Some(Team::T) => {
                    println!("Tied game.");
                    break;
                },
                Some(Team::O) => {
                    println!("You({}) have \x1b[32;1mwon\x1b[0m", gridnrend.active_team);
                    break;
                },
                Some(Team::X) => {
                    println!("You({}) have \x1b[31;1mlost\x1b[0m", Team::O);
                    break;
                },
                _ => {},
            }*/
            /*let mycteam = cli_team.my_team;
            println!("{}", mycteam);
            match gridnrend.winner.unwrap() {
                Team::T => {
                    println!("Tied game.");
                    break;
                },
                mycteam => {
                    println!("You({}) have \x1b[32;1mwon\x1b[0m.", mycteam);
                    break;
                },
                _ => {
                    println!("You({}) have \x1b[31;1mlost\x1b[0m.", mycteam);
                    break;
                },
            }*/
            if winner == Team::T {
                println!("Tied game.");
                break;
            }
            if winner == cli_team.my_team {
                    println!("You({}) have \x1b[32;1mwon\x1b[0m.", cli_team.my_team);
                    break;
            } 
            else {
                    println!("You({}) have \x1b[31;1mlost\x1b[0m.", cli_team.my_team);
                    break;
            }
            
            //println!("{} has \x1b[31;1mwon\x1b[0m.", gridnrend.winner.unwrap());
            //break;
        }
        gridnrend.print_grid(&mut screen);
        gridnrend.inputn_update(&mut screen);
        gridnrend.print_grid(&mut screen); //
        gridnrend.checkn_assert();
        let bytes = bincode::serialize(&mut gridnrend).unwrap();
        if let Err(_) = stream.write(&bytes) {
            println!("Lost connection...");
            return;
        }
        if let Some(winner) = gridnrend.winner {
            gridnrend.print_grid(&mut screen);
            /*if gridnrend.winner == Some(Team::T) {
                println!("Tied game.");
                break;
            }*/
            //println!("{} has \x1b[31;1mwon\x1b[0m.", gridnrend.winner.unwrap());
            //break;
            /*match gridnrend.winner {
                Some(Team::T) => {
                    println!("Tied game.");
                    break;
                },
                Some(Team::O) => {
                    println!("You({}) have \x1b[32;1mwon\x1b[0m", gridnrend.active_team);
                    break;
                },
                Some(Team::X) => {
                    println!("You({}) have \x1b[31;1mlost\x1b[0m", Team::O);
                    break;
                },
                _ => {},
            }*/
            /*let mycteam = cli_team.my_team;
            match gridnrend.winner.unwrap() {
                Team::T => {
                    println!("Tied game.");
                    break;
                },
                mycteam => {
                    println!("You({}) have \x1b[32;1mwon\x1b[0m.", mycteam);
                    break;
                },
                _ => {
                    println!("You({}) have \x1b[31;1mlost\x1b[0m.", mycteam);
                    break;
                },
            }*/
            if winner == Team::T {
                println!("Tied game.");
                break;
            }
            if winner == cli_team.my_team {
                    println!("You({}) have \x1b[32;1mwon\x1b[0m.", cli_team.my_team);
                    break;
            }
            else {
                    println!("You({}) have \x1b[31;1mlost\x1b[0m.", cli_team.my_team);
                    break;
            }
        }
    }
}

struct MyTeam{
    my_team: Team,
}
