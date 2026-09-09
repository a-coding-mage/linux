/* SPDX-License-Identifier: GPL-2.0 */
/* PCI Endpoint Controller (EPC) header file, translated from C. */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_interface_type {
    UNKNOWN_INTERFACE = -1,
    PRIMARY_INTERFACE,
    SECONDARY_INTERFACE,
}

pub unsafe fn pci_epc_interface_string(type_: pci_epc_interface_type) -> &'static [u8] {
    match type_ {
        pci_epc_interface_type::PRIMARY_INTERFACE => b"primary\0",
        pci_epc_interface_type::SECONDARY_INTERFACE => b"secondary\0",
        _ => b"UNKNOWN interface\0",
    }
}

#[repr(C)]
pub struct pci_epc_map {
    pub pci_addr: u64,
    pub pci_size: usize,
    pub map_pci_addr: u64,
    pub map_size: usize,
    pub phys_base: phys_addr_t,
    pub phys_addr: phys_addr_t,
    pub virt_base: *mut core::ffi::c_void,
    pub virt_addr: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_aux_resource_type {
    PCI_EPC_AUX_DOORBELL_MMIO,
}

#[repr(C)]
pub struct pci_epc_aux_resource {
    pub type_: pci_epc_aux_resource_type,
    pub phys_addr: phys_addr_t,
    pub size: resource_size_t,
    pub bar: pci_barno,
    pub bar_offset: resource_size_t,
    pub u: pci_epc_aux_resource_union,
}

#[repr(C)]
pub union pci_epc_aux_resource_union {
    pub db_mmio: pci_epc_aux_resource_db_mmio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_aux_resource_db_mmio {
    pub irq: i32,
    pub data: u32,
}

#[repr(C)]
pub struct pci_epc_ops {
    pub write_header: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, *mut pci_epf_header) -> i32>,
    pub set_bar: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, *mut pci_epf_bar) -> i32>,
    pub clear_bar: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, *mut pci_epf_bar)>,
    pub align_addr: Option<unsafe extern "C" fn(*mut pci_epc, u64, *mut usize, *mut usize) -> u64>,
    pub map_addr: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, phys_addr_t, u64, usize) -> i32>,
    pub unmap_addr: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, phys_addr_t)>,
    pub set_msi: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, u8) -> i32>,
    pub get_msi: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8) -> i32>,
    pub set_msix: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, u16, pci_barno, u32) -> i32>,
    pub get_msix: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8) -> i32>,
    pub raise_irq: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, core::ffi::c_uint, u16) -> i32>,
    pub map_msi_irq: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, phys_addr_t, u8, u32, *mut u32, *mut u32) -> i32>,
    pub start: Option<unsafe extern "C" fn(*mut pci_epc) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut pci_epc)>,
    pub get_features: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8) -> *const pci_epc_features>,
    pub get_aux_resources_count: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8) -> i32>,
    pub get_aux_resources: Option<unsafe extern "C" fn(*mut pci_epc, u8, u8, *mut pci_epc_aux_resource, i32) -> i32>,
    pub owner: *mut module,
}

#[repr(C)]
pub struct pci_epc_mem_window {
    pub phys_base: phys_addr_t,
    pub size: usize,
    pub page_size: usize,
}

#[repr(C)]
pub struct pci_epc_mem {
    pub window: pci_epc_mem_window,
    pub bitmap: *mut core::ffi::c_ulong,
    pub pages: i32,
    pub lock: mutex,
}

