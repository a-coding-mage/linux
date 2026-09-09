// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the corresponding Linux and PowerPC headers are
// intentionally referenced here rather than reimplemented.

/* Has to run notrace because it is entered not completely "reconciled" */
pub unsafe fn system_call_exception(regs: *mut pt_regs, mut r0: c_ulong) -> c_long {
    let ret: c_long;
    let f: syscall_fn;

    if unlikely(!syscall_enter_from_user_mode_randomize_stack(regs, &mut r0)) {
        return syscall_get_error(current, regs);
    }

    if unlikely(test_and_clear_thread_flag(TIF_SYSCALL_RET)) {
        return syscall_get_error(current, regs);
    }

    if unlikely(r0 >= NR_syscalls) {
        if unlikely(trap_is_unsupported_scv(regs)) {
            /* Unsupported scv vector */
            _exception(SIGILL, regs, ILL_ILLOPC, (*regs).nip);
            return (*regs).gpr[3];
        }
        return -ENOSYS;
    }

    /* May be faster to do array_index_nospec? */
    barrier_nospec();

    // No COMPAT if we have SYSCALL_WRAPPER, see Kconfig.
    #[cfg(CONFIG_ARCH_HAS_SYSCALL_WRAPPER)]
    {
        f = sys_call_table[r0 as usize] as syscall_fn;
        ret = f(regs);
    }

    #[cfg(not(CONFIG_ARCH_HAS_SYSCALL_WRAPPER))]
    {
        if unlikely(is_compat_task()) {
            let r3: c_ulong;
            let r4: c_ulong;
            let r5: c_ulong;
            let r6: c_ulong;
            let r7: c_ulong;
            let r8: c_ulong;

            f = compat_sys_call_table[r0 as usize] as syscall_fn;

            r3 = (*regs).gpr[3] & 0x00000000ffffffffu64;
            r4 = (*regs).gpr[4] & 0x00000000ffffffffu64;
            r5 = (*regs).gpr[5] & 0x00000000ffffffffu64;
            r6 = (*regs).gpr[6] & 0x00000000ffffffffu64;
            r7 = (*regs).gpr[7] & 0x00000000ffffffffu64;
            r8 = (*regs).gpr[8] & 0x00000000ffffffffu64;

            ret = f(r3, r4, r5, r6, r7, r8);
        } else {
            f = sys_call_table[r0 as usize] as syscall_fn;

            ret = f(
                (*regs).gpr[3],
                (*regs).gpr[4],
                (*regs).gpr[5],
                (*regs).gpr[6],
                (*regs).gpr[7],
                (*regs).gpr[8],
            );
        }
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
