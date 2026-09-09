// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 Cai Huoqing
 * Synopsys DesignWare HDMA v0 core
 */

// Translated from the corresponding C implementation. Kernel/project
// dependencies are supplied by other translation units.

#[repr(u32)]
enum dw_hdma_control {
    DW_HDMA_V0_CB = BIT(0),
    DW_HDMA_V0_TCB = BIT(1),
    DW_HDMA_V0_LLP = BIT(2),
    DW_HDMA_V0_LWIE = BIT(3),
    DW_HDMA_V0_RWIE = BIT(4),
    DW_HDMA_V0_CCS = BIT(8),
    DW_HDMA_V0_LLE = BIT(9),
}

#[inline]
unsafe fn __dw_regs(dw: *mut dw_edma) -> *mut dw_hdma_v0_regs {
    (*(*dw).chip).reg_base
}

#[inline]
unsafe fn __dw_ch_regs(dw: *mut dw_edma, dir: dw_edma_dir, ch: u16) -> *mut dw_hdma_v0_ch_regs {
    if dir == EDMA_DIR_WRITE {
        &mut (*__dw_regs(dw)).ch[ch as usize].wr
    } else {
        &mut (*__dw_regs(dw)).ch[ch as usize].rd
    }
}

macro_rules! SET_CH_32 {
    ($dw:expr, $dir:expr, $ch:expr, $name:ident, $value:expr) => {
        unsafe { writel($value, &mut (*__dw_ch_regs($dw, $dir, $ch)).$name) }
    };
}

macro_rules! GET_CH_32 {
    ($dw:expr, $dir:expr, $ch:expr, $name:ident) => {
        unsafe { readl(&(*__dw_ch_regs($dw, $dir, $ch)).$name) }
    };
}

macro_rules! SET_BOTH_CH_32 {
    ($dw:expr, $ch:expr, $name:ident, $value:expr) => {{
        SET_CH_32!($dw, EDMA_DIR_WRITE, $ch, $name, $value);
        SET_CH_32!($dw, EDMA_DIR_READ, $ch, $name, $value);
    }};
}

unsafe fn dw_hdma_v0_core_int_setup(chan: *mut dw_edma_chan, mut val: u32) -> u32 {
    val &= !(HDMA_V0_LOCAL_ABORT_INT_EN | HDMA_V0_REMOTE_ABORT_INT_EN |
        HDMA_V0_LOCAL_STOP_INT_EN | HDMA_V0_REMOTE_STOP_INT_EN |
        HDMA_V0_ABORT_INT_MASK | HDMA_V0_STOP_INT_MASK);

    /*
     * HDMA_INT_STATUS.STOP and .ABORT are latched only when LSIE and
     * LAIE are enabled. A remote handler needs those status bits to
     * identify the source of the IMWr, so keep local generation enabled
     * and mask the local interrupt pins instead.
     */
    val |= HDMA_V0_LOCAL_ABORT_INT_EN | HDMA_V0_LOCAL_STOP_INT_EN;

    if (*chan).irq_mode == DW_EDMA_CH_IRQ_REMOTE {
        val |= HDMA_V0_REMOTE_ABORT_INT_EN |
            HDMA_V0_REMOTE_STOP_INT_EN |
            HDMA_V0_ABORT_INT_MASK | HDMA_V0_STOP_INT_MASK;
    }

    val
}

/* HDMA management callbacks */
unsafe fn dw_hdma_v0_core_ch_off(dw: *mut dw_edma, dir: dw_edma_dir, id: u16) {
    SET_CH_32!(dw, dir, id, int_setup, HDMA_V0_STOP_INT_MASK | HDMA_V0_ABORT_INT_MASK);
    SET_CH_32!(dw, dir, id, ch_en, 0);
    SET_CH_32!(dw, dir, id, int_clear, HDMA_V0_STOP_INT_MASK | HDMA_V0_ABORT_INT_MASK);
}

