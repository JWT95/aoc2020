use crate::common::read_input;
use anyhow::Result;

const ONE_ROW: char = 'B';
const ONE_COL: char = 'R';

struct PlaneSeat {
    row: u8,
    column: u8,
}

impl PlaneSeat {
    fn from_string(string: String) -> PlaneSeat {
        let chars: Vec<char> = string.chars().collect();
        let mut row = 0;
        for i in 0..7 {
            if chars[i] == ONE_ROW {
                row += 1 << (6 - i);
            }
        }

        let mut column = 0;
        for i in 7..10 {
            if chars[i] == ONE_COL {
                column += 1 << (9 - i);
            }
        }

        PlaneSeat { row, column }
    }

    fn seat_id(&self) -> u32 {
        self.row as u32 * 8 + self.column as u32
    }
}

fn _part_one() -> Result<()> {
    let max_seat = read_input("input/day_five.txt")?
        .map(PlaneSeat::from_string)
        .map(|seat| seat.seat_id())
        .max();

    println!("{:?}", max_seat);

    Ok(())
}

fn part_two() -> Result<()> {
    let mut seat_ids: Vec<_> = read_input("input/day_five.txt")?
        .map(PlaneSeat::from_string)
        .map(|seat| seat.seat_id())
        .collect();

    seat_ids.sort();

    for i in seat_ids[0]..*seat_ids.last().unwrap() {
        if !seat_ids.contains(&i) {
            println!("{:?}", i);
        }
    }

    Ok(())
}

pub fn day_five() -> Result<()> {
    part_two()?;
    Ok(())
}
