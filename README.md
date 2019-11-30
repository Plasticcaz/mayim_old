# mayim - For fun compiler written in Rust

## Structure
`mayim` is structured as a cargo workspace containing a series of crates that have cross dependencies:

* mayimc
  - A binary that does the driving of compiling a `.mayim` source file.
* tokenizer
  - A library that converts a source file into a list of tokens.
