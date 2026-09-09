/* SPDX-License-Identifier: GPL-2.0 */

pub const WILDFIRE_MAX_QBB: usize = 8;
pub const WILDFIRE_PCA_PER_QBB: usize = 4;
pub const WILDFIRE_IRQ_PER_PCA: usize = 64;
pub const WILDFIRE_NR_IRQS: usize = WILDFIRE_MAX_QBB * WILDFIRE_PCA_PER_QBB * WILDFIRE_IRQ_PER_PCA;

extern "C" {
    pub static mut wildfire_hard_qbb_map: [u8; WILDFIRE_MAX_QBB];
    pub static mut wildfire_soft_qbb_map: [u8; WILDFIRE_MAX_QBB];
    pub static mut wildfire_hard_qbb_mask: libc::c_ulong;
    pub static mut wildfire_soft_qbb_mask: libc::c_ulong;
    pub static mut wildfire_gp_mask: libc::c_ulong;
    pub static mut wildfire_hs_mask: libc::c_ulong;
    pub static mut wildfire_iop_mask: libc::c_ulong;
    pub static mut wildfire_ior_mask: libc::c_ulong;
    pub static mut wildfire_pca_mask: libc::c_ulong;
    pub static mut wildfire_cpu_mask: libc::c_ulong;
    pub static mut wildfire_mem_mask: libc::c_ulong;
}

pub const QBB_MAP_EMPTY: u8 = 0xff;
pub unsafe fn WILDFIRE_QBB_EXISTS(qbbno: libc::c_ulong) -> libc::c_ulong { wildfire_soft_qbb_mask & (1 << qbbno) }
pub unsafe fn WILDFIRE_MEM_EXISTS(qbbno: libc::c_ulong) -> libc::c_ulong { wildfire_mem_mask & (0xf << (qbbno << 2)) }
pub unsafe fn WILDFIRE_PCA_EXISTS(qbbno: libc::c_ulong, pcano: libc::c_ulong) -> libc::c_ulong { wildfire_pca_mask & (1 << ((qbbno << 2) + pcano)) }

#[repr(C, align(64))]
pub struct wildfire_64 { pub csr: libc::c_ulong }
#[repr(C, align(256))]
pub struct wildfire_256 { pub csr: libc::c_ulong }
#[repr(C, align(2048))]
pub struct wildfire_2k { pub csr: libc::c_ulong }

#[repr(C)] pub struct wildfire_qsd {
    pub qsd_whami: wildfire_64, pub qsd_rev: wildfire_64, pub qsd_port_present: wildfire_64, pub qsd_port_active: wildfire_64,
    pub qsd_fault_ena: wildfire_64, pub qsd_cpu_int_ena: wildfire_64, pub qsd_mem_config: wildfire_64, pub qsd_err_sum: wildfire_64,
    pub ce_sum: [wildfire_64; 4], pub dev_init: [wildfire_64; 4], pub it_int: [wildfire_64; 4], pub ip_int: [wildfire_64; 4],
    pub uce_sum: [wildfire_64; 4], pub se_sum__non_dev_int: [wildfire_64; 4], pub scratch: [wildfire_64; 4], pub qsd_timer: wildfire_64, pub qsd_diag: wildfire_64,
}
#[repr(C)] pub struct wildfire_fast_qsd { pub qsd_whami: wildfire_256, pub __pad1: wildfire_256, pub ce_sum: wildfire_256, pub dev_init: wildfire_256, pub it_int: wildfire_256, pub ip_int: wildfire_256, pub uce_sum: wildfire_256, pub se_sum: wildfire_256 }
#[repr(C)] pub struct wildfire_qsa {
    pub qsa_qbb_id: wildfire_2k, pub __pad1: wildfire_2k, pub qsa_port_ena: wildfire_2k, pub qsa_scratch: wildfire_2k, pub qsa_config: [wildfire_2k; 5], pub qsa_ref_int: wildfire_2k, pub qsa_qbb_pop: [wildfire_2k; 2], pub qsa_dtag_fc: wildfire_2k, pub __pad2: [wildfire_2k; 3], pub qsa_diag: wildfire_2k, pub qsa_diag_lock: [wildfire_2k; 4], pub __pad3: [wildfire_2k; 11], pub qsa_cpu_err_sum: wildfire_2k, pub qsa_misc_err_sum: wildfire_2k, pub qsa_tmo_err_sum: wildfire_2k, pub qsa_err_ena: wildfire_2k, pub qsa_tmo_config: wildfire_2k, pub qsa_ill_cmd_err_sum: wildfire_2k, pub __pad4: [wildfire_2k; 26], pub qsa_busy_mask: wildfire_2k, pub qsa_arr_valid: wildfire_2k, pub __pad5: [wildfire_2k; 2], pub qsa_port_map: [wildfire_2k; 4], pub qsa_arr_addr: [wildfire_2k; 8], pub qsa_arr_mask: [wildfire_2k; 8],
}

