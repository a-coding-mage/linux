// SPDX-License-Identifier: GPL-2.0-only
/*
 * Minimal BPF debugger
 *
 * Minimal BPF debugger that mimics the kernel's engine (w/o extensions)
 * and allows for single stepping through selected packets from a pcap
 * with a provided user filter in order to facilitate verification of a
 * BPF program. Besides others, this is useful to verify BPF programs
 * before attaching to a live system, and can be used in socket filters,
 * cls_bpf, xt_bpf, team driver and e.g. PTP code; in particular when a
 * single more complex BPF program is being used. Reasons for a more
 * complex BPF program are likely primarily to optimize execution time
 * for making a verdict when multiple simple BPF programs are combined
 * into one in order to prevent parsing same headers multiple times.
 *
 * More on how to debug BPF opcodes see Documentation/networking/filter.rst
 * which is the main document on BPF. Mini howto for getting started:
 *
 *  1) `./bpf_dbg` to enter the shell (shell cmds denoted with '>'):
 *  2) > load bpf 6,40 0 0 12,21 0 3 20... (output from `bpf_asm` or
 *     `tcpdump -iem1 -ddd port 22 | tr '\n' ','` to load as filter)
 *  3) > load pcap foo.pcap
 *  4) > run <n>/disassemble/dump/quit (self-explanatory)
 *  5) > breakpoint 2 (sets bp at loaded BPF insns 2, do `run` then;
 *       multiple bps can be set, of course, a call to `breakpoint`
 *       w/o args shows currently loaded bps, `breakpoint reset` for
 *       resetting all breakpoints)
 *  6) > select 3 (`run` etc will start from the 3rd packet in the pcap)
 *  7) > step [-<n>, +<n>] (performs single stepping through the BPF)
 *
 * Copyright 2013 Daniel Borkmann <borkmann@redhat.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const TCPDUMP_MAGIC: u32 = 0xa1b2c3d4;

const BPF_LDX_B: u16 = BPF_LDX | BPF_B;
const BPF_LDX_W: u16 = BPF_LDX | BPF_W;
const BPF_JMP_JA: u16 = BPF_JMP | BPF_JA;
const BPF_JMP_JEQ: u16 = BPF_JMP | BPF_JEQ;
const BPF_JMP_JGT: u16 = BPF_JMP | BPF_JGT;
const BPF_JMP_JGE: u16 = BPF_JMP | BPF_JGE;
const BPF_JMP_JSET: u16 = BPF_JMP | BPF_JSET;
const BPF_ALU_ADD: u16 = BPF_ALU | BPF_ADD;
const BPF_ALU_SUB: u16 = BPF_ALU | BPF_SUB;
const BPF_ALU_MUL: u16 = BPF_ALU | BPF_MUL;
const BPF_ALU_DIV: u16 = BPF_ALU | BPF_DIV;
const BPF_ALU_MOD: u16 = BPF_ALU | BPF_MOD;
const BPF_ALU_NEG: u16 = BPF_ALU | BPF_NEG;
const BPF_ALU_AND: u16 = BPF_ALU | BPF_AND;
const BPF_ALU_OR: u16 = BPF_ALU | BPF_OR;
const BPF_ALU_XOR: u16 = BPF_ALU | BPF_XOR;
const BPF_ALU_LSH: u16 = BPF_ALU | BPF_LSH;
const BPF_ALU_RSH: u16 = BPF_ALU | BPF_RSH;
const BPF_MISC_TAX: u16 = BPF_MISC | BPF_TAX;
const BPF_MISC_TXA: u16 = BPF_MISC | BPF_TXA;
const BPF_LD_B: u16 = BPF_LD | BPF_B;
const BPF_LD_H: u16 = BPF_LD | BPF_H;
const BPF_LD_W: u16 = BPF_LD | BPF_W;

const CMD_OK: c_int = 0;
const CMD_ERR: c_int = 1;
const CMD_EX: c_int = 2;

const BPF_LD: u16 = 0x00;
const BPF_LDX: u16 = 0x01;
const BPF_ST: u16 = 0x02;
const BPF_STX: u16 = 0x03;
const BPF_ALU: u16 = 0x04;
const BPF_JMP: u16 = 0x05;
const BPF_RET: u16 = 0x06;
const BPF_MISC: u16 = 0x07;
const BPF_W: u16 = 0x00;
const BPF_H: u16 = 0x08;
const BPF_B: u16 = 0x10;
const BPF_IMM: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_IND: u16 = 0x40;
const BPF_MEM: u16 = 0x60;
const BPF_LEN: u16 = 0x80;
const BPF_MSH: u16 = 0xa0;
const BPF_ADD: u16 = 0x00;
const BPF_SUB: u16 = 0x10;
const BPF_MUL: u16 = 0x20;
const BPF_DIV: u16 = 0x30;
const BPF_OR: u16 = 0x40;
const BPF_AND: u16 = 0x50;
const BPF_LSH: u16 = 0x60;
const BPF_RSH: u16 = 0x70;
const BPF_NEG: u16 = 0x80;
const BPF_MOD: u16 = 0x90;
const BPF_XOR: u16 = 0xa0;
const BPF_JA: u16 = 0x00;
const BPF_JEQ: u16 = 0x10;
const BPF_JGT: u16 = 0x20;
const BPF_JGE: u16 = 0x30;
const BPF_JSET: u16 = 0x40;
const BPF_K: u16 = 0x00;
const BPF_X: u16 = 0x08;
const BPF_A: u16 = 0x10;
const BPF_TAX: u16 = 0x00;
const BPF_TXA: u16 = 0x80;
const BPF_MEMWORDS: usize = 16;
const BPF_MAXINSNS: usize = 4096;
const SKF_AD_OFF: u32 = 0xfffff000;

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_ATTACH_FILTER: c_int = 26;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_SHARED: c_int = 0x01;
const MAP_LOCKED: c_int = 0x2000;
const SIGINT: c_int = 2;

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

fn bpf_class(code: u16) -> u16 { code & 0x07 }
fn bpf_op(code: u16) -> u16 { code & 0xf0 }
fn s_isreg(mode: u32) -> bool { (mode & 0o170000) == 0o100000 }

#[repr(C)]
#[derive(Copy, Clone)]
struct sock_filter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

#[repr(C)]
struct shell_cmd {
    name: *const c_char,
    func: unsafe fn(*mut c_char) -> c_int,
}

#[repr(C)]
struct pcap_filehdr {
    magic: u32,
    version_major: u16,
    version_minor: u16,
    thiszone: i32,
    sigfigs: u32,
    snaplen: u32,
    linktype: u32,
}

#[repr(C)]
struct pcap_timeval {
    tv_sec: i32,
    tv_usec: i32,
}

#[repr(C)]
struct pcap_pkthdr {
    ts: pcap_timeval,
    caplen: u32,
    len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_regs {
    A: u32,
    X: u32,
    M: [u32; BPF_MEMWORDS],
    R: u32,
    Rs: bool,
    Pc: u16,
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    _rest: [u8; 116],
}

type FILE = c_void;

unsafe extern "C" {
    static mut rl_instream: *mut FILE;
    static mut rl_outstream: *mut FILE;
    static mut rl_readline_name: *const c_char;
    static mut rl_terminal_name: *const c_char;
    static mut rl_catch_signals: c_int;
    static mut rl_catch_sigwinch: c_int;
    static mut rl_attempted_completion_function:
        Option<unsafe extern "C" fn(*const c_char, c_int, c_int) -> *mut *mut c_char>;
    static mut rl_end: c_int;
    static mut emacs_meta_keymap: *mut c_void;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut errno: c_int;

    fn vfprintf(stream: *mut FILE, fmt: *const c_char, ap: VaList) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: u32) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn ntohl(netlong: u32) -> u32;
    fn ntohs(netshort: u16) -> u16;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn readline(prompt: *const c_char) -> *mut c_char;
    fn add_history(line: *const c_char);
    fn read_history(filename: *const c_char) -> c_int;
    fn write_history(filename: *const c_char) -> c_int;
    fn clear_history();
    fn rl_completion_matches(text: *const c_char, entry_func: unsafe extern "C" fn(*const c_char, c_int) -> *mut c_char) -> *mut *mut c_char;
    fn rl_kill_line(count: c_int, key: c_int) -> c_int;
    fn rl_crlf() -> c_int;
    fn rl_refresh_line(ignore1: c_int, ignore2: c_int) -> c_int;
    fn rl_free_line_state();
    fn rl_bind_key(key: c_int, function: unsafe extern "C" fn(c_int, c_int) -> c_int) -> c_int;
    fn rl_bind_key_in_map(key: c_int, function: unsafe extern "C" fn(c_int, c_int) -> c_int, map: *mut c_void) -> c_int;
    fn rl_complete(count: c_int, key: c_int) -> c_int;
    fn rl_read_init_file(filename: *const c_char) -> c_int;
    fn rl_prep_terminal(meta_flag: c_int);
    fn rl_set_signals();
    fn rl_deprep_terminal();
}

type VaList = *mut c_void;

static mut bpf_image: [sock_filter; BPF_MAXINSNS + 1] =
    [sock_filter { code: 0, jt: 0, jf: 0, k: 0 }; BPF_MAXINSNS + 1];
static mut bpf_prog_len: c_uint = 0;

static mut bpf_breakpoints: [c_int; 64] = [0; 64];
static mut bpf_regs: [bpf_regs; BPF_MAXINSNS + 1] =
    [bpf_regs { A: 0, X: 0, M: [0; BPF_MEMWORDS], R: 0, Rs: false, Pc: 0 }; BPF_MAXINSNS + 1];
static mut bpf_curr: bpf_regs = bpf_regs { A: 0, X: 0, M: [0; BPF_MEMWORDS], R: 0, Rs: false, Pc: 0 };
static mut bpf_regs_len: c_uint = 0;

static mut pcap_fd: c_int = -1;
static mut pcap_packet: c_uint = 0;
static mut pcap_map_size: usize = 0;
static mut pcap_ptr_va_start: *mut c_char = ptr::null_mut();
static mut pcap_ptr_va_curr: *mut c_char = ptr::null_mut();

static op_table: [*const c_char; 151] = {
    let mut t = [ptr::null(); 151];
    t[BPF_ST as usize] = c!("st");
    t[BPF_STX as usize] = c!("stx");
    t[BPF_LD_B as usize] = c!("ldb");
    t[BPF_LD_H as usize] = c!("ldh");
    t[BPF_LD_W as usize] = c!("ld");
    t[BPF_LDX as usize] = c!("ldx");
    t[BPF_LDX_B as usize] = c!("ldxb");
    t[BPF_JMP_JA as usize] = c!("ja");
    t[BPF_JMP_JEQ as usize] = c!("jeq");
    t[BPF_JMP_JGT as usize] = c!("jgt");
    t[BPF_JMP_JGE as usize] = c!("jge");
    t[BPF_JMP_JSET as usize] = c!("jset");
    t[BPF_ALU_ADD as usize] = c!("add");
    t[BPF_ALU_SUB as usize] = c!("sub");
    t[BPF_ALU_MUL as usize] = c!("mul");
    t[BPF_ALU_DIV as usize] = c!("div");
    t[BPF_ALU_MOD as usize] = c!("mod");
    t[BPF_ALU_NEG as usize] = c!("neg");
    t[BPF_ALU_AND as usize] = c!("and");
    t[BPF_ALU_OR as usize] = c!("or");
    t[BPF_ALU_XOR as usize] = c!("xor");
    t[BPF_ALU_LSH as usize] = c!("lsh");
    t[BPF_ALU_RSH as usize] = c!("rsh");
    t[BPF_MISC_TAX as usize] = c!("tax");
    t[BPF_MISC_TXA as usize] = c!("txa");
    t[BPF_RET as usize] = c!("ret");
    t
};

unsafe fn rl_printf(fmt: *const c_char, mut args: ...) -> c_int {
    vfprintf(rl_outstream, fmt, args.as_va_list())
}

unsafe fn matches(cmd: *const c_char, pattern: *const c_char) -> c_int {
    let len = strlen(cmd);
    if len > strlen(pattern) {
        return -1;
    }
    memcmp(pattern as *const c_void, cmd as *const c_void, len)
}

unsafe fn hex_dump(buf: *const u8, len: usize) {
    let mut i: c_int;
    rl_printf(c!("%3u: "), 0);
    i = 0;
    while (i as usize) < len {
        if i != 0 && (i % 16) == 0 {
            rl_printf(c!("\n%3u: "), i);
        }
        rl_printf(c!("%02x "), *buf.add(i as usize) as c_int);
        i += 1;
    }
    rl_printf(c!("\n"));
}

unsafe fn bpf_prog_loaded() -> bool {
    if bpf_prog_len == 0 {
        rl_printf(c!("no bpf program loaded!\n"));
    }
    bpf_prog_len > 0
}

unsafe fn bpf_disasm(f: sock_filter, i: c_uint) {
    let op: *const c_char;
    let fmt: *const c_char;
    let mut val: c_int = f.k as c_int;
    let mut buf = [0 as c_char; 256];

    match f.code {
        x if x == (BPF_RET | BPF_K) => { op = op_table[BPF_RET as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_RET | BPF_A) => { op = op_table[BPF_RET as usize]; fmt = c!("a"); }
        x if x == (BPF_RET | BPF_X) => { op = op_table[BPF_RET as usize]; fmt = c!("x"); }
        BPF_MISC_TAX => { op = op_table[BPF_MISC_TAX as usize]; fmt = c!(""); }
        BPF_MISC_TXA => { op = op_table[BPF_MISC_TXA as usize]; fmt = c!(""); }
        BPF_ST => { op = op_table[BPF_ST as usize]; fmt = c!("M[%d]"); }
        BPF_STX => { op = op_table[BPF_STX as usize]; fmt = c!("M[%d]"); }
        x if x == (BPF_LD_W | BPF_ABS) => { op = op_table[BPF_LD_W as usize]; fmt = c!("[%d]"); }
        x if x == (BPF_LD_H | BPF_ABS) => { op = op_table[BPF_LD_H as usize]; fmt = c!("[%d]"); }
        x if x == (BPF_LD_B | BPF_ABS) => { op = op_table[BPF_LD_B as usize]; fmt = c!("[%d]"); }
        x if x == (BPF_LD_W | BPF_LEN) => { op = op_table[BPF_LD_W as usize]; fmt = c!("#len"); }
        x if x == (BPF_LD_W | BPF_IND) => { op = op_table[BPF_LD_W as usize]; fmt = c!("[x+%d]"); }
        x if x == (BPF_LD_H | BPF_IND) => { op = op_table[BPF_LD_H as usize]; fmt = c!("[x+%d]"); }
        x if x == (BPF_LD_B | BPF_IND) => { op = op_table[BPF_LD_B as usize]; fmt = c!("[x+%d]"); }
        x if x == (BPF_LD | BPF_IMM) => { op = op_table[BPF_LD_W as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_LDX | BPF_IMM) => { op = op_table[BPF_LDX as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_LDX_B | BPF_MSH) => { op = op_table[BPF_LDX_B as usize]; fmt = c!("4*([%d]&0xf)"); }
        x if x == (BPF_LD | BPF_MEM) => { op = op_table[BPF_LD_W as usize]; fmt = c!("M[%d]"); }
        x if x == (BPF_LDX | BPF_MEM) => { op = op_table[BPF_LDX as usize]; fmt = c!("M[%d]"); }
        BPF_JMP_JA => { op = op_table[BPF_JMP_JA as usize]; fmt = c!("%d"); val = i.wrapping_add(1).wrapping_add(f.k) as c_int; }
        x if x == (BPF_JMP_JGT | BPF_X) => { op = op_table[BPF_JMP_JGT as usize]; fmt = c!("x"); }
        x if x == (BPF_JMP_JGT | BPF_K) => { op = op_table[BPF_JMP_JGT as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_JMP_JGE | BPF_X) => { op = op_table[BPF_JMP_JGE as usize]; fmt = c!("x"); }
        x if x == (BPF_JMP_JGE | BPF_K) => { op = op_table[BPF_JMP_JGE as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_JMP_JEQ | BPF_X) => { op = op_table[BPF_JMP_JEQ as usize]; fmt = c!("x"); }
        x if x == (BPF_JMP_JEQ | BPF_K) => { op = op_table[BPF_JMP_JEQ as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_JMP_JSET | BPF_X) => { op = op_table[BPF_JMP_JSET as usize]; fmt = c!("x"); }
        x if x == (BPF_JMP_JSET | BPF_K) => { op = op_table[BPF_JMP_JSET as usize]; fmt = c!("#%#x"); }
        BPF_ALU_NEG => { op = op_table[BPF_ALU_NEG as usize]; fmt = c!(""); }
        x if x == (BPF_ALU_LSH | BPF_X) => { op = op_table[BPF_ALU_LSH as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_LSH | BPF_K) => { op = op_table[BPF_ALU_LSH as usize]; fmt = c!("#%d"); }
        x if x == (BPF_ALU_RSH | BPF_X) => { op = op_table[BPF_ALU_RSH as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_RSH | BPF_K) => { op = op_table[BPF_ALU_RSH as usize]; fmt = c!("#%d"); }
        x if x == (BPF_ALU_ADD | BPF_X) => { op = op_table[BPF_ALU_ADD as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_ADD | BPF_K) => { op = op_table[BPF_ALU_ADD as usize]; fmt = c!("#%d"); }
        x if x == (BPF_ALU_SUB | BPF_X) => { op = op_table[BPF_ALU_SUB as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_SUB | BPF_K) => { op = op_table[BPF_ALU_SUB as usize]; fmt = c!("#%d"); }
        x if x == (BPF_ALU_MUL | BPF_X) => { op = op_table[BPF_ALU_MUL as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_MUL | BPF_K) => { op = op_table[BPF_ALU_MUL as usize]; fmt = c!("#%d"); }
        x if x == (BPF_ALU_DIV | BPF_X) => { op = op_table[BPF_ALU_DIV as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_DIV | BPF_K) => { op = op_table[BPF_ALU_DIV as usize]; fmt = c!("#%d"); }
        x if x == (BPF_ALU_MOD | BPF_X) => { op = op_table[BPF_ALU_MOD as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_MOD | BPF_K) => { op = op_table[BPF_ALU_MOD as usize]; fmt = c!("#%d"); }
        x if x == (BPF_ALU_AND | BPF_X) => { op = op_table[BPF_ALU_AND as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_AND | BPF_K) => { op = op_table[BPF_ALU_AND as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_ALU_OR | BPF_X) => { op = op_table[BPF_ALU_OR as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_OR | BPF_K) => { op = op_table[BPF_ALU_OR as usize]; fmt = c!("#%#x"); }
        x if x == (BPF_ALU_XOR | BPF_X) => { op = op_table[BPF_ALU_XOR as usize]; fmt = c!("x"); }
        x if x == (BPF_ALU_XOR | BPF_K) => { op = op_table[BPF_ALU_XOR as usize]; fmt = c!("#%#x"); }
        _ => { op = c!("nosup"); fmt = c!("%#x"); val = f.code as c_int; }
    }

    memset(buf.as_mut_ptr() as *mut c_void, 0, size_of::<[c_char; 256]>());
    snprintf(buf.as_mut_ptr(), buf.len(), fmt, val);
    buf[buf.len() - 1] = 0;

    if bpf_class(f.code) == BPF_JMP && bpf_op(f.code) != BPF_JA {
        rl_printf(c!("l%d:\t%s %s, l%d, l%d\n"), i, op, buf.as_ptr(),
                  i + 1 + f.jt as c_uint, i + 1 + f.jf as c_uint);
    } else {
        rl_printf(c!("l%d:\t%s %s\n"), i, op, buf.as_ptr());
    }
}

unsafe fn bpf_dump_curr(r: *mut bpf_regs, f: *mut sock_filter) {
    let mut i: c_int = 0;
    let mut m: c_int = 0;
    rl_printf(c!("pc:       [%u]\n"), (*r).Pc as c_uint);
    rl_printf(c!("code:     [%u] jt[%u] jf[%u] k[%u]\n"), (*f).code as c_uint, (*f).jt as c_uint, (*f).jf as c_uint, (*f).k);
    rl_printf(c!("curr:     "));
    bpf_disasm(*f, (*r).Pc as c_uint);
    if (*f).jt != 0 || (*f).jf != 0 {
        rl_printf(c!("jt:       "));
        bpf_disasm(*f.add((*f).jt as usize + 1), (*r).Pc as c_uint + (*f).jt as c_uint + 1);
        rl_printf(c!("jf:       "));
        bpf_disasm(*f.add((*f).jf as usize + 1), (*r).Pc as c_uint + (*f).jf as c_uint + 1);
    }
    rl_printf(c!("A:        [%#08x][%u]\n"), (*r).A, (*r).A);
    rl_printf(c!("X:        [%#08x][%u]\n"), (*r).X, (*r).X);
    if (*r).Rs {
        rl_printf(c!("ret:      [%#08x][%u]!\n"), (*r).R, (*r).R);
    }
    while (i as usize) < BPF_MEMWORDS {
        if (*r).M[i as usize] != 0 {
            m += 1;
            rl_printf(c!("M[%d]: [%#08x][%u]\n"), i, (*r).M[i as usize], (*r).M[i as usize]);
        }
        i += 1;
    }
    if m == 0 {
        rl_printf(c!("M[0,%d]:  [%#08x][%u]\n"), (BPF_MEMWORDS - 1) as c_int, 0, 0);
    }
}

unsafe fn bpf_dump_pkt(pkt: *mut u8, pkt_caplen: u32, pkt_len: u32) {
    if pkt_caplen != pkt_len { rl_printf(c!("cap: %u, len: %u\n"), pkt_caplen, pkt_len); }
    else { rl_printf(c!("len: %u\n"), pkt_len); }
    hex_dump(pkt, pkt_caplen as usize);
}

unsafe fn bpf_disasm_all(f: *const sock_filter, len: c_uint) {
    let mut i = 0;
    while i < len {
        bpf_disasm(*f.add(i as usize), i);
        i += 1;
    }
}

unsafe fn bpf_dump_all(f: *const sock_filter, len: c_uint) {
    let mut i = 0;
    rl_printf(c!("/* { op, jt, jf, k }, */\n"));
    while i < len {
        rl_printf(c!("{ %#04x, %2u, %2u, %#010x },\n"), (*f.add(i as usize)).code as c_uint, (*f.add(i as usize)).jt as c_uint, (*f.add(i as usize)).jf as c_uint, (*f.add(i as usize)).k);
        i += 1;
    }
}

