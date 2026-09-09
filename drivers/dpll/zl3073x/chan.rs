// SPDX-License-Identifier: GPL-2.0-only

// C dependencies from linux/cleanup.h, linux/delay.h, linux/dev_printk.h,
// linux/ptp_clock_kernel.h, linux/string.h, linux/types.h, chan.h and core.h
// are supplied by the surrounding translation unit.

pub unsafe fn zl3073x_chan_state_update(zldev: *mut zl3073x_dev, index: u8) -> i32 {
    let chan = unsafe { &mut (*zldev).chan[index as usize] };
    let mut val: u64 = 0;
    let mut rc: i32;

    rc = unsafe { zl3073x_read_u8(zldev, ZL_REG_DPLL_MON_STATUS(index), &mut chan.mon_status) };
    if rc != 0 { return rc; }
    rc = unsafe { zl3073x_read_u8(zldev, ZL_REG_DPLL_REFSEL_STATUS(index), &mut chan.refsel_status) };
    if rc != 0 { return rc; }

    // Read df_offset only when locked to a reference. In NCO mode it was
    // captured at entry by nco_mode_set(); preserve it.
    if !unsafe { zl3073x_chan_is_locked(chan) } {
        if !unsafe { zl3073x_chan_mode_is_nco(chan) } { chan.df_offset = ZL_DPLL_DF_OFFSET_UNKNOWN; }
        return 0;
    }
    rc = unsafe { zl3073x_poll_zero_u8(zldev, ZL_REG_DPLL_DF_READ(index), ZL_DPLL_DF_READ_SEM, ZL_POLL_DF_READ_TIMEOUT_US) };
    if rc != 0 { return rc; }
    rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_DF_READ(index), ZL_DPLL_DF_READ_SEM | ZL_DPLL_DF_READ_REF_OFST) };
    if rc != 0 { return rc; }
    rc = unsafe { zl3073x_poll_zero_u8(zldev, ZL_REG_DPLL_DF_READ(index), ZL_DPLL_DF_READ_SEM, ZL_POLL_DF_READ_TIMEOUT_US) };
    if rc != 0 { return rc; }
    rc = unsafe { zl3073x_read_u48(zldev, ZL_REG_DPLL_DF_OFFSET(index), &mut val) };
    if rc != 0 { return rc; }
    chan.df_offset = sign_extend64(val, 47);
    0
}

pub unsafe fn zl3073x_chan_nco_mode_set(zldev: *mut zl3073x_dev, index: u8) -> i32 {
    let chan = unsafe { &mut (*zldev).chan[index as usize] };
    let prev_mode = unsafe { zl3073x_chan_mode_get(chan) };
    let mut val: u64 = 0;
    let mut df_read: u8;
    let mut rc: i32;
    if prev_mode == ZL_DPLL_MODE_REFSEL_MODE_FREERUN {
        unsafe { zl3073x_chan_mode_set(chan, ZL_DPLL_MODE_REFSEL_MODE_NCO); }
        rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_MODE_REFSEL(index), chan.mode_refsel) };
        if rc != 0 { unsafe { zl3073x_chan_mode_set(chan, prev_mode); } return rc; }
        chan.df_offset = ZL_DPLL_DF_OFFSET_UNKNOWN;
        return 0;
    }
    df_read = FIELD_PREP(ZL_DPLL_DF_READ_REF_OFST, 0) | FIELD_PREP(ZL_DPLL_DF_READ_CMD, ZL_DPLL_DF_READ_CMD_ACC_I);
    rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_DF_READ(index), df_read) };
    if rc != 0 { return rc; }
    fsleep(25000);
    unsafe { zl3073x_chan_mode_set(chan, ZL_DPLL_MODE_REFSEL_MODE_NCO); }
    rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_MODE_REFSEL(index), chan.mode_refsel) };
    if rc != 0 { unsafe { zl3073x_chan_mode_set(chan, prev_mode); } return rc; }
    fsleep(25000);
    rc = unsafe { zl3073x_read_u48(zldev, ZL_REG_DPLL_DF_OFFSET(index), &mut val) };
    if rc != 0 {
        unsafe { dev_warn((*zldev).dev, "Failed to read DPLL%u df_offset: %pe\n", index, ERR_PTR(rc)); }
        chan.df_offset = ZL_DPLL_DF_OFFSET_UNKNOWN;
    } else { chan.df_offset = sign_extend64(val, 47); }
    0
}

