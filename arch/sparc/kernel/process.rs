// SPDX-License-Identifier: GPL-2.0

/*
 * This file handles the architecture independent parts of process handling..
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn sparc_fork(regs: *mut pt_regs) -> c_long {
    let mut orig_i1: c_ulong;
    let ret: c_long;
    let mut args = kernel_clone_args {
        exit_signal: SIGCHLD,
        ..core::mem::zeroed()
    };

    synchronize_user_stack();

    orig_i1 = (*regs).u_regs[UREG_I1];
    /* Reuse the parent's stack for the child. */
    args.stack = (*regs).u_regs[UREG_FP];

    ret = kernel_clone(&mut args);

    /* If we get an error and potentially restart the system
     * call, we're screwed because copy_thread() clobbered
     * the parent's %o1.  So detect that case and restore it
     * here.
     */
    if (ret as c_ulong) >= (-ERESTART_RESTARTBLOCK as c_ulong) {
        (*regs).u_regs[UREG_I1] = orig_i1;
    }

    ret
}

pub unsafe fn sparc_vfork(regs: *mut pt_regs) -> c_long {
    let mut orig_i1: c_ulong;
    let ret: c_long;
    let mut args = kernel_clone_args {
        flags: CLONE_VFORK | CLONE_VM,
        exit_signal: SIGCHLD,
        ..core::mem::zeroed()
    };

    synchronize_user_stack();

    orig_i1 = (*regs).u_regs[UREG_I1];
    /* Reuse the parent's stack for the child. */
    args.stack = (*regs).u_regs[UREG_FP];

    ret = kernel_clone(&mut args);

    /* If we get an error and potentially restart the system
     * call, we're screwed because copy_thread() clobbered
     * the parent's %o1.  So detect that case and restore it
     * here.
     */
    if (ret as c_ulong) >= (-ERESTART_RESTARTBLOCK as c_ulong) {
        (*regs).u_regs[UREG_I1] = orig_i1;
    }

    ret
}

pub unsafe fn sparc_clone(regs: *mut pt_regs) -> c_long {
    let mut orig_i1: c_ulong;
    let flags: c_uint;
    let ret: c_long;
    let mut args: kernel_clone_args = core::mem::zeroed();

    synchronize_user_stack();

    orig_i1 = (*regs).u_regs[UREG_I1];
    flags = lower_32_bits((*regs).u_regs[UREG_I0]);
    args.flags = flags & !CSIGNAL;
    args.exit_signal = flags & CSIGNAL;
    args.tls = (*regs).u_regs[UREG_I3];

    // CONFIG_COMPAT conditional preserved from the source.
    #[cfg(CONFIG_COMPAT)]
    if test_thread_flag(TIF_32BIT) {
        args.pidfd = compat_ptr((*regs).u_regs[UREG_I2]);
        args.child_tid = compat_ptr((*regs).u_regs[UREG_I4]);
        args.parent_tid = compat_ptr((*regs).u_regs[UREG_I2]);
    } else {
        args.pidfd = (*regs).u_regs[UREG_I2] as *mut c_int;
        args.child_tid = (*regs).u_regs[UREG_I4] as *mut c_int;
        args.parent_tid = (*regs).u_regs[UREG_I2] as *mut c_int;
    }
    #[cfg(not(CONFIG_COMPAT))]
    {
        args.pidfd = (*regs).u_regs[UREG_I2] as *mut c_int;
        args.child_tid = (*regs).u_regs[UREG_I4] as *mut c_int;
        args.parent_tid = (*regs).u_regs[UREG_I2] as *mut c_int;
    }

    /* Did userspace give setup a separate stack for the child or are we
     * reusing the parent's?
     */
    if (*regs).u_regs[UREG_I1] != 0 {
        args.stack = (*regs).u_regs[UREG_I1];
    } else {
        args.stack = (*regs).u_regs[UREG_FP];
    }

    ret = kernel_clone(&mut args);

    /* If we get an error and potentially restart the system
     * call, we're screwed because copy_thread() clobbered
     * the parent's %o1.  So detect that case and restore it
     * here.
     */
    if (ret as c_ulong) >= (-ERESTART_RESTARTBLOCK as c_ulong) {
        (*regs).u_regs[UREG_I1] = orig_i1;
    }

    ret
}

pub unsafe fn sparc_clone3(regs: *mut pt_regs) -> c_long {
    let sz: c_ulong;
    let cl_args: *mut clone_args;

    synchronize_user_stack();

    cl_args = (*regs).u_regs[UREG_I0] as *mut clone_args;
    sz = (*regs).u_regs[UREG_I1];

    sys_clone3(cl_args, sz)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
