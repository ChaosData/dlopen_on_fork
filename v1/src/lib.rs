/*
Copyright (c) 2019 NCC Group.
Copyright (c) 2024 Jeff Dileo.
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

#![no_std]
//#![feature(lang_items, linkage, asm)]
#![feature(lang_items, linkage)]
#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]
#![allow(non_snake_case)]

mod fallback;

use fallback::*;

extern crate libc; // for types only

use core::arch::asm;

//#![cfg_attr(feature = "used_linker", feature(used_with_arg))]
// Prevent a spurious 'unused_imports' warning
//#[allow(unused_imports)]
#[macro_use]
extern crate ctor;


 
/*
extern crate alloc;

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
  loop {}
}
*/

//#[macro_use]
//extern crate syscall;

pub const WRITE : usize = 1;

fn write(fd: usize, buf: &[u8]) -> usize {
  unsafe {
    //syscall!(WRITE, fd, buf.as_ptr(), buf.len())
    let ret : usize;
    /*asm!("syscall" : "={rax}"(ret)
         : "{rax}"(WRITE), "{rdi}"(fd), "{rsi}"(buf.as_ptr()), "{rdx}"(buf.len())
         : "rcx", "r11", "memory"
         : "volatile");
    */
    asm!(
      "syscall",
      inout("rax") WRITE => ret,
      in("rdi") fd,
      in("rsi") buf.as_ptr(),
      in("rdx") buf.len(),
      out("rcx") _,
      out("r11") _
    );
    ret
  }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
  unsafe { write(1, b"panic called\n\0"); }
  unsafe {
    asm!("hlt")
  }
  loop {}
}


/*
use static_alloc::Slab;

#[global_allocator]
static A: Slab<[u8; 1 << 16]> = Slab::uninit();
*/

use cstr_core::CStr;

//use cstr_core::CString;
//use cstr_core::c_char;

type c_char = u8;
type c_void = u8;

#[repr(C)]
pub struct link_map {
  l_addr: usize,
  l_name: *const cstr_core::c_char,
  l_ld: usize,
  l_next: *mut link_map,
  l_prev: *mut link_map
}

#[repr(C)]
pub struct unique_sym {
  hashval: u32,
  name: *const c_char,
  sym: *const usize,
  map: *const link_map
}

#[repr(C)]
pub struct unique_sym_table {
  lock: libc::pthread_mutex_t,
  entries: *const unique_sym,
  size: usize,
  n_elements: usize,
  free: *const c_void
}

#[repr(C)]
pub struct r_debug {
  r_version: i32,
  r_map: *const link_map,
  r_brk: usize,
  r_state: i32,
  r_ldbase: usize
}

#[repr(C)]
pub struct link_namespaces {
  _ns_loaded: *mut link_map,
  _ns_nloaded: u32,
  _ns_main_searchlist: *const c_void,
  _ns_global_scope_alloc: usize,
  _ns_unique_sym_table: unique_sym_table,
  _ns_debug: r_debug
}

const DL_NNS: usize = 16;

#[repr(C)]
pub struct rtld_global {
  _dl_ns: [link_namespaces; DL_NNS],
  _dl_nns: usize,
  //_dl_load_write_lock: libc::pthread_mutex_t,
  //_dl_load_adds: u64,
  //_dl_initfirst: *const link_map
}

const RTLD_LAZY: i32 = 0x1;

extern "C" {
  //#[no_mangle]
  #[linkage="extern_weak"]
  static printf: *const c_void;
  //extern fn(format: *const c_char, ...) -> i32;


  //#[no_mangle]
  #[linkage="extern_weak"]
  static _rtld_global: *const rtld_global;

  //#[no_mangle]
  #[linkage="extern_weak"]
  static dlerror: *const c_void;
  // extern fn() -> *const cstr_core::c_char;

  //#[no_mangle]
  #[linkage="extern_weak"]
  static dlopen: *const c_void;
  // extern fn(filename: *const c_char, flags: i32) -> *const link_map;

  //#[no_mangle]
  #[linkage="extern_weak"]
  static dlsym: *const c_void;
  // extern fn(handle: *const c_void, symbol: *const c_char) -> *const c_void;

  //#[no_mangle]
  #[linkage="extern_weak"]
  static getenv: *const c_void;
  // extern fn(name: *const c_char) -> *const c_char;

  // instead of using the glibc dlinfo with RTLD_DI_LINKMAP,
  //   we just treat dlopen as returning a link_map instead of void*.
  // this is what the opaque handle is anyway and dlinfo just
  //   casts and returns the handle back.
  // even musl reimplements dlopen/dlinfo this way, likely for compatibility
  //   b/c most people prefer not to infect their code w/ `#define _GNU_SOURCE`

  //#[no_mangle]
  //#[linkage="extern_weak"]
  //static dlinfo: *const extern fn();
  // extern fn(handle: *const c_void, request: i32, info: *mut c_void) -> i32;
}

