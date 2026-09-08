/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Google LLC
 */

// C header dependencies: dwarf.h, elfutils/libdw.h, elfutils/libdwfl.h,
// stdlib.h, stdio.h, hash.h, hashtable.h, and xalloc.h.

/* Options -- in gendwarfksyms.c */
unsafe extern "C" {
    pub static mut debug: ::core::ffi::c_int;
    pub static mut dump_dies: ::core::ffi::c_int;
    pub static mut dump_die_map: ::core::ffi::c_int;
    pub static mut dump_types: ::core::ffi::c_int;
    pub static mut dump_versions: ::core::ffi::c_int;
    pub static mut stable: ::core::ffi::c_int;
    pub static mut symtypes: ::core::ffi::c_int;
}

pub const __PREFIX: &str = "gendwarfksyms: ";
pub const SYMBOL_PTR_PREFIX: &str = "__gendwarfksyms_ptr_";
pub const SYMBOL_PTR_PREFIX_LEN: usize = SYMBOL_PTR_PREFIX.len();
pub const CACHE_HASH_BITS: u32 = 10;

// The following variadic macros retain their C preprocessor interfaces.
macro_rules! __println { ($prefix:expr, $format:expr $(, $arg:expr)*) => {
    unsafe { libc::fprintf(libc::stderr, concat!($prefix, "gendwarfksyms: %s: ", $format, "\n"),
        core::ffi::CStr::from_bytes_with_nul_unchecked(concat!(module_path!(), "\0")).as_ptr() $(, $arg)*) }
}; }
macro_rules! debug { ($format:expr $(, $arg:expr)*) => { if unsafe { debug != 0 } { __println!("", $format $(, $arg)*); } }; }
macro_rules! warn { ($format:expr $(, $arg:expr)*) => { __println!("warning: ", $format $(, $arg)*); }; }
macro_rules! error { ($format:expr $(, $arg:expr)*) => {{ __println!("error: ", $format $(, $arg)*); unsafe { libc::exit(1) } }}; }
macro_rules! __die_debug { ($color:expr, $format:expr $(, $arg:expr)*) => { if unsafe { dump_dies != 0 && dump_die_map != 0 } { /* fprintf(stderr, "\\033[" #color "m<" format ">\\033[39m", __VA_ARGS__) */ } }; }
macro_rules! die_debug_r { ($format:expr $(, $arg:expr)*) => { __die_debug!(91, $format $(, $arg)*); }; }
macro_rules! die_debug_g { ($format:expr $(, $arg:expr)*) => { __die_debug!(92, $format $(, $arg)*); }; }
macro_rules! die_debug_b { ($format:expr $(, $arg:expr)*) => { __die_debug!(94, $format $(, $arg)*); }; }

// C statement-expression helpers; callers must provide the C result variable semantics.
macro_rules! __check { ($expr:expr, $test:expr) => {{ let __res = $expr; if $test { error!("`%s` failed: %d", stringify!($expr), __res); } __res }}; }
macro_rules! check { ($expr:expr) => { __check!($expr, __res != 0) }; }
macro_rules! checkp { ($expr:expr) => { __check!($expr, __res < 0) }; }

pub const DW_TAG_enumerator_type: i32 = DW_TAG_enumerator;
pub const DW_TAG_formal_parameter_type: i32 = DW_TAG_formal_parameter;
pub const DW_TAG_member_type: i32 = DW_TAG_member;
pub const DW_TAG_template_type_parameter_type: i32 = DW_TAG_template_type_parameter;
pub const DW_TAG_typedef_type: i32 = DW_TAG_typedef;
pub const DW_TAG_variant_part_type: i32 = DW_TAG_variant_part;
pub const DW_TAG_variant_type: i32 = DW_TAG_variant;

pub type uintptr_t = usize;
pub type Elf64_Addr = u64;
pub type FILE = libc::FILE;
pub enum Dwarf_Die {}
pub enum hlist_node {}
pub enum list_head {}

#[inline]
pub unsafe fn addr_hash(addr: uintptr_t) -> libc::c_uint { hash_ptr(addr as *const libc::c_void) }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum symbol_state { SYMBOL_UNPROCESSED, SYMBOL_MAPPED, SYMBOL_PROCESSED }
#[repr(C)] pub struct symbol_addr { pub section: u32, pub address: Elf64_Addr }
#[repr(C)] pub struct symbol { pub name: *const libc::c_char, pub addr: symbol_addr, pub addr_hash: hlist_node, pub name_hash: hlist_node, pub state: symbol_state, pub die_addr: uintptr_t, pub ptr_die_addr: uintptr_t, pub crc: libc::c_ulong }
pub type symbol_callback_t = unsafe extern "C" fn(*mut symbol, *mut libc::c_void);

