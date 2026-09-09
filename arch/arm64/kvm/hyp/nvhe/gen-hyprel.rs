// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 - Google LLC
 * Author: David Brazdil <dbrazdil@google.com>
 *
 * Generates relocation information used by the kernel to convert
 * absolute addresses in hyp data from kernel VAs to hyp VAs.
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem::size_of;
use std::ptr;

const HYP_SECTION_PREFIX: &[u8] = b".hyp\0";
const HYP_RELOC_SECTION: &str = ".hyp.reloc";
const HYP_SECTION_SYMBOL_PREFIX: &str = "__hyp_section_";

const R_AARCH64_ABS64: u32 = 257;
const R_AARCH64_ABS32: u32 = 258;
const R_AARCH64_PREL64: u32 = 260;
const R_AARCH64_PREL32: u32 = 261;
const R_AARCH64_PREL16: u32 = 262;
const R_AARCH64_PLT32: u32 = 314;
const R_AARCH64_LD_PREL_LO19: u32 = 273;
const R_AARCH64_ADR_PREL_LO21: u32 = 274;
const R_AARCH64_ADR_PREL_PG_HI21: u32 = 275;
const R_AARCH64_ADR_PREL_PG_HI21_NC: u32 = 276;
const R_AARCH64_ADD_ABS_LO12_NC: u32 = 277;
const R_AARCH64_LDST8_ABS_LO12_NC: u32 = 278;
const R_AARCH64_TSTBR14: u32 = 279;
const R_AARCH64_CONDBR19: u32 = 280;
const R_AARCH64_JUMP26: u32 = 282;
const R_AARCH64_CALL26: u32 = 283;
const R_AARCH64_LDST16_ABS_LO12_NC: u32 = 284;
const R_AARCH64_LDST32_ABS_LO12_NC: u32 = 285;
const R_AARCH64_LDST64_ABS_LO12_NC: u32 = 286;
const R_AARCH64_MOVW_PREL_G0: u32 = 287;
const R_AARCH64_MOVW_PREL_G0_NC: u32 = 288;
const R_AARCH64_MOVW_PREL_G1: u32 = 289;
const R_AARCH64_MOVW_PREL_G1_NC: u32 = 290;
const R_AARCH64_MOVW_PREL_G2: u32 = 291;
const R_AARCH64_MOVW_PREL_G2_NC: u32 = 292;
const R_AARCH64_MOVW_PREL_G3: u32 = 293;
const R_AARCH64_LDST128_ABS_LO12_NC: u32 = 299;

#[repr(C)]
struct Elf64_Ehdr { e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32, e_entry: u64, e_phoff: u64, e_shoff: u64, e_flags: u32, e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16, e_shnum: u16, e_shstrndx: u16 }
#[repr(C)]
struct Elf64_Shdr { sh_name: u32, sh_type: u32, sh_flags: u64, sh_addr: u64, sh_offset: u64, sh_size: u64, sh_link: u32, sh_info: u32, sh_addralign: u64, sh_entsize: u64 }
#[repr(C)]
struct Elf64_Rela { r_offset: u64, r_info: u64, r_addend: i64 }

#[repr(C)]
struct ElfState { path: *const c_char, begin: *mut c_char, size: usize, ehdr: *mut Elf64_Ehdr, sh_table: *mut Elf64_Shdr, sh_string: *const c_char }
static mut ELF: ElfState = ElfState { path: ptr::null(), begin: ptr::null_mut(), size: 0, ehdr: ptr::null_mut(), sh_table: ptr::null_mut(), sh_string: ptr::null() };

unsafe extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, st: *mut Stat) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: i64) -> *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(err: c_int) -> *const c_char;
    fn __errno_location() -> *mut c_int;
    fn exit(status: c_int) -> !;
}
#[repr(C)] struct Stat { st_size: i64, _rest: [u8; 256] }
const O_RDONLY: c_int = 0; const PROT_READ: c_int = 1; const MAP_PRIVATE: c_int = 2; const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;
const EXIT_FAILURE: c_int = 1; const EXIT_SUCCESS: c_int = 0;
const EI_MAG0: usize = 0; const EI_MAG1: usize = 1; const EI_MAG2: usize = 2; const EI_MAG3: usize = 3; const EI_CLASS: usize = 4; const EI_DATA: usize = 5;
const ELFCLASS64: u8 = 2; const ELFDATA2LSB: u8 = 1; const ELFDATA2MSB: u8 = 2; const ET_REL: u16 = 1; const EM_AARCH64: u16 = 183; const SHN_UNDEF: u16 = 0; const SHT_REL: u32 = 9; const SHT_RELA: u32 = 4;

