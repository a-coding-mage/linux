/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
 */

/* Dependency symbols from the Linux TI-SCI and device headers are supplied externally. */

/* Global registers */
pub const UDMA_REV_REG: u32 = 0x0;
pub const UDMA_PERF_CTL_REG: u32 = 0x4;
pub const UDMA_EMU_CTL_REG: u32 = 0x8;
pub const UDMA_PSIL_TO_REG: u32 = 0x10;
pub const UDMA_UTC_CTL_REG: u32 = 0x1c;
#[inline]
pub const fn UDMA_CAP_REG(i: u32) -> u32 { 0x20 + i * 4 }
pub const UDMA_RX_FLOW_ID_FW_OES_REG: u32 = 0x80;
pub const UDMA_RX_FLOW_ID_FW_STATUS_REG: u32 = 0x88;

/* BCHANRT/TCHANRT/RCHANRT registers */
pub const UDMA_CHAN_RT_CTL_REG: u32 = 0x0;
pub const UDMA_CHAN_RT_SWTRIG_REG: u32 = 0x8;
pub const UDMA_CHAN_RT_STDATA_REG: u32 = 0x80;

#[inline]
pub const fn UDMA_CHAN_RT_PEER_REG(i: u32) -> u32 { 0x200 + i * 0x4 }
pub const UDMA_CHAN_RT_PEER_STATIC_TR_XY_REG: u32 = UDMA_CHAN_RT_PEER_REG(0);
pub const UDMA_CHAN_RT_PEER_STATIC_TR_Z_REG: u32 = UDMA_CHAN_RT_PEER_REG(1);
pub const UDMA_CHAN_RT_PEER_BCNT_REG: u32 = UDMA_CHAN_RT_PEER_REG(4);
pub const UDMA_CHAN_RT_PEER_RT_EN_REG: u32 = UDMA_CHAN_RT_PEER_REG(8);

pub const UDMA_CHAN_RT_PCNT_REG: u32 = 0x400;
pub const UDMA_CHAN_RT_BCNT_REG: u32 = 0x408;
pub const UDMA_CHAN_RT_SBCNT_REG: u32 = 0x410;

/* UDMA_CAP Registers */
#[inline] pub const fn UDMA_CAP2_TCHAN_CNT(val: u32) -> u32 { val & 0x1ff }
#[inline] pub const fn UDMA_CAP2_ECHAN_CNT(val: u32) -> u32 { (val >> 9) & 0x1ff }
#[inline] pub const fn UDMA_CAP2_RCHAN_CNT(val: u32) -> u32 { (val >> 18) & 0x1ff }
#[inline] pub const fn UDMA_CAP3_RFLOW_CNT(val: u32) -> u32 { val & 0x3fff }
#[inline] pub const fn UDMA_CAP3_HCHAN_CNT(val: u32) -> u32 { (val >> 14) & 0x1ff }
#[inline] pub const fn UDMA_CAP3_UCHAN_CNT(val: u32) -> u32 { (val >> 23) & 0x1ff }
#[inline] pub const fn BCDMA_CAP2_BCHAN_CNT(val: u32) -> u32 { val & 0x1ff }
#[inline] pub const fn BCDMA_CAP2_TCHAN_CNT(val: u32) -> u32 { (val >> 9) & 0x1ff }
#[inline] pub const fn BCDMA_CAP2_RCHAN_CNT(val: u32) -> u32 { (val >> 18) & 0x1ff }
#[inline] pub const fn BCDMA_CAP3_HBCHAN_CNT(val: u32) -> u32 { (val >> 14) & 0x1ff }
#[inline] pub const fn BCDMA_CAP3_UBCHAN_CNT(val: u32) -> u32 { (val >> 23) & 0x1ff }
#[inline] pub const fn BCDMA_CAP4_HRCHAN_CNT(val: u32) -> u32 { val & 0xff }
#[inline] pub const fn BCDMA_CAP4_URCHAN_CNT(val: u32) -> u32 { (val >> 8) & 0xff }
#[inline] pub const fn BCDMA_CAP4_HTCHAN_CNT(val: u32) -> u32 { (val >> 16) & 0xff }
#[inline] pub const fn BCDMA_CAP4_UTCHAN_CNT(val: u32) -> u32 { (val >> 24) & 0xff }
#[inline] pub const fn PKTDMA_CAP4_TFLOW_CNT(val: u32) -> u32 { val & 0x3fff }

/* UDMA_CHAN_RT_CTL_REG */
pub const UDMA_CHAN_RT_CTL_EN: u32 = 1 << 31;
pub const UDMA_CHAN_RT_CTL_TDOWN: u32 = 1 << 30;
pub const UDMA_CHAN_RT_CTL_PAUSE: u32 = 1 << 29;
pub const UDMA_CHAN_RT_CTL_FTDOWN: u32 = 1 << 28;
pub const UDMA_CHAN_RT_CTL_ERROR: u32 = 1;

/* UDMA_CHAN_RT_PEER_RT_EN_REG */
pub const UDMA_PEER_RT_EN_ENABLE: u32 = 1 << 31;
pub const UDMA_PEER_RT_EN_TEARDOWN: u32 = 1 << 30;
pub const UDMA_PEER_RT_EN_PAUSE: u32 = 1 << 29;
pub const UDMA_PEER_RT_EN_FLUSH: u32 = 1 << 28;
pub const UDMA_PEER_RT_EN_IDLE: u32 = 1 << 1;

