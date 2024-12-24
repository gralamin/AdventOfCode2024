extern crate filelib;

use std::collections::HashMap;

pub use filelib::{load, split_lines_by_blanks};
use log::{error, info};

type Number = u128;
type Key = String;
type Instruction = (Key, Key, Key, Operation);

fn parse_variables(lines: &Vec<String>) -> HashMap<Key, Number> {
    let mut map = HashMap::new();
    for line in lines {
        let (name, num_s) = line.split_once(": ").unwrap();
        let num: Number = num_s.parse().unwrap();
        map.insert(name.to_string(), num);
    }
    return map;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn operate(&self, a: Number, b: Number) -> Number {
        return match self {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        };
    }
}

fn parse_operations(lines: &Vec<String>) -> Vec<Instruction> {
    let mut result = vec![];
    for line in lines {
        let (operands, store_in) = line.split_once(" -> ").unwrap();
        let (first_arg, rest) = operands.split_once(" ").unwrap();
        let (op_str, second_arg) = rest.split_once(" ").unwrap();
        let op = match op_str {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Unknown operation {}", op_str),
        };
        result.push((
            first_arg.to_string(),
            second_arg.to_string(),
            store_in.to_string(),
            op,
        ));
    }
    return result;
}

fn can_do_operation(first_arg: &Key, second_arg: &Key, state: &HashMap<Key, Number>) -> bool {
    return state.contains_key(first_arg) && state.contains_key(second_arg);
}

fn do_operation(
    first_arg: &Key,
    second_arg: &Key,
    store_in: &Key,
    op: Operation,
    state: &mut HashMap<Key, Number>,
) {
    let a: Number = *state
        .get(first_arg)
        .expect("can do operation should have already ran");
    let b: Number = *state
        .get(second_arg)
        .expect("can do operation should have already ran");
    let result = op.operate(a, b);
    state.insert(store_in.clone(), result);
}

fn do_instructions(instructions: &Vec<Instruction>, state: &mut HashMap<Key, Number>) {
    let possible: Vec<Instruction> = instructions
        .iter()
        .filter(|(a, b, _, _)| can_do_operation(a, b, &state))
        .cloned()
        .collect();
    if possible.len() == 0 {
        // Impossible to solve now
        return;
    }

    let not_possible: Vec<Instruction> = instructions
        .iter()
        .filter(|(a, b, _, _)| !can_do_operation(a, b, &state))
        .cloned()
        .collect();
    let required: Vec<Instruction> = not_possible
        .iter()
        .filter(|(_, _, store, _)| store.starts_with("z"))
        .cloned()
        .collect();
    for (a, b, c, op) in possible {
        info!(
            "{:?} {:?} {:?} -> {:?}",
            a.clone(),
            b.clone(),
            op,
            c.clone()
        );
        do_operation(&a, &b, &c, op, state);
    }
    if required.len() == 0 {
        return;
    }
    // Otherwise recurse
    do_instructions(&not_possible, state);
}

fn get_number_in_letter(state: &HashMap<Key, Number>, key_starts_with: &str) -> Number {
    let mut num = 0;
    let mut keys: Vec<&Key> = state
        .keys()
        .filter(|k| k.starts_with(key_starts_with))
        .collect();
    keys.sort();
    keys.reverse();
    for k in keys {
        num *= 2;
        num += state.get(k).unwrap();
    }
    return num;
}

