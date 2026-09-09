// SPDX-License-Identifier: GPL-2.0-only

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem;
use std::ptr;

const OP_SPECIAL: u32 = 0x00;
const OP_REGIMM: u32 = 0x01;
const OP_BEQ: u32 = 0x04;
const OP_BNE: u32 = 0x05;
const OP_BLEZ: u32 = 0x06;
const OP_BGTZ: u32 = 0x07;
const OP_BEQL: u32 = 0x14;
const OP_BNEL: u32 = 0x15;
const OP_BLEZL: u32 = 0x16;
const OP_BGTZL: u32 = 0x17;
const OP_LL: u32 = 0x30;
const OP_LLD: u32 = 0x34;
const OP_SC: u32 = 0x38;
const OP_SCD: u32 = 0x3c;

const REGIMM_BLTZ: u32 = 0x00;
const REGIMM_BGEZ: u32 = 0x01;
const REGIMM_BLTZL: u32 = 0x02;
const REGIMM_BGEZL: u32 = 0x03;
const REGIMM_BLTZAL: u32 = 0x10;
const REGIMM_BGEZAL: u32 = 0x11;
const REGIMM_BLTZALL: u32 = 0x12;
const REGIMM_BGEZALL: u32 = 0x13;

const SPECIAL_SYNC: u32 = 0x0f;
const EINVAL: c_int = 22;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 1;
const MAP_PRIVATE: c_int = 2;
const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;
const SHT_PROGBITS: u32 = 1;
const SHF_EXECINSTR: u64 = 4;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const SELFMAG: usize = 4;
const ELFMAG: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[repr(C)]
struct Elf64Ehdr {
    e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32,
    e_entry: u64, e_phoff: u64, e_shoff: u64, e_flags: u32,
    e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16,
    e_shnum: u16, e_shstrndx: u16,
}

#[repr(C)]
struct Elf64Shdr {
    sh_name: u32, sh_type: u32, sh_flags: u64, sh_addr: u64,
    sh_offset: u64, sh_size: u64, sh_link: u32, sh_info: u32,
    sh_addralign: u64, sh_entsize: u64,
}

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, buf: *mut Stat) -> c_int;
    fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: i64) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn close(fd: c_int) -> c_int;
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;
}

#[repr(C)]
struct Stat { st_dev: u64, st_ino: u64, st_nlink: u64, st_mode: u32, _pad: u32, st_uid: u32, st_gid: u32, _pad2: u32, st_rdev: u64, st_size: i64, st_blksize: i64, st_blocks: i64, _rest: [u8; 48] }

unsafe fn usage(f: *mut c_void) { fprintf(f, b"Usage: loongson3-llsc-check /path/to/vmlinux\n\0".as_ptr() as *const c_char); }
fn se16(x: u16) -> i32 { (x as i16) as i32 }
fn is_ll(insn: u32) -> bool { matches!(insn >> 26, OP_LL | OP_LLD) }
fn is_sc(insn: u32) -> bool { matches!(insn >> 26, OP_SC | OP_SCD) }
fn is_sync(insn: u32) -> bool { (insn >> 11) == 0 && (insn & 0x3f) == SPECIAL_SYNC }

fn is_branch(insn: u32, off: &mut i32) -> bool {
    match insn >> 26 {
        OP_BEQ | OP_BEQL | OP_BNE | OP_BNEL | OP_BGTZ | OP_BGTZL | OP_BLEZ | OP_BLEZL => { *off = se16(insn as u16) + 1; true }
        OP_REGIMM => match (insn >> 16) & 0x1f {
            REGIMM_BGEZ | REGIMM_BGEZL | REGIMM_BGEZAL | REGIMM_BGEZALL | REGIMM_BLTZ | REGIMM_BLTZL | REGIMM_BLTZAL | REGIMM_BLTZALL => { *off = se16(insn as u16) + 1; true }
            _ => false,
        },
        _ => false,
    }
}

