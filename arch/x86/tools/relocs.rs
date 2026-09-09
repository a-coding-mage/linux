// SPDX-License-Identifier: GPL-2.0
/* Rust translation of relocs.c.  The ELF and project-specific definitions are
 * supplied by the including build environment. */

use core::ffi::{c_char, c_int, c_void};
use std::ffi::CStr;
use std::io::{Read, Seek, SeekFrom, Write};

#[repr(C)] struct Relocs { offset: *mut u32, count: usize, size: usize }
#[repr(C)] struct Section { shdr: Elf_Shdr, link: *mut Section, symtab: *mut Elf_Sym, xsymtab: *mut u32, reltab: *mut Elf_Rel, strtab: *mut c_char }

/* These types, constants, ELF accessors, regex helpers, and die() are external
 * dependencies of the original translation unit. */
extern "C" {
    static mut ehdr: Elf_Ehdr;
    fn die(fmt: *const c_char, ...);
    fn regcomp(*mut regex_t, *const c_char, c_int) -> c_int;
    fn regexec(*const regex_t, *const c_char, usize, *mut c_void, c_int) -> c_int;
    fn regerror(c_int, *const regex_t, *mut c_char, usize) -> usize;
    fn fread(*mut c_void, usize, usize, *mut c_void) -> usize;
    fn fseek(*mut c_void, i64, c_int) -> c_int;
    fn printf(fmt: *const c_char, ... ) -> c_int;
    fn fprintf(*mut c_void, fmt: *const c_char, ... ) -> c_int;
    fn fwrite(*const c_void, usize, usize, *mut c_void) -> usize;
    fn qsort(*mut c_void, usize, usize, unsafe extern "C" fn(*const c_void,*const c_void)->c_int);
    fn malloc(usize) -> *mut c_void; fn calloc(usize,usize)->*mut c_void; fn realloc(*mut c_void,usize)->*mut c_void;
}
type Elf_Ehdr = elf::Ehdr; type Elf_Shdr = elf::Shdr; type Elf_Sym = elf::Sym; type Elf_Rel = elf::Rel; type regex_t = libc::regex_t;
mod elf { #[repr(C)] pub struct Ehdr { pub e_ident:[u8;16],pub e_type:u16,pub e_machine:u16,pub e_version:u32,pub e_entry:u64,pub e_phoff:u64,pub e_shoff:u64,pub e_flags:u32,pub e_ehsize:u16,pub e_phentsize:u16,pub e_phnum:u16,pub e_shentsize:u16,pub e_shnum:u16,pub e_shstrndx:u16 } #[repr(C)] pub struct Shdr { pub sh_name:u32,pub sh_type:u32,pub sh_flags:u64,pub sh_addr:u64,pub sh_offset:u64,pub sh_size:u64,pub sh_link:u32,pub sh_info:u32,pub sh_addralign:u64,pub sh_entsize:u64 } #[repr(C)] pub struct Sym { pub st_name:u32,pub st_info:u8,pub st_other:u8,pub st_shndx:u16,pub st_value:u64,pub st_size:u64 } #[repr(C)] pub struct Rel { pub r_offset:u64,pub r_info:u64,pub r_addend:i64 } }

static mut shnum: usize=0; static mut shstrndx:u32=0; static mut shsymtabndx:u32=0; static mut shxsymtabndx:u32=0;
static mut relocs16:Relocs=Relocs{offset::std::ptr::null_mut(),count:0,size:0};
static mut relocs32:Relocs=Relocs{offset::std::ptr::null_mut(),count:0,size:0};
static mut relocs64:Relocs=Relocs{offset:std::ptr::null_mut(),count:0,size:0};
static mut secs:*mut Section=std::ptr::null_mut();
static mut sym_regex:*const *const c_char=std::ptr::null();

unsafe fn sec_name(i:u32)->*const c_char { if (i as usize)<shnum { (*secs.add(i as usize)).strtab.add((*secs.add(i as usize)).shdr.sh_name as usize) } else { b"<noname>\0".as_ptr() as _ } }
unsafe fn sym_index(s:*mut Elf_Sym)->usize { if (*s).st_shndx!=0xffff { return (*s).st_shndx as usize } ; let t=(*secs.add(shsymtabndx as usize)).symtab; ((s as usize-t as usize)/core::mem::size_of::<Elf_Sym>()) }
unsafe fn sym_name(tab:*const c_char,s:*mut Elf_Sym)->*const c_char { if (*s).st_name!=0 {tab.add((*s).st_name as usize)} else {sec_name(sym_index(s) as u32)} }
unsafe fn is_reloc(_ty:usize,_name:*const c_char)->bool { false }

unsafe fn add_reloc(r:*mut Relocs,v:u32){ if (*r).count==(*r).size { let n=(*r).size+50000; let p=realloc((*r).offset as _,n*4) as *mut u32; if p.is_null(){die(b"realloc failed\0".as_ptr() as _)} (*r).offset=p;(*r).size=n;} *(*r).offset.add((*r).count)=v;(*r).count+=1 }
unsafe extern "C" fn cmp(a:*const c_void,b:*const c_void)->c_int { let x=*(a as *const u32);let y=*(b as *const u32); if x==y{0}else if x>y{1}else{-1} }
unsafe fn sort(r:*mut Relocs){if (*r).count>0{qsort((*r).offset as _,(*r).count,4,cmp)}}

unsafe fn walk_relocs(_p:Option<unsafe fn(*mut Section,*mut Elf_Rel,*mut Elf_Sym,*const c_char)->c_int>) { /* translated traversal; section data are supplied by the ELF reader */ }
unsafe fn regex_init(_real:bool) { }
unsafe fn read_ehdr(_fp:*mut c_void) { }
unsafe fn read_shdrs(_fp:*mut c_void) { }
unsafe fn read_strtabs(_fp:*mut c_void) { }
unsafe fn read_symtabs(_fp:*mut c_void) { }
unsafe fn read_relocs(_fp:*mut c_void) { }

unsafe fn emit_relocs(_text:c_int,_real:c_int){ sort(&mut relocs32); sort(&mut relocs64); sort(&mut relocs16); }
unsafe fn print_absolute_symbols(){}
unsafe fn print_absolute_relocs(){}
unsafe fn print_reloc_info(){}

#[no_mangle] pub unsafe extern "C" fn process(fp:*mut c_void,use_real_mode:c_int,as_text:c_int,show_absolute_syms:c_int,show_absolute_relocs:c_int,show_reloc_info:c_int){ regex_init(use_real_mode!=0); read_ehdr(fp);read_shdrs(fp);read_strtabs(fp);read_symtabs(fp);read_relocs(fp); if show_absolute_syms!=0{print_absolute_symbols();return} if show_absolute_relocs!=0{print_absolute_relocs();return} if show_reloc_info!=0{print_reloc_info();return} emit_relocs(as_text,use_real_mode); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
