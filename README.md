## hashstate

```rust
let mut state: hashstate::State = hashstate::State::empty();

for _ in 0..4 {
  let e1 = &mut *state.preserve(|| 0);
  let e2 = &mut *state.preserve(|| 42);
  dbg!(*e1, *e2);
  *e1 += 1;
  *e2 += 1;
}
```

```
[examples/main.rs:7] *e1 = 0
[examples/main.rs:7] *e2 = 42
[examples/main.rs:7] *e1 = 1
[examples/main.rs:7] *e2 = 43
[examples/main.rs:7] *e1 = 2
[examples/main.rs:7] *e2 = 44
[examples/main.rs:7] *e1 = 3
[examples/main.rs:7] *e2 = 45
```
