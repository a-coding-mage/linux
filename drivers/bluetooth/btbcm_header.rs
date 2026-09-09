/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 *  Bluetooth support for Broadcom devices
 *
 *  Copyright (C) 2015  Intel Corporation
 */

pub const BCM_UART_CLOCK_48MHZ: u8 = 0x01;
pub const BCM_UART_CLOCK_24MHZ: u8 = 0x02;

#[repr(C, packed)]
pub struct bcm_update_uart_baud_rate {
    pub zero: u16,
    pub baud_rate: u32,
}

#[repr(C, packed)]
pub struct bcm_write_uart_clock_setting {
    pub type_: u8,
}

#[repr(C, packed)]
pub struct bcm_set_sleep_mode {
    pub sleep_mode: u8,
    pub idle_host: u8,
    pub idle_dev: u8,
    pub bt_wake_active: u8,
    pub host_wake_active: u8,
    pub allow_host_sleep: u8,
    pub combine_modes: u8,
    pub tristate_control: u8,
    pub usb_auto_sleep: u8,
    pub usb_resume_timeout: u8,
    pub break_to_host: u8,
    pub pulsed_host_wake: u8,
}

#[repr(C, packed)]
pub struct bcm_set_pcm_int_params {
    pub routing: u8,
    pub rate: u8,
    pub frame_sync: u8,
    pub sync_mode: u8,
    pub clock_mode: u8,
}

#[repr(C, packed)]
pub struct bcm_set_pcm_format_params {
    pub lsb_first: u8,
    pub fill_value: u8,
    pub fill_method: u8,
    pub fill_num: u8,
    pub right_justify: u8,
}

// The CONFIG_BT_BCM build-time condition is preserved here as a Rust cfg.
#[cfg(feature = "CONFIG_BT_BCM")]
extern "C" {
    pub fn btbcm_check_bdaddr(hdev: *mut hci_dev) -> i32;
    pub fn btbcm_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> i32;
    pub fn btbcm_patchram(hdev: *mut hci_dev, fw: *const firmware) -> i32;
    pub fn btbcm_read_pcm_int_params(
        hdev: *mut hci_dev,
        params: *mut bcm_set_pcm_int_params,
    ) -> i32;
    pub fn btbcm_write_pcm_int_params(
        hdev: *mut hci_dev,
        params: *const bcm_set_pcm_int_params,
    ) -> i32;
    pub fn btbcm_setup_patchram(hdev: *mut hci_dev) -> i32;
    pub fn btbcm_setup_apple(hdev: *mut hci_dev) -> i32;
    pub fn btbcm_initialize(
        hdev: *mut hci_dev,
        fw_load_done: *mut bool,
        use_autobaud_mode: bool,
    ) -> i32;
    pub fn btbcm_finalize(
        hdev: *mut hci_dev,
        fw_load_done: *mut bool,
        use_autobaud_mode: bool,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_check_bdaddr(_hdev: *mut hci_dev) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_set_bdaddr(_hdev: *mut hci_dev, _bdaddr: *const bdaddr_t) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_read_pcm_int_params(
    _hdev: *mut hci_dev,
    _params: *mut bcm_set_pcm_int_params,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_write_pcm_int_params(
    _hdev: *mut hci_dev,
    _params: *const bcm_set_pcm_int_params,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_patchram(_hdev: *mut hci_dev, _fw: *const firmware) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_setup_patchram(_hdev: *mut hci_dev) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_setup_apple(_hdev: *mut hci_dev) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_initialize(
    _hdev: *mut hci_dev,
    _fw_load_done: *mut bool,
    _use_autobaud_mode: bool,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_BT_BCM"))]
#[inline]
pub unsafe fn btbcm_finalize(
    _hdev: *mut hci_dev,
    _fw_load_done: *mut bool,
    _use_autobaud_mode: bool,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
