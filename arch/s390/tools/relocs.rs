// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int, c_void, CStr};
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::os::raw::c_ulong;
use std::ptr;

// C dependencies: stdio, stdarg, stdlib, stdint, inttypes, string, errno,
// unistd, elf, byteswap, and endian provide the corresponding declarations.

const ELF_BITS: usize = 64;
const ELF_MACHINE: u16 = 22; // EM_S390
const ELF_CLASS: u8 = 2; // ELFCLASS64
const ELF_ENDIAN: u8 = 2; // ELFDATA2MSB
const SHT_REL_TYPE: u32 = 4; // SHT_RELA
const FMT: &str = "{}";

type Elf_Addr = u64;
type Elf32_Word = u32;

#[repr(C)]
#[derive(Clone, Copy)]
struct Elf_Ehdr {
    e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32,
    e_entry: u64, e_phoff: u64, e_shoff: u64, e_flags: u32,
    e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16,
    e_shnum: u16, e_shstrndx: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Elf_Phdr { _data: [u8; 56] }
#[repr(C)]
#[derive(Clone, Copy)]
struct Elf_Shdr {
    sh_name: u32, sh_type: u32, sh_flags: u64, sh_addr: u64,
    sh_offset: u64, sh_size: u64, sh_link: u32, sh_info: u32,
    sh_addralign: u64, sh_entsize: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Elf_Sym {
    st_name: u32, st_info: u8, st_other: u8, st_shndx: u16,
    st_value: u64, st_size: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Elf_Rel { r_offset: u64, r_info: u64, r_addend: i64 }

static mut EHDR: Elf_Ehdr = Elf_Ehdr { e_ident: [0; 16], e_type: 0, e_machine: 0, e_version: 0, e_entry: 0, e_phoff: 0, e_shoff: 0, e_flags: 0, e_ehsize: 0, e_phentsize: 0, e_phnum: 0, e_shentsize: 0, e_shnum: 0, e_shstrndx: 0 };
static mut SHNUM: usize = 0;
static mut SHSTRNDX: usize = 0;
static mut SHSYMTABNDX: usize = 0;
static mut SHXSYMTABNDX: usize = 0;

#[repr(C)]
struct Relocs { offset: *mut u32, count: usize, size: usize }
static mut RELOCS64: Relocs = Relocs { offset: ptr::null_mut(), count: 0, size: 0 };
#[repr(C)]
struct Section {
    shdr: Elf_Shdr, link: *mut Section, symtab: *mut Elf_Sym,
    xsymtab: *mut Elf32_Word, reltab: *mut Elf_Rel, strtab: *mut c_char,
}
static mut SECS: *mut Section = ptr::null_mut();

unsafe fn sec_name(shndx: usize) -> *const c_char {
    let mut name = b"<noname>\0".as_ptr() as *const c_char;
    let sec_strtab = (*SECS.add(SHSTRNDX)).strtab;
    if shndx < SHNUM { name = sec_strtab.add((*SECS.add(shndx)).shdr.sh_name as usize); }
    else if shndx == 0xfff1 { name = b"ABSOLUTE\0".as_ptr() as *const c_char; }
    else if shndx == 0xfff2 { name = b"COMMON\0".as_ptr() as *const c_char; }
    name
}
unsafe fn sym_name(sym_strtab: *const c_char, sym: *mut Elf_Sym) -> *const c_char {
    if (*sym).st_name != 0 { sym_strtab.add((*sym).st_name as usize) } else { sec_name(sym_index(sym) as usize) }
}
unsafe fn elf16_to_cpu(v: u16) -> u16 { if EHDR.e_ident[5] == 1 { v.swap_bytes() } else { v } }
unsafe fn elf32_to_cpu(v: u32) -> u32 { if EHDR.e_ident[5] == 1 { v.swap_bytes() } else { v } }
unsafe fn elf64_to_cpu(v: u64) -> u64 { v.swap_bytes() }
unsafe fn sym_index(sym: *mut Elf_Sym) -> u16 {
    if (*sym).st_shndx != 0xffff { return (*sym).st_shndx; }
    let symtab = (*SECS.add(SHSYMTABNDX)).symtab;
    let xsymtab = (*SECS.add(SHXSYMTABNDX)).xsymtab;
    let index = (sym as usize - symtab as usize) / size_of::<Elf_Sym>();
    elf32_to_cpu(*xsymtab.add(index)) as u16
}
unsafe fn die(msg: &str) -> ! { eprint!("{}", msg); std::process::exit(1) }

unsafe fn read_ehdr(fp: &mut std::fs::File) {
    let p = &mut EHDR as *mut Elf_Ehdr as *mut u8;
    if fp.read_exact(std::slice::from_raw_parts_mut(p, size_of::<Elf_Ehdr>())).is_err() { die("Cannot read ELF header\n"); }
    if &EHDR.e_ident[0..4] != b"\x7fELF" { die("No ELF magic\n"); }
    if EHDR.e_ident[4] != ELF_CLASS { die("Not a 64 bit executable\n"); }
    if EHDR.e_ident[5] != ELF_ENDIAN { die("ELF endian mismatch\n"); }
    if EHDR.e_ident[6] != 1 { die("Unknown ELF version\n"); }
    EHDR.e_type=elf16_to_cpu(EHDR.e_type); EHDR.e_machine=elf16_to_cpu(EHDR.e_machine); EHDR.e_version=elf32_to_cpu(EHDR.e_version);
    EHDR.e_entry=elf64_to_cpu(EHDR.e_entry); EHDR.e_phoff=elf64_to_cpu(EHDR.e_phoff); EHDR.e_shoff=elf64_to_cpu(EHDR.e_shoff); EHDR.e_flags=elf32_to_cpu(EHDR.e_flags);
    EHDR.e_ehsize=elf16_to_cpu(EHDR.e_ehsize); EHDR.e_phentsize=elf16_to_cpu(EHDR.e_phentsize); EHDR.e_phnum=elf16_to_cpu(EHDR.e_phnum); EHDR.e_shentsize=elf16_to_cpu(EHDR.e_shentsize); EHDR.e_shnum=elf16_to_cpu(EHDR.e_shnum); EHDR.e_shstrndx=elf16_to_cpu(EHDR.e_shstrndx);
    SHNUM=EHDR.e_shnum as usize; SHSTRNDX=EHDR.e_shstrndx as usize;
    if EHDR.e_type != 2 && EHDR.e_type != 3 { die("Unsupported ELF header type\n"); }
    if EHDR.e_machine != ELF_MACHINE { die("Not for IBM S/390\n"); }
    if EHDR.e_version != 1 { die("Unknown ELF version\n"); }
    if SHSTRNDX >= SHNUM { die("String table index out of bounds\n"); }
}

unsafe fn read_shdrs(fp: &mut std::fs::File) {
    let layout = std::alloc::Layout::array::<Section>(SHNUM).unwrap(); SECS=std::alloc::alloc_zeroed(layout) as *mut Section;
    if SECS.is_null() { die("Unable to allocate section headers\n"); }
    fp.seek(SeekFrom::Start(EHDR.e_shoff)).unwrap();
    for i in 0..SHNUM { let sec=&mut *SECS.add(i); let p=&mut sec.shdr as *mut Elf_Shdr as *mut u8; if fp.read_exact(std::slice::from_raw_parts_mut(p,size_of::<Elf_Shdr>())).is_err(){die("Cannot read ELF section headers\n");} sec.shdr.sh_name=elf32_to_cpu(sec.shdr.sh_name); sec.shdr.sh_type=elf32_to_cpu(sec.shdr.sh_type); sec.shdr.sh_flags=elf64_to_cpu(sec.shdr.sh_flags); sec.shdr.sh_addr=elf64_to_cpu(sec.shdr.sh_addr); sec.shdr.sh_offset=elf64_to_cpu(sec.shdr.sh_offset); sec.shdr.sh_size=elf64_to_cpu(sec.shdr.sh_size); sec.shdr.sh_link=elf32_to_cpu(sec.shdr.sh_link); sec.shdr.sh_info=elf32_to_cpu(sec.shdr.sh_info); sec.shdr.sh_addralign=elf64_to_cpu(sec.shdr.sh_addralign); sec.shdr.sh_entsize=elf64_to_cpu(sec.shdr.sh_entsize); if (sec.shdr.sh_link as usize)<SHNUM { sec.link=SECS.add(sec.shdr.sh_link as usize); } }
}

unsafe fn read_strtabs(fp: &mut std::fs::File) { for i in 0..SHNUM { let s=&mut *SECS.add(i); if s.shdr.sh_type!=3 {continue;} let l=s.shdr.sh_size as usize; s.strtab=std::alloc::alloc(std::alloc::Layout::array::<u8>(l).unwrap()) as *mut c_char; fp.seek(SeekFrom::Start(s.shdr.sh_offset)).unwrap(); if fp.read_exact(std::slice::from_raw_parts_mut(s.strtab as *mut u8,l)).is_err(){die("Cannot read symbol table\n");} } }
unsafe fn read_symtabs(fp: &mut std::fs::File) { for i in 0..SHNUM { let s=&mut *SECS.add(i); if s.shdr.sh_type==18 { s.xsymtab=std::alloc::alloc(std::alloc::Layout::array::<u8>(s.shdr.sh_size as usize).unwrap()) as *mut u32; fp.seek(SeekFrom::Start(s.shdr.sh_offset)).unwrap(); fp.read_exact(std::slice::from_raw_parts_mut(s.xsymtab as *mut u8,s.shdr.sh_size as usize)).unwrap(); SHXSYMTABNDX=i; } else if s.shdr.sh_type==2 { let n=s.shdr.sh_size as usize/size_of::<Elf_Sym>(); s.symtab=std::alloc::alloc(std::alloc::Layout::array::<Elf_Sym>(n).unwrap()) as *mut Elf_Sym; fp.seek(SeekFrom::Start(s.shdr.sh_offset)).unwrap(); fp.read_exact(std::slice::from_raw_parts_mut(s.symtab as *mut u8,s.shdr.sh_size as usize)).unwrap(); for j in 0..n {let x=&mut *s.symtab.add(j); x.st_name=elf32_to_cpu(x.st_name); x.st_value=elf64_to_cpu(x.st_value); x.st_size=elf64_to_cpu(x.st_size); x.st_shndx=elf16_to_cpu(x.st_shndx);} SHSYMTABNDX=i; } } }
unsafe fn read_relocs(fp: &mut std::fs::File) { for i in 0..SHNUM { let s=&mut *SECS.add(i); if s.shdr.sh_type!=SHT_REL_TYPE {continue;} let n=s.shdr.sh_size as usize/size_of::<Elf_Rel>(); s.reltab=std::alloc::alloc(std::alloc::Layout::array::<Elf_Rel>(n).unwrap()) as *mut Elf_Rel; fp.seek(SeekFrom::Start(s.shdr.sh_offset)).unwrap(); fp.read_exact(std::slice::from_raw_parts_mut(s.reltab as *mut u8,s.shdr.sh_size as usize)).unwrap(); for j in 0..n {let r=&mut *s.reltab.add(j); r.r_offset=elf64_to_cpu(r.r_offset); r.r_info=elf64_to_cpu(r.r_info); r.r_addend=elf64_to_cpu(r.r_addend) as i64;} } }
unsafe fn add_reloc(r:&mut Relocs, offset:u32) { if r.count==r.size {let n=r.size+50000; r.offset=std::alloc::realloc(r.offset as *mut u8, std::alloc::Layout::array::<u32>(r.size).unwrap(), n*size_of::<u32>()) as *mut u32; r.size=n;} *r.offset.add(r.count)=offset; r.count+=1; }
unsafe fn do_reloc(_sec:*mut Section, rel:*mut Elf_Rel, sym:*mut Elf_Sym, symname:*const c_char)->i32 { let t=((*rel).r_info&0xffffffff) as u32; match t { 0|3|16|18|19|20|21|22|31=>{}, 4=>{if (*sym).st_shndx!=0xfff1{die("Unsupported relocation type\n");} let p=b"__kcfi_typeid_\0"; if CStr::from_ptr(symname).to_bytes().starts_with(&p[..p.len()-1]){}else{die("Invalid absolute relocation\n");}}, 22=>add_reloc(&mut RELOCS64,(*rel).r_offset as u32), _=>die("Unsupported relocation type\n")}; 0 }
unsafe fn walk_relocs(){for i in 0..SHNUM{let s=&mut *SECS.add(i);if s.shdr.sh_type!=SHT_REL_TYPE{continue;}let st=s.link;let ap=SECS.add(s.shdr.sh_info as usize);if (*ap).shdr.sh_flags&2==0{continue;}for j in 0..s.shdr.sh_size as usize/size_of::<Elf_Rel>(){let r=s.reltab.add(j);let sym=(*st).symtab.add(((*r).r_info>>32) as usize);do_reloc(s,r,sym,sym_name((*(*st).link).strtab,sym));}}}
unsafe fn emit_relocs(){walk_relocs();std::slice::from_raw_parts_mut(RELOCS64.offset,RELOCS64.count).sort_unstable();println!(".section \".vmlinux.relocs_64\",\"a\"");for i in 0..RELOCS64.count{println!("\t.long 0x{:08x}",*RELOCS64.offset.add(i));}}
unsafe fn process(fp:&mut std::fs::File){read_ehdr(fp);read_shdrs(fp);read_strtabs(fp);read_symtabs(fp);read_relocs(fp);emit_relocs();}
unsafe fn usage()->!{die("relocs vmlinux\n")}
fn main(){let a:Vec<String>=std::env::args().collect();if a.len()!=2{unsafe{usage()}}let mut f=std::fs::File::open(&a[1]).unwrap();unsafe{process(&mut f)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
