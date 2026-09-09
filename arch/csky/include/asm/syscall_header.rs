/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    pub static mut sys_call_table: [*mut c_void; 0];
}

#[inline]
pub unsafe fn syscall_get_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> core::ffi::c_int {
    *regs_syscallid(regs)
}

#[inline]
pub unsafe fn syscall_set_nr(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    sysno: core::ffi::c_int,
) {
    *regs_syscallid(regs) = sysno;
}

#[inline]
pub unsafe fn syscall_rollback(_task: *mut task_struct, regs: *mut pt_regs) {
    (*regs).a0 = (*regs).orig_a0;
}

#[inline]
pub unsafe fn syscall_get_error(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> core::ffi::c_long {
    let error: c_ulong = (*regs).a0 as c_ulong;

    if is_err_value(error) {
        error as core::ffi::c_long
    } else {
        0
    }
}

#[inline]
pub unsafe fn syscall_get_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
) -> core::ffi::c_long {
    (*regs).a0
}

#[inline]
pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: core::ffi::c_int,
    val: core::ffi::c_long,
) {
    (*regs).a0 = if error != 0 { error as core::ffi::c_long } else { val };
}

#[inline]
pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut c_ulong,
) {
    *args = (*regs).orig_a0;
    let args = args.add(1);
    memcpy(
        args as *mut c_void,
        &(*regs).a1 as *const _ as *const c_void,
        5 * core::mem::size_of::<c_ulong>(),
    );
}

#[inline]
pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const c_ulong,
) {
    memcpy(
        &mut (*regs).a0 as *mut _ as *mut c_void,
        args as *const c_void,
        6 * core::mem::size_of_val(&(*regs).a0),
    );
    /*
     * Also copy the first argument into orig_a0
     * so that syscall_get_arguments() would return it
     * instead of the previous value.
     */
    (*regs).orig_a0 = (*regs).a0;
}

#[inline]
pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> core::ffi::c_int {
    AUDIT_ARCH_CSKY
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
