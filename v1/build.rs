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

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {
  println!("cargo:rerun-if-env-changed=FALLBACK_LIBC_PATH");
  //println!("cargo:rerun-if-env-changed=FALLBACK_DLOPEN_PATH");
  //println!("cargo:rerun-if-changed=/proc/self/comm");

  let man_dir_s = env::var("CARGO_MANIFEST_DIR").unwrap();
  let src_dir = Path::new(&man_dir_s).join("src");

  let dest_path = Path::new(&src_dir).join("fallback.rs");
  let mut f = File::create(&dest_path).unwrap();

  let mut key = env::var("FALLBACK_LIBC_PATH");
  //let mut key: Option<&'static str> = option_env!("FALLBACK_LIBC_PATH");
  let fallback_libc_path = key.unwrap_or("/lib/x86_64-linux-gnu/libc.so.6".to_owned());
  //println!("cargo:warning=got fallback_libc_path as {}", fallback_libc_path);

  if fallback_libc_path.len() < 1024 {
    writeln!(&mut f, "pub static FALLBACK_LIBC_PATH: &'static str = \"{}\\0\";", fallback_libc_path).unwrap();
  } else {
    writeln!(&mut f, "pub static FALLBACK_LIBC_PATH: &'static str = \"\\0\";").unwrap();
    println!("cargo:warning={}", "FALLBACK_LIBC_PATH too long");
  }
  //key = option_env!("FALLBACK_DLOPEN_PATH");
  key = env::var("FALLBACK_DLOPEN_PATH");
  match key {
    Ok(fallback_dlopen_path) => {
      if fallback_dlopen_path.len() < 1024 {
        writeln!(&mut f, "pub static FALLBACK_DLOPEN_PATH: &'static str = \"{}\\0\";", fallback_dlopen_path).unwrap();
      } else {
        writeln!(&mut f, "pub static FALLBACK_DLOPEN_PATH: &'static str = \"\\0\";").unwrap();
        println!("cargo:warning={}", "FALLBACK_DLOPEN_PATH too long");
      }
    },
    Err(_) => {
      writeln!(&mut f, "pub static FALLBACK_DLOPEN_PATH: &'static str = \"\\0\";").unwrap();
    }
  }
}
