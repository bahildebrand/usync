# Usync
A Rust future runtime for bare-metal no_std devices.

![Rust Workflow](https://github.com/bahildebrand/usync/workflows/Rust/badge.svg)

## Development
The following section will help set up your development environment. All
examples in this repo were developed with the
[STM32F4DISCOVERY](https://www.st.com/en/evaluation-tools/stm32f4discovery.html)
board, so the instructions are focused on that setup. Note that this library can
currently run on any Cortex-M based MCU, but has not been extensively tested in
those environments.

### Prerequisites
#### Nightly Compiler
Usync uses several experimental features, and thus requires the nightly
compiler. To switch to nightly run the following command:
```
rustup default nightly
```

#### Cortex-M4F Target
Since all examples for this project are targeting the STM32F407, we need to
install the Cortex-M4F. To add the target run the following command:
```
rustup target add thumbv7em-none-eabihf
```

#### Renode(Optional)
[Renode](https://renode.io/) is an open source MCU emulator. All examples in
this library can run in Renode. You only need to install this dependency if you
plan to run an emulated example. Downloads can be found
[here](https://renode.io/#downloads).

#### cargo-make(Optional)
[cargo-make](https://github.com/sagiegurari/cargo-make) is a workflow manager
for cargo. It is used to provide a single command to build and launch examples
in Renode. To install run the following command:
```
cargo install --force cargo-make
```

### Running Examples
All examples can be built normally and executed in target with your choice of
debugger. To quickly run an example in renode run the following command:
```
cargo make --makefile renode.toml run <example_name>
```

### Running Integration Tests
Integration tests are all written in the
[Robot Framework](https://robotframework.org/), and can be run with the
following command
```
cargo make --makefile renode.toml test <test-name>
```