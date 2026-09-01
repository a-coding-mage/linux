/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2024 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2024 David Vernet <dvernet@meta.com>
 */

/* C dependencies removed from executable Rust:
 * <bpf/btf.h>, <bpf/libbpf.h>, <fcntl.h>, <stdlib.h>, <unistd.h>
 */

use core::ffi::{c_char, c_int, c_long, c_void};

pub type u64 = u64;
pub type s32 = i32;
pub type __u32 = u32;
pub type ssize_t = isize;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_enum {
    pub name_off: __u32,
    pub val: c_int,
}

#[repr(C)]
pub struct btf_enum64 {
    pub name_off: __u32,
    _private: [u8; 0],
}

unsafe extern "C" {
    #[linkage = "extern_weak"]
    pub static mut __COMPAT_vmlinux_btf: *mut btf;

    pub fn btf__load_vmlinux_btf() -> *mut btf;
    pub fn btf__find_by_name(btf: *const btf, name: *const c_char) -> c_int;
    pub fn btf__find_by_name_kind(btf: *const btf, name: *const c_char, kind: c_int) -> c_int;
    pub fn btf__type_by_id(btf: *const btf, id: c_int) -> *const btf_type;
    pub fn btf_is_enum(t: *const btf_type) -> bool;
    pub fn btf_is_enum64(t: *const btf_type) -> bool;
    pub fn btf_enum(t: *const btf_type) -> *mut btf_enum;
    pub fn btf_enum64(t: *const btf_type) -> *mut btf_enum64;
    pub fn btf_vlen(t: *const btf_type) -> __u32;
    pub fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    pub fn btf_enum64_value(e: *const btf_enum64) -> u64;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    pub fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    pub fn close(fd: c_int) -> c_int;
    pub fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;

    pub static mut errno: c_int;
    pub static mut stderr: *mut c_void;
}

pub const BTF_KIND_STRUCT: c_int = 4;
pub const O_RDONLY: c_int = 0;
pub const ENOENT: c_int = 2;
pub const ERANGE: c_int = 34;

/* External assertion / reporting macros from included project headers. */
macro_rules! SCX_BUG_ON {
    ($cond:expr, $($arg:tt)*) => {
        if $cond {
            panic!($($arg)*);
        }
    };
}

macro_rules! SCX_ENUM_INIT {
    ($($arg:tt)*) => {
        compile_error!("SCX_ENUM_INIT is supplied by another translated header")
    };
}

pub unsafe fn __COMPAT_load_vmlinux_btf() {
    unsafe {
        if __COMPAT_vmlinux_btf.is_null() {
            __COMPAT_vmlinux_btf = btf__load_vmlinux_btf();
            SCX_BUG_ON!(
                __COMPAT_vmlinux_btf.is_null(),
                "btf__load_vmlinux_btf()"
            );
        }
    }
}

pub unsafe fn __COMPAT_read_enum(
    type_: *const c_char,
    name: *const c_char,
    v: *mut u64,
) -> bool {
    let t: *const btf_type;
    let mut n: *const c_char;
    let tid: s32;
    let mut i: __u32;

    unsafe {
        __COMPAT_load_vmlinux_btf();

        tid = btf__find_by_name(__COMPAT_vmlinux_btf, type_);
        if tid < 0 {
            return false;
        }

        t = btf__type_by_id(__COMPAT_vmlinux_btf, tid);
        SCX_BUG_ON!((t.is_null()), "btf__type_by_id({})", tid);

        if btf_is_enum(t) {
            let e: *mut btf_enum = btf_enum(t);

            i = 0;
            while i < btf_vlen(t) {
                n = btf__name_by_offset(__COMPAT_vmlinux_btf, (*e.add(i as usize)).name_off);
                SCX_BUG_ON!(n.is_null(), "btf__name_by_offset()");
                if strcmp(n, name) == 0 {
                    *v = (*e.add(i as usize)).val as u64;
                    return true;
                }
                i += 1;
            }
        } else if btf_is_enum64(t) {
            let e: *mut btf_enum64 = btf_enum64(t);

            i = 0;
            while i < btf_vlen(t) {
                n = btf__name_by_offset(__COMPAT_vmlinux_btf, (*e.add(i as usize)).name_off);
                SCX_BUG_ON!(n.is_null(), "btf__name_by_offset()");
                if strcmp(n, name) == 0 {
                    *v = btf_enum64_value(e.add(i as usize));
                    return true;
                }
                i += 1;
            }
        }

        false
    }
}