unsafe fn dw_hdma_v0_core_off(dw: *mut dw_edma) {
    let mut dir = EDMA_DIR_WRITE;
    let mut id = 0;
    while id < (*dw).wr_ch_cnt {
        SET_CH_32!(dw, dir, id, int_setup, HDMA_V0_STOP_INT_MASK | HDMA_V0_ABORT_INT_MASK);
        SET_CH_32!(dw, dir, id, int_clear, HDMA_V0_STOP_INT_MASK | HDMA_V0_ABORT_INT_MASK);
        SET_CH_32!(dw, dir, id, ch_en, 0);
        id += 1;
    }
    dir = EDMA_DIR_READ;
    id = 0;
    while id < (*dw).rd_ch_cnt {
        SET_CH_32!(dw, dir, id, int_setup, HDMA_V0_STOP_INT_MASK | HDMA_V0_ABORT_INT_MASK);
        SET_CH_32!(dw, dir, id, int_clear, HDMA_V0_STOP_INT_MASK | HDMA_V0_ABORT_INT_MASK);
        SET_CH_32!(dw, dir, id, ch_en, 0);
        id += 1;
    }
}

unsafe fn dw_hdma_v0_core_quiesce(dw: *mut dw_edma) -> i32 {
    for id in 0..(*dw).wr_ch_cnt { dw_hdma_v0_core_ch_off(dw, EDMA_DIR_WRITE, id); }
    for id in 0..(*dw).rd_ch_cnt { dw_hdma_v0_core_ch_off(dw, EDMA_DIR_READ, id); }
    0
}

unsafe fn dw_hdma_v0_core_ch_quiesce(chan: *mut dw_edma_chan) -> i32 {
    dw_hdma_v0_core_ch_off((*chan).dw, (*chan).dir, (*chan).id);
    0
}

unsafe fn dw_hdma_v0_core_ch_count(_dw: *mut dw_edma, _dir: dw_edma_dir) -> u16 {
    /* The HDMA IP has no way to know the number of hardware channels. */
    HDMA_V0_MAX_NR_CH
}

unsafe fn dw_hdma_v0_core_ch_status(chan: *mut dw_edma_chan) -> dma_status {
    let dw = (*chan).dw;
    let tmp = FIELD_GET(HDMA_V0_CH_STATUS_MASK, GET_CH_32!(dw, (*chan).dir, (*chan).id, ch_stat));
    if tmp == 1 { DMA_IN_PROGRESS } else if tmp == 3 { DMA_COMPLETE } else { DMA_ERROR }
}

unsafe fn dw_hdma_v0_core_clear_done_int(chan: *mut dw_edma_chan) {
    SET_CH_32!((*chan).dw, (*chan).dir, (*chan).id, int_clear, HDMA_V0_STOP_INT_MASK);
}

unsafe fn dw_hdma_v0_core_clear_abort_int(chan: *mut dw_edma_chan) {
    SET_CH_32!((*chan).dw, (*chan).dir, (*chan).id, int_clear, HDMA_V0_ABORT_INT_MASK);
}

unsafe fn dw_hdma_v0_core_status_int(chan: *mut dw_edma_chan) -> u32 {
    GET_CH_32!((*chan).dw, (*chan).dir, (*chan).id, int_stat)
}

unsafe fn dw_hdma_v0_core_handle_int(dw_irq: *mut dw_edma_irq, dir: dw_edma_dir,
    done: dw_edma_handler_t, abort: dw_edma_handler_t) -> irqreturn_t {
    let dw = (*dw_irq).dw;
    let (total, off, mask) = if dir == EDMA_DIR_WRITE {
        ((*dw).wr_ch_cnt, 0, (*dw_irq).wr_mask)
    } else {
        ((*dw).rd_ch_cnt, (*dw).wr_ch_cnt, (*dw_irq).rd_mask)
    };
    let mut ret = IRQ_NONE;
    for_each_set_bit!(pos, mask, total) {
        let chan = &mut (*dw).chan[(pos + off) as usize] as *mut dw_edma_chan;
        if unlikely(dw_edma_core_ch_ignore_irq(chan)) { continue; }
        let val = dw_hdma_v0_core_status_int(chan);
        if FIELD_GET(HDMA_V0_STOP_INT_MASK, val) != 0 {
            dw_hdma_v0_core_clear_done_int(chan); done(chan); ret = IRQ_HANDLED;
        }
        if FIELD_GET(HDMA_V0_ABORT_INT_MASK, val) != 0 {
            dw_hdma_v0_core_clear_abort_int(chan); abort(chan); ret = IRQ_HANDLED;
        }
    }
    ret
}

