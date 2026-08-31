/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2024 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2024 David Vernet <dvernet@meta.com>
 */

// Header guard and C include mechanics are intentionally omitted in Rust.

pub type u64 = u64;
pub type u32 = u32;
pub type s32 = i32;
pub type size_t = usize;

// External kernel/BPF types supplied by other translated headers.
pub enum cgroup {}
pub enum task_struct {}
pub enum bpf_iter_scx_dsq {}
pub enum bpf_cpumask {}
pub enum cpumask {}
pub enum scx_bpf_select_cpu_and_args {}
pub enum scx_bpf_dsq_insert_vtime_args {}
pub enum scx_enq_flags {}

extern "C" {
    fn bpf_core_enum_value_exists<T>(ent: T) -> bool;
    fn bpf_core_enum_value<T>(ent: T) -> u64;
    fn bpf_core_type_exists<T>() -> bool;
    fn bpf_ksym_exists<T>(sym: T) -> bool;
    fn bpf_iter_scx_dsq_new(it: *mut bpf_iter_scx_dsq, dsq_id: u64, flags: u64) -> i32;
    fn bpf_iter_scx_dsq_next(it: *mut bpf_iter_scx_dsq) -> *mut task_struct;
    fn bpf_iter_scx_dsq_destroy(it: *mut bpf_iter_scx_dsq);
    fn scx_bpf_dsq_peek(dsq_id: u64) -> *mut task_struct;
    fn bpf_ktime_get_ns() -> u64;
    fn scx_bpf_now() -> u64;
    fn scx_bpf_events(events: *mut core::ffi::c_void, size: size_t);
    fn scx_bpf_nr_node_ids() -> u32;
    fn scx_bpf_cpu_node(cpu: s32) -> s32;
    fn scx_bpf_get_idle_cpumask() -> *const cpumask;
    fn scx_bpf_get_idle_smtmask() -> *const cpumask;
    fn scx_bpf_get_idle_cpumask_node(node: s32) -> *const cpumask;
    fn scx_bpf_get_idle_smtmask_node(node: s32) -> *const cpumask;
    fn scx_bpf_pick_idle_cpu(cpus_allowed: *const cpumask, flags: u64) -> s32;
    fn scx_bpf_pick_any_cpu(cpus_allowed: *const cpumask, flags: u64) -> s32;
    fn scx_bpf_pick_idle_cpu_node(cpus_allowed: *const cpumask, node: s32, flags: u64) -> s32;
    fn scx_bpf_pick_any_cpu_node(cpus_allowed: *const cpumask, node: s32, flags: u64) -> s32;
    fn __scx_bpf_select_cpu_and(
        p: *mut task_struct,
        cpus_allowed: *const cpumask,
        args: *const scx_bpf_select_cpu_and_args,
    ) -> s32;
    fn __scx_bpf_dsq_insert_vtime(
        p: *mut task_struct,
        args: *const scx_bpf_dsq_insert_vtime_args,
    ) -> bool;
    fn scx_bpf_error(fmt: *const core::ffi::c_char);
}

pub const EOPNOTSUPP: i32 = 95;

// Statement-expression macro translation. The enum type and entry are supplied
// by the caller in C; Rust keeps that dependency generic.
pub unsafe fn __COMPAT_ENUM_OR_ZERO<T: Copy + From<u8>>(__ent: T) -> T {
    let mut __ret = T::from(0);
    if bpf_core_enum_value_exists(__ent) {
        __ret = __ent;
    }
    __ret
}

/* v6.12: 819513666966 ("sched_ext: Add cgroup support") */
extern "C" {
    pub fn scx_bpf_task_cgroup___new(p: *mut task_struct) -> *mut cgroup;
}

pub unsafe fn scx_bpf_task_cgroup(p: *mut task_struct) -> *mut cgroup {
    if bpf_ksym_exists(scx_bpf_task_cgroup___new as unsafe extern "C" fn(*mut task_struct) -> *mut cgroup) {
        scx_bpf_task_cgroup___new(p)
    } else {
        core::ptr::null_mut()
    }
}

