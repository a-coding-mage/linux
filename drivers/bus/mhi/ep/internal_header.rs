/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022, Linaro Ltd.
 *
 */

// Dependency intent: symbols from ../common.h and Linux bitfield helpers are
// supplied by other translated dependencies.

extern "C" {
    pub static mhi_ep_bus_type: bus_type;
}

pub const MHI_REG_OFFSET: u32 = 0x100;
pub const BHI_REG_OFFSET: u32 = 0x200;

/* MHI registers */
pub const EP_MHIREGLEN: u32 = MHI_REG_OFFSET + MHIREGLEN;
pub const EP_MHIVER: u32 = MHI_REG_OFFSET + MHIVER;
pub const EP_MHICFG: u32 = MHI_REG_OFFSET + MHICFG;
pub const EP_CHDBOFF: u32 = MHI_REG_OFFSET + CHDBOFF;
pub const EP_ERDBOFF: u32 = MHI_REG_OFFSET + ERDBOFF;
pub const EP_BHIOFF: u32 = MHI_REG_OFFSET + BHIOFF;
pub const EP_BHIEOFF: u32 = MHI_REG_OFFSET + BHIEOFF;
pub const EP_DEBUGOFF: u32 = MHI_REG_OFFSET + DEBUGOFF;
pub const EP_MHICTRL: u32 = MHI_REG_OFFSET + MHICTRL;
pub const EP_MHISTATUS: u32 = MHI_REG_OFFSET + MHISTATUS;
pub const EP_CCABAP_LOWER: u32 = MHI_REG_OFFSET + CCABAP_LOWER;
pub const EP_CCABAP_HIGHER: u32 = MHI_REG_OFFSET + CCABAP_HIGHER;
pub const EP_ECABAP_LOWER: u32 = MHI_REG_OFFSET + ECABAP_LOWER;
pub const EP_ECABAP_HIGHER: u32 = MHI_REG_OFFSET + ECABAP_HIGHER;
pub const EP_CRCBAP_LOWER: u32 = MHI_REG_OFFSET + CRCBAP_LOWER;
pub const EP_CRCBAP_HIGHER: u32 = MHI_REG_OFFSET + CRCBAP_HIGHER;
pub const EP_CRDB_LOWER: u32 = MHI_REG_OFFSET + CRDB_LOWER;
pub const EP_CRDB_HIGHER: u32 = MHI_REG_OFFSET + CRDB_HIGHER;
pub const EP_MHICTRLBASE_LOWER: u32 = MHI_REG_OFFSET + MHICTRLBASE_LOWER;
pub const EP_MHICTRLBASE_HIGHER: u32 = MHI_REG_OFFSET + MHICTRLBASE_HIGHER;
pub const EP_MHICTRLLIMIT_LOWER: u32 = MHI_REG_OFFSET + MHICTRLLIMIT_LOWER;
pub const EP_MHICTRLLIMIT_HIGHER: u32 = MHI_REG_OFFSET + MHICTRLLIMIT_HIGHER;
pub const EP_MHIDATABASE_LOWER: u32 = MHI_REG_OFFSET + MHIDATABASE_LOWER;
pub const EP_MHIDATABASE_HIGHER: u32 = MHI_REG_OFFSET + MHIDATABASE_HIGHER;
pub const EP_MHIDATALIMIT_LOWER: u32 = MHI_REG_OFFSET + MHIDATALIMIT_LOWER;
pub const EP_MHIDATALIMIT_HIGHER: u32 = MHI_REG_OFFSET + MHIDATALIMIT_HIGHER;

/* MHI BHI registers */
pub const EP_BHI_INTVEC: u32 = BHI_REG_OFFSET + BHI_INTVEC;
pub const EP_BHI_EXECENV: u32 = BHI_REG_OFFSET + BHI_EXECENV;

/* MHI Doorbell registers */
#[inline]
pub const fn CHDB_LOWER_n(n: u32) -> u32 { 0x400 + 0x8 * n }
#[inline]
pub const fn CHDB_HIGHER_n(n: u32) -> u32 { 0x404 + 0x8 * n }
#[inline]
pub const fn ERDB_LOWER_n(n: u32) -> u32 { 0x800 + 0x8 * n }
#[inline]
pub const fn ERDB_HIGHER_n(n: u32) -> u32 { 0x804 + 0x8 * n }

