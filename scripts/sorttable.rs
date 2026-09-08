// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of sorttable.c. */

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

// The ELF types, parser, accessors, and constants are supplied by the surrounding build.
extern "C" {
    static mut elf_parser: ElfParser;
    fn elf_map_machine(ehdr: *mut Elf_Ehdr) -> c_int;
    fn elf_map_long_size(addr: *mut c_void) -> c_int;
    fn elf_map(path: *const c_char, size: *mut usize, flags: c_int) -> *mut c_void;
    fn elf_unmap(addr: *mut c_void, size: usize);
}

#[repr(C)] pub struct Elf_Ehdr { pub e32: Elf32Ehdr, pub e64: Elf64Ehdr }
#[repr(C)] pub struct Elf32Ehdr { pub e_ident: [u8; 16], pub e_machine: u16, pub e_ehsize: u16, pub e_shentsize: u16 }
#[repr(C)] pub struct Elf64Ehdr { pub e_ident: [u8; 16], pub e_machine: u16, pub e_ehsize: u16, pub e_shentsize: u16 }
#[repr(C)] pub struct Elf_Shdr { _x: [u8; 0] }
#[repr(C)] pub struct Elf_Sym { _x: [u8; 0] }
#[repr(C)] pub struct Elf_Rela { _x: [u8; 0] }
pub type Elf32_Word = u32; pub type Elf32_Addr = u32; pub type Elf64_Addr = u64;
#[repr(C)] pub struct ElfParser {
    pub r: unsafe extern "C" fn(*const c_void) -> u32,
    pub r8: unsafe extern "C" fn(*const c_void) -> u64,
    pub r2: unsafe extern "C" fn(*const c_void) -> u16,
    pub w: unsafe extern "C" fn(u32, *mut c_void),
    pub w8: unsafe extern "C" fn(u64, *mut c_void),
    pub rela_write_addend: unsafe extern "C" fn(*mut Elf_Rela, u64),
}

type TableSort = unsafe extern "C" fn(*mut c_char, c_int);
const EM_ARCOMPACT: c_int = 93; const EM_XTENSA: c_int = 94; const EM_AARCH64: c_int = 183;
const EM_MICROBLAZE: c_int = 189; const EM_ARCV2: c_int = 195; const EM_RISCV: c_int = 243; const EM_LOONGARCH: c_int = 258;
const ERRSTR_MAXSZ: usize = 256; const FUNC_BLK_SIZE: usize = 1024; const FUNC_BLK_MASK: usize = FUNC_BLK_SIZE - 1;

static mut compare_extable: Option<unsafe extern "C" fn(*const c_void,*const c_void)->c_int> = None;
static mut extable_ent_size: c_int = 0; static mut long_size: c_int = 0;
unsafe fn get_index(start: *mut c_void, entsize: c_int, index: c_int) -> *mut c_void { (start as *mut u8).offset((entsize*index) as isize) as *mut c_void }
unsafe fn is_shndx_special(i: u32) -> bool { i != SHN_XINDEX && i >= SHN_LORESERVE && i <= SHN_HIRESERVE }
unsafe fn get_secindex(shndx:u32, sym_offs:usize, p:*const Elf32_Word)->c_int { if is_shndx_special(shndx){return (shndx-(SHN_HIRESERVE+1)) as c_int} if shndx!=SHN_XINDEX{return shndx as c_int} (elf_parser.r)((p.add(sym_offs)) as *const c_void) as c_int }

const SHN_XINDEX:u32=0xffff; const SHN_LORESERVE:u32=0xff00; const SHN_HIRESERVE:u32=0xffff;
extern "C" {
    fn ehdr_shoff(*const Elf_Ehdr)->u64; fn ehdr_shentsize(*const Elf_Ehdr)->c_int; fn ehdr_shnum(*const Elf_Ehdr)->u32; fn ehdr_shstrndx(*const Elf_Ehdr)->u32;
    fn shdr_size(*const Elf_Shdr)->u64; fn shdr_offset(*const Elf_Shdr)->u64; fn shdr_addr(*const Elf_Shdr)->u64; fn shdr_entsize(*const Elf_Shdr)->c_int; fn shdr_type(*const Elf_Shdr)->u32; fn shdr_name(*const Elf_Shdr)->c_int; fn shdr_link(*const Elf_Shdr)->u32;
    fn rela_offset(*const Elf_Rela)->u64; fn rela_info(*const Elf_Rela)->u64; fn rela_addend(*const Elf_Rela)->u64; fn sym_name(*const Elf_Sym)->u32; fn sym_value(*const Elf_Sym)->u64; fn sym_type(*const Elf_Sym)->u8; fn sym_shndx(*const Elf_Sym)->u32;
}

