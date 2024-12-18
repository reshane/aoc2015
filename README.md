# Advent of Code 2015
 - https://adventofcode.com/2015

## Quick Start
Tests can be run from the command line with `cargo t` or `cargo test`
Provide the `--features long_tests` flag to enable long running tests
Running a given day can be done by creating an input file as follows:
```
curl -b Cookie="[COOKIE]" "https://adventofcode.com/2015/day/{DAY_NUMBER}/input > ./input/{DAY_NUMBER}.txt
```
and then running the preferred day with cargo:
```
cargo r [DAY_NUMBER]
```
