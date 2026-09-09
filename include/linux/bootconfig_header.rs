/* SPDX-License-Identifier: GPL-2.0 */
/* Extra Boot Config. Translated from linux/bootconfig.h. */

use core::ffi::{c_char, c_void};

pub const BOOTCONFIG_MAGIC: &str = "#BOOTCONFIG\n";
pub const BOOTCONFIG_MAGIC_LEN: usize = 12;
pub const BOOTCONFIG_ALIGN_SHIFT: usize = 2;
pub const BOOTCONFIG_ALIGN: usize = 1usize << BOOTCONFIG_ALIGN_SHIFT;
pub const BOOTCONFIG_ALIGN_MASK: usize = BOOTCONFIG_ALIGN - 1;

/// Calculate checksum of bootconfig data.
#[inline]
pub unsafe fn xbc_calc_checksum(data: *const c_void, mut size: u32) -> u32 {
    let mut p = data as *const u8;
    let mut ret = 0u32;
    while size != 0 {
        ret = ret.wrapping_add(*p);
        p = p.add(1);
        size -= 1;
    }
    ret
}

/* XBC tree node */
#[repr(C, packed)]
pub struct xbc_node {
    pub next: u16,
    pub child: u16,
    pub parent: u16,
    pub data: u16,
}

pub const XBC_KEY: u16 = 0;
pub const XBC_VALUE: u16 = 1 << 15;
pub const XBC_DATA_MAX: u16 = XBC_VALUE - 1;
pub const XBC_NODE_MAX: usize = 8192;
pub const XBC_KEYLEN_MAX: usize = 256;
pub const XBC_DEPTH_MAX: usize = 16;

extern "C" {
    pub fn cmdline_has_extra_options() -> bool;
    pub fn xbc_root_node() -> *mut xbc_node;
    pub fn xbc_node_index(node: *mut xbc_node) -> u16;
    pub fn xbc_node_get_parent(node: *mut xbc_node) -> *mut xbc_node;
    pub fn xbc_node_get_child(node: *mut xbc_node) -> *mut xbc_node;
    pub fn xbc_node_get_next(node: *mut xbc_node) -> *mut xbc_node;
    pub fn xbc_node_get_data(node: *mut xbc_node) -> *const c_char;
    pub fn xbc_node_find_subkey(parent: *mut xbc_node, key: *const c_char) -> *mut xbc_node;
    pub fn xbc_node_find_value(
        parent: *mut xbc_node,
        key: *const c_char,
        vnode: *mut *mut xbc_node,
    ) -> *const c_char;
    pub fn xbc_node_find_next_leaf(root: *mut xbc_node, leaf: *mut xbc_node) -> *mut xbc_node;
    pub fn xbc_node_find_next_key_value(root: *mut xbc_node, leaf: *mut *mut xbc_node) -> *const c_char;
    pub fn xbc_node_compose_key_after(
        root: *mut xbc_node,
        node: *mut xbc_node,
        buf: *mut c_char,
        size: usize,
    ) -> i32;
    pub fn xbc_snprint_cmdline(buf: *mut c_char, size: usize, root: *mut xbc_node) -> i32;
    pub fn xbc_init(buf: *const c_char, size: usize, emsg: *mut *const c_char, epos: *mut i32) -> i32;
    pub fn xbc_get_info(node_size: *mut i32, data_size: *mut usize) -> i32;
    pub fn _xbc_exit(early: bool);
}

#[inline]
pub unsafe fn xbc_node_is_value(node: *mut xbc_node) -> bool {
    (*node).data & XBC_VALUE != 0
}

#[inline]
pub unsafe fn xbc_node_is_key(node: *mut xbc_node) -> bool {
    !xbc_node_is_value(node)
}

#[inline]
pub unsafe fn xbc_node_is_array(node: *mut xbc_node) -> bool {
    xbc_node_is_value(node) && (*node).child != 0
}

#[inline]
pub unsafe fn xbc_node_is_leaf(node: *mut xbc_node) -> bool {
    xbc_node_is_key(node)
        && ((*node).child == 0 || xbc_node_is_value(xbc_node_get_child(node)))
}