/*
 * v6.13: The verb `dispatch` was too overloaded and confusing. kfuncs are
 * renamed to unload the verb.
 *
 * scx_bpf_dispatch_from_dsq() and friends were added during v6.12 by
 * 4c30f5ce4f7a ("sched_ext: Implement scx_bpf_dispatch[_vtime]_from_dsq()").
 *
 * v7.1: scx_bpf_dsq_move_to_local___v2() to add @enq_flags.
 */
extern "C" {
    pub fn scx_bpf_dsq_move_to_local___v2___compat(dsq_id: u64, enq_flags: u64) -> bool;
    pub fn scx_bpf_dsq_move_to_local___v1(dsq_id: u64) -> bool;
    pub fn scx_bpf_dsq_move_set_slice___new(it__iter: *mut bpf_iter_scx_dsq, slice: u64);
    pub fn scx_bpf_dsq_move_set_vtime___new(it__iter: *mut bpf_iter_scx_dsq, vtime: u64);
    pub fn scx_bpf_dsq_move___new(
        it__iter: *mut bpf_iter_scx_dsq,
        p: *mut task_struct,
        dsq_id: u64,
        enq_flags: u64,
    ) -> bool;
    pub fn scx_bpf_dsq_move_vtime___new(
        it__iter: *mut bpf_iter_scx_dsq,
        p: *mut task_struct,
        dsq_id: u64,
        enq_flags: u64,
    ) -> bool;

    pub fn scx_bpf_consume___old(dsq_id: u64) -> bool;
    pub fn scx_bpf_dispatch_from_dsq_set_slice___old(it__iter: *mut bpf_iter_scx_dsq, slice: u64);
    pub fn scx_bpf_dispatch_from_dsq_set_vtime___old(it__iter: *mut bpf_iter_scx_dsq, vtime: u64);
    pub fn scx_bpf_dispatch_from_dsq___old(
        it__iter: *mut bpf_iter_scx_dsq,
        p: *mut task_struct,
        dsq_id: u64,
        enq_flags: u64,
    ) -> bool;
    pub fn scx_bpf_dispatch_vtime_from_dsq___old(
        it__iter: *mut bpf_iter_scx_dsq,
        p: *mut task_struct,
        dsq_id: u64,
        enq_flags: u64,
    ) -> bool;
}

pub unsafe fn scx_bpf_dsq_move_to_local(dsq_id: u64, enq_flags: u64) -> bool {
    if bpf_ksym_exists(scx_bpf_dsq_move_to_local___v2___compat as unsafe extern "C" fn(u64, u64) -> bool) {
        scx_bpf_dsq_move_to_local___v2___compat(dsq_id, enq_flags)
    } else if bpf_ksym_exists(scx_bpf_dsq_move_to_local___v1 as unsafe extern "C" fn(u64) -> bool) {
        scx_bpf_dsq_move_to_local___v1(dsq_id)
    } else {
        scx_bpf_consume___old(dsq_id)
    }
}

pub unsafe fn scx_bpf_dsq_move_set_slice(it__iter: *mut bpf_iter_scx_dsq, slice: u64) {
    if bpf_ksym_exists(scx_bpf_dsq_move_set_slice___new as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, u64)) {
        scx_bpf_dsq_move_set_slice___new(it__iter, slice);
    } else if bpf_ksym_exists(scx_bpf_dispatch_from_dsq_set_slice___old as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, u64)) {
        scx_bpf_dispatch_from_dsq_set_slice___old(it__iter, slice);
    }
}

pub unsafe fn scx_bpf_dsq_move_set_vtime(it__iter: *mut bpf_iter_scx_dsq, vtime: u64) {
    if bpf_ksym_exists(scx_bpf_dsq_move_set_vtime___new as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, u64)) {
        scx_bpf_dsq_move_set_vtime___new(it__iter, vtime);
    } else if bpf_ksym_exists(scx_bpf_dispatch_from_dsq_set_vtime___old as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, u64)) {
        scx_bpf_dispatch_from_dsq_set_vtime___old(it__iter, vtime);
    }
}

