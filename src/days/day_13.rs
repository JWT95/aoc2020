use crate::common::read_input;
use anyhow::Result;

pub fn day_13() -> Result<()> {
    let inputs: Vec<String> = read_input("input/day_13.txt")?.collect();

    part_two(inputs)
}

fn _part_one(inputs: Vec<String>) -> Result<()> {
    let arrival_time = inputs[0].parse::<u64>()?;

    let buses = inputs[1]
        .split(',')
        .map(|x| x.parse::<u64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap());

    let bus = buses
        .map(|bus| (bus, bus - (arrival_time % bus)))
        .min_by(|x, y| x.1.cmp(&y.1));

    println!("{:?}", bus);

    Ok(())
}

fn part_two(inputs: Vec<String>) -> Result<()> {
    let buses: Vec<_> = inputs[1]
        .split(',')
        .enumerate()
        .map(|(i, x)| (i, x.parse::<usize>()))
        .filter(|x| x.1.is_ok())
        .map(|(x, y)| (x, y.unwrap()))
        .map(|(x, y)| (y - (x % y), y))
        .collect();

    println!("{:?}", buses);

    let a: Vec<i128> = buses.iter().map(|x| x.0 as i128).collect();
    let n: Vec<i128> = buses.iter().map(|x| x.1 as i128).collect();

    let result = chinese_remainder_gauss(n, a);

    println!("{:?}", result);

    Ok(())
}

// CRT based solution. Inspired by https://medium.com/@astartekraus/the-chinese-remainder-theorem-ea110f48248c

#[test]
fn test_crg() {
    let n = vec![3, 4, 5];
    let a = vec![0, 3, 4];

    assert_eq!(chinese_remainder_gauss(n, a), 39);
}

fn chinese_remainder_gauss(n: Vec<i128>, a: Vec<i128>) -> i128 {
    let big_n = n.iter().fold(1, |acc, x| acc * x);
    let mut result = 0;
    for i in 0..n.len() {
        let ai = a[i];
        let ni = n[i];
        let bi = big_n / ni;
        result += ai * bi * invmod(bi, ni);
    }

    return ((result % big_n) + big_n) % big_n;
}

fn invmod(a: i128, m: i128) -> i128 {
    let (x, _) = extended_euclid(a, m);

    return x % m;
}

fn extended_euclid(mut x: i128, mut y: i128) -> (i128, i128) {
    let mut x0: i128 = 1;
    let mut x1: i128 = 0;
    let mut y0: i128 = 0;
    let mut y1: i128 = 1;

    while y > 0 {
        let q = x / y;
        let old_x = x;
        let old_y = y;
        x = y;
        y = old_x % old_y;
        let old_x0 = x0;
        let old_x1 = x1;
        x0 = x1;
        x1 = old_x0 - q * old_x1;
        let old_y0 = y0;
        let old_y1 = y1;
        y0 = y1;
        y1 = old_y0 - q * old_y1;
    }

    return (x0, y0);
}
