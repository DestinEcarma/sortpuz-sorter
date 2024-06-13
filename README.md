# SortPuz Sorter

SortPuz-Sorter is a Rust project inspired by the popular puzzle game SortPuz. This project implements an algorithm to solve SortPuz using Depth First Search (DFS). While the DFS algorithm may not always provide the most optimal or shortest solution, it effectively solves the puzzle.

## Usage

First, build the executable file or run it directly in release mode for optimal performance:

```sh
cargo build --release
```

or

```sh
cargo run --release
```

There is also an option to use the ball method via features:

```sh
cargo run --release --features ball
```

Finally, you can display all available options and commands by running the program with the `-h` or `--help` argument:

```sh
cargo run --release -- -h
```

## License
This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
