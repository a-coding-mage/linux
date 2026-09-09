/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus SVC code
 *
 * Copyright 2015 Google Inc.
 * Copyright 2015 Linaro Ltd.
 */

// Translated from svc.h. Types and functions supplied by other headers are
// intentionally referenced as external dependencies.

use core::ffi::c_void;

#[repr(C)]
pub struct gb_svc_l2_timer_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gb_host_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gb_connection {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gb_svc_watchdog {
    _private: [u8; 0],
}

pub const GB_SVC_CPORT_FLAG_E2EFC: u32 = 1u32 << 0;
pub const GB_SVC_CPORT_FLAG_CSD_N: u32 = 1u32 << 1;
pub const GB_SVC_CPORT_FLAG_CSV_N: u32 = 1u32 << 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_svc_state {
    GB_SVC_STATE_RESET = 0,
    GB_SVC_STATE_PROTOCOL_VERSION,
    GB_SVC_STATE_SVC_HELLO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_svc_watchdog_bite {
    GB_SVC_WATCHDOG_BITE_RESET_UNIPRO = 0,
    GB_SVC_WATCHDOG_BITE_PANIC_KERNEL,
}

#[repr(C)]
pub struct svc_debugfs_pwrmon_rail {
    pub id: u8,
    pub svc: *mut gb_svc,
}

#[repr(C)]
pub struct gb_svc {
    pub dev: device,

    pub hd: *mut gb_host_device,
    pub connection: *mut gb_connection,
    pub state: gb_svc_state,
    pub device_id_map: ida,
    pub wq: *mut workqueue_struct,

    pub endo_id: u16,
    pub ap_intf_id: u8,

    pub protocol_major: u8,
    pub protocol_minor: u8,

    pub watchdog: *mut gb_svc_watchdog,
    pub action: gb_svc_watchdog_bite,

    pub debugfs_dentry: *mut dentry,
    pub pwrmon_rails: *mut svc_debugfs_pwrmon_rail,
}

// Equivalent to container_of(d, struct gb_svc, dev); requires the C layout
// and returns the enclosing structure pointer.
pub unsafe fn to_gb_svc(d: *mut device) -> *mut gb_svc {
    (d as *mut u8).sub(core::mem::offset_of!(gb_svc, dev)) as *mut gb_svc
}

extern "C" {
    pub fn gb_svc_create(hd: *mut gb_host_device) -> *mut gb_svc;
    pub fn gb_svc_add(svc: *mut gb_svc) -> i32;
    pub fn gb_svc_del(svc: *mut gb_svc);
    pub fn gb_svc_put(svc: *mut gb_svc);

    pub fn gb_svc_pwrmon_intf_sample_get(
        svc: *mut gb_svc,
        intf_id: u8,
        measurement_type: u8,
        value: *mut u32,
    ) -> i32;
    pub fn gb_svc_intf_device_id(svc: *mut gb_svc, intf_id: u8, device_id: u8) -> i32;
    pub fn gb_svc_route_create(
        svc: *mut gb_svc,
        intf1_id: u8,
        dev1_id: u8,
        intf2_id: u8,
        dev2_id: u8,
    ) -> i32;
    pub fn gb_svc_route_destroy(svc: *mut gb_svc, intf1_id: u8, intf2_id: u8);
    pub fn gb_svc_connection_create(
        svc: *mut gb_svc,
        intf1_id: u8,
        cport1_id: u16,
        intf2_id: u8,
        cport2_id: u16,
        cport_flags: u8,
    ) -> i32;
    pub fn gb_svc_connection_destroy(
        svc: *mut gb_svc,
        intf1_id: u8,
        cport1_id: u16,
        intf2_id: u8,
        cport2_id: u16,
    );
    pub fn gb_svc_intf_eject(svc: *mut gb_svc, intf_id: u8) -> i32;
    pub fn gb_svc_intf_vsys_set(svc: *mut gb_svc, intf_id: u8, enable: bool) -> i32;
    pub fn gb_svc_intf_refclk_set(svc: *mut gb_svc, intf_id: u8, enable: bool) -> i32;
    pub fn gb_svc_intf_unipro_set(svc: *mut gb_svc, intf_id: u8, enable: bool) -> i32;
    pub fn gb_svc_intf_activate(svc: *mut gb_svc, intf_id: u8, intf_type: *mut u8) -> i32;
    pub fn gb_svc_intf_resume(svc: *mut gb_svc, intf_id: u8) -> i32;

    pub fn gb_svc_dme_peer_get(
        svc: *mut gb_svc,
        intf_id: u8,
        attr: u16,
        selector: u16,
        value: *mut u32,
    ) -> i32;
    pub fn gb_svc_dme_peer_set(
        svc: *mut gb_svc,
        intf_id: u8,
        attr: u16,
        selector: u16,
        value: u32,
    ) -> i32;
    pub fn gb_svc_intf_set_power_mode(
        svc: *mut gb_svc,
        intf_id: u8,
        hs_series: u8,
        tx_mode: u8,
        tx_gear: u8,
        tx_nlanes: u8,
        tx_amplitude: u8,
        tx_hs_equalizer: u8,
        rx_mode: u8,
        rx_gear: u8,
        rx_nlanes: u8,
        flags: u8,
        quirks: u32,
        local: *mut gb_svc_l2_timer_cfg,
        remote: *mut gb_svc_l2_timer_cfg,
    ) -> i32;
    pub fn gb_svc_intf_set_power_mode_hibernate(svc: *mut gb_svc, intf_id: u8) -> i32;
    pub fn gb_svc_ping(svc: *mut gb_svc) -> i32;
    pub fn gb_svc_watchdog_create(svc: *mut gb_svc) -> i32;
    pub fn gb_svc_watchdog_destroy(svc: *mut gb_svc);
    pub fn gb_svc_watchdog_enabled(svc: *mut gb_svc) -> bool;
    pub fn gb_svc_watchdog_enable(svc: *mut gb_svc) -> i32;
    pub fn gb_svc_watchdog_disable(svc: *mut gb_svc) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
