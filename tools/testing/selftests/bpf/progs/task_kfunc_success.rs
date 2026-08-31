// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include "../bpf_experimental.h"
// #include "bpf_misc.h"
// #include "task_kfunc_common.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u64 = u64;
type s32 = i32;

#[repr(C)]
pub struct task_struct {
    pub pid: s32,
    pub comm: [i8; 16],
    pub group_leader: *mut task_struct,
    pub rcu_users: i32,
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_kptr_lock_value {
    pub lock: bpf_spin_lock,
    pub task: *mut task_struct,
}

#[repr(C)]
pub struct __tasks_kfunc_map_value {
    pub task: *mut task_struct,
}

unsafe extern "C" {
    #[link_name = "task_kptr_lock_map"]
    static mut task_kptr_lock_map: core::ffi::c_void;

    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_release(p: *mut task_struct);
    fn bpf_task_from_pid(pid: s32) -> *mut task_struct;
    fn bpf_task_from_vpid(pid: s32) -> *mut task_struct;
    fn bpf_testmod_test_mod_kfunc(i: i32);
    fn invalid_kfunc();

    fn bpf_task_acquire(p: *mut task_struct) -> *mut task_struct;
    fn bpf_task_acquire___one(task: *mut task_struct) -> *mut task_struct;
    /* The two-param bpf_task_acquire doesn't exist */
    fn bpf_task_acquire___two(
        p: *mut task_struct,
        ctx: *mut core::ffi::c_void,
    ) -> *mut task_struct;
    /* Incorrect type for first param */
    fn bpf_task_acquire___three(ctx: *mut core::ffi::c_void) -> *mut task_struct;

    fn tasks_kfunc_map_insert(task: *mut task_struct) -> isize;
    fn tasks_kfunc_map_value_lookup(task: *mut task_struct) -> *mut __tasks_kfunc_map_value;
    fn bpf_kptr_xchg(
        kptr: *mut *mut task_struct,
        ptr: *mut task_struct,
    ) -> *mut task_struct;
    fn bpf_obj_new___tasks_kfunc_map_value() -> *mut __tasks_kfunc_map_value;
    fn bpf_obj_drop(ptr: *mut __tasks_kfunc_map_value);
    fn bpf_probe_read_kernel(
        dst: *mut core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_strncmp(s1: *const i8, s1_sz: u32, s2: *const i8) -> i32;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_preempt_disable();
    fn bpf_preempt_enable();
    fn bpf_local_irq_save(flags: *mut u64);
    fn bpf_local_irq_restore(flags: *mut u64);
}

unsafe fn bpf_ksym_exists<T>(sym: T) -> bool {
    let _ = sym;
    true
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut err: i32 = 0;
#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

/* Prototype for all of the program trace events below:
 *
 * TRACE_EVENT(task_newtask,
 *         TP_PROTO(struct task_struct *p, u64 clone_flags)
 */

unsafe fn is_test_kfunc_task() -> bool {
    let cur_pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;

    unsafe { pid == cur_pid }
}

unsafe fn test_acquire_release(task: *mut task_struct) -> i32 {
    let mut acquired: *mut task_struct = core::ptr::null_mut();

    if !unsafe { bpf_ksym_exists(bpf_task_acquire as unsafe extern "C" fn(*mut task_struct) -> *mut task_struct) } {
        unsafe {
            err = 3;
        }
        return 0;
    }
    if !unsafe { bpf_ksym_exists(bpf_testmod_test_mod_kfunc as unsafe extern "C" fn(i32)) } {
        unsafe {
            err = 4;
        }
        return 0;
    }
    if unsafe { bpf_ksym_exists(invalid_kfunc as unsafe extern "C" fn()) } {
        /* the verifier's dead code elimination should remove this */
        unsafe {
            err = 5;
        }
        loop {}
    }

    acquired = unsafe { bpf_task_acquire(task) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    } else {
        unsafe {
            err = 6;
        }
    }

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_kfunc_flavor_relo(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut acquired: *mut task_struct = core::ptr::null_mut();
    let mut fake_ctx: i32 = 42;
    let _ = clone_flags;

    if unsafe { bpf_ksym_exists(bpf_task_acquire___one as unsafe extern "C" fn(*mut task_struct) -> *mut task_struct) } {
        acquired = unsafe { bpf_task_acquire___one(task) };
    } else if unsafe { bpf_ksym_exists(bpf_task_acquire___two as unsafe extern "C" fn(*mut task_struct, *mut core::ffi::c_void) -> *mut task_struct) } {
        /* Here, bpf_object__resolve_ksym_func_btf_id's find_ksym_btf_id
         * call will find vmlinux's bpf_task_acquire, but subsequent
         * bpf_core_types_are_compat will fail
         */
        acquired = unsafe {
            bpf_task_acquire___two(task, &mut fake_ctx as *mut _ as *mut core::ffi::c_void)
        };
        unsafe {
            err = 3;
        }
        return 0;
    } else if unsafe { bpf_ksym_exists(bpf_task_acquire___three as unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut task_struct) } {
        /* bpf_core_types_are_compat will fail similarly to above case */
        acquired = unsafe {
            bpf_task_acquire___three(&mut fake_ctx as *mut _ as *mut core::ffi::c_void)
        };
        unsafe {
            err = 4;
        }
        return 0;
    }

    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    } else {
        unsafe {
            err = 5;
        }
    }
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_kfunc_flavor_relo_not_found(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let _ = task;
    let _ = clone_flags;

    /* Neither symbol should successfully resolve.
     * Success or failure of one ___flavor should not affect others
     */
    if unsafe { bpf_ksym_exists(bpf_task_acquire___two as unsafe extern "C" fn(*mut task_struct, *mut core::ffi::c_void) -> *mut task_struct) } {
        unsafe {
            err = 1;
        }
    } else if unsafe { bpf_ksym_exists(bpf_task_acquire___three as unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut task_struct) } {
        unsafe {
            err = 2;
        }
    }

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_acquire_release_argument(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let _ = clone_flags;
    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    unsafe { test_acquire_release(task) }
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_acquire_release_current(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let _ = task;
    let _ = clone_flags;
    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    unsafe { test_acquire_release(bpf_get_current_task_btf()) }
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_acquire_leave_in_map(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let status: isize;
    let _ = clone_flags;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    status = unsafe { tasks_kfunc_map_insert(task) };
    if status != 0 {
        unsafe {
            err = 1;
        }
    }

    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_xchg_release(ctx: *const core::ffi::c_void) -> i32 {
    let mut task: *mut task_struct;
    let mut kptr: *mut task_struct;
    let mut acquired: *mut task_struct;
    let v: *mut __tasks_kfunc_map_value;
    let local: *mut __tasks_kfunc_map_value;
    let mut refcnt: i32 = 0;
    let mut refcnt_after_drop: i32 = 0;
    let status: isize;

    let _ = ctx;

    task = unsafe { bpf_get_current_task_btf() };
    status = unsafe { tasks_kfunc_map_insert(task) };
    if status != 0 {
        unsafe {
            err = 1;
        }
        return 0;
    }

    v = unsafe { tasks_kfunc_map_value_lookup(task) };
    if v.is_null() {
        unsafe {
            err = 2;
        }
        return 0;
    }

    kptr = unsafe { bpf_kptr_xchg(&mut (*v).task, core::ptr::null_mut()) };
    if kptr.is_null() {
        unsafe {
            err = 3;
        }
        return 0;
    }

    local = unsafe { bpf_obj_new___tasks_kfunc_map_value() };
    if local.is_null() {
        unsafe {
            err = 4;
            bpf_task_release(kptr);
        }
        return 0;
    }

    kptr = unsafe { bpf_kptr_xchg(&mut (*local).task, kptr) };
    if !kptr.is_null() {
        unsafe {
            err = 5;
            bpf_obj_drop(local);
            bpf_task_release(kptr);
        }
        return 0;
    }

    kptr = unsafe { bpf_kptr_xchg(&mut (*local).task, core::ptr::null_mut()) };
    if kptr.is_null() {
        unsafe {
            err = 6;
            bpf_obj_drop(local);
        }
        return 0;
    }

    /* Stash a copy into local kptr and check if it is released recursively. */
    acquired = unsafe { bpf_task_acquire(kptr) };
    if acquired.is_null() {
        unsafe {
            err = 7;
            bpf_obj_drop(local);
            bpf_task_release(kptr);
        }
        return 0;
    }
    unsafe {
        bpf_probe_read_kernel(
            &mut refcnt as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&refcnt),
            &(*acquired).rcu_users as *const _ as *const core::ffi::c_void,
        );
    }

    acquired = unsafe { bpf_kptr_xchg(&mut (*local).task, acquired) };
    if !acquired.is_null() {
        unsafe {
            err = 8;
            bpf_obj_drop(local);
            bpf_task_release(kptr);
            bpf_task_release(acquired);
        }
        return 0;
    }

    unsafe { bpf_obj_drop(local) };

    unsafe {
        bpf_probe_read_kernel(
            &mut refcnt_after_drop as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&refcnt_after_drop),
            &(*kptr).rcu_users as *const _ as *const core::ffi::c_void,
        );
    }
    if refcnt != refcnt_after_drop + 1 {
        unsafe {
            err = 9;
            bpf_task_release(kptr);
        }
        return 0;
    }

    unsafe { bpf_task_release(kptr) };
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_map_acquire_release(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let mut kptr: *mut task_struct;
    let v: *mut __tasks_kfunc_map_value;
    let status: isize;
    let _ = clone_flags;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    status = unsafe { tasks_kfunc_map_insert(task) };
    if status != 0 {
        unsafe {
            err = 1;
        }
        return 0;
    }

    v = unsafe { tasks_kfunc_map_value_lookup(task) };
    if v.is_null() {
        unsafe {
            err = 2;
        }
        return 0;
    }

    unsafe { bpf_rcu_read_lock() };
    kptr = unsafe { (*v).task };
    if kptr.is_null() {
        unsafe {
            err = 3;
        }
    } else {
        kptr = unsafe { bpf_task_acquire(kptr) };
        if kptr.is_null() {
            unsafe {
                err = 4;
            }
        } else {
            unsafe { bpf_task_release(kptr) };
        }
    }
    unsafe { bpf_rcu_read_unlock() };

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_current_acquire_release(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let current: *mut task_struct;
    let acquired: *mut task_struct;
    let _ = task;
    let _ = clone_flags;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    current = unsafe { bpf_get_current_task_btf() };
    acquired = unsafe { bpf_task_acquire(current) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    } else {
        unsafe {
            err = 1;
        }
    }

    0
}

unsafe fn lookup_compare_pid(p: *const task_struct) {
    let acquired: *mut task_struct;

    acquired = unsafe { bpf_task_from_pid((*p).pid) };
    if acquired.is_null() {
        unsafe {
            err = 1;
        }
        return;
    }

    if unsafe { (*acquired).pid != (*p).pid } {
        unsafe {
            err = 2;
        }
    }
    unsafe { bpf_task_release(acquired) };
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_from_pid_arg(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let _ = clone_flags;
    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    unsafe { lookup_compare_pid(task) };
    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_from_pid_current(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let _ = task;
    let _ = clone_flags;
    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    unsafe { lookup_compare_pid(bpf_get_current_task_btf()) };
    0
}

unsafe fn is_pid_lookup_valid(pid: s32) -> i32 {
    let acquired: *mut task_struct;

    acquired = unsafe { bpf_task_from_pid(pid) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
        return 1;
    }

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_from_pid_invalid(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let _ = clone_flags;
    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    unsafe {
        bpf_strncmp((*task).comm.as_ptr(), 12, c"foo".as_ptr());
        bpf_strncmp((*task).comm.as_ptr(), 16, c"foo".as_ptr());
        bpf_strncmp((*task).comm.as_ptr().add(8), 4, c"foo".as_ptr());
    }

    if unsafe { is_pid_lookup_valid(-1) } != 0 {
        unsafe {
            err = 1;
        }
        return 0;
    }

    if unsafe { is_pid_lookup_valid(0xcafef00du32 as s32) } != 0 {
        unsafe {
            err = 2;
        }
        return 0;
    }

    0
}

#[unsafe(link_section = "tp_btf/task_newtask")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_trusted_walked(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let acquired: *mut task_struct;
    let _ = clone_flags;

    /* task->group_leader is listed as a trusted, non-NULL field of task struct. */
    acquired = unsafe { bpf_task_acquire((*task).group_leader) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    } else {
        unsafe {
            err = 1;
        }
    }

    0
}

// SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_spin_unlock_non_sleepable() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_spin_lock(&mut (*v).lock);
        task = (*v).task;
        bpf_spin_unlock(&mut (*v).lock);
    }
    if task.is_null() {
        return 0;
    }

    acquired = unsafe { bpf_task_acquire(task) };
    if !acquired.is_null() {
        unsafe { bpf_task_release(acquired) };
    }
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_spin_unlock_explicit_rcu() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_rcu_read_lock();
        bpf_spin_lock(&mut (*v).lock);
        task = (*v).task;
        bpf_spin_unlock(&mut (*v).lock);
    }
    if !task.is_null() {
        acquired = unsafe { bpf_task_acquire(task) };
        if !acquired.is_null() {
            unsafe { bpf_task_release(acquired) };
        }
    }
    unsafe { bpf_rcu_read_unlock() };
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_spin_unlock_preempt_disabled() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_preempt_disable();
        bpf_spin_lock(&mut (*v).lock);
        task = (*v).task;
        bpf_spin_unlock(&mut (*v).lock);
    }
    if !task.is_null() {
        acquired = unsafe { bpf_task_acquire(task) };
        if !acquired.is_null() {
            unsafe { bpf_task_release(acquired) };
        }
    }
    unsafe { bpf_preempt_enable() };
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_spin_unlock_irq_disabled() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let mut flags: u64 = 0;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_local_irq_save(&mut flags);
        bpf_spin_lock(&mut (*v).lock);
        task = (*v).task;
        bpf_spin_unlock(&mut (*v).lock);
    }
    if !task.is_null() {
        acquired = unsafe { bpf_task_acquire(task) };
        if !acquired.is_null() {
            unsafe { bpf_task_release(acquired) };
        }
    }
    unsafe { bpf_local_irq_restore(&mut flags) };
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_rcu_unlock_preempt_disabled() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_preempt_disable();
        bpf_rcu_read_lock();
        task = (*v).task;
        bpf_rcu_read_unlock();
    }
    if !task.is_null() {
        acquired = unsafe { bpf_task_acquire(task) };
        if !acquired.is_null() {
            unsafe { bpf_task_release(acquired) };
        }
    }
    unsafe { bpf_preempt_enable() };
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_rcu_unlock_irq_disabled() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let mut flags: u64 = 0;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_local_irq_save(&mut flags);
        bpf_rcu_read_lock();
        task = (*v).task;
        bpf_rcu_read_unlock();
    }
    if !task.is_null() {
        acquired = unsafe { bpf_task_acquire(task) };
        if !acquired.is_null() {
            unsafe { bpf_task_release(acquired) };
        }
    }
    unsafe { bpf_local_irq_restore(&mut flags) };
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_preempt_enable_explicit_rcu() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_preempt_disable();
        task = (*v).task;
        bpf_rcu_read_lock();
        bpf_preempt_enable();
    }
    if !task.is_null() {
        acquired = unsafe { bpf_task_acquire(task) };
        if !acquired.is_null() {
            unsafe { bpf_task_release(acquired) };
        }
    }
    unsafe { bpf_rcu_read_unlock() };
    0
}

// SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_kfunc_acquire_after_irq_restore_explicit_rcu() -> i32 {
    let v: *mut task_kptr_lock_value;
    let task: *mut task_struct;
    let acquired: *mut task_struct;
    let mut flags: u64 = 0;
    let key: i32 = 0;

    v = unsafe {
        bpf_map_lookup_elem(
            &mut task_kptr_lock_map,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut task_kptr_lock_value
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_local_irq_save(&mut flags);
        task = (*v).task;
        bpf_rcu_read_lock();
        bpf_local_irq_restore(&mut flags);
    }
    if !task.is_null() {
        acquired = unsafe { bpf_task_acquire(task) };
        if !acquired.is_null() {
            unsafe { bpf_task_release(acquired) };
        }
    }
    unsafe { bpf_rcu_read_unlock() };
    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_from_vpid_current(ctx: *const core::ffi::c_void) -> i32 {
    let current: *mut task_struct;
    let v_task: *mut task_struct;
    let _ = ctx;

    v_task = unsafe { bpf_task_from_vpid(1) };
    if v_task.is_null() {
        unsafe {
            err = 1;
        }
        return 0;
    }

    current = unsafe { bpf_get_current_task_btf() };

    /* The current process should be the init process (pid 1) in the new pid namespace. */
    if current != v_task {
        unsafe {
            err = 2;
        }
    }

    unsafe { bpf_task_release(v_task) };
    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_task_from_vpid_invalid(ctx: *const core::ffi::c_void) -> i32 {
    let mut v_task: *mut task_struct;
    let _ = ctx;

    v_task = unsafe { bpf_task_from_vpid(-1) };
    if !v_task.is_null() {
        unsafe {
            err = 1;
        }
        unsafe { bpf_task_release(v_task) };
        return 0;
    }

    /* There should be only one process (current process) in the new pid namespace. */
    v_task = unsafe { bpf_task_from_vpid(2) };
    if !v_task.is_null() {
        unsafe {
            err = 2;
        }
        unsafe { bpf_task_release(v_task) };
        return 0;
    }

    v_task = unsafe { bpf_task_from_vpid(9999) };
    if !v_task.is_null() {
        unsafe {
            err = 3;
        }
        unsafe { bpf_task_release(v_task) };
        return 0;
    }

    0
}
