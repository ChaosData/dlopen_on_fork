# dlopen_on_fork

A small, portable LD_PRELOAD hook for injecting code (e.g. frida-gadget.so)
into `fork(2)` children.

The current version is implemented in Rust and requires Rust nightly
(get it via rustup). The initial PoC in C is also included in this repo
for posterity.

```bash
## terminal A
$ curl -L -O https://github.com/frida/frida/releases/download/16.3.3/frida-gadget-16.3.3-linux-x86_64.so.xz      
$ cat frida-gadget-16.3.3-linux-x86_64.so.xz | unxz > test/frida-gadget.so
$ cd test/ && make && cd ../
$ cd v1/
$ rustc +nightly --version
rustc 1.81.0-nightly (ba1d7f4a0 2024-06-29)
$ cargo +nightly build --release
$ LD_PRELOAD=./target/release/libforkhook.so DLOPEN_PATH=../test/frida-gadget.so ../test/main
[Frida INFO] Listening on 127.0.0.1 TCP port 27142
[Frida INFO] Listening on 127.0.0.1 TCP port 27143
```

```bash
## terminal B
$ frida -H 127.0.0.1:27143 -f re.frida.Gadget -q
$ frida -H 127.0.0.1:27142 -f re.frida.Gadget -q
```

```bash
## terminal A
fork() == 0, we are the child
fork() == 63885, we are the parent (63884)
$ 
```

Alternatively:

```bash
## terminal B
$ frida -H 127.0.0.1:27142 -f re.frida.Gadget -q
$ frida -H 127.0.0.1:27143 -f re.frida.Gadget -q
```

```bash
## terminal A
fork() == 64249, we are the parent (64248)
$ fork() == 0, we are the child
```
