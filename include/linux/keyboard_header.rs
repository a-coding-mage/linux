/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations supplied by <uapi/linux/keyboard.h>.

// Opaque declarations corresponding to C forward declarations and external types.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vc_data {
    _private: [u8; 0],
}

extern "C" {
    pub static mut key_maps: [*mut u16; MAX_NR_KEYMAPS];
    pub static mut plain_map: [u16; NR_KEYS];
}

#[repr(C)]
pub struct keyboard_notifier_param {
    pub vc: *mut vc_data,       /* VC on which the keyboard press was done */
    pub down: ::core::ffi::c_int,     /* Pressure of the key? */
    pub shift: ::core::ffi::c_int,    /* Current shift mask */
    pub ledstate: ::core::ffi::c_int, /* Current led state */
    pub value: ::core::ffi::c_uint,   /* keycode, unicode value or keysym */
}

extern "C" {
    pub fn register_keyboard_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn unregister_keyboard_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
