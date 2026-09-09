/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct pci_sysdata {
    pub domain: ::core::ffi::c_int, /* PCI domain */
    pub node: ::core::ffi::c_int,   /* NUMA node */
    #[cfg(feature = "CONFIG_ACPI")]
    pub companion: *mut acpi_device, /* ACPI companion device */
    #[cfg(feature = "CONFIG_X86_64")]
    pub iommu: *mut ::core::ffi::c_void, /* IOMMU private data */
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub fwnode: *mut ::core::ffi::c_void, /* IRQ domain for MSI assignment */
    #[cfg(feature = "CONFIG_VMD")]
    pub vmd_dev: *mut pci_dev, /* VMD Device if in Intel VMD domain */
}

extern "C" {
    pub static mut pci_routeirq: ::core::ffi::c_int;
    pub static mut noioapicquirk: ::core::ffi::c_int;
    pub static mut noioapicreroute: ::core::ffi::c_int;
}

#[inline]
pub unsafe fn to_pci_sysdata(bus: *const pci_bus) -> *mut pci_sysdata {
    (*bus).sysdata
}

#[cfg(feature = "CONFIG_PCI")]
#[cfg(feature = "CONFIG_PCI_DOMAINS")]
#[inline]
pub unsafe fn pci_domain_nr(bus: *mut pci_bus) -> ::core::ffi::c_int {
    (*to_pci_sysdata(bus)).domain
}

#[cfg(feature = "CONFIG_PCI")]
#[cfg(feature = "CONFIG_PCI_DOMAINS")]
#[inline]
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> ::core::ffi::c_int {
    pci_domain_nr(bus)
}

#[cfg(feature = "CONFIG_PCI")]
#[cfg(feature = "CONFIG_PCI_MSI")]
#[inline]
pub unsafe fn _pci_root_bus_fwnode(bus: *mut pci_bus) -> *mut ::core::ffi::c_void {
    (*to_pci_sysdata(bus)).fwnode
}

#[cfg(feature = "CONFIG_PCI")]
#[cfg(feature = "CONFIG_VMD")]
#[inline]
pub unsafe fn is_vmd(bus: *mut pci_bus) -> bool {
    (*to_pci_sysdata(bus)).vmd_dev != ::core::ptr::null_mut()
}

#[cfg(feature = "CONFIG_PCI")]
#[cfg(not(feature = "CONFIG_VMD"))]
#[inline]
pub fn is_vmd(_bus: *mut pci_bus) -> bool {
    false
}

#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn pcibios_assign_all_busses() -> ::core::ffi::c_uint;
    pub fn pci_legacy_init() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub fn pcibios_assign_all_busses() -> ::core::ffi::c_uint { 0 }

extern "C" {
    pub static mut pci_mem_start: ::core::ffi::c_ulong;
}

pub const PCIBIOS_MIN_IO: ::core::ffi::c_ulong = 0x1000;
/* C macro: (pci_mem_start). Read the external object at the point of use. */
#[inline]
pub unsafe fn PCIBIOS_MIN_MEM() -> ::core::ffi::c_ulong { pci_mem_start }

pub const PCIBIOS_MIN_CARDBUS_IO: ::core::ffi::c_ulong = 0x4000;

extern "C" {
    pub static mut pcibios_enabled: ::core::ffi::c_int;
    pub fn pcibios_scan_root(bus: ::core::ffi::c_int);
    pub fn pcibios_get_irq_routing_table() -> *mut irq_routing_table;
    pub fn pcibios_set_irq_routing(dev: *mut pci_dev, pin: ::core::ffi::c_int,
                                   irq: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn pci_dev_has_default_msi_parent_domain(dev: *mut pci_dev) -> bool;
    pub fn pat_enabled() -> bool;
    pub fn pci_iommu_alloc();
}

/* #define HAVE_PCI_MMAP */
/* #define ARCH_GENERIC_PCI_MMAP_RESOURCE */
#[inline]
pub unsafe fn arch_can_pci_mmap_wc() -> bool { pat_enabled() }

#[cfg(feature = "CONFIG_PCI")]
extern "C" { pub fn early_quirks(); }

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub fn early_quirks() {}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn __pcibus_to_node(bus: *const pci_bus) -> ::core::ffi::c_int {
    (*to_pci_sysdata(bus)).node
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn cpumask_of_pcibus(bus: *const pci_bus) -> *const cpumask {
    let node = __pcibus_to_node(bus);
    if node == NUMA_NO_NODE { cpu_online_mask } else { cpumask_of_node(node) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
