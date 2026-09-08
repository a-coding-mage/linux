/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies are supplied by the surrounding translation unit.

#[cfg(target_pointer_width = "32")]
pub type Elf_Ehdr = Elf32_Ehdr;
#[cfg(target_pointer_width = "32")]
pub type Elf_Shdr = Elf32_Shdr;
#[cfg(target_pointer_width = "32")]
pub type Elf_Sym = Elf32_Sym;
#[cfg(target_pointer_width = "32")]
pub type Elf_Addr = Elf32_Addr;
#[cfg(target_pointer_width = "32")]
pub type Elf_Section = Elf32_Half;
#[cfg(target_pointer_width = "32")]
pub type Elf_Rel = Elf32_Rel;
#[cfg(target_pointer_width = "32")]
pub type Elf_Rela = Elf32_Rela;

#[cfg(target_pointer_width = "64")]
pub type Elf_Ehdr = Elf64_Ehdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Shdr = Elf64_Shdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Sym = Elf64_Sym;
#[cfg(target_pointer_width = "64")]
pub type Elf_Addr = Elf64_Addr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Section = Elf64_Half;
#[cfg(target_pointer_width = "64")]
pub type Elf_Rel = Elf64_Rel;
#[cfg(target_pointer_width = "64")]
pub type Elf_Rela = Elf64_Rela;

#[inline]
pub unsafe fn bswap<T: Copy>(x: T) -> T {
    // C's _Static_assert restricts T to 1, 2, 4, or 8 bytes; the caller
    // supplies the corresponding integer type and native byte swap.
    let mut out = x;
    match core::mem::size_of::<T>() {
        1 => {}
        2 => out = core::mem::transmute_copy::<u16, T>(&u16::from_be_bytes(core::mem::transmute_copy::<T, [u8; 2]>(&x))),
        4 => out = core::mem::transmute_copy::<u32, T>(&u32::from_be_bytes(core::mem::transmute_copy::<T, [u8; 4]>(&x))),
        8 => out = core::mem::transmute_copy::<u64, T>(&u64::from_be_bytes(core::mem::transmute_copy::<T, [u8; 8]>(&x))),
        _ => panic!("bug"),
    }
    out
}

#[macro_export]
macro_rules! TO_NATIVE {
    ($x:expr) => {{
        if $crate::target_is_big_endian == $crate::host_is_big_endian { $x } else { unsafe { $crate::bswap($x) } }
    }};
}

#[macro_export]
macro_rules! ARRAY_SIZE { ($arr:expr) => { ($arr.len()) }; }

#[repr(C)]
pub struct buffer {
    pub p: *mut core::ffi::c_char,
    pub pos: core::ffi::c_int,
    pub size: core::ffi::c_int,
}

extern "C" {
    pub fn buf_printf(buf: *mut buffer, fmt: *const core::ffi::c_char, ...);
    pub fn buf_write(buf: *mut buffer, s: *const core::ffi::c_char, len: core::ffi::c_int);
}

#[repr(C)]
pub struct module_alias {
    pub node: list_head,
    pub builtin_modname: *mut core::ffi::c_char,
    pub str_: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct module {
    pub list: list_head,
    pub exported_symbols: list_head,
    pub unresolved_symbols: list_head,
    pub dump_file: *const core::ffi::c_char,
    pub is_gpl_compatible: bool,
    pub is_vmlinux: bool,
    pub seen: bool,
    pub has_init: bool,
    pub has_cleanup: bool,
    pub srcversion: [core::ffi::c_char; 25],
    pub missing_namespaces: list_head,
    pub imported_namespaces: list_head,
    pub aliases: list_head,
    pub no_trim_symbol: *mut core::ffi::c_char,
    pub no_trim_symbol_len: core::ffi::c_uint,
    pub name: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct elf_info {
    pub size: usize,
    pub hdr: *mut Elf_Ehdr,
    pub sechdrs: *mut Elf_Shdr,
    pub symtab_start: *mut Elf_Sym,
    pub symtab_stop: *mut Elf_Sym,
    pub export_symbol_secndx: core::ffi::c_uint,
    pub strtab: *mut core::ffi::c_char,
    pub modinfo: *mut core::ffi::c_char,
    pub modinfo_len: core::ffi::c_uint,
    pub no_trim_symbol: *mut core::ffi::c_char,
    pub no_trim_symbol_len: core::ffi::c_uint,
    pub num_sections: core::ffi::c_uint,
    pub secindex_strings: core::ffi::c_uint,
    pub symtab_shndx_start: *mut Elf32_Word,
    pub symtab_shndx_stop: *mut Elf32_Word,
    pub symsearch: *mut symsearch,
}

#[inline]
pub unsafe fn get_secindex(info: *const elf_info, sym: *const Elf_Sym) -> core::ffi::c_uint {
    let mut index = (*sym).st_shndx as core::ffi::c_uint;
    if index == SHN_XINDEX as core::ffi::c_uint {
        return *(*info).symtab_shndx_start.add(sym.offset_from((*info).symtab_start) as usize);
    }
    if index >= SHN_LORESERVE as core::ffi::c_uint && index <= SHN_HIRESERVE as core::ffi::c_uint {
        return index.wrapping_sub(SHN_HIRESERVE as core::ffi::c_uint).wrapping_sub(1);
    }
    index
}

#[inline]
pub unsafe fn is_valid_name(elf: *mut elf_info, sym: *mut Elf_Sym) -> bool {
    let name = (*elf).strtab.add((*sym).st_name as usize);
    if name.is_null() || *name == 0 { return false; }
    !is_mapping_symbol(name)
}

extern "C" {
    pub fn symsearch_init(elf: *mut elf_info);
    pub fn symsearch_finish(elf: *mut elf_info);
    pub fn symsearch_find_nearest(elf: *mut elf_info, addr: Elf_Addr, secndx: core::ffi::c_uint, allow_negative: bool, min_distance: Elf_Addr) -> *mut Elf_Sym;
    pub fn handle_moddevtable(mod_: *mut module, info: *mut elf_info, sym: *mut Elf_Sym, symname: *const core::ffi::c_char);
    pub fn get_src_version(modname: *const core::ffi::c_char, sum: *mut core::ffi::c_char, sumlen: core::ffi::c_uint);
    pub static mut target_is_big_endian: bool;
    pub static mut host_is_big_endian: bool;
    pub fn get_basename(path: *const core::ffi::c_char) -> *const core::ffi::c_char;
    pub fn read_text_file(filename: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn get_line(stringp: *mut *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn sym_get_data(info: *const elf_info, sym: *const Elf_Sym) -> *mut core::ffi::c_void;
    pub fn modpost_log(is_error: bool, mod_: *mut module, fmt: *const core::ffi::c_char, ...);
    pub fn is_mapping_symbol(name: *const core::ffi::c_char) -> bool;
}

#[macro_export]
macro_rules! warn { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::modpost_log(false, core::ptr::null_mut(), $fmt $(, $arg)*); } }; }
#[macro_export]
macro_rules! error { ($fmt:expr $(, $arg:expr)*) => { unsafe { $crate::modpost_log(true, core::ptr::null_mut(), $fmt $(, $arg)*); } }; }
#[macro_export]
macro_rules! fatal { ($fmt:expr $(, $arg:expr)*) => {{ $crate::error!($fmt $(, $arg)*); unsafe { libc::exit(1) }; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