pub unsafe fn scx_bpf_dsq_move(
    it__iter: *mut bpf_iter_scx_dsq,
    p: *mut task_struct,
    dsq_id: u64,
    enq_flags: u64,
) -> bool {
    if bpf_ksym_exists(scx_bpf_dsq_move___new as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, *mut task_struct, u64, u64) -> bool) {
        scx_bpf_dsq_move___new(it__iter, p, dsq_id, enq_flags)
    } else if bpf_ksym_exists(scx_bpf_dispatch_from_dsq___old as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, *mut task_struct, u64, u64) -> bool) {
        scx_bpf_dispatch_from_dsq___old(it__iter, p, dsq_id, enq_flags)
    } else {
        false
    }
}

pub unsafe fn scx_bpf_dsq_move_vtime(
    it__iter: *mut bpf_iter_scx_dsq,
    p: *mut task_struct,
    dsq_id: u64,
    enq_flags: u64,
) -> bool {
    if bpf_ksym_exists(scx_bpf_dsq_move_vtime___new as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, *mut task_struct, u64, u64) -> bool) {
        scx_bpf_dsq_move_vtime___new(it__iter, p, dsq_id, enq_flags)
    } else if bpf_ksym_exists(scx_bpf_dispatch_vtime_from_dsq___old as unsafe extern "C" fn(*mut bpf_iter_scx_dsq, *mut task_struct, u64, u64) -> bool) {
        scx_bpf_dispatch_vtime_from_dsq___old(it__iter, p, dsq_id, enq_flags)
    } else {
        false
    }
}

/*
 * v6.15: 950ad93df2fc ("bpf: add kfunc for populating cpumask bits")
 *
 * Compat macro will be dropped on v6.19 release.
 */
extern "C" {
    pub fn bpf_cpumask_populate(dst: *mut bpf_cpumask, src: *mut core::ffi::c_void, src__sz: size_t) -> i32;
}

pub unsafe fn __COMPAT_bpf_cpumask_populate(
    cpumask: *mut bpf_cpumask,
    src: *mut core::ffi::c_void,
    size__sz: size_t,
) -> i32 {
    if bpf_ksym_exists(bpf_cpumask_populate as unsafe extern "C" fn(*mut bpf_cpumask, *mut core::ffi::c_void, size_t) -> i32) {
        bpf_cpumask_populate(cpumask, src, size__sz)
    } else {
        -EOPNOTSUPP
    }
}

/*
 * v6.19: Introduce lockless peek API for user DSQs.
 *
 * Preserve the following macro until v6.21.
 */
pub unsafe fn __COMPAT_scx_bpf_dsq_peek(dsq_id: u64) -> *mut task_struct {
    let mut p: *mut task_struct = core::ptr::null_mut();
    let mut it: core::mem::MaybeUninit<bpf_iter_scx_dsq> = core::mem::MaybeUninit::uninit();

    if bpf_ksym_exists(scx_bpf_dsq_peek as unsafe extern "C" fn(u64) -> *mut task_struct) {
        return scx_bpf_dsq_peek(dsq_id);
    }
    if bpf_iter_scx_dsq_new(it.as_mut_ptr(), dsq_id, 0) == 0 {
        p = bpf_iter_scx_dsq_next(it.as_mut_ptr());
    }
    bpf_iter_scx_dsq_destroy(it.as_mut_ptr());
    p
}

/*
 * v7.1: scx_bpf_sub_dispatch() for sub-sched dispatch. Preserve until
 * we drop the compat layer for older kernels that lack the kfunc.
 */
extern "C" {
    pub fn scx_bpf_sub_dispatch___compat(cgroup_id: u64) -> bool;
}

pub unsafe fn scx_bpf_sub_dispatch(cgroup_id: u64) -> bool {
    if bpf_ksym_exists(scx_bpf_sub_dispatch___compat as unsafe extern "C" fn(u64) -> bool) {
        scx_bpf_sub_dispatch___compat(cgroup_id)
    } else {
        false
    }
}

/*
 * v7.3: scx_bpf_cid_override() for explicit cid and shard mapping. Ignore if
 * missing.
 */
extern "C" {
    pub fn scx_bpf_cid_override___compat(
        cpu_to_cid__arena: *const s32,
        cpu_to_cid_cnt: u32,
        shard_start__arena: *const s32,
        shard_start_cnt: u32,
    );
}

