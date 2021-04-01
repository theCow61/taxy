use taxy::GridnRend;
use taxy::Team;
use termion::raw::IntoRawMode;

pub fn run(hostip: &str) {
    // let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut screen = termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    let listener = std::net::TcpListener::bind(hostip).expect("Could not bind...");
    let (mut stream, socket_addr) = listener.accept().expect("Failed to accept...");
    println!("{} has connected...", socket_addr);
    let mut gridnrend = GridnRend::new().unwrap();
    // gridnrend.print_grid();
    // gridnrend.inputn_update();
    // gridnrend.checkn_assert();
    loop {
        gridnrend.print_grid(&mut screen);
        gridnrend.inputn_update(&mut screen);
        gridnrend.print_grid(&mut screen); //
        gridnrend.checkn_assert();
        // let bytes = bincode::serialize(&mut gridnrend).unwrap();
        // println!("{:?}", bytes);
        // stream.write(&bytes).unwrap();
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
        // let mut toDec: Vec<u8> = Vec::new();
        // let mut dex = loop {
        // match stream.read(&mut toDec) {
        // Ok(_) => { break toDec; },
        // Err(_) => { continue; },
        // }
        // };
        // stream.read(&mut toDec).unwrap();
        // println!("{:?}", toDec);
        // gridnrend = bincode::deserialize(&mut toDec).unwrap();
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
