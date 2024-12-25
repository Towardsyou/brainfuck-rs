#[derive(Debug, PartialEq)]
pub enum Opcode {
    // use python -c "print(hex(ord('>')))" to get hex values of opcodes
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            v => panic!("Unknown opcode: {:x}", v),
        }
    }
}
pub struct Code {
    pub ops: Vec<Opcode>,
    pub jtable: std::collections::HashMap<usize, usize>,
}

impl From<&str> for Code {
    fn from(s: &str) -> Self {
        let value = s.as_bytes();
        let mut ops = Vec::new();
        let mut jtable = std::collections::HashMap::new();
        let mut jstk = Vec::new();
        for op in value {
            if *op == 0xa || *op == 0x20 {
                continue; // skip empty characters
            }
            let opcode = Opcode::from(*op);
            if opcode == Opcode::LB {
                jstk.push(ops.len());
            } else if opcode == Opcode::RB {
                let left = jstk.pop().expect("Invalid syntax, no matching '[' found");
                jtable.insert(left, ops.len());
                jtable.insert(ops.len(), left);
            }
            ops.push(opcode);
        }
        Code { ops, jtable }
    }
}
