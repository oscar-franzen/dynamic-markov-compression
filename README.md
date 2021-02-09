## Dynamic Markov Compression in Rust
Dynamic Markov Compression [1,2] is a lossless data compression
algorithm. DMC uses Markov models to achieve compression. DMC never
gained widespread adoption despite having found application in some
specialised fields like genomics [3]. The original paper describes the
theory in detail [2]. As a Rust-learning exercise I ported the
original C code to Rust.

DMC achieves compression performance somewhere between gzip and
bzip2.

## Compiling the code
```bash
git clone https://github.com/oscar-franzen/dynamic-markov-compression
rust b --release
```
Binaries usually end up in `./target/release/`.

## Examplee usage
```bash
dmc --compress <input file>
dmc --decompress <input file>
```

## Benchmark
I will run the benchmark on the Linux kernel source code. Downloading
the kernel and print number of bytes:

```bash
wget https://git.kernel.org/torvalds/t/linux-5.11-rc7.tar.gz
du -b linux-5.11-rc7.tar.gz
# 189255808
```

Decompress the tar archive:
```bash
gunzip linux-5.11-rc7.tar.gz
```

Compress it with DMC:
```bash
dmc -c linux-5.11-rc7.tar
```

The above command will create a file called `linux-5.11-rc7.tar.dmc`,
which is the compressed file. Check it's size:

```bash
du -b linux-5.11-rc7.tar.dmc
# 186908889
```

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