unsafe fn fatal_error(message: &str) -> ! { eprintln!("error: {}: {}", CStr::from_ptr(ELF.path).to_string_lossy(), message); exit(EXIT_FAILURE) }
unsafe fn assert_op<T: PartialOrd + std::fmt::Display>(lhs: T, rhs: T, op: &str, ok: bool) { if !ok { fatal_error(&format!("assertion lhs {} rhs failed (lhs={}, rhs={})", op, lhs, rhs)); } }
unsafe fn section_name(shdr: *mut Elf64_Shdr) -> *const c_char { ELF.sh_string.add(u32::from((*shdr).sh_name) as usize) }
unsafe fn section_begin(shdr: *mut Elf64_Shdr) -> *const c_char { ELF.begin.add((*shdr).sh_offset as usize) }
unsafe fn section_by_off(off: u64) -> *mut Elf64_Shdr { assert_op(off, 0, "!=", off != 0); ELF.begin.add(off as usize) as *mut Elf64_Shdr }
unsafe fn section_by_idx(idx: u16) -> *mut Elf64_Shdr { assert_op(idx, SHN_UNDEF, "!=", idx != SHN_UNDEF); ELF.sh_table.add(idx as usize) }
unsafe fn starts_with(s: *const c_char, prefix: &[u8]) -> bool { let a = CStr::from_ptr(s).to_bytes(); a.starts_with(&prefix[..prefix.len()-1]) }

unsafe fn init_elf(path: *const c_char) {
    ELF.path = path;
    let fd = open(path, O_RDONLY);
    if fd < 0 { fatal_error("Could not open ELF file"); }
    let mut stat = Stat { st_size: 0, _rest: [0; 256] };
    if fstat(fd, &mut stat) < 0 { close(fd); fatal_error("Could not get status of ELF file"); }
    ELF.begin = mmap(ptr::null_mut(), stat.st_size as usize, PROT_READ, MAP_PRIVATE, fd, 0) as *mut c_char;
    if ELF.begin == MAP_FAILED { close(fd); fatal_error("Could not mmap ELF file"); }
    close(fd); ELF.size = stat.st_size as usize;
    assert_op(ELF.size, size_of::<Elf64_Ehdr>(), ">=", ELF.size >= size_of::<Elf64_Ehdr>());
    ELF.ehdr = ELF.begin as *mut Elf64_Ehdr;
    assert_op((*ELF.ehdr).e_ident[EI_MAG0], 0x7f, "==", (*ELF.ehdr).e_ident[EI_MAG0] == 0x7f);
    assert_op((*ELF.ehdr).e_ident[EI_MAG1], b'E', "==", (*ELF.ehdr).e_ident[EI_MAG1] == b'E'); assert_op((*ELF.ehdr).e_ident[EI_MAG2], b'L', "==", (*ELF.ehdr).e_ident[EI_MAG2] == b'L'); assert_op((*ELF.ehdr).e_ident[EI_MAG3], b'F', "==", (*ELF.ehdr).e_ident[EI_MAG3] == b'F');
    assert_op((*ELF.ehdr).e_ident[EI_CLASS], ELFCLASS64, "==", (*ELF.ehdr).e_ident[EI_CLASS] == ELFCLASS64);
    // The build selects CONFIG_CPU_LITTLE_ENDIAN or CONFIG_CPU_BIG_ENDIAN.
    let endian = if cfg!(target_endian = "little") { ELFDATA2LSB } else { ELFDATA2MSB };
    assert_op((*ELF.ehdr).e_ident[EI_DATA], endian, "==", (*ELF.ehdr).e_ident[EI_DATA] == endian);
    assert_op((*ELF.ehdr).e_type, ET_REL, "==", (*ELF.ehdr).e_type == ET_REL); assert_op((*ELF.ehdr).e_machine, EM_AARCH64, "==", (*ELF.ehdr).e_machine == EM_AARCH64);
    ELF.sh_table = section_by_off((*ELF.ehdr).e_shoff); ELF.sh_string = section_begin(section_by_idx((*ELF.ehdr).e_shstrndx));
}

