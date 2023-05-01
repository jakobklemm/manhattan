use clap::Parser;
use manhattan::Arguments;
use manhattan::Error;

fn main() -> Result<(), Error> {
    let _args = Arguments::parse();
    println!("Hello, world!");

    Ok(())
}