#[repr(C)]
pub struct pci_epc {
    pub dev: device,
    pub pci_epf: list_head,
    pub list_lock: mutex,
    pub ops: *const pci_epc_ops,
    pub windows: *mut *mut pci_epc_mem,
    pub mem: *mut pci_epc_mem,
    pub num_windows: core::ffi::c_uint,
    pub max_functions: u8,
    pub max_vfs: *mut u8,
    pub group: *mut config_group,
    pub lock: mutex,
    pub function_num_map: core::ffi::c_ulong,
    pub domain_nr: i32,
    pub init_complete: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_bar_type {
    BAR_PROGRAMMABLE = 0,
    BAR_FIXED,
    BAR_RESIZABLE,
    BAR_RESERVED,
    BAR_DISABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_bar_rsvd_region_type {
    PCI_EPC_BAR_RSVD_DMA_CTRL_MMIO = 0,
    PCI_EPC_BAR_RSVD_MSIX_TBL_RAM,
    PCI_EPC_BAR_RSVD_MSIX_PBA_RAM,
}

#[repr(C)]
pub struct pci_epc_bar_rsvd_region {
    pub type_: pci_epc_bar_rsvd_region_type,
    pub offset: resource_size_t,
    pub size: resource_size_t,
}

#[repr(C)]
pub struct pci_epc_bar_desc {
    pub type_: pci_epc_bar_type,
    pub fixed_size: u64,
    pub only_64bit: bool,
    pub nr_rsvd_regions: u8,
    pub rsvd_regions: *const pci_epc_bar_rsvd_region,
}

#[repr(C)]
pub struct pci_epc_features {
    pub linkup_notifier: u32,
    pub dynamic_inbound_mapping: u32,
    pub subrange_mapping: u32,
    pub msi_capable: u32,
    pub msix_capable: u32,
    pub intx_capable: u32,
    pub bar: [pci_epc_bar_desc; PCI_STD_NUM_BARS],
    pub align: usize,
}

pub unsafe fn epc_set_drvdata(epc: *mut pci_epc, data: *mut core::ffi::c_void) {
    dev_set_drvdata(unsafe { &mut (*epc).dev }, data);
}

pub unsafe fn epc_get_drvdata(epc: *mut pci_epc) -> *mut core::ffi::c_void {
    dev_get_drvdata(unsafe { &mut (*epc).dev })
}

extern "C" {
    pub fn __devm_pci_epc_create(dev: *mut device, ops: *const pci_epc_ops, owner: *mut module) -> *mut pci_epc;
    pub fn __pci_epc_create(dev: *mut device, ops: *const pci_epc_ops, owner: *mut module) -> *mut pci_epc;
    pub fn pci_epc_destroy(epc: *mut pci_epc);
    pub fn pci_epc_add_epf(epc: *mut pci_epc, epf: *mut pci_epf, type_: pci_epc_interface_type) -> i32;
    pub fn pci_epc_linkup(epc: *mut pci_epc);
    pub fn pci_epc_linkdown(epc: *mut pci_epc);
    pub fn pci_epc_init_notify(epc: *mut pci_epc);
    pub fn pci_epc_notify_pending_init(epc: *mut pci_epc, epf: *mut pci_epf);
    pub fn pci_epc_deinit_notify(epc: *mut pci_epc);
    pub fn pci_epc_bus_master_enable_notify(epc: *mut pci_epc);
    pub fn pci_epc_remove_epf(epc: *mut pci_epc, epf: *mut pci_epf, type_: pci_epc_interface_type);
    pub fn pci_epc_write_header(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, hdr: *mut pci_epf_header) -> i32;
    pub fn pci_epc_bar_size_to_rebar_cap(size: usize, cap: *mut u32) -> i32;
    pub fn pci_epc_set_bar(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, epf_bar: *mut pci_epf_bar) -> i32;
    pub fn pci_epc_clear_bar(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, epf_bar: *mut pci_epf_bar);
    pub fn pci_epc_map_addr(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, phys_addr: phys_addr_t, pci_addr: u64, size: usize) -> i32;
    pub fn pci_epc_unmap_addr(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, phys_addr: phys_addr_t);
    pub fn pci_epc_set_msi(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, nr_irqs: u8) -> i32;
    pub fn pci_epc_get_msi(epc: *mut pci_epc, func_no: u8, vfunc_no: u8) -> i32;
    pub fn pci_epc_set_msix(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, nr_irqs: u16, bar: pci_barno, offset: u32) -> i32;
    pub fn pci_epc_get_msix(epc: *mut pci_epc, func_no: u8, vfunc_no: u8) -> i32;
    pub fn pci_epc_map_msi_irq(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, phys_addr: phys_addr_t, interrupt_num: u8, entry_size: u32, msi_data: *mut u32, msi_addr_offset: *mut u32) -> i32;
    pub fn pci_epc_raise_irq(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, type_: core::ffi::c_uint, interrupt_num: u16) -> i32;
    pub fn pci_epc_start(epc: *mut pci_epc) -> i32;
    pub fn pci_epc_stop(epc: *mut pci_epc);
    pub fn pci_epc_get_features(epc: *mut pci_epc, func_no: u8, vfunc_no: u8) -> *const pci_epc_features;
    pub fn pci_epc_get_aux_resources_count(epc: *mut pci_epc, func_no: u8, vfunc_no: u8) -> i32;
    pub fn pci_epc_get_aux_resources(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, resources: *mut pci_epc_aux_resource, num_resources: i32) -> i32;
    pub fn pci_epc_get_first_free_bar(features: *const pci_epc_features) -> pci_barno;
    pub fn pci_epc_get_next_free_bar(features: *const pci_epc_features, bar: pci_barno) -> pci_barno;
    pub fn pci_epc_get(epc_name: *const core::ffi::c_char) -> *mut pci_epc;
    pub fn pci_epc_put(epc: *mut pci_epc);
    pub fn pci_epc_mem_init(epc: *mut pci_epc, base: phys_addr_t, size: usize, page_size: usize) -> i32;
    pub fn pci_epc_multi_mem_init(epc: *mut pci_epc, window: *mut pci_epc_mem_window, num_windows: core::ffi::c_uint) -> i32;
    pub fn pci_epc_mem_exit(epc: *mut pci_epc);
    pub fn pci_epc_mem_alloc_addr(epc: *mut pci_epc, phys_addr: *mut phys_addr_t, size: usize) -> *mut core::ffi::c_void;
    pub fn pci_epc_mem_free_addr(epc: *mut pci_epc, phys_addr: phys_addr_t, virt_addr: *mut core::ffi::c_void, size: usize);
    pub fn pci_epc_mem_map(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, pci_addr: u64, pci_size: usize, map: *mut pci_epc_map) -> i32;
    pub fn pci_epc_mem_unmap(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, map: *mut pci_epc_map);
}

extern "C" {
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
}

// When CONFIG_PCI_ENDPOINT is disabled, these inline C stubs replace the
// declarations above.
#[cfg(not(feature = "CONFIG_PCI_ENDPOINT"))]
pub unsafe fn pci_epc_init_notify(_epc: *mut pci_epc) {}
#[cfg(not(feature = "CONFIG_PCI_ENDPOINT"))]
pub unsafe fn pci_epc_deinit_notify(_epc: *mut pci_epc) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
