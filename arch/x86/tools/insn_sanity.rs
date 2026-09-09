// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * x86 decoder sanity test - based on test_get_insn.c
 *
 * Copyright (C) IBM Corporation, 2009
 * Copyright (C) Hitachi, Ltd., 2011
 */

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Read};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};
use std::ptr;

const DEFAULT_MAX_ITER: c_ulong = 10000;
const INSN_NOP: u8 = 0x90;

/* Supplied by the x86 decoder implementation. */
#[repr(C)]
pub struct insn_field {
    pub value: c_int,
    pub bytes: [u8; 4],
    pub got: c_int,
    pub nbytes: c_int,
}

#[repr(C)]
pub struct insn {
    pub prefixes: insn_field,
    pub rex_prefix: insn_field,
    pub vex_prefix: insn_field,
    pub opcode: insn_field,
    pub modrm: insn_field,
    pub sib: insn_field,
    pub displacement: insn_field,
    pub immediate1: insn_field,
    pub immediate2: insn_field,
    pub attr: c_uint,
    pub opnd_bytes: c_int,
    pub addr_bytes: c_int,
    pub length: c_int,
    pub x86_64: c_int,
    pub kaddr: *mut u8,
    pub next_byte: *mut u8,
}

extern "C" {
    fn insn_decode(insn: *mut insn, kaddr: *mut u8, max_bytes: usize, x86_64: c_int) -> c_int;
    fn srand(seed: c_uint);
    fn random() -> c_long;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
}

type c_long = isize;

static mut PROG: *const c_char = ptr::null();
static mut VERBOSE: c_int = 0;
static mut X86_64: c_int = 0;
static mut SEED: c_uint = 0;
static mut ITER_START: c_ulong = 0;
static mut ITER_END: c_ulong = DEFAULT_MAX_ITER;
static mut INPUT_FILE: *mut File = ptr::null_mut();

const MAX_INSN_SIZE: usize = 15;
const INSN_MODE_32: c_int = 1;
const INSN_MODE_64: c_int = 2;

unsafe fn usage(err: *const c_char) -> ! {
    if !err.is_null() {
        eprintln!("{}: Error: {}\n", cstr(PROG), cstr(err));
    }
    eprintln!("Usage: {} [-y|-n|-v] [-s seed[,no]] [-m max] [-i input]", cstr(PROG));
    eprintln!("\t-y\t64bit mode");
    eprintln!("\t-n\t32bit mode");
    eprintln!("\t-v\tVerbosity(-vv dumps any decoded result)");
    eprintln!("\t-s\tGive a random seed (and iteration number)");
    eprintln!("\t-m\tGive a maximum iteration number");
    eprintln!("\t-i\tGive an input file with decoded binary");
    std::process::exit(1)
}

unsafe fn cstr(s: *const c_char) -> String {
    if s.is_null() { String::new() } else { CStr::from_ptr(s).to_string_lossy().into_owned() }
}

unsafe fn dump_field(name: &str, indent: &str, field: *const insn_field) {
    let f = &*field;
    eprintln!("{}.{} = {{", indent, name);
    eprintln!("{}\t.value = {}, bytes[] = {{{:x}, {:x}, {:x}, {:x}}},", indent, f.value, f.bytes[0], f.bytes[1], f.bytes[2], f.bytes[3]);
    eprintln!("{}\t.got = {}, .nbytes = {} }},", indent, f.got, f.nbytes);
}

unsafe fn dump_insn(x: *const insn) {
    let i = &*x;
    println!("Instruction = {{");
    dump_field("prefixes", "\t", &i.prefixes);
    dump_field("rex_prefix", "\t", &i.rex_prefix);
    dump_field("vex_prefix", "\t", &i.vex_prefix);
    dump_field("opcode", "\t", &i.opcode);
    dump_field("modrm", "\t", &i.modrm);
    dump_field("sib", "\t", &i.sib);
    dump_field("displacement", "\t", &i.displacement);
    dump_field("immediate1", "\t", &i.immediate1);
    dump_field("immediate2", "\t", &i.immediate2);
    println!("\t.attr = {:x}, .opnd_bytes = {}, .addr_bytes = {},", i.attr, i.opnd_bytes, i.addr_bytes);
    println!("\t.length = {}, .x86_64 = {}, .kaddr = {:?}}}", i.length, i.x86_64, i.kaddr);
}

unsafe fn dump_stream(msg: &str, nr_iter: c_ulong, buf: *const u8, x: *const insn) {
    println!("{}:", msg);
    dump_insn(x);
    println!("You can reproduce this with below command(s);");
    print!(" $ echo");
    for j in 0..MAX_INSN_SIZE { print!(" {:02x}", *buf.add(j)); }
    println!(" | {} -i -", cstr(PROG));
    if INPUT_FILE.is_null() { println!("Or \n $ {} -s 0x{:x},{}", cstr(PROG), SEED, nr_iter); }
}

