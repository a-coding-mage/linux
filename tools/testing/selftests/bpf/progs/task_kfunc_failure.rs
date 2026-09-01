// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// <vmlinux.h>
// <bpf/bpf_tracing.h>
// <bpf/bpf_helpers.h>
// ../bpf_experimental.h
// bpf_misc.h
// task_kfunc_common.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type u64 = u64;
type s32 = i32;

const BPF_NOEXIST: u64 = 1;
const ENOENT: i32 = 2;
const EEXIST: i32 = 17;

#[repr(C)]
pub struct task_struct {
    pub pid: s32,
    pub comm: [::core::ffi::c_char; 16],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __tasks_kfunc_map_value {
    pub task: *mut task_struct,
}

#[repr(C)]
pub struct task_kptr_lock_value {
    pub lock: bpf_spin_lock,
    pub task: *mut task_struct,
}

unsafe extern "C" {
    static mut __tasks_kfunc_map: ::core::ffi::c_void;
    static mut task_kptr_lock_map: ::core::ffi::c_void;

    fn tasks_kfunc_map_insert(task: *mut task_struct) -> i32;
    fn tasks_kfunc_map_value_lookup(task: *mut task_struct) -> *mut __tasks_kfunc_map_value;
    fn bpf_task_acquire(task: *mut task_struct) -> *mut task_struct;
    fn bpf_task_release(task: *mut task_struct);
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_kptr_xchg(kptr: *mut *mut task_struct, ptr: *mut task_struct) -> *mut task_struct;
    fn bpf_probe_read_kernel(
        dst: *mut ::core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const ::core::ffi::c_void,
    ) -> i64;
    fn bpf_map_update_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
        value: *const ::core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_lookup_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_obj_new__tasks_kfunc_map_value() -> *mut __tasks_kfunc_map_value;
    fn bpf_obj_drop(local: *mut __tasks_kfunc_map_value);
    fn bpf_task_from_pid(pid: s32) -> *mut task_struct;
    fn bpf_task_from_vpid(pid: s32) -> *mut task_struct;
    fn bpf_strncmp(
        s1: *const ::core::ffi::c_char,
        s1_sz: u32,
        s2: *const ::core::ffi::c_char,
    ) -> i32;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_preempt_disable();
    fn bpf_preempt_enable();
    fn bpf_local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn bpf_local_irq_restore(flags: *mut ::core::ffi::c_ulong);
    fn __sink(ptr: *mut task_struct);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Prototype for all of the program trace events below:
 *
 * TRACE_EVENT(task_newtask,
 *         TP_PROTO(struct task_struct *p, u64 clone_flags)
 */

unsafe fn insert_lookup_task(task: *mut task_struct) -> *mut __tasks_kfunc_map_value {
    let status: i32;

    status = unsafe { tasks_kfunc_map_insert(task) };
    if status != 0 {
        return ::core::ptr::null_mut();
    }

    unsafe { tasks_kfunc_map_value_lookup(task) }
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_acquire_untrusted(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;
    let v: *mut __tasks_kfunc_map_value;

    v = unsafe { insert_lookup_task(task) };
    if v.is_null() {
        return 0;
    }

    /* Can't invoke bpf_task_acquire() on an untrusted pointer. */
    acquired = unsafe { bpf_task_acquire((*v).task) };
    if acquired.is_null() {
        return 0;
    }

    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("R1 is fp expected STRUCT task_struct")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_acquire_fp(
    _task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;
    let mut stack_task: *mut task_struct = (&clone_flags as *const u64).cast_mut().cast();

    /* Can't invoke bpf_task_acquire() on a random frame pointer. */
    acquired = unsafe { bpf_task_acquire((&mut stack_task as *mut *mut task_struct).cast()) };
    if acquired.is_null() {
        return 0;
    }

    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("kretprobe/free_task")
// __failure __msg("calling kernel function bpf_task_acquire is not allowed")
#[unsafe(no_mangle)]
#[unsafe(link_section = "kretprobe/free_task")]
pub unsafe extern "C" fn task_kfunc_acquire_unsafe_kretprobe(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;

    /* Can't call bpf_task_acquire() or bpf_task_release() in an untrusted prog. */
    acquired = unsafe { bpf_task_acquire(task) };
    if acquired.is_null() {
        return 0;
    }
    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("kretprobe/free_task")
// __failure __msg("calling kernel function bpf_task_acquire is not allowed")
#[unsafe(no_mangle)]
#[unsafe(link_section = "kretprobe/free_task")]
pub unsafe extern "C" fn task_kfunc_acquire_unsafe_kretprobe_rcu(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;

    unsafe { bpf_rcu_read_lock() };
    if task.is_null() {
        unsafe { bpf_rcu_read_unlock() };
        return 0;
    }
    /* Can't call bpf_task_acquire() or bpf_task_release() in an untrusted prog. */
    acquired = unsafe { bpf_task_acquire(task) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    }
    unsafe { bpf_rcu_read_unlock() };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_acquire_null(
    _task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;

    /* Can't invoke bpf_task_acquire() on a NULL pointer. */
    acquired = unsafe { bpf_task_acquire(::core::ptr::null_mut()) };
    if acquired.is_null() {
        return 0;
    }
    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Unreleased reference")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_acquire_unreleased(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;

    acquired = unsafe { bpf_task_acquire(task) };

    /* Acquired task is never released. */
    unsafe { __sink(acquired) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Unreleased reference")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_xchg_unreleased(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let kptr: *mut task_struct;
    let v: *mut __tasks_kfunc_map_value;

    v = unsafe { insert_lookup_task(task) };
    if v.is_null() {
        return 0;
    }

    kptr = unsafe { bpf_kptr_xchg(&mut (*v).task, ::core::ptr::null_mut()) };
    if kptr.is_null() {
        return 0;
    }

    /* Kptr retrieved from map is never released. */

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_acquire_release_no_null_check(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;

    acquired = unsafe { bpf_task_acquire(task) };
    /* Can't invoke bpf_task_release() on an acquired task without a NULL check. */
    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_release_untrusted(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let v: *mut __tasks_kfunc_map_value;

    v = unsafe { insert_lookup_task(task) };
    if v.is_null() {
        return 0;
    }

    /* Can't invoke bpf_task_release() on an untrusted pointer. */
    unsafe { bpf_task_release((*v).task) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_release_fp(
    _task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct = (&clone_flags as *const u64).cast_mut().cast();

    /* Cannot release random frame pointer. */
    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_release_null(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let mut local: __tasks_kfunc_map_value = __tasks_kfunc_map_value {
        task: ::core::ptr::null_mut(),
    };
    let v: *mut __tasks_kfunc_map_value;
    let mut status: i64;
    let acquired: *mut task_struct;
    let old: *mut task_struct;
    let mut pid: s32 = 0;

    status = unsafe {
        bpf_probe_read_kernel(
            (&mut pid as *mut s32).cast(),
            ::core::mem::size_of_val(&pid) as u32,
            (&(*task).pid as *const s32).cast(),
        )
    };
    if status != 0 {
        return 0;
    }

    local.task = ::core::ptr::null_mut();
    status = unsafe {
        bpf_map_update_elem(
            (&mut __tasks_kfunc_map as *mut ::core::ffi::c_void),
            (&pid as *const s32).cast(),
            (&local as *const __tasks_kfunc_map_value).cast(),
            BPF_NOEXIST,
        )
    };
    if status != 0 {
        return status as i32;
    }

    v = unsafe {
        bpf_map_lookup_elem(
            (&mut __tasks_kfunc_map as *mut ::core::ffi::c_void),
            (&pid as *const s32).cast(),
        )
    }
    .cast();
    if v.is_null() {
        return -ENOENT;
    }

    acquired = unsafe { bpf_task_acquire(task) };
    if acquired.is_null() {
        return -EEXIST;
    }

    old = unsafe { bpf_kptr_xchg(&mut (*v).task, acquired) };

    /* old cannot be passed to bpf_task_release() without a NULL check. */
    unsafe { bpf_task_release(old) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_release_unacquired(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    /* Cannot release trusted task pointer which was not acquired. */
    unsafe { bpf_task_release(task) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("bpf_obj_drop cannot be used in tracing programs on types with NMI unsafe fields")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_obj_drop_with_kptr(
    _task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let local: *mut __tasks_kfunc_map_value;

    local = unsafe { bpf_obj_new__tasks_kfunc_map_value() };
    if local.is_null() {
        return 0;
    }

    unsafe { bpf_obj_drop(local) };
    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("bpf_obj_drop cannot be used in tracing programs on types with NMI unsafe fields")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_obj_drop_nmi_with_kptr(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let local: *mut __tasks_kfunc_map_value;
    let acquired: *mut task_struct;
    let old: *mut task_struct;

    let _ = clone_flags;

    local = unsafe { bpf_obj_new__tasks_kfunc_map_value() };
    if local.is_null() {
        return 0;
    }

    acquired = unsafe { bpf_task_acquire(task) };
    if !acquired.is_null() {
        old = unsafe { bpf_kptr_xchg(&mut (*local).task, acquired) };
        if !old.is_null() {
            unsafe { bpf_task_release(old) };
        }
    }

    unsafe { bpf_obj_drop(local) };
    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_from_pid_no_null_check(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;

    acquired = unsafe { bpf_task_from_pid((*task).pid) };

    /* Releasing bpf_task_from_pid() lookup without a NULL check. */
    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_from_vpid_no_null_check(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;

    acquired = unsafe { bpf_task_from_vpid((*task).pid) };

    /* Releasing bpf_task_from_vpid() lookup without a NULL check. */
    unsafe { bpf_task_release(acquired) };

    0
}

// SEC("lsm/task_free")
// __failure __msg("R1 must be a rcu pointer")
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm/task_free")]
pub unsafe extern "C" fn task_kfunc_from_lsm_task_free(task: *mut task_struct) -> i32 {
    let acquired: *mut task_struct;

    /* the argument of lsm task_free hook is untrusted. */
    acquired = unsafe { bpf_task_acquire(task) };
    if acquired.is_null() {
        return 0;
    }

    unsafe { bpf_task_release(acquired) };
    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("access beyond the end of member comm")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_access_comm1(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    unsafe { bpf_strncmp((*task).comm.as_ptr(), 17, c"foo".as_ptr()) };
    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("access beyond the end of member comm")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_access_comm2(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    unsafe { bpf_strncmp((*task).comm.as_ptr().add(1), 16, c"foo".as_ptr()) };
    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("write into memory")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_access_comm3(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    unsafe {
        bpf_probe_read_kernel(
            (*task).comm.as_mut_ptr().cast(),
            16,
            (*task).comm.as_ptr().cast(),
        )
    };
    0
}

// SEC("fentry/__set_task_comm")
// __failure __msg("R1 type=ptr_ expected")
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/__set_task_comm")]
pub unsafe extern "C" fn task_access_comm4(
    task: *mut task_struct,
    _buf: *const ::core::ffi::c_char,
    _exec: bool,
) -> i32 {
    /*
     * task->comm is a legacy ptr_to_btf_id. The verifier cannot guarantee
     * its safety. Hence it cannot be accessed with normal load insns.
     */
    unsafe { bpf_strncmp((*task).comm.as_ptr(), 16, c"foo".as_ptr()) };
    0
}

// SEC("tp_btf/task_newtask")
// __failure __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tp_btf/task_newtask")]
pub unsafe extern "C" fn task_kfunc_release_in_map(
    task: *mut task_struct,
    _clone_flags: u64,
) -> i32 {
    let local: *mut task_struct;
    let v: *mut __tasks_kfunc_map_value;

    if unsafe { tasks_kfunc_map_insert(task) } != 0 {
        return 0;
    }

    v = unsafe { tasks_kfunc_map_value_lookup(task) };
    if v.is_null() {
        return 0;
    }

    unsafe { bpf_rcu_read_lock() };
    local = unsafe { (*v).task };
    if local.is_null() {
        unsafe { bpf_rcu_read_unlock() };
        return 0;
    }
    /* Can't release a kptr that's still stored in a map. */
    unsafe { bpf_task_release(local) };
    unsafe { bpf_rcu_read_unlock() };

    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure __msg("R1 must be a rcu pointer")
#[unsafe(no_mangle)]
#[unsafe(link_section = "?fentry.s/sys_getpgid")]
pub unsafe extern "C" fn task_kfunc_acquire_after_final_spin_unlock() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let mut key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            (&mut task_kptr_lock_map as *mut ::core::ffi::c_void),
            (&mut key as *mut i32).cast(),
        )
    }
    .cast();
    if v.is_null() {
        return 0;
    }

    unsafe { bpf_spin_lock(&mut (*v).lock) };
    task = unsafe { (*v).task };
    unsafe { bpf_spin_unlock(&mut (*v).lock) };
    if task.is_null() {
        return 0;
    }

    acquired = unsafe { bpf_task_acquire(task) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    }
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure __msg("R1 must be a rcu pointer")
#[unsafe(no_mangle)]
#[unsafe(link_section = "?fentry.s/sys_getpgid")]
pub unsafe extern "C" fn task_kfunc_acquire_after_preempt_enable() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let mut key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            (&mut task_kptr_lock_map as *mut ::core::ffi::c_void),
            (&mut key as *mut i32).cast(),
        )
    }
    .cast();
    if v.is_null() {
        return 0;
    }

    unsafe { bpf_preempt_disable() };
    task = unsafe { (*v).task };
    unsafe { bpf_preempt_enable() };
    if task.is_null() {
        return 0;
    }

    acquired = unsafe { bpf_task_acquire(task) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    }
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure __msg("R1 must be a rcu pointer")
#[unsafe(no_mangle)]
#[unsafe(link_section = "?fentry.s/sys_getpgid")]
pub unsafe extern "C" fn task_kfunc_acquire_after_irq_restore() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            (&mut task_kptr_lock_map as *mut ::core::ffi::c_void),
            (&mut key as *mut i32).cast(),
        )
    }
    .cast();
    if v.is_null() {
        return 0;
    }

    unsafe { bpf_local_irq_save(&mut flags) };
    task = unsafe { (*v).task };
    unsafe { bpf_local_irq_restore(&mut flags) };
    if task.is_null() {
        return 0;
    }

    acquired = unsafe { bpf_task_acquire(task) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
