# Tuyotuyo-rs
4color 3next 6*13 puyo puyo only ai <br>

# How to build<br>
```
cargo build --release
```
if your cpu support hardware bmi2. you can add +bmi2 in `tuyotuyo-rs/.cargo/config.toml` `target-feature`<br>
it will be increase 3.9x speed up

# WIP Todo
- [ ] fast puyo pop calculation - TODO fix emu 15 bug
- [ ] chain detector
- [ ] key input generator
- [ ] puyo puyo esports based timing duel simulator
- [ ] add ai
