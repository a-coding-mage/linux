/* SPDX-License-Identifier: GPL-2.0-or-later */

#[repr(C)]
pub struct dsa_port {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdev_lag_upper_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernel_hwtstamp_config {
    _private: [u8; 0],
}

extern "C" {
    pub fn dsa_conduit_setup(dev: *mut net_device, cpu_dp: *mut dsa_port) -> i32;
    pub fn dsa_conduit_teardown(dev: *mut net_device);
    pub fn dsa_conduit_lag_setup(
        lag_dev: *mut net_device,
        cpu_dp: *mut dsa_port,
        uinfo: *mut netdev_lag_upper_info,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn dsa_conduit_lag_teardown(lag_dev: *mut net_device, cpu_dp: *mut dsa_port);
    pub fn __dsa_conduit_hwtstamp_validate(
        dev: *mut net_device,
        config: *const kernel_hwtstamp_config,
        extack: *mut netlink_ext_ack,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
