/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/irqchip/arm-gic-common.h
 *
 * Copyright (C) 2016 ARM Limited, All Rights Reserved.
 */

// Dependency: linux/irqchip/arm-vgic-info.h

pub const GICD_INT_DEF_PRI: u32 = 0xa0;

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn gicv2m_init(
        parent_handle: *mut fwnode_handle,
        parent: *mut irq_domain,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
