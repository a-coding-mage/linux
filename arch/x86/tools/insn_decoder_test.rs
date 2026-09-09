// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) IBM Corporation, 2009
 */

use libc::{c_char, c_int, c_uint, c_void, FILE};
use std::ptr;

// Declarations supplied by the included kernel instruction-decoder sources.
use crate::{insn, insn_field, insn_decode, INSN_MODE_32, INSN_MODE_64, KSYM_NAME_LEN};

extern "C" {
    static mut optind: c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: *mut libc::va_list) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strspn(s: *const c_char, accept: *const c_char) -> usize;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
}

static mut PROG: *const c_char = ptr::null();
static mut VERBOSE: c_int = 0;
static mut X86_64: c_int = 0;

unsafe fn usage() {
    fprintf(stderr, b"Usage: objdump -d a.out | awk -f objdump_reformat.awk | %s [-y|-n] [-v]\n\0".as_ptr() as *const c_char, PROG);
    fprintf(stderr, b"\t-y\t64bit mode\n\0".as_ptr() as *const c_char);
    fprintf(stderr, b"\t-n\t32bit mode\n\0".as_ptr() as *const c_char);
    fprintf(stderr, b"\t-v\tverbose mode\n\0".as_ptr() as *const c_char);
    exit(1);
}

unsafe fn malformed_line(line: *const c_char, line_nr: c_int) {
    fprintf(stderr, b"%s: error: malformed line %d:\n%s\0".as_ptr() as *const c_char, PROG, line_nr, line);
    exit(3);
}

unsafe fn pr_warn(fmt: *const c_char, mut ap: ...) {
    // C varargs are forwarded to vfprintf here.
    fprintf(stderr, b"%s: warning: \0".as_ptr() as *const c_char, PROG);
    vfprintf(stderr, fmt, &mut ap);
}

unsafe fn dump_field(fp: *mut FILE, name: *const c_char, indent: *const c_char, field: *mut insn_field) {
    fprintf(fp, b"%s.%s = {\n\0".as_ptr() as *const c_char, indent, name);
    fprintf(fp, b"%s\t.value = %d, bytes[] = {%x, %x, %x, %x},\n\0".as_ptr() as *const c_char,
        indent, (*field).value, (*field).bytes[0], (*field).bytes[1], (*field).bytes[2], (*field).bytes[3]);
    fprintf(fp, b"%s\t.got = %d, .nbytes = %d},\n\0".as_ptr() as *const c_char,
        indent, (*field).got, (*field).nbytes);
}

unsafe fn dump_insn(fp: *mut FILE, instruction: *mut insn) {
    fprintf(fp, b"Instruction = {\n\0".as_ptr() as *const c_char);
    dump_field(fp, b"prefixes\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).prefixes);
    dump_field(fp, b"rex_prefix\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).rex_prefix);
    dump_field(fp, b"vex_prefix\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).vex_prefix);
    dump_field(fp, b"opcode\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).opcode);
    dump_field(fp, b"modrm\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).modrm);
    dump_field(fp, b"sib\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).sib);
    dump_field(fp, b"displacement\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).displacement);
    dump_field(fp, b"immediate1\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).immediate1);
    dump_field(fp, b"immediate2\0".as_ptr() as *const c_char, b"\t\0".as_ptr() as *const c_char, &mut (*instruction).immediate2);
    fprintf(fp, b"\t.attr = %x, .opnd_bytes = %d, .addr_bytes = %d,\n\0".as_ptr() as *const c_char, (*instruction).attr, (*instruction).opnd_bytes, (*instruction).addr_bytes);
    fprintf(fp, b"\t.length = %d, .x86_64 = %d, .kaddr = %p}\n\0".as_ptr() as *const c_char, (*instruction).length, (*instruction).x86_64, (*instruction).kaddr);
}

unsafe fn parse_args(argc: c_int, argv: *mut *mut c_char) {
    PROG = *argv;
    let mut c;
    while { c = getopt(argc, argv, b"ynv\0".as_ptr() as *const c_char); c != -1 } {
        match c {
            121 => X86_64 = 1,
            110 => X86_64 = 0,
            118 => VERBOSE = 1,
            _ => usage(),
        }
    }
}

const BUFSIZE: usize = 256 + KSYM_NAME_LEN as usize;

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut line = [0i8; BUFSIZE];
    let mut sym = [0i8; BUFSIZE];
    strcpy(sym.as_mut_ptr(), b"<unknown>\0".as_ptr() as *const c_char);
    let mut insn_buff = [0u8; 16];
    let mut instruction: insn = std::mem::zeroed();
    let mut insns = 0;
    let mut warnings = 0;
    parse_args(argc, argv);
    while !fgets(line.as_mut_ptr(), BUFSIZE as c_int, stdin()).is_null() {
        let mut copy = [0i8; BUFSIZE];
        if line[0] == b'<' as i8 { strcpy(sym.as_mut_ptr(), line.as_ptr()); continue; }
        insns += 1;
        memset(insn_buff.as_mut_ptr() as *mut c_void, 0, 16);
        strcpy(copy.as_mut_ptr(), line.as_ptr());
        let tab1 = strchr(copy.as_ptr(), b'\t' as c_int);
        if tab1.is_null() { malformed_line(line.as_ptr(), insns); }
        let mut s = tab1.add(1);
        s = s.add(strspn(s, b" \0".as_ptr() as *const c_char));
        let tab2 = strchr(s, b'\t' as c_int);
        if tab2.is_null() { malformed_line(line.as_ptr(), insns); }
        *tab2 = 0;
        let mut nb = 0usize;
        while s < tab2 {
            let mut b = 0u32;
            if sscanf(s, b"%x\0".as_ptr() as *const c_char, &mut b) == 1 { insn_buff[nb] = b as u8; nb += 1; s = s.add(3); } else { break; }
        }
        let ret = insn_decode(&mut instruction, insn_buff.as_ptr(), 16, if X86_64 != 0 { INSN_MODE_64 } else { INSN_MODE_32 });
        if ret < 0 || instruction.length != nb as i32 {
            warnings += 1;
            fprintf(stderr, b"%s: warning: Found an x86 instruction decoder bug, please report this.\n\0".as_ptr() as *const c_char, PROG);
            fprintf(stderr, b"%s\0".as_ptr() as *const c_char, line.as_ptr());
            fprintf(stderr, b"%s: warning: objdump says %d bytes, but insn_get_length() says %d\n\0".as_ptr() as *const c_char, PROG, nb, instruction.length);
            if VERBOSE != 0 { dump_insn(stderr, &mut instruction); }
        }
    }
    if warnings != 0 { fprintf(stderr, b"%s: warning: Decoded and checked %d instructions with %d failures\n\0".as_ptr() as *const c_char, PROG, insns, warnings); }
    else { fprintf(stdout, b"  %s: success: Decoded and checked %d instructions\n\0".as_ptr() as *const c_char, PROG, insns); }
    0
}

unsafe fn stdin() -> *mut FILE { libc::stdin }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
