use shelly::{say, Opt};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{}", say(opt)?);

    Ok(())
}
