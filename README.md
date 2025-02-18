# My brainfuck interpreter
This is my brain fuck interpreter written in rust. It took great inspiration from ![Brainfuck-rs](https://github.com/Jomy10/Brainfuck-rs/) but uses a jump table for the loops instead of repeated code. Likely less efficient.

## Usage

```
cargo run <file path>
```

Takes in the input brainfuck code from the file. You can supply a Write trait class to it for output and it will take input from stdin (If i find a way I will use a trait for this).