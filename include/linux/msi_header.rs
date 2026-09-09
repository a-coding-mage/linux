/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/msi.h. Types supplied by the surrounding kernel are
// intentionally referenced but not defined here.

#[repr(C, packed)]
pub struct arch_msi_msg_addr_lo_t { pub address_lo: u32 }
#[repr(C, packed)]
pub struct arch_msi_msg_addr_hi_t { pub address_hi: u32 }
#[repr(C, packed)]
pub struct arch_msi_msg_data_t { pub data: u32 }

#[repr(C)]
pub union msi_msg {
    pub address_lo: u32,
    pub arch_addr_lo: arch_msi_msg_addr_lo_t,
    pub address_hi: u32,
    pub arch_addr_hi: arch_msi_msg_addr_hi_t,
    pub data: u32,
    pub arch_data: arch_msi_msg_data_t,
}

#[repr(C)] pub struct msi_desc { pub irq: core::ffi::c_uint, pub nvec_used: core::ffi::c_uint, pub dev: *mut device, pub msg: msi_msg, pub affinity: *mut irq_affinity_desc, pub write_msi_msg: Option<unsafe extern "C" fn(*mut msi_desc, *mut core::ffi::c_void)>, pub write_msi_msg_data: *mut core::ffi::c_void, pub msi_index: u16, pub data: msi_desc_data }
#[repr(C)] pub struct pci_msi_desc { pub msi_mask: u32, pub msi_attrib: msi_attrib, pub mask: pci_msi_mask }
#[repr(C)] pub struct msi_attrib { pub is_msix: u8, pub multiple: u8, pub multi_cap: u8, pub can_mask: u8, pub is_64: u8, pub is_virtual: u8, pub default_irq: core::ffi::c_uint }
#[repr(C)] pub union pci_msi_mask { pub mask_pos: u8, pub mask_base: *mut core::ffi::c_void }
#[repr(C)] pub union msi_domain_cookie { pub value: u64, pub ptr: *mut core::ffi::c_void, pub iobase: *mut core::ffi::c_void }
#[repr(C)] pub struct msi_desc_data { pub dcookie: msi_domain_cookie, pub icookie: msi_instance_cookie }

#[repr(C)] pub struct msi_dev_domain { pub store: xarray, pub domain: *mut irq_domain }

#[repr(C)] pub struct msi_domain_ops {
    pub get_hwirq: Option<unsafe extern "C" fn(*mut msi_domain_info, *mut msi_alloc_info_t) -> irq_hw_number_t>,
    pub msi_init: Option<unsafe extern "C" fn(*mut irq_domain, *mut msi_domain_info, core::ffi::c_uint, irq_hw_number_t, *mut msi_alloc_info_t) -> i32>,
    pub msi_free: Option<unsafe extern "C" fn(*mut irq_domain, *mut msi_domain_info, core::ffi::c_uint)>,
    pub msi_prepare: Option<unsafe extern "C" fn(*mut irq_domain, *mut device, i32, *mut msi_alloc_info_t) -> i32>,
    pub msi_teardown: Option<unsafe extern "C" fn(*mut irq_domain, *mut msi_alloc_info_t)>,
    pub prepare_desc: Option<unsafe extern "C" fn(*mut irq_domain, *mut msi_alloc_info_t, *mut msi_desc)>,
    pub set_desc: Option<unsafe extern "C" fn(*mut msi_alloc_info_t, *mut msi_desc)>,
    pub domain_alloc_irqs: Option<unsafe extern "C" fn(*mut irq_domain, *mut device, i32) -> i32>,
    pub domain_free_irqs: Option<unsafe extern "C" fn(*mut irq_domain, *mut device)>,
    pub msi_translate: Option<unsafe extern "C" fn(*mut irq_domain, *mut irq_fwspec, *mut irq_hw_number_t, *mut core::ffi::c_uint) -> i32>,
}
#[repr(C)] pub struct msi_domain_info { pub flags: u32, pub bus_token: irq_domain_bus_token, pub hwsize: core::ffi::c_uint, pub ops: *mut msi_domain_ops, pub dev: *mut device, pub chip: *mut irq_chip, pub chip_data: *mut core::ffi::c_void, pub handler: irq_flow_handler_t, pub handler_data: *mut core::ffi::c_void, pub handler_name: *const core::ffi::c_char, pub alloc_data: *mut msi_alloc_info_t, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct msi_domain_template { pub name: [core::ffi::c_char; 48], pub chip: irq_chip, pub ops: msi_domain_ops, pub info: msi_domain_info, pub alloc_info: msi_alloc_info_t }
#[repr(C)] pub struct msi_parent_ops { pub supported_flags: u32, pub required_flags: u32, pub chip_flags: u32, pub bus_select_token: u32, pub bus_select_mask: u32, pub prefix: *const core::ffi::c_char, pub init_dev_msi_info: Option<unsafe extern "C" fn(*mut device, *mut irq_domain, *mut irq_domain, *mut msi_domain_info) -> bool> }