macro_rules! printf {
  ( $format:expr ) => {{
    unsafe {
      if printf as usize != 0 {
        core::intrinsics::transmute::<*const c_void,
          extern fn(format: *const c_char, ...) -> i32
        >(printf)($format as *const c_char)
      } else {
        write(1, $format) as i32
      }
      /*match CString::new($format) {
        Ok(f) => printf(f.as_ptr()),
        Err(_) => 0
      }*/
    }
  }};

  ( $format:expr, $( $arg:expr ),+ ) => {{
    unsafe {
      if printf as usize != 0 {
        core::intrinsics::transmute::<*const c_void,
          extern fn(format: *const c_char, ...) -> i32
        >(printf)($format as *const c_char, $($arg),+ )
      } else {
        write(1, $format) as i32
      }
      /*match CString::new($format) {
        Ok(f) => printf(f.as_ptr(), $($arg),+ ),
        Err(_) => 0
      }*/
    }
  }};
}

unsafe fn _dlerror() -> *const cstr_core::c_char {
  if dlerror as usize != 0 {
    core::intrinsics::transmute::<*const c_void,
      extern fn() -> *const cstr_core::c_char
    >(dlerror)()
  } else {
    printf!(b"_dlerror failed: dlerror missing\n\0");
    0 as *const cstr_core::c_char
  }
}

unsafe fn _dlopen(filename: *const c_char, flags: i32) -> *const link_map {
  if dlopen as usize != 0 {
    core::intrinsics::transmute::<*const c_void,
      extern fn(filename: *const c_char, flags: i32) -> *const link_map
    >(dlopen)(filename, flags)
  } else {
    printf!(b"_dlopen failed: dlopen missing\n\0");
    0 as *const link_map
  }
}

unsafe fn _dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void {
  if dlsym as usize != 0 {
    core::intrinsics::transmute::<*const c_void,
      extern fn(handle: *const c_void, symbol: *const c_char) -> *const c_void
    >(dlsym)(handle, symbol)
  } else {
    printf!(b"_dlsym failed: dlsym missing\n\0");
    0 as *const c_void
  }
}

unsafe fn _getenv(name: *const c_char) -> *const c_char {
  if getenv as usize != 0 {
    core::intrinsics::transmute::<*const c_void,
      extern fn(name: *const c_char) -> *const c_char
    >(getenv)(name)
  } else {
    printf!(b"_getenv failed: getenv missing\n\0");
    0 as *const c_char
  }
}

#[allow(dead_code)]
#[cfg(any(target_arch = "x86_64"))]
pub fn yolo_dlopen(filename: *const c_char, flags: usize) -> *const link_map {
  // it turns out that this wasn't necessary, the caller of the internal dlopen
  // that checks is the dlopen stub itself that we call, so that should pass.
  // it didn't b/c we were referencing libdl.so by its absolute path, which
  // meant the bad strcmp check in libc borked on it,
  // as it wants just "libdl.so(.6)"
  // but this (smuggling in a fake return address) was annoying to do,
  // so i'm leaving it here for posterity

  let ret : usize;
  let start = unsafe { printf } as *const ();
  let mut ret_addr = start as *const u8;
  unsafe {
    loop {
      if *ret_addr == 0xc3 {
        break
      }
      ret_addr = ret_addr.add(1)
    }
  }

  let dlopen_addr = unsafe { dlopen } as *const();

  printf!(b"start: %p\nstart_ret: %p\ndlopen: %p\n\0",
          start, ret_addr, dlopen_addr);

  unsafe {
    asm!(r#"
      //int $$0x3
      call 2f
      jmp 3f
      1:
      movq $1, %rcx
      push %rcx
      movq $2, %rcx
      movq $3, %rdi
      movq $4, %rsi
      jmpq *%rcx
      2:
      jmp 1b
      3:
      nop
      "#,
      out("rax") ret,
      inout("rdi") ret_addr => _,
      inout("rsi") dlopen_addr => _,
      in("rdx") filename,
      inout("rcx") flags => _
//      : "={rax}"(ret) // outputs
//      : "r"(ret_addr), "r"(dlopen_addr), "r"(filename), "r"(flags) // inputs
//      : "rdi", "rsi", "rcx", "memory" // clobbers
//      : "volatile" // options
    );
  }
  //printf!(b"ret: %p\n\0", ret);
  //printf!(b"%s\n\0", _dlerror());
  ret as *const link_map
}

