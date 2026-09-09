/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Advanced Micro Devices, Inc.
 * Author: Joerg Roedel <joerg.roedel@amd.com>
 *
 * This header file contains the interface of the interrupt remapping code to
 * the x86 interrupt management code.
 */

// Dependencies supplied by the surrounding translation unit:
// asm/irqdomain.h, asm/hw_irq.h, asm/io_apic.h

use core::ffi::c_char;

pub struct MsiMsg;
pub struct IrqAllocInfo;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum IrqRemapCap {
    IRQ_POSTING_CAP = 0,
}

pub const IRQ_REMAP_XAPIC_MODE: u32 = 0;
pub const IRQ_REMAP_X2APIC_MODE: u32 = 1;

/*
 * This is mainly used to communicate information back-and-forth
 * between SVM and IOMMU for setting up and tearing down posted
 * interrupt
 */
#[repr(C)]
pub struct AmdIommuPiData {
    pub vapic_addr: u64,       /* Physical address of the vCPU's vAPIC. */
    pub ga_tag: u32,
    pub vector: u32,           /* Guest vector of the interrupt */
    pub cpu: core::ffi::c_int,
    pub ga_log_intr: bool,
    pub is_guest_mode: bool,
    pub ir_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct IntelIommuPiData {
    pub pi_desc_addr: u64,     /* Physical address of PI Descriptor */
    pub vector: u32,           /* Guest vector of the interrupt */
}

/* CONFIG_IRQ_REMAP */
#[cfg(feature = "CONFIG_IRQ_REMAP")]
extern "C" {
    pub static mut irq_2_ir_lock: raw_spinlock_t;

    pub fn irq_remapping_cap(cap: IrqRemapCap) -> bool;
    pub fn set_irq_remapping_broken();
    pub fn irq_remapping_prepare() -> core::ffi::c_int;
    pub fn irq_remapping_enable() -> core::ffi::c_int;
    pub fn irq_remapping_disable();
    pub fn irq_remapping_reenable(arg: core::ffi::c_int) -> core::ffi::c_int;
    pub fn irq_remap_enable_fault_handling() -> core::ffi::c_int;
    pub fn panic_if_irq_remap(msg: *const c_char);

    pub static mut enable_posted_msi: bool;
}

#[cfg(feature = "CONFIG_IRQ_REMAP")]
pub unsafe fn arch_get_ir_parent_domain() -> *mut irq_domain {
    x86_vector_domain
}

#[cfg(feature = "CONFIG_IRQ_REMAP")]
pub unsafe fn posted_msi_enabled() -> bool {
    cfg!(feature = "CONFIG_X86_POSTED_MSI")
        && enable_posted_msi
        && irq_remapping_cap(IrqRemapCap::IRQ_POSTING_CAP)
}

#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn irq_remapping_cap(_cap: IrqRemapCap) -> bool { false }
#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn set_irq_remapping_broken() {}
#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn irq_remapping_prepare() -> core::ffi::c_int { -ENODEV }
#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn irq_remapping_enable() -> core::ffi::c_int { -ENODEV }
#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn irq_remapping_disable() {}
#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn irq_remapping_reenable(_eim: core::ffi::c_int) -> core::ffi::c_int { -ENODEV }
#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn irq_remap_enable_fault_handling() -> core::ffi::c_int { -ENODEV }
#[cfg(not(feature = "CONFIG_IRQ_REMAP"))]
pub unsafe fn panic_if_irq_remap(_msg: *const c_char) {}

/* CONFIG_X86_POSTED_MSI */
#[cfg(feature = "CONFIG_X86_POSTED_MSI")]
extern "C" {
    pub fn intel_ack_posted_msi_irq(irqd: *mut irq_data);
}
#[cfg(not(feature = "CONFIG_X86_POSTED_MSI"))]
pub const intel_ack_posted_msi_irq: Option<unsafe extern "C" fn(*mut irq_data)> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
