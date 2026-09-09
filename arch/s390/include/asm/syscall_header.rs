/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Access to user system call parameters and results
 *
 *  Copyright IBM Corp. 2008
 *  Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 */

// Dependencies supplied by the surrounding kernel translation:
// uapi/linux/audit.h, linux/sched.h, linux/err.h, and asm/ptrace.h.

extern "C" {
    pub static sys_call_table: [sys_call_ptr_t; 0];
}

#[inline]
pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut pt_regs) -> c_long {
    if test_pt_regs_flag(regs, PIF_SYSCALL) {
        ((*regs).int_code & 0xffff) as c_long
    } else {
        -1
    }
}

#[inline]
pub unsafe fn syscall_set_nr(_task: *mut task_struct, regs: *mut pt_regs, nr: c_int) {
    /*
     * Unlike syscall_get_nr(), syscall_set_nr() can be called only when
     * the target task is stopped for tracing on entering syscall, so
     * there is no need to have the same check syscall_get_nr() has.
     */
    (*regs).int_code = ((*regs).int_code & !0xffff) | ((nr as c_ulong) & 0xffff);
}

#[inline]
pub unsafe fn syscall_rollback(_task: *mut task_struct, regs: *mut pt_regs) {
    (*regs).gprs[2] = (*regs).orig_gpr2;
}

#[inline]
pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut pt_regs) -> c_long {
    let error: c_ulong = (*regs).gprs[2];
    if IS_ERR_VALUE(error) { error as c_long } else { 0 }
}

#[inline]
pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut pt_regs) -> c_long {
    (*regs).gprs[2] as c_long
}

#[inline]
pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: c_int,
    val: c_long,
) {
    set_pt_regs_flag(regs, PIF_SYSCALL_RET_SET);
    (*regs).gprs[2] = if error != 0 { error as c_ulong } else { val as c_ulong };
}

#[inline]
pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut c_ulong,
) {
    let mask: c_ulong = !0;
    let mut i: c_int = 1;
    while i < 6 {
        *args.add(i as usize) = (*regs).gprs[(2 + i) as usize] & mask;
        i += 1;
    }
    *args = (*regs).orig_gpr2 & mask;
}

#[inline]
pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const c_ulong,
) {
    (*regs).orig_gpr2 = *args;
    let mut n: c_int = 1;
    while n < 6 {
        (*regs).gprs[(2 + n) as usize] = *args.add(n as usize);
        n += 1;
    }
}

#[inline]
pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> c_int {
    AUDIT_ARCH_S390X
}

// The following wrappers correspond to GENERATE_SYSCALL_FUNC(0..6).
#[inline(always)]
pub unsafe fn syscall0(syscall: c_ulong) -> c_long {
    let rc: c_long;
    core::arch::asm!("svc 0", in("r1") syscall, lateout("r2") rc, options(nostack));
    rc
}

macro_rules! define_syscall {
    ($name:ident, $($arg:ident : $reg:tt),+) => {
        #[inline(always)]
        pub unsafe fn $name(syscall: c_ulong, $($arg: c_long),+) -> c_long {
            let rc: c_long;
            core::arch::asm!(
                "svc 0",
                in("r1") syscall,
                $(in($reg) $arg,)+
                lateout("r2") rc,
                options(nostack)
            );
            rc
        }
    };
}

define_syscall!(syscall1, arg1: "r2");
define_syscall!(syscall2, arg1: "r2", arg2: "r3");
define_syscall!(syscall3, arg1: "r2", arg2: "r3", arg3: "r4");
define_syscall!(syscall4, arg1: "r2", arg2: "r3", arg3: "r4", arg4: "r5");
define_syscall!(syscall5, arg1: "r2", arg2: "r3", arg3: "r4", arg4: "r5", arg5: "r6");
define_syscall!(syscall6, arg1: "r2", arg2: "r3", arg3: "r4", arg4: "r5", arg5: "r6", arg6: "r7");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
