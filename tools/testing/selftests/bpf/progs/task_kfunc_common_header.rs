/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* C header guard removed: _TASK_KFUNC_COMMON_H. */
/* Dependencies from C includes: errno.h, vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h. */

#[repr(C)]
pub struct __tasks_kfunc_map_value {
    pub task: *mut task_struct,
}

/*
 * Original BPF map declaration:
 * __uint(type, BPF_MAP_TYPE_HASH);
 * __type(key, int);
 * __type(value, struct __tasks_kfunc_map_value);
 * __uint(max_entries, 1);
 * SEC(".maps")
 */
#[repr(C)]
pub struct __tasks_kfunc_map__def {
    _unused: [u8; 0],
}

#[link_section = ".maps"]
pub static mut __tasks_kfunc_map: __tasks_kfunc_map__def = __tasks_kfunc_map__def { _unused: [] };

#[repr(C)]
pub struct task_kptr_lock_value {
    pub lock: bpf_spin_lock,
    pub task: *mut task_struct,
}

/*
 * Original BPF map declaration:
 * __uint(type, BPF_MAP_TYPE_ARRAY);
 * __type(key, int);
 * __type(value, struct task_kptr_lock_value);
 * __uint(max_entries, 1);
 * SEC(".maps")
 */
#[repr(C)]
pub struct task_kptr_lock_map__def {
    _unused: [u8; 0],
}

#[link_section = ".maps"]
pub static mut task_kptr_lock_map: task_kptr_lock_map__def = task_kptr_lock_map__def { _unused: [] };

unsafe extern "C" {
    #[link_name = "bpf_task_acquire"]
    pub fn bpf_task_acquire(p: *mut task_struct) -> *mut task_struct;
    #[link_name = "bpf_task_release"]
    pub fn bpf_task_release(p: *mut task_struct);
    #[link_name = "bpf_task_from_pid"]
    pub fn bpf_task_from_pid(pid: s32) -> *mut task_struct;
    #[link_name = "bpf_task_from_vpid"]
    pub fn bpf_task_from_vpid(vpid: s32) -> *mut task_struct;
    #[link_name = "bpf_rcu_read_lock"]
    pub fn bpf_rcu_read_lock();
    #[link_name = "bpf_rcu_read_unlock"]
    pub fn bpf_rcu_read_unlock();
    /* Original declaration was weak ksym. */
    #[link_name = "bpf_local_irq_save"]
    pub fn bpf_local_irq_save(flags: *mut core::ffi::c_ulong);
    /* Original declaration was weak ksym. */
    #[link_name = "bpf_local_irq_restore"]
    pub fn bpf_local_irq_restore(flags: *mut core::ffi::c_ulong);
}

pub unsafe fn tasks_kfunc_map_value_lookup(
    p: *mut task_struct,
) -> *mut __tasks_kfunc_map_value {
    let mut pid: s32 = 0;
    let mut status: core::ffi::c_long;

    status = unsafe {
        bpf_probe_read_kernel(
            &mut pid as *mut s32 as *mut core::ffi::c_void,
            core::mem::size_of_val(&pid) as u64,
            &unsafe { (*p).pid } as *const _ as *const core::ffi::c_void,
        )
    };
    if status != 0 {
        return core::ptr::null_mut();
    }

    unsafe {
        bpf_map_lookup_elem(
            &raw mut __tasks_kfunc_map as *mut _ as *mut core::ffi::c_void,
            &mut pid as *mut s32 as *mut core::ffi::c_void,
        ) as *mut __tasks_kfunc_map_value
    }
}

pub unsafe fn tasks_kfunc_map_insert(p: *mut task_struct) -> core::ffi::c_int {
    let mut local: __tasks_kfunc_map_value = __tasks_kfunc_map_value {
        task: core::ptr::null_mut(),
    };
    let mut v: *mut __tasks_kfunc_map_value;
    let mut status: core::ffi::c_long;
    let mut acquired: *mut task_struct;
    let mut old: *mut task_struct;
    let mut pid: s32 = 0;

    status = unsafe {
        bpf_probe_read_kernel(
            &mut pid as *mut s32 as *mut core::ffi::c_void,
            core::mem::size_of_val(&pid) as u64,
            &unsafe { (*p).pid } as *const _ as *const core::ffi::c_void,
        )
    };
    if status != 0 {
        return status as core::ffi::c_int;
    }

    local.task = core::ptr::null_mut();
    status = unsafe {
        bpf_map_update_elem(
            &raw mut __tasks_kfunc_map as *mut _ as *mut core::ffi::c_void,
            &mut pid as *mut s32 as *mut core::ffi::c_void,
            &mut local as *mut __tasks_kfunc_map_value as *mut core::ffi::c_void,
            BPF_NOEXIST as u64,
        )
    };
    if status != 0 {
        return status as core::ffi::c_int;
    }

    v = unsafe {
        bpf_map_lookup_elem(
            &raw mut __tasks_kfunc_map as *mut _ as *mut core::ffi::c_void,
            &mut pid as *mut s32 as *mut core::ffi::c_void,
        ) as *mut __tasks_kfunc_map_value
    };
    if v.is_null() {
        unsafe {
            bpf_map_delete_elem(
                &raw mut __tasks_kfunc_map as *mut _ as *mut core::ffi::c_void,
                &mut pid as *mut s32 as *mut core::ffi::c_void,
            );
        }
        return -ENOENT;
    }

    acquired = unsafe { bpf_task_acquire(p) };
    if acquired.is_null() {
        return -ENOENT;
    }

    old = unsafe { bpf_kptr_xchg(&mut (*v).task as *mut *mut task_struct, acquired) };
    if !old.is_null() {
        unsafe {
            bpf_task_release(old);
        }
        return -EEXIST;
    }

    0
}