unsafe fn bpf_runnable(f: *mut sock_filter, len: c_uint) -> bool {
    let bpf = sock_fprog { filter: f, len: len as u16 };
    let sock = socket(AF_INET, SOCK_DGRAM, 0);
    if sock < 0 { rl_printf(c!("cannot open socket!\n")); return false; }
    let ret = setsockopt(sock, SOL_SOCKET, SO_ATTACH_FILTER, &bpf as *const _ as *const c_void, size_of::<sock_fprog>() as u32);
    close(sock);
    if ret < 0 { rl_printf(c!("program not allowed to run by kernel!\n")); return false; }
    let mut i = 0;
    while i < len {
        if bpf_class((*f.add(i as usize)).code) == BPF_LD && (*f.add(i as usize)).k > SKF_AD_OFF {
            rl_printf(c!("extensions currently not supported!\n"));
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn bpf_reset_breakpoints() {
    let mut i = 0;
    while i < bpf_breakpoints.len() { bpf_breakpoints[i] = -1; i += 1; }
}

unsafe fn bpf_set_breakpoints(where_: c_uint) {
    let mut i = 0;
    let mut set = false;
    while i < bpf_breakpoints.len() {
        if bpf_breakpoints[i] == where_ as c_int {
            rl_printf(c!("breakpoint already set!\n"));
            set = true;
            break;
        }
        if bpf_breakpoints[i] == -1 && set == false {
            bpf_breakpoints[i] = where_ as c_int;
            set = true;
        }
        i += 1;
    }
    if !set { rl_printf(c!("too many breakpoints set, reset first!\n")); }
}

unsafe fn bpf_dump_breakpoints() {
    let mut i = 0;
    rl_printf(c!("breakpoints: "));
    while i < bpf_breakpoints.len() {
        if bpf_breakpoints[i] >= 0 { rl_printf(c!("%d "), bpf_breakpoints[i]); }
        i += 1;
    }
    rl_printf(c!("\n"));
}

unsafe fn bpf_reset() {
    bpf_regs_len = 0;
    memset(&raw mut bpf_regs as *mut c_void, 0, size_of::<[bpf_regs; BPF_MAXINSNS + 1]>());
    memset(&raw mut bpf_curr as *mut c_void, 0, size_of::<bpf_regs>());
}

unsafe fn bpf_safe_regs() {
    memcpy((&raw mut bpf_regs).cast::<bpf_regs>().add(bpf_regs_len as usize) as *mut c_void, &raw const bpf_curr as *const c_void, size_of::<bpf_regs>());
    bpf_regs_len += 1;
}

unsafe fn bpf_restore_regs(off: c_int) -> bool {
    let index = (bpf_regs_len as c_int - 1 + off) as c_uint;
    if index == 0 {
        bpf_reset();
        true
    } else if index < bpf_regs_len {
        memcpy(&raw mut bpf_curr as *mut c_void, (&raw const bpf_regs).cast::<bpf_regs>().add(index as usize) as *const c_void, size_of::<bpf_regs>());
        bpf_regs_len = index;
        true
    } else {
        rl_printf(c!("reached bottom of register history stack!\n"));
        false
    }
}

unsafe fn extract_u32(pkt: *mut u8, off: u32) -> u32 {
    let mut r: u32 = 0;
    memcpy(&mut r as *mut _ as *mut c_void, pkt.add(off as usize) as *const c_void, size_of::<u32>());
    ntohl(r)
}

unsafe fn extract_u16(pkt: *mut u8, off: u32) -> u16 {
    let mut r: u16 = 0;
    memcpy(&mut r as *mut _ as *mut c_void, pkt.add(off as usize) as *const c_void, size_of::<u16>());
    ntohs(r)
}

unsafe fn extract_u8(pkt: *mut u8, off: u32) -> u8 { *pkt.add(off as usize) }

unsafe fn set_return(r: *mut bpf_regs) {
    (*r).R = 0;
    (*r).Rs = true;
}

unsafe fn bpf_single_step(r: *mut bpf_regs, f: *mut sock_filter, pkt: *mut u8, pkt_caplen: u32, pkt_len: u32) {
    let K = (*f).k;
    let mut d: c_int;
    match (*f).code {
        x if x == (BPF_RET | BPF_K) => { (*r).R = K; (*r).Rs = true; }
        x if x == (BPF_RET | BPF_A) => { (*r).R = (*r).A; (*r).Rs = true; }
        x if x == (BPF_RET | BPF_X) => { (*r).R = (*r).X; (*r).Rs = true; }
        BPF_MISC_TAX => (*r).X = (*r).A,
        BPF_MISC_TXA => (*r).A = (*r).X,
        BPF_ST => (*r).M[K as usize] = (*r).A,
        BPF_STX => (*r).M[K as usize] = (*r).X,
        x if x == (BPF_LD_W | BPF_ABS) => { d = pkt_caplen.wrapping_sub(K) as c_int; if d >= size_of::<u32>() as c_int { (*r).A = extract_u32(pkt, K); } else { set_return(r); } }
        x if x == (BPF_LD_H | BPF_ABS) => { d = pkt_caplen.wrapping_sub(K) as c_int; if d >= size_of::<u16>() as c_int { (*r).A = extract_u16(pkt, K) as u32; } else { set_return(r); } }
        x if x == (BPF_LD_B | BPF_ABS) => { d = pkt_caplen.wrapping_sub(K) as c_int; if d >= size_of::<u8>() as c_int { (*r).A = extract_u8(pkt, K) as u32; } else { set_return(r); } }
        x if x == (BPF_LD_W | BPF_IND) => { d = pkt_caplen.wrapping_sub((*r).X.wrapping_add(K)) as c_int; if d >= size_of::<u32>() as c_int { (*r).A = extract_u32(pkt, (*r).X.wrapping_add(K)); } }
        x if x == (BPF_LD_H | BPF_IND) => { d = pkt_caplen.wrapping_sub((*r).X.wrapping_add(K)) as c_int; if d >= size_of::<u16>() as c_int { (*r).A = extract_u16(pkt, (*r).X.wrapping_add(K)) as u32; } else { set_return(r); } }
        x if x == (BPF_LD_B | BPF_IND) => { d = pkt_caplen.wrapping_sub((*r).X.wrapping_add(K)) as c_int; if d >= size_of::<u8>() as c_int { (*r).A = extract_u8(pkt, (*r).X.wrapping_add(K)) as u32; } else { set_return(r); } }
        x if x == (BPF_LDX_B | BPF_MSH) => { d = pkt_caplen.wrapping_sub(K) as c_int; if d >= size_of::<u8>() as c_int { (*r).X = extract_u8(pkt, K) as u32; (*r).X = ((*r).X & 0xf) << 2; } else { set_return(r); } }
        x if x == (BPF_LD_W | BPF_LEN) => (*r).A = pkt_len,
        x if x == (BPF_LDX_W | BPF_LEN) => (*r).A = pkt_len,
        x if x == (BPF_LD | BPF_IMM) => (*r).A = K,
        x if x == (BPF_LDX | BPF_IMM) => (*r).X = K,
        x if x == (BPF_LD | BPF_MEM) => (*r).A = (*r).M[K as usize],
        x if x == (BPF_LDX | BPF_MEM) => (*r).X = (*r).M[K as usize],
        BPF_JMP_JA => (*r).Pc = (*r).Pc.wrapping_add(K as u16),
        x if x == (BPF_JMP_JGT | BPF_X) => (*r).Pc = (*r).Pc.wrapping_add(if (*r).A > (*r).X { (*f).jt } else { (*f).jf } as u16),
        x if x == (BPF_JMP_JGT | BPF_K) => (*r).Pc = (*r).Pc.wrapping_add(if (*r).A > K { (*f).jt } else { (*f).jf } as u16),
        x if x == (BPF_JMP_JGE | BPF_X) => (*r).Pc = (*r).Pc.wrapping_add(if (*r).A >= (*r).X { (*f).jt } else { (*f).jf } as u16),
        x if x == (BPF_JMP_JGE | BPF_K) => (*r).Pc = (*r).Pc.wrapping_add(if (*r).A >= K { (*f).jt } else { (*f).jf } as u16),
        x if x == (BPF_JMP_JEQ | BPF_X) => (*r).Pc = (*r).Pc.wrapping_add(if (*r).A == (*r).X { (*f).jt } else { (*f).jf } as u16),
        x if x == (BPF_JMP_JEQ | BPF_K) => (*r).Pc = (*r).Pc.wrapping_add(if (*r).A == K { (*f).jt } else { (*f).jf } as u16),
        x if x == (BPF_JMP_JSET | BPF_X) => (*r).Pc = (*r).Pc.wrapping_add(if ((*r).A & (*r).X) != 0 { (*f).jt } else { (*f).jf } as u16),
        x if x == (BPF_JMP_JSET | BPF_K) => (*r).Pc = (*r).Pc.wrapping_add(if ((*r).A & K) != 0 { (*f).jt } else { (*f).jf } as u16),
        BPF_ALU_NEG => (*r).A = (0u32).wrapping_sub((*r).A),
        x if x == (BPF_ALU_LSH | BPF_X) => (*r).A = (*r).A.wrapping_shl((*r).X),
        x if x == (BPF_ALU_LSH | BPF_K) => (*r).A = (*r).A.wrapping_shl(K),
        x if x == (BPF_ALU_RSH | BPF_X) => (*r).A >>= (*r).X,
        x if x == (BPF_ALU_RSH | BPF_K) => (*r).A >>= K,
        x if x == (BPF_ALU_ADD | BPF_X) => (*r).A = (*r).A.wrapping_add((*r).X),
        x if x == (BPF_ALU_ADD | BPF_K) => (*r).A = (*r).A.wrapping_add(K),
        x if x == (BPF_ALU_SUB | BPF_X) => (*r).A = (*r).A.wrapping_sub((*r).X),
        x if x == (BPF_ALU_SUB | BPF_K) => (*r).A = (*r).A.wrapping_sub(K),
        x if x == (BPF_ALU_MUL | BPF_X) => (*r).A = (*r).A.wrapping_mul((*r).X),
        x if x == (BPF_ALU_MUL | BPF_K) => (*r).A = (*r).A.wrapping_mul(K),
        x if x == (BPF_ALU_DIV | BPF_X) || x == (BPF_ALU_MOD | BPF_X) => { if (*r).X == 0 { set_return(r); } else if (*f).code == (BPF_ALU_DIV | BPF_X) { (*r).A /= (*r).X; } else { (*r).A %= (*r).X; } }
        x if x == (BPF_ALU_DIV | BPF_K) || x == (BPF_ALU_MOD | BPF_K) => { if K == 0 { set_return(r); } else if (*f).code == (BPF_ALU_DIV | BPF_K) { (*r).A /= K; } else { (*r).A %= K; } }
        x if x == (BPF_ALU_AND | BPF_X) => (*r).A &= (*r).X,
        x if x == (BPF_ALU_AND | BPF_K) => (*r).A &= K,
        x if x == (BPF_ALU_OR | BPF_X) => (*r).A |= (*r).X,
        x if x == (BPF_ALU_OR | BPF_K) => (*r).A |= K,
        x if x == (BPF_ALU_XOR | BPF_X) => (*r).A ^= (*r).X,
        x if x == (BPF_ALU_XOR | BPF_K) => (*r).A ^= K,
        _ => {}
    }
}

unsafe fn bpf_pc_has_breakpoint(pc: u16) -> bool {
    let mut i = 0;
    while i < bpf_breakpoints.len() {
        if bpf_breakpoints[i] >= 0 && bpf_breakpoints[i] == pc as c_int { return true; }
        i += 1;
    }
    false
}

unsafe fn bpf_handle_breakpoint(r: *mut bpf_regs, f: *mut sock_filter, pkt: *mut u8, pkt_caplen: u32, pkt_len: u32) -> bool {
    rl_printf(c!("-- register dump --\n"));
    bpf_dump_curr(r, f.add((*r).Pc as usize));
    rl_printf(c!("-- packet dump --\n"));
    bpf_dump_pkt(pkt, pkt_caplen, pkt_len);
    rl_printf(c!("(breakpoint)\n"));
    true
}

unsafe fn bpf_run_all(f: *mut sock_filter, _bpf_len: u16, pkt: *mut u8, pkt_caplen: u32, pkt_len: u32) -> c_int {
    let mut stop = false;
    while bpf_curr.Rs == false && stop == false {
        bpf_safe_regs();
        if bpf_pc_has_breakpoint(bpf_curr.Pc) { stop = bpf_handle_breakpoint(&raw mut bpf_curr, f, pkt, pkt_caplen, pkt_len); }
        bpf_single_step(&raw mut bpf_curr, f.add(bpf_curr.Pc as usize), pkt, pkt_caplen, pkt_len);
        bpf_curr.Pc = bpf_curr.Pc.wrapping_add(1);
    }
    if stop { -1 } else { bpf_curr.R as c_int }
}

unsafe fn bpf_run_stepping(f: *mut sock_filter, _bpf_len: u16, pkt: *mut u8, pkt_caplen: u32, pkt_len: u32, next: c_int) -> c_int {
    let mut stop = false;
    let mut i = 1;
    while !bpf_curr.Rs && !stop {
        bpf_safe_regs();
        if { let old = i; i += 1; old } == next { stop = bpf_handle_breakpoint(&raw mut bpf_curr, f, pkt, pkt_caplen, pkt_len); }
        bpf_single_step(&raw mut bpf_curr, f.add(bpf_curr.Pc as usize), pkt, pkt_caplen, pkt_len);
        bpf_curr.Pc = bpf_curr.Pc.wrapping_add(1);
    }
    if stop { -1 } else { bpf_curr.R as c_int }
}

unsafe fn pcap_loaded() -> bool {
    if pcap_fd < 0 { rl_printf(c!("no pcap file loaded!\n")); }
    pcap_fd >= 0
}

unsafe fn pcap_curr_pkt() -> *mut pcap_pkthdr { pcap_ptr_va_curr as *mut pcap_pkthdr }

unsafe fn pcap_next_pkt() -> bool {
    let hdr = pcap_curr_pkt();
    if pcap_ptr_va_curr.add(size_of::<pcap_pkthdr>()).offset_from(pcap_ptr_va_start) as usize >= pcap_map_size { return false; }
    if (*hdr).caplen == 0 || (*hdr).len == 0 || (*hdr).caplen > (*hdr).len { return false; }
    if pcap_ptr_va_curr.add(size_of::<pcap_pkthdr>() + (*hdr).caplen as usize).offset_from(pcap_ptr_va_start) as usize >= pcap_map_size { return false; }
    pcap_ptr_va_curr = pcap_ptr_va_curr.add(size_of::<pcap_pkthdr>() + (*hdr).caplen as usize);
    true
}

unsafe fn pcap_reset_pkt() {
    pcap_ptr_va_curr = pcap_ptr_va_start.add(size_of::<pcap_filehdr>());
}

unsafe fn try_load_pcap(file: *const c_char) -> c_int {
    let mut sb: stat = zeroed();
    pcap_fd = open(file, O_RDONLY);
    if pcap_fd < 0 { rl_printf(c!("cannot open pcap [%s]!\n"), strerror(errno)); return CMD_ERR; }
    if fstat(pcap_fd, &mut sb) < 0 { rl_printf(c!("cannot fstat pcap file!\n")); return CMD_ERR; }
    if !s_isreg(sb.st_mode) { rl_printf(c!("not a regular pcap file, duh!\n")); return CMD_ERR; }
    pcap_map_size = sb.st_size as usize;
    if pcap_map_size <= size_of::<pcap_filehdr>() { rl_printf(c!("pcap file too small!\n")); return CMD_ERR; }
    pcap_ptr_va_start = mmap(ptr::null_mut(), pcap_map_size, PROT_READ, MAP_SHARED | MAP_LOCKED, pcap_fd, 0) as *mut c_char;
    if pcap_ptr_va_start == (-1isize as *mut c_char) { rl_printf(c!("mmap of file failed!")); return CMD_ERR; }
    let hdr = pcap_ptr_va_start as *mut pcap_filehdr;
    if (*hdr).magic != TCPDUMP_MAGIC { rl_printf(c!("wrong pcap magic!\n")); return CMD_ERR; }
    pcap_reset_pkt();
    CMD_OK
}

unsafe fn try_close_pcap() {
    if pcap_fd >= 0 {
        munmap(pcap_ptr_va_start as *mut c_void, pcap_map_size);
        close(pcap_fd);
        pcap_ptr_va_start = ptr::null_mut();
        pcap_ptr_va_curr = ptr::null_mut();
        pcap_map_size = 0;
        pcap_packet = 0;
        pcap_fd = -1;
    }
}

unsafe fn cmd_load_bpf(bpf_string: *mut c_char) -> c_int {
    let mut sp: c_char = 0;
    let mut token: *mut c_char;
    let separator = ',' as c_char;
    let mut bpf_len: u16 = 0;
    let mut i: u16 = 0;
    let mut tmp = sock_filter { code: 0, jt: 0, jf: 0, k: 0 };
    bpf_prog_len = 0;
    memset(&raw mut bpf_image as *mut c_void, 0, size_of::<[sock_filter; BPF_MAXINSNS + 1]>());
    if sscanf(bpf_string, c!("%hu%c"), &mut bpf_len, &mut sp) != 2 || sp != separator || bpf_len as usize > BPF_MAXINSNS || bpf_len == 0 {
        rl_printf(c!("syntax error in head length encoding!\n"));
        return CMD_ERR;
    }
    token = bpf_string;
    while { token = strchr(token, separator as c_int); !token.is_null() && { token = token.add(1); *token != 0 } } {
        if i >= bpf_len { rl_printf(c!("program exceeds encoded length!\n")); return CMD_ERR; }
        if sscanf(token, c!("%hu %hhu %hhu %u,"), &mut tmp.code, &mut tmp.jt, &mut tmp.jf, &mut tmp.k) != 4 {
            rl_printf(c!("syntax error at instruction %d!\n"), i as c_int);
            return CMD_ERR;
        }
        bpf_image[i as usize] = tmp;
        i += 1;
    }
    if i != bpf_len { rl_printf(c!("syntax error exceeding encoded length!\n")); return CMD_ERR; }
    else { bpf_prog_len = bpf_len as c_uint; }
    if !bpf_runnable((&raw mut bpf_image).cast::<sock_filter>(), bpf_prog_len) { bpf_prog_len = 0; }
    CMD_OK
}

unsafe fn cmd_load_pcap(file: *mut c_char) -> c_int {
    let mut tmp: *mut c_char = ptr::null_mut();
    let file_trim = strtok_r(file, c!(" "), &mut tmp);
    if file_trim.is_null() { return CMD_ERR; }
    try_close_pcap();
    try_load_pcap(file_trim)
}

unsafe fn cmd_load(arg: *mut c_char) -> c_int {
    let mut cont: *mut c_char = ptr::null_mut();
    let tmp = strdup(arg);
    let mut ret = CMD_OK;
    let subcmd = strtok_r(tmp, c!(" "), &mut cont);
    if subcmd.is_null() { rl_printf(c!("bpf <code>:  load bpf code\n")); rl_printf(c!("pcap <file>: load pcap file\n")); ret = CMD_ERR; }
    else if matches(subcmd, c!("bpf")) == 0 {
        bpf_reset();
        bpf_reset_breakpoints();
        if cont.is_null() { ret = CMD_ERR; } else { ret = cmd_load_bpf(cont); }
    } else if matches(subcmd, c!("pcap")) == 0 {
        ret = cmd_load_pcap(cont);
    } else {
        rl_printf(c!("bpf <code>:  load bpf code\n"));
        rl_printf(c!("pcap <file>: load pcap file\n"));
        ret = CMD_ERR;
    }
    free(tmp as *mut c_void);
    ret
}

unsafe fn cmd_step(num: *mut c_char) -> c_int {
    if !bpf_prog_loaded() || !pcap_loaded() { return CMD_ERR; }
    let mut steps = strtol(num, ptr::null_mut(), 10) as c_int;
    if steps == 0 || strlen(num) == 0 { steps = 1; }
    if steps < 0 {
        if !bpf_restore_regs(steps) { return CMD_ERR; }
        steps = 1;
    }
    let hdr = pcap_curr_pkt();
    let ret = bpf_run_stepping((&raw mut bpf_image).cast::<sock_filter>(), bpf_prog_len as u16, (hdr as *mut u8).add(size_of::<pcap_pkthdr>()), (*hdr).caplen, (*hdr).len, steps);
    if ret >= 0 || bpf_curr.Rs {
        bpf_reset();
        if !pcap_next_pkt() { rl_printf(c!("(going back to first packet)\n")); pcap_reset_pkt(); }
        else { rl_printf(c!("(next packet)\n")); }
    }
    CMD_OK
}

unsafe fn cmd_select(num: *mut c_char) -> c_int {
    if !pcap_loaded() || strlen(num) == 0 { return CMD_ERR; }
    let mut which = strtoul(num, ptr::null_mut(), 10) as c_uint;
    if which == 0 { rl_printf(c!("packet count starts with 1, clamping!\n")); which = 1; }
    pcap_reset_pkt();
    bpf_reset();
    let mut i: c_uint = 0;
    let mut have_next = true;
    while i < which {
        have_next = pcap_next_pkt();
        if !have_next { break; }
        i += 1;
    }
    if !have_next || pcap_curr_pkt().is_null() {
        rl_printf(c!("no packet #%u available!\n"), which);
        pcap_reset_pkt();
        return CMD_ERR;
    }
    CMD_OK
}

unsafe fn cmd_breakpoint(subcmd: *mut c_char) -> c_int {
    if !bpf_prog_loaded() { return CMD_ERR; }
    if strlen(subcmd) == 0 { bpf_dump_breakpoints(); }
    else if matches(subcmd, c!("reset")) == 0 { bpf_reset_breakpoints(); }
    else {
        let where_ = strtoul(subcmd, ptr::null_mut(), 10) as c_uint;
        if where_ < bpf_prog_len {
            bpf_set_breakpoints(where_);
            rl_printf(c!("breakpoint at: "));
            bpf_disasm(bpf_image[where_ as usize], where_);
        }
    }
    CMD_OK
}

unsafe fn cmd_run(num: *mut c_char) -> c_int {
    static mut pass: u32 = 0;
    static mut fail: u32 = 0;
    let mut has_limit = true;
    let mut pkts: c_int;
    let mut i: c_int = 0;
    if !bpf_prog_loaded() || !pcap_loaded() { return CMD_ERR; }
    pkts = strtol(num, ptr::null_mut(), 10) as c_int;
    if pkts == 0 || strlen(num) == 0 { has_limit = false; }
    loop {
        let hdr = pcap_curr_pkt();
        let ret = bpf_run_all((&raw mut bpf_image).cast::<sock_filter>(), bpf_prog_len as u16, (hdr as *mut u8).add(size_of::<pcap_pkthdr>()), (*hdr).caplen, (*hdr).len);
        if ret > 0 { pass = pass.wrapping_add(1); }
        else if ret == 0 { fail = fail.wrapping_add(1); }
        else { return CMD_OK; }
        bpf_reset();
        if !(pcap_next_pkt() && (!has_limit || { i += 1; i < pkts })) { break; }
    }
    rl_printf(c!("bpf passes:%u fails:%u\n"), pass, fail);
    pcap_reset_pkt();
    bpf_reset();
    pass = 0;
    fail = 0;
    CMD_OK
}

unsafe fn cmd_disassemble(line_string: *mut c_char) -> c_int {
    let mut single_line = false;
    let mut line: c_ulong = 0;
    if !bpf_prog_loaded() { return CMD_ERR; }
    if strlen(line_string) > 0 {
        line = strtoul(line_string, ptr::null_mut(), 10);
        if line < bpf_prog_len as c_ulong { single_line = true; }
    }
    if single_line { bpf_disasm(bpf_image[line as usize], line as c_uint); }
    else { bpf_disasm_all((&raw const bpf_image).cast::<sock_filter>(), bpf_prog_len); }
    CMD_OK
}

unsafe fn cmd_dump(_dontcare: *mut c_char) -> c_int {
    if !bpf_prog_loaded() { return CMD_ERR; }
    bpf_dump_all((&raw const bpf_image).cast::<sock_filter>(), bpf_prog_len);
    CMD_OK
}

unsafe fn cmd_quit(_dontcare: *mut c_char) -> c_int { CMD_EX }

static cmds: [shell_cmd; 8] = [
    shell_cmd { name: c!("load"), func: cmd_load },
    shell_cmd { name: c!("select"), func: cmd_select },
    shell_cmd { name: c!("step"), func: cmd_step },
    shell_cmd { name: c!("run"), func: cmd_run },
    shell_cmd { name: c!("breakpoint"), func: cmd_breakpoint },
    shell_cmd { name: c!("disassemble"), func: cmd_disassemble },
    shell_cmd { name: c!("dump"), func: cmd_dump },
    shell_cmd { name: c!("quit"), func: cmd_quit },
];

unsafe fn execf(arg: *mut c_char) -> c_int {
    let mut cont: *mut c_char = ptr::null_mut();
    let tmp = strdup(arg);
    let mut ret = 0;
    let cmd = strtok_r(tmp, c!(" "), &mut cont);
    if !cmd.is_null() {
        let len = strlen(cmd);
        let mut i = 0;
        while i < cmds.len() {
            if len == strlen(cmds[i].name) && strncmp(cmds[i].name, cmd, len) == 0 {
                ret = (cmds[i].func)(cont);
                break;
            }
            i += 1;
        }
    }
    free(tmp as *mut c_void);
    ret
}

unsafe extern "C" fn shell_comp_gen(buf: *const c_char, state: c_int) -> *mut c_char {
    static mut list_index: c_int = 0;
    static mut len: c_int = 0;
    if state == 0 {
        list_index = 0;
        len = strlen(buf) as c_int;
    }
    while (list_index as usize) < cmds.len() {
        let name = cmds[list_index as usize].name;
        list_index += 1;
        if strncmp(name, buf, len as usize) == 0 { return strdup(name); }
    }
    ptr::null_mut()
}

unsafe extern "C" fn shell_completion(buf: *const c_char, start: c_int, _end: c_int) -> *mut *mut c_char {
    let mut matches_: *mut *mut c_char = ptr::null_mut();
    if start == 0 { matches_ = rl_completion_matches(buf, shell_comp_gen); }
    matches_
}

unsafe extern "C" fn intr_shell(_sig: c_int) {
    if rl_end != 0 { rl_kill_line(-1, 0); }
    rl_crlf();
    rl_refresh_line(0, 0);
    rl_free_line_state();
}

unsafe fn init_shell(fin: *mut FILE, fout: *mut FILE) {
    let mut file = [0 as c_char; 128];
    snprintf(file.as_mut_ptr(), file.len(), c!("%s/.bpf_dbg_history"), getenv(c!("HOME")));
    read_history(file.as_ptr());
    rl_instream = fin;
    rl_outstream = fout;
    rl_readline_name = c!("bpf_dbg");
    rl_terminal_name = getenv(c!("TERM"));
    rl_catch_signals = 0;
    rl_catch_sigwinch = 1;
    rl_attempted_completion_function = Some(shell_completion);
    rl_bind_key('\t' as c_int, rl_complete);
    rl_bind_key_in_map('\t' as c_int, rl_complete, emacs_meta_keymap);
    rl_bind_key_in_map('\x1b' as c_int, rl_complete, emacs_meta_keymap);
    snprintf(file.as_mut_ptr(), file.len(), c!("%s/.bpf_dbg_init"), getenv(c!("HOME")));
    rl_read_init_file(file.as_ptr());
    rl_prep_terminal(0);
    rl_set_signals();
    signal(SIGINT, intr_shell);
}

unsafe fn exit_shell(fin: *mut FILE, fout: *mut FILE) {
    let mut file = [0 as c_char; 128];
    snprintf(file.as_mut_ptr(), file.len(), c!("%s/.bpf_dbg_history"), getenv(c!("HOME")));
    write_history(file.as_ptr());
    clear_history();
    rl_deprep_terminal();
    try_close_pcap();
    if fin != stdin { fclose(fin); }
    if fout != stdout { fclose(fout); }
}

unsafe fn run_shell_loop(fin: *mut FILE, fout: *mut FILE) -> c_int {
    init_shell(fin, fout);
    loop {
        let buf = readline(c!("> "));
        if buf.is_null() { break; }
        let ret = execf(buf);
        if ret == CMD_EX { free(buf as *mut c_void); break; }
        if ret == CMD_OK && strlen(buf) > 0 { add_history(buf); }
        free(buf as *mut c_void);
    }
    exit_shell(fin, fout);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut fin: *mut FILE = ptr::null_mut();
    let mut fout: *mut FILE = ptr::null_mut();
    if argc >= 2 { fin = fopen(*argv.add(1), c!("r")); }
    if argc >= 3 { fout = fopen(*argv.add(2), c!("w")); }
    run_shell_loop(if !fin.is_null() { fin } else { stdin }, if !fout.is_null() { fout } else { stdout })
}
