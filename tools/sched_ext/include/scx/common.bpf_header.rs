/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

/*
 * Header guard and C include directives from common.bpf.h are intentionally not
 * executable Rust. This translation depends on the Rust forms of vmlinux.h,
 * bpf helpers/tracing, errno, user_exit_info.bpf.h, enum_defs.autogen.h,
 * bpf_arena_common.bpf.h, compat.bpf.h, enums.bpf.h, and cid.bpf.h.
 *
 * The C header defines BPF_NO_KFUNC_PROTOTYPES to suppress generated kfunc
 * prototypes whose address-space attributes are missing.
 */

pub type s32 = i32;
pub type s64 = i64;
pub type u32 = u32;
pub type u64 = u64;
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type size_t = usize;
pub type c_int = core::ffi::c_int;
pub type c_char = core::ffi::c_char;
pub type c_void = core::ffi::c_void;
pub type c_ulong = core::ffi::c_ulong;
pub type c_ulonglong = core::ffi::c_ulonglong;

pub const PF_IDLE: u32 = 0x00000002; /* I am an IDLE thread */
pub const PF_IO_WORKER: u32 = 0x00000010; /* Task is an IO worker */
pub const PF_WQ_WORKER: u32 = 0x00000020; /* I'm a workqueue worker */
pub const PF_KCOMPACTD: u32 = 0x00010000; /* I am kcompactd */
pub const PF_KSWAPD: u32 = 0x00020000; /* I am kswapd */
pub const PF_KTHREAD: u32 = 0x00200000; /* I am a kernel thread */
pub const PF_EXITING: u32 = 0x00000004;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const NR_CPUS: u32 = 1024;
pub const NUMA_NO_NODE: i32 = -1;

unsafe extern "C" {
    pub static LINUX_KERNEL_VERSION: c_int;
    pub static CONFIG_CC_VERSION_TEXT: [c_char; 64];
    pub static CONFIG_LOCALVERSION: [c_char; 64];
}

/*
 * Earlier versions of clang/pahole lost upper 32bits in 64bit enums which can
 * lead to really confusing misbehaviors. The C header uses _Static_assert on
 * SCX_DSQ_FLAG_BUILTIN; keep the dependency visible for Rust builds.
 */
