# Arithmetic Logic Unit
## Run

```
cargo run --release
```

## Notes

Read input manually to extract parameters.

Found that each section of input corresponded to:

```rust
impl Instruction {
    fn update_state(&self, state: i64) -> i64 {
        match self {
            Instruction::Keep(_, _) => state,
            Instruction::Reduce(_, _) => state.div(26),
        }
    }
    // ...
}

fn evaluate(state: i64, input: i64, instruction: &Instruction) -> i64 {
    if state.rem(26).add(instruction.val_a()) != input {
        instruction
            .update_state(state)
            .mul(26)
            .add(input)
            .add(instruction.val_b())
    } else {
        instruction.update_state(state)
    }
}
```