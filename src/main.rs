mod server;
mod client;

fn _input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    // let res = input.parse::<u8>()?; // Experiment shows with this that ? if error returns with error and can be used with match on calling function. (Redirects Error)
    Ok(input)
}
fn main() {
    let mut args = std::env::args().skip(1);
    let is_server: bool;
    match args.next().expect("Usage: ./taxy [server/client] [ip]").as_str() {
        "server" => { is_server = true; },
        "client" => { is_server = false; },
        _ => panic!("Usage: ./taxy [server/client] [ip]"),
    }
    let hostip = format!("{}:42069", args.next().expect("Usage: ./taxy [server/client] [ip]"));
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
