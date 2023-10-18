# uPico control app

## Building instructions

1. Install rustup by following the instructions at https://rustup.rs
2. Clone this repo: `git clone git@github.com:dotcypress/upico.git && cd upico/app`
3. Build: `cargo build --release --no-default-features --features r01` (replace `r01` with your core module: `cm4`, `a04`, `a06`)
4. `sudo cp target/release/upico /usr/local/bin/`
5. `sudo cp upico.service /etc/systemd/system/`
6. `sudo systemctl enable upico`
7. `sudo systemctl start upico`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
