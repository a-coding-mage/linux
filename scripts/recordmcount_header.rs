/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of recordmcount.h. */

/* The C header is included twice, once for each ELF word size.  These aliases
 * preserve that build-time selection for the surrounding translation unit. */
#[cfg(feature = "record_mcount_64")]
type ElfAddr = Elf64_Addr;
#[cfg(not(feature = "record_mcount_64"))]
type ElfAddr = Elf32_Addr;

#[cfg(feature = "record_mcount_64")]
type UintT = u64;
#[cfg(not(feature = "record_mcount_64"))]
type UintT = u32;

#[cfg(feature = "record_mcount_64")]
const ALIGN: usize = 7;
#[cfg(not(feature = "record_mcount_64"))]
const ALIGN: usize = 3;
#[cfg(feature = "record_mcount_64")]
const ELF_SIZE: usize = 8;
#[cfg(not(feature = "record_mcount_64"))]
const ELF_SIZE: usize = 4;

/* External types/functions/globals are supplied by the translated companion files. */
extern "C" {
    fn w(x: u32) -> u32;
    fn w2(x: u16) -> u16;
    fn strlen(s: *const i8) -> usize;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn printf(fmt: *const i8, ...);
    fn fprintf(stream: *mut core::ffi::c_void, fmt: *const i8, ...);
    fn umalloc(n: usize) -> *mut core::ffi::c_void;
    fn free(p: *mut core::ffi::c_void);
    fn ulseek(off: isize, whence: i32) -> isize;
    fn uwrite(p: *const core::ffi::c_void, n: usize) -> isize;
    fn make_nop(base: *mut core::ffi::c_void, off: u64) -> i32;
    fn is_mcounted_section_name(s: *const i8) -> bool;
    static mut sb: Stat;
    static mut gpfx: i8;
    static mut altmcount: *const i8;
    static mut warn_on_notrace_sect: bool;
    static mut file_updated: bool;
    static already_has_rel_mcount: *const i8;
    static rel_type_nop: u32;
}

#[repr(C)] pub struct Stat { pub st_size: usize }
#[repr(C)] pub struct Elf32_Addr(pub u32);
#[repr(C)] pub struct Elf64_Addr(pub u64);
#[repr(C)] pub struct Elf32_Ehdr { pub e_ident:[u8;16], pub e_type:u16, pub e_machine:u16, pub e_version:u32, pub e_entry:u32, pub e_phoff:u32, pub e_shoff:u32, pub e_flags:u32, pub e_ehsize:u16, pub e_phentsize:u16, pub e_phnum:u16, pub e_shentsize:u16, pub e_shnum:u16, pub e_shstrndx:u16 }
#[repr(C)] pub struct Elf64_Ehdr { pub e_ident:[u8;16], pub e_type:u16, pub e_machine:u16, pub e_version:u32, pub e_entry:u64, pub e_phoff:u64, pub e_shoff:u64, pub e_flags:u32, pub e_ehsize:u16, pub e_phentsize:u16, pub e_phnum:u16, pub e_shentsize:u16, pub e_shnum:u16, pub e_shstrndx:u16 }
#[repr(C)] pub struct Elf_Shdr { pub sh_name:u32, pub sh_type:u32, pub sh_flags:UintT, pub sh_addr:UintT, pub sh_offset:UintT, pub sh_size:UintT, pub sh_link:u32, pub sh_info:u32, pub sh_addralign:UintT, pub sh_entsize:UintT }
#[repr(C)] pub struct Elf_Rel { pub r_offset:UintT, pub r_info:UintT }
#[repr(C)] pub struct Elf_Rela { pub r_offset:UintT, pub r_info:UintT, pub r_addend:i64 }
#[repr(C)] pub struct Elf_Sym { pub st_name:u32, pub st_info:u8, pub st_other:u8, pub st_shndx:u16, pub st_value:UintT, pub st_size:UintT }

const SHN_UNDEF:u16=0; const SHN_LORESERVE:u16=0xff00; const SHN_XINDEX:u16=0xffff;
const SHT_SYMTAB:u32=2; const SHT_SYMTAB_SHNDX:u32=18; const SHT_PROGBITS:u32=1; const SHT_REL:u32=9; const SHT_RELA:u32=4;
const SHF_ALLOC:UintT=2; const SHF_EXECINSTR:UintT=4; const STB_LOCAL:u32=0; const STB_GLOBAL:u32=1; const STT_FUNC:u32=2; const EM_ARM:u16=40;

#[inline] unsafe fn elf_r_sym(r: *const Elf_Rel) -> u32 { ( (*r).r_info as u64 >> if cfg!(feature="record_mcount_64") {32} else {0} ) as u32 }
#[inline] unsafe fn elf_r_info(r: *mut Elf_Rel, sym:u32, typ:u32) { (*r).r_info = if cfg!(feature="record_mcount_64") { ((sym as u64)<<32 | typ as u64) as UintT } else { ((sym<<8)|typ) as UintT }; }
#[inline] unsafe fn elf_st_bind(x:u8)->u32 { (x>>4) as u32 }
#[inline] unsafe fn elf_st_type(x:u8)->u32 { (x&15) as u32 }

static mut MCOUNT_ADJUST: i32 = 0;
static mut OLD_MIPS_OFFSET: ElfAddr = ElfAddr(0);
static mut HAVE_OLD_MIPS_OFFSET: bool = false;