unsafe fn dw_hdma_v0_write_ll_data(chan: *mut dw_edma_chan, i: i32, control: u32,
    size: u32, sar: u64, dar: u64) {
    let ofs = i as isize * core::mem::size_of::<dw_hdma_v0_lli>() as isize;
    if (*(*chan).dw).chip.as_ref().unwrap().flags & DW_EDMA_CHIP_LOCAL != 0 {
        let lli = (*chan).ll_region.vaddr.mem.offset(ofs) as *mut dw_hdma_v0_lli;
        (*lli).transfer_size = size; (*lli).sar.reg = sar; (*lli).dar.reg = dar;
        dma_wmb(); (*lli).control = control;
    } else {
        let lli = (*chan).ll_region.vaddr.io.offset(ofs) as *mut dw_hdma_v0_lli;
        writel(size, &mut (*lli).transfer_size); writeq(sar, &mut (*lli).sar.reg);
        writeq(dar, &mut (*lli).dar.reg); writel(control, &mut (*lli).control);
    }
}

unsafe fn dw_hdma_v0_write_ll_link(chan: *mut dw_edma_chan, i: i32, control: u32, pointer: u64) {
    let ofs = i as isize * core::mem::size_of::<dw_hdma_v0_lli>() as isize;
    if (*(*chan).dw).chip.as_ref().unwrap().flags & DW_EDMA_CHIP_LOCAL != 0 {
        let llp = (*chan).ll_region.vaddr.mem.offset(ofs) as *mut dw_hdma_v0_llp;
        (*llp).llp.reg = pointer; dma_wmb(); (*llp).control = control;
    } else {
        let llp = (*chan).ll_region.vaddr.io.offset(ofs) as *mut dw_hdma_v0_llp;
        writeq(pointer, &mut (*llp).llp.reg); writel(control, &mut (*llp).control);
    }
}

unsafe fn dw_hdma_v0_core_ch_enable(chan: *mut dw_edma_chan) {
    let dw = (*chan).dw;
    SET_CH_32!(dw, (*chan).dir, (*chan).id, ch_en, BIT(0));
    let tmp = dw_hdma_v0_core_int_setup(chan, GET_CH_32!(dw, (*chan).dir, (*chan).id, int_setup));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, int_setup, tmp);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, control1, HDMA_V0_LINKLIST_EN);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, llp.lsb, lower_32_bits((*chan).ll_region.paddr));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, llp.msb, upper_32_bits((*chan).ll_region.paddr));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, cycle_sync,
        HDMA_V0_CONSUMER_CYCLE_STAT | HDMA_V0_CONSUMER_CYCLE_BIT);
}

unsafe fn dw_hdma_v0_sync_ll_data(chan: *mut dw_edma_chan) {
    if (*(*chan).dw).chip.as_ref().unwrap().flags & DW_EDMA_CHIP_LOCAL == 0 {
        readl((*chan).ll_region.vaddr.io);
    }
}

