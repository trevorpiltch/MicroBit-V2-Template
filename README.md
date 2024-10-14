# Micro:Bit V2 Cargo Template
This project is a sample Cargo project for the [Micro:Bit V2](https://microbit.org/) microcontroller. It is written in Rust based on the [Rust Discovery Book](https://github.com/rust-embedded/discovery/). 

## Installation
1. **Install cargo:** [link](https://doc.rust-lang.org/cargo/getting-started/installation.html) \
2. **Install cargo-generate:**
```bash
cargo install cargo-generate
```
3. **Install the toolchains:** [link](https://developer.arm.com/downloads/-/gnu-rm)
4. **Install the target:**
```bash
rustup target add thumbv7em-none-eabihf
```

## Usage
Enter this command from the directory where you want to create the project. The system will prompt you for a new project name.
```
cargo generate trevorpiltch/MicroBit-V2-Template
```

### Building
> Don't forget to `cd` into your project directory
```
cargo build --target thumbv7em-none-eabihf
```

### Flashing
> Don't forget to plug your Micro:Bit into your computer
```
cargo embed --target thumbv7em-none-eabihf
```