/* UDMA_TCHAN_RT_PEER_STATIC_TR_XY_REG / UDMA_RCHAN_RT_PEER_STATIC_TR_XY_REG */
pub const PDMA_STATIC_TR_X_MASK: u32 = 0x7 << 24;
pub const PDMA_STATIC_TR_X_SHIFT: u32 = 24;
pub const PDMA_STATIC_TR_Y_MASK: u32 = 0xfff;
pub const PDMA_STATIC_TR_Y_SHIFT: u32 = 0;
#[inline] pub const fn PDMA_STATIC_TR_Y(x: u32) -> u32 { (x << PDMA_STATIC_TR_Y_SHIFT) & PDMA_STATIC_TR_Y_MASK }
#[inline] pub const fn PDMA_STATIC_TR_X(x: u32) -> u32 { (x << PDMA_STATIC_TR_X_SHIFT) & PDMA_STATIC_TR_X_MASK }
pub const PDMA_STATIC_TR_XY_ACC32: u32 = 1 << 30;
pub const PDMA_STATIC_TR_XY_BURST: u32 = 1 << 31;

/* UDMA_TCHAN_RT_PEER_STATIC_TR_Z_REG / UDMA_RCHAN_RT_PEER_STATIC_TR_Z_REG */
#[inline] pub const fn PDMA_STATIC_TR_Z(x: u32, mask: u32) -> u32 { x & mask }

/* Address Space Select */
pub const K3_ADDRESS_ASEL_SHIFT: u32 = 48;

pub enum udma_dev {}
pub enum udma_tchan {}
pub enum udma_rchan {}
pub enum udma_rflow {}
pub enum device_node {}
pub enum device {}
pub enum k3_ringacc {}
pub enum ti_sci_handle {}
pub enum ti_sci_rm_udmap_ops {}
pub enum ti_sci_rm_psil_ops {}
pub enum ti_sci_resource {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum udma_rm_range {
    RM_RANGE_BCHAN = 0,
    RM_RANGE_TCHAN,
    RM_RANGE_RCHAN,
    RM_RANGE_RFLOW,
    RM_RANGE_TFLOW,
    RM_RANGE_LAST,
}

#[repr(C)]
pub struct udma_tisci_rm {
    pub tisci: *const ti_sci_handle,
    pub tisci_udmap_ops: *const ti_sci_rm_udmap_ops,
    pub tisci_dev_id: u32,
    /* tisci information for PSI-L thread pairing/unpairing */
    pub tisci_psil_ops: *const ti_sci_rm_psil_ops,
    pub tisci_navss_dev_id: u32,
    pub rm_ranges: [*mut ti_sci_resource; 5],
}

extern "C" {
    pub fn xudma_navss_psil_pair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> i32;
    pub fn xudma_navss_psil_unpair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> i32;
    pub fn of_xudma_dev_get(np: *mut device_node, property: *const i8) -> *mut udma_dev;
    pub fn xudma_get_device(ud: *mut udma_dev) -> *mut device;
    pub fn xudma_get_ringacc(ud: *mut udma_dev) -> *mut k3_ringacc;
    pub fn xudma_dev_get_psil_base(ud: *mut udma_dev) -> u32;
    pub fn xudma_dev_get_tisci_rm(ud: *mut udma_dev) -> *mut udma_tisci_rm;
    pub fn xudma_alloc_gp_rflow_range(ud: *mut udma_dev, from: i32, cnt: i32) -> i32;
    pub fn xudma_free_gp_rflow_range(ud: *mut udma_dev, from: i32, cnt: i32) -> i32;
    pub fn xudma_tchan_get(ud: *mut udma_dev, id: i32) -> *mut udma_tchan;
    pub fn xudma_rchan_get(ud: *mut udma_dev, id: i32) -> *mut udma_rchan;
    pub fn xudma_rflow_get(ud: *mut udma_dev, id: i32) -> *mut udma_rflow;
    pub fn xudma_tchan_put(ud: *mut udma_dev, p: *mut udma_tchan);
    pub fn xudma_rchan_put(ud: *mut udma_dev, p: *mut udma_rchan);
    pub fn xudma_rflow_put(ud: *mut udma_dev, p: *mut udma_rflow);
    pub fn xudma_tchan_get_id(p: *mut udma_tchan) -> i32;
    pub fn xudma_rchan_get_id(p: *mut udma_rchan) -> i32;
    pub fn xudma_rflow_get_id(p: *mut udma_rflow) -> i32;
    pub fn xudma_tchanrt_read(tchan: *mut udma_tchan, reg: i32) -> u32;
    pub fn xudma_tchanrt_write(tchan: *mut udma_tchan, reg: i32, val: u32);
    pub fn xudma_rchanrt_read(rchan: *mut udma_rchan, reg: i32) -> u32;
    pub fn xudma_rchanrt_write(rchan: *mut udma_rchan, reg: i32, val: u32);
    pub fn xudma_rflow_is_gp(ud: *mut udma_dev, id: i32) -> bool;
    pub fn xudma_get_rflow_ring_offset(ud: *mut udma_dev) -> i32;
    pub fn xudma_is_pktdma(ud: *mut udma_dev) -> i32;
    pub fn xudma_pktdma_tflow_get_irq(ud: *mut udma_dev, udma_tflow_id: i32) -> i32;
    pub fn xudma_pktdma_rflow_get_irq(ud: *mut udma_dev, udma_rflow_id: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
