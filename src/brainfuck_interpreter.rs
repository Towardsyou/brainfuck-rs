use std::io::Read;

use crate::opcode::Code;

pub struct BFInterpreter {
    stk: Vec<u32>,
    pc: usize, // program counter for which op to execute next
    sp: usize, // stack pointer for which data we are currently on
}

impl Default for BFInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl BFInterpreter {
    pub fn new() -> Self {
        BFInterpreter {
            stk: vec![0; 1],
            pc: 0,
            sp: 0,
        }
    }

    pub fn run(&mut self, input: &str) {
        let code = Code::from(input);
        loop {
            if self.pc >= code.ops.len() {
                break;
            }
            let op = &code.ops[self.pc];
            match *op {
                crate::opcode::Opcode::SHR => {
                    self.sp += 1;
                    if self.sp >= self.stk.len() {
                        self.stk.push(0);
                    }
                }
                crate::opcode::Opcode::SHL => {
                    if self.sp > 0 {
                        self.sp -= 1;
                    }
                }
                crate::opcode::Opcode::ADD => {
                    self.stk[self.sp] = self.stk[self.sp].wrapping_add(1);
                }
                crate::opcode::Opcode::SUB => {
                    self.stk[self.sp] = self.stk[self.sp].wrapping_sub(1);
                }
                crate::opcode::Opcode::PUTCHAR => {
                    print!("{}", self.stk[self.sp] as u8 as char);
                }
                crate::opcode::Opcode::GETCHAR => {
                    let mut buf = vec![0; 1];
                    std::io::stdin()
                        .read_exact(&mut buf)
                        .expect("Failed to read input");
                    self.stk[self.sp] = buf[0] as u32;
                }
                crate::opcode::Opcode::LB => {
                    if self.stk[self.sp] == 0 {
                        self.pc = code.jtable[&self.pc];
                    }
                }
                crate::opcode::Opcode::RB => {
                    if self.stk[self.sp] != 0 {
                        self.pc = code.jtable[&self.pc];
                    }
                }
            }
            self.pc += 1;
        }
    }
}
