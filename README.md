# mayim - For fun compiler written in Rust

## Structure
`mayim` is structured as a cargo workspace containing a series of crates that have cross dependencies.

These crates can be viewed as being part of a pipeline, but the idea is that eventually they might be 
used in a language server, or in a text editor plugin, or the like.

* mayimc
  - A binary that does the driving of compiling a `.mayim` source file.
* syntax
  - A library that handles converting a source file into a list of top-level expressions.
