/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CCI cache coherent interconnect support
 *
 * Copyright (C) 2013 ARM Ltd.
 */

// C dependencies: linux/errno.h, linux/types.h, and asm/arm-cci.h.

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[cfg(CONFIG_ARM_CCI)]
unsafe extern "C" {
    pub fn cci_probed() -> bool;
}

#[cfg(not(CONFIG_ARM_CCI))]
#[inline]
pub fn cci_probed() -> bool {
    false
}

#[cfg(CONFIG_ARM_CCI400_PORT_CTRL)]
unsafe extern "C" {
    pub fn cci_ace_get_port(dn: *mut device_node) -> i32;
    pub fn cci_disable_port_by_cpu(mpidr: u64) -> i32;
    pub fn __cci_control_port_by_device(dn: *mut device_node, enable: bool) -> i32;
    pub fn __cci_control_port_by_index(port: u32, enable: bool) -> i32;
}

#[cfg(not(CONFIG_ARM_CCI400_PORT_CTRL))]
#[inline]
pub unsafe fn cci_ace_get_port(_dn: *mut device_node) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(CONFIG_ARM_CCI400_PORT_CTRL))]
#[inline]
pub fn cci_disable_port_by_cpu(_mpidr: u64) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(CONFIG_ARM_CCI400_PORT_CTRL))]
#[inline]
pub unsafe fn __cci_control_port_by_device(
    _dn: *mut device_node,
    _enable: bool,
) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(CONFIG_ARM_CCI400_PORT_CTRL))]
#[inline]
pub fn __cci_control_port_by_index(_port: u32, _enable: bool) -> i32 {
    -19 // -ENODEV
}

unsafe extern "C" {
    pub fn cci_enable_port_for_self();
}

#[inline]
pub unsafe fn cci_disable_port_by_device(dev: *mut device_node) -> i32 {
    __cci_control_port_by_device(dev, false)
}

#[inline]
pub unsafe fn cci_enable_port_by_device(dev: *mut device_node) -> i32 {
    __cci_control_port_by_device(dev, true)
}

#[inline]
pub fn cci_disable_port_by_index(dev: u32) -> i32 {
    __cci_control_port_by_index(dev, false)
}

#[inline]
pub fn cci_enable_port_by_index(dev: u32) -> i32 {
    __cci_control_port_by_index(dev, true)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