#[inline]
pub unsafe fn xbc_find_value(key: *const c_char, vnode: *mut *mut xbc_node) -> *const c_char {
    xbc_node_find_value(core::ptr::null_mut(), key, vnode)
}

#[inline]
pub unsafe fn xbc_find_node(key: *const c_char) -> *mut xbc_node {
    xbc_node_find_subkey(core::ptr::null_mut(), key)
}

#[inline]
pub unsafe fn xbc_node_get_subkey(node: *mut xbc_node) -> *mut xbc_node {
    let child = xbc_node_get_child(node);
    if !child.is_null() && xbc_node_is_value(child) {
        xbc_node_get_next(child)
    } else {
        child
    }
}

#[inline]
pub unsafe fn xbc_node_compose_key(node: *mut xbc_node, buf: *mut c_char, size: usize) -> i32 {
    xbc_node_compose_key_after(core::ptr::null_mut(), node, buf, size)
}

#[inline]
pub unsafe fn xbc_exit() {
    _xbc_exit(false);
}

#[cfg(feature = "config_boot_config_embed")]
extern "C" {
    pub fn xbc_get_embedded_bootconfig(size: *mut usize) -> *const c_char;
}

#[cfg(not(feature = "config_boot_config_embed"))]
#[inline]
pub unsafe fn xbc_get_embedded_bootconfig(_size: *mut usize) -> *const c_char {
    core::ptr::null()
}

#[cfg(feature = "config_boot_config")]
extern "C" {
    pub fn bootconfig_cmdline_requested(boot_cmdline: *const c_char, end_offset: *mut i32) -> bool;
}

#[cfg(feature = "config_cmdline_from_bootconfig")]
extern "C" {
    pub fn xbc_prepend_embedded_cmdline(dst: *mut c_char, size: usize);
    pub fn xbc_embedded_cmdline_applied() -> bool;
}

#[cfg(not(feature = "config_cmdline_from_bootconfig"))]
#[inline]
pub unsafe fn xbc_prepend_embedded_cmdline(_dst: *mut c_char, _size: usize) {}

#[cfg(not(feature = "config_cmdline_from_bootconfig"))]
#[inline]
pub unsafe fn xbc_embedded_cmdline_applied() -> bool { false }

/* C iteration macros are represented as callback-free iterator helpers. */
#[inline]
pub unsafe fn xbc_array_for_each_value<F: FnMut(*mut xbc_node, *const c_char)>(
    mut anode: *mut xbc_node,
    mut f: F,
) {
    let mut value = xbc_node_get_data(anode);
    while !anode.is_null() {
        f(anode, value);
        anode = xbc_node_get_child(anode);
        value = if !anode.is_null() { xbc_node_get_data(anode) } else { core::ptr::null() };
    }
}

/* Direct Rust forms of the source iteration macros. */
#[macro_export]
macro_rules! xbc_node_for_each_child {
    ($parent:expr, $child:ident, $body:block) => {{
        let mut $child = unsafe { $crate::xbc_node_get_child($parent) };
        while !$child.is_null() {
            $body
            $child = unsafe { $crate::xbc_node_get_next($child) };
        }
    }};
}

#[macro_export]
macro_rules! xbc_node_for_each_subkey {
    ($parent:expr, $child:ident, $body:block) => {{
        let mut $child = unsafe { $crate::xbc_node_get_subkey($parent) };
        while !$child.is_null() {
            $body
            $child = unsafe { $crate::xbc_node_get_next($child) };
        }
    }};
}

#[macro_export]
macro_rules! xbc_node_for_each_key_value {
    ($node:expr, $knode:ident, $value:ident, $body:block) => {{
        let mut $knode: *mut $crate::xbc_node = core::ptr::null_mut();
        let mut $value = unsafe { $crate::xbc_node_find_next_key_value($node, &mut $knode) };
        while !$knode.is_null() {
            $body
            $value = unsafe { $crate::xbc_node_find_next_key_value($node, &mut $knode) };
        }
    }};
}

#[macro_export]
macro_rules! xbc_for_each_key_value {
    ($knode:ident, $value:ident, $body:block) => {
        $crate::xbc_node_for_each_key_value!(core::ptr::null_mut(), $knode, $value, $body)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
