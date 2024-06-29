# Password Generator

This project implements a password generator with an interactive interface that allows users to generate and optionally copy passwords to their clipboard.

## Features

- Generate random passwords of customizable length.
- Choose whether to include numbers, uppercase letters, and special characters in the generated passwords.
- Copy generated passwords to clipboard for easy use.

## Modules

- utils: provides functions to capture values from user and parse it entries on string, numbers, boolean, etc...
- pwd: handles user inputs via the io submodule and generate a password mixing a list of characters using generate function from the gen submodule

## Installation

Make sure you have Rust installed. Clone the repository and build the project:

```bash
git clone https://github.com/aquelaronte/password-generator.git
cd password-generator
cargo build --release
```

## Usage
Run the application to interactively generate passwords:
```bash
./target/release/password_generator
```
Follow the prompts to specify password length and inclusion of numbers, uppercase letters, and special characters.

Application can also run on a faster experience by adding flags, see flags by running:
```bash
./password_generator -h
```