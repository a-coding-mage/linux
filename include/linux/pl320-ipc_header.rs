/* SPDX-License-Identifier: GPL-2.0-only */
/*
 */

// External type supplied by the surrounding kernel interfaces.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    pub fn pl320_ipc_transmit(data: *mut u32) -> i32;
    pub fn pl320_ipc_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn pl320_ipc_unregister_notifier(nb: *mut notifier_block) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
