/*
Copyright (c) 2019 NCC Group.
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

// $ gcc -std=c11 -Wall -Wextra -fPIC -shared -o libforkhook.so forkhook.c -ldl

#define _GNU_SOURCE
#include <stdlib.h>
#include <stdio.h>
#include <dlfcn.h>
#include <link.h>
#include <stdint.h>
#include <string.h>
#include <sys/types.h>

static void* handle = NULL;
static pid_t(*_fork)(void) = NULL;
static char* libc_path = NULL;
static char* dlopen_path = NULL;

#ifndef FALLBACK_LIBC_PATH
#define FALLBACK_LIBC_PATH "/lib/x86_64-linux-gnu/libc.so.6"
#endif

#ifndef FALLBACK_DLOPEN_PATH
#define FALLBACK_DLOPEN_PATH NULL
#endif

void __attribute__ ((constructor)) setup(void) {
  struct link_map* lm = NULL;
  void* m = dlopen(NULL, RTLD_LAZY);
  int a = dlinfo(m, RTLD_DI_LINKMAP, (void*)&lm);
  if (a != 0) {
    puts(dlerror());
  } else {
    struct link_map* lmc = lm;
    while (1) {
      //printf("l_name: %s\n", lmc->l_name);
      char const* r = strstr(lmc->l_name, "/libc.so");
      if (r != NULL && r != lmc->l_name) {
        libc_path = lmc->l_name;
        break;
      }

      if (lmc->l_next != NULL) {
        lmc = lmc->l_next;
      } else {
        break;
      }
    }
  }
  if (libc_path == NULL) {
    puts("falling back to static libc path...");
    libc_path = FALLBACK_LIBC_PATH;
  }
  if (libc_path == NULL) {
    puts("...no fallback path set");
    return;
  }

  //printf("libc: %s\n", libc_path);

  dlopen_path = getenv("DLOPEN_PATH");
  if (dlopen_path == NULL) {
    dlopen_path = FALLBACK_DLOPEN_PATH;
  }

  //printf("libc_path: %s\n", libc_path);
  handle = dlopen(libc_path, RTLD_LAZY);
  if (handle == NULL) {
    puts(dlerror());
    exit(1);
  }
  _fork = (pid_t(*)(void))dlsym(handle, "fork");
}

pid_t fork(void) {
  pid_t ret = _fork();
  if (dlopen_path != NULL) {
    (void)dlopen(dlopen_path, RTLD_LAZY);
  }
  return ret;
}

