### How to reproduce?

Run `docker build .`

The error is:

```
3.944   = note: /target/debug/deps/pure_nix_repro-f2775a2bb21016af.3jpem8iwsy31qben.rcgu.o: In function `nix::sys::memfd::memfd_create':
3.944           //usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/nix-0.27.1/src/sys/memfd.rs:57: undefined reference to `memfd_create'
3.944           collect2: error: ld returned 1 exit status

```

The failing code is there: [./pure_nix_repro/src/main.rs](pure_nix_repro/src/main.rs)

