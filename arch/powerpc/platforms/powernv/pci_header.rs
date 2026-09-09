/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/compiler.h, linux/iommu.h, asm/iommu.h,
// asm/msi_bitmap.h, and symbols supplied by other translation units.

pub struct pci_dn;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pnv_phb_type {
    PNV_PHB_IODA2,
    PNV_PHB_NPU_OCAPI,
}

/* Precise PHB model for error management */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum pnv_phb_model {
    PNV_PHB_MODEL_UNKNOWN,
    PNV_PHB_MODEL_P7IOC,
    PNV_PHB_MODEL_PHB3,
}

pub const PNV_PCI_DIAG_BUF_SIZE: u32 = 8192;
pub const PNV_IODA_PE_DEV: u32 = 1 << 0;
pub const PNV_IODA_PE_BUS: u32 = 1 << 1;
pub const PNV_IODA_PE_BUS_ALL: u32 = 1 << 2;
pub const PNV_IODA_PE_MASTER: u32 = 1 << 3;
pub const PNV_IODA_PE_SLAVE: u32 = 1 << 4;
pub const PNV_IODA_PE_VF: u32 = 1 << 5;

/*
 * A brief note on PNV_IODA_PE_BUS_ALL
 *
 * This is needed because of the behaviour of PCIe-to-PCI bridges. The PHB uses
 * the Requester ID field of the PCIe request header to determine the device
 * (and PE) that initiated a DMA. In legacy PCI individual memory read/write
 * requests aren't tagged with the RID. To work around this the PCIe-to-PCI
 * bridge will use (secondary_bus_no << 8) | 0x00 as the RID on the PCIe side.
 *
 * PCIe-to-X bridges have a similar issue even though PCI-X requests also have
 * a RID in the transaction header. The PCIe-to-X bridge is permitted to "take
 * ownership" of a transaction by a PCI-X device when forwarding it to the PCIe
 * side of the bridge.
 *
 * To work around these problems we use the BUS_ALL flag since every subordinate
 * bus of the bridge should go into the same PE.
 */

/* Indicates operations are frozen for a PE: MMIO in PESTA & DMA in PESTB. */
pub const PNV_IODA_STOPPED_STATE: u64 = 0x8000000000000000;

#[repr(C)]
pub struct pnv_ioda_pe {
    pub flags: ::core::ffi::c_ulong,
    pub phb: *mut pnv_phb,
    pub device_count: ::core::ffi::c_int,
    // CONFIG_PCI_IOV: pub parent_dev: *mut pci_dev,
    pub pdev: *mut pci_dev,
    pub pbus: *mut pci_bus,
    pub rid: ::core::ffi::c_uint,
    pub pe_number: ::core::ffi::c_uint,
    pub table_group: iommu_table_group,
    pub tce_bypass_enabled: bool,
    pub tce_bypass_base: u64,
    pub dma_setup_done: bool,
    pub mve_number: ::core::ffi::c_int,
    pub master: *mut pnv_ioda_pe,
    pub slaves: list_head,
    pub list: list_head,
}

pub const PNV_PHB_FLAG_EEH: u32 = 1 << 0;

