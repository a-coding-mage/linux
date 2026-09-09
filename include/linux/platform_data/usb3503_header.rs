/* SPDX-License-Identifier: GPL-2.0 */

pub const USB3503_I2C_NAME: &str = "usb3503";

pub const USB3503_OFF_PORT1: u32 = 1 << 1;
pub const USB3503_OFF_PORT2: u32 = 1 << 2;
pub const USB3503_OFF_PORT3: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum usb3503_mode {
    USB3503_MODE_UNKNOWN,
    USB3503_MODE_HUB,
    USB3503_MODE_STANDBY,
    USB3503_MODE_BYPASS,
}

#[repr(C)]
pub struct usb3503_platform_data {
    pub initial_mode: usb3503_mode,
    pub port_off_mask: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
