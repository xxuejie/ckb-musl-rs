# ckb-musl-rs

This is a Rust binding of patched-[musl](https://github.com/xxuejie/musl) for CKB.

See <sample> for an example using this crate.

To build everything:

```bash
$ git clone --recursive https://github.com/xxuejie/ckb-musl-rs
$ cd ckb-musl-rs
$ ./build.sh

$ cd sample
$ make build
$ ckb-debugger --bin build/release/sample 1024
This is a sample contract!
Received in C: 1024

p1: 0x9f020
p2: 0x9f520
p3: 0x9f920
p4: 0x9f020
p5: 0xa0000
Run result: 0
Total cycles consumed: 46764(45.7K)
Transfer cycles: 10196(10.0K), running cycles: 36568(35.7K)
```