unsafe extern "C" fn compare_extable_32(a:*const c_void,b:*const c_void)->c_int { let av=(elf_parser.r)(a); let bv=(elf_parser.r)(b); if av<bv{-1}else if av>bv{1}else{0} }
unsafe extern "C" fn compare_extable_64(a:*const c_void,b:*const c_void)->c_int { let av=(elf_parser.r8)(a); let bv=(elf_parser.r8)(b); if av<bv{-1}else if av>bv{1}else{0} }

#[repr(C)] struct FuncInfo { addr:u64, size:u64 }
static mut function_list:*mut FuncInfo=ptr::null_mut(); static mut function_list_size:usize=0; static mut before_func:c_int=0;
unsafe fn add_field(addr:u64,size:u64)->c_int { if function_list_size & FUNC_BLK_MASK == 0 { let n=function_list_size+FUNC_BLK_SIZE; let p=libc::realloc(function_list as *mut c_void,n*std::mem::size_of::<FuncInfo>()) as *mut FuncInfo; if p.is_null(){return -1} function_list=p; } (*function_list.add(function_list_size))=FuncInfo{addr,size}; function_list_size+=1; 0 }

unsafe extern "C" fn compare_relative_table(a:*const c_void,b:*const c_void)->c_int { let av=(elf_parser.r)(a) as i32; let bv=(elf_parser.r)(b) as i32; av.cmp(&bv) as c_int }
unsafe extern "C" fn sort_relative_table(ext:*mut c_char,n:c_int) { let mut i=0; while i<n { let p=ext.add(i as usize*1) as *mut u32; (elf_parser.w)((elf_parser.r)(p as *const c_void).wrapping_add(i as u32),p as *mut c_void); i+=4; } libc::qsort(ext as *mut c_void,(n/8) as usize,8,Some(compare_relative_table)); i=0; while i<n {let p=ext.add(i as usize) as *mut u32;(elf_parser.w)((elf_parser.r)(p as *const c_void).wrapping_sub(i as u32),p as *mut c_void);i+=4;} }
unsafe extern "C" fn sort_relative_table_with_data(ext:*mut c_char,n:c_int) { let mut i=0; while i<n {let p=ext.add(i as usize) as *mut u32;(elf_parser.w)((elf_parser.r)(p as *const c_void).wrapping_add(i as u32),p as *mut c_void);(elf_parser.w)((elf_parser.r)(p.add(1) as *const c_void).wrapping_add(i as u32+4),p.add(1) as *mut c_void);i+=12;} libc::qsort(ext as *mut c_void,(n/12) as usize,12,Some(compare_relative_table));i=0;while i<n{let p=ext.add(i as usize) as *mut u32;(elf_parser.w)((elf_parser.r)(p as *const c_void).wrapping_sub(i as u32),p as *mut c_void);(elf_parser.w)((elf_parser.r)(p.add(1) as *const c_void).wrapping_sub(i as u32+4),p.add(1) as *mut c_void);i+=12;} }

// The remaining file-local orchestration is represented literally through the C ABI helpers.
extern "C" { fn do_sort(ehdr:*mut Elf_Ehdr,fname:*const c_char,custom:Option<TableSort>)->c_int; }
#[no_mangle] pub unsafe extern "C" fn do_file(fname:*const c_char,addr:*mut c_void)->c_int { let mut custom=None; match elf_map_machine(addr as *mut Elf_Ehdr) { EM_AARCH64|EM_RISCV|4|258|22|62 => custom=Some(sort_relative_table_with_data), 15|20|21 => custom=Some(sort_relative_table), EM_ARCOMPACT|EM_ARCV2|40|189|8|94=>{}, _=>return -1 }; match elf_map_long_size(addr){4=>{compare_extable=Some(compare_extable_32);long_size=4;extable_ent_size=8},8=>{compare_extable=Some(compare_extable_64);long_size=8;extable_ent_size=16},_=>return -1}; do_sort(addr as *mut Elf_Ehdr,fname,custom) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
