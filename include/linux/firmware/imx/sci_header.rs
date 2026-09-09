/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 *
 * Header file containing the public System Controller Interface (SCI)
 * definitions.
 */

// Dependencies supplied by the corresponding Linux firmware/interface modules:
// linux/firmware/imx/ipc.h
// linux/firmware/imx/svc/misc.h
// linux/firmware/imx/svc/pm.h
// linux/firmware/imx/svc/rm.h

// The C condition is CONFIG_IMX_SCU.  The declarations below are selected
// when the corresponding Rust configuration is enabled.
#[cfg(feature = "CONFIG_IMX_SCU")]
extern "C" {
    pub fn imx_scu_enable_general_irq_channel(dev: *mut device) -> i32;
    pub fn imx_scu_irq_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn imx_scu_irq_unregister_notifier(nb: *mut notifier_block) -> i32;
    pub fn imx_scu_irq_group_enable(group: u8, mask: u32, enable: u8) -> i32;
    pub fn imx_scu_irq_get_status(group: u8, irq_status: *mut u32) -> i32;
    pub fn imx_scu_soc_init(dev: *mut device) -> i32;
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_soc_init(dev: *mut device) -> i32 {
    let _ = dev;
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_enable_general_irq_channel(dev: *mut device) -> i32 {
    let _ = dev;
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_irq_register_notifier(nb: *mut notifier_block) -> i32 {
    let _ = nb;
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_irq_unregister_notifier(nb: *mut notifier_block) -> i32 {
    let _ = nb;
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_irq_group_enable(group: u8, mask: u32, enable: u8) -> i32 {
    let _ = (group, mask, enable);
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub unsafe fn imx_scu_irq_get_status(group: u8, irq_status: *mut u32) -> i32 {
    let _ = (group, irq_status);
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
