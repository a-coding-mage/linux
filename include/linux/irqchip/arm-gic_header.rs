/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  include/linux/irqchip/arm-gic.h
 *
 *  Copyright (C) 2002 ARM Limited, All Rights Reserved.
 */

pub const GIC_CPU_CTRL: u32 = 0x00;
pub const GIC_CPU_PRIMASK: u32 = 0x04;
pub const GIC_CPU_BINPOINT: u32 = 0x08;
pub const GIC_CPU_INTACK: u32 = 0x0c;
pub const GIC_CPU_EOI: u32 = 0x10;
pub const GIC_CPU_RUNNINGPRI: u32 = 0x14;
pub const GIC_CPU_HIGHPRI: u32 = 0x18;
pub const GIC_CPU_ALIAS_BINPOINT: u32 = 0x1c;
pub const GIC_CPU_ACTIVEPRIO: u32 = 0xd0;
pub const GIC_CPU_IDENT: u32 = 0xfc;
pub const GIC_CPU_DEACTIVATE: u32 = 0x1000;

pub const GICC_ENABLE: u32 = 0x1;
pub const GICC_INT_PRI_THRESHOLD: u32 = 0xf0;

pub const GIC_CPU_CTRL_EnableGrp0_SHIFT: u32 = 0;
pub const GIC_CPU_CTRL_EnableGrp0: u32 = 1 << GIC_CPU_CTRL_EnableGrp0_SHIFT;
pub const GIC_CPU_CTRL_EnableGrp1_SHIFT: u32 = 1;
pub const GIC_CPU_CTRL_EnableGrp1: u32 = 1 << GIC_CPU_CTRL_EnableGrp1_SHIFT;
pub const GIC_CPU_CTRL_AckCtl_SHIFT: u32 = 2;
pub const GIC_CPU_CTRL_AckCtl: u32 = 1 << GIC_CPU_CTRL_AckCtl_SHIFT;
pub const GIC_CPU_CTRL_FIQEn_SHIFT: u32 = 3;
pub const GIC_CPU_CTRL_FIQEn: u32 = 1 << GIC_CPU_CTRL_FIQEn_SHIFT;
pub const GIC_CPU_CTRL_CBPR_SHIFT: u32 = 4;
pub const GIC_CPU_CTRL_CBPR: u32 = 1 << GIC_CPU_CTRL_CBPR_SHIFT;
pub const GIC_CPU_CTRL_EOImodeNS_SHIFT: u32 = 9;
pub const GIC_CPU_CTRL_EOImodeNS: u32 = 1 << GIC_CPU_CTRL_EOImodeNS_SHIFT;

pub const GICC_IAR_INT_ID_MASK: u32 = 0x3ff;
pub const GICC_INT_SPURIOUS: u32 = 1023;
pub const GICC_DIS_BYPASS_MASK: u32 = 0x1e0;

pub const GIC_DIST_CTRL: u32 = 0x000;
pub const GIC_DIST_CTR: u32 = 0x004;
pub const GIC_DIST_IIDR: u32 = 0x008;
pub const GIC_DIST_IGROUP: u32 = 0x080;
pub const GIC_DIST_ENABLE_SET: u32 = 0x100;
pub const GIC_DIST_ENABLE_CLEAR: u32 = 0x180;
pub const GIC_DIST_PENDING_SET: u32 = 0x200;
pub const GIC_DIST_PENDING_CLEAR: u32 = 0x280;
pub const GIC_DIST_ACTIVE_SET: u32 = 0x300;
pub const GIC_DIST_ACTIVE_CLEAR: u32 = 0x380;
pub const GIC_DIST_PRI: u32 = 0x400;
pub const GIC_DIST_TARGET: u32 = 0x800;
pub const GIC_DIST_CONFIG: u32 = 0xc00;
pub const GIC_DIST_SOFTINT: u32 = 0xf00;
pub const GIC_DIST_SGI_PENDING_CLEAR: u32 = 0xf10;
pub const GIC_DIST_SGI_PENDING_SET: u32 = 0xf20;

pub const GICD_ENABLE: u32 = 0x1;
pub const GICD_DISABLE: u32 = 0x0;
pub const GICD_INT_ACTLOW_LVLTRIG: u32 = 0x0;
pub const GICD_INT_EN_CLR_X32: u32 = 0xffffffff;
pub const GICD_INT_EN_SET_SGI: u32 = 0x0000ffff;
pub const GICD_INT_EN_CLR_PPI: u32 = 0xffff0000;

pub const GICD_IIDR_IMPLEMENTER_SHIFT: u32 = 0;
pub const GICD_IIDR_IMPLEMENTER_MASK: u32 = 0xfff << GICD_IIDR_IMPLEMENTER_SHIFT;
pub const GICD_IIDR_REVISION_SHIFT: u32 = 12;
pub const GICD_IIDR_REVISION_MASK: u32 = 0xf << GICD_IIDR_REVISION_SHIFT;
pub const GICD_IIDR_VARIANT_SHIFT: u32 = 16;
pub const GICD_IIDR_VARIANT_MASK: u32 = 0xf << GICD_IIDR_VARIANT_SHIFT;
pub const GICD_IIDR_PRODUCT_ID_SHIFT: u32 = 24;
pub const GICD_IIDR_PRODUCT_ID_MASK: u32 = 0xff << GICD_IIDR_PRODUCT_ID_SHIFT;