#[repr(C)] pub enum msi_desc_filter { MSI_DESC_ALL, MSI_DESC_NOTASSOCIATED, MSI_DESC_ASSOCIATED }

pub const MSI_MAX_INDEX: u32 = u16::MAX as u32;
pub const MSI_FLAG_USE_DEF_DOM_OPS: u32 = 1 << 0;
pub const MSI_FLAG_USE_DEF_CHIP_OPS: u32 = 1 << 1;
pub const MSI_FLAG_ACTIVATE_EARLY: u32 = 1 << 2;
pub const MSI_FLAG_MUST_REACTIVATE: u32 = 1 << 3;
pub const MSI_FLAG_DEV_SYSFS: u32 = 1 << 4;
pub const MSI_FLAG_ALLOC_SIMPLE_MSI_DESCS: u32 = 1 << 5;
pub const MSI_FLAG_FREE_MSI_DESCS: u32 = 1 << 6;
pub const MSI_FLAG_USE_DEV_FWNODE: u32 = 1 << 7;
pub const MSI_FLAG_PARENT_PM_DEV: u32 = 1 << 8;
pub const MSI_FLAG_PCI_MSI_MASK_PARENT: u32 = 1 << 9;
pub const MSI_FLAG_PCI_MSI_STARTUP_PARENT: u32 = 1 << 10;
pub const MSI_GENERIC_FLAGS_MASK: u32 = 0xffff;
pub const MSI_DOMAIN_FLAGS_MASK: u32 = 0xffff0000;
pub const MSI_FLAG_MULTI_PCI_MSI: u32 = 1 << 16;
pub const MSI_FLAG_PCI_MSIX: u32 = 1 << 17;
pub const MSI_FLAG_LEVEL_CAPABLE: u32 = 1 << 18;
pub const MSI_FLAG_MSIX_CONTIGUOUS: u32 = 1 << 19;
pub const MSI_FLAG_PCI_MSIX_ALLOC_DYN: u32 = 1 << 20;
pub const MSI_FLAG_NO_AFFINITY: u32 = 1 << 21;
pub const MSI_FLAG_NO_MASK: u32 = 1 << 22;
pub const MSI_CHIP_FLAG_SET_EOI: u32 = 1;
pub const MSI_CHIP_FLAG_SET_ACK: u32 = 1 << 1;

