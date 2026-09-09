// SPDX-License-Identifier: GPL-2.0-only

// Linux header and local header dependencies are supplied by the surrounding
// translation unit.

extern "C" {
    fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
}

/// zl3073x_out_state_fetch - fetch output state from hardware
pub unsafe fn zl3073x_out_state_fetch(zldev: *mut zl3073x_dev, index: u8) -> i32 {
    let out = &mut (*zldev).out[index as usize];
    let mut rc: i32;

    // Read output configuration
    rc = zl3073x_read_u8(zldev, ZL_REG_OUTPUT_CTRL(index), &mut out.ctrl);
    if rc != 0 {
        return rc;
    }

    dev_dbg(
        (*zldev).dev,
        "OUT%u is %s and connected to SYNTH%u\\n",
        index,
        str_enabled_disabled(zl3073x_out_is_enabled(out)),
        zl3073x_out_synth_get(out),
    );

    // guard(mutex)(&zldev->multiop_lock);

    // Read output configuration
    rc = zl3073x_mb_op(
        zldev,
        ZL_REG_OUTPUT_MB_SEM,
        ZL_OUTPUT_MB_SEM_RD,
        ZL_REG_OUTPUT_MB_MASK,
        1u32 << index,
    );
    if rc != 0 {
        return rc;
    }

    // Read output mode
    rc = zl3073x_read_u8(zldev, ZL_REG_OUTPUT_MODE, &mut out.mode);
    if rc != 0 {
        return rc;
    }

    dev_dbg(
        (*zldev).dev,
        "OUT%u has signal format 0x%02x\\n",
        index,
        zl3073x_out_signal_format_get(out),
    );

    // Read output divisor
    rc = zl3073x_read_u32(zldev, ZL_REG_OUTPUT_DIV, &mut out.div);
    if rc != 0 {
        return rc;
    }
    if out.div == 0 {
        dev_err((*zldev).dev, "Zero divisor for OUT%u got from device\\n", index);
        return -EINVAL;
    }
    dev_dbg((*zldev).dev, "OUT%u divisor: %u\\n", index, out.div);

    // Read output width
    rc = zl3073x_read_u32(zldev, ZL_REG_OUTPUT_WIDTH, &mut out.width);
    if rc != 0 {
        return rc;
    }
    rc = zl3073x_read_u32(
        zldev,
        ZL_REG_OUTPUT_ESYNC_PERIOD,
        &mut out.esync_n_period,
    );
    if rc != 0 {
        return rc;
    }
    if out.esync_n_period == 0 {
        dev_err(
            (*zldev).dev,
            "Zero esync divisor for OUT%u got from device\\n",
            index,
        );
        return -EINVAL;
    }
    rc = zl3073x_read_u32(zldev, ZL_REG_OUTPUT_ESYNC_WIDTH, &mut out.esync_n_width);
    if rc != 0 {
        return rc;
    }
    zl3073x_read_u32(zldev, ZL_REG_OUTPUT_PHASE_COMP, &mut out.phase_comp)
}

/// zl3073x_out_state_get - get current output state
pub unsafe fn zl3073x_out_state_get(
    zldev: *mut zl3073x_dev,
    index: u8,
) -> *const zl3073x_out {
    &(*zldev).out[index as usize] as *const zl3073x_out
}

/// zl3073x_out_state_set - commit output state changes to hardware
pub unsafe fn zl3073x_out_state_set(
    zldev: *mut zl3073x_dev,
    index: u8,
    out: *const zl3073x_out,
) -> i32 {
    let dout = &mut (*zldev).out[index as usize];
    let out = &*out;
    let mut rc: i32;

    // Reject attempts to change invariant fields (set at fetch only)
    if memcmp(
        core::ptr::addr_of!(dout.inv).cast(),
        core::ptr::addr_of!(out.inv).cast(),
        core::mem::size_of_val(&out.inv),
    ) != 0 {
        return -EINVAL;
    }

    // Skip HW write if configuration hasn't changed
    if memcmp(
        core::ptr::addr_of!(dout.cfg).cast(),
        core::ptr::addr_of!(out.cfg).cast(),
        core::mem::size_of_val(&out.cfg),
    ) == 0 {
        return 0;
    }

    // guard(mutex)(&zldev->multiop_lock);

    // Read output configuration into mailbox
    rc = zl3073x_mb_op(
        zldev,
        ZL_REG_OUTPUT_MB_SEM,
        ZL_OUTPUT_MB_SEM_RD,
        ZL_REG_OUTPUT_MB_MASK,
        1u32 << index,
    );
    if rc != 0 {
        return rc;
    }

    // Update mailbox with changed values
    if dout.div != out.div {
        rc = zl3073x_write_u32(zldev, ZL_REG_OUTPUT_DIV, out.div);
    } else {
        rc = 0;
    }
    if rc == 0 && dout.width != out.width {
        rc = zl3073x_write_u32(zldev, ZL_REG_OUTPUT_WIDTH, out.width);
    }
    if rc == 0 && dout.esync_n_period != out.esync_n_period {
        rc = zl3073x_write_u32(zldev, ZL_REG_OUTPUT_ESYNC_PERIOD, out.esync_n_period);
    }
    if rc == 0 && dout.esync_n_width != out.esync_n_width {
        rc = zl3073x_write_u32(zldev, ZL_REG_OUTPUT_ESYNC_WIDTH, out.esync_n_width);
    }
    if rc == 0 && dout.mode != out.mode {
        rc = zl3073x_write_u8(zldev, ZL_REG_OUTPUT_MODE, out.mode);
    }
    if rc == 0 && dout.phase_comp != out.phase_comp {
        rc = zl3073x_write_u32(zldev, ZL_REG_OUTPUT_PHASE_COMP, out.phase_comp);
    }
    if rc != 0 {
        return rc;
    }

    // Commit output configuration
    rc = zl3073x_mb_op(
        zldev,
        ZL_REG_OUTPUT_MB_SEM,
        ZL_OUTPUT_MB_SEM_WR,
        ZL_REG_OUTPUT_MB_MASK,
        1u32 << index,
    );
    if rc != 0 {
        return rc;
    }

    // After successful commit store new state
    dout.cfg = out.cfg;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
