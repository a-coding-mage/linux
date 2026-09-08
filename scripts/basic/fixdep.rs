/*
 * "Optimize" a list of dependencies as spit out by gcc -MD
 * for the kernel build
 *
 * This is a direct Rust translation of fixdep.c.
 */

use std::ffi::{CStr, CString};
use std::io::{self, Read, Write};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

extern "C" {
    fn xmalloc(size: usize) -> *mut c_void;
}

fn usage() -> ! {
    eprintln!("Usage: fixdep <depfile> <target> <cmdline>");
    std::process::exit(1)
}

#[repr(C)]
struct Item {
    next: *mut Item,
    len: c_uint,
    hash: c_uint,
    name: [c_char; 0],
}

const HASHSZ: usize = 256;
static mut CONFIG_HASHTAB: [*mut Item; HASHSZ] = [ptr::null_mut(); HASHSZ];
static mut FILE_HASHTAB: [*mut Item; HASHSZ] = [ptr::null_mut(); HASHSZ];

unsafe fn strhash(str_: *const c_char, sz: usize) -> c_uint {
    let mut hash: c_uint = 2166136261u32;
    for i in 0..sz {
        hash = (hash ^ (*str_.add(i) as u8 as c_uint)).wrapping_mul(0x01000193);
    }
    hash
}

unsafe fn add_to_hashtable(name: *const c_char, len: usize, hash: c_uint, hashtab: *mut *mut Item) {
    let aux = xmalloc(std::mem::size_of::<Item>() + len) as *mut Item;
    ptr::copy_nonoverlapping(name as *const u8, (*aux).name.as_mut_ptr() as *mut u8, len);
    (*aux).len = len as c_uint;
    (*aux).hash = hash;
    let slot = (hash as usize) % HASHSZ;
    (*aux).next = *hashtab.add(slot);
    *hashtab.add(slot) = aux;
}

unsafe fn in_hashtable(name: *const c_char, len: usize, hashtab: *mut *mut Item) -> bool {
    let hash = strhash(name, len);
    let mut aux = *hashtab.add((hash as usize) % HASHSZ);
    while !aux.is_null() {
        if (*aux).hash == hash && (*aux).len as usize == len &&
            libc::memcmp((*aux).name.as_ptr() as *const c_void, name as *const c_void, len) == 0 {
            return true;
        }
        aux = (*aux).next;
    }
    add_to_hashtable(name, len, hash, hashtab);
    false
}

unsafe fn use_config(m: *const c_char, slen: usize) {
    if in_hashtable(m, slen, CONFIG_HASHTAB.as_mut_ptr()) { return; }
    let name = CStr::from_ptr(m).to_string_lossy();
    println!("    $(wildcard include/config/{}{}) \\", &name[..slen], "");
}

unsafe fn str_ends_with(s: *const c_char, slen: usize, sub: &[u8]) -> bool {
    slen >= sub.len() && libc::memcmp(s.add(slen - sub.len()) as *const c_void, sub.as_ptr() as *const c_void, sub.len()) == 0
}

unsafe fn parse_config_file(mut p: *const c_char) {
    let start = p;
    while !(libc::strstr(p, b"CONFIG_\0".as_ptr() as *const c_char)).is_null() {
        p = libc::strstr(p, b"CONFIG_\0".as_ptr() as *const c_char);
        if p > start && (libc::isalnum(*p.sub(1) as c_int) != 0 || *p.sub(1) == b'_' as c_char) { p = p.add(7); continue; }
        p = p.add(7);
        let q = { let mut q = p; while libc::isalnum(*q as c_int) != 0 || *q == b'_' as c_char { q = q.add(1); } q };
        let r = if str_ends_with(p, q.offset_from(p) as usize, b"_MODULE") { q.sub(7) } else { q };
        if r > p { use_config(p, r.offset_from(p) as usize); }
        p = q;
    }
}