pub unsafe fn zl3073x_chan_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> i32 {
    let chan = unsafe { &mut (*zldev).chan[index as usize] };
    let mut rc: i32;
    rc = unsafe { zl3073x_read_u8(zldev, ZL_REG_DPLL_CTRL(index), &mut chan.ctrl) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_read_u8(zldev, ZL_REG_DPLL_MODE_REFSEL(index), &mut chan.mode_refsel) }; if rc != 0 { return rc; }
    unsafe { dev_dbg((*zldev).dev, "DPLL%u mode: %u, ref: %u\n", index, zl3073x_chan_mode_get(chan), zl3073x_chan_ref_get(chan)); }
    rc = unsafe { zl3073x_chan_state_update(zldev, index) }; if rc != 0 { return rc; }
    if unsafe { zl3073x_chan_mode_is_nco(chan) } { chan.df_offset = ZL_DPLL_DF_OFFSET_UNKNOWN; }
    unsafe { dev_dbg((*zldev).dev, "DPLL%u lock_state: %u, ho: %u, sel_state: %u, sel_ref: %u\n", index, zl3073x_chan_lock_state_get(chan), if zl3073x_chan_is_ho_ready(chan) { 1 } else { 0 }, zl3073x_chan_refsel_state_get(chan), zl3073x_chan_refsel_ref_get(chan)); }
    // guard(mutex)(&zldev->multiop_lock);
    rc = unsafe { zl3073x_mb_op(zldev, ZL_REG_DPLL_MB_SEM, ZL_DPLL_MB_SEM_RD, ZL_REG_DPLL_MB_MASK, BIT(index)) }; if rc != 0 { return rc; }
    for i in 0..chan.ref_prio.len() {
        rc = unsafe { zl3073x_read_u8(zldev, ZL_REG_DPLL_REF_PRIO(i as _), &mut chan.ref_prio[i]) }; if rc != 0 { return rc; }
    }
    0
}

pub unsafe fn zl3073x_chan_state_get(zldev: *mut zl3073x_dev, index: u8) -> *const zl3073x_chan { unsafe { &(*zldev).chan[index as usize] } }

pub unsafe fn zl3073x_chan_tod_ready_wait(zldev: *mut zl3073x_dev, ch: u8) -> i32 {
    let mut tod_ctrl = 0u8;
    let rc = unsafe { zl3073x_read_u8(zldev, ZL_REG_DPLL_TOD_CTRL(ch), &mut tod_ctrl) }; if rc != 0 { return rc; }
    if tod_ctrl & ZL_DPLL_TOD_CTRL_SEM == 0 { return 0; }
    let timeout = match FIELD_GET(ZL_DPLL_TOD_CTRL_CMD, tod_ctrl) {
        ZL_DPLL_TOD_CTRL_CMD_WR_NEXT_1HZ => ZL_POLL_TOD_WR_TIMEOUT_US,
        ZL_DPLL_TOD_CTRL_CMD_RD_CURRENT | ZL_DPLL_TOD_CTRL_CMD_RD_NEXT_1HZ => ZL_POLL_TOD_RD_TIMEOUT_US,
        _ => return -EOPNOTSUPP,
    };
    let rc = unsafe { zl3073x_poll_zero_u8(zldev, ZL_REG_DPLL_TOD_CTRL(ch), ZL_DPLL_TOD_CTRL_SEM, timeout) };
    if rc == -ETIMEDOUT { -EBUSY } else { rc }
}

unsafe fn zl3073x_chan_tod_ctrl(zldev: *mut zl3073x_dev, ch: u8, cmd: u8) -> i32 { unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_TOD_CTRL(ch), ZL_DPLL_TOD_CTRL_SEM | cmd) } }

