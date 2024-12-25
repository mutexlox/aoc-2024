### Advent of Code 2024

My solutions repo to AoC 2024.

To run a given day (where `inputs/dayXX.txt` is the path to that day's input
file):

```
$ cargo run --bin dayXX -- inputs/dayXX.txt
```

or first build and then run:

```
$ cargo build
$ targets/debug/dayXX inputs/dayXX.txt
```

Mostly these will work for any input file, except for day 17, which involved a
manual decompilation of part of the input, which is hard-coded into the
solution.
