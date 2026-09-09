/*
 * Access to user system call parameters and results
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * See asm-generic/syscall.h for descriptions of what we must do here.
 *
 * Copyright (C) 2012 Ralf Baechle <ralf@linux-mips.org>
 */

// C dependencies supplied externally:
// linux/compiler.h, uapi/linux/audit.h, linux/elf-em.h, linux/kernel.h,
// linux/sched.h, linux/uaccess.h, asm/ptrace.h, and asm/unistd.h.

// __NR_syscall is only defined if _MIPS_SIM == _MIPS_SIM_ABI32.
#[cfg(not(defined(__NR_syscall)))]
pub const __NR_syscall: i32 = 4000;

#[inline]
pub unsafe fn mips_syscall_is_indirect(task: *mut task_struct, regs: *mut pt_regs) -> bool {
    /* O32 ABI syscall() - Either 64-bit with O32 or 32-bit */
    (IS_ENABLED(CONFIG_32BIT)
        || test_tsk_thread_flag(task, TIF_32BIT_REGS))
        && (*regs).regs[2] == __NR_syscall as _
}

#[inline]
pub unsafe fn syscall_get_nr(task: *mut task_struct, _regs: *mut pt_regs) -> i64 {
    (*task_thread_info(task)).syscall
}

#[inline]
pub unsafe fn syscall_set_nr(task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    /*
     * New syscall number has to be assigned to regs[2] because
     * it is loaded from there unconditionally after return from
     * syscall_trace_enter() invocation.
     *
     * Consequently, if the syscall was indirect and nr != __NR_syscall,
     * then after this assignment the syscall will cease to be indirect.
     */
    (*task_thread_info(task)).syscall = nr as _;
    (*regs).regs[2] = nr as _;
}

#[inline]
pub unsafe fn mips_syscall_update_nr(task: *mut task_struct, regs: *mut pt_regs) {
    /*
     * v0 is the system call number, except for O32 ABI syscall(), where it
     * ends up in a0.
     */
    if mips_syscall_is_indirect(task, regs) {
        (*task_thread_info(task)).syscall = (*regs).regs[4] as _;
    } else {
        (*task_thread_info(task)).syscall = (*regs).regs[2] as _;
    }
}

#[inline]
pub unsafe fn mips_get_syscall_arg(
    arg: *mut c_ulong, task: *mut task_struct, regs: *mut pt_regs, n: c_uint,
) {
    // CONFIG_32BIT: cases 0..=3 use regs[4+n], and cases 4..=7 use regs->args[n].
    #[cfg(CONFIG_32BIT)]
    {
        match n {
            0..=3 => { *arg = (*regs).regs[4 + n as usize]; return; }
            4..=7 => { *arg = (*regs).args[n as usize]; return; }
            _ => {}
        }
    }
    #[cfg(not(CONFIG_32BIT))]
    {
        *arg = (*regs).regs[4 + n as usize];
        if IS_ENABLED(CONFIG_MIPS32_O32) && test_tsk_thread_flag(task, TIF_32BIT_REGS) {
            *arg = (*arg) as c_uint as c_ulong;
        }
    }
}

#[inline]
pub unsafe fn mips_set_syscall_arg(
    arg: *mut c_ulong, _task: *mut task_struct, regs: *mut pt_regs, n: c_uint,
) {
    // CONFIG_32BIT: cases 0..=3 use regs[4+n], and cases 4..=7 use regs->args[n].
    #[cfg(CONFIG_32BIT)]
    {
        match n {
            0..=3 => { (*regs).regs[4 + n as usize] = *arg; return; }
            4..=7 => { (*regs).args[n as usize] = *arg; return; }
            _ => {}
        }
    }
    #[cfg(not(CONFIG_32BIT))]
    { (*regs).regs[4 + n as usize] = *arg; }
}

#[inline]
pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    if (*regs).regs[7] != 0 { -((*regs).regs[2] as i64) } else { 0 }
}

#[inline]
pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    (*regs).regs[2] as i64
}

#[inline]
pub unsafe fn syscall_rollback(_task: *mut task_struct, _regs: *mut pt_regs) {
    /* Do nothing */
}

#[inline]
pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct, regs: *mut pt_regs, error: i32, val: i64,
) {
    if error != 0 {
        (*regs).regs[2] = (-error) as _;
        (*regs).regs[7] = 1;
    } else {
        (*regs).regs[2] = val as _;
        (*regs).regs[7] = 0;
    }
}

#[inline]
pub unsafe fn syscall_get_arguments(task: *mut task_struct, regs: *mut pt_regs, args: *mut c_ulong) {
    let mut i: c_uint = 0;
    let mut n: c_uint = 6;

    /* O32 ABI syscall() */
    if mips_syscall_is_indirect(task, regs) { i += 1; }

    while n != 0 {
        mips_get_syscall_arg(args.add(i as usize), task, regs, i);
        i += 1;
        n -= 1;
    }
}

#[inline]
pub unsafe fn syscall_set_arguments(task: *mut task_struct, regs: *mut pt_regs, args: *mut c_ulong) {
    let mut i: c_uint = 0;
    let mut n: c_uint = 6;
    while n != 0 {
        mips_set_syscall_arg(args.add(i as usize), task, regs, i);
        i += 1;
        n -= 1;
    }
}

pub static mut sys_call_table: *const c_ulong = core::ptr::null();
pub static mut sys32_call_table: *const c_ulong = core::ptr::null();
pub static mut sysn32_call_table: *const c_ulong = core::ptr::null();

#[inline]
pub unsafe fn syscall_get_arch(task: *mut task_struct) -> i32 {
    let mut arch = AUDIT_ARCH_MIPS;
    #[cfg(CONFIG_64BIT)]
    if !test_tsk_thread_flag(task, TIF_32BIT_REGS) {
        arch |= __AUDIT_ARCH_64BIT;
        /* N32 sets only TIF_32BIT_ADDR */
        if test_tsk_thread_flag(task, TIF_32BIT_ADDR) {
            arch |= __AUDIT_ARCH_CONVENTION_MIPS64_N32;
        }
    }
    #[cfg(__LITTLE_ENDIAN)]
    { arch |= __AUDIT_ARCH_LE; }
    arch
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
