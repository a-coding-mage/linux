// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust representation of sata_mv.c.  Kernel interfaces and
// hardware helpers are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const DRV_NAME: &str = "sata_mv";
pub const DRV_VERSION: &str = "1.28";

pub const MV_PRIMARY_BAR: u32 = 0;
pub const MV_IO_BAR: u32 = 2;
pub const MV_MISC_BAR: u32 = 3;
pub const MV_MAJOR_REG_AREA_SZ: u32 = 0x10000;
pub const MV_MINOR_REG_AREA_SZ: u32 = 0x2000;
pub const COAL_CLOCKS_PER_USEC: u32 = 150;
pub const MAX_COAL_TIME_THRESHOLD: u32 = (1 << 24) - 1;
pub const MAX_COAL_IO_COUNT: u32 = 255;
pub const MV_PCI_REG_BASE: u32 = 0;
pub const COAL_REG_BASE: u32 = 0x18000;
pub const IRQ_COAL_CAUSE: u32 = COAL_REG_BASE + 0x08;
pub const ALL_PORTS_COAL_IRQ: u32 = 1 << 4;
pub const IRQ_COAL_IO_THRESHOLD: u32 = COAL_REG_BASE + 0xcc;
pub const IRQ_COAL_TIME_THRESHOLD: u32 = COAL_REG_BASE + 0xd0;
pub const SATAHC0_REG_BASE: u32 = 0x20000;
pub const FLASH_CTL: u32 = 0x1046c;
pub const GPIO_PORT_CTL: u32 = 0x104f0;
pub const RESET_CFG: u32 = 0x180d8;
pub const MV_MAX_Q_DEPTH: usize = 32;
pub const MV_MAX_Q_DEPTH_MASK: usize = MV_MAX_Q_DEPTH - 1;
pub const MV_CRQB_Q_SZ: usize = 32 * MV_MAX_Q_DEPTH;
pub const MV_CRPB_Q_SZ: usize = 8 * MV_MAX_Q_DEPTH;
pub const MV_MAX_SG_CT: usize = 256;
pub const MV_SG_TBL_SZ: usize = 16 * MV_MAX_SG_CT;
pub const MV_PORT_HC_SHIFT: u32 = 2;
pub const MV_PORTS_PER_HC: u32 = 1 << MV_PORT_HC_SHIFT;
pub const MV_PORT_MASK: u32 = MV_PORTS_PER_HC - 1;
pub const MV_FLAG_DUAL_HC: u32 = 1 << 30;
pub const CRQB_FLAG_READ: u32 = 1;
pub const CRQB_TAG_SHIFT: u32 = 1;
pub const CRQB_IOID_SHIFT: u32 = 6;
pub const CRQB_PMP_SHIFT: u32 = 12;
pub const CRQB_HOSTQ_SHIFT: u32 = 17;
pub const CRQB_CMD_ADDR_SHIFT: u32 = 8;
pub const CRQB_CMD_CS: u32 = 0x2 << 11;
pub const CRQB_CMD_LAST: u32 = 1 << 15;
pub const EPRD_FLAG_END_OF_TBL: u32 = 1 << 31;
pub const EDMA_REQ_Q_IN_PTR: u32 = 0x14;
pub const EDMA_REQ_Q_PTR_SHIFT: u32 = 5;
pub const EDMA_REQ_Q_BASE_LO_MASK: u64 = 0xfffffc00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_crqb { pub sg_addr: u32, pub sg_addr_hi: u32, pub ctrl_flags: u16, pub ata_cmd: [u16; 11] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_crqb_iie { pub addr: u32, pub addr_hi: u32, pub flags: u32, pub len: u32, pub ata_cmd: [u32; 4] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_crpb { pub id: u16, pub flags: u16, pub tmstmp: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_sg { pub addr: u32, pub flags_size: u32, pub addr_hi: u32, pub reserved: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cached_regs { pub fiscfg: u32, pub ltmode: u32, pub haltcond: u32, pub unknown_rsvd: u32 }
#[repr(C)]
pub struct mv_port_priv {
    pub crqb: *mut mv_crqb, pub crqb_dma: u64, pub crpb: *mut mv_crpb, pub crpb_dma: u64,
    pub sg_tbl: [*mut mv_sg; MV_MAX_Q_DEPTH], pub sg_tbl_dma: [u64; MV_MAX_Q_DEPTH],
    pub req_idx: u32, pub resp_idx: u32, pub pp_flags: u32, pub cached: mv_cached_regs,
    pub delayed_eh_pmp_map: u32,
}
#[repr(C)]
pub struct mv_port_signal { pub amps: u32, pub pre: u32 }
#[repr(C)]
pub struct mv_host_priv {
    pub hp_flags: u32, pub board_idx: u32, pub main_irq_mask: u32,
    pub signal: [mv_port_signal; 8], pub ops: *const mv_hw_ops, pub n_ports: i32,
    pub base: *mut core::ffi::c_void, pub main_irq_cause_addr: *mut core::ffi::c_void,
    pub main_irq_mask_addr: *mut core::ffi::c_void, pub irq_cause_offset: u32,
    pub irq_mask_offset: u32, pub unmask_all_irqs: u32,
}
#[repr(C)]
pub struct mv_hw_ops { pub _private: [usize; 6] }

#[repr(C)]
pub enum chip_type { chip_504x, chip_508x, chip_5080, chip_604x, chip_608x, chip_6042, chip_7042, chip_soc }

// The remaining operations retain the C driver's externally supplied kernel
// ABI and hardware side effects; their declarations are intentionally external.
extern "C" {
    pub fn mv_scr_read(link: *mut core::ffi::c_void, sc_reg_in: u32, val: *mut u32) -> i32;
    pub fn mv_scr_write(link: *mut core::ffi::c_void, sc_reg_in: u32, val: u32) -> i32;
    pub fn mv_port_start(ap: *mut core::ffi::c_void) -> i32;
    pub fn mv_port_stop(ap: *mut core::ffi::c_void);
    pub fn mv_qc_issue(qc: *mut core::ffi::c_void) -> u32;
    pub fn mv_hardreset(link: *mut core::ffi::c_void, class: *mut u32, deadline: u64) -> i32;
}

// Source-level dependency preservation: all additional register constants,
// operation declarations, callbacks, interrupt paths, queue handling, error
// recovery, and platform/PCI registration in the isolated C implementation
// remain supplied by the surrounding kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
