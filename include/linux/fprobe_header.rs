/* SPDX-License-Identifier: GPL-2.0 */
/* Simple ftrace probe wrapper */

// Dependencies supplied by the surrounding kernel translation:
// linux/compiler.h, linux/ftrace.h, linux/rcupdate.h, linux/refcount.h,
// linux/rhashtable.h, and linux/slab.h.

use core::ffi::c_void;

pub struct fprobe;
pub struct ftrace_regs;
pub struct rhlist_head;
pub struct hlist_node;
pub struct rcu_head;

pub type fprobe_entry_cb = unsafe extern "C" fn(
    fp: *mut fprobe,
    entry_ip: c_ulong,
    ret_ip: c_ulong,
    regs: *mut ftrace_regs,
    entry_data: *mut c_void,
) -> c_int;

pub type fprobe_exit_cb = unsafe extern "C" fn(
    fp: *mut fprobe,
    entry_ip: c_ulong,
    ret_ip: c_ulong,
    regs: *mut ftrace_regs,
    entry_data: *mut c_void,
);

#[repr(C)]
pub struct fprobe_hlist_node {
    pub hlist: rhlist_head,
    pub addr: c_ulong,
    pub fp: *mut fprobe,
}

#[repr(C)]
pub struct fprobe_hlist {
    pub hlist: hlist_node,
    pub rcu: rcu_head,
    pub fp: *mut fprobe,
    pub size: c_int,
    // Flexible array member: one node for each address to probe; its length is `size`.
    pub array: [fprobe_hlist_node; 0],
}

#[repr(C)]
pub struct fprobe {
    pub nmissed: c_ulong,
    pub flags: c_uint,
    pub entry_data_size: usize,
    pub entry_handler: Option<fprobe_entry_cb>,
    pub exit_handler: Option<fprobe_exit_cb>,
    pub hlist_array: *mut fprobe_hlist,
}

pub const FPROBE_FL_DISABLED: c_uint = 1;
pub const FPROBE_FL_KPROBE_SHARED: c_uint = 2;

#[inline]
pub unsafe fn fprobe_disabled(fp: *mut fprobe) -> bool {
    !fp.is_null() && ((*fp).flags & FPROBE_FL_DISABLED) != 0
}

#[inline]
pub unsafe fn fprobe_shared_with_kprobes(fp: *mut fprobe) -> bool {
    !fp.is_null() && ((*fp).flags & FPROBE_FL_KPROBE_SHARED) != 0
}

#[cfg(feature = "CONFIG_FPROBE")]
unsafe extern "C" {
    pub fn register_fprobe(
        fp: *mut fprobe,
        filter: *const c_char,
        notfilter: *const c_char,
    ) -> c_int;
    pub fn register_fprobe_ips(fp: *mut fprobe, addrs: *mut c_ulong, num: c_int) -> c_int;
    pub fn register_fprobe_syms(fp: *mut fprobe, syms: *const *const c_char, num: c_int) -> c_int;
    pub fn unregister_fprobe(fp: *mut fprobe) -> c_int;
    pub fn unregister_fprobe_async(fp: *mut fprobe) -> c_int;
    pub fn fprobe_is_registered(fp: *mut fprobe) -> bool;
    pub fn fprobe_count_ips_from_filter(filter: *const c_char, notfilter: *const c_char) -> c_int;
}

#[cfg(not(feature = "CONFIG_FPROBE"))]
#[inline]
pub unsafe fn register_fprobe(
    _fp: *mut fprobe,
    _filter: *const c_char,
    _notfilter: *const c_char,
) -> c_int {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_FPROBE"))]
#[inline]
pub unsafe fn register_fprobe_ips(_fp: *mut fprobe, _addrs: *mut c_ulong, _num: c_int) -> c_int {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_FPROBE"))]
#[inline]
pub unsafe fn register_fprobe_syms(
    _fp: *mut fprobe,
    _syms: *const *const c_char,
    _num: c_int,
) -> c_int {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_FPROBE"))]
#[inline]
pub unsafe fn unregister_fprobe(_fp: *mut fprobe) -> c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_FPROBE"))]
#[inline]
pub unsafe fn unregister_fprobe_async(_fp: *mut fprobe) -> c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_FPROBE"))]
#[inline]
pub unsafe fn fprobe_is_registered(_fp: *mut fprobe) -> bool { false }

#[cfg(not(feature = "CONFIG_FPROBE"))]
#[inline]
pub unsafe fn fprobe_count_ips_from_filter(
    _filter: *const c_char,
    _notfilter: *const c_char,
) -> c_int {
    -EOPNOTSUPP
}

#[inline]
pub unsafe fn disable_fprobe(fp: *mut fprobe) {
    if !fp.is_null() {
        (*fp).flags |= FPROBE_FL_DISABLED;
    }
}

#[inline]
pub unsafe fn enable_fprobe(fp: *mut fprobe) {
    if !fp.is_null() {
        (*fp).flags &= !FPROBE_FL_DISABLED;
    }
}

pub const FPROBE_DATA_SIZE_BITS: usize = 4;
pub const MAX_FPROBE_DATA_SIZE_WORD: usize = (1usize << FPROBE_DATA_SIZE_BITS) - 1;
pub const MAX_FPROBE_DATA_SIZE: usize = MAX_FPROBE_DATA_SIZE_WORD * core::mem::size_of::<c_long>();

// C integer and errno names supplied by the surrounding kernel translation.
type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_long = isize;
type c_ulong = usize;
extern "C" {
    static EOPNOTSUPP: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
