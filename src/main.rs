mod args;
use clap::Parser;

fn main() {
    let cli = args::Cli::parse();

    // match cli.action.as_str() {
    //     "add" => declarch::add_pkg(&cli.packages),
    //     "rm" => declarch::rm_pkg(&cli.packages),
    //     _ => println!("Bad arguments"),
    // }

    // match declarch::run() {
    //     Ok(_) => (),
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
}
