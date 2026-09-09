/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/notifier.h.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

/*
 * The pvclock gtod notifier is called when the system time is updated
 * and is used to keep guest time synchronized with host time.
 *
 * The 'action' parameter in the notifier function is false (0), or
 * true (non-zero) if system time was stepped.
 */
unsafe extern "C" {
    pub fn pvclock_gtod_register_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn pvclock_gtod_unregister_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
