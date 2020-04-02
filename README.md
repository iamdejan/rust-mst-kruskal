# Rust MST Kruskal

[![Build Status](https://travis-ci.org/iamdejan/rust-mst-kruskal.svg?branch=master)](https://travis-ci.org/iamdejan/rust-mst-kruskal)

This is a simple program to find cost of minimum spanning tree (MST) using Kruskal algorithm. This repository contains code from [my LinkedIn article]().

## Getting Started

### Prerequisites
Install Rust and Cargo using [this guide](https://www.rust-lang.org/learn/get-started).

## Running the program
To run the program and use the sample maze, just run:
1) `$ cargo run < input1.txt`; or
2) `$ cargo run < input2.txt`.

If you want to use your own input, just run `$ cargo run`.

### STDIN Input Format
The input format is as follows:
```
[number of vertices]
[number of edges]
[for each edge]:
    [source (start from 0)] [destination (start from 0)] [cost]
```

See `input1.txt` and `input2.txt` for examples.

## Authors
- **Giovanni Dejan** - [iamdejan](https://github.com/iamdejan)
