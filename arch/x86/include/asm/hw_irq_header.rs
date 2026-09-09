/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of x86/include/asm/hw_irq.h. */

/* The C header includes asm/irq_vectors.h, linux/percpu.h, linux/profile.h,
 * linux/smp.h, linux/atomic.h, asm/irq.h, and asm/sections.h. Their symbols
 * are supplied by the surrounding translation unit. */

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub struct msi_desc {
    _private: [u8; 0],
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum irq_alloc_type {
    X86_IRQ_ALLOC_TYPE_IOAPIC = 1,
    X86_IRQ_ALLOC_TYPE_HPET,
    X86_IRQ_ALLOC_TYPE_PCI_MSI,
    X86_IRQ_ALLOC_TYPE_PCI_MSIX,
    X86_IRQ_ALLOC_TYPE_DMAR,
    X86_IRQ_ALLOC_TYPE_AMDVI,
    X86_IRQ_ALLOC_TYPE_UV,
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub struct ioapic_alloc_info {
    pub pin: ::core::ffi::c_int,
    pub node: ::core::ffi::c_int,
    /* C bit-fields are represented by their underlying u32 storage. */
    pub is_level: u32,
    pub active_low: u32,
    pub valid: u32,
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub struct uv_alloc_info {
    pub limit: ::core::ffi::c_int,
    pub blade: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_ulong,
    pub name: *mut ::core::ffi::c_char,
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub union irq_alloc_info_data {
    pub ioapic: ioapic_alloc_info,
    pub uv: uv_alloc_info,
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub struct irq_alloc_info {
    pub type_: irq_alloc_type,
    pub flags: u32,
    pub devid: u32,
    pub hwirq: irq_hw_number_t,
    pub mask: *const cpumask,
    pub desc: *mut msi_desc,
    pub data: *mut ::core::ffi::c_void,
    pub data_union: irq_alloc_info_data,
}

#[cfg(feature = "irq_domain_hierarchy")]
#[repr(C)]
pub struct irq_cfg {
    pub dest_apicid: ::core::ffi::c_uint,
    pub vector: ::core::ffi::c_uint,
}

#[cfg(feature = "irq_domain_hierarchy")]
extern "C" {
    pub fn irq_cfg(irq: ::core::ffi::c_uint) -> *mut irq_cfg;
    pub fn irqd_cfg(irq_data: *mut irq_data) -> *mut irq_cfg;
    #[cfg(feature = "smp")]
    pub fn vector_schedule_cleanup(cfg: *mut irq_cfg);
    #[cfg(feature = "smp")]
    pub fn irq_complete_move(cfg: *mut irq_cfg);
    pub fn apic_ack_edge(data: *mut irq_data);
}

#[cfg(all(feature = "irq_domain_hierarchy", not(feature = "smp")))]
#[inline]
pub unsafe fn vector_schedule_cleanup(_c: *mut irq_cfg) {}

#[cfg(all(feature = "irq_domain_hierarchy", not(feature = "smp")))]
#[inline]
pub unsafe fn irq_complete_move(_c: *mut irq_cfg) {}

#[cfg(feature = "x86_local_apic")]
extern "C" {
    pub fn lock_vector_lock();
    pub fn unlock_vector_lock();
}

#[cfg(not(feature = "x86_local_apic"))]
#[inline]
pub unsafe fn lock_vector_lock() {}

#[cfg(not(feature = "x86_local_apic"))]
#[inline]
pub unsafe fn unlock_vector_lock() {}

extern "C" {
    pub fn elcr_set_level_irq(irq: ::core::ffi::c_uint);
    pub static mut irq_entries_start: ::core::ffi::c_char;
    pub static mut spurious_entries_start: ::core::ffi::c_char;
}

#[cfg(feature = "tracing")]
pub use irq_entries_start as trace_irq_entries_start;

/* VECTOR_UNUSED = NULL; VECTOR_SHUTDOWN = (void *)-1L;
 * VECTOR_RETRIGGERED = (void *)-2L. */
pub const VECTOR_UNUSED: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
pub const VECTOR_SHUTDOWN: *mut ::core::ffi::c_void = (-1isize) as *mut ::core::ffi::c_void;
pub const VECTOR_RETRIGGERED: *mut ::core::ffi::c_void = (-2isize) as *mut ::core::ffi::c_void;

/* NR_VECTORS, irq_hw_number_t, and cpumask are supplied by included headers. */
pub type vector_irq_t = [*mut irq_desc; NR_VECTORS];

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

/* DECLARE_PER_CPU(vector_irq_t, vector_irq); */
extern "C" {
    pub static mut vector_irq: vector_irq_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
