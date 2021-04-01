/*
 * theCow61 <theCow61@github.com>
 * target: x86_64-unknown-linux-musl
 * TODO: Check shasums of client to make sure binary is same as server (optional)
*/

mod client;
mod server;
use taxy::Team;
use termion::raw::IntoRawMode;

fn _input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    // let res = input.parse::<u8>()?; // Experiment shows with this that ? if error returns with error and can be used with match on calling function. (Redirects Error)
    Ok(input)
}
fn offline() {
    let mut gridnrend = taxy::GridnRend::new().unwrap();
    // let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut screen = termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    loop {
        gridnrend.print_grid(&mut screen);
        gridnrend.inputn_update(&mut screen);
        match gridnrend.checkn_assert() {
            Some(tee) => {
                gridnrend.print_grid(&mut screen);
                if tee == Team::T {
                    println!("Tied game.");
                    break;
                }
                println!("{} has won.", tee);
                break;
            }
            None => continue,
        }
    }
}
fn main() {
    let mut args = std::env::args().skip(1);
    let is_server = match args.next() {
        Some(valv) => match valv.as_str() {
            "server" => true,
            "client" => false,
            "offline" => {
                offline();
                return;
            }
            _ => {
                println!("Usage: ./taxy [server/client/offline] [ip:port]");
                return;
            }
        },
        None => {
            println!("Usage: ./taxy [server/client/offline] [ip:port]");
            return;
        }
    };
    /*let is_server: bool = match args.next().expect("Usage: ./taxy [server/client/offline] [ip:port]").as_str() {
        "server" => true,
        "client" => false,
        "offline" => { offline(); return; },
        _ => { println!("Usage: ./taxy [server/client/offline] [ip:port]"); return },
    }; */
    let hostip = match args.next() {
        Some(valv) => valv,
        None => {
            println!("Usage: ./taxy [server/client/offline] [ip:port]");
            return;
        }
    };
    // let hostip = format!("{}", args.next().expect("Usage: ./taxy [server/client/offline] [ip:port]"));
    match is_server {
        true => {
            println!("Hosting on {}", hostip);
            server::run(&hostip);
        }
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
    // match input() {
    //     Ok(i) => println!("{}", i),
    //     Err(_) => println!("format issue"),
    // }
}
