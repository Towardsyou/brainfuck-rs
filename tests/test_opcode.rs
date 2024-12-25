use brainfuck_rs::opcode::{Code, Opcode};

#[test]
fn test_hello_world() {
    let code = "><+-.,[]";
    let c = Code::from(code);
    assert_eq!(
        c.ops,
        vec![
            Opcode::SHR,
            Opcode::SHL,
            Opcode::ADD,
            Opcode::SUB,
            Opcode::PUTCHAR,
            Opcode::GETCHAR,
            Opcode::LB,
            Opcode::RB,
        ]
    );
    let mut hm = std::collections::HashMap::<usize, usize>::new();
    let l = code
        .replace("\n", "")
        .replace(" ", "")
        .find('[')
        .expect("[ should be in the code");
    let r = code
        .replace("\n", "")
        .replace(" ", "")
        .find(']')
        .expect("] should be in the code");
    hm.insert(l, r);
    hm.insert(r, l);
    assert_eq!(c.jtable, hm);
}

#[test]
fn test_skip_spacing() {
    let code = "><+
    -.,    []";
    let c = Code::from(code);
    assert_eq!(
        c.ops,
        vec![
            Opcode::SHR,
            Opcode::SHL,
            Opcode::ADD,
            Opcode::SUB,
            Opcode::PUTCHAR,
            Opcode::GETCHAR,
            Opcode::LB,
            Opcode::RB,
        ]
    );
    let mut hm = std::collections::HashMap::<usize, usize>::new();
    let l = code
        .replace("\n", "")
        .replace(" ", "")
        .find('[')
        .expect("[ should be in the code");
    let r = code
        .replace("\n", "")
        .replace(" ", "")
        .find(']')
        .expect("] should be in the code");
    hm.insert(l, r);
    hm.insert(r, l);
    assert_eq!(c.jtable, hm);
}