pub unsafe fn zl3073x_chan_tod_read(zldev: *mut zl3073x_dev, ch: u8, next_hz: bool, ts: *mut timespec64, sts: *mut ptp_system_timestamp) -> i32 {
    let cmd = if next_hz { ZL_DPLL_TOD_CTRL_CMD_RD_NEXT_1HZ } else { ZL_DPLL_TOD_CTRL_CMD_RD_CURRENT };
    let mut rc = unsafe { zl3073x_chan_tod_ready_wait(zldev, ch) }; if rc != 0 { return rc; }
    unsafe { ptp_read_system_prets(sts); }
    rc = unsafe { zl3073x_chan_tod_ctrl(zldev, ch, cmd) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_chan_tod_ready_wait(zldev, ch) }; if rc != 0 { return rc; }
    unsafe { ptp_read_system_postts(sts); }
    let mut sec = 0u64; let mut nsec = 0u32;
    rc = unsafe { zl3073x_read_u48(zldev, ZL_REG_DPLL_TOD_SEC(ch), &mut sec) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_read_u32(zldev, ZL_REG_DPLL_TOD_NS(ch), &mut nsec) }; if rc != 0 { return rc; }
    unsafe { (*ts).tv_sec = sec; (*ts).tv_nsec = nsec; } 0
}

pub unsafe fn zl3073x_chan_tod_write(zldev: *mut zl3073x_dev, ch: u8, ts: timespec64) -> i32 {
    let mut rc = unsafe { zl3073x_chan_tod_ready_wait(zldev, ch) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_write_u48(zldev, ZL_REG_DPLL_TOD_SEC(ch), ts.tv_sec) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_write_u32(zldev, ZL_REG_DPLL_TOD_NS(ch), ts.tv_nsec) }; if rc != 0 { return rc; }
    unsafe { zl3073x_chan_tod_ctrl(zldev, ch, ZL_DPLL_TOD_CTRL_CMD_WR_NEXT_1HZ) }
}

pub unsafe fn zl3073x_chan_tod_adjust(zldev: *mut zl3073x_dev, ch: u8, delta: timespec64) -> i32 {
    const ZL_TOD_MAX_RETRIES: i32 = 20; const THRESHOLD_NS: i64 = 20 * NSEC_PER_MSEC;
    let mut ts_next = timespec64::default(); let mut ts_cur = timespec64::default(); let mut diff;
    let mut i = 0;
    while i < ZL_TOD_MAX_RETRIES {
        let mut rc = unsafe { zl3073x_chan_tod_read(zldev, ch, true, &mut ts_next, core::ptr::null_mut()) }; if rc != 0 { return rc; }
        rc = unsafe { zl3073x_chan_tod_read(zldev, ch, false, &mut ts_cur, core::ptr::null_mut()) }; if rc != 0 { return rc; }
        diff = timespec64_sub(ts_next, ts_cur);
        if diff.tv_sec > 0 || (diff.tv_sec == 0 && diff.tv_nsec >= THRESHOLD_NS) { break; }
        i += 1;
    }
    if i == ZL_TOD_MAX_RETRIES { unsafe { dev_warn((*zldev).dev, "DPLL%u ToD adjust failed to get stable margin\n", ch); } return -EBUSY; }
    ts_next = timespec64_add(ts_next, delta); if !timespec64_valid_settod(&ts_next) { return -EINVAL; }
    unsafe { zl3073x_chan_tod_write(zldev, ch, ts_next) }
}

pub unsafe fn zl3073x_chan_df_offset_set(zldev: *mut zl3073x_dev, ch: u8, offset: i64) -> i32 {
    let rc = unsafe { zl3073x_write_u48(zldev, ZL_REG_DPLL_DF_OFFSET(ch), offset) };
    if rc == 0 { unsafe { (*zldev).chan[ch as usize].df_offset = offset; } } rc
}

