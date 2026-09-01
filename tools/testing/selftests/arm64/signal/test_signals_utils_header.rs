/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019 ARM Limited */

/* C header dependencies removed from executable Rust:
 * <assert.h>, <stdio.h>, <stdint.h>, <string.h>, <linux/compiler.h>,
 * and "test_signals.h".
 */

use core::arch::asm;
use core::ffi::{c_int, c_void};

extern "C" {
    pub fn test_init(td: *mut tdescr) -> c_int;
    pub fn test_setup(td: *mut tdescr) -> c_int;
    pub fn test_cleanup(td: *mut tdescr);
    pub fn test_run(td: *mut tdescr) -> c_int;
    pub fn test_result(td: *mut tdescr);
}

/* If __NR_prctl is not supplied by generated bindings, the C header used 167. */
pub const __NR_prctl: i64 = 167;

/*
 * The prctl takes 1 argument but we need to ensure that the other
 * values passed in registers to the syscall are zero since the kernel
 * validates them.
 */
#[inline(always)]
pub unsafe fn gcs_set_state(state: i64) -> i64 {
    let mut _arg1: i64 = PR_SET_SHADOW_STACK_STATUS as i64;
    let _arg2: i64 = state as i64;
    let _arg3: i64 = 0;
    let _arg4: i64 = 0;
    let _arg5: i64 = 0;
    let _num: i64 = __NR_prctl;

    unsafe {
        asm!(
            "svc #0",
            inout("x0") _arg1,
            in("x1") _arg2,
            in("x2") _arg3,
            in("x3") _arg4,
            in("x4") _arg5,
            in("x8") _num,
            options(nostack),
        );
    }
    _arg1
}

#[inline(always)]
pub unsafe fn get_gcspr_el0() -> u64 {
    let val: u64;

    unsafe {
        asm!("mrs {0}, S3_3_C2_C5_1", out(reg) val, options(nostack, preserves_flags));
    }

    val
}

pub const SYS_POR_EL0: &str = "S3_3_C10_C2_4";

#[inline]
pub unsafe fn get_por_el0() -> u64 {
    let val: u64;

    unsafe {
        asm!("mrs {0}, S3_3_C10_C2_4", out(reg) val, options(nostack, preserves_flags));
    }

    val
}

