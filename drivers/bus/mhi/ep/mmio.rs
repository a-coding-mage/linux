// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Linaro Ltd.
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

use core::ffi::c_void;

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn __ffs(value: u32) -> u32;
}

pub unsafe fn mhi_ep_mmio_read(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32) -> u32 {
    readl((*mhi_cntrl).mmio.add(offset as usize) as *const c_void)
}

pub unsafe fn mhi_ep_mmio_write(mhi_cntrl: *mut mhi_ep_cntrl, offset: u32, val: u32) {
    writel(val, (*mhi_cntrl).mmio.add(offset as usize) as *mut c_void);
}

pub unsafe fn mhi_ep_mmio_masked_write(
    mhi_cntrl: *mut mhi_ep_cntrl,
    offset: u32,
    mask: u32,
    val: u32,
) {
    let mut regval: u32;

    regval = mhi_ep_mmio_read(mhi_cntrl, offset);
    regval &= !mask;
    regval |= (val << __ffs(mask)) & mask;
    mhi_ep_mmio_write(mhi_cntrl, offset, regval);
}

pub unsafe fn mhi_ep_mmio_masked_read(dev: *mut mhi_ep_cntrl, offset: u32, mask: u32) -> u32 {
    let mut regval: u32;

    regval = mhi_ep_mmio_read(dev, offset);
    regval &= mask;
    regval >>= __ffs(mask);

    regval
}

pub unsafe fn mhi_ep_mmio_get_mhi_state(
    mhi_cntrl: *mut mhi_ep_cntrl,
    state: *mut mhi_state,
    mhi_reset: *mut bool,
) {
    let regval = mhi_ep_mmio_read(mhi_cntrl, EP_MHICTRL);
    *state = ((regval & MHICTRL_MHISTATE_MASK) >> __ffs(MHICTRL_MHISTATE_MASK)) as mhi_state;
    *mhi_reset = ((regval & MHICTRL_RESET_MASK) >> __ffs(MHICTRL_RESET_MASK)) != 0;
}

unsafe fn mhi_ep_mmio_set_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32, enable: bool) {
    let chid_shift = ch_id % 32;
    let chid_mask = 1u32 << chid_shift;
    let chdb_idx = ch_id / 32;
    let val = if enable { 1 } else { 0 };

    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CHDB_INT_MASK_n(chdb_idx), chid_mask, val);

    /* Update the local copy of the channel mask */
    (*mhi_cntrl).chdb[chdb_idx as usize].mask &= !chid_mask;
    (*mhi_cntrl).chdb[chdb_idx as usize].mask |= val << chid_shift;
}

pub unsafe fn mhi_ep_mmio_enable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32) {
    mhi_ep_mmio_set_chdb(mhi_cntrl, ch_id, true);
}

pub unsafe fn mhi_ep_mmio_disable_chdb(mhi_cntrl: *mut mhi_ep_cntrl, ch_id: u32) {
    mhi_ep_mmio_set_chdb(mhi_cntrl, ch_id, false);
}

unsafe fn mhi_ep_mmio_set_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl, enable: bool) {
    let val = if enable { MHI_CHDB_INT_MASK_n_EN_ALL } else { 0 };
    let mut i = 0;
    while i < MHI_MASK_ROWS_CH_DB {
        mhi_ep_mmio_write(mhi_cntrl, MHI_CHDB_INT_MASK_n(i), val);
        (*mhi_cntrl).chdb[i as usize].mask = val;
        i += 1;
    }
}

pub unsafe fn mhi_ep_mmio_enable_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_set_chdb_interrupts(mhi_cntrl, true);
}

unsafe fn mhi_ep_mmio_mask_chdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_set_chdb_interrupts(mhi_cntrl, false);
}

pub unsafe fn mhi_ep_mmio_read_chdb_status_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) -> bool {
    let mut chdb = false;
    let mut i = 0;
    while i < MHI_MASK_ROWS_CH_DB {
        (*mhi_cntrl).chdb[i as usize].status =
            mhi_ep_mmio_read(mhi_cntrl, MHI_CHDB_INT_STATUS_n(i));
        if (*mhi_cntrl).chdb[i as usize].status != 0 {
            chdb = true;
        }
        i += 1;
    }

    /* Return whether a channel doorbell interrupt occurred or not */
    chdb
}

unsafe fn mhi_ep_mmio_set_erdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl, enable: bool) {
    let val = if enable { MHI_ERDB_INT_MASK_n_EN_ALL } else { 0 };
    let mut i = 0;
    while i < MHI_MASK_ROWS_EV_DB {
        mhi_ep_mmio_write(mhi_cntrl, MHI_ERDB_INT_MASK_n(i), val);
        i += 1;
    }
}

unsafe fn mhi_ep_mmio_mask_erdb_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_set_erdb_interrupts(mhi_cntrl, false);
}

pub unsafe fn mhi_ep_mmio_enable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK, MHI_CTRL_MHICTRL_MASK, 1);
}

pub unsafe fn mhi_ep_mmio_disable_ctrl_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK, MHI_CTRL_MHICTRL_MASK, 0);
}

pub unsafe fn mhi_ep_mmio_enable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK, MHI_CTRL_CRDB_MASK, 1);
}

pub unsafe fn mhi_ep_mmio_disable_cmdb_interrupt(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_masked_write(mhi_cntrl, MHI_CTRL_INT_MASK, MHI_CTRL_CRDB_MASK, 0);
}

