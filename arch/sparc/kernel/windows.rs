// SPDX-License-Identifier: GPL-2.0
/* windows.c: Routines to deal with register window management
 *            at the C-code level.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// The declarations and constants below are supplied by the surrounding kernel.

/* Do save's until all user register windows are out of the cpu. */
pub unsafe fn flush_user_windows() {
    let mut ctr: i32 = 0;

    core::arch::asm!(
        "1:",
        "ld [%%g6 + {uwinmask}], %%g4",
        "orcc %%g0, %%g4, %%g0",
        "add {ctr}, 1, {ctr}",
        "bne 1b",
        " save %%sp, -64, %%sp",
        "2:",
        "subcc {ctr}, 1, {ctr}",
        "bne 2b",
        " restore %%g0, %%g0, %%g0",
        ctr = inout(reg) ctr,
        uwinmask = const TI_UWINMASK,
        out("g4") _,
        options(nostack)
    );
}

unsafe fn shift_window_buffer(first_win: i32, last_win: i32, tp: *mut thread_info) {
    let mut i = first_win;
    while i < last_win {
        (*tp).rwbuf_stkptrs[i as usize] = (*tp).rwbuf_stkptrs[(i + 1) as usize];
        core::ptr::copy_nonoverlapping(
            &(*tp).reg_window[(i + 1) as usize] as *const reg_window32,
            &mut (*tp).reg_window[i as usize] as *mut reg_window32,
            1,
        );
        i += 1;
    }
}

/* Place as many of the user's current register windows
 * on the stack that we can.  Even if the %sp is unaligned
 * we still copy the window there, the only case that we don't
 * succeed is if the %sp points to a bum mapping altogether.
 * setup_frame() and do_sigreturn() use this before shifting
 * the user stack around.  Future instruction and hardware
 * bug workaround routines will need this functionality as
 * well.
 */
pub unsafe fn synchronize_user_stack() {
    let tp = current_thread_info();
    let mut window: i32;

    flush_user_windows();
    if (*tp).w_saved == 0 {
        return;
    }

    /* Ok, there is some dirty work to do. */
    window = (*tp).w_saved - 1;
    while window >= 0 {
        let sp: usize = (*tp).rwbuf_stkptrs[window as usize];

        /* Ok, let it rip. */
        if copy_to_user(
            sp as *mut core::ffi::c_void,
            &(*tp).reg_window[window as usize] as *const reg_window32,
            core::mem::size_of::<reg_window32>(),
        ) != 0 {
            window -= 1;
            continue;
        }

        shift_window_buffer(window, (*tp).w_saved - 1, tp);
        (*tp).w_saved -= 1;
        window -= 1;
    }
}

/* An optimization. */
#[cfg(any())]
unsafe fn copy_aligned_window(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    core::arch::asm!(
        "ldd [{src}], %%g2\n\t",
        "ldd [{src} + 0x8], %%g4\n\t",
        "std %%g2, [{dest}]\n\t",
        "std %%g4, [{dest} + 0x8]\n\t",
        "ldd [{src} + 0x10], %%g2\n\t",
        "ldd [{src} + 0x18], %%g4\n\t",
        "std %%g2, [{dest} + 0x10]\n\t",
        "std %%g4, [{dest} + 0x18]\n\t",
        "ldd [{src} + 0x20], %%g2\n\t",
        "ldd [{src} + 0x28], %%g4\n\t",
        "std %%g2, [{dest} + 0x20]\n\t",
        "std %%g4, [{dest} + 0x28]\n\t",
        "ldd [{src} + 0x30], %%g2\n\t",
        "ldd [{src} + 0x38], %%g4\n\t",
        "std %%g2, [{dest} + 0x30]\n\t",
        "std %%g4, [{dest} + 0x38]\n\t",
        dest = in(reg) dest,
        src = in(reg) src,
        out("g2") _,
        out("g4") _,
    );
}

/* Try to push the windows in a threads window buffer to the
 * user stack.  Unaligned %sp's are not allowed here.
 */
pub unsafe fn try_to_clear_window_buffer(regs: *mut pt_regs, who: i32) {
    let tp = current_thread_info();
    let mut window: i32 = 0;

    flush_user_windows();
    while window < (*tp).w_saved {
        let sp: usize = (*tp).rwbuf_stkptrs[window as usize];

        if (sp & 7) != 0
            || copy_to_user(
                sp as *mut core::ffi::c_void,
                &(*tp).reg_window[window as usize] as *const reg_window32,
                core::mem::size_of::<reg_window32>(),
            ) != 0
        {
            force_exit_sig(SIGILL);
            return;
        }
        window += 1;
    }
    (*tp).w_saved = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
