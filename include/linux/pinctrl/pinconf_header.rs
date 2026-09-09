/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface the pinconfig portions of the pinctrl subsystem
 *
 * Copyright (C) 2011 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 * This interface is used in the core to keep track of pins.
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

#[repr(C)]
pub struct pinctrl_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

/**
 * struct pinconf_ops - pin config operations, to be implemented by
 * pin configuration capable drivers.
 * @is_generic: for pin controllers that want to use the generic interface,
 *\tthis flag tells the framework that it's generic.
 * @pin_config_get: get the config of a certain pin, if the requested config
 *\tis not available on this controller this should return -ENOTSUPP
 *\tand if it is available but disabled it should return -EINVAL
 * @pin_config_set: configure an individual pin
 * @pin_config_group_get: get configurations for an entire pin group; should
 *\treturn -ENOTSUPP and -EINVAL using the same rules as pin_config_get.
 * @pin_config_group_set: configure all pins in a group
 * @pin_config_dbg_show: optional debugfs display hook that will provide
 *\tper-device info for a certain pin in debugfs
 * @pin_config_group_dbg_show: optional debugfs display hook that will provide
 *\tper-device info for a certain group in debugfs
 * @pin_config_config_dbg_show: optional debugfs display hook that will decode
 *\tand display a driver's pin configuration parameter
 */
#[repr(C)]
pub struct pinconf_ops {
    // Present when CONFIG_GENERIC_PINCONF is enabled.
    #[cfg(feature = "CONFIG_GENERIC_PINCONF")]
    pub is_generic: bool,

    pub pin_config_get: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        pin: u32,
        config: *mut libc::c_ulong,
    ) -> libc::c_int>,
    pub pin_config_set: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        pin: u32,
        configs: *mut libc::c_ulong,
        num_configs: u32,
    ) -> libc::c_int>,
    pub pin_config_group_get: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        selector: u32,
        config: *mut libc::c_ulong,
    ) -> libc::c_int>,
    pub pin_config_group_set: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        selector: u32,
        configs: *mut libc::c_ulong,
        num_configs: u32,
    ) -> libc::c_int>,
    pub pin_config_dbg_show: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        s: *mut seq_file,
        offset: u32,
    )>,
    pub pin_config_group_dbg_show: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        s: *mut seq_file,
        selector: u32,
    )>,
    pub pin_config_config_dbg_show: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        s: *mut seq_file,
        config: libc::c_ulong,
    )>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