pub unsafe fn mhi_ep_mmio_mask_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_disable_ctrl_interrupt(mhi_cntrl);
    mhi_ep_mmio_disable_cmdb_interrupt(mhi_cntrl);
    mhi_ep_mmio_mask_chdb_interrupts(mhi_cntrl);
    mhi_ep_mmio_mask_erdb_interrupts(mhi_cntrl);
}

unsafe fn mhi_ep_mmio_clear_interrupts(mhi_cntrl: *mut mhi_ep_cntrl) {
    let mut i = 0;
    while i < MHI_MASK_ROWS_CH_DB {
        mhi_ep_mmio_write(mhi_cntrl, MHI_CHDB_INT_CLEAR_n(i), MHI_CHDB_INT_CLEAR_n_CLEAR_ALL);
        i += 1;
    }
    i = 0;
    while i < MHI_MASK_ROWS_EV_DB {
        mhi_ep_mmio_write(mhi_cntrl, MHI_ERDB_INT_CLEAR_n(i), MHI_ERDB_INT_CLEAR_n_CLEAR_ALL);
        i += 1;
    }
    mhi_ep_mmio_write(
        mhi_cntrl,
        MHI_CTRL_INT_CLEAR,
        MHI_CTRL_INT_MMIO_WR_CLEAR | MHI_CTRL_INT_CRDB_CLEAR | MHI_CTRL_INT_CRDB_MHICTRL_CLEAR,
    );
}

pub unsafe fn mhi_ep_mmio_get_chc_base(mhi_cntrl: *mut mhi_ep_cntrl) {
    let mut regval = mhi_ep_mmio_read(mhi_cntrl, EP_CCABAP_HIGHER);
    (*mhi_cntrl).ch_ctx_host_pa = (regval as u64) << 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_CCABAP_LOWER);
    (*mhi_cntrl).ch_ctx_host_pa |= regval as u64;
}

pub unsafe fn mhi_ep_mmio_get_erc_base(mhi_cntrl: *mut mhi_ep_cntrl) {
    let mut regval = mhi_ep_mmio_read(mhi_cntrl, EP_ECABAP_HIGHER);
    (*mhi_cntrl).ev_ctx_host_pa = (regval as u64) << 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_ECABAP_LOWER);
    (*mhi_cntrl).ev_ctx_host_pa |= regval as u64;
}

pub unsafe fn mhi_ep_mmio_get_crc_base(mhi_cntrl: *mut mhi_ep_cntrl) {
    let mut regval = mhi_ep_mmio_read(mhi_cntrl, EP_CRCBAP_HIGHER);
    (*mhi_cntrl).cmd_ctx_host_pa = (regval as u64) << 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, EP_CRCBAP_LOWER);
    (*mhi_cntrl).cmd_ctx_host_pa |= regval as u64;
}

pub unsafe fn mhi_ep_mmio_get_db(ring: *mut mhi_ep_ring) -> u64 {
    let mhi_cntrl = (*ring).mhi_cntrl;
    let mut regval = mhi_ep_mmio_read(mhi_cntrl, (*ring).db_offset_h);
    let mut db_offset = (regval as u64) << 32;
    regval = mhi_ep_mmio_read(mhi_cntrl, (*ring).db_offset_l);
    db_offset |= regval as u64;
    db_offset
}

pub unsafe fn mhi_ep_mmio_set_env(mhi_cntrl: *mut mhi_ep_cntrl, value: u32) {
    mhi_ep_mmio_write(mhi_cntrl, EP_BHI_EXECENV, value);
}

pub unsafe fn mhi_ep_mmio_clear_reset(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_masked_write(mhi_cntrl, EP_MHICTRL, MHICTRL_RESET_MASK, 0);
}

pub unsafe fn mhi_ep_mmio_reset(mhi_cntrl: *mut mhi_ep_cntrl) {
    mhi_ep_mmio_write(mhi_cntrl, EP_MHICTRL, 0);
    mhi_ep_mmio_write(mhi_cntrl, EP_MHISTATUS, 0);
    mhi_ep_mmio_clear_interrupts(mhi_cntrl);
}

pub unsafe fn mhi_ep_mmio_init(mhi_cntrl: *mut mhi_ep_cntrl) {
    (*mhi_cntrl).chdb_offset = mhi_ep_mmio_read(mhi_cntrl, EP_CHDBOFF);
    (*mhi_cntrl).erdb_offset = mhi_ep_mmio_read(mhi_cntrl, EP_ERDBOFF);
    let regval = mhi_ep_mmio_read(mhi_cntrl, EP_MHICFG);
    (*mhi_cntrl).event_rings = (regval & MHICFG_NER_MASK) >> __ffs(MHICFG_NER_MASK);
    (*mhi_cntrl).hw_event_rings = (regval & MHICFG_NHWER_MASK) >> __ffs(MHICFG_NHWER_MASK);
    mhi_ep_mmio_reset(mhi_cntrl);
}

pub unsafe fn mhi_ep_mmio_update_ner(mhi_cntrl: *mut mhi_ep_cntrl) {
    let regval = mhi_ep_mmio_read(mhi_cntrl, EP_MHICFG);
    (*mhi_cntrl).event_rings = (regval & MHICFG_NER_MASK) >> __ffs(MHICFG_NER_MASK);
    (*mhi_cntrl).hw_event_rings = (regval & MHICFG_NHWER_MASK) >> __ffs(MHICFG_NHWER_MASK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
