<img align="right" width="150" height="150" top="100" src="https://www.rust-lang.org/static/images/rust-logo-blk.svg">

# Bytecodes interpreter
A very basic and experimental bytecode interpreter.

## Getting started

Firstly you need to [install rust](https://www.rust-lang.org/tools/install).

### Clone the project
```bash
# with SSH
git clone git@github.com:Yashiru/rust-bytecode-interpreter.git

# with HTTP
git clone https://github.com/Yashiru/rust-bytecode-interpreter.git
```

### Build the project

```bash
cargo build
```

### Run the project

```bash
cargo run
```

## Update the input bytecode

Go into `./src/main.rs` and update the byte code inside the `interpretor::text_to_operations` function.

```rust
// main.rs file
mod interpretor;

fn main() {
    let mut interpretor = interpretor::Interpretor::new(
        interpretor::text_to_operations("
----------->[BYTE CODE HERE]
        ")
    );

    interpretor.interpret();
}
```

### Exemple byte code
To run this exemple program
```typescript
let x = 1;
let y = 2;
return (x+1)*y

let x = 0;
let i = 0;
while(i != 5){
    x = x+10;
    i = i+1;
}
return x;
```
You must use this following byte code
```
LOAD_VAL 1
WRITE_VAR 'x'
LOAD_VAL 2
WRITE_VAR 'y'
READ_VAR 'x'
LOAD_VAL 1
ADD
READ_VAR 'y'
MULTIPLY
RETURN_VALUE
LOAD_VAL 0
WRITE_VAR 'x'
LOAD_VAL 0
WRITE_VAR 'i'
READ_VAR 'i'
TEST_DIFFERENT_FROM 5
JUMP_IF_FALSE 26
READ_VAR 'x'
LOAD_VAL 10
ADD
WRITE_VAR 'x'
READ_VAR 'i'
LOAD_VAL 1
ADD
WRITE_VAR 'i'
JUMP_LOOP 14
READ_VAR 'x'
RETURN_VALUE
```

## Supported byte code opnames
* **LOAD_VAL** Load push a value on the stack
* **WRITE_VAR** Write the value on top of the stack to a variable
* **READ_VAR** Load the value of the variable on top of the stack 
* **ADD** Adds the last two values, pop them from the stack and push the result on the stack
* **MULTIPLY** Multiply the last two values, pop them from the stack and push the result on the stack
* **RETURN_VALUE** Return the value on the stack (In this case, the interpreter print the result)
* **TEST_LESS_THAN** Test if a value is less than the last value on the stack
* **TEST_MORE_THAN** Test if a value is more than the last value on the stack
* **TEST_EQUALS_TO** Test if a value is equals to the last value on the stack
* **TEST_DIFFERENT_FROM** Test if a value is different the last value on the stack
* **JUMP_IF_FALSE** Check if the result of a test is False, and jump if it is
* **JUMP_LOOP** Jump to the start of a loop
