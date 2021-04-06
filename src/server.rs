use taxy::GridnRend;
use taxy::Team;
use termion::raw::IntoRawMode;

pub fn run(hostip: &str) {
    let mut screen =
        termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    let listener = std::net::TcpListener::bind(hostip).expect("Could not bind...");
    let (mut stream, socket_addr) = listener.accept().expect("Failed to accept...");
    println!("{} has connected...", socket_addr);
    let mut gridnrend = GridnRend::new().unwrap();

    loop {
        gridnrend.print_grid(&mut screen);
        gridnrend.inputn_update(&mut screen);
        gridnrend.print_grid(&mut screen); //
        gridnrend.checkn_assert();

        bincode::serialize_into(&mut stream, &gridnrend).unwrap();
        if gridnrend.winner != None {
            gridnrend.print_grid(&mut screen);
            if gridnrend.winner == Some(Team::T) {
                println!("Tied game.");
                break;
            }
            println!("{} has won.", gridnrend.winner.unwrap());
            break;
        }

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
    }
}
