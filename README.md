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