pub unsafe fn scx_bpf_cid_override(
    cpu_to_cid: *const s32,
    cpu_to_cid_cnt: u32,
    shard_start: *const s32,
    shard_start_cnt: u32,
) {
    if bpf_ksym_exists(scx_bpf_cid_override___compat as unsafe extern "C" fn(*const s32, u32, *const s32, u32)) {
        scx_bpf_cid_override___compat(cpu_to_cid, cpu_to_cid_cnt, shard_start, shard_start_cnt);
    }
}

/**
 * __COMPAT_is_enq_cpu_selected - Test if SCX_ENQ_CPU_SELECTED is on
 * in a compatible way. We will preserve this __COMPAT helper until v6.16.
 *
 * @enq_flags: enqueue flags from ops.enqueue()
 *
 * Return: True if SCX_ENQ_CPU_SELECTED is turned on in @enq_flags
 */
pub unsafe fn __COMPAT_is_enq_cpu_selected(enq_flags: u64) -> bool {
    // C condition: #ifdef HAVE_SCX_ENQ_CPU_SELECTED. If unavailable in the
    // compile-time vmlinux.h, the original helper returns true.
    const HAVE_SCX_ENQ_CPU_SELECTED: bool = false;
    if HAVE_SCX_ENQ_CPU_SELECTED {
        /*
         * This is the case that a BPF code compiled against vmlinux.h
         * where the enum SCX_ENQ_CPU_SELECTED exists.
         *
         * The C code temporarily suspends macro expansion of
         * 'SCX_ENQ_CPU_SELECTED' with push_macro / undef / pop_macro.
         */
        extern "C" {
            static SCX_ENQ_CPU_SELECTED: scx_enq_flags;
        }

        /*
         * When the kernel did not have SCX_ENQ_CPU_SELECTED,
         * select_task_rq_scx() has never been skipped. Thus, this case
         * should be considered that the CPU has already been selected.
         */
        if !bpf_core_enum_value_exists(SCX_ENQ_CPU_SELECTED) {
            return true;
        }

        let flag = bpf_core_enum_value(SCX_ENQ_CPU_SELECTED);
        (enq_flags & flag) != 0
    } else {
        /*
         * This is the case that a BPF code compiled against vmlinux.h
         * where the enum SCX_ENQ_CPU_SELECTED does NOT exist.
         */
        true
    }
}

pub unsafe fn __COMPAT_scx_bpf_now() -> u64 {
    if bpf_ksym_exists(scx_bpf_now as unsafe extern "C" fn() -> u64) {
        scx_bpf_now()
    } else {
        bpf_ktime_get_ns()
    }
}

/*
 * v6.15: Introduce event counters.
 *
 * Preserve the following macro until v6.17.
 */
pub unsafe fn __COMPAT_scx_bpf_events(events: *mut core::ffi::c_void, size: size_t) {
    if bpf_ksym_exists(scx_bpf_events as unsafe extern "C" fn(*mut core::ffi::c_void, size_t)) {
        scx_bpf_events(events, size);
    }
}

/*
 * v6.15: Introduce NUMA-aware kfuncs to operate with per-node idle
 * cpumasks.
 *
 * Preserve the following __COMPAT_scx_*_node macros until v6.17.
 */
pub unsafe fn __COMPAT_scx_bpf_nr_node_ids() -> u32 {
    if bpf_ksym_exists(scx_bpf_nr_node_ids as unsafe extern "C" fn() -> u32) {
        scx_bpf_nr_node_ids()
    } else {
        1u32
    }
}

pub unsafe fn __COMPAT_scx_bpf_cpu_node(cpu: s32) -> s32 {
    if bpf_ksym_exists(scx_bpf_cpu_node as unsafe extern "C" fn(s32) -> s32) {
        scx_bpf_cpu_node(cpu)
    } else {
        0
    }
}

pub unsafe fn __COMPAT_scx_bpf_get_idle_cpumask_node(node: s32) -> *const cpumask {
    if bpf_ksym_exists(scx_bpf_get_idle_cpumask_node as unsafe extern "C" fn(s32) -> *const cpumask) {
        scx_bpf_get_idle_cpumask_node(node)
    } else {
        scx_bpf_get_idle_cpumask()
    }
}