unsafe fn read_file(filename: *const c_char) -> *mut c_char {
    let path = CStr::from_ptr(filename).to_string_lossy();
    let mut file = std::fs::File::open(path.as_ref()).unwrap_or_else(|_| { eprintln!("fixdep: error opening file: {}", path); std::process::exit(2) });
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap_or_else(|_| { eprintln!("fixdep: read"); std::process::exit(2) });
    let buf = xmalloc(data.len() + 1) as *mut c_char;
    ptr::copy_nonoverlapping(data.as_ptr(), buf as *mut u8, data.len());
    *buf.add(data.len()) = 0;
    buf
}

unsafe fn is_ignored_file(s: *const c_char, len: usize) -> bool { str_ends_with(s, len, b"include/generated/autoconf.h") }
unsafe fn is_no_parse_file(s: *const c_char, len: usize) -> bool { str_ends_with(s, len, b".rlib") || str_ends_with(s, len, b".rmeta") || str_ends_with(s, len, b".so") }

unsafe fn parse_dep_file(mut p: *mut c_char, target: &CStr) {
    let mut saw_any_target = false;
    let mut is_target = true;
    let mut is_source = false;
    while *p != 0 {
        match *p as u8 {
            b'#' => { p = p.add(1); while *p != 0 && *p != b'\n' as c_char { if *p == b'\\' as c_char { p = p.add(1); } p = p.add(1); } continue; }
            b' ' | b'\t' => { p = p.add(1); continue; }
            b'\\' if *p.add(1) == b'\n' as c_char => { p = p.add(2); continue; }
            b'\n' => { p = p.add(1); is_target = true; continue; }
            b':' => { p = p.add(1); is_target = false; is_source = true; continue; }
            _ => {}
        }
        let mut q = p;
        while *q != 0 && *q != b' ' as c_char && *q != b'\t' as c_char && *q != b'\n' as c_char && *q != b'#' as c_char && *q != b':' as c_char {
            if *q == b'\\' as c_char { if *q.add(1) == b'\n' as c_char { break; } if *q.add(1) == b'#' as c_char || *q.add(1) == b':' as c_char { ptr::copy(p, p.add(1), q.offset_from(p) as usize); p = p.add(1); } }
            q = q.add(1);
        }
        if is_target { p = q; continue; }
        let saved = *q; *q = 0;
        let mut need_parse = false;
        let len = q.offset_from(p) as usize;
        if is_source { if !saw_any_target { saw_any_target = true; println!("source_{} := {}\n", target.to_string_lossy(), CStr::from_ptr(p).to_string_lossy()); println!("deps_{} := \\", target.to_string_lossy()); need_parse = true; } }
        else if !is_ignored_file(p, len) && !in_hashtable(p, len, FILE_HASHTAB.as_mut_ptr()) { println!("  {} \\", CStr::from_ptr(p).to_string_lossy()); need_parse = true; }
        if need_parse && !is_no_parse_file(p, len) { let buf = read_file(p); parse_config_file(buf); libc::free(buf as *mut c_void); }
        is_source = false; *q = saved; p = q;
    }
    if !saw_any_target { eprintln!("fixdep: parse error; no targets found"); std::process::exit(1); }
    println!("\n{}: $(deps_{})\n", target.to_string_lossy(), target.to_string_lossy());
    println!("$(deps_{}):", target.to_string_lossy());
}

fn main() {
    let args: Vec<CString> = std::env::args_os().map(|a| CString::new(a.as_encoded_bytes()).unwrap()).collect();
    if args.len() != 4 { usage(); }
    unsafe {
        println!("savedcmd_{} := {}\n", args[2].to_string_lossy(), args[3].to_string_lossy());
        let buf = read_file(args[1].as_ptr());
        parse_dep_file(buf, &CStr::from_ptr(args[2].as_ptr()));
        libc::free(buf as *mut c_void);
        io::stdout().flush().unwrap();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
