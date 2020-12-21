mod common;
mod days;

use anyhow::Result;
use days::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    day: u32,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();
    match cli.day {
        1 => day_one::day_one()?,
        2 => day_two::day_two()?,
        3 => day_three::day_three()?,
        4 => day_four::day_four()?,
        5 => day_five::day_five()?,
        6 => day_six::day_six()?,
        7 => day_seven::day_seven()?,
        8 => day_eight::day_eight()?,
        9 => day_nine::day_nine()?,
        10 => day_ten::day_ten()?,
        11 => day_eleven::day_eleven()?,
        12 => day_twelve::day_twelve()?,
        13 => day_thirteen::day_thirteen()?,
        14 => day_fourteen::day_fourteen()?,
        15 => day_fifteen::day_fifteen()?,
        16 => day_sixteen::day_sixteen()?,
        _ => unimplemented!(),
    }

    Ok(())
}
