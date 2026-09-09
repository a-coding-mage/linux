/* SPDX-License-Identifier: GPL-2.0 */
/*
 * board initialization should put one of these into dev->platform_data
 * and place the sl811hs onto platform_bus named "sl811-hcd".
 */

use core::ffi::c_int;

// External dependency supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sl811_platform_data {
    pub can_wakeup: u8,

    /* given port_power, msec/2 after power on till power good */
    pub potpg: u8,

    /* mA/2 power supplied on this port (max = default = 250) */
    pub power: u8,

    /* sl811 relies on an external source of VBUS current */
    pub port_power: Option<unsafe extern "C" fn(dev: *mut device, is_on: c_int)>,

    /* pulse sl811 nRST (probably with a GPIO) */
    pub reset: Option<unsafe extern "C" fn(dev: *mut device)>,

    /* some boards need something like these: */
    /* int		(*check_overcurrent)(struct device *dev); */
    /* void		(*clock_enable)(struct device *dev, int is_on); */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