pub const MHI_CTRL_INT_STATUS: u32 = 0x4;
pub const MHI_CTRL_INT_STATUS_MSK: u32 = BIT(0);
pub const MHI_CTRL_INT_STATUS_CRDB_MSK: u32 = BIT(1);
#[inline]
pub const fn MHI_CHDB_INT_STATUS_n(n: u32) -> u32 { 0x28 + 0x4 * n }
#[inline]
pub const fn MHI_ERDB_INT_STATUS_n(n: u32) -> u32 { 0x38 + 0x4 * n }

pub const MHI_CTRL_INT_CLEAR: u32 = 0x4c;
pub const MHI_CTRL_INT_MMIO_WR_CLEAR: u32 = BIT(2);
pub const MHI_CTRL_INT_CRDB_CLEAR: u32 = BIT(1);
pub const MHI_CTRL_INT_CRDB_MHICTRL_CLEAR: u32 = BIT(0);
#[inline]
pub const fn MHI_CHDB_INT_CLEAR_n(n: u32) -> u32 { 0x70 + 0x4 * n }
pub const MHI_CHDB_INT_CLEAR_n_CLEAR_ALL: u32 = GENMASK(31, 0);
#[inline]
pub const fn MHI_ERDB_INT_CLEAR_n(n: u32) -> u32 { 0x80 + 0x4 * n }
pub const MHI_ERDB_INT_CLEAR_n_CLEAR_ALL: u32 = GENMASK(31, 0);

/* Unlike the usual "masking" convention, writing "1" to a bit in this
 * register enables the interrupt and writing "0" will disable it.. */
pub const MHI_CTRL_INT_MASK: u32 = 0x94;
pub const MHI_CTRL_INT_MASK_MASK: u32 = GENMASK(1, 0);
pub const MHI_CTRL_MHICTRL_MASK: u32 = BIT(0);
pub const MHI_CTRL_CRDB_MASK: u32 = BIT(1);
#[inline]
pub const fn MHI_CHDB_INT_MASK_n(n: u32) -> u32 { 0xb8 + 0x4 * n }
pub const MHI_CHDB_INT_MASK_n_EN_ALL: u32 = GENMASK(31, 0);
#[inline]
pub const fn MHI_ERDB_INT_MASK_n(n: u32) -> u32 { 0xc8 + 0x4 * n }
pub const MHI_ERDB_INT_MASK_n_EN_ALL: u32 = GENMASK(31, 0);

pub const NR_OF_CMD_RINGS: u32 = 1;
pub const MHI_MASK_ROWS_CH_DB: u32 = 4;
pub const MHI_MASK_ROWS_EV_DB: u32 = 4;
pub const MHI_MASK_CH_LEN: u32 = 32;
pub const MHI_MASK_EV_LEN: u32 = 32;

#[repr(C, packed(4))]
pub struct mhi_generic_ctx {
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub rbase: u64,
    pub rlen: u64,
    pub rp: u64,
    pub wp: u64,
}

#[repr(C)]
pub enum mhi_ep_ring_type { RING_TYPE_CMD, RING_TYPE_ER, RING_TYPE_CH }

#[repr(C)]
pub union mhi_ep_ring_ctx {
    pub cmd: mhi_cmd_ctxt,
    pub ev: mhi_event_ctxt,
    pub ch: mhi_chan_ctxt,
    pub generic: mhi_generic_ctx,
}

#[repr(C)]
pub struct mhi_ep_ring_item { pub node: list_head, pub ring: *mut mhi_ep_ring }

#[repr(C)]
pub struct mhi_ep_ring {
    pub mhi_cntrl: *mut mhi_ep_cntrl,
    pub ring_ctx: *mut mhi_ep_ring_ctx,
    pub ring_cache: *mut mhi_ring_element,
    pub type_: mhi_ep_ring_type,
    pub intmodt_work: delayed_work,
    pub rbase: u64,
    pub rd_offset: usize,
    pub wr_offset: usize,
    pub ring_size: usize,
    pub db_offset_h: u32,
    pub db_offset_l: u32,
    pub ch_id: u32,
    pub er_index: u32,
    pub irq_vector: u32,
    pub intmodt: u32,
    pub started: bool,
    pub irq_pending: bool,
}

