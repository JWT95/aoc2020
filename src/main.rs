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
        1 => day_01::day_01()?,
        2 => day_02::day_02()?,
        3 => day_03::day_03()?,
        4 => day_04::day_04()?,
        5 => day_05::day_05()?,
        6 => day_06::day_06()?,
        7 => day_07::day_07()?,
        8 => day_08::day_08()?,
        9 => day_09::day_09()?,
        10 => day_10::day_10()?,
        11 => day_11::day_11()?,
        12 => day_12::day_12()?,
        13 => day_13::day_13()?,
        14 => day_14::day_14()?,
        15 => day_15::day_15()?,
        16 => day_16::day_16()?,
        17 => day_17::day_17()?,
        18 => day_18::day_18()?,
        19 => day_19::day_19()?,
        20 => day_20::day_20()?,
        21 => day_21::day_21()?,
        22 => day_22::day_22()?,
        23 => day_23::day_23()?,
        24 => day_24::day_24()?,
        25 => day_25::day_25()?,
        _ => unimplemented!(),
    }

    Ok(())
}
