// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Alex Smith <alex.smith@imgtec.com>
 */

/* Rust translation of genvdso.c. The generated output remains C source. */

use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

const SHT_GNU_ATTRIBUTES: u32 = 0x6ffffff5;
const SHT_MIPS_ABIFLAGS: u32 = 0x7000002a;

const ABI_O32: u32 = 1 << 0;
const ABI_N32: u32 = 1 << 1;
const ABI_N64: u32 = 1 << 2;
const ABI_ALL: u32 = ABI_O32 | ABI_N32 | ABI_N64;

#[repr(C)]
struct VdsoSymbol {
    name: *const c_char,
    offset_name: *const c_char,
    abis: u32,
}

static mut VDSO_SYMBOLS: [VdsoSymbol; 3] = [
    VdsoSymbol { name: b"__vdso_sigreturn\0".as_ptr() as *const c_char, offset_name: b"off_sigreturn\0".as_ptr() as *const c_char, abis: ABI_O32 },
    VdsoSymbol { name: b"__vdso_rt_sigreturn\0".as_ptr() as *const c_char, offset_name: b"off_rt_sigreturn\0".as_ptr() as *const c_char, abis: ABI_ALL },
    VdsoSymbol { name: ptr::null(), offset_name: ptr::null(), abis: 0 },
];

static mut PROGRAM_NAME: *const c_char = ptr::null();
static mut VDSO_NAME: *const c_char = ptr::null();
static mut ELF_CLASS: u8 = 0;
static mut ELF_ABI: u32 = 0;
static mut NEED_SWAP: bool = false;
static mut OUT_FILE: *mut c_void = ptr::null_mut();

fn swap_uint16(val: u16) -> u16 { unsafe { if NEED_SWAP { val.swap_bytes() } else { val } } }
fn swap_uint32(val: u32) -> u32 { unsafe { if NEED_SWAP { val.swap_bytes() } else { val } } }
fn swap_uint64(val: u64) -> u64 { unsafe { if NEED_SWAP { val.swap_bytes() } else { val } } }

#[repr(C)]
struct Elf32Ehdr {
    e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32,
    e_entry: u32, e_phoff: u32, e_shoff: u32, e_flags: u32,
    e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16,
    e_shnum: u16, e_shstrndx: u16,
}

extern "C" {
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, stat: *mut libc::stat) -> c_int;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: libc::off_t) -> *mut c_void;
    fn msync(addr: *mut c_void, len: usize, flags: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn patch_vdso64(path: *const c_char, vdso: *mut c_void) -> bool;
    fn patch_vdso32(path: *const c_char, vdso: *mut c_void) -> bool;
    fn get_symbols64(path: *const c_char, vdso: *mut c_void) -> bool;
    fn get_symbols32(path: *const c_char, vdso: *mut c_void) -> bool;
}

