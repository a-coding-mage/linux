/* SPDX-License-Identifier: GPL-2.0 */
/* PCI Endpoint Function (EPF) header file. */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

pub struct pci_epf;
pub struct pci_epc_features;
pub enum pci_epc_interface_type {}
pub struct config_group;
pub struct device;
pub struct device_driver;
pub struct module;
pub struct list_head;
pub struct mutex;
pub struct msi_msg;
pub struct pci_epc;
pub struct pci_epf_device_id;
pub enum pci_interrupt_pin {}

pub type dma_addr_t = u64;
pub type resource_size_t = u64;

pub const PCI_STD_NUM_BARS: usize = 6;

#[repr(i32)]
pub enum pci_barno {
    NO_BAR = -1,
    BAR_0,
    BAR_1,
    BAR_2,
    BAR_3,
    BAR_4,
    BAR_5,
}

#[repr(C)]
pub struct pci_epf_header {
    pub vendorid: u16,
    pub deviceid: u16,
    pub revid: u8,
    pub progif_code: u8,
    pub subclass_code: u8,
    pub baseclass_code: u8,
    pub cache_line_size: u8,
    pub subsys_vendor_id: u16,
    pub subsys_id: u16,
    pub interrupt_pin: pci_interrupt_pin,
}

#[repr(C)]
pub struct pci_epf_ops {
    pub bind: Option<unsafe extern "C" fn(*mut pci_epf) -> i32>,
    pub unbind: Option<unsafe extern "C" fn(*mut pci_epf)>,
    pub add_cfs: Option<unsafe extern "C" fn(*mut pci_epf, *mut config_group) -> *mut config_group>,
}

#[repr(C)]
pub struct pci_epc_event_ops {
    pub epc_init: Option<unsafe extern "C" fn(*mut pci_epf) -> i32>,
    pub epc_deinit: Option<unsafe extern "C" fn(*mut pci_epf)>,
    pub link_up: Option<unsafe extern "C" fn(*mut pci_epf) -> i32>,
    pub link_down: Option<unsafe extern "C" fn(*mut pci_epf) -> i32>,
    pub bus_master_enable: Option<unsafe extern "C" fn(*mut pci_epf) -> i32>,
}

#[repr(C)]
pub struct pci_epf_driver {
    pub probe: Option<unsafe extern "C" fn(*mut pci_epf, *const pci_epf_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut pci_epf)>,
    pub driver: device_driver,
    pub ops: *const pci_epf_ops,
    pub owner: *mut module,
    pub epf_group: list_head,
    pub id_table: *const pci_epf_device_id,
}

#[repr(C)]
pub struct pci_epf_bar_submap {
    pub phys_addr: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
pub struct pci_epf_bar {
    pub phys_addr: dma_addr_t,
    pub addr: *mut c_void,
    pub size: usize,
    pub mem_size: usize,
    pub barno: pci_barno,
    pub flags: i32,
    pub num_submap: u32,
    pub submap: *mut pci_epf_bar_submap,
}

#[repr(i32)]
pub enum pci_epf_doorbell_type {
    PCI_EPF_DOORBELL_MSI = 0,
    PCI_EPF_DOORBELL_EMBEDDED,
}

#[repr(C)]
pub struct pci_epf_doorbell_msg {
    pub msg: msi_msg,
    pub virq: i32,
    pub irq_flags: usize,
    pub type_: pci_epf_doorbell_type,
    pub bar: pci_barno,
    pub offset: resource_size_t,
    pub iova_base: dma_addr_t,
    pub iova_size: usize,
}

#[repr(C)]
pub struct pci_epf {
    pub dev: device,
    pub name: *const core::ffi::c_char,
    pub header: *mut pci_epf_header,
    pub bar: [pci_epf_bar; PCI_STD_NUM_BARS],
    pub msi_interrupts: u8,
    pub msix_interrupts: u16,
    pub func_no: u8,
    pub vfunc_no: u8,
    pub epc: *mut pci_epc,
    pub epf_pf: *mut pci_epf,
    pub driver: *mut pci_epf_driver,
    pub id: *const pci_epf_device_id,
    pub list: list_head,
    pub lock: mutex,
    pub sec_epc: *mut pci_epc,
    pub sec_epc_list: list_head,
    pub sec_epc_bar: [pci_epf_bar; PCI_STD_NUM_BARS],
    pub sec_epc_func_no: u8,
    pub group: *mut config_group,
    pub is_bound: u32,
    pub is_vf: u32,
    pub vfunction_num_map: usize,
    pub pci_vepf: list_head,
    pub event_ops: *const pci_epc_event_ops,
    pub db_msg: *mut pci_epf_doorbell_msg,
    pub num_db: u16,
}

#[repr(C)]
pub struct pci_epf_msix_tbl {
    pub msg_addr: u64,
    pub msg_data: u32,
    pub vector_ctrl: u32,
}

pub unsafe fn to_pci_epf_driver(drv: *mut device_driver) -> *mut pci_epf_driver { todo!("container_of_const") }
pub unsafe fn to_pci_epf(epf_dev: *mut device) -> *mut pci_epf { todo!("container_of") }

// Equivalent of pci_epf_register_driver(driver), with THIS_MODULE supplied by the build.
pub unsafe fn pci_epf_register_driver(driver: *mut pci_epf_driver, this_module: *mut module) -> i32 {
    __pci_epf_register_driver(driver, this_module)
}

extern "C" {
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    pub fn pci_epf_create(name: *const core::ffi::c_char) -> *mut pci_epf;
    pub fn pci_epf_destroy(epf: *mut pci_epf);
    pub fn __pci_epf_register_driver(driver: *mut pci_epf_driver, owner: *mut module) -> i32;
    pub fn pci_epf_unregister_driver(driver: *mut pci_epf_driver);
    pub fn pci_epf_alloc_space(epf: *mut pci_epf, size: usize, bar: pci_barno, epc_features: *const pci_epc_features, type_: pci_epc_interface_type) -> *mut c_void;
    pub fn pci_epf_free_space(epf: *mut pci_epf, addr: *mut c_void, bar: pci_barno, type_: pci_epc_interface_type);
    pub fn pci_epf_assign_bar_space(epf: *mut pci_epf, size: usize, bar: pci_barno, epc_features: *const pci_epc_features, type_: pci_epc_interface_type, bar_addr: dma_addr_t) -> i32;
    pub fn pci_epf_align_inbound_addr(epf: *mut pci_epf, bar: pci_barno, addr: u64, base: *mut dma_addr_t, off: *mut usize) -> i32;
    pub fn pci_epf_bind(epf: *mut pci_epf) -> i32;
    pub fn pci_epf_unbind(epf: *mut pci_epf);
    pub fn pci_epf_add_vepf(epf_pf: *mut pci_epf, epf_vf: *mut pci_epf) -> i32;
    pub fn pci_epf_remove_vepf(epf_pf: *mut pci_epf, epf_vf: *mut pci_epf);
}

#[inline]
pub unsafe fn epf_set_drvdata(epf: *mut pci_epf, data: *mut c_void) { dev_set_drvdata(&mut (*epf).dev, data); }

#[inline]
pub unsafe fn epf_get_drvdata(epf: *mut pci_epf) -> *mut c_void { dev_get_drvdata(&mut (*epf).dev) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
