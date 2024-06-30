#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>

int main() {
  pid_t p = fork();
  if (p == 0) {
    puts("fork() == 0, we are the child");
  } else {
    printf("fork() == %d, we are the parent (%d)\n", p, getpid());
  }
  return 0;
}
