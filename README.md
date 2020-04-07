# Scheme Compiler

This is an experiment to build a compiler for a Scheme that compiles directly down to x86-64 assembly. This is inspired by the incremental approach taken in [CMSC430](https://www.cs.umd.edu/class/fall2019/cmsc430/) class at the University of Maryland.

This project is just to see how the incremental approach to compiler development is in a typed language, the aim is not to be a production grade compiler for Scheme.

## Usage

You need a working Rust setup on your machine. A C compiler is needed as well (this is to convert assembly to object code only).

### Compile & Run

```bash
$ cd runtime
$ SRC=filename.rkt cargo build # Compile the filename.rkt file into the binary target/debug/runtime
$ ./target/debug/runtime # Run the binary
```

### Interpret

There is an interpreter to check if the compiled code is behaving correctly. To run the interpreter:

```bash
$ cd interpreter
$ cargo build
$ SRC=filename.rkt ./target/debug/interpreter
```

### Testing

Many sample scheme programs are given in `tests` directory. To test them use:

```bash
$ cargo test
```

The above script runs the compiler and interpreter on all the provided sample program and checks if their outputs are consistent.

## Differences from the class

* Use a strongly typed language for developing the compiler. Rust was a natural choice here, because compiling and linking against assembly fits nicely with its build system. Rust is a perfect fit for systems programming, making it a good choice to write a compile code's runtime system. Use of typed AST, typed assembly code and exhaustive pattern matching helps the compiler catch many bugs during development.
* [Planned] Use an IR for possible compiler optimizations instead of directly compiling to assembly.
* [Planned] Build a working garbage collector for memory management. The compiler designed in class never deallocates memory.

## Source code reference

* `compiler`: Transforms the parsed Scheme program into assembly.
* `interpreter`: Runs an interpreter over the provided source program and reports the output.
* `language`: Contains the parser and AST definitions for our language.
* `runtime`: Contains the runtime support for our compiled code.