macro_rules! __COMPAT_ENUM_OR_ZERO {
    ($type_:expr, $ent:expr) => {{
        let mut __val: u64 = 0;
        unsafe {
            __COMPAT_read_enum($type_, $ent, &mut __val);
        }
        __val
    }};
}

pub unsafe fn __COMPAT_has_ksym(ksym: *const c_char) -> bool {
    unsafe {
        __COMPAT_load_vmlinux_btf();
        btf__find_by_name(__COMPAT_vmlinux_btf, ksym) >= 0
    }
}

pub unsafe fn __COMPAT_struct_has_field(type_: *const c_char, field: *const c_char) -> bool {
    let t: *const btf_type;
    let m: *const btf_member;
    let mut n: *const c_char;
    let tid: s32;
    let mut i: __u32;

    unsafe {
        __COMPAT_load_vmlinux_btf();
        tid = btf__find_by_name_kind(__COMPAT_vmlinux_btf, type_, BTF_KIND_STRUCT);
        if tid < 0 {
            return false;
        }

        t = btf__type_by_id(__COMPAT_vmlinux_btf, tid);
        SCX_BUG_ON!(t.is_null(), "btf__type_by_id({})", tid);

        m = btf_members(t);

        i = 0;
        while i < btf_vlen(t) {
            n = btf__name_by_offset(__COMPAT_vmlinux_btf, (*m.add(i as usize)).name_off);
            SCX_BUG_ON!(n.is_null(), "btf__name_by_offset()");
            if strcmp(n, field) == 0 {
                return true;
            }
            i += 1;
        }

        false
    }
}

#[repr(C)]
pub struct btf_member {
    pub name_off: __u32,
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn btf_members(t: *const btf_type) -> *const btf_member;
}

macro_rules! SCX_OPS_FLAG {
    ($name:ident) => {
        __COMPAT_ENUM_OR_ZERO!(c"scx_ops_flags".as_ptr(), c stringify!($name).as_ptr())
    };
}

macro_rules! SCX_OPS_KEEP_BUILTIN_IDLE {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_KEEP_BUILTIN_IDLE)
    };
}
macro_rules! SCX_OPS_ENQ_LAST {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_ENQ_LAST)
    };
}
macro_rules! SCX_OPS_ENQ_EXITING {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_ENQ_EXITING)
    };
}
macro_rules! SCX_OPS_SWITCH_PARTIAL {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_SWITCH_PARTIAL)
    };
}
macro_rules! SCX_OPS_ENQ_MIGRATION_DISABLED {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_ENQ_MIGRATION_DISABLED)
    };
}
macro_rules! SCX_OPS_ALLOW_QUEUED_WAKEUP {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_ALLOW_QUEUED_WAKEUP)
    };
}
macro_rules! SCX_OPS_BUILTIN_IDLE_PER_NODE {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_BUILTIN_IDLE_PER_NODE)
    };
}
macro_rules! SCX_OPS_ALWAYS_ENQ_IMMED {
    () => {
        SCX_OPS_FLAG!(SCX_OPS_ALWAYS_ENQ_IMMED)
    };
}

macro_rules! SCX_PICK_IDLE_FLAG {
    ($name:ident) => {
        __COMPAT_ENUM_OR_ZERO!(c"scx_pick_idle_cpu_flags".as_ptr(), c stringify!($name).as_ptr())
    };
}

