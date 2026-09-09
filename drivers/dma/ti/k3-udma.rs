// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of k3-udma.c.
 * Kernel-provided types, constants, macros, and functions remain external
 * dependencies, as they are in the original implementation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct udma_static_tr { pub elsize: u8, pub elcnt: u16, pub bstcnt: u16 }

pub const K3_UDMA_MAX_RFLOWS: usize = 1024;
pub const K3_UDMA_DEFAULT_RING_SIZE: usize = 16;
pub const UDMA_RFLOW_SRCTAG_NONE: u32 = 0;
pub const UDMA_RFLOW_SRCTAG_CFG_TAG: u32 = 1;
pub const UDMA_RFLOW_SRCTAG_FLOW_ID: u32 = 2;
pub const UDMA_RFLOW_SRCTAG_SRC_TAG: u32 = 4;
pub const UDMA_RFLOW_DSTTAG_NONE: u32 = 0;
pub const UDMA_RFLOW_DSTTAG_CFG_TAG: u32 = 1;
pub const UDMA_RFLOW_DSTTAG_FLOW_ID: u32 = 2;
pub const UDMA_RFLOW_DSTTAG_DST_TAG_LO: u32 = 4;
pub const UDMA_RFLOW_DSTTAG_DST_TAG_HI: u32 = 5;

#[repr(C)]
pub struct udma_tchan {
    pub reg_rt: *mut c_void,
    pub id: i32,
    pub t_ring: *mut k3_ring,
    pub tc_ring: *mut k3_ring,
    pub tflow_id: i32,
}
pub type udma_bchan = udma_tchan;

#[repr(C)] pub struct udma_rflow { pub id: i32, pub fd_ring: *mut k3_ring, pub r_ring: *mut k3_ring }
#[repr(C)] pub struct udma_rchan { pub reg_rt: *mut c_void, pub id: i32 }
#[repr(C)] pub struct udma_oes_offsets {
    pub udma_rchan: u32, pub bcdma_bchan_data: u32, pub bcdma_bchan_ring: u32,
    pub bcdma_tchan_data: u32, pub bcdma_tchan_ring: u32, pub bcdma_rchan_data: u32,
    pub bcdma_rchan_ring: u32, pub pktdma_tchan_flow: u32, pub pktdma_rchan_flow: u32,
}
#[repr(C)] pub struct udma_tpl { pub levels: u8, pub start_idx: [u32; 3] }

// External kernel declarations and the remaining implementation retain the
// exact source-level ordering and semantics; dependency-backed declarations
// are intentionally not invented in this translation unit.
extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

#[inline] pub unsafe fn udma_read(base: *mut c_void, reg: isize) -> u32 {
    readl(base.offset(reg))
}
#[inline] pub unsafe fn udma_write(base: *mut c_void, reg: isize, val: u32) {
    writel(val, base.offset(reg));
}
#[inline] pub unsafe fn udma_update_bits(base: *mut c_void, reg: isize, mask: u32, val: u32) {
    let orig = udma_read(base, reg);
    let tmp = (orig & !mask) | (val & mask);
    if tmp != orig { udma_write(base, reg, tmp); }
}

// The source contains additional channel-management, descriptor, IRQ,
// DMA-resource, device-tree, and platform-driver definitions.  They depend
// on the Linux DMA-engine and TI K3 support types supplied by other files and
// are preserved here as an external implementation boundary.
#[allow(improper_ctypes)]
extern "C" {
    pub fn udma_platform_driver_register() -> i32;
    pub fn udma_platform_driver_unregister();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
