// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/srcline.h.
// C includes removed; this header depends on linux/list.h, linux/rbtree.h,
// linux/types.h, and forward declarations for struct dso and struct symbol.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u64 = u64;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

// Opaque translations of externally supplied Linux list/rbtree types.
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut srcline_full_filename: bool;

    pub fn get_srcline(
        dso: *mut dso,
        addr: u64,
        sym: *mut symbol,
        show_sym: bool,
        show_addr: bool,
        ip: u64,
    ) -> *mut c_char;

    pub fn __get_srcline(
        dso: *mut dso,
        addr: u64,
        sym: *mut symbol,
        show_sym: bool,
        show_addr: bool,
        unwind_inlines: bool,
        ip: u64,
    ) -> *mut c_char;

    pub fn zfree_srcline(srcline: *mut *mut c_char);

    pub fn get_srcline_split(dso: *mut dso, addr: u64, line: *mut c_uint) -> *mut c_char;

    // insert the srcline into the DSO, which will take ownership
    pub fn srcline__tree_insert(tree: *mut rb_root_cached, addr: u64, srcline: *mut c_char);

    // find previously inserted srcline
    pub fn srcline__tree_find(tree: *mut rb_root_cached, addr: u64) -> *mut c_char;

    // delete all srclines within the tree
    pub fn srcline__tree_delete(tree: *mut rb_root_cached);

    pub static mut srcline__unknown: *mut c_char;
}

pub const SRCLINE_UNKNOWN: *mut c_char = unsafe { srcline__unknown };

pub const MAX_INLINE_NEST: usize = 1024;

#[repr(C)]
pub struct inline_list {
    pub symbol: *mut symbol,
    pub srcline: *mut c_char,
    pub list: list_head,
}

#[repr(C)]
pub struct inline_node {
    pub addr: u64,
    pub val: list_head,
    pub rb_node: rb_node,
}

unsafe extern "C" {
    // parse inlined frames for the given address
    pub fn dso__parse_addr_inlines(
        dso: *mut dso,
        addr: u64,
        sym: *mut symbol,
    ) -> *mut inline_node;

    // free resources associated to the inline node list
    pub fn inline_node__delete(node: *mut inline_node);
    pub fn inline_node__clear_frames(node: *mut inline_node);

    // insert the inline node list into the DSO, which will take ownership
    pub fn inlines__tree_insert(tree: *mut rb_root_cached, inlines: *mut inline_node);

    // find previously inserted inline node list
    pub fn inlines__tree_find(tree: *mut rb_root_cached, addr: u64) -> *mut inline_node;

    // delete all nodes within the tree of inline_node s
    pub fn inlines__tree_delete(tree: *mut rb_root_cached);

    pub fn inline_list__append(
        symbol: *mut symbol,
        srcline: *mut c_char,
        node: *mut inline_node,
    ) -> c_int;

    pub fn inline_list__append_tail(
        symbol: *mut symbol,
        srcline: *mut c_char,
        node: *mut inline_node,
    ) -> c_int;

    pub fn srcline_from_fileline(file: *const c_char, line: c_uint) -> *mut c_char;

    pub fn new_inline_sym(
        dso: *mut dso,
        base_sym: *mut symbol,
        funcname: *const c_char,
    ) -> *mut symbol;

    pub fn addr2line_configure(
        var: *const c_char,
        value: *const c_char,
        cb: *mut c_void,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