pub unsafe fn __COMPAT_scx_bpf_get_idle_smtmask_node(node: s32) -> *const cpumask {
    if bpf_ksym_exists(scx_bpf_get_idle_smtmask_node as unsafe extern "C" fn(s32) -> *const cpumask) {
        scx_bpf_get_idle_smtmask_node(node)
    } else {
        scx_bpf_get_idle_smtmask()
    }
}

pub unsafe fn __COMPAT_scx_bpf_pick_idle_cpu_node(
    cpus_allowed: *const cpumask,
    node: s32,
    flags: u64,
) -> s32 {
    if bpf_ksym_exists(scx_bpf_pick_idle_cpu_node as unsafe extern "C" fn(*const cpumask, s32, u64) -> s32) {
        scx_bpf_pick_idle_cpu_node(cpus_allowed, node, flags)
    } else {
        scx_bpf_pick_idle_cpu(cpus_allowed, flags)
    }
}

pub unsafe fn __COMPAT_scx_bpf_pick_any_cpu_node(
    cpus_allowed: *const cpumask,
    node: s32,
    flags: u64,
) -> s32 {
    if bpf_ksym_exists(scx_bpf_pick_any_cpu_node as unsafe extern "C" fn(*const cpumask, s32, u64) -> s32) {
        scx_bpf_pick_any_cpu_node(cpus_allowed, node, flags)
    } else {
        scx_bpf_pick_any_cpu(cpus_allowed, flags)
    }
}

/*
 * v6.19: To work around BPF maximum parameter limit, the following kfuncs are
 * replaced with variants that pack scalar arguments in a struct. Wrappers are
 * provided to maintain source compatibility.
 *
 * v6.13: scx_bpf_dsq_insert_vtime() renaming is also handled here. See the
 * block on dispatch renaming above for more details.
 *
 * The kernel will carry the compat variants until v6.23 to maintain binary
 * compatibility. After v6.23 release, remove the compat handling and move the
 * wrappers to common.bpf.h.
 */
extern "C" {
    pub fn scx_bpf_select_cpu_and___compat(
        p: *mut task_struct,
        prev_cpu: s32,
        wake_flags: u64,
        cpus_allowed: *const cpumask,
        flags: u64,
    ) -> s32;
    pub fn scx_bpf_dispatch_vtime___compat(
        p: *mut task_struct,
        dsq_id: u64,
        slice: u64,
        vtime: u64,
        enq_flags: u64,
    );
    pub fn scx_bpf_dsq_insert_vtime___compat(
        p: *mut task_struct,
        dsq_id: u64,
        slice: u64,
        vtime: u64,
        enq_flags: u64,
    );
}

/**
 * scx_bpf_select_cpu_and - Pick an idle CPU usable by task @p
 * @p: task_struct to select a CPU for
 * @prev_cpu: CPU @p was on previously
 * @wake_flags: %SCX_WAKE_* flags
 * @cpus_allowed: cpumask of allowed CPUs
 * @flags: %SCX_PICK_IDLE* flags
 *
 * Inline wrapper that packs scalar arguments into a struct and calls
 * __scx_bpf_select_cpu_and(). See __scx_bpf_select_cpu_and() for details.
 */
pub unsafe fn scx_bpf_select_cpu_and(
    p: *mut task_struct,
    prev_cpu: s32,
    wake_flags: u64,
    cpus_allowed: *const cpumask,
    flags: u64,
) -> s32 {
    if bpf_core_type_exists::<scx_bpf_select_cpu_and_args>() {
        // Field initialization depends on the external definition of
        // struct scx_bpf_select_cpu_and_args.
        let args = core::mem::MaybeUninit::<scx_bpf_select_cpu_and_args>::zeroed();
        let args = args.assume_init();
        __scx_bpf_select_cpu_and(p, cpus_allowed, &args)
    } else {
        scx_bpf_select_cpu_and___compat(p, prev_cpu, wake_flags, cpus_allowed, flags)
    }
}

/*
 * scx_bpf_select_cpu_and() is now an inline wrapper. Use this instead of
 * bpf_ksym_exists(scx_bpf_select_cpu_and) to test availability.
 */