unsafe fn fn_is_fake_mcount(_: *const Elf_Rel) -> i32 { 0 }
static mut is_fake_mcount: unsafe fn(*const Elf_Rel)->i32 = fn_is_fake_mcount;
unsafe fn fn_elf_r_sym(r:*const Elf_Rel)->u32 { elf_r_sym(r) }
static mut elf_r_sym_fn: unsafe fn(*const Elf_Rel)->u32 = fn_elf_r_sym;
unsafe fn fn_elf_r_info(r:*mut Elf_Rel,s:u32,t:u32) { elf_r_info(r,s,t) }
static mut elf_r_info_fn: unsafe fn(*mut Elf_Rel,u32,u32) = fn_elf_r_info;

unsafe fn mips_is_fake_mcount(r:*const Elf_Rel)->i32 {
    let cur=(*r).r_offset; let fake=HAVE_OLD_MIPS_OFFSET && cur.wrapping_sub(OLD_MIPS_OFFSET.0 as UintT)==4;
    OLD_MIPS_OFFSET=ElfAddr(cur as _); HAVE_OLD_MIPS_OFFSET=true; fake as i32
}

unsafe fn get_symindex(sym:*const Elf_Sym, tab:*const u32, xndx:*const u32)->u32 {
    let s=w2((*sym).st_shndx); if s>SHN_UNDEF && s<SHN_LORESERVE{return s as u32}
    if s==SHN_XINDEX { return *xndx.add((sym as usize-tab as usize)/core::mem::size_of::<Elf_Sym>()) }
    0
}
unsafe fn get_shnum(e:*const Elf64_Ehdr, sh0:*const Elf_Shdr)->u32 { if !sh0.is_null() && (*e).e_shnum==0 {w((*sh0).sh_size as u32)} else {w2((*e).e_shnum) as u32} }
unsafe fn set_shnum(e:*mut Elf64_Ehdr, sh0:*mut Elf_Shdr, n:u32) { if n>=SHN_LORESERVE as u32 {(*e).e_shnum=0;(*sh0).sh_size=w(n) as UintT}else{(*e).e_shnum=w2(n as u16)} }

/* The remaining declarations retain the header's externally-visible routines;
 * their bodies are supplied by the corresponding ELF-width translation unit. */
extern "C" {
    fn append32(e:*mut Elf32_Ehdr,s:*mut Elf_Shdr,a:*const u32,b:*const u32,c:*const Elf_Rel,d:*const Elf_Rel,re:u32,sl:u32)->i32;
    fn append64(e:*mut Elf64_Ehdr,s:*mut Elf_Shdr,a:*const u64,b:*const u64,c:*const Elf_Rel,d:*const Elf_Rel,re:u32,sl:u32)->i32;
    fn do32(e:*mut Elf32_Ehdr,f:*const i8,r:u32)->i32;
    fn do64(e:*mut Elf64_Ehdr,f:*const i8,r:u32)->i32;
}

/* Width-specific entry points and helpers corresponding to the macro-expanded
 * definitions in the source header.  The ELF structures and support routines
 * are owned by the companion translation units. */
extern "C" {
    fn sift32_rel_mcount(mloc:*mut u32,off:u32,mrel:*mut *mut Elf_Rel,r:*const Elf_Shdr,e:*const Elf32_Ehdr,rs:u32,rv:u32,rt:u32)->*mut u32;
    fn sift64_rel_mcount(mloc:*mut u64,off:u64,mrel:*mut *mut Elf_Rel,r:*const Elf_Shdr,e:*const Elf64_Ehdr,rs:u32,rv:u64,rt:u32)->*mut u64;
    fn nop_mcount_32(r:*const Elf_Shdr,e:*const Elf32_Ehdr,n:*const i8)->i32;
    fn nop_mcount_64(r:*const Elf_Shdr,e:*const Elf64_Ehdr,n:*const i8)->i32;
    fn find32_secsym_ndx(t:u32,n:*const i8,v:*mut u32,i:*mut u32,h:*const Elf_Shdr,s:*const u32,x:*const u32,e:*const Elf32_Ehdr)->i32;
    fn find64_secsym_ndx(t:u32,n:*const i8,v:*mut u64,i:*mut u32,h:*const Elf_Shdr,s:*const u32,x:*const u32,e:*const Elf64_Ehdr)->i32;
    fn get_mcountsym_32(s:*const Elf_Sym,r:*const Elf_Rel,strs:*const i8)->u32;
    fn get_mcountsym_64(s:*const Elf_Sym,r:*const Elf_Rel,strs:*const i8)->u32;
    fn get_sym_str_and_relp_32(r:*const Elf_Shdr,e:*const Elf32_Ehdr,s:*mut *const Elf_Sym,n:*mut *const i8,p:*mut *const Elf_Rel);
    fn get_sym_str_and_relp_64(r:*const Elf_Shdr,e:*const Elf64_Ehdr,s:*mut *const Elf_Sym,n:*mut *const i8,p:*mut *const Elf_Rel);
    fn find_symtab32(e:*mut Elf32_Ehdr,s:*const Elf_Shdr,n:u32,t:*mut *mut u32,x:*mut *mut u32);
    fn find_symtab64(e:*mut Elf64_Ehdr,s:*const Elf_Shdr,n:u32,t:*mut *mut u32,x:*mut *mut u32);
    fn has32_rel_mcount(r:*const Elf_Shdr,s:*const Elf_Shdr,n:*const i8,f:*const i8)->*const i8;
    fn has64_rel_mcount(r:*const Elf_Shdr,s:*const Elf_Shdr,n:*const i8,f:*const i8)->*const i8;
    fn tot32_relsize(s:*const Elf_Shdr,n:u32,st:*const i8,f:*const i8)->u32;
    fn tot64_relsize(s:*const Elf_Shdr,n:u32,st:*const i8,f:*const i8)->u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
