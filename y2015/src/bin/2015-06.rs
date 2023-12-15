use std::{fmt::Display, io};

use aoc_shared::{fold_decimal_from, read_input, FlatVec2D};
use regex::bytes::Regex;

#[derive(Copy, Clone)]
enum Action {
    On,
    Off,
    Toggle,
}

struct Instruction {
    pub verb: Action,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

struct InstrIter<'a> {
    curr_iter: (usize, usize),
    instruction: &'a Instruction,
}

impl<'a> InstrIter<'a> {
    fn new(instruction: &'a Instruction) -> Self {
        Self {
            curr_iter: instruction.start,
            instruction,
        }
    }
}

impl Instruction {
    pub fn new(verb: Action, start: (usize, usize), end: (usize, usize)) -> Self {
        Instruction { verb, start, end }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.verb {
            Action::On => f.write_str("turn on ")?,
            Action::Off => f.write_str("turn off ")?,
            Action::Toggle => f.write_str("toggle ")?,
        };
        f.write_fmt(format_args!(
            "{},{} through {},{}",
            self.start.0, self.start.1, self.end.0, self.end.1,
        ))
    }
}

impl<'a> Iterator for InstrIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (currx, curry) = self.curr_iter;
        let startx = self.instruction.start.0;
        let lastx = self.instruction.end.0;
        let lasty = self.instruction.end.1;
        // start and end are inclusive intervals [start.x, end.x], [start.y, end.y]
        if curry <= lasty {
            let ret = Some(self.curr_iter);
            if currx < lastx {
                self.curr_iter.0 += 1;
            } else {
                self.curr_iter.0 = startx;
                self.curr_iter.1 += 1;
            }
            ret
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Instruction {
    type Item = <InstrIter<'a> as Iterator>::Item;

    type IntoIter = InstrIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        InstrIter::new(self)
    }
}

fn parse(input: Vec<u8>) -> Vec<Instruction> {
    let re = Regex::new(
        r#"(?x)
    (?:turn)?
    [[:space:]]*
    (?<verb>on|off|toggle)
    [[:space:]]+
    (?<start1>[[:digit:]]+),(?<start2>[[:digit:]]+)
    [[:space:]]+
    (?:through)
    [[:space:]]+
    (?<end1>[[:digit:]]+),(?<end2>[[:digit:]]+)
    [[:space:]]*
"#,
    )
    .unwrap();

    re.captures_iter(&input)
        .map(|captures| {
            let verb = captures
                .name("verb")
                .expect("every input line needs an action.");

            let start1 = captures
                .name("start1")
                .map(|m| fold_decimal_from(m.as_bytes()))
                .expect("every input needs a starting x coord.");
            let start2 = captures
                .name("start2")
                .map(|m| fold_decimal_from(m.as_bytes()))
                .expect("every input needs a starting y coord.");

            let end1 = captures
                .name("end1")
                .map(|m| fold_decimal_from(m.as_bytes()))
                .expect("every input needs a ending x coord.");
            let end2 = captures
                .name("end2")
                .map(|m| fold_decimal_from(m.as_bytes()))
                .expect("every input needs a ending y coord.");

            Instruction::new(
                match verb.as_bytes() {
                    b"on" => Action::On,
                    b"off" => Action::Off,
                    b"toggle" => Action::Toggle,
                    _ => unreachable!(),
                },
                (start1, start2),
                (end1, end2),
            )
        })
        .collect()
}

fn run_instruction(lights: &mut FlatVec2D<bool>, instruction: &Instruction) {
    let action = instruction.verb;
    for coords in instruction {
        match action {
            Action::On => lights[coords] = true,
            Action::Off => lights[coords] = false,
            Action::Toggle if lights[coords] => {
                lights[coords] = false;
            }
            Action::Toggle => {
                lights[coords] = true;
            }
        }
    }
}

fn run_instruction_2(lights: &mut FlatVec2D<u64>, instruction: &Instruction) {
    let action = instruction.verb;
    for coords in instruction {
        match action {
            Action::On => lights[coords] += 1,
            Action::Off => lights[coords] = lights[coords].saturating_sub(1),
            Action::Toggle => lights[coords] += 2,
        }
    }
}

fn count_lights_on(lights: &FlatVec2D<bool>) -> usize {
    lights.0.iter().filter(|&&val| val).count()
}

fn total_brightness(lights: &FlatVec2D<u64>) -> u64 {
    lights.0.iter().sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let instructions = parse(input);

    let mut lights: FlatVec2D<bool> = FlatVec2D::new(1000, 1000);
    let mut lights_varbright: FlatVec2D<u64> = FlatVec2D::new(1000, 1000);

    for instruction in instructions {
        run_instruction(&mut lights, &instruction);
        run_instruction_2(&mut lights_varbright, &instruction)
    }

    let cnt = count_lights_on(&lights);
    let brt = total_brightness(&lights_varbright);
    println!("Part1 {cnt}, Part2 {brt}");

    Ok(())
}

#[cfg(test)]
mod test {
    use aoc_shared::FlatVec2D;

    use crate::{count_lights_on, run_instruction, Action, Instruction};

    #[test]
    fn examples_p1() {
        use Action::*;

        // turns on all lights
        let instr1 = Instruction::new(On, (0, 0), (999, 999));
        let mut lights: FlatVec2D<bool> = FlatVec2D::new(1000, 1000);
        run_instruction(&mut lights, &instr1);
        assert_eq!(count_lights_on(&lights), 1000 * 1000);

        // turns off first row
        let instr2 = Instruction::new(Toggle, (0, 0), (999, 0));
        run_instruction(&mut lights, &instr2);
        assert_eq!(count_lights_on(&lights), 1000 * 999);

        // turns off 4 lights
        let instr2 = Instruction::new(Off, (499, 499), (500, 500));
        run_instruction(&mut lights, &instr2);
        assert_eq!(count_lights_on(&lights), (1000 * 999) - 4);
    }
}
