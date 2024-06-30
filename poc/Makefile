CC = gcc
#MUSLGCC ?= musl-gcc
CFLAGS = -Wall -Wextra

all:
	$(CC) $(CFLAGS) -std=c11 -fPIC -shared -o libforkhook.so forkhook.c -ldl

#static:
#	$(MUSLGCC) -static $(CFLAGS) -std=c11 -fPIC -shared -Wl,-Bstatic -o libforkhook.so forkhook.c -ldl

#static:
#	$(CC) -static $(CFLAGS) -std=c11 -fPIC -shared -lc /usr/lib/x86_64-linux-gnu/libdl.a -o libforkhook.so forkhook.c

clean:
	rm -rf libforkhook.so main
