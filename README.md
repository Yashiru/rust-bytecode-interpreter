# Bytecodes interpreter

## Getting started

Firstly you need to [install rust](https://www.rust-lang.org/tools/install).

### Clone the project
```bash
# with SSH
git clone git@gitlab.com:leofasano/bytecodes-interpreter.git

# with HTTP
git clone https://gitlab.com/leofasano/bytecodes-interpreter.git
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

Go into _**./src/main.rs**_ and update the byte code inside the **interpretor::text_to_operations** function.

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
You can use this following byte code
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