unsafe fn map_vdso(path: *const c_char, size: &mut usize) -> *mut c_void {
    let fd = open(path, libc::O_RDWR);
    if fd < 0 { return ptr::null_mut(); }
    let mut st = std::mem::zeroed::<libc::stat>();
    if fstat(fd, &mut st) != 0 { close(fd); return ptr::null_mut(); }
    let addr = mmap(ptr::null_mut(), st.st_size as usize, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
    if addr == libc::MAP_FAILED { close(fd); return ptr::null_mut(); }
    let ehdr = addr as *const Elf32Ehdr;
    if (*ehdr).e_ident[0..4] != *b"\x7fELF" { close(fd); return ptr::null_mut(); }
    ELF_CLASS = (*ehdr).e_ident[4];
    if ELF_CLASS != libc::ELFCLASS32 as u8 && ELF_CLASS != libc::ELFCLASS64 as u8 { close(fd); return ptr::null_mut(); }
    match (*ehdr).e_ident[5] { libc::ELFDATA2LSB | libc::ELFDATA2MSB => NEED_SWAP = (*ehdr).e_ident[5] != if cfg!(target_endian = "little") { libc::ELFDATA2LSB as u8 } else { libc::ELFDATA2MSB as u8 }, _ => { close(fd); return ptr::null_mut(); } }
    if swap_uint16((*ehdr).e_machine) != libc::EM_MIPS || swap_uint16((*ehdr).e_type) != libc::ET_DYN { close(fd); return ptr::null_mut(); }
    *size = st.st_size as usize; close(fd); addr
}

unsafe fn patch_vdso(path: *const c_char, vdso: *mut c_void) -> bool { if ELF_CLASS == libc::ELFCLASS64 as u8 { patch_vdso64(path, vdso) } else { patch_vdso32(path, vdso) } }
unsafe fn get_symbols(path: *const c_char, vdso: *mut c_void) -> bool { if ELF_CLASS == libc::ELFCLASS64 as u8 { get_symbols64(path, vdso) } else { get_symbols32(path, vdso) } }

fn main() {
    let args: Vec<CString> = std::env::args().map(|s| CString::new(s).unwrap()).collect();
    let argc = args.len();
    unsafe {
        PROGRAM_NAME = args[0].as_ptr();
        if argc < 4 || argc > 5 { return; }
        VDSO_NAME = if argc > 4 { args[4].as_ptr() } else { b"\0".as_ptr() as *const c_char };
        let mut dbg_size = 0usize;
        let mut vdso_size = 0usize;
        let dbg = map_vdso(args[1].as_ptr(), &mut dbg_size);
        if dbg.is_null() { return; }
        let vdso = map_vdso(args[2].as_ptr(), &mut vdso_size);
        if vdso.is_null() { return; }
        if !patch_vdso(args[1].as_ptr(), dbg) || !patch_vdso(args[2].as_ptr(), vdso) { return; }
        if msync(dbg, dbg_size, libc::MS_SYNC) != 0 || msync(vdso, vdso_size, libc::MS_SYNC) != 0 { return; }

        let mut out = match std::fs::File::create(CStr::from_ptr(args[3].as_ptr()).to_string_lossy().as_ref()) { Ok(f) => f, Err(_) => return };
        writeln!(out, "/* Automatically generated - do not edit */").unwrap();
        writeln!(out, "#include <linux/linkage.h>\n#include <linux/mm.h>\n#include <asm/vdso.h>").unwrap();
        writeln!(out, "static int vdso_mremap(const struct vm_special_mapping *sm, struct vm_area_struct *new_vma) {{\n\tcurrent->mm->context.vdso = (void *)(new_vma->vm_start);\n\treturn 0;\n}}").unwrap();
        writeln!(out, "static unsigned char vdso_image_data[PAGE_ALIGN({})] __page_aligned_data = {{", vdso_size).unwrap();
        for i in 0..vdso_size { if i % 10 == 0 { write!(out, "\n\t").unwrap(); } write!(out, "0x{:02x}, ", *(vdso as *const u8).add(i)).unwrap(); }
        writeln!(out, "\n}};\nstatic struct page *vdso_pages[PAGE_ALIGN({}) / PAGE_SIZE];", vdso_size).unwrap();
        let name = CStr::from_ptr(VDSO_NAME).to_string_lossy();
        writeln!(out, "struct mips_vdso_image vdso_image{}{} = {{", if name.is_empty() { "" } else { "_" }, name).unwrap();
        writeln!(out, "\t.data = vdso_image_data,\n\t.size = PAGE_ALIGN({}),\n\t.mapping = {{\n\t\t.name = \"[vdso]\",\n\t\t.pages = vdso_pages,\n\t\t.mremap = vdso_mremap,\n\t}},", vdso_size).unwrap();
        if !get_symbols(args[1].as_ptr(), dbg) { let _ = std::fs::remove_file(CStr::from_ptr(args[3].as_ptr()).to_string_lossy().as_ref()); return; }
        writeln!(out, "}};").unwrap();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
