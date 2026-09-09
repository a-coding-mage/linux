/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Support for generic time stamping devices on MII buses.
 * Copyright (C) 2018 Richard Cochran <richardcochran@gmail.com>
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here rather than reimplemented.

/// Callback interface to MII time stamping devices.
///
/// Drivers for PHY time stamping devices should embed this within a private
/// structure, obtaining a reference to it using container_of(). Drivers for
/// non-PHY time stamping devices should return a pointer to this from the
/// probe_channel() callback of their mii_timestamping_ctrl interface.
#[repr(C)]
pub struct mii_timestamper {
    /// Requests an Rx timestamp for `skb`.
    pub rxtstamp: Option<unsafe extern "C" fn(
        mii_ts: *mut mii_timestamper,
        skb: *mut sk_buff,
        type_: ::core::ffi::c_int,
    ) -> bool>,

    /// Requests a Tx timestamp for `skb`.
    pub txtstamp: Option<unsafe extern "C" fn(
        mii_ts: *mut mii_timestamper,
        skb: *mut sk_buff,
        type_: ::core::ffi::c_int,
    )>,

    /// Handles SIOCSHWTSTAMP ioctl for hardware time stamping.
    pub hwtstamp_set: Option<unsafe extern "C" fn(
        mii_ts: *mut mii_timestamper,
        kernel_config: *mut kernel_hwtstamp_config,
        extack: *mut netlink_ext_ack,
    ) -> ::core::ffi::c_int>,

    /// Handles SIOCGHWTSTAMP ioctl for hardware time stamping.
    pub hwtstamp_get: Option<unsafe extern "C" fn(
        mii_ts: *mut mii_timestamper,
        kernel_config: *mut kernel_hwtstamp_config,
    ) -> ::core::ffi::c_int>,

    /// Allows the device to respond to changes in the link state.
    pub link_state: Option<unsafe extern "C" fn(
        mii_ts: *mut mii_timestamper,
        phydev: *mut phy_device,
    )>,

    /// Handles ethtool queries for hardware time stamping.
    pub ts_info: Option<unsafe extern "C" fn(
        mii_ts: *mut mii_timestamper,
        ts_info: *mut kernel_ethtool_ts_info,
    ) -> ::core::ffi::c_int>,

    /// Remembers the device to which the instance belongs.
    pub device: *mut device,
}

/// MII time stamping controller interface.
#[repr(C)]
pub struct mii_timestamping_ctrl {
    /// Announces the presence of the `port` channel.
    pub probe_channel: Option<unsafe extern "C" fn(
        device: *mut device,
        port: ::core::ffi::c_uint,
    ) -> *mut mii_timestamper>,

    /// Releases an instance obtained via `probe_channel`.
    pub release_channel: Option<unsafe extern "C" fn(
        device: *mut device,
        mii_ts: *mut mii_timestamper,
    )>,
}

// The following declarations are available when CONFIG_NETWORK_PHY_TIMESTAMPING
// is enabled in the kernel build.
#[cfg(CONFIG_NETWORK_PHY_TIMESTAMPING)]
extern "C" {
    pub fn register_mii_tstamp_controller(
        device: *mut device,
        ctrl: *mut mii_timestamping_ctrl,
    ) -> ::core::ffi::c_int;

    pub fn unregister_mii_tstamp_controller(device: *mut device);

    pub fn register_mii_timestamper(
        node: *mut device_node,
        port: ::core::ffi::c_uint,
    ) -> *mut mii_timestamper;

    pub fn unregister_mii_timestamper(mii_ts: *mut mii_timestamper);
}

// When CONFIG_NETWORK_PHY_TIMESTAMPING is disabled, the C header provides
// these inline stubs.
#[cfg(not(CONFIG_NETWORK_PHY_TIMESTAMPING))]
#[inline]
pub unsafe fn register_mii_tstamp_controller(
    _device: *mut device,
    _ctrl: *mut mii_timestamping_ctrl,
) -> ::core::ffi::c_int {
    EOPNOTSUPP
}

#[cfg(not(CONFIG_NETWORK_PHY_TIMESTAMPING))]
#[inline]
pub unsafe fn unregister_mii_tstamp_controller(_device: *mut device) {}

#[cfg(not(CONFIG_NETWORK_PHY_TIMESTAMPING))]
#[inline]
pub unsafe fn register_mii_timestamper(
    _node: *mut device_node,
    _port: ::core::ffi::c_uint,
) -> *mut mii_timestamper {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_NETWORK_PHY_TIMESTAMPING))]
#[inline]
pub unsafe fn unregister_mii_timestamper(_mii_ts: *mut mii_timestamper) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