pub unsafe fn zl3073x_chan_tie_write(zldev: *mut zl3073x_dev, ch: u8, delta_ns: i64) -> i32 {
    // guard(mutex)(&zldev->tie_lock);
    let mut rc = unsafe { zl3073x_poll_zero_u8(zldev, ZL_REG_DPLL_TIE_CTRL, ZL_DPLL_TIE_CTRL_OP, ZL_POLL_TIE_WR_TIMEOUT_US) }; if rc != 0 { return rc; }
    let tie_data = delta_ns.wrapping_mul(100000);
    rc = unsafe { zl3073x_write_u48(zldev, ZL_REG_DPLL_TIE_DATA(ch), tie_data) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_TIE_CTRL_MASK, BIT(ch)) }; if rc != 0 { return rc; }
    unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_TIE_CTRL, ZL_DPLL_TIE_CTRL_OP_WR) }
}

pub unsafe fn zl3073x_chan_phase_step(zldev: *mut zl3073x_dev, ch: u8, out_mask: u16, step_cycles: i32, tod_step: bool) -> i32 {
    // guard(mutex)(&zldev->phase_step_lock);
    let mut rc = unsafe { zl3073x_poll_zero_u8(zldev, ZL_REG_OUTPUT_PHASE_STEP_CTRL, ZL_OUTPUT_PHASE_STEP_CTRL_OP, ZL_POLL_PHASE_STEP_TIMEOUT_US) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_write_u32(zldev, ZL_REG_OUTPUT_PHASE_STEP_DATA, step_cycles) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_write_u16(zldev, ZL_REG_OUTPUT_PHASE_STEP_MASK, out_mask) }; if rc != 0 { return rc; }
    rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_OUTPUT_PHASE_STEP_NUMBER, 1) }; if rc != 0 { return rc; }
    let mut ctrl = FIELD_PREP(ZL_OUTPUT_PHASE_STEP_CTRL_DPLL, ch) | FIELD_PREP(ZL_OUTPUT_PHASE_STEP_CTRL_OP, ZL_OUTPUT_PHASE_STEP_CTRL_OP_WRITE);
    if tod_step { ctrl |= ZL_OUTPUT_PHASE_STEP_CTRL_TOD_STEP; }
    unsafe { zl3073x_write_u8(zldev, ZL_REG_OUTPUT_PHASE_STEP_CTRL, ctrl) }
}

pub unsafe fn zl3073x_chan_state_set(zldev: *mut zl3073x_dev, index: u8, chan: *const zl3073x_chan) -> i32 {
    let dchan = unsafe { &mut (*zldev).chan[index as usize] }; let chan = unsafe { &*chan };
    // C: if (!memcmp(&dchan->cfg, &chan->cfg, sizeof(chan->cfg))) return 0;
    if unsafe { memcmp(core::ptr::addr_of!(dchan.cfg).cast(), core::ptr::addr_of!(chan.cfg).cast(), core::mem::size_of_val(&chan.cfg)) == 0 } { return 0; }
    let mut rc: i32;
    if dchan.ctrl != chan.ctrl { rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_CTRL(index), chan.ctrl) }; if rc != 0 { return rc; } dchan.ctrl = chan.ctrl; }
    if dchan.mode_refsel != chan.mode_refsel { rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_MODE_REFSEL(index), chan.mode_refsel) }; if rc != 0 { return rc; } dchan.mode_refsel = chan.mode_refsel; }
    if dchan.ref_prio == chan.ref_prio { dchan.cfg = chan.cfg; return 0; }
    // guard(mutex)(&zldev->multiop_lock);
    rc = unsafe { zl3073x_mb_op(zldev, ZL_REG_DPLL_MB_SEM, ZL_DPLL_MB_SEM_RD, ZL_REG_DPLL_MB_MASK, BIT(index)) }; if rc != 0 { return rc; }
    for i in 0..chan.ref_prio.len() { if dchan.ref_prio[i] != chan.ref_prio[i] { rc = unsafe { zl3073x_write_u8(zldev, ZL_REG_DPLL_REF_PRIO(i as _), chan.ref_prio[i]) }; if rc != 0 { return rc; } } }
    rc = unsafe { zl3073x_mb_op(zldev, ZL_REG_DPLL_MB_SEM, ZL_DPLL_MB_SEM_WR, ZL_REG_DPLL_MB_MASK, BIT(index)) }; if rc != 0 { return rc; }
    dchan.cfg = chan.cfg; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