#[repr(C)] pub struct wildfire_iop {
    pub ioa_config: wildfire_64, pub iod_config: wildfire_64, pub iop_switch_credits: wildfire_64, pub __pad1: wildfire_64, pub iop_hose_credits: wildfire_64, pub __pad2: [wildfire_64; 11],
    pub iop_hose: [wildfire_iop_hose; 4], pub ioa_hose_0_ctrl: wildfire_64, pub iod_hose_0_ctrl: wildfire_64, pub ioa_hose_1_ctrl: wildfire_64, pub iod_hose_1_ctrl: wildfire_64, pub ioa_hose_2_ctrl: wildfire_64, pub iod_hose_2_ctrl: wildfire_64, pub ioa_hose_3_ctrl: wildfire_64, pub iod_hose_3_ctrl: wildfire_64,
    pub iop_dev_int: [wildfire_iop_dev_int; 4], pub iop_err_int_target: wildfire_64, pub __pad5: [wildfire_64; 7], pub iop_qbb_err_sum: wildfire_64, pub __pad6: wildfire_64, pub iop_qbb_se_sum: wildfire_64, pub __pad7: wildfire_64, pub ioa_err_sum: wildfire_64, pub iod_err_sum: wildfire_64, pub __pad8: [wildfire_64; 4], pub ioa_diag_force_err: wildfire_64, pub iod_diag_force_err: wildfire_64, pub __pad9: [wildfire_64; 4], pub iop_diag_send_err_int: wildfire_64, pub __pad10: [wildfire_64; 15], pub ioa_scratch: wildfire_64, pub iod_scratch: wildfire_64,
}
#[repr(C)] pub struct wildfire_iop_hose { pub __pad3: wildfire_64, pub init: wildfire_64 }
#[repr(C)] pub struct wildfire_iop_dev_int { pub target: wildfire_64, pub __pad4: wildfire_64 }
#[repr(C)] pub struct wildfire_gp { pub gpa_qbb_map: [wildfire_2k; 4], pub gpa_mem_pop_map: wildfire_2k, pub gpa_scratch: wildfire_2k, pub gpa_diag: wildfire_2k, pub gpa_config_0: wildfire_2k, pub __pad1: wildfire_2k, pub gpa_init_id: wildfire_2k, pub gpa_config_2: wildfire_2k }
#[repr(C)] pub struct wildfire_pca { pub pca_what_am_i: wildfire_64, pub pca_err_sum: wildfire_64, pub pca_diag_force_err: wildfire_64, pub pca_diag_send_err_int: wildfire_64, pub pca_hose_credits: wildfire_64, pub pca_scratch: wildfire_64, pub pca_micro_addr: wildfire_64, pub pca_micro_data: wildfire_64, pub pca_pend_int: wildfire_64, pub pca_sent_int: wildfire_64, pub __pad1: wildfire_64, pub pca_stdio_edge_level: wildfire_64, pub __pad2: [wildfire_64; 52], pub pca_int: [wildfire_pca_int; 4], pub __pad3: [wildfire_64; 56], pub pca_alt_sent_int: [wildfire_64; 32] }
#[repr(C)] pub struct wildfire_pca_int { pub target: wildfire_64, pub enable: wildfire_64 }
#[repr(C)] pub struct wildfire_ne { pub ne_what_am_i: wildfire_64 }
#[repr(C)] pub struct wildfire_fe { pub fe_what_am_i: wildfire_64 }
#[repr(C)] pub struct wildfire_pci { pub pci_io_addr_ext: wildfire_64, pub pci_ctrl: wildfire_64, pub pci_err_sum: wildfire_64, pub pci_err_addr: wildfire_64, pub pci_stall_cnt: wildfire_64, pub pci_iack_special: wildfire_64, pub __pad1: [wildfire_64; 2], pub pci_pend_int: wildfire_64, pub pci_sent_int: wildfire_64, pub __pad2: [wildfire_64; 54], pub pci_window: [wildfire_pci_window; 4], pub pci_flush_tlb: wildfire_64, pub pci_perf_mon: wildfire_64 }
#[repr(C)] pub struct wildfire_pci_window { pub wbase: wildfire_64, pub wmask: wildfire_64, pub tbase: wildfire_64 }