#[repr(C)]
#[derive(Copy, Clone)]
pub enum die_state { DIE_INCOMPLETE, DIE_FQN, DIE_UNEXPANDED, DIE_COMPLETE, DIE_SYMBOL, DIE_LAST = DIE_SYMBOL as isize }
#[repr(C)] pub enum die_fragment_type { FRAGMENT_EMPTY, FRAGMENT_STRING, FRAGMENT_LINEBREAK, FRAGMENT_DIE }
#[repr(C)] pub union die_fragment_data { pub str_: *mut libc::c_char, pub linebreak: libc::c_int, pub addr: uintptr_t }
#[repr(C)] pub struct die_fragment { pub type_: die_fragment_type, pub data: die_fragment_data, pub list: list_head }
#[inline] pub unsafe fn die_state_name(state: die_state) -> *const libc::c_char { match state { die_state::DIE_INCOMPLETE => b"DIE_INCOMPLETE\0".as_ptr() as _, die_state::DIE_FQN => b"DIE_FQN\0".as_ptr() as _, die_state::DIE_UNEXPANDED => b"DIE_UNEXPANDED\0".as_ptr() as _, die_state::DIE_COMPLETE => b"DIE_COMPLETE\0".as_ptr() as _, die_state::DIE_SYMBOL => b"DIE_SYMBOL\0".as_ptr() as _, _ => error!("unexpected die_state: %d", state as libc::c_int) } }
#[repr(C)] pub struct die { pub state: die_state, pub mapped: bool, pub fqn: *mut libc::c_char, pub tag: libc::c_int, pub addr: uintptr_t, pub fragments: list_head, pub hash: hlist_node }
pub type die_map_callback_t = unsafe extern "C" fn(*mut die, *mut libc::c_void);

#[repr(C)] pub struct cache { pub cache: [u8; 1 << CACHE_HASH_BITS] }
#[repr(C)] pub struct expansion_state { pub expand: bool, pub current_fqn: *const libc::c_char }
#[repr(C)] pub struct kabi_state { pub members: libc::c_int, pub placeholder: Dwarf_Die, pub orig_name: *const libc::c_char }
#[repr(C)] pub struct state { pub sym: *mut symbol, pub die: Dwarf_Die, pub first_list_item: bool, pub expand: expansion_state, pub expansion_cache: cache, pub kabi: kabi_state }
pub type die_callback_t = unsafe extern "C" fn(*mut state, *mut die, *mut Dwarf_Die) -> libc::c_int;
pub type die_match_callback_t = unsafe extern "C" fn(*mut Dwarf_Die) -> bool;

unsafe extern "C" {
    pub fn hash_ptr(ptr: *const libc::c_void) -> libc::c_uint;
    pub fn is_symbol_ptr(name: *const libc::c_char) -> bool;
    pub fn symbol_read_exports(file: *mut FILE) -> libc::c_int;
    pub fn symbol_read_symtab(fd: libc::c_int);
    pub fn symbol_get(name: *const libc::c_char) -> *mut symbol;
    pub fn symbol_set_ptr(sym: *mut symbol, ptr: *mut Dwarf_Die);
    pub fn symbol_set_die(sym: *mut symbol, die: *mut Dwarf_Die);
    pub fn symbol_set_crc(sym: *mut symbol, crc: libc::c_ulong);
    pub fn symbol_for_each(func: symbol_callback_t, arg: *mut libc::c_void);
    pub fn symbol_print_versions(); pub fn symbol_free();
    pub fn __die_map_get(addr: uintptr_t, state: die_state, res: *mut *mut die) -> libc::c_int;
    pub fn die_map_get(die: *mut Dwarf_Die, state: die_state) -> *mut die;
    pub fn die_map_add_string(pd: *mut die, str_: *const libc::c_char);
    pub fn die_map_add_linebreak(pd: *mut die, linebreak: libc::c_int);
    pub fn die_map_for_each(func: die_map_callback_t, arg: *mut libc::c_void);
    pub fn die_map_add_die(pd: *mut die, child: *mut die); pub fn die_map_free();
    pub fn cache_set(cache: *mut cache, key: libc::c_ulong, value: libc::c_int);
    pub fn cache_get(cache: *mut cache, key: libc::c_ulong) -> libc::c_int;
    pub fn cache_init(cache: *mut cache); pub fn cache_free(cache: *mut cache);
    pub fn match_all(die: *mut Dwarf_Die) -> bool;
    pub fn process_die_container(state: *mut state, cache: *mut die, die: *mut Dwarf_Die, func: die_callback_t, mat: die_match_callback_t) -> libc::c_int;
    pub fn process_cu(cudie: *mut Dwarf_Die);
    pub fn generate_symtypes_and_versions(file: *mut FILE);
    pub fn kabi_get_byte_size(fqn: *const libc::c_char, value: *mut libc::c_ulong) -> bool;
    pub fn kabi_is_enumerator_ignored(fqn: *const libc::c_char, field: *const libc::c_char) -> bool;
    pub fn kabi_get_enumerator_value(fqn: *const libc::c_char, field: *const libc::c_char, value: *mut libc::c_ulong) -> bool;
    pub fn kabi_is_declonly(fqn: *const libc::c_char) -> bool;
    pub fn kabi_get_type_string(ty: *const libc::c_char, str_: *mut *const libc::c_char) -> bool;
    pub fn kabi_read_rules(fd: libc::c_int); pub fn kabi_free();
}

#[inline] pub unsafe fn cache_mark_expanded(cache: *mut cache, addr: *mut libc::c_void) { cache_set(cache, addr as libc::c_ulong, 1); }
#[inline] pub unsafe fn cache_was_expanded(cache: *mut cache, addr: *mut libc::c_void) -> bool { cache_get(cache, addr as libc::c_ulong) == 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
