// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies: vmlinux.h, bpf/bpf_tracing.h, bpf/bpf_helpers.h,
// bpf_misc.h, cpumask_common.h.

type u64 = u64;

// TODO: supplied by cpumask_common.h in the original C translation unit.
const CPUMASK_KPTR_FIELDS_MAX: usize = 0;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct kptr_nested_array_2 {
    pub mask: *mut bpf_cpumask,
}

#[repr(C)]
pub struct kptr_nested_array_1 {
    /* Make btf_parse_fields() in map_create() return -E2BIG */
    pub d_2: [kptr_nested_array_2; CPUMASK_KPTR_FIELDS_MAX + 1],
}

#[repr(C)]
pub struct kptr_nested_array {
    pub d_1: kptr_nested_array_1,
}

// private(MASK_NESTED)
static mut global_mask_nested_arr: kptr_nested_array = kptr_nested_array {
    d_1: kptr_nested_array_1 {
        d_2: [kptr_nested_array_2 {
            mask: core::ptr::null_mut(),
        }; CPUMASK_KPTR_FIELDS_MAX + 1],
    },
};

/* Prototype for all of the program trace events below:
 *
 * TRACE_EVENT(task_newtask,
 *         TP_PROTO(struct task_struct *p, u64 clone_flags)
 */

