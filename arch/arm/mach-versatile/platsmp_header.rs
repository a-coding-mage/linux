/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2011 ARM Ltd.
 *  All Rights Reserved
 */

// Opaque declaration supplied by the surrounding kernel translation.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut versatile_cpu_release: core::ffi::c_int;

    pub fn versatile_secondary_startup();
    pub fn versatile_secondary_init(cpu: core::ffi::c_uint);
    pub fn versatile_boot_secondary(
        cpu: core::ffi::c_uint,
        idle: *mut task_struct,
    ) -> core::ffi::c_int;
    pub fn versatile_immitation_cpu_die(
        cpu: core::ffi::c_uint,
        actrl_mask: core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
