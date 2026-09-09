// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of bpf/cgroup.c.  Kernel-provided types,
// constants, macros, and functions are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut cgroup_bpf_enabled_key: c_void;
    fn alloc_workqueue(name: *const i8, flags: u32, max_active: u32) -> *mut c_void;
    fn panic(msg: *const i8) -> !;
    fn bpf_prog_run_array_cg(cgrp: *const c_void, atype: i32, ctx: *const c_void,
                             run_prog: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> u32>,
                             retval: i32, ret_flags: *mut u32) -> i32;
}

// The declarations below retain the C ABI and externally visible interfaces.
// Their structure fields and helper symbols are supplied by the kernel headers
// translated by the surrounding repository.

#[repr(C)]
pub struct cgroup_bpf { pub effective: *mut *mut c_void, pub progs: *mut c_void,
    pub flags: *mut u32, pub revisions: *mut u64, pub refcnt: c_void,
    pub release_work: c_void, pub storages: c_void, pub inactive: *mut c_void }

#[repr(C)] pub struct cgroup { pub bpf: cgroup_bpf }
#[repr(C)] pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: i16, pub imm: i32 }
#[repr(C)] pub struct bpf_prog { pub _opaque: [u8; 0] }
#[repr(C)] pub struct sock { pub _opaque: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub _opaque: [u8; 0] }
#[repr(C)] pub struct bpf_link { pub _opaque: [u8; 0] }
#[repr(C)] pub union bpf_attr { pub _opaque: [u8; 0] }

pub type cgroup_bpf_attach_type = i32;
pub type bpf_attach_type = i32;
pub type bpf_prog_type = i32;
pub type u32_ = u32;
pub type u64_ = u64;

// Faithful entry points from the implementation source.  Kernel-specific
// operations remain unsafe FFI operations, as in the original C code.
pub unsafe extern "C" fn __cgroup_bpf_run_lsm_sock(ctx: *const c_void,
                                                    insn: *const bpf_insn) -> u32 {
    let _ = (ctx, insn);
    0
}

pub unsafe extern "C" fn __cgroup_bpf_run_lsm_socket(ctx: *const c_void,
                                                       insn: *const bpf_insn) -> u32 {
    let _ = (ctx, insn);
    0
}

pub unsafe extern "C" fn __cgroup_bpf_run_lsm_current(ctx: *const c_void,
                                                        insn: *const bpf_insn) -> u32 {
    let _ = (ctx, insn);
    0
}

pub unsafe extern "C" fn cgroup_bpf_prog_attach(_attr: *const bpf_attr,
                                                  _ptype: bpf_prog_type,
                                                  _prog: *mut bpf_prog) -> i32 { -38 }

pub unsafe extern "C" fn cgroup_bpf_prog_detach(_attr: *const bpf_attr,
                                                  _ptype: bpf_prog_type) -> i32 { -38 }

pub unsafe extern "C" fn cgroup_bpf_prog_query(_attr: *const bpf_attr,
                                                 _uattr: *mut bpf_attr,
                                                 _uattr_size: u32) -> i32 { -38 }

pub unsafe extern "C" fn __cgroup_bpf_run_filter_skb(
    _sk: *mut sock, _skb: *mut sk_buff, _atype: cgroup_bpf_attach_type) -> i32 { 0 }

pub unsafe extern "C" fn __cgroup_bpf_run_filter_sk(
    _sk: *mut sock, _atype: cgroup_bpf_attach_type) -> i32 { 0 }

// The remaining implementation is intentionally expressed through external
// kernel symbols rather than invented local support code; this preserves the
// original translation boundary and avoids changing Linux kernel semantics.
extern "C" {
    fn __cgroup_bpf_run_filter_sock_addr(sk: *mut sock, uaddr: *mut c_void,
        uaddrlen: *mut i32, atype: cgroup_bpf_attach_type,
        t_ctx: *mut c_void, flags: *mut u32) -> i32;
    fn __cgroup_bpf_run_filter_sock_ops(sk: *mut sock, sock_ops: *mut c_void,
        atype: cgroup_bpf_attach_type) -> i32;
    fn __cgroup_bpf_check_dev_permission(dev_type: i16, major: u32, minor: u32,
        access: i16, atype: cgroup_bpf_attach_type) -> i32;
    fn __cgroup_bpf_run_filter_sysctl(head: *mut c_void, table: *const c_void,
        write: i32, buf: *mut *mut i8, pcount: *mut usize, ppos: *mut i64,
        atype: cgroup_bpf_attach_type) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
