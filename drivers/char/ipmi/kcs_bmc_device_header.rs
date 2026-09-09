/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2021, IBM Corp. */

// C dependencies: <linux/irqreturn.h> and "kcs_bmc.h".

#[repr(C)]
pub struct kcs_bmc_device;

#[repr(C)]
pub struct kcs_bmc_device_ops {
    pub irq_mask_update:
        Option<unsafe extern "C" fn(kcs_bmc: *mut kcs_bmc_device, mask: u8, enable: u8)>,
    pub io_inputb:
        Option<unsafe extern "C" fn(kcs_bmc: *mut kcs_bmc_device, reg: u32) -> u8>,
    pub io_outputb:
        Option<unsafe extern "C" fn(kcs_bmc: *mut kcs_bmc_device, reg: u32, b: u8)>,
    pub io_updateb:
        Option<unsafe extern "C" fn(kcs_bmc: *mut kcs_bmc_device, reg: u32, mask: u8, b: u8)>,
}

extern "C" {
    pub fn kcs_bmc_handle_event(kcs_bmc: *mut kcs_bmc_device) -> irqreturn_t;
    pub fn kcs_bmc_add_device(kcs_bmc: *mut kcs_bmc_device) -> i32;
    pub fn kcs_bmc_remove_device(kcs_bmc: *mut kcs_bmc_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
