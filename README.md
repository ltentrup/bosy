# BoSyHyper CAV Artifact

All tools needed to compile BoSyHyper are installed in the Docker image:

* Rust compiler
* Debian packages `build-essential`, `cmake`, `z3`, `python`, and `git`

## Sources

BoSyHyper is written in Rust and is split into different packages.
The main logic, that is, the encoding is `bosy/src/encoding.rs`.

## Building

* `cargo build --release` build a static binary `./target/release/bosy` containing the translator from HyperLTL synthesis specifications to SMT queries
* `make` builds the external tools that are called by BoSyHyper

## Reproducing Results

We provided a convenient shell script that reproduces Table 1 in the paper.
In most cases, BoSyHyper is called with the LTL and HyperLTL specifications and the bounds as indicated in the paper and the resulting SMT query is written to disk. Then, z3 is called to solve those instances and the solving times are reported.

## Remarks

We found that the z3 version we used in our experiments has a large variance in the solving times between multiple runs of the same instance, thus, we only report rough estimates for the solving times.
The benchmarks were executed on a machine with a 3.3 Ghz dual-core Core i7 and 16 GB of memory.
