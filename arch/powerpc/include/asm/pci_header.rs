/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Translated from the PowerPC PCI header.  C includes and build-time
 * configuration are supplied by the surrounding kernel translation. */

/* Return values for pci_controller_ops.probe_mode function. */
pub const PCI_PROBE_NONE: i32 = -1; // Don't look at this bus at all
pub const PCI_PROBE_NORMAL: i32 = 0; // Do normal PCI probing
pub const PCI_PROBE_DEVTREE: i32 = 1; // Instantiate from device tree

pub const PCIBIOS_MIN_IO: u64 = 0x1000;
pub const PCIBIOS_MIN_MEM: u64 = 0x10000000;

/* Values for the `which` argument to sys_pciconfig_iobase syscall. */
pub const IOBASE_BRIDGE_NUMBER: i32 = 0;
pub const IOBASE_MEMORY: i32 = 1;
pub const IOBASE_IO: i32 = 2;
pub const IOBASE_ISA_IO: i32 = 3;
pub const IOBASE_ISA_MEM: i32 = 4;

/* Set this to 1 if the kernel should re-assign all PCI bus numbers. */
#[inline]
pub unsafe fn pcibios_assign_all_busses() -> bool {
    pci_has_flag(PCI_REASSIGN_ALL_BUS)
}

#[inline]
pub unsafe fn pci_get_legacy_ide_irq(dev: *mut pci_dev, channel: i32) -> i32 {
    if let Some(callback) = ppc_md.pci_get_legacy_ide_irq {
        return callback(dev, channel);
    }
    if channel != 0 { 15 } else { 14 }
}

/* Under CONFIG_PCI this is an external initialization function; otherwise
 * the C macro expands to nothing. */
#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn set_pci_dma_ops(dma_ops: *const dma_map_ops);
}

/* CONFIG_PPC64 defines PCI_DISABLE_MWI. */
#[cfg(feature = "CONFIG_PPC64")]
pub const PCI_DISABLE_MWI: bool = true;

extern "C" {
    pub fn pci_domain_nr(bus: *mut pci_bus) -> i32;
    pub fn pci_proc_domain(bus: *mut pci_bus) -> i32;

    pub fn pci_legacy_read(
        bus: *mut pci_bus, port: loff_t, val: *mut u32, count: usize,
    ) -> i32;
    pub fn pci_legacy_write(
        bus: *mut pci_bus, port: loff_t, val: u32, count: usize,
    ) -> i32;
    pub fn pci_mmap_legacy_page_range(
        bus: *mut pci_bus, vma: *mut vm_area_struct, mmap_state: pci_mmap_state,
    ) -> i32;

    pub fn pcibios_claim_one_bus(b: *mut pci_bus);
    pub fn pcibios_finish_adding_to_bus(bus: *mut pci_bus);
    pub fn pcibios_resource_survey();

    pub fn init_phb_dynamic(dn: *mut device_node) -> *mut pci_controller;
    pub fn remove_phb_dynamic(phb: *mut pci_controller) -> i32;
    pub fn of_create_pci_dev(
        node: *mut device_node, bus: *mut pci_bus, devfn: i32,
    ) -> *mut pci_dev;
    pub fn pci_parse_of_flags(addr0: u32, bridge: i32) -> u32;
    pub fn of_scan_pci_bridge(dev: *mut pci_dev);
    pub fn of_scan_bus(node: *mut device_node, bus: *mut pci_bus);
    pub fn of_rescan_bus(node: *mut device_node, bus: *mut pci_bus);

    pub fn pci_phys_mem_access_prot(
        pfn: c_ulong, size: c_ulong, prot: pgprot_t,
    ) -> pgprot_t;
    pub fn pcibios_io_space_offset(hose: *mut pci_controller) -> resource_size_t;
    pub fn pcibios_setup_bus_self(bus: *mut pci_bus);
    pub fn pcibios_setup_phb_io_space(hose: *mut pci_controller);
    pub fn pcibios_scan_phb(hose: *mut pci_controller);
}

pub const HAVE_PCI_MMAP: i32 = 1;
pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: i32 = 1;

#[inline]
pub const fn arch_can_pci_mmap_io() -> i32 { 1 }
#[inline]
pub const fn arch_can_pci_mmap_wc() -> i32 { 1 }

pub const HAVE_PCI_LEGACY: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
