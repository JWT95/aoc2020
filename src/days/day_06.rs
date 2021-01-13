use crate::common::read_input;
use anyhow::Result;
use std::collections::HashSet;

type Form = Vec<String>;

pub fn day_06() -> Result<()> {
    let lines = read_input("input/day_06.txt")?;

    let lines: Vec<_> = lines.collect();

    let mut forms = vec![];
    let mut current_form = Form::new();
    for line in lines {
        if line == String::new() {
            forms.push(current_form.clone());
            current_form = Form::new();
        } else {
            current_form.push(line.as_str().into())
        }
    }
    forms.push(current_form);

    part_two(forms);

    Ok(())
}

fn _part_one(forms: Vec<Form>) {
    let total: usize = forms
        .iter()
        .map(|form| form.join(""))
        .map(|form| {
            let form_set: HashSet<char> = form.chars().collect();
            form_set.len()
        })
        .sum();

    println!("{:?}", total);
}

fn part_two_form_value(form: &Form) -> usize {
    form[0]
        .chars()
        .filter(|c| form.iter().all(|row| row.chars().any(|d| d == *c)))
        .count()
}

fn part_two(forms: Vec<Form>) {
    let total: usize = forms.iter().map(|form| part_two_form_value(form)).sum();

    println!("{:?}", total);
}
