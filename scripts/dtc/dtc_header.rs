/* SPDX-License-Identifier: GPL-2.0-or-later */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::MaybeUninit;

// C headers: stdio.h, string.h, stdlib.h, stdint.h, stdbool.h, stdarg.h,
// assert.h, ctype.h, errno.h, unistd.h, inttypes.h, libfdt_env.h, fdt.h,
// and "util.h" supply the corresponding external definitions.

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
    pub fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    pub fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    pub fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
}

#[cfg(feature = "debug")]
// C: #define debug(...) printf(__VA_ARGS__)
#[macro_export]
macro_rules! debug { ($($arg:tt)*) => { unsafe { ::core::format_args!($($arg)*) } }; }
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug { ($($arg:tt)*) => {}; }

pub const DEFAULT_FDT_VERSION: c_int = 17;

extern "C" {
    pub static mut quiet: c_int;
    pub static mut reservenum: c_uint;
    pub static mut minsize: c_int;
    pub static mut padsize: c_int;
    pub static mut alignsize: c_int;
    pub static mut phandle_format: c_int;
    pub static mut generate_symbols: c_int;
    pub static mut generate_fixups: c_int;
    pub static mut auto_label_aliases: c_int;
    pub static mut annotate: c_int;
}

pub const PHANDLE_LEGACY: c_int = 0x1;
pub const PHANDLE_EPAPR: c_int = 0x2;
pub const PHANDLE_BOTH: c_int = 0x3;

pub type cell_t = u32;

#[inline]
pub unsafe fn phandle_is_valid(phandle: cell_t) -> bool { phandle != 0 && phandle != !0u32 }

#[inline]
pub unsafe fn dtb_ld16(p: *const c_void) -> u16 {
    let bp = p as *const u8;
    ((*bp as u16) << 8) | *bp.add(1) as u16
}

#[inline]
pub unsafe fn dtb_ld32(p: *const c_void) -> u32 {
    let bp = p as *const u8;
    ((*bp as u32) << 24) | ((*bp.add(1) as u32) << 16) |
        ((*bp.add(2) as u32) << 8) | *bp.add(3) as u32
}

#[inline]
pub unsafe fn dtb_ld64(p: *const c_void) -> u64 {
    let bp = p as *const u8;
    ((*bp as u64) << 56) | ((*bp.add(1) as u64) << 48) |
        ((*bp.add(2) as u64) << 40) | ((*bp.add(3) as u64) << 32) |
        ((*bp.add(4) as u64) << 24) | ((*bp.add(5) as u64) << 16) |
        ((*bp.add(6) as u64) << 8) | *bp.add(7) as u64
}

#[inline] pub unsafe fn streq(a: *const c_char, b: *const c_char) -> bool { strcmp(a,b) == 0 }
#[inline] pub unsafe fn strstarts(s: *const c_char, prefix: *const c_char) -> bool { strncmp(s,prefix,strlen(prefix)) == 0 }
#[inline] pub unsafe fn strprefixeq(a: *const c_void, n: usize, b: *const c_char) -> bool { strlen(b) == n && memcmp(a,b,n) == 0 }
#[inline]
pub unsafe fn strends(s: *const c_char, suffix: *const c_char) -> bool {
    let len = strlen(s); let suffix_len = strlen(suffix);
    len >= suffix_len && streq(s.add(len - suffix_len), suffix)
}
#[inline] pub const fn align(x: usize, a: usize) -> usize { (x + a - 1) & !(a - 1) }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum markertype { TYPE_NONE, REF_PHANDLE, REF_PATH, LABEL, TYPE_UINT8, TYPE_UINT16, TYPE_UINT32, TYPE_UINT64, TYPE_STRING }
#[inline] pub fn is_type_marker(t: markertype) -> bool { (t as c_int) >= (markertype::TYPE_UINT8 as c_int) }

extern "C" { pub fn markername(markertype: markertype) -> *const c_char; }

#[repr(C)] pub struct marker { pub type_: markertype, pub offset: c_uint, pub ref_: *mut c_char, pub next: *mut marker }
#[repr(C)] pub struct data { pub len: c_uint, pub val: *mut c_char, pub markers: *mut marker }
pub const empty_data: data = data { len: 0, val: core::ptr::null_mut(), markers: core::ptr::null_mut() };

