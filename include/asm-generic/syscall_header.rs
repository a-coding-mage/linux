/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Access to user system call parameters and results
 *
 * Copyright (C) 2008-2009 Red Hat, Inc.  All rights reserved.
 *
 * This file is a stub providing documentation for what functions
 * arch/ARCH/include/asm/syscall.h files need to define.  Most arch definitions
 * will be simple inlines.
 *
 * All of these functions expect to be called with no locks,
 * and only when the caller is sure that the task of interest
 * cannot return to user mode while we are looking at it.
 */

// C header guard: _ASM_SYSCALL_H

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/**
 * syscall_get_nr - find what system call a task is executing
 * @task: task of interest, must be blocked
 * @regs: task_pt_regs() of @task
 *
 * If @task is executing a system call or is at system call
 * tracing about to attempt one, returns the system call number.
 * If @task is not executing a system call, i.e. it's blocked
 * inside the kernel for a fault or signal, returns -1.
 *
 * Note this returns int even on 64-bit machines.  Only 32 bits of
 * system call number can be meaningful.  If the actual arch value
 * is 64 bits, this truncates to 32 bits so 0xffffffff means -1.
 *
 * It's only valid to call this when @task is known to be blocked.
 */
extern "C" {
    pub fn syscall_get_nr(task: *mut task_struct, regs: *mut pt_regs) -> i32;

    /**
     * syscall_set_nr - change the system call a task is executing
     * @task: task of interest, must be blocked
     * @regs: task_pt_regs() of @task
     * @nr: system call number
     *
     * Changes the system call number @task is about to execute.
     *
     * It's only valid to call this when @task is stopped for tracing on
     * entry to a system call, due to %SYSCALL_WORK_SYSCALL_TRACE or
     * %SYSCALL_WORK_SYSCALL_AUDIT.
     */
    pub fn syscall_set_nr(task: *mut task_struct, regs: *mut pt_regs, nr: i32);

    /**
     * syscall_rollback - roll back registers after an aborted system call
     * @task: task of interest, must be in system call exit tracing
     * @regs: task_pt_regs() of @task
     *
     * It's only valid to call this when @task is stopped for system
     * call exit tracing (due to %SYSCALL_WORK_SYSCALL_TRACE or
     * %SYSCALL_WORK_SYSCALL_AUDIT), after ptrace_report_syscall_permit_entry()
     * returned False to prevent the system call from taking place.
     *
     * This rolls back the register state in @regs so it's as if the
     * system call instruction was a no-op.  The registers containing
     * the system call number and arguments are as they were before the
     * system call instruction.  This may not be the same as what the
     * register state looked like at system call entry tracing.
     */
    pub fn syscall_rollback(task: *mut task_struct, regs: *mut pt_regs);

    /**
     * syscall_get_error - check result of traced system call
     * @task: task of interest, must be blocked
     * @regs: task_pt_regs() of @task
     *
     * Returns 0 if the system call succeeded, or -ERRORCODE if it failed.
     *
     * It's only valid to call this when @task is stopped for tracing on exit
     * from a system call, due to %SYSCALL_WORK_SYSCALL_TRACE or
     * %SYSCALL_WORK_SYSCALL_AUDIT.
     */
    pub fn syscall_get_error(task: *mut task_struct, regs: *mut pt_regs) -> core::ffi::c_long;

    /**
     * syscall_get_return_value - get the return value of a traced system call
     * @task: task of interest, must be blocked
     * @regs: task_pt_regs() of @task
     *
     * Returns the return value of the successful system call.
     * This value is meaningless if syscall_get_error() returned nonzero.
     *
     * It's only valid to call this when @task is stopped for tracing on exit
     * from a system call, due to %SYSCALL_WORK_SYSCALL_TRACE or
     * %SYSCALL_WORK_SYSCALL_AUDIT.
     */
    pub fn syscall_get_return_value(task: *mut task_struct, regs: *mut pt_regs) -> core::ffi::c_long;

    /**
     * syscall_set_return_value - change the return value of a traced system call
     * @task: task of interest, must be blocked
     * @regs: task_pt_regs() of @task
     * @error: negative error code, or zero to indicate success
     * @val: user return value if @error is zero
     *
     * This changes the results of the system call that user mode will see.
     * If @error is zero, the user sees a successful system call with a
     * return value of @val.  If @error is nonzero, it's a negated errno
     * code; the user sees a failed system call with this errno code.
     *
     * It's only valid to call this when @task is stopped for tracing on exit
     * from a system call, due to %SYSCALL_WORK_SYSCALL_TRACE or
     * %SYSCALL_WORK_SYSCALL_AUDIT.
     */
    pub fn syscall_set_return_value(
        task: *mut task_struct,
        regs: *mut pt_regs,
        error: i32,
        val: core::ffi::c_long,
    );

    /**
     * syscall_get_arguments - extract system call parameter values
     * @task: task of interest, must be blocked
     * @regs: task_pt_regs() of @task
     * @args: array filled with argument values
     *
     * Fetches 6 arguments to the system call.  First argument is stored in
     * @args[0], and so on.
     *
     * It's only valid to call this when @task is stopped for tracing on
     * entry to a system call, due to %SYSCALL_WORK_SYSCALL_TRACE or
     * %SYSCALL_WORK_SYSCALL_AUDIT.
     */
    pub fn syscall_get_arguments(
        task: *mut task_struct,
        regs: *mut pt_regs,
        args: *mut core::ffi::c_ulong,
    );

    /**
     * syscall_set_arguments - change system call parameter value
     * @task: task of interest, must be in system call entry tracing
     * @regs: task_pt_regs() of @task
     * @args: array of argument values to store
     *
     * Changes 6 arguments to the system call.
     * The first argument gets value @args[0], and so on.
     *
     * It's only valid to call this when @task is stopped for tracing on
     * entry to a system call, due to %SYSCALL_WORK_SYSCALL_TRACE or
     * %SYSCALL_WORK_SYSCALL_AUDIT.
     */
    pub fn syscall_set_arguments(
        task: *mut task_struct,
        regs: *mut pt_regs,
        args: *const core::ffi::c_ulong,
    );

    /**
     * syscall_get_arch - return the AUDIT_ARCH for the current system call
     * @task: task of interest, must be blocked
     *
     * Returns the AUDIT_ARCH_* based on the system call convention in use.
     *
     * It's only valid to call this when @task is stopped on entry to a system
     * call, due to %SYSCALL_WORK_SYSCALL_TRACE, %SYSCALL_WORK_SYSCALL_AUDIT, or
     * %SYSCALL_WORK_SECCOMP.
     *
     * Architectures which permit CONFIG_HAVE_ARCH_SECCOMP_FILTER must
     * provide an implementation of this.
     */
    pub fn syscall_get_arch(task: *mut task_struct) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