unsafe fn dw_hdma_v0_core_non_ll_start(chan: *mut dw_edma_chan, child: *mut dw_edma_burst) {
    let dw = (*chan).dw;
    SET_CH_32!(dw, (*chan).dir, (*chan).id, ch_en, HDMA_V0_CH_EN);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, sar.lsb, lower_32_bits((*child).sar));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, sar.msb, upper_32_bits((*child).sar));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, dar.lsb, lower_32_bits((*child).dar));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, dar.msb, upper_32_bits((*child).dar));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, transfer_size, (*child).sz);
    let val = dw_hdma_v0_core_int_setup(chan, GET_CH_32!(dw, (*chan).dir, (*chan).id, int_setup));
    SET_CH_32!(dw, (*chan).dir, (*chan).id, int_setup, val);
    let val = GET_CH_32!(dw, (*chan).dir, (*chan).id, control1) & !HDMA_V0_LINKLIST_EN;
    SET_CH_32!(dw, (*chan).dir, (*chan).id, control1, val);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, doorbell, HDMA_V0_DOORBELL_START);
}

unsafe fn dw_hdma_v0_core_ch_config(chan: *mut dw_edma_chan) {
    let dw = (*chan).dw;
    SET_CH_32!(dw, (*chan).dir, (*chan).id, msi_stop.lsb, (*chan).msi.address_lo);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, msi_stop.msb, (*chan).msi.address_hi);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, msi_abort.lsb, (*chan).msi.address_lo);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, msi_abort.msb, (*chan).msi.address_hi);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, msi_msgdata, (*chan).msi.data);
    SET_CH_32!(dw, (*chan).dir, (*chan).id, func_num,
        FIELD_PREP(HDMA_V0_FUNC_NUM_PF_MASK, (*chan).func_no));
}

unsafe fn dw_hdma_v0_core_ll_data(chan: *mut dw_edma_chan, burst: *mut dw_edma_burst,
    idx: u32, cb: bool, _irq: bool) {
    let mut control = 0;
    if cb { control |= DW_HDMA_V0_CB as u32; }
    dw_hdma_v0_write_ll_data(chan, idx as i32, control, (*burst).sz, (*burst).sar, (*burst).dar);
}

unsafe fn dw_hdma_v0_core_ll_link(chan: *mut dw_edma_chan, idx: u32, cb: bool, addr: u64) {
    let mut control = (DW_HDMA_V0_LLP as u32) | (DW_HDMA_V0_TCB as u32);
    if !cb { control |= DW_HDMA_V0_CB as u32; }
    dw_hdma_v0_write_ll_link(chan, idx as i32, control, addr);
}

unsafe fn dw_hdma_v0_core_ch_doorbell(chan: *mut dw_edma_chan) {
    dw_hdma_v0_sync_ll_data(chan);
    SET_CH_32!((*chan).dw, (*chan).dir, (*chan).id, doorbell, HDMA_V0_DOORBELL_START);
}

/* HDMA debugfs callbacks */
unsafe fn dw_hdma_v0_core_debugfs_on(dw: *mut dw_edma) { dw_hdma_v0_debugfs_on(dw); }

unsafe fn dw_hdma_v0_core_db_offset(_dw: *mut dw_edma) -> resource_size_t {
    /* Implement once the correct offset is known. */
    !0
}

static dw_hdma_v0_core: dw_edma_core_ops = dw_edma_core_ops {
    off: Some(dw_hdma_v0_core_off),
    quiesce: Some(dw_hdma_v0_core_quiesce),
    ch_quiesce: Some(dw_hdma_v0_core_ch_quiesce),
    ch_count: Some(dw_hdma_v0_core_ch_count),
    ch_status: Some(dw_hdma_v0_core_ch_status),
    handle_int: Some(dw_hdma_v0_core_handle_int),
    non_ll_start: Some(dw_hdma_v0_core_non_ll_start),
    ll_data: Some(dw_hdma_v0_core_ll_data),
    ll_link: Some(dw_hdma_v0_core_ll_link),
    ch_doorbell: Some(dw_hdma_v0_core_ch_doorbell),
    ch_enable: Some(dw_hdma_v0_core_ch_enable),
    ch_config: Some(dw_hdma_v0_core_ch_config),
    debugfs_on: Some(dw_hdma_v0_core_debugfs_on),
    db_offset: Some(dw_hdma_v0_core_db_offset),
}

pub unsafe fn dw_hdma_v0_core_register(dw: *mut dw_edma) {
    (*dw).core = &dw_hdma_v0_core;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
