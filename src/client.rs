use std::io::prelude::*;
use taxy::GridnRend;
use taxy::Team;
use termion::raw::IntoRawMode;

pub fn run(hostip: &str) {
    let mut screen =
        termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    let mut stream = std::net::TcpStream::connect(hostip).expect("Could not connect...");
    let mut gridnrend: GridnRend;
    loop {
        gridnrend = bincode::deserialize_from(&mut stream).unwrap();
        if gridnrend.winner != None {
            gridnrend.print_grid(&mut screen);
            if gridnrend.winner == Some(Team::T) {
                println!("Tied game.");
                break;
            }
            println!("{} has won.", gridnrend.winner.unwrap());
            break;
        }
        gridnrend.print_grid(&mut screen);
        gridnrend.inputn_update(&mut screen);
        gridnrend.print_grid(&mut screen); //
        gridnrend.checkn_assert();
        let bytes = bincode::serialize(&mut gridnrend).unwrap();
        stream.write(&bytes).unwrap();
        if gridnrend.winner != None {
            gridnrend.print_grid(&mut screen);
            if gridnrend.winner == Some(Team::T) {
                println!("Tied game.");
                break;
            }
            println!("{} has won.", gridnrend.winner.unwrap());
            break;
        }
    }
}