pub const WILDFIRE_ENTITY_SHIFT: libc::c_ulong = 18;
pub const WILDFIRE_GP_ENTITY: libc::c_ulong = 0x10 << WILDFIRE_ENTITY_SHIFT;
pub const WILDFIRE_IOP_ENTITY: libc::c_ulong = 0x08 << WILDFIRE_ENTITY_SHIFT;
pub const WILDFIRE_QSA_ENTITY: libc::c_ulong = 0x04 << WILDFIRE_ENTITY_SHIFT;
pub const WILDFIRE_QSD_ENTITY_SLOW: libc::c_ulong = 0x05 << WILDFIRE_ENTITY_SHIFT;
pub const WILDFIRE_QSD_ENTITY_FAST: libc::c_ulong = 0x01 << WILDFIRE_ENTITY_SHIFT;
pub unsafe fn WILDFIRE_PCA_ENTITY(pca: libc::c_ulong) -> libc::c_ulong { (0xc | pca) << WILDFIRE_ENTITY_SHIFT }
/* IDENT_ADDR is supplied by the Alpha platform dependencies. */
pub const WILDFIRE_BASE: libc::c_ulong = IDENT_ADDR | (1 << 40);
pub const WILDFIRE_QBB_MASK: libc::c_ulong = 0x0f;
pub unsafe fn WILDFIRE_QBB(q: libc::c_long) -> libc::c_ulong { ((!q) as libc::c_ulong & WILDFIRE_QBB_MASK) << 36 }
pub unsafe fn WILDFIRE_HOSE(h: libc::c_long) -> libc::c_ulong { (h as libc::c_ulong) << 33 }
pub unsafe fn WILDFIRE_QBB_IO(q: libc::c_long) -> libc::c_ulong { WILDFIRE_BASE | WILDFIRE_QBB(q) }
pub unsafe fn WILDFIRE_QBB_HOSE(q: libc::c_long, h: libc::c_long) -> libc::c_ulong { WILDFIRE_QBB_IO(q) | WILDFIRE_HOSE(h) }
pub unsafe fn WILDFIRE_MEM(q: libc::c_long, h: libc::c_long) -> libc::c_ulong { WILDFIRE_QBB_HOSE(q,h) | 0x000000000 }
pub unsafe fn WILDFIRE_CONF(q: libc::c_long, h: libc::c_long) -> libc::c_ulong { WILDFIRE_QBB_HOSE(q,h) | 0x1FE000000 }
pub unsafe fn WILDFIRE_IO(q: libc::c_long, h: libc::c_long) -> libc::c_ulong { WILDFIRE_QBB_HOSE(q,h) | 0x1FF000000 }
pub unsafe fn WILDFIRE_qsd(q: libc::c_long) -> *mut wildfire_qsd { (WILDFIRE_QBB_IO(q) | WILDFIRE_QSD_ENTITY_SLOW | (((1 << 13) - 1) << 23)) as *mut wildfire_qsd }
pub unsafe fn WILDFIRE_fast_qsd() -> *mut wildfire_fast_qsd { (WILDFIRE_QBB_IO(0) | WILDFIRE_QSD_ENTITY_FAST | (((1 << 13) - 1) << 23)) as *mut wildfire_fast_qsd }
pub unsafe fn WILDFIRE_qsa(q: libc::c_long) -> *mut wildfire_qsa { (WILDFIRE_QBB_IO(q) | WILDFIRE_QSA_ENTITY | (((1 << 13) - 1) << 23)) as *mut wildfire_qsa }
pub unsafe fn WILDFIRE_iop(q: libc::c_long) -> *mut wildfire_iop { (WILDFIRE_QBB_IO(q) | WILDFIRE_IOP_ENTITY | (((1 << 13) - 1) << 23)) as *mut wildfire_iop }
pub unsafe fn WILDFIRE_gp(q: libc::c_long) -> *mut wildfire_gp { (WILDFIRE_QBB_IO(q) | WILDFIRE_GP_ENTITY | (((1 << 13) - 1) << 23)) as *mut wildfire_gp }
pub unsafe fn WILDFIRE_pca(q: libc::c_long, pca: libc::c_ulong) -> *mut wildfire_pca { (WILDFIRE_QBB_IO(q) | WILDFIRE_PCA_ENTITY(pca) | (((1 << 13) - 1) << 23)) as *mut wildfire_pca }
pub unsafe fn WILDFIRE_ne(q: libc::c_long, pca: libc::c_ulong) -> *mut wildfire_ne { (WILDFIRE_QBB_IO(q) | WILDFIRE_PCA_ENTITY(pca) | (((1 << 13) - 1) << 23) | (1 << 16)) as *mut wildfire_ne }
pub unsafe fn WILDFIRE_fe(q: libc::c_long, pca: libc::c_ulong) -> *mut wildfire_fe { (WILDFIRE_QBB_IO(q) | WILDFIRE_PCA_ENTITY(pca) | (((1 << 13) - 1) << 23) | (3 << 15)) as *mut wildfire_fe }
pub unsafe fn WILDFIRE_pci(q: libc::c_long, h: libc::c_ulong) -> *mut wildfire_pci { (WILDFIRE_QBB_IO(q) | WILDFIRE_PCA_ENTITY((h & 6) >> 1) | ((((h & 1) | 2) << 16)) | (((1 << 13) - 1) << 23)) as *mut wildfire_pci }
pub const WILDFIRE_IO_SPACE: libc::c_ulong = 8 * 1024 * 1024;

pub const WILDFIRE_IO_BIAS: libc::c_ulong = unsafe { WILDFIRE_IO(0, 0) };
pub const WILDFIRE_MEM_BIAS: libc::c_ulong = unsafe { WILDFIRE_MEM(0, 0) };

/* Kernel-only inline mappings; __iomem and asm/io_trivial.h are external dependencies. */
pub unsafe fn wildfire_ioportmap(addr: libc::c_ulong) -> *mut core::ffi::c_void { (addr + WILDFIRE_IO_BIAS) as *mut core::ffi::c_void }
pub unsafe fn wildfire_ioremap(addr: libc::c_ulong, _size: libc::c_ulong) -> *mut core::ffi::c_void { (addr + WILDFIRE_MEM_BIAS) as *mut core::ffi::c_void }
pub unsafe fn wildfire_is_ioaddr(addr: libc::c_ulong) -> libc::c_int { (addr >= WILDFIRE_BASE) as libc::c_int }
pub unsafe fn wildfire_is_mmio(xaddr: *const core::ffi::c_void) -> libc::c_int { ((xaddr as libc::c_ulong & 0x100000000) == 0) as libc::c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
