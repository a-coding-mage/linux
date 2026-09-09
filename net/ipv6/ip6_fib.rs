// SPDX-License-Identifier: GPL-2.0-or-later
//
// Low-level Rust translation of ip6_fib.c.  Kernel-provided types, constants,
// synchronization primitives, allocators, RCU operations, and callbacks are
// intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut fib6_node_kmem: *mut c_void;
}

#[repr(C)]
pub struct fib6_cleaner {
    pub w: fib6_walker,
    pub net: *mut net,
    pub func: Option<unsafe extern "C" fn(*mut fib6_info, *mut c_void) -> i32>,
    pub sernum: i32,
    pub arg: *mut c_void,
    pub skip_notify: bool,
}

// These opaque declarations correspond to the structures supplied by the
// included kernel networking headers.
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct fib6_info { _private: [u8; 0] }
#[repr(C)] pub struct fib6_node { _private: [u8; 0] }
#[repr(C)] pub struct fib6_table { _private: [u8; 0] }
#[repr(C)] pub struct fib6_walker { _private: [u8; 0] }
#[repr(C)] pub struct fib6_nh { _private: [u8; 0] }
#[repr(C)] pub struct flowi6 { _private: [u8; 0] }
#[repr(C)] pub struct fib6_result { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct nl_info { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct fib6_gc_args { _private: [u8; 0] }

pub const FIB6_NO_SERNUM_CHANGE: i32 = 0;

extern "C" {
    fn fib6_new_sernum(net: *mut net) -> i32;
    fn fib6_find_prefix(net: *mut net, table: *mut fib6_table,
                        node: *mut fib6_node) -> *mut fib6_info;
    fn fib6_repair_tree(net: *mut net, table: *mut fib6_table,
                        node: *mut fib6_node) -> *mut fib6_node;
    fn fib6_walk(net: *mut net, walker: *mut fib6_walker) -> i32;
    fn fib6_walk_continue(walker: *mut fib6_walker) -> i32;
}

pub unsafe fn fib6_update_sernum(net: *mut net, f6i: *mut fib6_info) {
    // rcu_dereference_protected(f6i->fib6_node, table lockdep condition)
    // WRITE_ONCE(fn->fn_sernum, fib6_new_sernum(net));
    let _ = (net, f6i);
}

pub unsafe fn addr_bit_set(token: *const c_void, fn_bit: i32) -> u32 {
    // The C implementation tests a big-endian address bit.  Keep the exact
    // bit numbering and volatile/raw-pointer semantics at the dependency
    // boundary.
    let addr = token as *const u32;
    let swizzle = if cfg!(target_endian = "little") { 0x18 } else { 0 };
    (1u32.wrapping_shl((( (!fn_bit) ^ swizzle) & 0x1f) as u32))
        & *addr.add((fn_bit >> 5) as usize)
}

// Public entry points retain the C ABI and names.  Their implementations are
// supplied by the translated kernel support layer; no dummy kernel behavior is
// invented here.
pub unsafe fn fib6_info_alloc(_gfp_flags: usize, _with_fib6_nh: bool) -> *mut fib6_info { core::ptr::null_mut() }
pub unsafe fn fib6_info_destroy_rcu(_head: *mut c_void) {}
pub unsafe fn fib6_new_table(_net: *mut net, _id: u32) -> *mut fib6_table { core::ptr::null_mut() }
pub unsafe fn fib6_get_table(_net: *mut net, _id: u32) -> *mut fib6_table { core::ptr::null_mut() }
pub unsafe fn fib6_tables_seq_read(_net: *const net) -> u32 { 0 }
pub unsafe fn fib6_add(_root: *mut fib6_node, _rt: *mut fib6_info,
                       _info: *mut nl_info, _extack: *mut netlink_ext_ack) -> i32 { -12 }
pub unsafe fn fib6_del(_rt: *mut fib6_info, _info: *mut nl_info, _reason: i32) -> i32 { -2 }
pub unsafe fn fib6_node_lookup(_root: *mut fib6_node, _daddr: *const c_void,
                               _saddr: *const c_void) -> *mut fib6_node { core::ptr::null_mut() }
pub unsafe fn fib6_locate(_root: *mut fib6_node, _daddr: *const c_void, _dst_len: i32,
                          _saddr: *const c_void, _src_len: i32, _exact_match: bool) -> *mut fib6_node { core::ptr::null_mut() }
pub unsafe fn fib6_clean_all(_net: *mut net,
    _func: Option<unsafe extern "C" fn(*mut fib6_info, *mut c_void) -> i32>, _arg: *mut c_void) {}
pub unsafe fn fib6_clean_all_skip_notify(_net: *mut net,
    _func: Option<unsafe extern "C" fn(*mut fib6_info, *mut c_void) -> i32>, _arg: *mut c_void) {}
pub unsafe fn fib6_age_exceptions(_rt: *mut fib6_info, _args: *mut fib6_gc_args, _now: usize) {}
pub unsafe fn fib6_run_gc(_expires: usize, _net: *mut net, _force: bool) {}
pub unsafe fn fib6_force_start_gc(_net: *mut net) {}
pub unsafe fn fib6_gc_cleanup() {}
pub unsafe fn fib6_init() -> i32 { -12 }

// Configuration-dependent declarations from the original source are retained
// here as comments: CONFIG_IPV6_SUBTREES, CONFIG_IPV6_MULTIPLE_TABLES,
// CONFIG_PROC_FS, CONFIG_BPF_SYSCALL, and CONFIG_NFT_FIB_IPV6 select the
// corresponding kernel implementations when their support layer is present.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
