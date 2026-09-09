/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2021, IBM Corp. */

// Translated from kcs_bmc_client.h.
// The Linux irqreturn_t type and kcs_bmc.h declarations are supplied by
// external dependencies.

#[repr(C)]
pub struct kcs_bmc_driver_ops {
    pub add_device:
        Option<unsafe extern "C" fn(kcs_bmc: *mut crate::kcs_bmc_device) -> ::core::ffi::c_int>,
    pub remove_device:
        Option<unsafe extern "C" fn(kcs_bmc: *mut crate::kcs_bmc_device) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct kcs_bmc_driver {
    pub entry: crate::list_head,
    pub ops: *const kcs_bmc_driver_ops,
}

#[repr(C)]
pub struct kcs_bmc_client_ops {
    pub event: Option<unsafe extern "C" fn(client: *mut kcs_bmc_client) -> crate::irqreturn_t>,
}

#[repr(C)]
pub struct kcs_bmc_client {
    pub ops: *const kcs_bmc_client_ops,
    pub dev: *mut crate::kcs_bmc_device,
}

unsafe extern "C" {
    pub fn kcs_bmc_register_driver(drv: *mut kcs_bmc_driver);
    pub fn kcs_bmc_unregister_driver(drv: *mut kcs_bmc_driver);

    pub fn kcs_bmc_enable_device(
        kcs_bmc: *mut crate::kcs_bmc_device,
        client: *mut kcs_bmc_client,
    ) -> ::core::ffi::c_int;
    pub fn kcs_bmc_disable_device(
        kcs_bmc: *mut crate::kcs_bmc_device,
        client: *mut kcs_bmc_client,
    );

    pub fn kcs_bmc_update_event_mask(
        kcs_bmc: *mut crate::kcs_bmc_device,
        mask: u8,
        events: u8,
    );

    pub fn kcs_bmc_read_data(kcs_bmc: *mut crate::kcs_bmc_device) -> u8;
    pub fn kcs_bmc_write_data(kcs_bmc: *mut crate::kcs_bmc_device, data: u8);
    pub fn kcs_bmc_read_status(kcs_bmc: *mut crate::kcs_bmc_device) -> u8;
    pub fn kcs_bmc_write_status(kcs_bmc: *mut crate::kcs_bmc_device, data: u8);
    pub fn kcs_bmc_update_status(
        kcs_bmc: *mut crate::kcs_bmc_device,
        mask: u8,
        val: u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
