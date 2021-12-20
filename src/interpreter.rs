use std::io::Read;
use std::io::Write;

const MAX_STACK_SIZE: usize = 1000000;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub(crate) enum Instruction {
    IncrementPtr,
    DecrementPtr,
    IncrementValueAtPtr,
    DecrementValueAtPtr,
    PutChar,
    GetChar,
    WhileBegin,
    WhileEnd,
    Exit,
}

pub(crate) fn interpret(instructions: &[Instruction]) {
    let mut stack: [i32; MAX_STACK_SIZE] = [0; MAX_STACK_SIZE];
    let mut sp: usize = 0;
    let mut ip: usize = 0;

    loop {
        let instruction = instructions[ip];
        match instruction {
            Instruction::IncrementPtr => sp += 1,
            Instruction::DecrementPtr => sp -= 1,
            Instruction::IncrementValueAtPtr => stack[sp] += 1,
            Instruction::DecrementValueAtPtr => stack[sp] -= 1,
            Instruction::GetChar => {
                let ch = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as i32)
                    .expect("unable to get char");
                stack[sp] = ch;
            }
            Instruction::PutChar => {
                let ch = stack[sp];
                std::io::stdout()
                    .write(&[ch as u8])
                    .expect("unable to write char");
            }
            Instruction::WhileBegin => {
                if stack[sp] == 0 {
                    let mut j = ip;
                    let mut brace_count = 0;
                    while j < instructions.len() {
                        if instructions[j] == Instruction::WhileBegin {
                            brace_count += 1;
                        } else if instructions[j] == Instruction::WhileEnd {
                            if brace_count == 0 {
                                break;
                            } else {
                                brace_count -= 1
                            }
                        }
                        j += 1
                    }
                    assert!(brace_count == 0, "unclosed '[' loop");
                    ip = j;
                }
            }
            Instruction::WhileEnd => {
                if stack[sp] != 0 {
                    let mut j = (ip - 1) as i32;
                    let mut brace_count = 0;
                    while j >= 0 {
                        if instructions[j as usize] == Instruction::WhileBegin {
                            if brace_count == 0 {
                                break;
                            } else {
                                brace_count -= 1
                            }
                        } else if instructions[j as usize] == Instruction::WhileEnd {
                            brace_count += 1;
                        }
                        j -= 1;
                    }
                    assert!(brace_count == 0, "unclosed ']' loop");
                    ip = if j < 0 { 1 } else { (j) as usize };
                }
            }
            Instruction::Exit => std::process::exit(0),
        };
        ip += 1;
    }
}
