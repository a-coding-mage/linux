/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2018, Intel Corporation. */

/* Translated from failover.h. The Linux netdevice types are supplied by
 * external dependencies; __rcu annotations are represented by raw pointers. */

#[repr(C)]
pub struct failover_ops {
    pub slave_pre_register: Option<unsafe extern "C" fn(
        slave_dev: *mut net_device,
        failover_dev: *mut net_device,
    ) -> ::core::ffi::c_int>,
    pub slave_register: Option<unsafe extern "C" fn(
        slave_dev: *mut net_device,
        failover_dev: *mut net_device,
    ) -> ::core::ffi::c_int>,
    pub slave_pre_unregister: Option<unsafe extern "C" fn(
        slave_dev: *mut net_device,
        failover_dev: *mut net_device,
    ) -> ::core::ffi::c_int>,
    pub slave_unregister: Option<unsafe extern "C" fn(
        slave_dev: *mut net_device,
        failover_dev: *mut net_device,
    ) -> ::core::ffi::c_int>,
    pub slave_link_change: Option<unsafe extern "C" fn(
        slave_dev: *mut net_device,
        failover_dev: *mut net_device,
    ) -> ::core::ffi::c_int>,
    pub slave_name_change: Option<unsafe extern "C" fn(
        slave_dev: *mut net_device,
        failover_dev: *mut net_device,
    ) -> ::core::ffi::c_int>,
    pub slave_handle_frame: Option<unsafe extern "C" fn(
        pskb: *mut *mut sk_buff,
    ) -> rx_handler_result_t>,
}

#[repr(C)]
pub struct failover {
    pub list: list_head,
    pub failover_dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub ops: *mut failover_ops,
}

unsafe extern "C" {
    pub fn failover_register(
        dev: *mut net_device,
        ops: *mut failover_ops,
    ) -> *mut failover;
    pub fn failover_unregister(failover: *mut failover);
    pub fn failover_slave_unregister(slave_dev: *mut net_device) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