fn is_old_glibc_error(dle: *const cstr_core::c_char) -> bool {
  unsafe {
    if !dle.is_null() {
      let err = CStr::from_ptr(dle);
      if err == CStr::from_bytes_with_nul_unchecked(b"dlopen: invalid caller\0") {
        return true
      }
    }
  }
  false
}

unsafe fn fixup_link_map() {
  if _rtld_global as usize == 0 {
    return
  }

  let rtld_global = &*_rtld_global;
  let nns = rtld_global._dl_nns;

  for i in 0..nns {
    let mut ns_loaded = rtld_global._dl_ns[i]._ns_loaded as *mut link_map;
    while ns_loaded as usize != 0 {
      let l_name = (*ns_loaded).l_name;
      //printf!(b"rtld_global._dl_ns[%u].l_name: %s\n\0", i, l_name);
      match CStr::from_ptr(l_name).to_str() {
        Ok(name) => {
          match name.find("libdl.so") {
            Some(_) => {
              match name.rfind("/") {
                Some(off) => {
                  (*ns_loaded).l_name = l_name.add(off+1);
                },
                None => { }
              }
            },
            None => { }
          }
        },
        Err(_) => {
          printf!(b"failed to decode\n\0");
        }
      }
      ns_loaded = (*ns_loaded).l_next;
    }
  }
}

static mut REAL_FORK: Option<extern fn() -> i32> = None;
static mut DLOPEN_PATH: Option<*const c_char> = None;

unsafe fn _fork() -> i32 {
  match REAL_FORK {
    Some(f) => {
      f()
    },
    None => {
      printf!(b"_fork failed: REAL_FORK not set\n\0");
      0
    }
  }
}

//#[no_mangle]
//pub extern "C" fn myinit() {
#[ctor]
fn myinit() {
  //printf!(b"init()\n\0");
  if unsafe { dlopen as usize } == 0 {
    printf!(b"dlopen not found\n\0");
    return
  } else {
    //printf!(b"dlopen: %p\n\0", dlopen as *const () as *const c_void);
  }

  let mut lm = unsafe { _dlopen(0 as *const c_char, RTLD_LAZY) };
  if lm.is_null() {
    unsafe {
      let dle = _dlerror();
      if is_old_glibc_error(dle) {
        fixup_link_map();
        lm = _dlopen(0 as *const c_char, RTLD_LAZY);
      } else {
        printf!(b"dlerror() -> '%s'\n\0", dle);
      }
    }
    /*
    if !dle.is_null() {
      let err = CStr::from_ptr(dle);
      if err == CStr::from_bytes_with_nul_unchecked(b"dlopen: invalid caller\0") {
        yolo_dlopen(0 as *const c_char, RTLD_LAZY as usize);
      } else {
        printf!(b"%s\n\0", err.as_ptr());
      }
    } else {
      yolo_dlopen(0 as *const c_char, RTLD_LAZY as usize);
    }
    */
  }
  //printf!(b"lm: %p\n\0", lm);
  if lm.is_null() {
    return;
  }

  let mut lmc = lm;
  let mut libc_path: *const c_char = core::ptr::null();
  while lmc as usize != 0 {
    let l_name = unsafe { (*lmc).l_name };
    match unsafe { CStr::from_ptr(l_name).to_str() } {
      Ok(name) => {
        match name.find("libc.so") {
          Some(_) => {
            libc_path = l_name as *const c_char;
          },
          None => { }
        }
      },
      Err(_) => {
        printf!(b"failed to decode\n\0");
      }
    }
    lmc = unsafe { (*lmc).l_next };
  }
    
  //printf!(b"libc_path: %s\n\0", libc_path);
  if libc_path.is_null() {
    if FALLBACK_LIBC_PATH.len() > 1 { // NUL is included
      printf!(b"falling back to static libc path...\n\0");
      libc_path = FALLBACK_LIBC_PATH.as_ptr();
    } else {
      printf!(b"...no fallback path set\n\0");
      panic!();
    }
  }

  let handle = unsafe { _dlopen(libc_path , RTLD_LAZY) };
  if handle as usize == 0 {
    printf!(b"dlopen(libc_path) returned null\n\0");
    panic!();
  }

  let __fork = unsafe { _dlsym(handle as *const c_void, b"fork\0" as *const c_char) };
  if __fork as usize == 0 {
    printf!(b"no fork\n\0");
    panic!();
  }
  unsafe {
    REAL_FORK = Some(
      core::intrinsics::transmute::<*const c_void,
        extern fn() -> i32
      >(__fork)
    );
  }

  let dlopen_path = unsafe { _getenv(b"DLOPEN_PATH\0" as *const c_char) };
  if dlopen_path as usize != 0 {
    unsafe { DLOPEN_PATH = Some(dlopen_path); }
  } else {
    if FALLBACK_DLOPEN_PATH.len() > 1 { // NUL is included
      unsafe { DLOPEN_PATH = Some(FALLBACK_DLOPEN_PATH.as_ptr()); }
    }
  }
}

