use std::io::prelude::*;
use taxy::GridnRend;

pub fn run(hostip: &str) {
    let mut stream = std::net::TcpStream::connect(hostip).expect("Could not connect...");
    let mut gridnrend: GridnRend;
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
        gridnrend = bincode::deserialize_from(&mut stream).unwrap();
        if gridnrend.winner != None {
            gridnrend.print_grid();
            println!("{} has won.", gridnrend.winner.unwrap());
            break;
        }
        gridnrend.print_grid();
        gridnrend.inputn_update();
        gridnrend.print_grid(); //
        gridnrend.checkn_assert();
        let bytes = bincode::serialize(&mut gridnrend).unwrap();
        stream.write(&bytes).unwrap();
        if gridnrend.winner != None {
            gridnrend.print_grid();
            println!("{} has won.", gridnrend.winner.unwrap());
            break;
        }
    }

}