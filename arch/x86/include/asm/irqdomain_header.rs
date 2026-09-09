/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/irqdomain.h, asm/hw_irq.h

// CONFIG_X86_LOCAL_APIC
pub const X86_IRQ_ALLOC_LEGACY: u32 = 0x1;

extern "C" {
    pub fn x86_fwspec_is_ioapic(fwspec: *mut irq_fwspec) -> ::core::ffi::c_int;
    pub fn x86_fwspec_is_hpet(fwspec: *mut irq_fwspec) -> ::core::ffi::c_int;

    pub static mut x86_vector_domain: *mut irq_domain;

    pub fn init_irq_alloc_info(info: *mut irq_alloc_info, mask: *const cpumask);
    pub fn copy_irq_alloc_info(dst: *mut irq_alloc_info, src: *mut irq_alloc_info);
}

// CONFIG_X86_IO_APIC
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ioapic_domain_type {
    IOAPIC_DOMAIN_INVALID,
    IOAPIC_DOMAIN_LEGACY,
    IOAPIC_DOMAIN_STRICT,
    IOAPIC_DOMAIN_DYNAMIC,
}

#[repr(C)]
pub struct ioapic_domain_cfg {
    pub r#type: ioapic_domain_type,
    pub ops: *const irq_domain_ops,
    pub dev: *mut device_node,
}

extern "C" {
    pub static mp_ioapic_irqdomain_ops: irq_domain_ops;

    pub fn mp_irqdomain_alloc(
        domain: *mut irq_domain,
        virq: ::core::ffi::c_uint,
        nr_irqs: ::core::ffi::c_uint,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn mp_irqdomain_free(
        domain: *mut irq_domain,
        virq: ::core::ffi::c_uint,
        nr_irqs: ::core::ffi::c_uint,
    );
    pub fn mp_irqdomain_activate(
        domain: *mut irq_domain,
        irq_data: *mut irq_data,
        reserve: bool,
    ) -> ::core::ffi::c_int;
    pub fn mp_irqdomain_deactivate(domain: *mut irq_domain, irq_data: *mut irq_data);
    pub fn mp_irqdomain_ioapic_idx(domain: *mut irq_domain) -> ::core::ffi::c_int;
}

// CONFIG_PCI_MSI
extern "C" {
    pub fn x86_create_pci_msi_domain();
    pub fn native_create_pci_msi_domain() -> *mut irq_domain;
    pub static mut x86_pci_msi_default_domain: *mut irq_domain;
}

// Without CONFIG_PCI_MSI, the C header provides:
// static inline void x86_create_pci_msi_domain(void) { }
// #define native_create_pci_msi_domain NULL
// #define x86_pci_msi_default_domain NULL


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
