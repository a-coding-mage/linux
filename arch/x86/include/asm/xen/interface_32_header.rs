/* SPDX-License-Identifier: GPL-2.0 */
/******************************************************************************
 * arch-x86_32.h
 *
 * Guest OS interface to x86 32-bit Xen.
 *
 * Copyright (c) 2004, K A Fraser
 */

/*
 * These flat segments are in the Xen-private section of every GDT. Since these
 * are also present in the initial GDT, many OSes will be able to avoid
 * installing their own GDT.
 */
pub const FLAT_RING1_CS: u32 = 0xe019; /* GDT index 259 */
pub const FLAT_RING1_DS: u32 = 0xe021; /* GDT index 260 */
pub const FLAT_RING1_SS: u32 = 0xe021; /* GDT index 260 */
pub const FLAT_RING3_CS: u32 = 0xe02b; /* GDT index 261 */
pub const FLAT_RING3_DS: u32 = 0xe033; /* GDT index 262 */
pub const FLAT_RING3_SS: u32 = 0xe033; /* GDT index 262 */

pub const FLAT_KERNEL_CS: u32 = FLAT_RING1_CS;
pub const FLAT_KERNEL_DS: u32 = FLAT_RING1_DS;
pub const FLAT_KERNEL_SS: u32 = FLAT_RING1_SS;
pub const FLAT_USER_CS: u32 = FLAT_RING3_CS;
pub const FLAT_USER_DS: u32 = FLAT_RING3_DS;
pub const FLAT_USER_SS: u32 = FLAT_RING3_SS;

/* And the trap vector is... */
pub const TRAP_INSTR: &str = "int $0x82";

pub const __MACH2PHYS_VIRT_START: u32 = 0xF5800000;
pub const __MACH2PHYS_VIRT_END: u32 = 0xF6800000;
pub const __MACH2PHYS_SHIFT: u32 = 2;

/*
 * Virtual addresses beyond this are not modifiable by guest OSes. The
 * machine->physical mapping table starts at this address, read-only.
 */
pub const __HYPERVISOR_VIRT_START: u32 = 0xF5800000;

#[repr(C)]
pub struct cpu_user_regs {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub eax: u32,
    pub error_code: u16,    /* private */
    pub entry_vector: u16,  /* private */
    pub eip: u32,
    pub cs: u16,
    pub saved_upcall_mask: u8,
    pub _pad0: u8,
    pub eflags: u32,        /* eflags.IF == !saved_upcall_mask */
    pub esp: u32,
    pub ss: u16,
    pub _pad1: u16,
    pub es: u16,
    pub _pad2: u16,
    pub ds: u16,
    pub _pad3: u16,
    pub fs: u16,
    pub _pad4: u16,
    pub gs: u16,
    pub _pad5: u16,
}

/* DEFINE_GUEST_HANDLE_STRUCT(cpu_user_regs); */

pub type tsc_timestamp_t = u64; /* RDTSC timestamp */

#[repr(C)]
pub struct arch_vcpu_info {
    pub cr2: ::core::ffi::c_ulong,
    pub pad: [::core::ffi::c_ulong; 5], /* sizeof(struct vcpu_info) == 64 */
}

#[repr(C)]
pub struct xen_callback {
    pub cs: ::core::ffi::c_ulong,
    pub eip: ::core::ffi::c_ulong,
}
pub type xen_callback_t = xen_callback;

#[inline]
pub const fn XEN_CALLBACK(__cs: ::core::ffi::c_ulong, __eip: ::core::ffi::c_ulong) -> xen_callback {
    xen_callback { cs: __cs, eip: __eip }
}

/*
 * Page-directory addresses above 4GB do not fit into architectural %cr3.
 * When accessing %cr3, or equivalent field in vcpu_guest_context, guests
 * must use the following accessor macros to pack/unpack valid MFNs.
 *
 * Note that Xen is using the fact that the pagetable base is always
 * page-aligned, and putting the 12 MSB of the address into the 12 LSB
 * of cr3.
 */
#[inline]
pub const fn xen_pfn_to_cr3(pfn: u32) -> u32 {
    (pfn << 12) | (pfn >> 20)
}

#[inline]
pub const fn xen_cr3_to_pfn(cr3: u32) -> u32 {
    (cr3 >> 12) | (cr3 << 20)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
