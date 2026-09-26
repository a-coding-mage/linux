// SPDX-License-Identifier: GPL-2.0
/*
 * "Optimize" a list of dependencies as spit out by gcc -MD
 * for the build framework.
 *
 * Original author:
 *   Copyright    2002 by Kai Germaschewski  <kai.germaschewski@gmx.de>
 *
 * This code has been borrowed from kbuild's fixdep (scripts/basic/fixdep.c),
 * Please check it for detailed explanation. This fixdep borow only the
 * base transformation of dependecies without the CONFIG mangle.
 */

//! Dependency rewriting for the tools build bootstrap (not scripts/basic).

use std::ffi::{c_char, c_int, c_void, CString};
use std::fs::File;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::ffi::OsStrExt;
use std::process;

// The host headers supply the exact stat layout and redirected libc symbols,
// including the build's _FILE_OFFSET_BITS and _TIME_BITS settings.
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unreachable_pub)]
mod libc {
    include!(env!("TOOLS_FIXDEP_LIBC_BINDINGS"));
}

// C stdio preserves fixdep's buffering, ignored output errors and SIGPIPE.
unsafe extern "C" {
    static mut stdout: *mut c_void;
    fn fwrite(ptr: *const c_void, size: usize, count: usize, stream: *mut c_void) -> usize;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(message: *const c_char);
    fn strerror(error: c_int) -> *const c_char;
    fn dprintf(fd: c_int, format: *const c_char, ...) -> c_int;
}

struct Mapping {
    address: *mut c_void,
    len: usize,
}

impl Mapping {
    fn byte(&self, index: usize) -> u8 {
        assert!(index < self.len);
        // SAFETY: the caller constructed this object from a successful mmap;
        // index is within that mapping. Do not form a Rust slice: an ILP32
        // mapping can exceed isize::MAX even though C can still traverse it.
        unsafe { *self.address.cast::<u8>().wrapping_add(index) }
    }

    fn output_token(&self, start: usize, end: usize) {
        let mut nul = start;
        while nul < end && self.byte(nul) != 0 {
            nul += 1;
        }
        // SAFETY: this range is within the mapping, including an empty token
        // at its end. C printf("%s") stops at the first embedded NUL.
        unsafe { fwrite(self.address.cast::<u8>().wrapping_add(start).cast(),
                        1, nul - start, stdout); }
    }
}

impl Drop for Mapping {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.address, self.len); }
    }
}

fn output(bytes: &[u8]) {
    // SAFETY: bytes is readable for its length; stdout is libc's FILE pointer.
    unsafe { fwrite(bytes.as_ptr().cast(), 1, bytes.len(), stdout); }
}

fn diagnostic(prefix: &[u8], path: &CString, error: std::io::Error) -> ! {
    // Rust filesystem errors retain errno, even if formatting would change it.
    let prefix = CString::new(prefix).unwrap();
    unsafe {
        dprintf(2, c"%s%s%s%s\n".as_ptr(), prefix.as_ptr(), path.as_ptr(),
                if path.as_bytes().is_empty() { c"".as_ptr() } else { c": ".as_ptr() },
                strerror(error.raw_os_error().unwrap_or(5)));
    }
    process::exit(2);
}

/*
 * Important: The generated source_foo.o and deps_foo.o assignments are also
 * parsed by scripts/mod/sumversion.c. Match the tools C parser byte for byte.
 */
fn parse_dep_file(map: &Mapping, target: &[u8]) {
    let mut m = 0;
    let mut has_target = false;
    let mut saw_any_target = false;
    let mut is_first_dep = false;
    while m < map.len {
        while m < map.len && matches!(map.byte(m), b' ' | b'\\' | b'\n') {
            m += 1;
        }
        let mut p = m;
        while p < map.len && !matches!(map.byte(p), b' ' | b'\\' | b'\n') {
            p += 1;
        }
        // C examines p[-1] even after trailing whitespace, emitting an empty
        // dependency in that case. An empty mapping never reaches this parser.
        if p > 0 && map.byte(p - 1) == b':' {
            is_first_dep = true;
            has_target = true;
        } else if has_target {
            if is_first_dep {
                if !saw_any_target {
                    saw_any_target = true;
                    output(b"source_"); output(target); output(b" := ");
                    map.output_token(m, p); output(b"\n\ndeps_"); output(target);
                    output(b" := \\\n");
                }
                is_first_dep = false;
            } else {
                output(b"  "); map.output_token(m, p); output(b" \\\n");
            }
        }
        if p == map.len {
            break;
        }
        m = p + 1;
    }
    if !saw_any_target {
        unsafe { dprintf(2, c"fixdep: parse error; no targets found\n".as_ptr()); }
        process::exit(1);
    }
    output(b"\n"); output(target); output(b": $(deps_"); output(target);
    output(b")\n\n$(deps_"); output(target); output(b"):\n");
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 4 {
        unsafe { dprintf(2, c"Usage: fixdep <depfile> <target> <cmdline>\n".as_ptr()); }
        process::exit(1);
    }
    let path = CString::new(args[1].as_bytes()).unwrap();
    let target = CString::new(args[2].as_bytes()).unwrap();
    let command = CString::new(args[3].as_bytes()).unwrap();
    unsafe { printf(c"cmd_%s := %s\n\n".as_ptr(), target.as_ptr(), command.as_ptr()); }
    let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY as c_int) };
    if fd < 0 {
        diagnostic(b"fixdep: error opening depfile: ", &path, std::io::Error::last_os_error());
    }
    // SAFETY: successful open returned a new descriptor owned by this process.
    let file = unsafe { File::from_raw_fd(fd) };
    let mut metadata = std::mem::MaybeUninit::<libc::stat>::zeroed();
    if unsafe { libc::fstat(file.as_raw_fd(), metadata.as_mut_ptr()) } < 0 {
        diagnostic(b"fixdep: error fstat'ing depfile: ", &path, std::io::Error::last_os_error());
    }
    // SAFETY: fstat initialized the generated host ABI structure on success.
    let size = unsafe { metadata.assume_init() }.st_size;
    let len = size as usize;
    if size == 0 {
        unsafe { dprintf(2, c"fixdep: %s is empty\n".as_ptr(), path.as_ptr()); }
        return;
    }
    // Keep the C size_t conversion, mmap failure's successful exit, and mmap's
    // fault/special-file behavior rather than substituting read().
    let map = unsafe { libc::mmap(std::ptr::null_mut(), len, libc::PROT_READ as c_int,
                                 libc::MAP_PRIVATE as c_int, file.as_raw_fd(), 0) };
    if map == usize::MAX as *mut c_void {
        unsafe { perror(c"fixdep: mmap".as_ptr()); }
        return;
    }
    // A successful mmap owns this address range. As with the C original,
    // concurrent truncation may deliver SIGBUS.
    parse_dep_file(&Mapping { address: map, len }, target.as_bytes());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