pub fn ___vmlinux_h_sanity_check___() {
    let _ = SCX_DSQ_FLAG_BUILTIN;
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}
pub type cpumask_t = cpumask;
#[repr(C)]
pub struct scx_bpf_select_cpu_and_args {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scx_bpf_dsq_insert_vtime_args {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_iter_scx_dsq {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scx_event_stats {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scx_cid_topo {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scx_cmask {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rq {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_list_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_rb_root {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_iter_css {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cgroup_subsys_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_cpumask {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_iter_bits {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_res_spin_lock {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn scx_bpf_create_dsq(dsq_id: u64, node: s32) -> s32;
    pub fn scx_bpf_select_cpu_dfl(
        p: *mut task_struct,
        prev_cpu: s32,
        wake_flags: u64,
        is_idle: *mut bool,
    ) -> s32;
    pub fn __scx_bpf_select_cpu_and(
        p: *mut task_struct,
        cpus_allowed: *const cpumask,
        args: *mut scx_bpf_select_cpu_and_args,
    ) -> s32;
    pub fn __scx_bpf_dsq_insert_vtime(
        p: *mut task_struct,
        args: *mut scx_bpf_dsq_insert_vtime_args,
    ) -> bool;
    pub fn scx_bpf_dispatch_nr_slots() -> u32;
    pub fn scx_bpf_dispatch_cancel();
    pub fn scx_bpf_kick_cpu(cpu: s32, flags: u64);
    pub fn scx_bpf_dsq_nr_queued(dsq_id: u64) -> s32;
    pub fn scx_bpf_destroy_dsq(dsq_id: u64);
    pub fn scx_bpf_dsq_peek(dsq_id: u64) -> *mut task_struct;
    pub fn bpf_iter_scx_dsq_new(it: *mut bpf_iter_scx_dsq, dsq_id: u64, flags: u64) -> c_int;
    pub fn bpf_iter_scx_dsq_next(it: *mut bpf_iter_scx_dsq) -> *mut task_struct;
    pub fn bpf_iter_scx_dsq_destroy(it: *mut bpf_iter_scx_dsq);
    pub fn scx_bpf_exit_bstr(
        exit_code: s64,
        fmt: *mut c_char,
        data: *mut c_ulonglong,
        data__sz: u32,
    );
    pub fn scx_bpf_error_bstr(fmt: *mut c_char, data: *mut c_ulonglong, data_len: u32);
    pub fn scx_bpf_dump_bstr(fmt: *mut c_char, data: *mut c_ulonglong, data_len: u32);
    pub fn scx_bpf_cpuperf_cap(cpu: s32) -> u32;
    pub fn scx_bpf_cpuperf_cur(cpu: s32) -> u32;
    pub fn scx_bpf_cpuperf_set(cpu: s32, perf: u32);
    pub fn scx_bpf_nr_node_ids() -> u32;
    pub fn scx_bpf_nr_cpu_ids() -> u32;
    pub fn scx_bpf_cpu_node(cpu: s32) -> c_int;
    pub fn scx_bpf_get_possible_cpumask() -> *const cpumask;
    pub fn scx_bpf_get_online_cpumask() -> *const cpumask;
    pub fn scx_bpf_put_cpumask(cpumask: *const cpumask);
    pub fn scx_bpf_get_idle_cpumask_node(node: c_int) -> *const cpumask;
    pub fn scx_bpf_get_idle_cpumask() -> *const cpumask;
    pub fn scx_bpf_get_idle_smtmask_node(node: c_int) -> *const cpumask;
    pub fn scx_bpf_get_idle_smtmask() -> *const cpumask;
    pub fn scx_bpf_put_idle_cpumask(cpumask: *const cpumask);
    pub fn scx_bpf_test_and_clear_cpu_idle(cpu: s32) -> bool;
    pub fn scx_bpf_pick_idle_cpu_node(
        cpus_allowed: *const cpumask_t,
        node: c_int,
        flags: u64,
    ) -> s32;
    pub fn scx_bpf_pick_idle_cpu(cpus_allowed: *const cpumask_t, flags: u64) -> s32;
    pub fn scx_bpf_pick_any_cpu_node(
        cpus_allowed: *const cpumask_t,
        node: c_int,
        flags: u64,
    ) -> s32;
    pub fn scx_bpf_pick_any_cpu(cpus_allowed: *const cpumask_t, flags: u64) -> s32;
    pub fn scx_bpf_task_running(p: *const task_struct) -> bool;
    pub fn scx_bpf_task_cpu(p: *const task_struct) -> s32;
    pub fn scx_bpf_locked_rq() -> *mut rq;
    pub fn scx_bpf_cpu_curr(cpu: s32) -> *mut task_struct;
    pub fn scx_bpf_tid_to_task(tid: u64) -> *mut task_struct;
    pub fn scx_bpf_now() -> u64;
    pub fn scx_bpf_events(events: *mut scx_event_stats, events__sz: size_t);
    pub fn scx_bpf_cpu_to_cid(cpu: s32) -> s32;
    pub fn scx_bpf_cid_to_cpu(cid: s32) -> s32;
    pub fn scx_bpf_cid_topo(cid: s32, out: *mut scx_cid_topo);
    pub fn scx_bpf_kick_cid(cid: s32, flags: u64);
    pub fn scx_bpf_task_cid(p: *const task_struct) -> s32;
    pub fn scx_bpf_this_cid() -> s32;
    pub fn scx_bpf_cid_curr(cid: s32) -> *mut task_struct;
    pub fn scx_bpf_nr_cids() -> u32;
    pub fn scx_bpf_nr_online_cids() -> u32;
    pub fn scx_bpf_cidperf_cap(cid: s32) -> u32;
    pub fn scx_bpf_cidperf_cur(cid: s32) -> u32;
    pub fn scx_bpf_cidperf_set(cid: s32, perf: u32) -> s32;