#[repr(C)] pub struct mhi_ep_cmd { pub ring: mhi_ep_ring }
#[repr(C)] pub struct mhi_ep_event { pub ring: mhi_ep_ring }
#[repr(C)] pub struct mhi_ep_state_transition { pub node: list_head, pub state: mhi_state }

#[repr(C)]
pub struct mhi_ep_chan {
    pub name: *mut core::ffi::c_char,
    pub mhi_dev: *mut mhi_ep_device,
    pub ring: mhi_ep_ring,
    pub lock: mutex,
    pub xfer_cb: Option<unsafe extern "C" fn(*mut mhi_ep_device, *mut mhi_result)>,
    pub state: mhi_ch_state,
    pub dir: dma_data_direction,
    pub rd_offset: usize,
    pub tre_loc: u64,
    pub tre_size: u32,
    pub tre_bytes_left: u32,
    pub chan: u32,
    pub skip_td: bool,
}

extern "C" {
    pub fn mhi_ep_ring_init(ring: *mut mhi_ep_ring, type_: mhi_ep_ring_type, id: u32);
    pub fn mhi_ep_ring_reset(mhi_cntrl: *mut mhi_ep_cntrl, ring: *mut mhi_ep_ring);
    pub fn mhi_ep_ring_start(mhi_cntrl: *mut mhi_ep_cntrl, ring: *mut mhi_ep_ring, ctx: *mut mhi_ep_ring_ctx) -> i32;
    pub fn mhi_ep_ring_addr2offset(ring: *mut mhi_ep_ring, ptr: u64) -> usize;
    pub fn mhi_ep_ring_add_element(ring: *mut mhi_ep_ring, element: *mut mhi_ring_element) -> i32;
    pub fn mhi_ep_ring_inc_index(ring: *mut mhi_ep_ring);
    pub fn mhi_ep_update_wr_offset(ring: *mut mhi_ep_ring) -> i32;
    pub fn mhi_ep_mmio_read(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32) -> u32;
    pub fn mhi_ep_mmio_write(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32, val: u32);
    pub fn mhi_ep_mmio_masked_write(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32, mask: u32, val: u32);
    pub fn mhi_ep_mmio_masked_read(dev: *mut mhi_ep_cntrl, offset: u32, mask: u32) -> u32;
    pub fn mhi_ep_mmio_enable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_disable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_enable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_disable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_enable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32);
    pub fn mhi_ep_mmio_disable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32);
    pub fn mhi_ep_mmio_enable_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_read_chdb_status_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) -> bool;
    pub fn mhi_ep_mmio_mask_interrupts(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_get_chc_base(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_get_erc_base(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_get_crc_base(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_get_db(ring: *mut mhi_ep_ring) -> u64;
    pub fn mhi_ep_mmio_set_env(mhi_cntrl: *mut mhi_ep_cntrl, value: u32);
    pub fn mhi_ep_mmio_clear_reset(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_reset(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_get_mhi_state(mhi_cntrl: *mut mhi_ep_cntrl, state: *mut mhi_state, mhi_reset: *mut bool);
    pub fn mhi_ep_mmio_init(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_mmio_update_ner(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_send_state_change_event(mhi_cntrl: *mut mhi_ep_cntrl, state: mhi_state) -> i32;
    pub fn mhi_ep_send_ee_event(mhi_cntrl: *mut mhi_ep_cntrl, exec_env: mhi_ee_type) -> i32;
    pub fn mhi_ep_check_mhi_state(mhi_cntrl: *mut mhi_ep_cntrl, cur_mhi_state: mhi_state, mhi_state: mhi_state) -> bool;
    pub fn mhi_ep_set_mhi_state(mhi_cntrl: *mut mhi_ep_cntrl, mhi_state: mhi_state) -> i32;
    pub fn mhi_ep_set_m0_state(mhi_cntrl: *mut mhi_ep_cntrl) -> i32;
    pub fn mhi_ep_set_m3_state(mhi_cntrl: *mut mhi_ep_cntrl) -> i32;
    pub fn mhi_ep_set_ready_state(mhi_cntrl: *mut mhi_ep_cntrl) -> i32;
    pub fn mhi_ep_handle_syserr(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_resume_channels(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_suspend_channels(mhi_cntrl: *mut mhi_ep_cntrl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
