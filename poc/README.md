# dlopen_on_fork

A small LD_PRELOAD hook for injecting code (e.g. frida-gadget.so) into `fork(2)` children.

```bash
$ # terminal A
$ LD_PRELOAD=./libforkhook.so DLOPEN_PATH=./frida-gadget.so ./main
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