unsafe fn emit_prologue() { println!(".data\n.pushsection {}, \"a\"", HYP_RELOC_SECTION); }
unsafe fn emit_section_prologue(name: *const c_char) { println!(".global {}{}", HYP_SECTION_SYMBOL_PREFIX, CStr::from_ptr(name).to_string_lossy()); }
unsafe fn emit_rela_abs64(rela: *mut Elf64_Rela, name: *const c_char) { static mut RELOC_OFFSET: usize = 0; println!(".word 0"); println!(".reloc {}, R_AARCH64_PREL32, {}{} + 0x{:x}", RELOC_OFFSET, HYP_SECTION_SYMBOL_PREFIX, CStr::from_ptr(name).to_string_lossy(), (*rela).r_offset); RELOC_OFFSET += 4; }

unsafe fn emit_rela_section(sh_rela: *mut Elf64_Shdr) {
    let orig = ELF.sh_table.add((*sh_rela).sh_info as usize); let name = section_name(orig); if !starts_with(name, HYP_SECTION_PREFIX) { return; } emit_section_prologue(name);
    let count = ((*sh_rela).sh_size / size_of::<Elf64_Rela>() as u64) as usize; let relas = ELF.begin.add((*sh_rela).sh_offset as usize) as *mut Elf64_Rela;
    for i in 0..count { let rela = relas.add(i); assert_op((*rela).r_offset, (*orig).sh_size, "<", (*rela).r_offset < (*orig).sh_size); let typ = (*rela).r_info as u32;
        match typ { R_AARCH64_ABS64 => emit_rela_abs64(rela, name), R_AARCH64_ABS32 | R_AARCH64_PREL64 | R_AARCH64_PREL32 | R_AARCH64_PREL16 | R_AARCH64_PLT32 | R_AARCH64_LD_PREL_LO19 | R_AARCH64_ADR_PREL_LO21 | R_AARCH64_ADR_PREL_PG_HI21 | R_AARCH64_ADR_PREL_PG_HI21_NC | R_AARCH64_ADD_ABS_LO12_NC | R_AARCH64_LDST8_ABS_LO12_NC | R_AARCH64_LDST16_ABS_LO12_NC | R_AARCH64_LDST32_ABS_LO12_NC | R_AARCH64_LDST64_ABS_LO12_NC | R_AARCH64_LDST128_ABS_LO12_NC | R_AARCH64_TSTBR14 | R_AARCH64_CONDBR19 | R_AARCH64_JUMP26 | R_AARCH64_CALL26 | R_AARCH64_MOVW_PREL_G0 | R_AARCH64_MOVW_PREL_G0_NC | R_AARCH64_MOVW_PREL_G1 | R_AARCH64_MOVW_PREL_G1_NC | R_AARCH64_MOVW_PREL_G2 | R_AARCH64_MOVW_PREL_G2_NC | R_AARCH64_MOVW_PREL_G3 => (), _ => fatal_error(&format!("Unexpected RELA type {}", typ)) }
    }
}
unsafe fn emit_all_relocs() { for i in 0..(*ELF.ehdr).e_shnum as usize { let shdr = ELF.sh_table.add(i); match (*shdr).sh_type { SHT_REL => fatal_error(&format!("Unexpected SHT_REL section \"{}\"", CStr::from_ptr(section_name(shdr)).to_string_lossy())), SHT_RELA => emit_rela_section(shdr), _ => () } } }

fn main() { unsafe { let args: Vec<_> = std::env::args().collect(); if args.len() != 2 { eprintln!("Usage: {} <elf_input>", args[0]); std::process::exit(EXIT_FAILURE); } let path = std::ffi::CString::new(args[1].as_bytes()).unwrap(); init_elf(path.as_ptr()); emit_prologue(); emit_all_relocs(); println!(".popsection"); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
