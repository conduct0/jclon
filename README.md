# JSON Parser

## Goal

Parse JSON text into Rust data structures.

## Interface
```rust
fn parse(input: &str) -> Result<Value, ParseError>
```
## Types Needed

- Value enum (null, bool, number, string, array, object)
- ParseError struct

## Phases

1. Parse primitives (null, bool, number, string)
2. Parse arrays and objects
3. Handle escape sequences
4. Add tests
5. Make a CLI

## Constraint

No external dependencies.

