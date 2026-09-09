// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) IBM Corporation, 2011
 *
 * Authors: Sukadev Bhattiprolu <sukadev@linux.vnet.ibm.com>
 *          Anton Blanchard <anton@au.ibm.com>
 */

unsafe extern "C" {
    fn in_interrupt() -> i32;
    fn preempt_disable();
    fn pagefault_disable();
    fn enable_kernel_altivec();
    fn disable_kernel_altivec();
    fn pagefault_enable();
    fn preempt_enable_no_resched();
    fn need_resched() -> i32;
    fn set_dec(value: i32);
    fn preempt_enable();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn enter_vmx_usercopy() -> i32 {
    if in_interrupt() != 0 {
        return 0;
    }

    preempt_disable();
    /*
     * We need to disable page faults as they can call schedule and
     * thus make us lose the VMX context. So on page faults, we just
     * fail which will cause a fallback to the normal non-vmx copy.
     */
    pagefault_disable();

    enable_kernel_altivec();

    1
}

// EXPORT_SYMBOL(enter_vmx_usercopy);

/*
 * This function must return 0 because we tail call optimise when calling
 * from __copy_tofrom_user_power7 which returns 0 on success.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_vmx_usercopy() -> i32 {
    disable_kernel_altivec();
    pagefault_enable();
    preempt_enable_no_resched();
    /*
     * Must never explicitly call schedule (including preempt_enable())
     * while in a kuap-unlocked user copy, because the AMR register will
     * not be saved and restored across context switch. However preempt
     * kernels need to be preempted as soon as possible if need_resched
     * is set and we are preemptible. The hack here is to schedule a
     * decrementer to fire here and reschedule for us if necessary.
     */
    if need_resched() != 0 {
        set_dec(1);
    }
    0
}

// EXPORT_SYMBOL(exit_vmx_usercopy);

/*
 * Can be called from kexec copy_page() path with MMU off. The kexec
 * code sets preempt_count to HARDIRQ_OFFSET so we return early here.
 * Since in_interrupt() is always inline, __no_sanitize_address on this
 * function is sufficient to avoid KASAN shadow memory accesses in real
 * mode.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn enter_vmx_ops() -> i32 {
    if in_interrupt() != 0 {
        return 0;
    }

    preempt_disable();

    enable_kernel_altivec();

    1
}

/*
 * All calls to this function will be optimised into tail calls. We are
 * passed a pointer to the destination which we return as required by a
 * memcpy implementation.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_vmx_ops(dest: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    disable_kernel_altivec();
    preempt_enable();
    dest
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