macro_rules! SCX_PICK_IDLE_CORE {
    () => {
        SCX_PICK_IDLE_FLAG!(SCX_PICK_IDLE_CORE)
    };
}
macro_rules! SCX_PICK_IDLE_IN_NODE {
    () => {
        SCX_PICK_IDLE_FLAG!(SCX_PICK_IDLE_IN_NODE)
    };
}

pub unsafe fn scx_hotplug_seq() -> c_long {
    let fd: c_int;
    let mut buf = [0 as c_char; 32];
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let len: ssize_t;
    let val: c_long;

    unsafe {
        fd = open(c"/sys/kernel/sched_ext/hotplug_seq".as_ptr(), O_RDONLY);
        if fd < 0 {
            return -ENOENT as c_long;
        }

        len = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);
        SCX_BUG_ON!(len <= 0, "read failed ({})", len);
        buf[len as usize] = 0;
        close(fd);

        errno = 0;
        val = strtoul(buf.as_ptr(), &mut endptr, 10);
        SCX_BUG_ON!(
            errno == ERANGE
                || endptr == buf.as_mut_ptr()
                || (*endptr != b'\n' as c_char && *endptr != b'\0' as c_char),
            "invalid num hotplug events: {}",
            val
        );

        val
    }
}

/*
 * Open the sched_ext_ops skeleton.
 *
 * struct sched_ext_ops can change over time. Two complementary mechanisms
 * keep BPF schedulers built against newer headers running on older kernels:
 *
 * 1. Load-time fix-up (SCX_OPS_OPEN()). For each optional ops callback or field
 *    added to struct sched_ext_ops, an explicit stanza below probes the
 *    running kernel's BTF via __COMPAT_struct_has_field() and, if the field
 *    is missing, clears it in the in-memory struct_ops (with a warning to
 *    stderr) before load. Handles additive changes - a new stanza must be
 *    added here for each new optional field.
 *
 * 2. Multi-variant struct_ops via compat.bpf.h::SCX_OPS_DEFINE(). That
 *    macro can be expanded to emit several variants of struct sched_ext_ops,
 *    and SCX_OPS_LOAD()/ATTACH() can pick the right one based on what the
 *    kernel supports. Needed when an existing operation has to change
 *    incompatibly (e.g. a callback signature changes); the load-time
 *    fix-up above only handles purely additive changes.
 *
 * ec7e3b0463e1 ("implement-ops") in https://github.com/sched-ext/sched_ext is
 * the current minimum required kernel version.
 *
 * COMPAT:
 * - v6.17: ops.cgroup_set_bandwidth()
 * - v6.19: ops.cgroup_set_idle()
 * - v7.1:  ops.sub_attach(), ops.sub_detach(), ops.sub_cgroup_id
 * - v7.3:  ops.rescue_bandwidth_ppt, ops.rescue_quantum_us
 */
macro_rules! __SCX_OPS_OPEN {
    ($ops_name:ident, $scx_name:ident, $ops_struct:expr) => {{
        let __oskel;

        SCX_BUG_ON!(
            unsafe { !__COMPAT_struct_has_field($ops_struct.as_ptr(), c"dump".as_ptr()) },
            "{}.dump() missing, kernel too old?",
            $ops_struct.to_str().unwrap_or("<invalid>")
        );

        paste::paste! {
            __oskel = unsafe { [<$scx_name __open>]() };
        }
        SCX_BUG_ON!(__oskel.is_null(), "Could not open {}", stringify!($scx_name));
        unsafe {
            (*(*__oskel).struct_ops.$ops_name).hotplug_seq = scx_hotplug_seq();
        }
        SCX_ENUM_INIT!(__oskel);
        __oskel
    }};
}

