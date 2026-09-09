// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/dcache.c -- source-level Rust translation.
 *
 * Kernel types, synchronization primitives, allocation routines, and helper
 * macros referenced below are supplied by the surrounding kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Linux-kernel dependency declarations (provided by other translated units).
extern "C" {
    static mut sysctl_vfs_cache_pressure: c_int;
    static mut sysctl_vfs_cache_pressure_denom: c_int;
    static mut dentry_hashtable: *mut hlist_bl_head;
    static mut d_hash_shift: u32;
    fn mult_frac(val: c_ulong, mul: c_int, div: c_int) -> c_ulong;
    fn runtime_const_ptr<T>(p: *mut T) -> *mut T;
    fn runtime_const_shift_right_32(v: c_ulong, shift: u32) -> usize;
    fn hash_32(v: u32, bits: u32) -> u32;
    fn this_cpu_inc(v: *mut c_long);
    fn this_cpu_dec(v: *mut c_long);
    fn per_cpu(v: *mut c_long, cpu: c_int) -> c_long;
    fn for_each_possible_cpu(cpu: *mut c_int);
    fn proc_doulongvec_minmax(t: *const ctl_table, w: c_int, b: *mut c_void,
                               l: *mut usize, p: *mut loff_t) -> c_int;
    fn proc_dointvec_minmax(t: *const ctl_table, w: c_int, b: *mut c_void,
                            l: *mut usize, p: *mut loff_t) -> c_int;
    fn register_sysctl_init(name: *const c_char, table: *const ctl_table);
}

type c_long = isize;
type loff_t = i64;

#[repr(C)]
pub struct hlist_bl_head { pub first: *mut c_void } 
#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, c_int, *mut c_void, *mut usize, *mut loff_t) -> c_int>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

#[repr(C)]
pub struct dentry_stat_t {
    pub nr_dentry: c_long,
    pub nr_unused: c_long,
    pub age_limit: c_long,
    pub want_pages: c_long,
    pub nr_negative: c_long,
    pub dummy: c_long,
}

static mut DENTRY_STAT: dentry_stat_t = dentry_stat_t {
    nr_dentry: 0, nr_unused: 0, age_limit: 45, want_pages: 0,
    nr_negative: 0, dummy: 0,
};
static mut NR_DENTRY: c_long = 0;
static mut NR_DENTRY_UNUSED: c_long = 0;
static mut NR_DENTRY_NEGATIVE: c_long = 0;
static mut DENTRY_NEGATIVE_POLICY: c_int = 0;

#[inline]
pub unsafe extern "C" fn vfs_pressure_ratio(val: c_ulong) -> c_ulong {
    mult_frac(val, sysctl_vfs_cache_pressure, sysctl_vfs_cache_pressure_denom)
}

#[inline]
unsafe fn d_hash(hashlen: c_ulong) -> *mut hlist_bl_head {
    runtime_const_ptr(dentry_hashtable).add(
        runtime_const_shift_right_32(hashlen, d_hash_shift))
}

const IN_LOOKUP_SHIFT: u32 = 10;
static mut IN_LOOKUP_HASHTABLE: [hlist_bl_head; 1 << IN_LOOKUP_SHIFT] =
    [hlist_bl_head { first: core::ptr::null_mut() }; 1 << IN_LOOKUP_SHIFT];

#[repr(C)]
pub struct qstr { pub hash_len: u64, pub name: *const u8 }

#[inline]
unsafe fn in_lookup_hash(parent: *const dentry, mut hash: u32) -> *mut hlist_bl_head {
    hash = hash.wrapping_add(parent as usize as c_ulong as u32);
    IN_LOOKUP_HASHTABLE.as_mut_ptr().add(hash_32(hash, IN_LOOKUP_SHIFT) as usize)
}

#[repr(C)]
pub struct dentry { pub d_name: qstr }

#[inline]
unsafe fn dentry_string_cmp(mut cs: *const u8, mut ct: *const u8, mut tcount: u32) -> c_int {
    while tcount != 0 {
        if *cs != *ct { return 1; }
        cs = cs.add(1); ct = ct.add(1); tcount -= 1;
    }
    0
}

#[inline]
unsafe fn dentry_cmp(d: *const dentry, ct: *const u8, tcount: u32) -> c_int {
    dentry_string_cmp((*d).d_name.name, ct, tcount)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
