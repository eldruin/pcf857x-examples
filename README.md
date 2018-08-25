# Additional example programs for the [pcf857x] crate

[pcf857x]: https://crates.io/crates/pcf857x

This repository contains additional example programs using the PCF8574 I/O expander
with an STM32F3Discovery board.

These examples require that you connect the PCF8574 device to the pins PB6 (SCL)
and PB7 (SDA) of the discovery board.

For example, to run the caterpillar example:
First, connect your discovery board per USB, then connect OpenOCD in a terminal with:
```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

Then on another terminal run:
```
git clone https://github.com/eldruin/pcf857x-f3-examples
cd pcf857x-f3-examples
cargo run --example caterpillar
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

