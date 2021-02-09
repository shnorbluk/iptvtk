mod cli;
mod integ;
mod m3uread;
mod m3uwrite;
mod processor;

//use cli::parse_args;

fn main() {
    //let opts = parse_args(std::env::args_os()).unwrap_or_else(|err: clap::Error| err.exit());
    let opts = integ::main(std::env::args_os());
    println!("{:?}", opts)
}