#[inline] pub unsafe fn next_type_marker(mut m: *mut marker) -> *mut marker { while !m.is_null() { if is_type_marker((*m).type_) { break; } m=(*m).next; } m }
#[inline] pub unsafe fn type_marker_length(m: *mut marker) -> usize { let next=next_type_marker((*m).next); if !next.is_null() { ((*next).offset-(*m).offset) as usize } else { 0 } }

extern "C" {
    pub fn data_free(d: data); pub fn data_grow_for(d: data, xlen: c_uint) -> data;
    pub fn data_copy_mem(mem: *const c_char, len: c_int) -> data; pub fn data_copy_escape_string(s: *const c_char, len: c_int) -> data; pub fn data_copy_file(f: *mut FILE, len: usize) -> data;
    pub fn data_append_data(d: data, p: *const c_void, len: c_int) -> data; pub fn data_insert_at_marker(d: data, m: *mut marker, p: *const c_void, len: c_int) -> data; pub fn data_merge(d1: data, d2: data) -> data; pub fn data_append_cell(d: data, word: cell_t) -> data; pub fn data_append_integer(d: data, word: u64, bits: c_int) -> data; pub fn data_append_re(d: data, address: u64, size: u64) -> data; pub fn data_append_addr(d: data, addr: u64) -> data; pub fn data_append_byte(d: data, byte: u8) -> data; pub fn data_append_zeroes(d: data, len: c_int) -> data; pub fn data_append_align(d: data, align: c_int) -> data; pub fn data_insert_data(d: data, m: *mut marker, old: data) -> data;
    pub fn alloc_marker(offset: c_uint, type_: markertype, ref_: *mut c_char) -> *mut marker; pub fn data_add_marker(d: data, type_: markertype, ref_: *mut c_char) -> data; pub fn data_is_one_string(d: data) -> bool;
}

#[repr(C)] pub struct label { pub deleted: bool, pub label: *mut c_char, pub next: *mut label }
#[repr(C)] pub struct bus_type { pub name: *const c_char }
#[repr(C)] pub struct srcpos { _private: [u8; 0] }
#[repr(C)] pub struct property { pub deleted: bool, pub name: *mut c_char, pub val: data, pub next: *mut property, pub labels: *mut label, pub srcpos: *mut srcpos }
#[repr(C)] pub struct node { pub deleted: bool, pub name: *mut c_char, pub proplist: *mut property, pub children: *mut node, pub parent: *mut node, pub next_sibling: *mut node, pub fullpath: *mut c_char, pub basenamelen: usize, pub phandle: cell_t, pub addr_cells: c_int, pub size_cells: c_int, pub labels: *mut label, pub bus: *const bus_type, pub srcpos: *mut srcpos, pub omit_if_unused: bool, pub is_referenced: bool }

pub const MAX_PROPNAME_LEN: usize = 31; pub const MAX_NODENAME_LEN: usize = 31;
pub const DTSF_V1: c_uint = 0x0001; pub const DTSF_PLUGIN: c_uint = 0x0002;

#[repr(C)] pub struct reserve_info { pub address: u64, pub size: u64, pub next: *mut reserve_info, pub labels: *mut label }
#[repr(C)] pub struct dt_info { pub dtsflags: c_uint, pub reservelist: *mut reserve_info, pub boot_cpuid_phys: u32, pub dt: *mut node, pub outname: *const c_char }

