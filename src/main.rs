use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Add or Remove a package
    #[arg(value_parser = ["add", "rm"])]
    action: String,
    /// The package
    #[arg(trailing_var_arg = true, num_args(1..))]
    packages: Vec<String>,
}

fn main() {
    let args = Args::parse();

    match args.action.as_str() {
        "add" => declarch::add_pkg(&args.packages),
        "rm" => declarch::rm_pkg(&args.packages),
        _ => println!("Bad arguments"),
    }

    match declarch::run() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
        }
    }
}
