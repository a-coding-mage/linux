/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from xen.h. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_domain_type {
    XEN_NATIVE,
    XEN_PV_DOMAIN,
    XEN_HVM_DOMAIN,
}

#[cfg(feature = "CONFIG_XEN")]
extern "C" {
    pub static mut xen_domain_type: xen_domain_type;
}

#[cfg(not(feature = "CONFIG_XEN"))]
pub const xen_domain_type_value: xen_domain_type = xen_domain_type::XEN_NATIVE;

#[cfg(feature = "CONFIG_XEN_PVH")]
extern "C" {
    pub static mut xen_pvh: bool;
}

#[cfg(not(feature = "CONFIG_XEN_PVH"))]
pub const xen_pvh: bool = false;

#[cfg(feature = "CONFIG_X86")]
#[inline]
pub unsafe fn xen_pv_domain() -> bool {
    // cpu_feature_enabled(X86_FEATURE_XENPV), supplied by the architecture.
    cpu_feature_enabled(X86_FEATURE_XENPV)
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub const fn xen_pv_domain() -> bool { false }

#[inline]
pub unsafe fn xen_domain() -> bool {
    #[cfg(feature = "CONFIG_XEN")]
    { xen_domain_type != xen_domain_type::XEN_NATIVE }
    #[cfg(not(feature = "CONFIG_XEN"))]
    { xen_domain_type_value != xen_domain_type::XEN_NATIVE }
}

#[inline]
pub unsafe fn xen_hvm_domain() -> bool {
    #[cfg(feature = "CONFIG_XEN")]
    { xen_domain_type == xen_domain_type::XEN_HVM_DOMAIN }
    #[cfg(not(feature = "CONFIG_XEN"))]
    { xen_domain_type_value == xen_domain_type::XEN_HVM_DOMAIN }
}

#[inline]
pub unsafe fn xen_pvh_domain() -> bool {
    #[cfg(feature = "CONFIG_XEN_PVH")]
    { xen_pvh }
    #[cfg(not(feature = "CONFIG_XEN_PVH"))]
    { xen_pvh }
}

extern "C" {
    pub static mut xen_start_flags: u32;
}

#[cfg(feature = "CONFIG_XEN_PV")]
extern "C" {
    pub static mut xen_pv_pci_possible: bool;
}

#[cfg(not(feature = "CONFIG_XEN_PV"))]
pub const xen_pv_pci_possible: bool = false;

#[repr(C)]
pub struct hvm_start_info { _private: [u8; 0] }
extern "C" {
    pub static mut pvh_start_info: hvm_start_info;
    pub fn xen_prepare_pvh();
}

#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }
extern "C" {
    pub fn xen_pv_evtchn_do_upcall(regs: *mut pt_regs);
}

#[cfg(feature = "CONFIG_XEN_DOM0")]
#[inline]
pub unsafe fn xen_initial_domain() -> bool {
    xen_domain() && (xen_start_flags & SIF_INITDOMAIN) != 0
}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub const fn xen_initial_domain() -> bool { false }

#[repr(C)]
pub struct bio_vec { _private: [u8; 0] }
#[repr(C)]
pub struct page { _private: [u8; 0] }
extern "C" {
    pub fn xen_biovec_phys_mergeable(vec1: *const bio_vec, page: *const page) -> bool;
}

#[cfg(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_XEN_BALLOON"))]
extern "C" {
    pub static mut xen_saved_max_mem_size: u64;
}

#[cfg(feature = "CONFIG_XEN_UNPOPULATED_ALLOC")]
extern "C" {
    pub static mut xen_unpopulated_pages: usize;
    pub fn xen_alloc_unpopulated_pages(nr_pages: u32, pages: *mut *mut page) -> i32;
    pub fn xen_free_unpopulated_pages(nr_pages: u32, pages: *mut *mut page);
    pub fn arch_xen_unpopulated_init(res: *mut *mut resource) -> i32;
}

#[cfg(not(feature = "CONFIG_XEN_UNPOPULATED_ALLOC"))]
pub const xen_unpopulated_pages: usize = 0;

#[cfg(not(feature = "CONFIG_XEN_UNPOPULATED_ALLOC"))]
#[inline]
pub unsafe fn xen_alloc_unpopulated_pages(nr_pages: u32, pages: *mut *mut page) -> i32 {
    xen_alloc_ballooned_pages(nr_pages, pages)
}

#[cfg(not(feature = "CONFIG_XEN_UNPOPULATED_ALLOC"))]
#[inline]
pub unsafe fn xen_free_unpopulated_pages(nr_pages: u32, pages: *mut *mut page) {
    xen_free_ballooned_pages(nr_pages, pages)
}

#[repr(C)]
pub struct resource { _private: [u8; 0] }

#[cfg(all(feature = "CONFIG_XEN_DOM0", feature = "CONFIG_ACPI", feature = "CONFIG_X86"))]
extern "C" {
    pub fn xen_processor_present(acpi_id: u32) -> bool;
}

#[cfg(not(all(feature = "CONFIG_XEN_DOM0", feature = "CONFIG_ACPI", feature = "CONFIG_X86")))]
#[inline]
pub unsafe fn xen_processor_present(_acpi_id: u32) -> bool {
    BUG();
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