extern "C" {
    pub fn add_label(labels: *mut *mut label, label: *mut c_char); pub fn delete_labels(labels: *mut *mut label);
    pub fn build_property(name: *const c_char, val: data, srcpos: *mut srcpos) -> *mut property; pub fn build_property_delete(name: *const c_char) -> *mut property; pub fn chain_property(first: *mut property, list: *mut property) -> *mut property; pub fn reverse_properties(first: *mut property) -> *mut property;
    pub fn build_node(proplist: *mut property, children: *mut node, srcpos: *mut srcpos) -> *mut node; pub fn build_node_delete(srcpos: *mut srcpos) -> *mut node; pub fn name_node(node: *mut node, name: *const c_char) -> *mut node; pub fn omit_node_if_unused(node: *mut node) -> *mut node; pub fn reference_node(node: *mut node) -> *mut node; pub fn chain_node(first: *mut node, list: *mut node) -> *mut node; pub fn merge_nodes(old_node: *mut node, new_node: *mut node) -> *mut node; pub fn add_orphan_node(old_node: *mut node, new_node: *mut node, ref_: *mut c_char) -> *mut node;
    pub fn add_property(node: *mut node, prop: *mut property); pub fn delete_property_by_name(node: *mut node, name: *mut c_char); pub fn delete_property(prop: *mut property); pub fn add_child(parent: *mut node, child: *mut node); pub fn delete_node_by_name(parent: *mut node, name: *mut c_char); pub fn delete_node(node: *mut node); pub fn append_to_property(node: *mut node, name: *mut c_char, data: *const c_void, len: c_int, type_: markertype);
    pub fn get_unitname(node: *mut node) -> *const c_char; pub fn get_property(node: *mut node, propname: *const c_char) -> *mut property; pub fn propval_cell(prop: *mut property) -> cell_t; pub fn propval_cell_n(prop: *mut property, n: c_uint) -> cell_t; pub fn get_property_by_label(tree: *mut node, label: *const c_char, node: *mut *mut node) -> *mut property; pub fn get_marker_label(tree: *mut node, label: *const c_char, node: *mut *mut node, prop: *mut *mut property) -> *mut marker; pub fn get_subnode(node: *mut node, nodename: *const c_char) -> *mut node; pub fn get_node_by_path(tree: *mut node, path: *const c_char) -> *mut node; pub fn get_node_by_label(tree: *mut node, label: *const c_char) -> *mut node; pub fn get_node_by_phandle(tree: *mut node, phandle: cell_t) -> *mut node; pub fn get_node_by_ref(tree: *mut node, ref_: *const c_char) -> *mut node; pub fn get_node_phandle(root: *mut node, node: *mut node) -> cell_t; pub fn guess_boot_cpuid(tree: *mut node) -> u32;
    pub fn build_reserve_entry(start: u64, len: u64) -> *mut reserve_info; pub fn chain_reserve_entry(first: *mut reserve_info, list: *mut reserve_info) -> *mut reserve_info; pub fn add_reserve_entry(list: *mut reserve_info, new_: *mut reserve_info) -> *mut reserve_info;
    pub fn build_dt_info(dtsflags: c_uint, reservelist: *mut reserve_info, tree: *mut node, boot_cpuid_phys: u32) -> *mut dt_info; pub fn sort_tree(dti: *mut dt_info); pub fn generate_labels_from_tree(dti: *mut dt_info, name: *const c_char); pub fn generate_label_tree(dti: *mut dt_info, name: *const c_char, allocph: bool); pub fn generate_fixups_tree(dti: *mut dt_info, name: *const c_char); pub fn fixup_phandles(dti: *mut dt_info, name: *const c_char); pub fn generate_local_fixups_tree(dti: *mut dt_info, name: *const c_char); pub fn local_fixup_phandles(dti: *mut dt_info, name: *const c_char);
    pub fn parse_checks_option(warn: bool, error: bool, arg: *const c_char); pub fn process_checks(force: bool, dti: *mut dt_info); pub fn dt_to_blob(f: *mut FILE, dti: *mut dt_info, version: c_int); pub fn dt_to_asm(f: *mut FILE, dti: *mut dt_info, version: c_int); pub fn dt_from_blob(fname: *const c_char) -> *mut dt_info; pub fn property_add_marker(prop: *mut property, type_: markertype, offset: c_uint, ref_: *mut c_char); pub fn add_phandle_marker(dti: *mut dt_info, prop: *mut property, offset: c_uint); pub fn dt_to_source(f: *mut FILE, dti: *mut dt_info); pub fn dt_from_source(f: *const c_char) -> *mut dt_info; pub fn dt_to_yaml(f: *mut FILE, dti: *mut dt_info); pub fn dt_from_fs(dirname: *const c_char) -> *mut dt_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
