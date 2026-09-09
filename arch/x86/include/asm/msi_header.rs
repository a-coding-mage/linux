/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/hw_irq.h, asm/irqdomain.h

pub type msi_alloc_info_t = irq_alloc_info;

extern "C" {
    pub fn pci_msi_prepare(
        domain: *mut irq_domain,
        dev: *mut device,
        nvec: ::core::ffi::c_int,
        arg: *mut msi_alloc_info_t,
    ) -> ::core::ffi::c_int;
}

/* Structs and defines for the X86 specific MSI message format */

/*
 * The anonymous bit-field member occupies one u32:
 * vector:8, delivery_mode:3, dest_mode_logical:1, reserved:2,
 * active_low:1, is_level:1.
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct x86_msi_data_bits {
    pub bits: u32,
}

#[repr(C)]
pub union x86_msi_data {
    pub bits: u32,
    pub dmar_subhandle: u32,
}

pub type arch_msi_msg_data_t = x86_msi_data;

/*
 * The first anonymous bit-field member occupies one u32:
 * reserved_0:2, dest_mode_logical:1, redirect_hint:1, reserved_1:1,
 * virt_destid_8_14:7, destid_0_7:8, base_address:12.
 *
 * The second anonymous bit-field member occupies one u32:
 * dmar_reserved_0:2, dmar_index_15:1, dmar_subhandle_valid:1,
 * dmar_format:1, dmar_index_0_14:15, dmar_base_address:12.
 */
#[repr(C)]
pub union x86_msi_addr_lo {
    pub bits: u32,
    pub dmar_bits: u32,
}

pub type arch_msi_msg_addr_lo_t = x86_msi_addr_lo;

pub const X86_MSI_BASE_ADDRESS_LOW: u32 = 0xfee00000u32 >> 20;

/* reserved:8, destid_8_31:24 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct x86_msi_addr_hi {
    pub bits: u32,
}

pub type arch_msi_msg_addr_hi_t = x86_msi_addr_hi;

pub const X86_MSI_BASE_ADDRESS_HIGH: u32 = 0;

pub struct msi_msg;

extern "C" {
    pub fn x86_msi_msg_get_destid(msg: *mut msi_msg, extid: bool) -> u32;
}

pub const X86_VECTOR_MSI_FLAGS_SUPPORTED: u32 =
    MSI_GENERIC_FLAGS_MASK | MSI_FLAG_PCI_MSIX | MSI_FLAG_PCI_MSIX_ALLOC_DYN;

pub const X86_VECTOR_MSI_FLAGS_REQUIRED: u32 =
    MSI_FLAG_USE_DEF_DOM_OPS | MSI_FLAG_USE_DEF_CHIP_OPS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
