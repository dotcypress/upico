# uPico control app

## Building instructions

1. Install rustup by following the instructions at https://rustup.rs
2. Clone this repo: `git clone git@github.com:dotcypress/upico.git && cd upico/app`
3. Build: `cargo build --release`

## Installation

1. `apt-get install udisks2`
2. `sudo cp target/release/upico /usr/local/bin/`
3. `sudo cp ../drivers/r-01/upico.service /etc/systemd/system/`
4. `sudo systemctl enable upico`
5. `sudo systemctl start upico`

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
