/* SPDX-License-Identifier: GPL-2.0 */

/* Corresponds to CONFIG_UNWIND_USER. */

/// Return the word size used by a user stack, or zero for VM86 stacks.
#[inline]
pub unsafe fn unwind_user_word_size(regs: *mut pt_regs) -> i32 {
    // We can't unwind VM86 stacks.
    if ((*regs).flags & X86_VM_MASK) != 0 {
        return 0;
    }
    if user_64bit_mode(regs) {
        8
    } else {
        4
    }
}

/* Corresponds to CONFIG_HAVE_UNWIND_USER_FP. */

/// Initialize a user frame-pointer frame.
#[macro_export]
macro_rules! ARCH_INIT_USER_FP_FRAME {
    ($ws:expr) => {
        .cfa_off = 2 * ($ws),
        .ra_off = -1 * ($ws),
        .fp_off = -2 * ($ws),
        .use_fp = true,
    };
}

/// Initialize a user entry frame-pointer frame.
#[macro_export]
macro_rules! ARCH_INIT_USER_FP_ENTRY_FRAME {
    ($ws:expr) => {
        .cfa_off = 1 * ($ws),
        .ra_off = -1 * ($ws),
        .fp_off = 0,
        .use_fp = false,
    };
}

#[inline]
pub unsafe fn unwind_user_at_function_start(regs: *mut pt_regs) -> bool {
    is_uprobe_at_func_entry(regs)
}

/* C compatibility alias: unwind_user_at_function_start */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
