## Dynamic Markov Compression in Rust
Dynamic Markov Compression (DMC) [1,2] is a lossless data compression
algorithm. DMC uses Markov models to achieve compression. DMC never
gained widespread adoption despite having found application in some
specialised fields like genomics [3]. The original paper describes the
theory in detail [2]. As a Rust-learning exercise I ported the
original C code to Rust.

DMC achieves compression performance somewhere between gzip and bzip2.

## Compiling the code
```bash
git clone https://github.com/oscar-franzen/dynamic-markov-compression
cargo b --release
```
Binaries usually end up in `./target/release/`.

## Executable binary
Lazy to compile? A Linux 64 bit executable binary is placed in the
root of the git repository.

## Example usage
```bash
dmc --compress <input file>
dmc --decompress <input file>
```

The `--nodes` option can be used to specify the number of nodes to be
used in the predictor. Increasing the number of nodes may improve
compression. The default value for `--nodes` is 524269.

The `--threshold` option can be used to change the default (2.0) state
transition threshold.

```bash
dmc --nodes 1000000 --compress <input file>
dmc --nodes 1000000 --decompress <input file>

dmc --threshold 4 --nodes 1000000 --compress <input file>
dmc --threshold 4 --nodes 1000000 --decompress <input file>
```

Note, `--nodes` and `--threshold` must be set to the same value when
running `--decompress` or the output will be invalid (a future task
would be to implement storing this information in a file format
header).

## Benchmark
I will run the benchmark on the Linux kernel 5.11-rc7 source code and
the calgary corpus [4]. Downloading the kernel source code and print
number of bytes:

```bash
wget https://git.kernel.org/torvalds/t/linux-5.11-rc7.tar.gz
wget https://www.dcs.warwick.ac.uk/~nasir/work/fp/datasets/calgary-corpus.tar.gz

gunzip linux-5.11-rc7.tar.gz calgary-corpus.tar.gz
```

Dataset|Program|Metric|Value
------ | ------ | ----- | -----
foo | bar | test | 42


Compared to gzip, DMC is marginally better. However, bzip2 achieves a
much better compression (142828054 bytes compressed).

## Feedback
OF; <p.oscar.franzen@gmail.com>

## TODO
See `TODO.md`.

## References
1. https://en.wikipedia.org/wiki/Dynamic_Markov_compression
2. Data compression using dynamic Markov modelling, Cormack &
   Horspool (1987), Comp J, https://dl.acm.org/doi/10.1093/comjnl/30.6.541
3. https://github.com/rajatdiptabiswas/dna-compression
4. https://en.wikipedia.org/wiki/Calgary_corpus
