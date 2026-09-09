// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from ptrace-vsx.c. Kernel declarations are supplied by the
// surrounding PowerPC kernel sources.

/*
 * Regardless of transactions, `fp_state` holds the current running value of
 * all FPR registers and `ckfp_state` holds the last checkpointed value of all
 * FPR registers for the current transaction.
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
    let mut buf = [0u64; 33];

    flush_fp_to_thread(target);

    // Copy to local buffer then write that out.
    for i in 0..32 {
        buf[i] = (*target).thread.TS_FPR(i as i32);
    }
    buf[32] = (*target).thread.fp_state.fpscr;
    membuf_write(&mut to, buf.as_ptr() as *const _, 33 * core::mem::size_of::<u64>())
}

/*
 * Regardless of transactions, `fp_state` holds the current running value of
 * all FPR registers and `ckfp_state` holds the last checkpointed value of all
 * FPR registers for the current transaction.
 *
 * Userspace interface buffer layout:
 *
 * struct data {
 *     u64 fpr[32];
 *     u64 fpscr;
 * };
 */
pub unsafe fn fpr_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    pos: *mut u32,
    count: *mut u32,
    kbuf: *mut *const core::ffi::c_void,
    ubuf: *mut *const core::ffi::c_void,
) -> i32 {
    let mut buf = [0u64; 33];

    flush_fp_to_thread(target);

    for i in 0..32 {
        buf[i] = (*target).thread.TS_FPR(i as i32);
    }
    buf[32] = (*target).thread.fp_state.fpscr;

    // Copy to local buffer then write that out.
    let ret = user_regset_copyin(pos, count, kbuf, ubuf, buf.as_mut_ptr() as *mut _, 0, -1i64 as usize);
    if ret != 0 {
        return ret;
    }

    for i in 0..32 {
        (*target).thread.TS_FPR(i as i32) = buf[i];
    }
    (*target).thread.fp_state.fpscr = buf[32];
    0
}

/*
 * Currently to set and get all the vsx state, you need to call the fp and VMX
 * calls as well. This only get/sets the lower 32 128bit VSX registers.
 */
pub unsafe fn vsr_active(target: *mut task_struct, regset: *const user_regset) -> i32 {
    flush_vsx_to_thread(target);
    if (*target).thread.used_vsr { (*regset).n } else { 0 }
}

/* Userspace interface buffer layout: struct data { u64 vsx[32]; }; */
pub unsafe fn vsr_get(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut to: membuf,
) -> i32 {
    let mut buf = [0u64; 32];

    flush_tmregs_to_thread(target);
    flush_fp_to_thread(target);
    flush_altivec_to_thread(target);
    flush_vsx_to_thread(target);

    for i in 0..32 {
        buf[i] = (*target).thread.fp_state.fpr[i][TS_VSRLOWOFFSET];
    }

    membuf_write(&mut to, buf.as_ptr() as *const _, 32 * core::mem::size_of::<f64>())
}

/* Userspace interface buffer layout: struct data { u64 vsx[32]; }; */
pub unsafe fn vsr_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    pos: *mut u32,
    count: *mut u32,
    kbuf: *mut *const core::ffi::c_void,
    ubuf: *mut *const core::ffi::c_void,
) -> i32 {
    let mut buf = [0u64; 32];

    flush_tmregs_to_thread(target);
    flush_fp_to_thread(target);
    flush_altivec_to_thread(target);
    flush_vsx_to_thread(target);

    for i in 0..32 {
        buf[i] = (*target).thread.fp_state.fpr[i][TS_VSRLOWOFFSET];
    }

    let ret = user_regset_copyin(
        pos, count, kbuf, ubuf, buf.as_mut_ptr() as *mut _, 0,
        32 * core::mem::size_of::<f64>(),
    );
    if ret == 0 {
        for i in 0..32 {
            (*target).thread.fp_state.fpr[i][TS_VSRLOWOFFSET] = buf[i];
        }
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