unsafe fn check_ll(pc: u64, code: *mut u32, sz: usize) -> c_int {
    if !is_sync(u32::from_le(ptr::read(code.offset(-1)))) { fprintf(stderr, b"%llx: LL not preceded by sync\n\0".as_ptr() as *const c_char, pc); return -EINVAL; }
    let max = sz / 4; let mut sc_pos = 0;
    while sc_pos < max && !is_sc(u32::from_le(ptr::read(code.add(sc_pos)))) { sc_pos += 1; }
    if sc_pos >= max { fprintf(stderr, b"%llx: LL has no matching SC\n\0".as_ptr() as *const c_char, pc); return -EINVAL; }
    let mut i = 0; while i < sc_pos { let mut off = 0; if is_branch(u32::from_le(ptr::read(code.add(i))), &mut off) && !((off >= -(i as i32)) && (off <= sc_pos as i32)) && !is_sync(u32::from_le(ptr::read(code.offset((i as i32) + off)))) { fprintf(stderr, b"%llx: Branch target not a sync\n\0".as_ptr() as *const c_char, pc + (i * 4) as u64); return -EINVAL; } i += 1; }
    0
}

unsafe fn check_code(mut pc: u64, mut code: *mut u32, mut sz: usize) -> c_int {
    let mut err = 0; if sz % 4 != 0 { fprintf(stderr, b"%llx: Section size not a multiple of 4\n\0".as_ptr() as *const c_char, pc); err = -EINVAL; sz -= sz % 4; }
    if is_ll(u32::from_le(ptr::read(code))) { fprintf(stderr, b"%llx: First instruction in section is an LL\n\0".as_ptr() as *const c_char, pc); err = -EINVAL; }
    code = code.add(1); pc += 4; sz -= 4;
    while sz != 0 { if is_ll(u32::from_le(ptr::read(code))) { err |= check_ll(pc, code, sz); } code = code.add(1); pc += 4; sz -= 4; }
    err
}

unsafe fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut status = EXIT_FAILURE;
    if args.len() < 2 { usage(stderr); }
    else {
        let path = std::ffi::CString::new(args[1].as_bytes()).unwrap();
        let fd = open(path.as_ptr(), O_RDONLY);
        if fd == -1 { perror(b"Unable to open vmlinux\0".as_ptr() as *const c_char); }
        else {
            let mut st: Stat = mem::zeroed();
            if fstat(fd, &mut st) != 0 { perror(b"Unable to stat vmlinux\0".as_ptr() as *const c_char); }
            else {
                let image = mmap(ptr::null_mut(), st.st_size as usize, PROT_READ, MAP_PRIVATE, fd, 0);
                if image == MAP_FAILED { perror(b"Unable to mmap vmlinux\0".as_ptr() as *const c_char); }
                else {
                    let eh = image as *mut Elf64Ehdr;
                    let ident = (*eh).e_ident;
                    if ident[..SELFMAG] != ELFMAG { fprintf(stderr, b"vmlinux is not an ELF?\n\0".as_ptr() as *const c_char); }
                    else if ident[EI_CLASS] != ELFCLASS64 { fprintf(stderr, b"vmlinux is not 64b?\n\0".as_ptr() as *const c_char); }
                    else if ident[EI_DATA] != ELFDATA2LSB { fprintf(stderr, b"vmlinux is not little endian?\n\0".as_ptr() as *const c_char); }
                    else {
                        let mut err = 0;
                        for i in 0..u16::from_le((*eh).e_shnum) {
                            let sh = (image as *mut u8).add(u64::from_le((*eh).e_shoff) as usize + i as usize * u16::from_le((*eh).e_shentsize) as usize) as *mut Elf64Shdr;
                            if u32::from_le((*sh).sh_type) != SHT_PROGBITS || u64::from_le((*sh).sh_flags) & SHF_EXECINSTR == 0 { continue; }
                            err = check_code(u64::from_le((*sh).sh_addr), (image as *mut u8).add(u64::from_le((*sh).sh_offset) as usize) as *mut u32, u64::from_le((*sh).sh_size) as usize);
                            if err != 0 { break; }
                        }
                        if err == 0 { status = EXIT_SUCCESS; }
                    }
                    munmap(image, st.st_size as usize);
                }
            }
            close(fd);
        }
    }
    let word = if status != 0 { b"failure\n\0" } else { b"success\n\0" };
    fprintf(stdout, b"loongson3-llsc-check returns %s\0".as_ptr() as *const c_char, word.as_ptr() as *const c_char);
    std::process::exit(status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