/// Run through all of the operations and get the number from the z registers.
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "x00: 1",
///     "x01: 0",
///     "x02: 1",
///     "x03: 1",
///     "x04: 0",
///     "y00: 1",
///     "y01: 1",
///     "y02: 1",
///     "y03: 1",
///     "y04: 1"].iter().map(|s| s.to_string()).collect(),
///     vec!["ntg XOR fgs -> mjb",
///     "y02 OR x01 -> tnw",
///     "kwq OR kpj -> z05",
///     "x00 OR x03 -> fst",
///     "tgd XOR rvg -> z01",
///     "vdt OR tnw -> bfw",
///     "bfw AND frj -> z10",
///     "ffh OR nrd -> bqk",
///     "y00 AND y03 -> djm",
///     "y03 OR y00 -> psh",
///     "bqk OR frj -> z08",
///     "tnw OR fst -> frj",
///     "gnj AND tgd -> z11",
///     "bfw XOR mjb -> z00",
///     "x03 OR x00 -> vdt",
///     "gnj AND wpb -> z02",
///     "x04 AND y00 -> kjc",
///     "djm OR pbm -> qhw",
///     "nrd AND vdt -> hwm",
///     "kjc AND fst -> rvg",
///     "y04 OR y02 -> fgs",
///     "y01 AND x02 -> pbm",
///     "ntg OR kjc -> kwq",
///     "psh XOR fgs -> tgd",
///     "qhw XOR tgd -> z09",
///     "pbm OR djm -> kpj",
///     "x03 XOR y03 -> ffh",
///     "x00 XOR y04 -> ntg",
///     "bfw OR bqk -> z06",
///     "nrd XOR fgs -> wpb",
///     "frj XOR qhw -> z04",
///     "bqk OR frj -> z07",
///     "y03 OR x01 -> nrd",
///     "hwm AND bqk -> z03",
///     "tgd XOR rvg -> z12",
///     "tnw OR pbm -> gnj",
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day24::puzzle_a(&vec1), 2024);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> Number {
    let mut state = parse_variables(string_list.first().unwrap());
    let instructions = parse_operations(string_list.last().unwrap());
    do_instructions(&instructions, &mut state);
    return get_number_in_letter(&state, "z");
}

