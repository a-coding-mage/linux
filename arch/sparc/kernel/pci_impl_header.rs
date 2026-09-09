/* SPDX-License-Identifier: GPL-2.0 */
/* pci_impl.h: Helper definitions for PCI controller support.
 *
 * Copyright (C) 1999, 2007 David S. Miller (davem@davemloft.net)
 */

/* The abstraction used here is that there are PCI controllers, each with one
 * (Sabre) or two (PSYCHO/SCHIZO) PCI bus modules underneath. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[inline]
pub unsafe fn pci_stc_flushflag_init(stc: *mut strbuf) {
    (*(*stc).strbuf_flushflag) = 0u64;
}

#[inline]
pub unsafe fn pci_stc_flushflag_set(stc: *const strbuf) -> bool {
    (*(*stc).strbuf_flushflag) != 0u64
}

#[cfg(feature = "CONFIG_PCI_MSI")]
#[repr(C)]
pub struct sparc64_msiq_ops {
    pub get_head: Option<unsafe extern "C" fn(*mut pci_pbm_info, c_ulong, *mut c_ulong) -> c_int>,
    pub dequeue_msi: Option<unsafe extern "C" fn(*mut pci_pbm_info, c_ulong, *mut c_ulong, *mut c_ulong) -> c_int>,
    pub set_head: Option<unsafe extern "C" fn(*mut pci_pbm_info, c_ulong, c_ulong) -> c_int>,
    pub msi_setup: Option<unsafe extern "C" fn(*mut pci_pbm_info, c_ulong, c_ulong, c_int) -> c_int>,
    pub msi_teardown: Option<unsafe extern "C" fn(*mut pci_pbm_info, c_ulong) -> c_int>,
    pub msiq_alloc: Option<unsafe extern "C" fn(*mut pci_pbm_info) -> c_int>,
    pub msiq_free: Option<unsafe extern "C" fn(*mut pci_pbm_info)>,
    pub msiq_build_irq: Option<unsafe extern "C" fn(*mut pci_pbm_info, c_ulong, c_ulong) -> c_int>,
}

#[cfg(feature = "CONFIG_PCI_MSI")]
extern "C" {
    pub fn sparc64_pbm_msi_init(pbm: *mut pci_pbm_info, ops: *const sparc64_msiq_ops);
}

#[cfg(feature = "CONFIG_PCI_MSI")]
#[repr(C)]
pub struct sparc64_msiq_cookie {
    pub pbm: *mut pci_pbm_info,
    pub msiqid: c_ulong,
}

#[repr(C)]
pub struct pci_pbm_info {
    pub next: *mut pci_pbm_info,
    pub sibling: *mut pci_pbm_info,
    pub index: c_int,
    pub controller_regs: c_ulong,
    pub pbm_regs: c_ulong,
    pub sync_reg: c_ulong,
    pub portid: u32,
    pub devhandle: u32,
    pub chip_type: c_int,
    pub chip_version: c_int,
    pub chip_revision: c_int,
    pub name: *const c_char,
    pub op: *mut platform_device,
    pub ino_bitmap: u64,
    pub io_space: resource,
    pub mem_space: resource,
    pub mem64_space: resource,
    pub busn: resource,
    pub io_offset: resource_size_t,
    pub mem_offset: resource_size_t,
    pub mem64_offset: resource_size_t,
    pub config_space: c_ulong,
    pub config_space_reg_bits: c_ulong,
    pub pci_afsr: c_ulong,
    pub pci_afar: c_ulong,
    pub pci_csr: c_ulong,
    pub is_66mhz_capable: c_int,
    pub all_devs_66mhz: c_int,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msiq_num: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msiq_ent_count: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msiq_first: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msiq_first_devino: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msiq_rotor: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msiq_irq_cookies: *mut sparc64_msiq_cookie,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi_num: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi_first: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi_data_mask: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msix_data_width: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi32_start: u64,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi64_start: u64,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi32_len: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi64_len: u32,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi_queues: *mut c_void,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi_bitmap: *mut c_ulong,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi_irq_table: *mut c_uint,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub setup_msi_irq: Option<unsafe extern "C" fn(*mut c_uint, *mut pci_dev, *mut msi_desc) -> c_int>,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub teardown_msi_irq: Option<unsafe extern "C" fn(c_uint, *mut pci_dev)>,
    #[cfg(feature = "CONFIG_PCI_MSI")]
    pub msi_ops: *const sparc64_msiq_ops,
    pub stc: strbuf,
    pub iommu: *mut iommu,
    pub pci_first_busno: c_uint,
    pub pci_last_busno: c_uint,
    pub pci_bus: *mut pci_bus,
    pub pci_ops: *mut pci_ops,
    pub numa_node: c_int,
}

pub const PBM_CHIP_TYPE_SABRE: c_int = 1;
pub const PBM_CHIP_TYPE_PSYCHO: c_int = 2;
pub const PBM_CHIP_TYPE_SCHIZO: c_int = 3;
pub const PBM_CHIP_TYPE_SCHIZO_PLUS: c_int = 4;
pub const PBM_CHIP_TYPE_TOMATILLO: c_int = 5;

extern "C" {
    pub static mut pci_pbm_root: *mut pci_pbm_info;
    pub static mut pci_num_pbms: c_int;
    pub fn pci_get_pbm_props(pbm: *mut pci_pbm_info);
    pub fn pci_scan_one_pbm(pbm: *mut pci_pbm_info, parent: *mut device) -> *mut pci_bus;
    pub fn pci_determine_mem_io_space(pbm: *mut pci_pbm_info);
    pub fn pci_scan_for_target_abort(pbm: *mut pci_pbm_info, bus: *mut pci_bus);
    pub fn pci_scan_for_master_abort(pbm: *mut pci_pbm_info, bus: *mut pci_bus);
    pub fn pci_scan_for_parity_error(pbm: *mut pci_pbm_info, bus: *mut pci_bus);
    pub fn pci_config_read8(addr: *mut u8, ret: *mut u8);
    pub fn pci_config_read16(addr: *mut u16, ret: *mut u16);
    pub fn pci_config_read32(addr: *mut u32, ret: *mut u32);
    pub fn pci_config_write8(addr: *mut u8, val: u8);
    pub fn pci_config_write16(addr: *mut u16, val: u16);
    pub fn pci_config_write32(addr: *mut u32, val: u32);
    pub static mut sun4u_pci_ops: pci_ops;
    pub static mut sun4v_pci_ops: pci_ops;
    pub static mut pci_poke_in_progress: c_int;
    pub static mut pci_poke_cpu: c_int;
    pub static mut pci_poke_faulted: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
