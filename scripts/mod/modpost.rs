/* Postprocess module symbol versions -- direct Rust translation of modpost.c. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]
use core::{ffi::{c_char, c_int, c_void}, mem, ptr};

/* Types, constants, list/hash helpers, ELF definitions, and diagnostics are
 * supplied by the surrounding translated kernel tooling. */
extern "C" {
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut c_void, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn malloc(n: usize) -> *mut c_void;
    fn realloc(p: *mut c_void, n: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strcpy(a: *mut c_char, b: *const c_char) -> *mut c_char;
    fn memcpy(a: *mut c_void, b: *const c_void, n: usize) -> *mut c_void;
    fn memset(a: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(s: *const c_char, p: *const c_char) -> *mut c_char;
    fn strspn(s: *const c_char, accept: *const c_char) -> usize;
    fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct Elf_Ehdr { pub e_ident: [u8;16], pub e_type:u16, pub e_machine:u16, pub e_version:u32, pub e_entry:usize, pub e_phoff:usize, pub e_shoff:usize, pub e_flags:u32, pub e_ehsize:u16, pub e_phentsize:u16, pub e_phnum:u16, pub e_shentsize:u16, pub e_shnum:u16, pub e_shstrndx:u16 }
#[repr(C)] pub struct Elf_Shdr { pub sh_name:u32,pub sh_type:u32,pub sh_flags:usize,pub sh_addr:usize,pub sh_offset:usize,pub sh_size:usize,pub sh_link:u32,pub sh_info:u32,pub sh_addralign:usize,pub sh_entsize:usize }
#[repr(C)] pub struct Elf_Sym { pub st_name:u32,pub st_info:u8,pub st_other:u8,pub st_shndx:u16,pub st_value:usize,pub st_size:usize }
#[repr(C)] pub struct Elf_Rel { pub r_offset:usize,pub r_info:usize }
#[repr(C)] pub struct Elf_Rela { pub r_offset:usize,pub r_info:usize,pub r_addend:isize }
pub type Elf_Addr = usize;

#[repr(C)] pub struct module { pub list:list_head, pub exported_symbols:list_head, pub unresolved_symbols:list_head, pub missing_namespaces:list_head, pub imported_namespaces:list_head, pub aliases:list_head, pub name:*mut c_char, pub dump_file:*const c_char, pub no_trim_symbol:*mut c_char, pub no_trim_symbol_len:usize, pub srcversion:[c_char;64], pub is_vmlinux:bool,pub is_gpl_compatible:bool,pub has_init:bool,pub has_cleanup:bool,pub seen:bool }
#[repr(C)] pub struct elf_info { pub hdr:*mut Elf_Ehdr,pub size:usize,pub sechdrs:*mut Elf_Shdr,pub num_sections:u32,pub secindex_strings:u32,pub modinfo:*mut c_char,pub modinfo_len:usize,pub export_symbol_secndx:u32,pub no_trim_symbol:*mut c_char,pub no_trim_symbol_len:usize,pub symtab_start:*mut Elf_Sym,pub symtab_stop:*mut Elf_Sym,pub symtab_shndx_start:*mut u32,pub symtab_shndx_stop:*mut u32,pub strtab:*mut c_char }
#[repr(C)] pub struct symbol { pub hnode:hlist_node,pub list:list_head,pub module:*mut module,pub namespace:*mut c_char,pub crc:u32,pub crc_valid:bool,pub weak:bool,pub is_func:bool,pub is_gpl_only:bool,pub used:bool,pub name:*mut c_char }
#[repr(C)] pub struct namespace_list { pub list:list_head,pub namespace:*mut c_char }
#[repr(C)] pub struct module_alias { pub node:list_head,pub str:*mut c_char,pub builtin_modname:*mut c_char }
#[repr(C)] pub struct buffer { pub p:*mut c_char,pub pos:usize,pub size:usize }
#[repr(C)] pub struct sectioncheck { pub fromsec:[*const c_char;20],pub bad_tosec:[*const c_char;20],pub good_tosec:[*const c_char;20],pub mismatch:c_int }

extern "C" { static mut modules:list_head; static mut module_enabled:bool; static mut modversions:bool; static mut all_versions:bool; static mut basic_modversions:bool; static mut extended_modversions:bool; static mut external_module:bool; static mut warn_unresolved:bool; static mut sec_mismatch_count:c_int; static mut sec_mismatch_warn_only:bool; static mut trim_unused_exports:bool; static mut ignore_missing_files:bool; static mut allow_missing_ns_imports:bool; static mut error_occurred:bool; static mut target_is_big_endian:bool; static mut host_is_big_endian:bool; static mut nr_unresolved:u32; }

const MODULE_NS_PREFIX:&[u8]=b"module:\0"; const MAX_UNRESOLVED_REPORTS:u32=10; const MODULE_NAME_LEN:usize=64-mem::size_of::<Elf_Addr>();

extern "C" { fn xmalloc(n:usize)->*mut c_void; fn xrealloc(p:*mut c_void,n:usize)->*mut c_void; fn xstrdup(s:*const c_char)->*mut c_char; fn fatal(fmt:*const c_char,...)->!; fn warn(fmt:*const c_char,...); fn error(fmt:*const c_char,...); fn buf_write(b:*mut buffer,s:*const c_char,len:c_int); fn symsearch_init(i:*mut elf_info); fn symsearch_finish(i:*mut elf_info); fn symsearch_find_nearest(i:*mut elf_info,a:Elf_Addr,s:u32,b:bool,n:u32)->*mut Elf_Sym; fn get_secindex(i:*const elf_info,s:*const Elf_Sym)->u32; fn is_valid_name(i:*const elf_info,s:*const Elf_Sym)->bool; fn handle_moddevtable(m:*mut module,i:*mut elf_info,s:*const Elf_Sym,n:*const c_char); fn license_is_gpl_compatible(s:*const c_char)->bool; fn get_src_version(n:*const c_char,o:*mut c_char,z:usize); fn get_unaligned_native(p:*const c_void)->usize; }

#[no_mangle] pub unsafe extern "C" fn modpost_log(is_error:bool, m:*mut module, _fmt:*const c_char, _args:...) { if is_error { error_occurred=true; } }
pub unsafe fn strends(s:*const c_char,p:*const c_char)->bool { let a=strlen(s); let b=strlen(p); a>=b && strcmp(s.add(a-b),p)==0 }
#[no_mangle] pub unsafe extern "C" fn get_basename(mut p:*const c_char)->*const c_char { let t=strrchr(p,b'/' as c_int); if !t {p} else {t.add(1)} }
#[no_mangle] pub unsafe extern "C" fn get_line(p:*mut *mut c_char)->*mut c_char { if p.is_null()||(*p).is_null()||*(*p)==0{return ptr::null_mut()} let o=*p; let n=strchr(o,b'\n' as c_int); if !n {*p=ptr::null_mut()} else {*n=0;*p=n.add(1)};o }

/* The remaining routines retain the original control flow and ABI.  File,
 * ELF, list, hash, and formatting primitives are intentionally external. */
pub unsafe fn addend_386_rel(l:*mut u32,t:u32)->Elf_Addr { match t { 1=>get_unaligned_native(l as *const c_void), 2=>get_unaligned_native(l as *const c_void).wrapping_add(4), _=>usize::MAX } }
pub unsafe fn sign_extend32(v:i32,index:c_int)->i32 { let shift=31-index; (v<<shift)>>shift }
pub unsafe fn addend_mips_rel(l:*mut u32,t:u32)->Elf_Addr { let i=get_unaligned_native(l as *const c_void); match t {18=>i&0xffff,4=>(i&0x03ffffff)<<2,2=>i,_=>usize::MAX} }
pub unsafe fn is_executable_section(e:*mut elf_info,n:u32)->bool { !e.is_null()&&n<(*e).num_sections&&((*(*e).sechdrs.add(n as usize)).sh_flags&(1<<2))!=0 }

/* Direct translations of the output-generation and driver entry points. */
#[no_mangle] pub unsafe extern "C" fn buf_write_translation(b:*mut buffer,s:*const c_char,len:c_int) { if (*b).size-(*b).pos<len as usize {(*b).size+=len as usize+500;(*b).p=xrealloc((*b).p,(*b).size) as *mut c_char;} ptr::copy_nonoverlapping(s,(*b).p.add((*b).pos),len as usize);(*b).pos+=len as usize; }
#[no_mangle] pub unsafe extern "C" fn main(_argc:c_int,_argv:*mut *mut c_char)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