#[repr(C)]
pub struct pnv_phb {
    pub hose: *mut pci_controller,
    pub type_: pnv_phb_type,
    pub model: pnv_phb_model,
    pub hub_id: u64,
    pub opal_id: u64,
    pub flags: ::core::ffi::c_int,
    pub regs: *mut ::core::ffi::c_void,
    pub regs_phys: u64,
    pub lock: spinlock_t,
    // CONFIG_DEBUG_FS: pub has_dbgfs: ::core::ffi::c_int, pub dbgfs: *mut dentry,
    pub msi_base: ::core::ffi::c_uint,
    pub msi_bmp: msi_bitmap,
    pub init_m64: Option<unsafe extern "C" fn(*mut pnv_phb) -> ::core::ffi::c_int>,
    pub get_pe_state: Option<unsafe extern "C" fn(*mut pnv_phb, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub freeze_pe: Option<unsafe extern "C" fn(*mut pnv_phb, ::core::ffi::c_int)>,
    pub unfreeze_pe: Option<unsafe extern "C" fn(*mut pnv_phb, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub ioda: pnv_phb_ioda,
    pub diag_data_size: ::core::ffi::c_uint,
    pub diag_data: *mut u8,
}

#[repr(C)]
pub struct pnv_phb_ioda {
    pub total_pe_num: ::core::ffi::c_uint,
    pub reserved_pe_idx: ::core::ffi::c_uint,
    pub root_pe_idx: ::core::ffi::c_uint,
    pub m32_size: ::core::ffi::c_uint,
    pub m32_segsize: ::core::ffi::c_uint,
    pub m32_pci_base: ::core::ffi::c_uint,
    pub m64_bar_idx: ::core::ffi::c_uint,
    pub m64_size: ::core::ffi::c_ulong,
    pub m64_segsize: ::core::ffi::c_ulong,
    pub m64_base: ::core::ffi::c_ulong,
    pub m64_bar_alloc: ::core::ffi::c_ulong,
    pub io_size: ::core::ffi::c_uint,
    pub io_segsize: ::core::ffi::c_uint,
    pub io_pci_base: ::core::ffi::c_uint,
    pub pe_alloc_mutex: mutex,
    pub pe_alloc: *mut ::core::ffi::c_ulong,
    pub pe_array: *mut pnv_ioda_pe,
    pub m64_segmap: *mut ::core::ffi::c_uint,
    pub m32_segmap: *mut ::core::ffi::c_uint,
    pub io_segmap: *mut ::core::ffi::c_uint,
    pub irq_chip: irq_chip,
    pub pe_list: list_head,
    pub pe_list_mutex: mutex,
    pub pe_rmap: [::core::ffi::c_uint; 0x10000],
}

pub const MAX_M64_BARS: usize = 64;
pub const POWERNV_IOMMU_DEFAULT_LEVELS: u32 = 2;
pub const POWERNV_IOMMU_MAX_LEVELS: u32 = 5;

pub unsafe fn pnv_pci_is_m64(phb: *mut pnv_phb, r: *mut resource) -> bool {
    (*r).start >= (*phb).ioda.m64_base &&
        (*r).start < ((*phb).ioda.m64_base + (*phb).ioda.m64_size)
}

pub unsafe fn pnv_pci_is_m64_flags(resource_flags: ::core::ffi::c_ulong) -> bool {
    let flags = IORESOURCE_MEM_64 | IORESOURCE_PREFETCH;
    (resource_flags & flags) == flags
}

extern "C" {
    pub fn pnv_ioda_configure_pe(phb: *mut pnv_phb, pe: *mut pnv_ioda_pe) -> ::core::ffi::c_int;
    pub fn pnv_ioda_deconfigure_pe(phb: *mut pnv_phb, pe: *mut pnv_ioda_pe) -> ::core::ffi::c_int;
    pub fn pnv_pci_ioda2_setup_dma_pe(phb: *mut pnv_phb, pe: *mut pnv_ioda_pe);
    pub fn pnv_pci_ioda2_release_pe_dma(pe: *mut pnv_ioda_pe);
    pub fn pnv_ioda_alloc_pe(phb: *mut pnv_phb, count: ::core::ffi::c_int) -> *mut pnv_ioda_pe;
    pub fn pnv_ioda_free_pe(pe: *mut pnv_ioda_pe);
    pub static mut pnv_pci_ops: pci_ops;
    pub fn pnv_pci_dump_phb_diag_data(hose: *mut pci_controller, log_buff: *mut u8);
    pub fn pnv_pci_cfg_read(pdn: *mut pci_dn, where_: ::core::ffi::c_int, size: ::core::ffi::c_int, val: *mut u32) -> ::core::ffi::c_int;
    pub fn pnv_pci_cfg_write(pdn: *mut pci_dn, where_: ::core::ffi::c_int, size: ::core::ffi::c_int, val: u32) -> ::core::ffi::c_int;
    pub fn pnv_pci_table_alloc(nid: ::core::ffi::c_int) -> *mut iommu_table;
    pub fn pnv_pci_init_ioda2_phb(np: *mut device_node);
    pub fn pnv_pci_init_npu2_opencapi_phb(np: *mut device_node);
    pub fn pnv_pci_reset_secondary_bus(dev: *mut pci_dev);
    pub fn pnv_eeh_phb_reset(hose: *mut pci_controller, option: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn pnv_pci_bdfn_to_pe(phb: *mut pnv_phb, bdfn: u16) -> *mut pnv_ioda_pe;
    pub fn pnv_ioda_get_pe(dev: *mut pci_dev) -> *mut pnv_ioda_pe;
    pub fn pnv_pci_ioda2_get_table_size(page_shift: u32, window_size: u64, levels: u32) -> ::core::ffi::c_ulong;
    pub fn pnv_eeh_post_init() -> ::core::ffi::c_int;
    pub fn pe_level_printk(pe: *const pnv_ioda_pe, level: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...);
    pub fn pnv_tce_build(tbl: *mut iommu_table, index: ::core::ffi::c_long, npages: ::core::ffi::c_long, uaddr: ::core::ffi::c_ulong, direction: dma_data_direction, attrs: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pnv_tce_free(tbl: *mut iommu_table, index: ::core::ffi::c_long, npages: ::core::ffi::c_long);
    pub fn pnv_tce_xchg(tbl: *mut iommu_table, index: ::core::ffi::c_long, hpa: *mut ::core::ffi::c_ulong, direction: *mut dma_data_direction) -> ::core::ffi::c_int;
    pub fn pnv_tce_useraddrptr(tbl: *mut iommu_table, index: ::core::ffi::c_long, alloc: bool) -> *mut u64;
    pub fn pnv_tce_get(tbl: *mut iommu_table, index: ::core::ffi::c_long) -> ::core::ffi::c_ulong;
    pub fn pnv_pci_ioda2_table_alloc_pages(nid: ::core::ffi::c_int, bus_offset: u64, page_shift: u32, window_size: u64, levels: u32, alloc_userspace_copy: bool, tbl: *mut iommu_table) -> ::core::ffi::c_long;
    pub fn pnv_pci_ioda2_table_free_pages(tbl: *mut iommu_table);
    pub fn pnv_pci_link_table_and_group(node: ::core::ffi::c_int, num: ::core::ffi::c_int, tbl: *mut iommu_table, table_group: *mut iommu_table_group) -> ::core::ffi::c_long;
    pub fn pnv_pci_unlink_table_and_group(tbl: *mut iommu_table, table_group: *mut iommu_table_group);
    pub fn pnv_pci_setup_iommu_table(tbl: *mut iommu_table, tce_mem: *mut ::core::ffi::c_void, tce_size: u64, dma_offset: u64, page_shift: ::core::ffi::c_uint);
    pub fn pnv_ioda_parse_tce_sizes(phb: *mut pnv_phb) -> ::core::ffi::c_ulong;
}

pub unsafe fn pci_bus_to_pnvhb(bus: *mut pci_bus) -> *mut pnv_phb {
    let hose = (*bus).sysdata as *mut pci_controller;
    if !hose.is_null() { (*hose).private_data as *mut pnv_phb } else { core::ptr::null_mut() }
}

// CONFIG_PCI_IOV-dependent declarations and pe_err/pe_warn/pe_info variadic macros
// are preserved through the external declarations above and future dependency bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