#[inline]
pub unsafe fn set_por_el0(val: u64) {
    unsafe {
        asm!("msr S3_3_C10_C2_4, {0}", in(reg) val, options(nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn feats_ok(td: *mut tdescr) -> bool {
    unsafe {
        if ((*td).feats_incompatible & (*td).feats_supported) != 0 {
            return false;
        }
        ((*td).feats_required & (*td).feats_supported) == (*td).feats_required
    }
}

/*
 * Obtaining a valid and full-blown ucontext_t from userspace is tricky:
 * libc getcontext does() not save all the regs and messes with some of
 * them (pstate value in particular is not reliable).
 *
 * Here we use a service signal to grab the ucontext_t from inside a
 * dedicated signal handler, since there, it is populated by Kernel
 * itself in setup_sigframe(). The grabbed context is then stored and
 * made available in td->live_uc.
 *
 * As service-signal is used a SIGTRAP induced by a 'brk' instruction,
 * because here we have to avoid syscalls to trigger the signal since
 * they would cause any SVE sigframe content (if any) to be removed.
 *
 * Anyway this function really serves a dual purpose:
 *
 * 1. grab a valid sigcontext into td->live_uc for result analysis: in
 * such case it returns 1.
 *
 * 2. detect if, somehow, a previously grabbed live_uc context has been
 * used actively with a sigreturn: in such a case the execution would have
 * magically resumed in the middle of this function itself (seen_already==1):
 * in such a case return 0, since in fact we have not just simply grabbed
 * the context.
 *
 * This latter case is useful to detect when a fake_sigreturn test-case has
 * unexpectedly survived without hitting a SEGV.
 *
 * Note that the case of runtime dynamically sized sigframes (like in SVE
 * context) is still NOT addressed: sigframe size is supposed to be fixed
 * at sizeof(ucontext_t).
 */
#[inline(always)]
pub unsafe fn get_current_context(
    td: *mut tdescr,
    dest_uc: *mut ucontext_t,
    dest_sz: usize,
) -> bool {
    static mut SEEN_ALREADY: bool = false;
    let mut i: c_int;
    let uc: *mut i8 = dest_uc as *mut i8;

    assert!(!td.is_null() && !dest_uc.is_null());
    /* it's a genuine invocation..reinit */
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SEEN_ALREADY), false);
        (*td).live_uc_valid = 0;
        (*td).live_sz = dest_sz;
    }

    /*
     * This is a memset() but we don't want the compiler to
     * optimise it into either instructions or a library call
     * which might be incompatible with streaming mode.
     */
    i = 0;
    while (i as usize) < unsafe { (*td).live_sz } {
        unsafe {
            *uc.add(i as usize) = 0;
            OPTIMIZER_HIDE_VAR(uc);
        }
        i += 1;
    }

    unsafe {
        (*td).live_uc = dest_uc;
    }
    /*
     * Grab ucontext_t triggering a SIGTRAP.
     *
     * Note that:
     * - live_uc_valid is declared volatile sig_atomic_t in
     *   struct tdescr since it will be changed inside the
     *   sig_copyctx handler
     * - the additional 'memory' clobber is there to avoid possible
     *   compiler's assumption on live_uc_valid and the content
     *   pointed by dest_uc, which are all changed inside the signal
     *   handler
     * - BRK causes a debug exception which is handled by the Kernel
     *   and finally causes the SIGTRAP signal to be delivered to this
     *   test thread. Since such delivery happens on the ret_to_user()
     *   /do_notify_resume() debug exception return-path, we are sure
     *   that the registered SIGTRAP handler has been run to completion
     *   before the execution path is restored here: as a consequence
     *   we can be sure that the volatile sig_atomic_t live_uc_valid
     *   carries a meaningful result. Being in a single thread context
     *   we'll also be sure that any access to memory modified by the
     *   handler (namely ucontext_t) will be visible once returned.
     * - note that since we are using a breakpoint instruction here
     *   to cause a SIGTRAP, the ucontext_t grabbed from the signal
     *   handler would naturally contain a PC pointing exactly to this
     *   BRK line, which means that, on return from the signal handler,
     *   or if we place the ucontext_t on the stack to fake a sigreturn,
     *   we'll end up in an infinite loop of BRK-SIGTRAP-handler.
     *   For this reason we take care to artificially move forward the
     *   PC to the next instruction while inside the signal handler.
     */
    unsafe {
        asm!("brk #666", inout("m") *dest_uc, options(nostack));
    }

    /*
     * If we were grabbing a streaming mode context then we may
     * have entered streaming mode behind the system's back and
     * libc or compiler generated code might decide to do
     * something invalid in streaming mode, or potentially even
     * the state of ZA.  Issue a SMSTOP to exit both now we have
     * grabbed the state.
     */
    if unsafe { ((*td).feats_supported & FEAT_SME) != 0 } {
        unsafe {
            asm!("msr S0_3_C4_C6_3, xzr", options(nostack, preserves_flags));
        }
    }

    /*
     * If we get here with seen_already==1 it implies the td->live_uc
     * context has been used to get back here....this probably means
     * a test has failed to cause a SEGV...anyway live_uc does not
     * point to a just acquired copy of ucontext_t...so return 0
     */
    if unsafe { core::ptr::read_volatile(core::ptr::addr_of!(SEEN_ALREADY)) } {
        unsafe {
            fprintf(
                stdout,
                b"Unexpected successful sigreturn detected: live_uc is stale !\n\0".as_ptr()
                    as *const i8,
            );
        }
        return false;
    }
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SEEN_ALREADY), true);
    }

    unsafe { (*td).live_uc_valid != 0 }
}

extern "C" {
    pub fn fake_sigreturn(sigframe: *mut c_void, sz: usize, misalign_bytes: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
