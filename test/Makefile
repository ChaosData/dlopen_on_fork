CC = gcc
CFLAGS = -Wall -Wextra

all:
	$(CC) $(CFLAGS) -std=c11 -fPIC -pie -o main main.c
	$(CC) $(CFLAGS) -std=c11 -fPIC -pie -static -o main_static main.c

clean:
	rm -rf main
