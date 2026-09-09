/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/user-return-notifier.h, asm/nospec-branch.h, asm/io_bitmap.h,
// asm/fpu/api.h, and asm/fred.h.

/* Check that the stack and regs on entry from user mode are sane. */
#[inline(always)]
pub unsafe fn arch_enter_from_user_mode(regs: *mut pt_regs) {
    if cfg!(feature = "CONFIG_DEBUG_ENTRY") {
        /*
         * Make sure that the entry code gave us a sensible EFLAGS
         * register.  Native because we want to check the actual CPU
         * state, not the interrupt state as imagined by Xen.
         */
        let flags: ::core::ffi::c_ulong = native_save_fl();
        let mut mask: ::core::ffi::c_ulong = X86_EFLAGS_DF | X86_EFLAGS_NT;

        /*
         * For !SMAP hardware we patch out CLAC on entry.
         */
        if cpu_feature_enabled(X86_FEATURE_SMAP)
            || cpu_feature_enabled(X86_FEATURE_XENPV)
        {
            mask |= X86_EFLAGS_AC;
        }

        WARN_ON_ONCE(flags & mask);

        /* We think we came from user mode. Make sure pt_regs agrees. */
        WARN_ON_ONCE(!user_mode(regs));

        /*
         * All entries from user mode (except #DF) should be on the
         * normal thread stack and should have user pt_regs in the
         * correct location.
         */
        WARN_ON_ONCE(!on_thread_stack());
        WARN_ON_ONCE(regs != task_pt_regs(current));
    }
}

#[inline]
pub unsafe fn arch_exit_work(ti_work: ::core::ffi::c_ulong) {
    if ti_work & _TIF_USER_RETURN_NOTIFY != 0 {
        fire_user_return_notifiers();
    }

    if unlikely(ti_work & _TIF_IO_BITMAP != 0) {
        tss_update_io_bitmap();
    }

    if unlikely(ti_work & _TIF_NEED_FPU_LOAD != 0) {
        switch_fpu_return();
    }
}

#[inline]
pub unsafe fn arch_exit_to_user_mode_prepare(
    regs: *mut pt_regs,
    ti_work: ::core::ffi::c_ulong,
) {
    let _ = regs;
    fpregs_assert_state_consistent();

    if unlikely(ti_work != 0) {
        arch_exit_work(ti_work);
    }

    fred_update_rsp0();

    // CONFIG_COMPAT conditional preserved from the source header.
    #[cfg(feature = "CONFIG_COMPAT")]
    {
        /*
         * Compat syscalls set TS_COMPAT.  Make sure we clear it before
         * returning to user mode.  We need to clear it *after* signal
         * handling, because syscall restart has a fixup for compat
         * syscalls.  The fixup is exercised by the ptrace_syscall_32
         * selftest.
         *
         * We also need to clear TS_REGS_POKED_I386: the 32-bit tracer
         * special case only applies after poking regs and before the
         * very next return to user mode.
         */
        (*current_thread_info()).status &= !(TS_COMPAT | TS_I386_REGS_POKED);
    }

    /* Avoid unnecessary reads of 'x86_ibpb_exit_to_user' */
    if cpu_feature_enabled(X86_FEATURE_IBPB_EXIT_TO_USER)
        && this_cpu_read(x86_ibpb_exit_to_user)
    {
        indirect_branch_prediction_barrier();
        this_cpu_write(x86_ibpb_exit_to_user, false);
    }
}

#[inline(always)]
pub unsafe fn arch_exit_to_user_mode() {
    amd_clear_divider();
}

extern "C" {
    fn x86_entry_from_kvm(entry_type: ::core::ffi::c_uint, vector: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
