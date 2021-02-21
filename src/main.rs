/*
 * theCow61 <theCow61@github.com>
 * target: x86_64-unknown-linux-musl
*/

mod server;
mod client;
use taxy::Team;

fn _input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    // let res = input.parse::<u8>()?; // Experiment shows with this that ? if error returns with error and can be used with match on calling function. (Redirects Error)
    Ok(input)
}
fn offline() {
    let mut gridnrend = taxy::GridnRend::new().unwrap();
    loop {
        gridnrend.print_grid();
        gridnrend.inputn_update();
        match gridnrend.checkn_assert() {
            Some(tee) => {
                gridnrend.print_grid();
                if tee == Team::T {
                    println!("Tied game.");
                    break;
                }
                println!("{} has won.", tee);
                break;
            },
            None => continue,
        }
    }
}
fn main() {
    let mut args = std::env::args().skip(1);
    let is_server: bool;
    match args.next().expect("Usage: ./taxy [server/client/offline] [ip:port]").as_str() {
        "server" => { is_server = true; },
        "client" => { is_server = false; },
        "offline" => { offline(); return; },
        _ => panic!("Usage: ./taxy [server/client/offline] [ip]"),
    }
    let hostip = format!("{}", args.next().expect("Usage: ./taxy [server/client/offline] [ip:port]"));
    match is_server {
        true => {
            println!("Hosting on {}", hostip);
            server::run(&hostip);
        },
        false => {
            println!("Connecting to {}", hostip);
            client::run(&hostip);
        }
    }
    /*let mut gridnrend = GridnRend::new().unwrap();
    loop {
        // gridnrend.print_grid();
        // gridnrend.inputn_update();
        GridnRend::print_grid(&gridnrend);
        GridnRend::inputn_update(&mut gridnrend);
        match gridnrend.checkn_assert() {
            Some(tee) => {
                gridnrend.print_grid();
                println!("{} has won...", tee);
                break;
            },
            None => continue,
        }
    }*/
    // println!("{} is winner.", gridnrend.winner.unwrap());
    /*match input() {
        Ok(i) => println!("{}", i),
        Err(_) => println!("format issue"),
    }*/
}
