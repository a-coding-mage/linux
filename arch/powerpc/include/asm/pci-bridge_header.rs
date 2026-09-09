/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from pci-bridge.h. C preprocessor conditions are preserved in comments. */

use core::ffi::c_void;

#[repr(C)]
pub struct pci_controller_ops {
    pub dma_dev_setup: Option<unsafe extern "C" fn(pdev: *mut pci_dev)>,
    pub dma_bus_setup: Option<unsafe extern "C" fn(bus: *mut pci_bus)>,
    pub iommu_bypass_supported: Option<unsafe extern "C" fn(pdev: *mut pci_dev, mask: u64) -> bool>,
    pub probe_mode: Option<unsafe extern "C" fn(bus: *mut pci_bus) -> i32>,
    pub enable_device_hook: Option<unsafe extern "C" fn(pdev: *mut pci_dev) -> bool>,
    pub disable_device: Option<unsafe extern "C" fn(pdev: *mut pci_dev)>,
    pub release_device: Option<unsafe extern "C" fn(pdev: *mut pci_dev)>,
    pub window_alignment: Option<unsafe extern "C" fn(bus: *mut pci_bus, type_: c_ulong) -> resource_size_t>,
    pub setup_bridge: Option<unsafe extern "C" fn(bus: *mut pci_bus, type_: c_ulong)>,
    pub reset_secondary_bus: Option<unsafe extern "C" fn(pdev: *mut pci_dev)>,
    /* CONFIG_PCI_MSI: */
    pub setup_msi_irqs: Option<unsafe extern "C" fn(pdev: *mut pci_dev, nvec: i32, type_: i32) -> i32>,
    pub teardown_msi_irqs: Option<unsafe extern "C" fn(pdev: *mut pci_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(hose: *mut pci_controller)>,
    pub device_group: Option<unsafe extern "C" fn(hose: *mut pci_controller, pdev: *mut pci_dev) -> *mut iommu_group>,
}

#[repr(C)]
pub struct pci_controller {
    pub bus: *mut pci_bus,
    pub is_dynamic: i8,
    /* CONFIG_PPC64: */
    pub node: i32,
    pub dn: *mut device_node,
    pub list_node: list_head,
    pub parent: *mut device,
    pub first_busno: i32,
    pub last_busno: i32,
    pub self_busno: i32,
    pub busn: resource,
    pub io_base_virt: *mut c_void,
    /* CONFIG_PPC64: */
    pub io_base_alloc: *mut c_void,
    pub io_base_phys: resource_size_t,
    pub pci_io_size: resource_size_t,
    pub isa_mem_phys: resource_size_t,
    pub isa_mem_size: resource_size_t,
    pub controller_ops: pci_controller_ops,
    pub ops: *mut pci_ops,
    pub cfg_addr: *mut u32,
    pub cfg_data: *mut c_void,
    pub indirect_type: u32,
    pub io_resource: resource,
    pub mem_resources: [resource; 3],
    pub mem_offset: [resource_size_t; 3],
    pub global_number: i32,
    pub dma_window_base_cur: resource_size_t,
    pub dma_window_size: resource_size_t,
    /* CONFIG_PPC64: */
    pub buid: c_ulong,
    pub pci_data: *mut pci_dn,
    pub private_data: *mut c_void,
    pub dev_domain: *mut irq_domain,
    pub iommu: iommu_device,
}

pub const PPC_INDIRECT_TYPE_SET_CFG_TYPE: u32 = 0x00000001;
pub const PPC_INDIRECT_TYPE_EXT_REG: u32 = 0x00000002;
pub const PPC_INDIRECT_TYPE_SURPRESS_PRIMARY_BUS: u32 = 0x00000004;
pub const PPC_INDIRECT_TYPE_NO_PCIE_LINK: u32 = 0x00000008;
pub const PPC_INDIRECT_TYPE_BIG_ENDIAN: u32 = 0x00000010;
pub const PPC_INDIRECT_TYPE_BROKEN_MRM: u32 = 0x00000020;
pub const PPC_INDIRECT_TYPE_FSL_CFG_REG_LINK: u32 = 0x00000040;

extern "C" {
    pub fn early_read_config_byte(hose: *mut pci_controller, bus: i32, dev_fn: i32, where_: i32, val: *mut u8) -> i32;
    pub fn early_read_config_word(hose: *mut pci_controller, bus: i32, dev_fn: i32, where_: i32, val: *mut u16) -> i32;
    pub fn early_read_config_dword(hose: *mut pci_controller, bus: i32, dev_fn: i32, where_: i32, val: *mut u32) -> i32;
    pub fn early_write_config_byte(hose: *mut pci_controller, bus: i32, dev_fn: i32, where_: i32, val: u8) -> i32;
    pub fn early_write_config_word(hose: *mut pci_controller, bus: i32, dev_fn: i32, where_: i32, val: u16) -> i32;
    pub fn early_write_config_dword(hose: *mut pci_controller, bus: i32, dev_fn: i32, where_: i32, val: u32) -> i32;
    pub fn early_find_capability(hose: *mut pci_controller, bus: i32, dev_fn: i32, cap: i32) -> i32;
    pub fn setup_indirect_pci(hose: *mut pci_controller, cfg_addr: resource_size_t, cfg_data: resource_size_t, flags: u32);
    pub fn indirect_read_config(bus: *mut pci_bus, devfn: u32, offset: i32, len: i32, val: *mut u32) -> i32;
    pub fn __indirect_read_config(hose: *mut pci_controller, bus_number: u8, devfn: u32, offset: i32, len: i32, val: *mut u32) -> i32;
    pub fn indirect_write_config(bus: *mut pci_bus, devfn: u32, offset: i32, len: i32, val: u32) -> i32;
}

#[inline]
pub unsafe fn pci_bus_to_host(bus: *const pci_bus) -> *mut pci_controller {
    (*bus).sysdata as *mut pci_controller
}

/* CONFIG_PPC64 declarations and pci_dn layout. */
#[repr(C)]
pub struct pci_dn {
    pub flags: i32,
    pub busno: i32,
    pub devfn: i32,
    pub vendor_id: i32,
    pub device_id: i32,
    pub class_code: i32,
    pub parent: *mut pci_dn,
    pub phb: *mut pci_controller,
    pub table_group: *mut iommu_table_group,
    pub pci_ext_config_space: i32,
    pub edev: *mut eeh_dev,
    pub pe_number: u32,
    pub vfs_expanded: u16,
    pub num_vfs: u16,
    pub pe_num_map: *mut u32,
    pub m64_single_mode: bool,
    pub m64_map: *mut [i32; PCI_SRIOV_NUM_BARS],
    pub last_allow_rc: i32,
    pub mps: i32,
    pub child_list: list_head,
    pub list: list_head,
    pub holes: [resource; PCI_SRIOV_NUM_BARS],
}

pub const PCI_DN_FLAG_IOV_VF: i32 = 0x01;
pub const PCI_DN_FLAG_DEAD: i32 = 0x02;
pub const IODA_INVALID_PE: u32 = 0xFFFFFFFF;
pub const IODA_INVALID_M64: i32 = -1;

#[inline]
pub unsafe fn PCI_DN(dn: *mut device_node) -> *mut pci_dn {
    (*(dn)).data as *mut pci_dn
}

extern "C" {
    pub fn pci_get_pdn_by_devfn(bus: *mut pci_bus, devfn: i32) -> *mut pci_dn;
    pub fn pci_get_pdn(pdev: *mut pci_dev) -> *mut pci_dn;
    pub fn pci_add_device_node_info(hose: *mut pci_controller, dn: *mut device_node) -> *mut pci_dn;
    pub fn pci_remove_device_node_info(dn: *mut device_node);
    pub fn pci_find_bus_by_node(dn: *mut device_node) -> *mut pci_bus;
    pub fn pci_hp_remove_devices(bus: *mut pci_bus);
    pub fn pci_hp_add_devices(bus: *mut pci_bus);
    pub fn pcibios_unmap_io_space(bus: *mut pci_bus) -> i32;
    pub fn pcibios_map_io_space(bus: *mut pci_bus) -> i32;
    pub fn pci_find_hose_for_OF_device(node: *mut device_node) -> *mut pci_controller;
    pub fn pci_find_controller_for_domain(domain_nr: i32) -> *mut pci_controller;
    pub fn pci_process_bridge_OF_ranges(hose: *mut pci_controller, dev: *mut device_node, primary: i32);
    pub fn pcibios_alloc_controller(dev: *mut device_node) -> *mut pci_controller;
    pub fn pcibios_free_controller(phb: *mut pci_controller);
    pub fn pcibios_free_controller_deferred(bridge: *mut pci_host_bridge);
    pub fn pcibios_vaddr_is_ioport(address: *mut c_void) -> i32;
}

pub type resource_size_t = u64;
pub type c_ulong = u64;
pub const PCI_SRIOV_NUM_BARS: usize = 6;

/* External types supplied by the included kernel headers. */
extern "C" {
    type pci_dev; type pci_bus; type device_node; type list_head; type device; type resource;
    type pci_ops; type irq_domain; type iommu_device; type iommu_group; type iommu_table_group;
    type eeh_dev; type pci_host_bridge; type iommu_table;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
