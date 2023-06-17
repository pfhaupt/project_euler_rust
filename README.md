# Project Euler Solutions
This repository contains my solutions to various [Project Euler](projecteuler.net) problems, implemented in Rust. It serves as a personal learning project to explore and enhance my Rust programming skills.
## How to use this Repository
1. **Choose a Problem:** Browse the list of solved problems and select one that interests you.
2. **Review the solution:** Check out the Rust source file within the problem's directory for the implementation details. Each file has a link to the problem.
3. **Get inspired:** Some solutions are creative, some are smart, and some do what computers can do best - Crunch the numbers, aka bruteforce. All solutions fit into the "one-minute-rule" of Project Euler, where each problem can be solved in less than a minute of computation, given the correct algorithm.
4. **Compile and Run:** Run `cargo run [--release]` in the problem's directory to build and run the source file.
## Why Rust?
The primary motivation behind this repository is to learn and become proficient in the Rust programming language. By solving Project Euler problems in Rust, I can practice the language's features, explore its standard library, and deepen my understanding of Rust's ownership model and functional programming concepts.
## Contributing
Contributions are not currently accepted for this repository, as it is primarily a personal learning project. However, feel free to use the code as a reference or inspiration for your own solutions.
## Benchmarks
Currently the 10 slowest programs are:
| Time | Project |
| --- | --- |
|     >1s|./p041-pandigital-prime/Cargo.toml|
|     >1s|./p047-distinct-primes-factors/Cargo.toml|
|     >1s|./p051-prime-digit-replacements/Cargo.toml|
|     >1s|./p060-prime-pair-sets/Cargo.toml|
|     >1s|./p074-digit-factorial-chains/Cargo.toml|
|     >1s|./p076-counting-summations/Cargo.toml|
|     >1s|./p086-cuboid-route/Cargo.toml|
|     >1s|./p088-product-sum-numbers/Cargo.toml|
|     >1s|./p093-arithmetic-expressions/Cargo.toml|
|724.05ms|./p068-magic-5-gon-ring/Cargo.toml|

A `>` indicates that the program did not finish within my set timeout of 1s.