pub unsafe fn __COMPAT_HAS_scx_bpf_select_cpu_and() -> bool {
    bpf_core_type_exists::<scx_bpf_select_cpu_and_args>()
        || bpf_ksym_exists(scx_bpf_select_cpu_and___compat as unsafe extern "C" fn(*mut task_struct, s32, u64, *const cpumask, u64) -> s32)
}

/**
 * scx_bpf_dsq_insert_vtime - Insert a task into the vtime priority queue of a DSQ
 * @p: task_struct to insert
 * @dsq_id: DSQ to insert into
 * @slice: duration @p can run for in nsecs, 0 to keep the current value
 * @vtime: @p's ordering inside the vtime-sorted queue of the target DSQ
 * @enq_flags: SCX_ENQ_*
 *
 * Inline wrapper that packs scalar arguments into a struct and calls
 * __scx_bpf_dsq_insert_vtime(). See __scx_bpf_dsq_insert_vtime() for details.
 */
pub unsafe fn scx_bpf_dsq_insert_vtime(
    p: *mut task_struct,
    dsq_id: u64,
    slice: u64,
    vtime: u64,
    enq_flags: u64,
) -> bool {
    if bpf_core_type_exists::<scx_bpf_dsq_insert_vtime_args>() {
        // Field initialization depends on the external definition of
        // struct scx_bpf_dsq_insert_vtime_args.
        let args = core::mem::MaybeUninit::<scx_bpf_dsq_insert_vtime_args>::zeroed();
        let args = args.assume_init();
        __scx_bpf_dsq_insert_vtime(p, &args)
    } else if bpf_ksym_exists(scx_bpf_dsq_insert_vtime___compat as unsafe extern "C" fn(*mut task_struct, u64, u64, u64, u64)) {
        scx_bpf_dsq_insert_vtime___compat(p, dsq_id, slice, vtime, enq_flags);
        true
    } else {
        scx_bpf_dispatch_vtime___compat(p, dsq_id, slice, vtime, enq_flags);
        true
    }
}

/*
 * v6.19: scx_bpf_dsq_insert() now returns bool instead of void. Move
 * scx_bpf_dsq_insert() decl to common.bpf.h and drop compat helper after v6.22.
 * The extra ___compat suffix is to work around libbpf not ignoring __SUFFIX on
 * kernel side. The entire suffix can be dropped later.
 *
 * v6.13: scx_bpf_dsq_insert() renaming is also handled here. See the block on
 * dispatch renaming above for more details.
 */
extern "C" {
    pub fn scx_bpf_dsq_insert___v2___compat(
        p: *mut task_struct,
        dsq_id: u64,
        slice: u64,
        enq_flags: u64,
    ) -> bool;
    pub fn scx_bpf_dsq_insert___v1(
        p: *mut task_struct,
        dsq_id: u64,
        slice: u64,
        enq_flags: u64,
    );
    pub fn scx_bpf_dispatch___compat(
        p: *mut task_struct,
        dsq_id: u64,
        slice: u64,
        enq_flags: u64,
    );
}

pub unsafe fn scx_bpf_dsq_insert(
    p: *mut task_struct,
    dsq_id: u64,
    slice: u64,
    enq_flags: u64,
) -> bool {
    if bpf_ksym_exists(scx_bpf_dsq_insert___v2___compat as unsafe extern "C" fn(*mut task_struct, u64, u64, u64) -> bool) {
        scx_bpf_dsq_insert___v2___compat(p, dsq_id, slice, enq_flags)
    } else if bpf_ksym_exists(scx_bpf_dsq_insert___v1 as unsafe extern "C" fn(*mut task_struct, u64, u64, u64)) {
        scx_bpf_dsq_insert___v1(p, dsq_id, slice, enq_flags);
        true
    } else {
        scx_bpf_dispatch___compat(p, dsq_id, slice, enq_flags);
        true
    }
}

/*
 * v6.19: scx_bpf_task_set_slice() and scx_bpf_task_set_dsq_vtime() added to for
 * sub-sched authority checks. Drop the wrappers and move the decls to
 * common.bpf.h after v6.22.
 */
extern "C" {
    pub fn scx_bpf_task_set_slice___new(p: *mut task_struct, slice: u64) -> bool;
    pub fn scx_bpf_task_set_dsq_vtime___new(p: *mut task_struct, vtime: u64) -> bool;
}

