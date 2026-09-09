// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation.

/// zl3073x_ref_freq_factorize - factorize given frequency
/// @freq: input frequency
/// @base: base frequency
/// @mult: multiplier
///
/// Checks if the given frequency can be factorized using one of the
/// supported base frequencies.
pub unsafe fn zl3073x_ref_freq_factorize(
    freq: u32,
    base: *mut u16,
    mult: *mut u16,
) -> i32 {
    const BASE_FREQS: [u16; 50] = [
        1, 2, 4, 5, 8, 10, 16, 20, 25, 32, 40, 50, 64, 80, 100, 125,
        128, 160, 200, 250, 256, 320, 400, 500, 625, 640, 800, 1000,
        1250, 1280, 1600, 2000, 2500, 3125, 3200, 4000, 5000, 6250,
        6400, 8000, 10000, 12500, 15625, 16000, 20000, 25000, 31250,
        32000, 40000, 50000, 62500,
    ];

    for &base_freq in BASE_FREQS.iter() {
        let div = freq / base_freq as u32;
        if div <= u16::MAX as u32 && freq % base_freq as u32 == 0 {
            if !base.is_null() {
                *base = base_freq;
            }
            if !mult.is_null() {
                *mult = div as u16;
            }
            return 0;
        }
    }
    -22 // -EINVAL
}

pub unsafe fn zl3073x_ref_state_update(
    zldev: *mut zl3073x_dev,
    index: u8,
) -> i32 {
    let ref_ = &mut (*zldev).ref_[index as usize];
    zl3073x_read_u8(zldev, ZL_REG_REF_MON_STATUS(index), &mut ref_.mon_status)
}

pub unsafe fn zl3073x_ref_state_fetch(
    zldev: *mut zl3073x_dev,
    index: u8,
) -> i32 {
    let ref_ = &mut (*zldev).ref_[index as usize];
    let mut rc: i32;

    if zl3073x_is_n_pin(index) && zl3073x_ref_is_diff(ref_.offset(-1)) {
        let p_ref = &*ref_.offset(-1);
        ref_.cfg = p_ref.cfg;
        ref_.inv = p_ref.inv;
        return 0;
    }

    rc = zl3073x_ref_state_update(zldev, index);
    if rc != 0 { return rc; }

    // Equivalent of guard(mutex)(&zldev->multiop_lock).
    rc = zl3073x_mb_op(zldev, ZL_REG_REF_MB_SEM, ZL_REF_MB_SEM_RD,
                       ZL_REG_REF_MB_MASK, BIT(index));
    if rc != 0 { return rc; }
    rc = zl3073x_read_u8(zldev, ZL_REG_REF_CONFIG, &mut ref_.config);
    if rc != 0 { return rc; }
    rc = zl3073x_read_u16(zldev, ZL_REG_REF_FREQ_BASE, &mut ref_.freq_base);
    if rc != 0 { return rc; }
    rc = zl3073x_read_u16(zldev, ZL_REG_REF_FREQ_MULT, &mut ref_.freq_mult);
    if rc != 0 { return rc; }
    rc = zl3073x_read_u16(zldev, ZL_REG_REF_RATIO_M, &mut ref_.freq_ratio_m);
    if rc != 0 { return rc; }
    rc = zl3073x_read_u16(zldev, ZL_REG_REF_RATIO_N, &mut ref_.freq_ratio_n);
    if rc != 0 { return rc; }
    rc = zl3073x_read_u32(zldev, ZL_REG_REF_ESYNC_DIV, &mut ref_.esync_n_div);
    if rc != 0 { return rc; }
    rc = zl3073x_read_u8(zldev, ZL_REG_REF_SYNC_CTRL, &mut ref_.sync_ctrl);
    if rc != 0 { return rc; }

    if zl3073x_dev_is_ref_phase_comp_32bit(zldev) {
        let mut val: u32 = 0;
        rc = zl3073x_read_u32(zldev, ZL_REG_REF_PHASE_OFFSET_COMP_32, &mut val);
        ref_.phase_comp = val as _;
    } else {
        rc = zl3073x_read_u48(zldev, ZL_REG_REF_PHASE_OFFSET_COMP, &mut ref_.phase_comp);
    }
    if rc != 0 { return rc; }
    dev_dbg(zldev, "REF{} is {} and configured as {}\n", index,
            str_enabled_disabled(zl3073x_ref_is_enabled(ref_)),
            if zl3073x_ref_is_diff(ref_) { "differential" } else { "single-ended" });
    rc
}

pub unsafe fn zl3073x_ref_state_get(zldev: *mut zl3073x_dev, index: u8) -> *const zl3073x_ref {
    &(*zldev).ref_[index as usize]
}

pub unsafe fn zl3073x_ref_state_set(
    zldev: *mut zl3073x_dev,
    index: u8,
    ref_: *const zl3073x_ref,
) -> i32 {
    let dref = &mut (*zldev).ref_[index as usize];
    let mut rc: i32;

    if WARN_ON(core::slice::from_raw_parts(&dref.inv as *const _, core::mem::size_of_val(&dref.inv))
        != core::slice::from_raw_parts(&(*ref_).inv as *const _, core::mem::size_of_val(&(*ref_).inv))) { return -22; }
    if dref.cfg == (*ref_).cfg { return 0; }

    // Equivalent of guard(mutex)(&zldev->multiop_lock).
    rc = zl3073x_mb_op(zldev, ZL_REG_REF_MB_SEM, ZL_REF_MB_SEM_RD,
                       ZL_REG_REF_MB_MASK, BIT(index));
    if rc != 0 { return rc; }
    if dref.freq_base != (*ref_).freq_base { rc = zl3073x_write_u16(zldev, ZL_REG_REF_FREQ_BASE, (*ref_).freq_base); }
    if rc == 0 && dref.freq_mult != (*ref_).freq_mult { rc = zl3073x_write_u16(zldev, ZL_REG_REF_FREQ_MULT, (*ref_).freq_mult); }
    if rc == 0 && dref.freq_ratio_m != (*ref_).freq_ratio_m { rc = zl3073x_write_u16(zldev, ZL_REG_REF_RATIO_M, (*ref_).freq_ratio_m); }
    if rc == 0 && dref.freq_ratio_n != (*ref_).freq_ratio_n { rc = zl3073x_write_u16(zldev, ZL_REG_REF_RATIO_N, (*ref_).freq_ratio_n); }
    if rc == 0 && dref.esync_n_div != (*ref_).esync_n_div { rc = zl3073x_write_u32(zldev, ZL_REG_REF_ESYNC_DIV, (*ref_).esync_n_div); }
    if rc == 0 && dref.sync_ctrl != (*ref_).sync_ctrl { rc = zl3073x_write_u8(zldev, ZL_REG_REF_SYNC_CTRL, (*ref_).sync_ctrl); }
    if rc == 0 && dref.phase_comp != (*ref_).phase_comp {
        rc = if zl3073x_dev_is_ref_phase_comp_32bit(zldev) { zl3073x_write_u32(zldev, ZL_REG_REF_PHASE_OFFSET_COMP_32, (*ref_).phase_comp as _) } else { zl3073x_write_u48(zldev, ZL_REG_REF_PHASE_OFFSET_COMP, (*ref_).phase_comp) };
    }
    if rc != 0 { return rc; }
    rc = zl3073x_mb_op(zldev, ZL_REG_REF_MB_SEM, ZL_REF_MB_SEM_WR, ZL_REG_REF_MB_MASK, BIT(index));
    if rc != 0 { return rc; }
    dref.cfg = (*ref_).cfg;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