unsafe fn read_next_insn(buf: *mut u8) -> c_int {
    let file = &mut *INPUT_FILE;
    let mut line = String::new();
    if file.read_to_string(&mut line).is_err() || line.is_empty() { return 0; }
    let mut n = 0;
    for token in line.split_whitespace().take(MAX_INSN_SIZE) {
        if let Ok(v) = u8::from_str_radix(token, 16) { *buf.add(n) = v; n += 1; } else { break; }
    }
    n as c_int
}

unsafe fn generate_insn(buf: *mut u8) -> c_int {
    if !INPUT_FILE.is_null() { return read_next_insn(buf); }
    let mut i = 0;
    while i < MAX_INSN_SIZE - 1 { *(buf.add(i) as *mut c_ushort) = random() as c_ushort; i += 2; }
    while i < MAX_INSN_SIZE { *buf.add(i) = random() as u8; i += 1; }
    i as c_int
}

unsafe fn init_random_seed() {
    let mut f = match File::open("/dev/urandom") { Ok(v) => v, Err(_) => usage(CString::new("Failed to open /dev/urandom").unwrap().as_ptr()) };
    let mut bytes = [0u8; 4];
    if f.read_exact(&mut bytes).is_err() { usage(CString::new("Failed to open /dev/urandom").unwrap().as_ptr()); }
    SEED = u32::from_ne_bytes(bytes);
}

unsafe fn parse_args(argc: c_int, argv: *mut *mut c_char) {
    PROG = *argv;
    let options = CString::new("ynvs:m:i:").unwrap();
    loop {
        let c = getopt(argc, argv, options.as_ptr());
        if c == -1 { break; }
        match c as u8 as char {
            'y' => X86_64 = 1,
            'n' => X86_64 = 0,
            'v' => VERBOSE += 1,
            'i' => {
                let arg = CStr::from_ptr(optarg).to_string_lossy();
                if arg != "-" { INPUT_FILE = Box::into_raw(Box::new(File::open(arg.as_ref()).unwrap_or_else(|_| usage(CString::new("Failed to open input file").unwrap().as_ptr())))); }
                else { INPUT_FILE = Box::into_raw(Box::new(File::open("/dev/stdin").unwrap())); }
            }
            's' => {
                let arg = CStr::from_ptr(optarg).to_string_lossy();
                let mut p = arg.splitn(2, ',');
                SEED = p.next().unwrap().parse().unwrap_or_else(|_| usage(CString::new("Failed to parse seed").unwrap().as_ptr()));
                if let Some(v) = p.next() { ITER_START = v.parse().unwrap_or_else(|_| usage(CString::new("Failed to parse seed").unwrap().as_ptr())); }
                srand(SEED);
            }
            'm' => { ITER_END = CStr::from_ptr(optarg).to_string_lossy().parse().unwrap_or_else(|_| usage(CString::new("Failed to parse max_iter").unwrap().as_ptr())); }
            _ => usage(ptr::null()),
        }
    }
    if ITER_END < ITER_START { usage(CString::new("Max iteration number must be bigger than iter-num").unwrap().as_ptr()); }
    if !INPUT_FILE.is_null() && SEED != 0 { usage(CString::new("Don't use input file (-i) with random seed (-s)").unwrap().as_ptr()); }
    if INPUT_FILE.is_null() { if SEED == 0 { init_random_seed(); } srand(SEED); }
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_args(argc, argv);
    let mut buf = [0u8; MAX_INSN_SIZE * 2];
    for b in &mut buf[MAX_INSN_SIZE..] { *b = INSN_NOP; }
    let mut errors = 0;
    let mut count = 0;
    for i in 0..ITER_END {
        if generate_insn(buf.as_mut_ptr()) <= 0 { break; }
        if i < ITER_START { continue; }
        let mut decoded = std::mem::MaybeUninit::<insn>::uninit();
        let ret = insn_decode(decoded.as_mut_ptr(), buf.as_mut_ptr(), buf.len(), if X86_64 != 0 { INSN_MODE_64 } else { INSN_MODE_32 });
        let x = decoded.assume_init();
        if x.next_byte <= x.kaddr || x.kaddr.add(MAX_INSN_SIZE) < x.next_byte {
            dump_stream("Error: Found an access violation", i, buf.as_ptr(), &x); errors += 1;
        } else if VERBOSE != 0 && ret < 0 { dump_stream("Info: Found an undecodable input", i, buf.as_ptr(), &x); }
        else if VERBOSE >= 2 { dump_insn(&x); }
        count += 1;
    }
    eprintln!("  {}: {}: Decoded and checked {} {} instructions with {} errors (seed:0x{:x})", cstr(PROG), if errors != 0 { "failure" } else { "success" }, count, if INPUT_FILE.is_null() { "random" } else { "given" }, errors, SEED);
    if errors != 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
