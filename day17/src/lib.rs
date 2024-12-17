extern crate filelib;

pub use filelib::{load, split_lines_by_blanks};
use log::info;
use std::collections::HashMap;

type Number = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Instruction {
    Adv(Operand),
    Bxl(NonComboOperand),
    Bst(Operand),
    Jnz(NonComboOperand),
    Bxc(Operand),
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Operand {
    Literal(Number),
    RegisterA,
    RegisterB,
    RegisterC,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum NonComboOperand {
    Literal(Number),
}

const A: char = 'A';
const B: char = 'B';
const C: char = 'C';

#[derive(Debug, Clone, Eq, PartialEq)]
struct Computer {
    registers: HashMap<char, Number>,
    instruction_pointer: usize,
    output_buffer: Vec<String>,
}

impl Computer {
    fn new() -> Computer {
        return Computer {
            registers: HashMap::new(),
            instruction_pointer: 0,
            output_buffer: vec![],
        };
    }

    fn set_register(&mut self, register: char, value: Number) {
        *self.registers.entry(register).or_insert(0) = value;
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        info!("Handling {:?}", instruction);
        match instruction {
            Instruction::Adv(op) => self.handle_adv(op),
            Instruction::Bxl(op) => self.handle_bxl(op),
            Instruction::Bst(op) => self.handle_bst(op),
            Instruction::Jnz(op) => self.handle_jnz(op),
            Instruction::Bxc(op) => self.handle_bxc(op),
            Instruction::Out(op) => self.handle_out(op),
            Instruction::Bdv(op) => self.handle_bdv(op),
            Instruction::Cdv(op) => self.handle_cdv(op),
        }
        info!(
            "Instruction pointer: {}, Registers: {:?}",
            self.instruction_pointer, self.registers
        );
    }

    fn handle_adv(&mut self, op: Operand) {
        *self.registers.entry(A).or_insert(0) = self.handle_dv(op);
        self.instruction_pointer += 2;
    }

    fn handle_dv(&self, op: Operand) -> Number {
        info!(
            "Getting A {:?} {:?}",
            self.registers.get(&A),
            self.registers
        );
        let numerator = self.registers.get(&A).unwrap();
        info!("Got A");
        let combo = self.get_op_value(op);
        let base: Number = 2;
        let denominator = base.pow(combo as u32);
        return numerator / denominator;
    }

    fn get_op_value(&self, op: Operand) -> Number {
        let combo = match op {
            Operand::Literal(z) => z,
            Operand::RegisterA => *self.registers.get(&A).unwrap(),
            Operand::RegisterB => *self.registers.get(&B).unwrap(),
            Operand::RegisterC => *self.registers.get(&C).unwrap(),
        };
        return combo;
    }

    fn handle_bxl(&mut self, op: NonComboOperand) {
        let register_b = self.registers.get(&B).unwrap();
        let combo = match op {
            NonComboOperand::Literal(z) => z,
        };
        // ^ here is XOR
        *self.registers.entry(B).or_insert(0) = register_b ^ combo;
        self.instruction_pointer += 2;
    }

    fn handle_bst(&mut self, op: Operand) {
        let combo = self.get_op_value(op);
        *self.registers.entry(B).or_insert(0) = combo % 8;
        self.instruction_pointer += 2;
    }

    fn handle_jnz(&mut self, op: NonComboOperand) {
        if *self.registers.get(&A).unwrap() == 0 {
            self.instruction_pointer += 2;
            return;
        }
        let combo = match op {
            NonComboOperand::Literal(z) => z,
        };
        self.instruction_pointer = combo as usize;
    }

    fn handle_bxc(&mut self, _op: Operand) {
        let register_b = self.registers.get(&B).unwrap();
        let register_c = self.registers.get(&C).unwrap();
        *self.registers.entry(B).or_insert(0) = register_b ^ register_c;
        self.instruction_pointer += 2;
    }

    fn handle_out(&mut self, op: Operand) {
        let combo = self.get_op_value(op);
        self.output_buffer.push((combo % 8).to_string());
        self.instruction_pointer += 2;
    }

    fn handle_bdv(&mut self, op: Operand) {
        *self.registers.entry(B).or_insert(0) = self.handle_dv(op);
        self.instruction_pointer += 2;
    }

    fn handle_cdv(&mut self, op: Operand) {
        *self.registers.entry(C).or_insert(0) = self.handle_dv(op);
        self.instruction_pointer += 2;
    }
}

fn parse_register(registers: &Vec<String>) -> Computer {
    let mut result = Computer::new();
    for line in registers {
        let (register_rest, value_string) = line.split_once(": ").unwrap();
        let (_, character) = register_rest.split_once(" ").unwrap();
        let c: char = character.chars().next().unwrap();
        let v: Number = value_string.parse().unwrap();
        result.set_register(c, v);
    }
    return result;
}

fn parse_program(instructions: &Vec<String>) -> Vec<char> {
    let (_, string) = instructions.first().unwrap().split_once(": ").unwrap();
    let parts = string.split(",");
    let program: Vec<&str> = parts.collect();
    return program
        .into_iter()
        .map(|s| s.chars().next().unwrap())
        .collect();
}

fn parse_op(op: char) -> Operand {
    return match op {
        '0' => Operand::Literal(0),
        '1' => Operand::Literal(1),
        '2' => Operand::Literal(2),
        '3' => Operand::Literal(3),
        '4' => Operand::RegisterA,
        '5' => Operand::RegisterB,
        '6' => Operand::RegisterC,
        '7' => panic!("Reserved, should not appear"),
        _ => panic!("Unknown operand {}", op),
    };
}

fn parse_non_combo(op: char) -> NonComboOperand {
    let num: Number = op.to_digit(10).unwrap() as u64;
    return NonComboOperand::Literal(num);
}

fn parse_instruction(ins: char, op: Operand, lit: NonComboOperand) -> Instruction {
    return match ins {
        '0' => Instruction::Adv(op),
        '1' => Instruction::Bxl(lit),
        '2' => Instruction::Bst(op),
        '3' => Instruction::Jnz(lit),
        '4' => Instruction::Bxc(op),
        '5' => Instruction::Out(op),
        '6' => Instruction::Bdv(op),
        '7' => Instruction::Cdv(op),
        _ => panic!("Unknown Ins {}", ins),
    };
}

/// Compute the output
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "Register A: 729",
///     "Register B: 0",
///     "Register C: 0",
/// ].iter().map(|s| s.to_string()).collect(), vec![
///     "Program: 0,1,5,4,3,0",
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day17::puzzle_a(&vec1), "4,6,3,5,6,3,5,2,1,0");
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> String {
    let mut computer = parse_register(string_list.first().unwrap());
    let program: Vec<char> = parse_program(string_list.last().unwrap());
    run_program(&mut computer, &program);
    return computer.output_buffer.join(",");
}

fn run_program(computer: &mut Computer, program: &Vec<char>) {
    while computer.instruction_pointer < program.len() - 1 {
        let operand = parse_op(program[computer.instruction_pointer + 1]);
        let lit = parse_non_combo(program[computer.instruction_pointer + 1]);
        let instruction = parse_instruction(program[computer.instruction_pointer], operand, lit);
        computer.handle_instruction(instruction);
    }
}

fn is_program(computer: &Computer, program: &Vec<char>) -> bool {
    let potential_program: Vec<char> = computer
        .output_buffer
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect();
    return potential_program == *program;
}

/// Find a value such that the result = program
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "Register A: 2024",
///     "Register B: 0",
///     "Register C: 0",
/// ].iter().map(|s| s.to_string()).collect(), vec![
///     "Program: 0,3,5,4,3,0",
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day17::puzzle_b(&vec1), 117440);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> Number {
    let computer = parse_register(string_list.first().unwrap());
    let program: Vec<char> = parse_program(string_list.last().unwrap());
    let mut iniital_a: Number;
    let mut cur_computer;
    // The period of the digits seem to change after a fixed period (8^n).
    let mut factors: Vec<Number> = vec![0; program.len()];
    let base: Number = 8;
    loop {
        iniital_a = 0;
        for (i, f) in factors.iter().enumerate() {
            iniital_a += base.pow(i as u32) * f;
        }
        cur_computer = computer.clone();
        cur_computer.set_register(A, iniital_a);
        run_program(&mut cur_computer, &program);

        if is_program(&cur_computer, &program) {
            break;
        }
        // Start from least significant digit
        for i in (0..program.len()).rev() {
            // This output is too short, try to extend it by upping factor.
            if cur_computer.output_buffer.len() < i {
                factors[i] += 1;
                break;
            }
            // Easy case, we don't match at this digit, change it so we hopefully do!
            let c = cur_computer.output_buffer[i].chars().next().unwrap();
            if c != program[i] {
                factors[i] += 1;
                break;
            }
        }
    }
    return iniital_a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv() {
        let mut computer = Computer::new();
        computer.set_register(A, 225);
        let operand = Operand::Literal(1);
        let instruction = Instruction::Adv(operand);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&A).unwrap(), 112);
        assert_eq!(computer.instruction_pointer, 2);
    }

    #[test]
    fn test_bxl() {
        let mut computer = Computer::new();
        computer.set_register(B, 225);
        let operand = NonComboOperand::Literal(2);
        let instruction = Instruction::Bxl(operand);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&B).unwrap(), 227);
        assert_eq!(computer.instruction_pointer, 2);
    }

    #[test]
    fn test_bst() {
        let mut computer = Computer::new();
        computer.set_register(A, 225);
        computer.set_register(B, 0);
        let operand = Operand::RegisterA;
        let instruction = Instruction::Bst(operand);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&B).unwrap(), 225 % 8);
    }

    #[test]
    fn test_jnz() {
        let mut computer = Computer::new();
        computer.set_register(A, 0);
        let operand = NonComboOperand::Literal(1);
        let instruction = Instruction::Jnz(operand);
        computer.handle_instruction(instruction);
        assert_eq!(computer.instruction_pointer, 2);
        computer.set_register(A, 20);
        computer.handle_instruction(instruction);
        assert_eq!(computer.instruction_pointer, 1);
    }

    #[test]
    fn test_bxc() {
        let mut computer = Computer::new();
        computer.set_register(B, 225);
        computer.set_register(C, 2);
        let operand = Operand::Literal(2);
        let instruction = Instruction::Bxc(operand);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&B).unwrap(), 227);
        assert_eq!(*computer.registers.get(&C).unwrap(), 2);
        assert_eq!(computer.instruction_pointer, 2);
    }

    #[test]
    fn test_out() {
        let mut computer = Computer::new();
        computer.set_register(B, 225);
        let operand = Operand::RegisterB;
        let instruction = Instruction::Out(operand);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&B).unwrap(), 225);
        assert_eq!(computer.output_buffer, vec!["1"]);
        assert_eq!(computer.instruction_pointer, 2);
        computer.set_register(B, 92);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&B).unwrap(), 92);
        assert_eq!(computer.output_buffer, vec!["1", "4"]);
        assert_eq!(computer.instruction_pointer, 4);
    }

    #[test]
    fn test_bdv() {
        let mut computer = Computer::new();
        computer.set_register(A, 225);
        let operand = Operand::Literal(1);
        let instruction = Instruction::Bdv(operand);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&A).unwrap(), 225);
        assert_eq!(*computer.registers.get(&B).unwrap(), 112);
        assert_eq!(computer.instruction_pointer, 2);
    }

    #[test]
    fn test_cdv() {
        let mut computer = Computer::new();
        computer.set_register(A, 225);
        let operand = Operand::Literal(1);
        let instruction = Instruction::Cdv(operand);
        computer.handle_instruction(instruction);
        assert_eq!(*computer.registers.get(&A).unwrap(), 225);
        assert_eq!(*computer.registers.get(&C).unwrap(), 112);
        assert_eq!(computer.instruction_pointer, 2);
    }
}
