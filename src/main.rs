/*
 * theCow61 <theCow61@github.com>
 * target: x86_64-unknown-linux-musl
 * TODO: Check shasums of client to make sure binary is same as server (optional)
 * TODO: Make it say "{} has won" in bold green if YOU won but red if YOU lost.
*/

mod client;
mod server;
use argh::FromArgs;
use taxy::Team;
use termion::raw::IntoRawMode;

#[derive(FromArgs)]
/// TicTacToe 😏
struct Cli {
    #[argh(switch, short = 's')]
    /// listen
    server: bool,
    #[argh(switch, short = 'c')]
    /// connect
    client: bool,
    #[argh(switch, short = 'o')]
    /// offline (by default)
    offline: bool,
    #[argh(option, default = "String::from(\"0.0.0.0\")", short = 'h')]
    /// host to connect/listen (default is localhost)
    host: String,
    #[argh(option, default = "42069", short = 'p')]
    /// port to connect/listen (default is 42069)
    port: u16,
    // enable switch for no mouse terminal
}

fn _input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    // let res = input.parse::<u8>()?; // Experiment shows with this that ? if error returns with error and can be used with match on calling function. (Redirects Error)
    Ok(input)
}
fn offline() {
    let mut gridnrend = taxy::GridnRend::new().unwrap();
    // let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut screen =
        termion::input::MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
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
                println!("{} has \x1b[31;1mwon\x1b[0m.", tee);
                break;
            }
            None => continue,
        }
    }
}
fn main() {
    let cli: Cli = argh::from_env();
    if cli.offline {
        offline();
        return;
    } else if cli.server {
        server::run(&format!("{}:{}", cli.host, cli.port));
        return;
    } else if cli.client {
        client::run(&format!("{}:{}", cli.host, cli.port));
        return;
    } else {
        offline();
        return;
    }
    /*let mut args = std::env::args().skip(1);
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
    let mut hostip = match args.next() {
        Some(valv) => valv,
        None => {
            println!("Usage: ./taxy [server/client/offline] [ip:port]");
            return;
        }
    };
    // let hostip = format!("{}", args.next().expect("Usage: ./taxy [server/client/offline] [ip:port]"));
    match is_server {
        true => {
            let mut split = hostip.split(':');
            if split.next().expect("IP in form of <IP>:42069") == "localhost" {
                // hostip = format!("127.0.0.1:{}", split.next().expect("IP in form <IP>:42069"));
        hostip = format!("127.0.0.1:{}", split.next().unwrap_or("42069"));
            }
            println!("Hosting on {}", hostip);
            server::run(&hostip);
        }
        false => {
            let mut split = hostip.split(':');
            if split.next().expect("IP in form of <IP>:42069") == "localhost" {
                // hostip = format!("127.0.0.1:{}", split.next().expect("IP in form <IP>:42069"));
                hostip = format!("127.0.0.1:{}", split.next().unwrap_or("42069"));
            }
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
    // }*/
}
