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
the calgary corpus [4]. `--nodes` was set to `10000000`.

Downloading the kernel source code and print number of bytes:

```bash
wget https://git.kernel.org/torvalds/t/linux-5.11-rc7.tar.gz
wget https://www.dcs.warwick.ac.uk/~nasir/work/fp/datasets/calgary-corpus.tar.gz

gunzip linux-5.11-rc7.tar.gz calgary-corpus.tar.gz
```

### Data: linux-5.11-rc7.tar

Metric/Program     | dmc         | gzip        | bzip2       | xz
------------------ | ------------| ----------- | ----------- | -----------
bytes written      | 159,493,859 | 189,255,827 | 142,828,054 | 121,553,932
compression ratio  | 0.14        | 0.17        | 0.13        | 0.11
wall time (s)      | 290.1       | 24.8        | 86.2        | 403.6

### Data: calgary-corpus.tar

Metric/Program    | dmc     | gzip      | bzip2   | xz
----------------- | ------- | --------- | ------- | -------
bytes written     | 939,847 | 1,071,793 | 893,471 | 854,768
compression ratio | 0.28    | 0.32      | 0.27    | 0.26
wall time (s)     | 1.3     | 0.1       | 0.2     | 1.2

## Conclusions
`dmc` performs better than `gzip` but is slower. Both `bzip2` and `xz`
performs better than `dmc`. `xz` is the best performer, but also the slowest.

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