unsafe extern "C" {
    pub fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

macro_rules! SCX_OPS_OPEN {
    ($ops_name:ident, $scx_name:ident) => {{
        let __skel;

        __skel = __SCX_OPS_OPEN!($ops_name, $scx_name, c"sched_ext_ops");
        unsafe {
            if !(*(*__skel).struct_ops.$ops_name).cgroup_set_bandwidth.is_null()
                && !__COMPAT_struct_has_field(
                    c"sched_ext_ops".as_ptr(),
                    c"cgroup_set_bandwidth".as_ptr(),
                )
            {
                fprintf(
                    stderr,
                    c"WARNING: kernel doesn't support ops.cgroup_set_bandwidth()\n".as_ptr(),
                );
                (*(*__skel).struct_ops.$ops_name).cgroup_set_bandwidth = core::ptr::null_mut();
            }
            if !(*(*__skel).struct_ops.$ops_name).cgroup_set_idle.is_null()
                && !__COMPAT_struct_has_field(c"sched_ext_ops".as_ptr(), c"cgroup_set_idle".as_ptr())
            {
                fprintf(
                    stderr,
                    c"WARNING: kernel doesn't support ops.cgroup_set_idle()\n".as_ptr(),
                );
                (*(*__skel).struct_ops.$ops_name).cgroup_set_idle = core::ptr::null_mut();
            }
            if !(*(*__skel).struct_ops.$ops_name).sub_attach.is_null()
                && !__COMPAT_struct_has_field(c"sched_ext_ops".as_ptr(), c"sub_attach".as_ptr())
            {
                fprintf(
                    stderr,
                    c"WARNING: kernel doesn't support ops.sub_attach()\n".as_ptr(),
                );
                (*(*__skel).struct_ops.$ops_name).sub_attach = core::ptr::null_mut();
            }
            if !(*(*__skel).struct_ops.$ops_name).sub_detach.is_null()
                && !__COMPAT_struct_has_field(c"sched_ext_ops".as_ptr(), c"sub_detach".as_ptr())
            {
                fprintf(
                    stderr,
                    c"WARNING: kernel doesn't support ops.sub_detach()\n".as_ptr(),
                );
                (*(*__skel).struct_ops.$ops_name).sub_detach = core::ptr::null_mut();
            }
            if (*(*__skel).struct_ops.$ops_name).sub_cgroup_id > 0
                && !__COMPAT_struct_has_field(c"sched_ext_ops".as_ptr(), c"sub_cgroup_id".as_ptr())
            {
                fprintf(
                    stderr,
                    c"WARNING: kernel doesn't support ops.sub_cgroup_id\n".as_ptr(),
                );
                (*(*__skel).struct_ops.$ops_name).sub_cgroup_id = 0;
            }
            if (*(*__skel).struct_ops.$ops_name).rescue_bandwidth_ppt > 0
                && !__COMPAT_struct_has_field(
                    c"sched_ext_ops".as_ptr(),
                    c"rescue_bandwidth_ppt".as_ptr(),
                )
            {
                fprintf(
                    stderr,
                    c"WARNING: kernel doesn't support ops.rescue_bandwidth_ppt\n".as_ptr(),
                );
                (*(*__skel).struct_ops.$ops_name).rescue_bandwidth_ppt = 0;
            }
            if (*(*__skel).struct_ops.$ops_name).rescue_quantum_us > 0
                && !__COMPAT_struct_has_field(
                    c"sched_ext_ops".as_ptr(),
                    c"rescue_quantum_us".as_ptr(),
                )
            {
                fprintf(
                    stderr,
                    c"WARNING: kernel doesn't support ops.rescue_quantum_us\n".as_ptr(),
                );
                (*(*__skel).struct_ops.$ops_name).rescue_quantum_us = 0;
            }
        }
        __skel
    }};
}

/*
 * Open a cid-form (struct sched_ext_ops_cid) skeleton. The cid form postdates
 * every op the load-time fix-ups above handle, so none of them apply.
 */
macro_rules! SCX_OPS_CID_OPEN {
    ($ops_name:ident, $scx_name:ident) => {
        __SCX_OPS_OPEN!($ops_name, $scx_name, c"sched_ext_ops_cid")
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
