// SPDX-License-Identifier: GPL-2.0-or-later

// Dependency intent: declarations supplied by linux/regset.h, asm/switch_to.h,
// and ptrace-decl.h remain external to this translation unit.

/*
 * Regardless of transactions, 'fp_state' holds the current running
 * value of all FPR registers and 'ckfp_state' holds the last checkpointed
 * value of all FPR registers for the current transaction.
 *
 * Userspace interface buffer layout:
 *
 * struct data {
 *     u64 fpr[32];
 *     u64 fpscr;
 * };
 */
pub unsafe fn fpr_get(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut to: membuf,
) -> i32 {
    #[cfg(CONFIG_PPC_FPU_REGS)]
    {
        // BUILD_BUG_ON(offsetof(struct thread_fp_state, fpscr) !=
        //              offsetof(struct thread_fp_state, fpr[32]));
        flush_fp_to_thread(target);

        return membuf_write(
            &mut to,
            &(*target).thread.fp_state as *const _ as *const core::ffi::c_void,
            33 * core::mem::size_of::<u64>(),
        );
    }

    #[cfg(not(CONFIG_PPC_FPU_REGS))]
    {
        membuf_write(
            &mut to,
            &empty_zero_page as *const _ as *const core::ffi::c_void,
            33 * core::mem::size_of::<u64>(),
        )
    }
}

/*
 * Regardless of transactions, 'fp_state' holds the current running
 * value of all FPR registers and 'ckfp_state' holds the last checkpointed
 * value of all FPR registers for the current transaction.
 *
 * Userspace interface buffer layout:
 *
 * struct data {
 *     u64 fpr[32];
 *     u64 fpscr;
 * };
 *
 */
pub unsafe fn fpr_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut pos: u32,
    mut count: u32,
    mut kbuf: *const core::ffi::c_void,
    mut ubuf: *const core::ffi::c_void,
) -> i32 {
    #[cfg(CONFIG_PPC_FPU_REGS)]
    {
        // BUILD_BUG_ON(offsetof(struct thread_fp_state, fpscr) !=
        //              offsetof(struct thread_fp_state, fpr[32]));
        flush_fp_to_thread(target);

        return user_regset_copyin(
            &mut pos,
            &mut count,
            &mut kbuf,
            &mut ubuf,
            &mut (*target).thread.fp_state as *mut _ as *mut core::ffi::c_void,
            0,
            usize::MAX,
        );
    }

    #[cfg(not(CONFIG_PPC_FPU_REGS))]
    {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
