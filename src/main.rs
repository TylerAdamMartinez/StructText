use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
struct Args {
    values: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.values.is_empty() {
        eprintln!("At least one argument is needed");
        return;
    }

    for arg in args.values {
        println!("{arg}");
    }
}
