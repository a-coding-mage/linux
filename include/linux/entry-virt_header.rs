/* SPDX-License-Identifier: GPL-2.0 */
// C header guard: __LINUX_ENTRYVIRT_H

// Dependencies supplied by other translated kernel headers:
// linux/static_call_types.h, linux/resume_user_mode.h, linux/syscalls.h,
// linux/seccomp.h, linux/sched.h, linux/tick.h

/* Transfer to guest mode work */
// CONFIG_VIRT_XFER_TO_GUEST_WORK is a build-time configuration condition.
#[cfg(feature = "CONFIG_VIRT_XFER_TO_GUEST_WORK")]
pub const ARCH_XFER_TO_GUEST_MODE_WORK: ::core::ffi::c_ulong = 0;

#[cfg(feature = "CONFIG_VIRT_XFER_TO_GUEST_WORK")]
pub const XFER_TO_GUEST_MODE_WORK: ::core::ffi::c_ulong =
    _TIF_NEED_RESCHED
        | _TIF_NEED_RESCHED_LAZY
        | _TIF_SIGPENDING
        | _TIF_NOTIFY_SIGNAL
        | _TIF_NOTIFY_RESUME
        | ARCH_XFER_TO_GUEST_MODE_WORK;

/**
 * arch_xfer_to_guest_mode_handle_work - Architecture specific xfer to guest
 *                                      mode work handling function
 * @vcpu:    Pointer to current's VCPU data
 * @ti_work: Cached TIF flags gathered in xfer_to_guest_mode_handle_work()
 *
 * Invoked from xfer_to_guest_mode_handle_work(). Defaults to NOOP. Can be
 * replaced by architecture specific code.
 */
#[cfg(feature = "CONFIG_VIRT_XFER_TO_GUEST_WORK")]
pub unsafe extern "C" fn arch_xfer_to_guest_mode_handle_work(
    ti_work: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let _ = ti_work;
    0
}

/**
 * xfer_to_guest_mode_handle_work - Check and handle pending work which needs
 *                                   to be handled before going to guest mode
 *
 * Returns: 0 or an error code
 */
#[cfg(feature = "CONFIG_VIRT_XFER_TO_GUEST_WORK")]
unsafe extern "C" {
    pub fn xfer_to_guest_mode_handle_work() -> ::core::ffi::c_int;
}

/**
 * xfer_to_guest_mode_prepare - Perform last minute preparation work that
 *                              need to be handled while IRQs are disabled
 *                              upon entering to guest.
 *
 * Has to be invoked with interrupts disabled before the last call
 * to xfer_to_guest_mode_work_pending().
 */
#[cfg(feature = "CONFIG_VIRT_XFER_TO_GUEST_WORK")]
#[inline]
pub unsafe fn xfer_to_guest_mode_prepare() {
    lockdep_assert_irqs_disabled();
    tick_nohz_user_enter_prepare();
}

/**
 * __xfer_to_guest_mode_work_pending - Check if work is pending
 *
 * Returns: True if work pending, False otherwise.
 *
 * Bare variant of xfer_to_guest_mode_work_pending(). Can be called from
 * interrupt enabled code for racy quick checks with care.
 */
#[cfg(feature = "CONFIG_VIRT_XFER_TO_GUEST_WORK")]
#[inline]
pub unsafe fn __xfer_to_guest_mode_work_pending() -> bool {
    let ti_work: ::core::ffi::c_ulong = read_thread_flags();

    (ti_work & XFER_TO_GUEST_MODE_WORK) != 0
}

/**
 * xfer_to_guest_mode_work_pending - Check if work is pending which needs to be
 *                                   handled before returning to guest mode
 *
 * Returns: True if work pending, False otherwise.
 *
 * Has to be invoked with interrupts disabled before the transition to
 * guest mode.
 */
#[cfg(feature = "CONFIG_VIRT_XFER_TO_GUEST_WORK")]
#[inline]
pub unsafe fn xfer_to_guest_mode_work_pending() -> bool {
    lockdep_assert_irqs_disabled();
    __xfer_to_guest_mode_work_pending()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
