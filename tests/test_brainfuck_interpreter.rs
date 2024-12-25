use brainfuck_rs::brainfuck_interpreter::BFInterpreter;

#[test]
fn test_hello_world() {
    let code = "++++++++++[>+++++++>++++++++++>+++>+<<<<
-]>++.>+.+++++++..+++.>++.<<++++++++++++
+++.>.+++.------.--------.>+.>.";
    let mut interpreter = BFInterpreter::default();
    interpreter.run(code);
}