// SEC("tp_btf/task_newtask")
// __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn test_alloc_no_release(task: *mut task_struct, clone_flags: u64) -> i32 {
    let mut cpumask: *mut bpf_cpumask;

    cpumask = create_cpumask();
    __sink(cpumask);

    /* cpumask is never released. */
    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("NULL pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn test_alloc_double_release(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut cpumask: *mut bpf_cpumask;

    cpumask = create_cpumask();

    /* cpumask is released twice. */
    bpf_cpumask_release(cpumask);
    bpf_cpumask_release(cpumask);

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("must be referenced")
#[no_mangle]
pub unsafe extern "C" fn test_acquire_wrong_cpumask(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut cpumask: *mut bpf_cpumask;

    /* Can't acquire a non-struct bpf_cpumask. */
    cpumask = bpf_cpumask_acquire((*task).cpus_ptr as *mut bpf_cpumask);
    __sink(cpumask);

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("bpf_cpumask_set_cpu R2 expected pointer to STRUCT bpf_cpumask")
#[no_mangle]
pub unsafe extern "C" fn test_mutate_cpumask(task: *mut task_struct, clone_flags: u64) -> i32 {
    /* Can't set the CPU of a non-struct bpf_cpumask. */
    bpf_cpumask_set_cpu(0, (*task).cpus_ptr as *mut bpf_cpumask);

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn test_insert_remove_no_release(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut cpumask: *mut bpf_cpumask;
    let mut v: *mut __cpumask_map_value;

    cpumask = create_cpumask();
    if cpumask.is_null() {
        return 0;
    }

    if cpumask_map_insert(cpumask) != 0 {
        return 0;
    }

    v = cpumask_map_value_lookup();
    if v.is_null() {
        return 0;
    }

    cpumask = bpf_kptr_xchg(&mut (*v).cpumask, core::ptr::null_mut());

    /* cpumask is never released. */
    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("NULL pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn test_cpumask_null(task: *mut task_struct, clone_flags: u64) -> i32 {
    /* NULL passed to kfunc. */
    bpf_cpumask_empty(core::ptr::null_mut());

    return 0;
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure __msg("R2 must be a rcu pointer")
#[no_mangle]
pub unsafe extern "C" fn test_global_mask_out_of_rcu() -> i32 {
    let mut local: *mut bpf_cpumask;
    let mut prev: *mut bpf_cpumask;

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    prev = bpf_kptr_xchg(&mut global_mask, local);
    if !prev.is_null() {
        bpf_cpumask_release(prev);
        err = 3;
        return 0;
    }

    /*
     * Use a sleepable program so explicit RCU is the only source of RCU
     * protection.
     */
    bpf_rcu_read_lock();
    local = global_mask;
    if local.is_null() {
        err = 4;
        bpf_rcu_read_unlock();
        return 0;
    }

    bpf_rcu_read_unlock();

    /* RCU region is exited before calling KF_RCU kfunc. */

    bpf_cpumask_test_cpu(0, local as *const cpumask);

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("NULL pointer passed to trusted R2")
#[no_mangle]
pub unsafe extern "C" fn test_global_mask_no_null_check(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut local: *mut bpf_cpumask;
    let mut prev: *mut bpf_cpumask;

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    prev = bpf_kptr_xchg(&mut global_mask, local);
    if !prev.is_null() {
        bpf_cpumask_release(prev);
        err = 3;
        return 0;
    }

    bpf_rcu_read_lock();
    local = global_mask;

    /* No NULL check is performed on global cpumask kptr. */
    bpf_cpumask_test_cpu(0, local as *const cpumask);

    bpf_rcu_read_unlock();

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to helper R2")
#[no_mangle]
pub unsafe extern "C" fn test_global_mask_rcu_no_null_check(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut prev: *mut bpf_cpumask;
    let mut curr: *mut bpf_cpumask;

    curr = bpf_cpumask_create();
    if curr.is_null() {
        return 0;
    }

    prev = bpf_kptr_xchg(&mut global_mask, curr);
    if !prev.is_null() {
        bpf_cpumask_release(prev);
    }

    bpf_rcu_read_lock();
    curr = global_mask;
    /* PTR_TO_BTF_ID | PTR_MAYBE_NULL | MEM_RCU passed to bpf_kptr_xchg() */
    prev = bpf_kptr_xchg(&mut global_mask, curr);
    bpf_rcu_read_unlock();
    if !prev.is_null() {
        bpf_cpumask_release(prev);
    }

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("has no valid kptr")
#[no_mangle]
pub unsafe extern "C" fn test_invalid_nested_array(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut local: *mut bpf_cpumask;
    let mut prev: *mut bpf_cpumask;

    local = create_cpumask();
    if local.is_null() {
        return 0;
    }

    prev = bpf_kptr_xchg(
        &mut global_mask_nested_arr.d_1.d_2[CPUMASK_KPTR_FIELDS_MAX].mask,
        local,
    );
    if !prev.is_null() {
        bpf_cpumask_release(prev);
        err = 3;
        return 0;
    }

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("type=scalar expected=fp")
#[no_mangle]
pub unsafe extern "C" fn test_populate_invalid_destination(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut invalid: *mut bpf_cpumask = 0x123456usize as *mut bpf_cpumask;
    let mut bits: u64;
    let mut ret: i32;

    ret = bpf_cpumask_populate(
        invalid,
        &mut bits as *mut u64 as *mut core::ffi::c_void,
        core::mem::size_of_val(&bits) as u32,
    );
    if ret == 0 {
        err = 2;
    }

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("leads to invalid memory access")
#[no_mangle]
pub unsafe extern "C" fn test_populate_invalid_source(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut garbage: *mut core::ffi::c_void = 0x123456usize as *mut core::ffi::c_void;
    let mut local: *mut bpf_cpumask;
    let mut ret: i32;

    local = create_cpumask();
    if local.is_null() {
        err = 1;
        return 0;
    }

    ret = bpf_cpumask_populate(local, garbage, 8);
    if ret == 0 {
        err = 2;
    }

    bpf_cpumask_release(local);

    return 0;
}

// SEC("tp_btf/task_newtask")
// __failure __msg("expected pointer to STRUCT bpf_cpumask but R1 has a pointer to STRUCT cpumask")
#[no_mangle]
pub unsafe extern "C" fn test_populate_borrowed_destination(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut bits: u64;
    let mut ret: i32;

    /*
     * task->cpus_ptr is a borrowed, read-only struct cpumask *, not an
     * owned struct bpf_cpumask *. The verifier must reject it as a
     * writable destination for bpf_cpumask_populate().
     */
    ret = bpf_cpumask_populate(
        (*task).cpus_ptr as *mut bpf_cpumask,
        &mut bits as *mut u64 as *mut core::ffi::c_void,
        core::mem::size_of_val(&bits) as u32,
    );
    if ret == 0 {
        err = 2;
    }

    return 0;
}
