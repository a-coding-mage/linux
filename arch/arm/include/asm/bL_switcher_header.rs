/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/bL_switcher.h
 *
 * Created by:  Nicolas Pitre, April 2012
 * Copyright:   (C) 2012-2013  Linaro Limited
 */

// Dependency intent: linux/compiler.h and linux/types.h.

pub type bL_switch_completion_handler = unsafe extern "C" fn(cookie: *mut core::ffi::c_void);

unsafe extern "C" {
    pub fn bL_switch_request_cb(
        cpu: core::ffi::c_uint,
        new_cluster_id: core::ffi::c_uint,
        completer: Option<bL_switch_completion_handler>,
        completer_cookie: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn bL_switch_request(
    cpu: core::ffi::c_uint,
    new_cluster_id: core::ffi::c_uint,
) -> core::ffi::c_int {
    bL_switch_request_cb(cpu, new_cluster_id, None, core::ptr::null_mut())
}

/*
 * Register here to be notified about runtime enabling/disabling of
 * the switcher.
 *
 * The notifier chain is called with the switcher activation lock held:
 * the switcher will not be enabled or disabled during callbacks.
 * Callbacks must not call bL_switcher_{get,put}_enabled().
 */
pub const BL_NOTIFY_PRE_ENABLE: core::ffi::c_int = 0;
pub const BL_NOTIFY_POST_ENABLE: core::ffi::c_int = 1;
pub const BL_NOTIFY_PRE_DISABLE: core::ffi::c_int = 2;
pub const BL_NOTIFY_POST_DISABLE: core::ffi::c_int = 3;

// CONFIG_BL_SWITCHER is a build-time configuration condition from the C header.
// The declarations below correspond to the enabled configuration.

// External dependency supplied by the surrounding Linux translation.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn bL_switcher_register_notifier(nb: *mut notifier_block) -> core::ffi::c_int;
    pub fn bL_switcher_unregister_notifier(nb: *mut notifier_block) -> core::ffi::c_int;

    /*
     * Use these functions to temporarily prevent enabling/disabling of
     * the switcher.
     * bL_switcher_get_enabled() returns true if the switcher is currently
     * enabled.  Each call to bL_switcher_get_enabled() must be followed
     * by a call to bL_switcher_put_enabled().  These functions are not
     * recursive.
     */
    pub fn bL_switcher_get_enabled() -> bool;
    pub fn bL_switcher_put_enabled();

    pub fn bL_switcher_trace_trigger() -> core::ffi::c_int;
    pub fn bL_switcher_get_logical_index(mpidr: u32) -> core::ffi::c_int;
}

// CONFIG_BL_SWITCHER disabled fallbacks:
// bL_switcher_register_notifier and bL_switcher_unregister_notifier return 0;
// bL_switcher_get_enabled returns false; bL_switcher_put_enabled does nothing;
// bL_switcher_trace_trigger returns 0; bL_switcher_get_logical_index returns -EUNATCH.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