/// Do num_swaps to find the answer. I don't have good example input for this...
/// ```
/// let vec1: Vec<Vec<String>> = vec![vec![
///     "x00: 1",
///     "x01: 0",
///     "x02: 1",
///     "x03: 1",
///     "x04: 0",
///     "y00: 1",
///     "y01: 1",
///     "y02: 1",
///     "y03: 1",
///     "y04: 1"].iter().map(|s| s.to_string()).collect(),
///     vec!["ntg XOR fgs -> mjb",
///     "y02 OR x01 -> tnw",
///     "kwq OR kpj -> z05",
///     "x00 OR x03 -> fst",
///     "tgd XOR rvg -> z01",
///     "vdt OR tnw -> bfw",
///     "bfw AND frj -> z10",
///     "ffh OR nrd -> bqk",
///     "y00 AND y03 -> djm",
///     "y03 OR y00 -> psh",
///     "bqk OR frj -> z08",
///     "tnw OR fst -> frj",
///     "gnj AND tgd -> z11",
///     "bfw XOR mjb -> z00",
///     "x03 OR x00 -> vdt",
///     "gnj AND wpb -> z02",
///     "x04 AND y00 -> kjc",
///     "djm OR pbm -> qhw",
///     "nrd AND vdt -> hwm",
///     "kjc AND fst -> rvg",
///     "y04 OR y02 -> fgs",
///     "y01 AND x02 -> pbm",
///     "ntg OR kjc -> kwq",
///     "psh XOR fgs -> tgd",
///     "qhw XOR tgd -> z09",
///     "pbm OR djm -> kpj",
///     "x03 XOR y03 -> ffh",
///     "x00 XOR y04 -> ntg",
///     "bfw OR bqk -> z06",
///     "nrd XOR fgs -> wpb",
///     "frj XOR qhw -> z04",
///     "bqk OR frj -> z07",
///     "y03 OR x01 -> nrd",
///     "hwm AND bqk -> z03",
///     "tgd XOR rvg -> z12",
///     "tnw OR pbm -> gnj",
/// ].iter().map(|s| s.to_string()).collect()];
/// assert_eq!(day24::puzzle_b(&vec1, 6), "ffh,mjb,tgd,wpb,z02,z03,z05,z06,z07,z08,z10,z11");
pub fn puzzle_b(string_list: &Vec<Vec<String>>, num_swaps: usize) -> String {
    let instructions: Vec<(String, String, String, Operation)> =
        parse_operations(string_list.last().unwrap());

    let mut successful_swaps: Vec<Key> = vec![];

    // look for the following nodes that are in error, and hope we just end up with the correct amount
    // The other way I thought of only would take 3000 years to run
    for (in_a, in_b, out, op) in instructions.clone() {
        let start_x_a = in_a.starts_with("x");
        let start_x_b = in_b.starts_with("x");
        let start_y_a = in_a.starts_with("y");
        let start_y_b = in_b.starts_with("y");
        let start_z = out.starts_with("z");
        if op == Operation::And && ((start_x_a && start_y_b) || (start_x_b && start_y_a)) {
            if start_z && out != "z00" {
                // problem node, add this output.
                // This is essentially a misplaced adder
                // Look up how an adder circuit works
                successful_swaps.push(out.clone());
                info!(
                    "Identified issue rule 1 on {:?} {:?} {:?} -> {:?}",
                    in_a.clone(),
                    op,
                    in_b.clone(),
                    out.clone()
                );
                continue;
            }
            // We want to drop down in logic for other ruels later
        } else if op == Operation::Xor
            && !start_x_a
            && !start_y_b
            && !start_z
            && !start_y_a
            && !start_x_b
        {
            // A XOR must go onto an X and y, or output to a z
            successful_swaps.push(out.clone());
            info!(
                "Identified issue rule 2 on {:?} {:?} {:?} -> {:?}",
                in_a.clone(),
                op,
                in_b.clone(),
                out.clone()
            );
            continue;
        } else if op != Operation::Xor && start_z && out != "z45" {
            // Other than z45 that zs must be on a Xor, you can find this by manually inspecting the circuit diagram as a graph.
            // z45 happens to be special in this input - becaues its the final output.
            successful_swaps.push(out.clone());
            info!(
                "Identified issue rule 3 on {:?} {:?} {:?} -> {:?}",
                in_a.clone(),
                op,
                in_b.clone(),
                out.clone()
            );
            continue;
        }
        if op == Operation::And
            && ((start_x_a && start_y_b && in_a != "x00")
                || (start_x_b && start_y_a && in_b != "x00"))
        {
            let mut found = false;
            // the intermediate result of an x AND y should be an operand of one of the OR rules
            for (pos_in_a, pos_in_b, _, op) in instructions.iter() {
                if *op == Operation::Or && (out == pos_in_a.clone() || out == pos_in_b.clone()) {
                    found = true;
                    break;
                }
            }
            if !found {
                info!(
                    "Identified issue rule 4 on {:?} {:?} {:?} -> {:?}",
                    in_a.clone(),
                    op,
                    in_b.clone(),
                    out.clone()
                );
                successful_swaps.push(out.clone());
            }
        }
        // the intermediate result of 0 XOR y should be an operand of one of the AND rules
        else if op == Operation::Xor
            && ((start_x_a && start_y_b && in_a != "x00")
                || (start_x_b && start_y_a && in_b != "x00"))
        {
            let mut found = false;
            // the intermediate result of an x AND y should be an operand of one of the OR rules
            for (pos_in_a, pos_in_b, _, op) in instructions.iter() {
                if *op == Operation::And && (out == pos_in_a.clone() || out == pos_in_b.clone()) {
                    found = true;
                    break;
                }
            }
            if !found {
                info!(
                    "Identified issue rule 5 on {:?} {:?} {:?} -> {:?}",
                    in_a.clone(),
                    op,
                    in_b.clone(),
                    out.clone()
                );
                successful_swaps.push(out.clone());
            }
        }
    }

    if successful_swaps.len() != num_swaps * 2 {
        error!("Unexpected number of swaps {}", successful_swaps.len());
    }

    successful_swaps.sort();
    return successful_swaps.join(",");
}