#[no_mangle]
pub unsafe extern fn fork() -> i32 {
  let ret = _fork();
  match DLOPEN_PATH {
    Some(dlopen_path) => {
      _dlopen(dlopen_path, RTLD_LAZY);
    },
    None => { }
  }
  ret  
}

#[no_mangle]
unsafe extern "C" fn bcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32 {
  // for some reason, rust's &str methods end up emitting calls to libc bcm().
  // this ends up being due to a memcmp() call in the slice implementation
  // underlying things like &str::find() being emitted as a libc bcmp() call

  // $ rustc --version
  // rustc 1.42.0-nightly (0de96d37f 2019-12-19)
  // https://github.com/rust-lang/rust/blob/0de96d37f/src/libcore/slice/mod.rs#L5489

  // gdb$ stack
  // #0  bcmp (s1=0x7ffff79b0a97 "libc.solibc_path: %s\n\000", s2=0x7ffff79b0a99 "bc.solibc_path: %s\n\000", n=0x5) at src/lib.rs:480
  // #1  0x00007ffff7990bf8 in <[A] as core::slice::SlicePartialEq<A>>::equal () at src/libcore/slice/mod.rs:5489
  // #2  core::slice::<impl core::cmp::PartialEq<[B]> for [A]>::eq () at src/libcore/slice/mod.rs:5411
  // #3  core::cmp::impls::<impl core::cmp::PartialEq<&B> for &A>::eq () at src/libcore/cmp.rs:1173
  // #4  core::str::pattern::TwoWaySearcher::new () at src/libcore/str/pattern.rs:1035
  // #5  core::str::pattern::StrSearcher::new () at src/libcore/str/pattern.rs:761
  // #6  0x00007ffff79850d8 in <&str as core::str::pattern::Pattern>::into_searcher (self=..., haystack=...) at /rustc/0de96d37fbcc54978458c18f5067cd9817669bc8/src/libcore/str/pattern.rs:699
  // #7  core::str::<impl str>::find (self=..., pat=...) at /rustc/0de96d37fbcc54978458c18f5067cd9817669bc8/src/libcore/str/mod.rs:2964

  // due to this being in core, there doesn't seem to be a way do so something
  // like #[linkage="private"] w/ llvm linking attributes to expose the symbol
  // only to core and not to the binary actually being run.
  // as a result, we reimplement it here and try to be somewhat "clean."
  // if it doesn't get optimized and holds up the binary, blame the rust devs.

  // it probably isn't strictly necessary, given that you can't LD_PRELOAD
  // statically linked binaries, but if one were to try to run something like
  // this through LD_AUDIT, the strong symbol link to `bcmp` would cause the
  // .so to fail to load even the constructor, long before the check for
  // la_version() being implemented.

  //asm!("int $$0x3");
  //printf!(b"bcmp(\"%s\", \"%s\", %u)\n\0", s1, s2, n);
  let _s1 = s1 as *const c_char;
  let _s2 = s2 as *const c_char;
  for i in 0..(n as isize) {
    let __s1 = *_s1.offset(i);
    let __s2 = *_s2.offset(i);
    if __s1 != __s2 {
      if __s1 > __s2 {
        return 1;
      } else {
        return -1;
      }
    }
  }
  0
}


/*
#[used]
#[link_section = ".ctors"]
#[no_mangle]
pub static CONSTRUCTOR: extern fn() = myinit;
*/

#[lang = "eh_personality"]
extern "C" fn eh_personality() { }