extern "C" {
    pub fn __get_cached_msi_msg(entry: *mut msi_desc, msg: *mut msi_msg);
    pub fn get_cached_msi_msg(irq: core::ffi::c_uint, msg: *mut msi_msg);
    pub fn msi_setup_device_data(dev: *mut device) -> i32;
    pub fn __msi_lock_descs(dev: *mut device);
    pub fn __msi_unlock_descs(dev: *mut device);
    pub fn msi_domain_first_desc(dev: *mut device, domid: core::ffi::c_uint, filter: msi_desc_filter) -> *mut msi_desc;
    pub fn msi_next_desc(dev: *mut device, domid: core::ffi::c_uint, filter: msi_desc_filter) -> *mut msi_desc;
    pub fn msi_domain_insert_msi_desc(dev: *mut device, domid: core::ffi::c_uint, init_desc: *mut msi_desc) -> i32;
    pub fn msi_domain_free_msi_descs_range(dev: *mut device, domid: core::ffi::c_uint, first: core::ffi::c_uint, last: core::ffi::c_uint);
    pub fn msi_parent_init_dev_msi_info(dev: *mut device, domain: *mut irq_domain, parent: *mut irq_domain, info: *mut msi_domain_info) -> bool;
    pub fn msi_domain_set_affinity(data: *mut irq_data, mask: *const cpumask, force: bool) -> i32;
    pub fn msi_create_irq_domain(fwnode: *mut fwnode_handle, info: *mut msi_domain_info, parent: *mut irq_domain) -> *mut irq_domain;
    pub fn msi_create_parent_irq_domain(info: *mut irq_domain_info, ops: *const msi_parent_ops) -> *mut irq_domain;
    pub fn msi_create_device_irq_domain(dev: *mut device, domid: core::ffi::c_uint, template: *const msi_domain_template, hwsize: core::ffi::c_uint, domain_data: *mut core::ffi::c_void, chip_data: *mut core::ffi::c_void) -> bool;
    pub fn msi_remove_device_irq_domain(dev: *mut device, domid: core::ffi::c_uint);
    pub fn msi_match_device_irq_domain(dev: *mut device, domid: core::ffi::c_uint, token: irq_domain_bus_token) -> bool;
    pub fn msi_domain_alloc_irqs_range_locked(dev: *mut device, domid: core::ffi::c_uint, first: core::ffi::c_uint, last: core::ffi::c_uint) -> i32;
    pub fn msi_domain_alloc_irqs_range(dev: *mut device, domid: core::ffi::c_uint, first: core::ffi::c_uint, last: core::ffi::c_uint) -> i32;
    pub fn msi_domain_alloc_irqs_all_locked(dev: *mut device, domid: core::ffi::c_uint, nirqs: i32) -> i32;
    pub fn msi_domain_free_irqs_range_locked(dev: *mut device, domid: core::ffi::c_uint, first: core::ffi::c_uint, last: core::ffi::c_uint);
    pub fn msi_domain_free_irqs_range(dev: *mut device, domid: core::ffi::c_uint, first: core::ffi::c_uint, last: core::ffi::c_uint);
    pub fn msi_domain_free_irqs_all_locked(dev: *mut device, domid: core::ffi::c_uint);
    pub fn msi_domain_free_irqs_all(dev: *mut device, domid: core::ffi::c_uint);
    pub fn msi_get_domain_info(domain: *mut irq_domain) -> *mut msi_domain_info;
    pub fn platform_device_msi_init_and_alloc_irqs(dev: *mut device, nvec: core::ffi::c_uint, write: irq_write_msi_msg_t) -> i32;
    pub fn platform_device_msi_free_irqs_all(dev: *mut device);
    pub fn msi_device_has_isolated_msi(dev: *mut device) -> bool;
    pub fn arch_restore_msi_irqs(dev: *mut pci_dev) -> bool;
    pub fn arch_setup_msi_irq(dev: *mut pci_dev, desc: *mut msi_desc) -> i32;
    pub fn arch_teardown_msi_irq(irq: core::ffi::c_uint);
    pub fn arch_setup_msi_irqs(dev: *mut pci_dev, nvec: i32, kind: i32) -> i32;
    pub fn arch_teardown_msi_irqs(dev: *mut pci_dev);
    pub fn msi_device_populate_sysfs(dev: *mut device) -> i32;
    pub fn msi_device_destroy_sysfs(dev: *mut device);
    pub fn msi_desc_to_pci_dev(desc: *mut msi_desc) -> *mut pci_dev;
    pub fn pci_write_msi_msg(irq: core::ffi::c_uint, msg: *mut msi_msg);
    pub fn __pci_read_msi_msg(entry: *mut msi_desc, msg: *mut msi_msg);
    pub fn __pci_write_msi_msg(entry: *mut msi_desc, msg: *mut msi_msg);
    pub fn pci_msi_mask_irq(data: *mut irq_data);
    pub fn pci_msi_unmask_irq(data: *mut irq_data);
    pub fn pci_msi_domain_get_msi_rid(domain: *mut irq_domain, pdev: *mut pci_dev) -> u32;
    pub fn pci_msi_map_rid_ctlr_node(domain: *mut irq_domain, pdev: *mut pci_dev, node: *mut *mut fwnode_handle) -> u32;
    pub fn pci_msi_get_device_domain(pdev: *mut pci_dev) -> *mut irq_domain;
    pub fn pci_msix_prepare_desc(domain: *mut irq_domain, arg: *mut msi_alloc_info_t, desc: *mut msi_desc);
}

pub type irq_write_msi_msg_t = Option<unsafe extern "C" fn(*mut msi_desc, *mut msi_msg)>;
pub type msi_alloc_info_t = core::ffi::c_void;
pub type irq_hw_number_t = u64;
pub type irq_flow_handler_t = Option<unsafe extern "C" fn(*mut irq_desc)>;
pub type irq_domain_bus_token = u32;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct irq_chip { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct irq_desc { _private: [u8; 0] }
#[repr(C)] pub struct irq_fwspec { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain_info { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct irq_affinity_desc { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub union msi_instance_cookie { pub value: u64, pub ptr: *mut core::ffi::c_void }

pub const MSI_DEFAULT_DOMAIN: u32 = 0;
pub const fn msi_desc_to_dev(desc: *mut msi_desc) -> *mut device { unsafe { (*desc).dev } }

pub unsafe fn msi_first_desc(dev: *mut device, filter: msi_desc_filter) -> *mut msi_desc { msi_domain_first_desc(dev, MSI_DEFAULT_DOMAIN, filter) }
pub unsafe fn msi_insert_msi_desc(dev: *mut device, init_desc: *mut msi_desc) -> i32 { msi_domain_insert_msi_desc(dev, MSI_DEFAULT_DOMAIN, init_desc) }
pub unsafe fn msi_free_msi_descs_range(dev: *mut device, first: u32, last: u32) { msi_domain_free_msi_descs_range(dev, MSI_DEFAULT_DOMAIN, first, last) }
pub unsafe fn msi_free_msi_descs(dev: *mut device) { msi_free_msi_descs_range(dev, 0, MSI_MAX_INDEX) }
pub unsafe fn msi_domain_alloc_irqs(dev: *mut device, domid: u32, nirqs: i32) -> i32 { msi_domain_alloc_irqs_range(dev, domid, 0, nirqs.wrapping_sub(1) as u32) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
