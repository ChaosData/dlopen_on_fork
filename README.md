# dlopen_on_fork

A small, portable LD_PRELOAD hook for injecting code (e.g. frida-gadget.so)
into `fork(2)` children.

The current version is implemented in Rust and requires Rust nightly
(get it via rustup). An initial PoC in C is also included in this repo
for posterity.

```bash
$ # terminal A
$ cd test/ && make && cd ../
$ cd v1/
$ rustc --version
rustc 1.42.0-nightly (0de96d37f 2019-12-19)
$ cargo build --release                                                                                               
$ LD_PRELOAD="/lib/x86_64-linux-gnu/libdl.so.2 ./target/release/libforkhook.so" DLOPEN_PATH=../test/frida-gadget.so ../test/main
[Frida INFO] Listening on 127.0.0.1 TCP port 27142
[Frida INFO] Listening on 127.0.0.1 TCP port 27143
```

```bash
$ # terminal B
$ frida -H 127.0.0.1:27143 -f re.frida.Gadget -q
$ frida -H 127.0.0.1:27142 -f re.frida.Gadget -q
```

```bash
fork() == 14812, we are the parent (14811)

$ fork() == 0, we are the child
$ # terminal A
```