pub const GICH_HCR: u32 = 0x0;
pub const GICH_VTR: u32 = 0x4;
pub const GICH_VMCR: u32 = 0x8;
pub const GICH_MISR: u32 = 0x10;
pub const GICH_EISR0: u32 = 0x20;
pub const GICH_EISR1: u32 = 0x24;
pub const GICH_ELRSR0: u32 = 0x30;
pub const GICH_ELRSR1: u32 = 0x34;
pub const GICH_APR: u32 = 0xf0;
pub const GICH_LR0: u32 = 0x100;

pub const GICH_HCR_EN: u32 = 1 << 0;
pub const GICH_HCR_UIE: u32 = 1 << 1;
pub const GICH_HCR_LRENPIE: u32 = 1 << 2;
pub const GICH_HCR_NPIE: u32 = 1 << 3;
pub const GICH_HCR_VGrp0EIE: u32 = 1 << 4;
pub const GICH_HCR_VGrp0DIE: u32 = 1 << 5;
pub const GICH_HCR_VGrp1EIE: u32 = 1 << 6;
pub const GICH_HCR_VGrp1DIE: u32 = 1 << 7;
pub const GICH_HCR_EOICOUNT: u32 = 0xf8000000;

pub const GICH_LR_VIRTUALID: u32 = 0x3ff << 0;
pub const GICH_LR_PHYSID_CPUID_SHIFT: u32 = 10;
pub const GICH_LR_PHYSID_CPUID: u32 = 0x3ff << GICH_LR_PHYSID_CPUID_SHIFT;
pub const GICH_LR_PRIORITY_SHIFT: u32 = 23;
pub const GICH_LR_STATE: u32 = 3 << 28;
pub const GICH_LR_PENDING_BIT: u32 = 1 << 28;
pub const GICH_LR_ACTIVE_BIT: u32 = 1 << 29;
pub const GICH_LR_EOI: u32 = 1 << 19;
pub const GICH_LR_GROUP1: u32 = 1 << 30;
pub const GICH_LR_HW: u32 = 1 << 31;

pub const GICH_VMCR_ENABLE_GRP0_SHIFT: u32 = 0;
pub const GICH_VMCR_ENABLE_GRP0_MASK: u32 = 1 << GICH_VMCR_ENABLE_GRP0_SHIFT;
pub const GICH_VMCR_ENABLE_GRP1_SHIFT: u32 = 1;
pub const GICH_VMCR_ENABLE_GRP1_MASK: u32 = 1 << GICH_VMCR_ENABLE_GRP1_SHIFT;
pub const GICH_VMCR_ACK_CTL_SHIFT: u32 = 2;
pub const GICH_VMCR_ACK_CTL_MASK: u32 = 1 << GICH_VMCR_ACK_CTL_SHIFT;
pub const GICH_VMCR_FIQ_EN_SHIFT: u32 = 3;
pub const GICH_VMCR_FIQ_EN_MASK: u32 = 1 << GICH_VMCR_FIQ_EN_SHIFT;
pub const GICH_VMCR_CBPR_SHIFT: u32 = 4;
pub const GICH_VMCR_CBPR_MASK: u32 = 1 << GICH_VMCR_CBPR_SHIFT;
pub const GICH_VMCR_EOI_MODE_SHIFT: u32 = 9;
pub const GICH_VMCR_EOI_MODE_MASK: u32 = 1 << GICH_VMCR_EOI_MODE_SHIFT;
pub const GICH_VMCR_PRIMASK_SHIFT: u32 = 27;
pub const GICH_VMCR_PRIMASK_MASK: u32 = 0x1f << GICH_VMCR_PRIMASK_SHIFT;
pub const GICH_VMCR_BINPOINT_SHIFT: u32 = 21;
pub const GICH_VMCR_BINPOINT_MASK: u32 = 0x7 << GICH_VMCR_BINPOINT_SHIFT;
pub const GICH_VMCR_ALIAS_BINPOINT_SHIFT: u32 = 18;
pub const GICH_VMCR_ALIAS_BINPOINT_MASK: u32 = 0x7 << GICH_VMCR_ALIAS_BINPOINT_SHIFT;

pub const GICH_MISR_EOI: u32 = 1 << 0;
pub const GICH_MISR_U: u32 = 1 << 1;
pub const GICV_PMR_PRIORITY_SHIFT: u32 = 3;
pub const GICV_PMR_PRIORITY_MASK: u32 = 0x1f << GICV_PMR_PRIORITY_SHIFT;

// The following declarations correspond to the C header's !__ASSEMBLER__ section.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gic_chip_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn gic_cascade_irq(gic_nr: u32, irq: u32);
    pub fn gic_cpu_if_down(gic_nr: u32) -> i32;
    pub fn gic_cpu_save(gic: *mut gic_chip_data);
    pub fn gic_cpu_restore(gic: *mut gic_chip_data);
    pub fn gic_dist_save(gic: *mut gic_chip_data);
    pub fn gic_dist_restore(gic: *mut gic_chip_data);
    pub fn gic_of_init(node: *mut device_node, parent: *mut device_node) -> i32;
    pub fn gic_of_init_child(dev: *mut device, gic: *mut *mut gic_chip_data, irq: i32) -> i32;
    pub fn gic_send_sgi(cpu_id: u32, irq: u32);
    pub fn gic_get_cpu_id(cpu: u32) -> i32;
    pub fn gic_migrate_target(new_cpu_id: u32);
    pub fn gic_get_sgir_physaddr() -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
