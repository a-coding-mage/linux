// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2012
 * Author(s): Jan Glauber <jang@linux.vnet.ibm.com>
 */

// Linux and s390 header dependencies are supplied by the surrounding tree.

#[repr(C)]
pub struct runtime_instr_cb {
    pub rla: u32,
    pub s: u8,
    pub k: u8,
    pub ps: u8,
    pub pc: u8,
    pub key: u8,
    pub v: u8,
}

#[repr(C)]
pub struct psw_t {
    pub mask: usize,
}

#[repr(C)]
pub struct pt_regs {
    pub psw: psw_t,
}

#[repr(C)]
pub struct thread_struct {
    pub ri_cb: *mut runtime_instr_cb,
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

extern "C" {
    pub static mut current: *mut task_struct;

    fn kfree(ptr: *mut core::ffi::c_void);
    fn kzalloc_obj<T>() -> *mut T;
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize) -> *mut core::ffi::c_void;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn preempt_disable();
    fn preempt_enable();
    fn load_runtime_instr_cb(cb: *mut runtime_instr_cb);
    fn test_facility(facility: i32) -> bool;
}

/* empty control block to disable RI by loading it */
#[no_mangle]
pub static mut runtime_instr_empty_cb: runtime_instr_cb = runtime_instr_cb {
    rla: 0,
    s: 0,
    k: 0,
    ps: 0,
    pc: 0,
    key: 0,
    v: 0,
};

pub const S390_RUNTIME_INSTR_STOP: i32 = 0;
pub const S390_RUNTIME_INSTR_START: i32 = 1;
pub const EOPNOTSUPP: i32 = 95;
pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const PAGE_DEFAULT_KEY: u32 = 0;
pub const PSW_MASK_RI: usize = 1usize << 22;

pub unsafe fn runtime_instr_release(tsk: *mut task_struct) {
    kfree((*tsk).thread.ri_cb.cast());
}

unsafe fn disable_runtime_instr() {
    let task: *mut task_struct = current;
    let regs: *mut pt_regs;

    if (*task).thread.ri_cb.is_null() {
        return;
    }
    regs = task_pt_regs(task);
    preempt_disable();
    load_runtime_instr_cb(core::ptr::addr_of_mut!(runtime_instr_empty_cb));
    kfree((*task).thread.ri_cb.cast());
    (*task).thread.ri_cb = core::ptr::null_mut();
    preempt_enable();

    /* Make sure the RI bit is deleted from the PSW. */
    (*regs).psw.mask &= !PSW_MASK_RI;
}

unsafe fn init_runtime_instr_cb(cb: *mut runtime_instr_cb) {
    (*cb).rla = 0xfff;
    (*cb).s = 1;
    (*cb).k = 1;
    (*cb).ps = 1;
    (*cb).pc = 1;
    (*cb).key = (PAGE_DEFAULT_KEY >> 4) as u8;
    (*cb).v = 1;
}

/*
 * The signum argument is unused. In older kernels it was used to
 * specify a real-time signal. For backwards compatibility user space
 * should pass a valid real-time signal number.
 */
#[no_mangle]
pub unsafe extern "C" fn s390_runtime_instr(command: i32, _signum: i32) -> i32 {
    let cb: *mut runtime_instr_cb;

    if !test_facility(64) {
        return -EOPNOTSUPP;
    }

    if command == S390_RUNTIME_INSTR_STOP {
        disable_runtime_instr();
        return 0;
    }

    if command != S390_RUNTIME_INSTR_START {
        return -EINVAL;
    }

    if (*current).thread.ri_cb.is_null() {
        cb = kzalloc_obj::<runtime_instr_cb>();
        if cb.is_null() {
            return -ENOMEM;
        }
    } else {
        cb = (*current).thread.ri_cb;
        memset(cb.cast(), 0, core::mem::size_of::<runtime_instr_cb>());
    }

    init_runtime_instr_cb(cb);

    preempt_disable();
    (*current).thread.ri_cb = cb;
    load_runtime_instr_cb(cb);
    preempt_enable();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