    /* sub-scheduler cap control, scx_bpf_sub_caps() cgroup_id 0 == self */
    pub fn scx_bpf_sub_grant(
        cgroup_id: u64,
        caps: u64,
        cmask__arena: *const scx_cmask,
        denied_out__arena__nullable: *mut scx_cmask,
    ) -> s32;
    pub fn scx_bpf_sub_revoke(cgroup_id: u64, caps: u64, cmask__arena: *const scx_cmask);
    pub fn scx_bpf_sub_caps(cgroup_id: u64, caps: u64, out__arena: *mut scx_cmask) -> s32;
    pub fn scx_bpf_sub_kill_bstr(
        cgroup_id: u64,
        fmt: *mut c_char,
        data: *mut c_ulonglong,
        data__sz: u32,
    ) -> s32;
}

/* Use &___it as the C BPF_FOR_EACH_ITER argument inside bpf_for_each loops. */

#[macro_export]
macro_rules! scx_read_event {
    ($e:expr, $name:ident) => {{
        if bpf_core_field_exists!((*$e).$name) {
            unsafe { (*$e).$name }
        } else {
            0
        }
    }};
}

pub unsafe fn ___scx_bpf_bstr_format_checker(_fmt: *const c_char, _args: ...) {}

#[macro_export]
macro_rules! SCX_STRINGIFY {
    ($x:tt) => {
        stringify!($x)
    };
}

#[macro_export]
macro_rules! SCX_TOSTRING {
    ($x:tt) => {
        stringify!($x)
    };
}

/*
 * The C variadic formatting helpers scx_bpf_bstr_preamble(), scx_bpf_exit(),
 * scx_bpf_sub_kill(), scx_bpf_error(), scx_bpf_dump(), and
 * scx_bpf_dump_header() rely on GNU C statement expressions, static local char
 * arrays, ___bpf_narg(), and ___bpf_fill(). They should be supplied by the BPF
 * Rust macro layer with the same argument packing and side effects.
 *
 * BPF_STRUCT_OPS and BPF_STRUCT_OPS_SLEEPABLE map C SEC("struct_ops/...") plus
 * BPF_PROG declarations; RESIZABLE_ARRAY maps to a one-element array in a
 * custom data subsection.
 */

/*
 * MEMBER_VPTR and ARRAY_ELEM_PTR are verifier-facing pointer macros that
 * calculate byte offsets, bounds-check with BPF inline assembly, and return
 * NULL on out-of-bounds access. Their exact semantics require BPF assembly and
 * typeof(); callers should use an equivalent Rust/BPF macro.
 */

#[macro_export]
macro_rules! __sink {
    ($expr:expr) => {{
        core::arch::asm!("", inout(reg) $expr, options(nostack, preserves_flags));
    }};
}

/* list and rbtree */
/* __contains(name, node) adds a btf_decl_tag("contains:name:node"). */
/* private(name) places a hidden 8-byte-aligned object into .data.name. */

unsafe extern "C" {
    pub fn bpf_obj_new_impl(local_type_id: __u64, meta: *mut c_void) -> *mut c_void;
    pub fn bpf_obj_drop_impl(kptr: *mut c_void, meta: *mut c_void);
    pub fn bpf_list_push_front_impl(
        head: *mut bpf_list_head,
        node: *mut bpf_list_node,
        meta: *mut c_void,
        off: __u64,
    ) -> c_int;
    pub fn bpf_list_push_back_impl(
        head: *mut bpf_list_head,
        node: *mut bpf_list_node,
        meta: *mut c_void,
        off: __u64,
    ) -> c_int;
    pub fn bpf_list_pop_front(head: *mut bpf_list_head) -> *mut bpf_list_node;
    pub fn bpf_list_pop_back(head: *mut bpf_list_head) -> *mut bpf_list_node;
    pub fn bpf_rbtree_remove(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    pub fn bpf_rbtree_add_impl(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool,
        meta: *mut c_void,
        off: __u64,
    ) -> c_int;
    pub fn bpf_rbtree_first(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
    pub fn bpf_refcount_acquire_impl(kptr: *mut c_void, meta: *mut c_void) -> *mut c_void;

    /* task */
    pub fn bpf_task_from_pid(pid: s32) -> *mut task_struct;
    pub fn bpf_task_acquire(p: *mut task_struct) -> *mut task_struct;
    pub fn bpf_task_release(p: *mut task_struct);

    /* cgroup */
    pub fn bpf_cgroup_ancestor(cgrp: *mut cgroup, level: c_int) -> *mut cgroup;
    pub fn bpf_cgroup_acquire(cgrp: *mut cgroup) -> *mut cgroup;
    pub fn bpf_cgroup_release(cgrp: *mut cgroup);
    pub fn bpf_cgroup_from_id(cgid: u64) -> *mut cgroup;

    /* css iteration */
    pub fn bpf_iter_css_new(
        it: *mut bpf_iter_css,
        start: *mut cgroup_subsys_state,
        flags: c_uint,
    ) -> c_int;
    pub fn bpf_iter_css_next(it: *mut bpf_iter_css) -> *mut cgroup_subsys_state;
    pub fn bpf_iter_css_destroy(it: *mut bpf_iter_css);

    /* cpumask */
    pub fn bpf_cpumask_create() -> *mut bpf_cpumask;
    pub fn bpf_cpumask_acquire(cpumask: *mut bpf_cpumask) -> *mut bpf_cpumask;
    pub fn bpf_cpumask_release(cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_first(cpumask: *const cpumask) -> u32;
    pub fn bpf_cpumask_first_zero(cpumask: *const cpumask) -> u32;
    pub fn bpf_cpumask_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_test_cpu(cpu: u32, cpumask: *const cpumask) -> bool;
    pub fn bpf_cpumask_test_and_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> bool;
    pub fn bpf_cpumask_test_and_clear_cpu(cpu: u32, cpumask: *mut bpf_cpumask) -> bool;
    pub fn bpf_cpumask_setall(cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_clear(cpumask: *mut bpf_cpumask);
    pub fn bpf_cpumask_and(dst: *mut bpf_cpumask, src1: *const cpumask, src2: *const cpumask) -> bool;
    pub fn bpf_cpumask_or(dst: *mut bpf_cpumask, src1: *const cpumask, src2: *const cpumask);
    pub fn bpf_cpumask_xor(dst: *mut bpf_cpumask, src1: *const cpumask, src2: *const cpumask);
    pub fn bpf_cpumask_equal(src1: *const cpumask, src2: *const cpumask) -> bool;
    pub fn bpf_cpumask_intersects(src1: *const cpumask, src2: *const cpumask) -> bool;
    pub fn bpf_cpumask_subset(src1: *const cpumask, src2: *const cpumask) -> bool;
    pub fn bpf_cpumask_empty(cpumask: *const cpumask) -> bool;
    pub fn bpf_cpumask_full(cpumask: *const cpumask) -> bool;
    pub fn bpf_cpumask_copy(dst: *mut bpf_cpumask, src: *const cpumask);
    pub fn bpf_cpumask_any_distribute(cpumask: *const cpumask) -> u32;
    pub fn bpf_cpumask_any_and_distribute(src1: *const cpumask, src2: *const cpumask) -> u32;
    pub fn bpf_cpumask_weight(cpumask: *const cpumask) -> u32;

    pub fn bpf_iter_bits_new(it: *mut bpf_iter_bits, unsafe_ptr__ign: *const u64, nr_words: u32) -> c_int;
    pub fn bpf_iter_bits_next(it: *mut bpf_iter_bits) -> *mut c_int;
    pub fn bpf_iter_bits_destroy(it: *mut bpf_iter_bits);
}

pub type c_uint = core::ffi::c_uint;

#[inline]
pub unsafe fn bpf_list_push_front(head: *mut bpf_list_head, node: *mut bpf_list_node) -> c_int {
    unsafe { bpf_list_push_front_impl(head, node, core::ptr::null_mut(), 0) }
}

#[inline]
pub unsafe fn bpf_list_push_back(head: *mut bpf_list_head, node: *mut bpf_list_node) -> c_int {
    unsafe { bpf_list_push_back_impl(head, node, core::ptr::null_mut(), 0) }
}

#[inline]
pub unsafe fn bpf_rbtree_add(
    head: *mut bpf_rb_root,
    node: *mut bpf_rb_node,
    less: unsafe extern "C" fn(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool,
) -> c_int {
    unsafe { bpf_rbtree_add_impl(head, node, less, core::ptr::null_mut(), 0) }
}

#[inline]
pub unsafe fn bpf_refcount_acquire(kptr: *mut c_void) -> *mut c_void {
    unsafe { bpf_refcount_acquire_impl(kptr, core::ptr::null_mut()) }
}

#[repr(C)]
pub struct bpf_iter_possible {
    pub it: bpf_iter_bits,
    pub bitmap: *const cpumask,
}

#[inline]
pub unsafe fn bpf_iter_possible_new(
    it: *mut bpf_iter_possible,
    _unsafe_ptr__ign: *const u64,
    _nr_words: u32,
) -> c_int {
    unsafe {
        (*it).bitmap = scx_bpf_get_possible_cpumask();
        bpf_iter_bits_new(
            core::ptr::addr_of_mut!((*it).it),
            (*it).bitmap as *const u64,
            (core::mem::size_of::<cpumask>() / 8) as u32,
        )
    }
}

#[inline]
pub unsafe fn bpf_iter_possible_next(it: *mut bpf_iter_possible) -> *mut c_int {
    unsafe { bpf_iter_bits_next(core::ptr::addr_of_mut!((*it).it)) }
}

#[inline]
pub unsafe fn bpf_iter_possible_destroy(it: *mut bpf_iter_possible) {
    unsafe {
        scx_bpf_put_cpumask((*it).bitmap);
        bpf_iter_bits_destroy(core::ptr::addr_of_mut!((*it).it));
    }
}

/* for_each_possible_cpu(cpu) maps to bpf_for_each(possible, cpu, NULL, 0). */

#[repr(C)]
pub struct bpf_iter_online {
    pub it: bpf_iter_bits,
    pub bitmap: *const cpumask,
}

#[inline]
pub unsafe fn bpf_iter_online_new(
    it: *mut bpf_iter_online,
    _unsafe_ptr__ign: *const u64,
    _nr_words: u32,
) -> c_int {
    unsafe {
        (*it).bitmap = scx_bpf_get_online_cpumask();
        bpf_iter_bits_new(
            core::ptr::addr_of_mut!((*it).it),
            (*it).bitmap as *const u64,
            (core::mem::size_of::<cpumask>() / 8) as u32,
        )
    }
}

#[inline]
pub unsafe fn bpf_iter_online_next(it: *mut bpf_iter_online) -> *mut c_int {
    unsafe { bpf_iter_bits_next(core::ptr::addr_of_mut!((*it).it)) }
}

#[inline]
pub unsafe fn bpf_iter_online_destroy(it: *mut bpf_iter_online) {
    unsafe {
        scx_bpf_put_cpumask((*it).bitmap);
        bpf_iter_bits_destroy(core::ptr::addr_of_mut!((*it).it));
    }
}

/* for_each_online_cpu(cpu) maps to bpf_for_each(online, cpu, NULL, 0). */

/*
 * Access a cpumask in read-only mode (typically to check bits).
 */
#[inline]
pub unsafe fn cast_mask(mask: *mut bpf_cpumask) -> *const cpumask {
    mask as *const cpumask
}

/*
 * Return true if task @p cannot migrate to a different CPU, false otherwise.
 *
 * The C implementation checks p->migration_disabled with CO-RE field-existence
 * logic and compares against bpf_get_current_task_btf(). The field belongs to
 * the external task_struct definition, so the exact field access is left to the
 * dependency-provided Rust binding.
 */

unsafe extern "C" {
    pub fn bpf_rcu_read_lock();
    pub fn bpf_rcu_read_unlock();
    pub fn bpf_res_spin_lock(lock: *mut bpf_res_spin_lock) -> c_int;
    pub fn bpf_res_spin_unlock(lock: *mut bpf_res_spin_lock);
}

#[inline]
pub fn time_delta(after: u64, before: u64) -> s64 {
    let delta = after.wrapping_sub(before) as s64;
    if delta > 0 { delta } else { 0 }
}

#[inline]
pub fn time_after(a: u64, b: u64) -> bool {
    (b.wrapping_sub(a) as s64) < 0
}

#[inline]
pub fn time_before(a: u64, b: u64) -> bool {
    time_after(b, a)
}

#[inline]
pub fn time_after_eq(a: u64, b: u64) -> bool {
    (a.wrapping_sub(b) as s64) >= 0
}

#[inline]
pub fn time_before_eq(a: u64, b: u64) -> bool {
    time_after_eq(b, a)
}

#[inline]
pub fn time_in_range(a: u64, b: u64, c: u64) -> bool {
    time_after_eq(a, b) && time_before_eq(a, c)
}

#[inline]
pub fn time_in_range_open(a: u64, b: u64, c: u64) -> bool {
    time_after_eq(a, b) && time_before(a, c)
}

#[inline]
pub fn likely(x: bool) -> bool {
    x
}

#[inline]
pub fn unlikely(x: bool) -> bool {
    x
}

pub type __u8_alias_t = __u8;
pub type __u16_alias_t = __u16;
pub type __u32_alias_t = __u32;
pub type __u64_alias_t = __u64;

#[inline]
pub unsafe fn __read_once_size(p: *const c_void, res: *mut c_void, size: c_int) {
    unsafe {
        match size {
            1 => *(res as *mut __u8_alias_t) = core::ptr::read_volatile(p as *const __u8_alias_t),
            2 => *(res as *mut __u16_alias_t) = core::ptr::read_volatile(p as *const __u16_alias_t),
            4 => *(res as *mut __u32_alias_t) = core::ptr::read_volatile(p as *const __u32_alias_t),
            8 => *(res as *mut __u64_alias_t) = core::ptr::read_volatile(p as *const __u64_alias_t),
            _ => {
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                core::ptr::copy_nonoverlapping(p as *const u8, res as *mut u8, size as usize);
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            }
        }
    }
}

#[inline]
pub unsafe fn __write_once_size(p: *mut c_void, res: *mut c_void, size: c_int) {
    unsafe {
        match size {
            1 => core::ptr::write_volatile(p as *mut __u8_alias_t, *(res as *mut __u8_alias_t)),
            2 => core::ptr::write_volatile(p as *mut __u16_alias_t, *(res as *mut __u16_alias_t)),
            4 => core::ptr::write_volatile(p as *mut __u32_alias_t, *(res as *mut __u32_alias_t)),
            8 => core::ptr::write_volatile(p as *mut __u64_alias_t, *(res as *mut __u64_alias_t)),
            _ => {
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                core::ptr::copy_nonoverlapping(res as *const u8, p as *mut u8, size as usize);
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            }
        }
    }
}

#[macro_export]
macro_rules! READ_ONCE {
    ($x:expr) => {{
        core::ptr::read_volatile(core::ptr::addr_of!($x))
    }};
}

#[macro_export]
macro_rules! WRITE_ONCE {
    ($x:expr, $val:expr) => {{
        core::ptr::write_volatile(core::ptr::addr_of_mut!($x), $val);
        $val
    }};
}

#[macro_export]
macro_rules! __calc_avg {
    ($old:expr, $new:expr, $decay:expr) => {{
        let thr = 1 << $decay;
        let ret;
        if ($old < thr) || ($new < thr) {
            if ($old == 1) && ($new == 0) {
                ret = 0;
            } else {
                ret = ($old - ($old >> 1)) + ($new >> 1);
            }
        } else {
            ret = ($old - ($old >> $decay)) + ($new >> $decay);
        }
        ret
    }};
}

#[inline]
pub fn log2_u32(mut v: u32) -> u32 {
    let mut r: u32;
    let mut shift: u32;

    r = ((v > 0xFFFF) as u32) << 4;
    v >>= r;
    shift = ((v > 0xFF) as u32) << 3;
    v >>= shift;
    r |= shift;
    shift = ((v > 0xF) as u32) << 2;
    v >>= shift;
    r |= shift;
    shift = ((v > 0x3) as u32) << 1;
    v >>= shift;
    r |= shift;
    r |= v >> 1;
    r
}

#[inline]
pub fn log2_u64(v: u64) -> u32 {
    let hi: u32 = (v >> 32) as u32;
    if hi != 0 {
        log2_u32(hi) + 32 + 1
    } else {
        log2_u32(v as u32) + 1
    }
}

#[inline]
pub fn __sqrt_u64(x: u64) -> u64 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut r = if (1u64 << 32) > x { x } else { 1u64 << 32 };

    for _i in 0..8 {
        let q = x / r;
        if r <= q {
            break;
        }
        r = (r + q) >> 1;
    }
    r
}

#[inline]
pub fn ctzll(v: u64) -> c_int {
    /*
     * The C header uses __builtin_ctzll for native x86 or BPF with clang >= 19
     * and otherwise uses De Bruijn software emulation. Rust's trailing_zeros
     * maps to the intrinsic path; preserve the C fallback's zero behavior.
     */
    if v == 0 {
        return -1;
    }

    const LOOKUP_TABLE: [c_int; 64] = [
        0, 1, 48, 2, 57, 49, 28, 3, 61, 58, 50, 42, 38, 29, 17, 4, 62, 55, 59, 36, 53, 51, 43,
        22, 45, 39, 33, 30, 24, 18, 12, 5, 63, 47, 56, 27, 60, 41, 37, 16, 54, 35, 52, 21,
        44, 32, 23, 11, 46, 26, 40, 15, 34, 20, 31, 10, 25, 14, 19, 9, 13, 8, 7, 6,
    ];
    const DEBRUIJN_CONSTANT: u64 = 0x03f79d71b4cb0a89;
    let lowest_bit = v & v.wrapping_neg();
    let index = ((lowest_bit.wrapping_mul(DEBRUIJN_CONSTANT)) >> 58) as usize;
    *LOOKUP_TABLE.get(index).unwrap_or(&-1)
}

/*
 * scale_by_task_weight() and scale_by_task_weight_inverse() depend on the
 * external task_struct.scx.weight field from vmlinux bindings:
 *
 *     (value * p->scx.weight) / 100
 *     value * 100 / p->scx.weight
 */

unsafe extern "C" {
    pub fn bpf_get_prandom_u32() -> u32;
}

#[inline]
pub unsafe fn get_prandom_u64() -> u64 {
    unsafe { ((bpf_get_prandom_u32() as u64) << 32) | bpf_get_prandom_u32() as u64 }
}

#[repr(C)]
pub struct rq___local {
    /*
     * A monotonically increasing clock per CPU. It is rq->clock minus
     * cumulative IRQ time and hypervisor steal time. Unlike rq->clock,
     * it does not advance during IRQ processing or hypervisor preemption.
     */
    pub clock_task: u64,
    /*
     * Invariant version of clock_task scaled by CPU capacity and frequency.
     */
    pub clock_pelt: u64,
    /*
     * Accumulates the magnitude of each clock_pelt jump at idle exit.
     */
    pub lost_idle_time: c_ulong,
    /*
     * Shadow of paravirt_steal_clock(). Available only when
     * CONFIG_PARAVIRT_TIME_ACCOUNTING is on.
     */
    pub prev_steal_time_rq: u64,
}

unsafe extern "C" {
    pub static mut runqueues: rq;
}

#[repr(C)]
pub struct irqtime___local {
    /*
     * Cumulative IRQ time counter for this CPU, in nanoseconds. Available only
     * when CONFIG_IRQ_TIME_ACCOUNTING is on.
     */
    pub total: u64,
}

unsafe extern "C" {
    pub static mut cpu_irqtime: irqtime___local;
    pub fn bpf_per_cpu_ptr(ptr: *const c_void, cpu: u32) -> *mut c_void;
}

#[inline]
pub unsafe fn get_current_rq(cpu: u32) -> *mut rq___local {
    /*
     * WARNING: The caller must hold the rq lock for @cpu. Correctness is
     * enforced by calling context only.
     */
    unsafe { bpf_per_cpu_ptr(core::ptr::addr_of!(runqueues) as *const c_void, cpu) as *mut rq___local }
}

#[inline]
pub unsafe fn scx_clock_task(cpu: u32) -> u64 {
    let rq = unsafe { get_current_rq(cpu) };
    if !rq.is_null() {
        unsafe { (*rq).clock_task }
    } else {
        0
    }
}

#[inline]
pub unsafe fn scx_clock_pelt(cpu: u32) -> u64 {
    let rq = unsafe { get_current_rq(cpu) };
    if !rq.is_null() {
        unsafe { (*rq).clock_pelt.wrapping_sub((*rq).lost_idle_time as u64) }
    } else {
        0
    }
}

#[inline]
pub unsafe fn scx_clock_virt(cpu: u32) -> u64 {
    /*
     * The C version checks bpf_core_field_exists(prev_steal_time_rq) before the
     * per-CPU lookup. This Rust translation preserves the field read; CO-RE
     * field existence must be handled by the surrounding BPF Rust layer.
     */
    let rq = unsafe { get_current_rq(cpu) };
    if !rq.is_null() {
        unsafe { (*rq).prev_steal_time_rq }
    } else {
        0
    }
}

#[inline]
pub unsafe fn scx_clock_irq(cpu: u32) -> u64 {
    /*
     * The C version uses bpf_core_type_exists(struct irqtime___local) so kernels
     * without CONFIG_IRQ_TIME_ACCOUNTING return 0 before bpf_per_cpu_ptr().
     */
    let irqt = unsafe {
        bpf_per_cpu_ptr(core::ptr::addr_of!(cpu_irqtime) as *const c_void, cpu) as *mut irqtime___local
    };
    if !irqt.is_null() {
        unsafe { (*irqt).total }
    } else {
        0
    }
}

#[macro_export]
macro_rules! flex_array_size {
    ($p:expr, $member:ident, $count:expr) => {
        ($count) * core::mem::size_of_val(unsafe { &*(*$p).$member })
    };
}

/*
 * struct_size(p, member, count) and struct_size_t(type, member, count) are C
 * offsetof/typeof helpers for flexible arrays. Use the Rust dependency layer's
 * offset calculation for concrete translated structs.
 */