pub unsafe fn scx_bpf_task_set_slice(p: *mut task_struct, slice: u64) {
    if bpf_ksym_exists(scx_bpf_task_set_slice___new as unsafe extern "C" fn(*mut task_struct, u64) -> bool) {
        scx_bpf_task_set_slice___new(p, slice);
    } else {
        // C fallback: p->scx.slice = slice;
        // Requires the external Rust definition of task_struct with scx.slice.
        let _ = (p, slice);
    }
}

pub unsafe fn scx_bpf_task_set_dsq_vtime(p: *mut task_struct, vtime: u64) {
    if bpf_ksym_exists(scx_bpf_task_set_dsq_vtime___new as unsafe extern "C" fn(*mut task_struct, u64) -> bool) {
        scx_bpf_task_set_dsq_vtime___new(p, vtime);
    } else {
        // C fallback: p->scx.dsq_vtime = vtime;
        // Requires the external Rust definition of task_struct with scx.dsq_vtime.
        let _ = (p, vtime);
    }
}

/*
 * v6.19: The new void variant can be called from anywhere while the older v1
 * variant can only be called from ops.cpu_release(). The double ___ prefixes on
 * the v2 variant need to be removed once libbpf is updated to ignore ___ prefix
 * on kernel side. Drop the wrapper and move the decl to common.bpf.h after
 * v6.22.
 */
extern "C" {
    pub fn scx_bpf_reenqueue_local___v1() -> u32;
    pub fn scx_bpf_reenqueue_local___v2___compat();
}

pub unsafe fn __COMPAT_scx_bpf_reenqueue_local_from_anywhere() -> bool {
    bpf_ksym_exists(scx_bpf_reenqueue_local___v2___compat as unsafe extern "C" fn())
}

pub unsafe fn scx_bpf_reenqueue_local() {
    if __COMPAT_scx_bpf_reenqueue_local_from_anywhere() {
        scx_bpf_reenqueue_local___v2___compat();
    } else {
        scx_bpf_reenqueue_local___v1();
    }
}

/*
 * v7.1: New scx_bpf_dsq_reenq() that allows re-enqueues on more DSQs. This
 * will eventually deprecate scx_bpf_reenqueue_local().
 */
extern "C" {
    pub fn scx_bpf_dsq_reenq___compat(dsq_id: u64, reenq_flags: u64);
}

pub unsafe fn __COMPAT_has_generic_reenq() -> bool {
    bpf_ksym_exists(scx_bpf_dsq_reenq___compat as unsafe extern "C" fn(u64, u64))
}

extern "C" {
    static SCX_DSQ_LOCAL: u64;
}

pub unsafe fn scx_bpf_dsq_reenq(dsq_id: u64, reenq_flags: u64) {
    if bpf_ksym_exists(scx_bpf_dsq_reenq___compat as unsafe extern "C" fn(u64, u64)) {
        scx_bpf_dsq_reenq___compat(dsq_id, reenq_flags);
    } else if dsq_id == SCX_DSQ_LOCAL && reenq_flags == 0 {
        scx_bpf_reenqueue_local();
    } else {
        scx_bpf_error(b"kernel too old to reenqueue foreign local or user DSQs\0".as_ptr() as *const core::ffi::c_char);
    }
}

/*
 * Define sched_ext_ops. See compat.h::SCX_OPS_OPEN() for how backward
 * compatibility is handled (this macro can be expanded to emit multiple
 * variants for incompatible op changes; SCX_OPS_OPEN() handles purely
 * additive changes at load time).
 *
 * C macro preserved as source-level intent:
 *   SEC(".struct_ops.link")
 *   struct sched_ext_ops __name = { __VA_ARGS__, };
 */

/*
 * Define a cid-form sched_ext_ops. Programs targeting this struct_ops type
 * use cid-form callback signatures (select_cid, set_cmask, cid_online/offline,
 * dispatch with cid arg, etc.) and may only call the cid-form scx_bpf_*
 * kfuncs (kick_cid, task_cid, this_cid, ...).
 *
 * C macro preserved as source-level intent:
 *   SEC(".struct_ops.link")
 *   struct sched_ext_ops_cid __name = { __VA_ARGS__, };
 */
