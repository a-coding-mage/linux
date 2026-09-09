/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations guarded by __KERNEL__ in the original header. */

/*
 * The following structure is used to manage multiple PCI busses.
 */

pub struct pci_iommu_arena;
pub struct page;

/* A controller.  Used to manage multiple PCI busses.  */
#[repr(C)]
pub struct pci_controller {
    pub next: *mut pci_controller,
    pub bus: *mut pci_bus,
    pub io_space: *mut resource,
    pub mem_space: *mut resource,

    /* The following are for reporting to userland.  The invariant is
       that if we report a BWX-capable dense memory, we do not report
       a sparse memory at all, even if it exists.  */
    pub sparse_mem_base: ::core::ffi::c_ulong,
    pub dense_mem_base: ::core::ffi::c_ulong,
    pub sparse_io_base: ::core::ffi::c_ulong,
    pub dense_io_base: ::core::ffi::c_ulong,

    /* This one's for the kernel only.  It's in KSEG somewhere.  */
    pub config_space_base: ::core::ffi::c_ulong,

    pub index: ::core::ffi::c_uint,
    /* For compatibility with current (as of July 2003) pciutils
       and XFree86. Eventually will be removed. */
    pub need_domain_info: ::core::ffi::c_uint,

    pub sg_pci: *mut pci_iommu_arena,
    pub sg_isa: *mut pci_iommu_arena,

    pub sysdata: *mut ::core::ffi::c_void,
}

/* Override the logic in pci_scan_bus for skipping already-configured
   bus numbers.  */
#[inline]
pub const fn pcibios_assign_all_busses() -> ::core::ffi::c_int { 1 }

/* These correspond to alpha_mv.min_io_address and alpha_mv.min_mem_address. */
pub const PCIBIOS_MIN_IO: usize = 0; // alpha_mv.min_io_address
pub const PCIBIOS_MIN_MEM: usize = 0; // alpha_mv.min_mem_address

/* IOMMU controls.  */

#[inline]
pub unsafe fn pci_domain_nr(bus: *mut pci_bus) -> ::core::ffi::c_uint {
    (*((*bus).sysdata as *mut pci_controller)).index
}

#[inline]
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> ::core::ffi::c_int {
    let hose = (*bus).sysdata as *mut pci_controller;
    (*hose).need_domain_info as ::core::ffi::c_int
}

/* Values for the `which' argument to sys_pciconfig_iobase.  */
pub const IOBASE_HOSE: ::core::ffi::c_uint = 0;
pub const IOBASE_SPARSE_MEM: ::core::ffi::c_uint = 1;
pub const IOBASE_DENSE_MEM: ::core::ffi::c_uint = 2;
pub const IOBASE_SPARSE_IO: ::core::ffi::c_uint = 3;
pub const IOBASE_DENSE_IO: ::core::ffi::c_uint = 4;
pub const IOBASE_ROOT_BUS: ::core::ffi::c_uint = 5;
pub const IOBASE_FROM_HOSE: ::core::ffi::c_uint = 0x10000;

pub const HAVE_PCI_LEGACY: ::core::ffi::c_int = 1;

extern "C" {
    pub static mut isa_bridge: *mut pci_dev;

    pub fn pci_legacy_read(bus: *mut pci_bus, port: loff_t, val: *mut u32,
                           count: usize) -> ::core::ffi::c_int;
    pub fn pci_legacy_write(bus: *mut pci_bus, port: loff_t, val: u32,
                            count: usize) -> ::core::ffi::c_int;
    pub fn pci_mmap_legacy_page_range(bus: *mut pci_bus,
                                      vma: *mut vm_area_struct,
                                      mmap_state: pci_mmap_state)
        -> ::core::ffi::c_int;
    pub fn pci_legacy_has_sparse(bus: *mut pci_bus, type_: pci_mmap_state) -> bool;

    pub static pci_dev_resource_attr_group: attribute_group;
    pub static pci_dev_resource_sparse_attr_group: attribute_group;
    pub static pci_dev_resource_dense_attr_group: attribute_group;
}

/* ARCH_PCI_DEV_GROUPS expands to pointers to the three attribute groups. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
