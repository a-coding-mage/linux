// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2016
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Linux kernel and s390 dependencies supplied by other translation units.

use core::ffi::c_int;

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub gs_cb: *mut gs_cb,
    pub gs_bc_cb: *mut gs_cb,
}

#[repr(C)]
pub struct gs_cb {
    pub gsd: u64,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut tasklist_lock: read_write_lock;

    fn kfree(ptr: *mut core::ffi::c_void);
    fn kzalloc(size: usize, flags: usize) -> *mut core::ffi::c_void;
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn preempt_disable();
    fn preempt_enable();
    fn local_ctl_set_bit(reg: c_int, bit: c_int);
    fn local_ctl_clear_bit(reg: c_int, bit: c_int);
    fn load_gs_cb(gs_cb: *mut gs_cb);
    fn clear_thread_flag(flag: c_int);
    fn test_and_set_tsk_thread_flag(tsk: *mut task_struct, flag: c_int) -> c_int;
    fn kick_process(tsk: *mut task_struct);
    fn read_lock(lock: *mut read_write_lock);
    fn read_unlock(lock: *mut read_write_lock);
    fn cpu_has_gs() -> bool;
    fn for_each_thread(current: *mut task_struct, sibling: *mut *mut task_struct);
}

#[repr(C)]
pub struct read_write_lock {
    _private: [u8; 0],
}

const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const CR2_GUARDED_STORAGE_BIT: c_int = 0;
const TIF_GUARDED_STORAGE: c_int = 0;
const GS_ENABLE: c_int = 0;
const GS_DISABLE: c_int = 1;
const GS_SET_BC_CB: c_int = 2;
const GS_CLEAR_BC_CB: c_int = 3;
const GS_BROADCAST: c_int = 4;

pub unsafe fn guarded_storage_release(tsk: *mut task_struct) {
    kfree((*tsk).thread.gs_cb.cast());
    kfree((*tsk).thread.gs_bc_cb.cast());
}

unsafe fn gs_enable() -> c_int {
    let mut gs_cb: *mut gs_cb;

    if (*current).thread.gs_cb.is_null() {
        gs_cb = kzalloc(core::mem::size_of::<gs_cb>(), 0).cast();
        if gs_cb.is_null() {
            return -ENOMEM;
        }
        (*gs_cb).gsd = 25;
        preempt_disable();
        local_ctl_set_bit(2, CR2_GUARDED_STORAGE_BIT);
        load_gs_cb(gs_cb);
        (*current).thread.gs_cb = gs_cb;
        preempt_enable();
    }
    0
}

unsafe fn gs_disable() -> c_int {
    if !(*current).thread.gs_cb.is_null() {
        preempt_disable();
        kfree((*current).thread.gs_cb.cast());
        (*current).thread.gs_cb = core::ptr::null_mut();
        local_ctl_clear_bit(2, CR2_GUARDED_STORAGE_BIT);
        preempt_enable();
    }
    0
}

unsafe fn gs_set_bc_cb(u_gs_cb: *mut gs_cb) -> c_int {
    let mut gs_cb: *mut gs_cb = (*current).thread.gs_bc_cb;
    if gs_cb.is_null() {
        gs_cb = kzalloc(core::mem::size_of::<gs_cb>(), 0).cast();
        if gs_cb.is_null() {
            return -ENOMEM;
        }
        (*current).thread.gs_bc_cb = gs_cb;
    }
    if copy_from_user(
        gs_cb.cast(),
        u_gs_cb.cast(),
        core::mem::size_of::<gs_cb>(),
    ) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn gs_clear_bc_cb() -> c_int {
    let gs_cb: *mut gs_cb = (*current).thread.gs_bc_cb;
    (*current).thread.gs_bc_cb = core::ptr::null_mut();
    kfree(gs_cb.cast());
    0
}

pub unsafe fn gs_load_bc_cb(_regs: *mut pt_regs) {
    preempt_disable();
    clear_thread_flag(TIF_GUARDED_STORAGE);
    let gs_cb: *mut gs_cb = (*current).thread.gs_bc_cb;
    if !gs_cb.is_null() {
        kfree((*current).thread.gs_cb.cast());
        (*current).thread.gs_bc_cb = core::ptr::null_mut();
        local_ctl_set_bit(2, CR2_GUARDED_STORAGE_BIT);
        load_gs_cb(gs_cb);
        (*current).thread.gs_cb = gs_cb;
    }
    preempt_enable();
}

unsafe fn gs_broadcast() -> c_int {
    let mut sibling: *mut task_struct = core::ptr::null_mut();
    read_lock(&raw mut tasklist_lock);
    for_each_thread(current, &mut sibling);
    while !sibling.is_null() {
        if !(*sibling).thread.gs_bc_cb.is_null()
            && test_and_set_tsk_thread_flag(sibling, TIF_GUARDED_STORAGE) != 0
        {
            kick_process(sibling);
        }
        for_each_thread(current, &mut sibling);
    }
    read_unlock(&raw mut tasklist_lock);
    0
}

pub unsafe fn s390_guarded_storage(command: c_int, gs_cb: *mut gs_cb) -> c_int {
    if !cpu_has_gs() {
        return -EOPNOTSUPP;
    }
    match command {
        GS_ENABLE => gs_enable(),
        GS_DISABLE => gs_disable(),
        GS_SET_BC_CB => gs_set_bc_cb(gs_cb),
        GS_CLEAR_BC_CB => gs_clear_bc_cb(),
        GS_BROADCAST => gs_broadcast(),
        _ => -EINVAL,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
