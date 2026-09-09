/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/irqchip/arm-vgic-info.h
 *
 * Copyright (C) 2016 ARM Limited, All Rights Reserved.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h and linux/ioport.h

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gic_type {
    /* Full GICv2 */
    GIC_V2,
    /* Full GICv3, optionally with v2 compat */
    GIC_V3,
    /* Full GICv5, optionally with v3 compat */
    GIC_V5,
}

#[repr(C)]
pub struct gic_kvm_info {
    /* GIC type */
    pub type_: gic_type,
    /* Virtual CPU interface */
    pub vcpu: resource,
    /* GICv2 GICC VA */
    pub gicc_base: *mut core::ffi::c_void,
    /* Interrupt number */
    pub maint_irq: u32,
    /* No interrupt mask, no need to use the above field */
    pub no_maint_irq_mask: bool,
    /* Virtual control interface */
    pub vctrl: resource,
    /* vlpi support */
    pub has_v4: bool,
    /* rvpeid support */
    pub has_v4_1: bool,
    /* Deactivation impared, subpar stuff */
    pub no_hw_deactivation: bool,
}

// CONFIG_KVM is a build-time condition supplied by the surrounding build.
#[cfg(CONFIG_KVM)]
extern "C" {
    pub fn vgic_set_kvm_info(info: *const gic_kvm_info);
}

#[cfg(not(CONFIG_KVM))]
#[inline]
pub unsafe fn vgic_set_kvm_info(_info: *const gic_kvm_info) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
