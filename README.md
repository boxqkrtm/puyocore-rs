# tuyotuyo-rs
4color 3next 6*13 puyo puyo only ai <br>

# how to build<br>
```
cargo build --release
```
# more acceleration
if your cpu support native bmi2 simd<br>
add `+bmi2` in .cargo/config.toml target-feature<br>
don't use on amd `zen, zen+, zen2` cpu