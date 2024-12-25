# My Brainfuck interpreter

This is the repo for code during study Rust, interpreter, and JIT.

## Brainfuck

Brainfuck is simple language to implement for a study project (much simpler than Pascal).

It contains only 8 operators

|op|description|
|--|-----------|
|> |Increment the data pointer by one (to point to the next cell to the right).|
|< |Decrement the data pointer by one (to point to the next cell to the left).|
|+ |Increment the byte at the data pointer by one.|
|- |Decrement the byte at the data pointer by one.|
|. |Output the byte at the data pointer.|
|, |Accept one byte of input, storing its value in the byte at the data pointer.|
|[ |If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.|
|] |If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.|

## IR

IR (Intermediate Representation) is a representation of the code that is more abstract than machine code but less abstract than source code.

Naturally, you can tokenize the code and build a Abstract Syntax Tree (AST) with language-specific operators and types. Or you can use a generalized way for operators and types, called IR. LLVM is a famous project for a IR design.

## JIT

We know interpreter will execute the operators one by one, in the interpreter, while JIT will compile the operators into machine code and execute them directly.

During compilation, we can do some optimizations on the code, such as: 1. optimizing logic 2. remove necessary checks

## Design choices

1. How to deal with unknown input?
    1. Raise an error with line number and column number
    2. or even better give back possible guess for typos
    3. in this project, we ignore the unknown code.
