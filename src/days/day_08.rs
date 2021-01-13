use crate::common::read_input;
use anyhow::{Error, Result};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction: Vec<&str> = s.split(' ').collect();

        Ok(match instruction[0] {
            "acc" => Instruction::Acc(instruction[1].parse::<i32>()?),
            "nop" => Instruction::Nop(instruction[1].parse::<i32>()?),
            "jmp" => Instruction::Jmp(instruction[1].parse::<i32>()?),
            _ => anyhow::bail!("Unknown"),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    NotDone,
    Done,
}

impl Default for State {
    fn default() -> Self {
        State::NotDone
    }
}

#[derive(Debug, Default, Clone)]
struct Computer {
    head: i32,
    instructions: Vec<Instruction>,
    previous: HashSet<i32>,
    acc: i32,
    state: State,
}

impl Computer {
    fn run_step(&mut self) -> Result<()> {
        // Add head to the previous heads
        if self.previous.contains(&self.head) {
            anyhow::bail!("Repeat");
        } else {
            self.previous.insert(self.head.clone());
        }

        // Run the instruction
        let instruction = self.instructions.get(self.head as usize).copied();
        if let Some(instruction) = instruction {
            self.run_instruction(&instruction);
        } else {
            self.state = State::Done;
            anyhow::bail!("finished")
        }

        Ok(())
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Acc(acc) => {
                self.head += 1;
                self.acc += acc;
            }
            Instruction::Nop(_) => {
                self.head += 1;
            }
            Instruction::Jmp(jmp) => {
                self.head += jmp;
            }
        }
    }

    fn run(&mut self) {
        while !self.run_step().is_err() {}
    }
}

fn find_wrong_instruction(computer: &Computer) -> i32 {
    let num_instructions = computer.instructions.len();

    for i in 0..num_instructions {
        let mut test_computer = computer.clone();
        change_value_at(&mut test_computer, i);
        test_computer.run();

        if test_computer.state == State::Done {
            return test_computer.acc;
        }
    }

    -1
}

fn change_value_at(computer: &mut Computer, index: usize) {
    let value = computer.instructions[index];
    match value {
        Instruction::Jmp(jmp) => computer.instructions[index] = Instruction::Nop(jmp),
        Instruction::Nop(jmp) => computer.instructions[index] = Instruction::Nop(jmp),
        _ => {}
    }
}

pub fn day_08() -> Result<()> {
    let instructions: Vec<Instruction> = read_input("input/day_08.txt")?
        .map(|instruction| Instruction::from_str(&instruction).unwrap())
        .collect();

    let computer = Computer {
        instructions,
        ..Computer::default()
    };

    println!("{:?}", find_wrong_instruction(&computer));

    Ok(())
}
