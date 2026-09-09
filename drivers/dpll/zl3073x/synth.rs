// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the corresponding kernel and driver headers.

/**
 * zl3073x_synth_state_fetch - fetch synth state from hardware
 * @zldev: pointer to zl3073x_dev structure
 * @index: synth index to fetch state for
 *
 * Function fetches state of the given synthesizer from the hardware and
 * stores it for later use.
 *
 * Return: 0 on success, <0 on error
 */
pub unsafe fn zl3073x_synth_state_fetch(
    zldev: *mut zl3073x_dev,
    index: u8,
) -> i32 {
    let synth: *mut zl3073x_synth = &mut (*zldev).synth[index as usize];
    let mut rc: i32;

    /* Read synth control register */
    rc = zl3073x_read_u8(zldev, ZL_REG_SYNTH_CTRL(index), &mut (*synth).ctrl);
    if rc != 0 {
        return rc;
    }

    // C: guard(mutex)(&zldev->multiop_lock);
    // The mutex guard is supplied by the surrounding driver infrastructure.

    /* Read synth configuration */
    rc = zl3073x_mb_op(
        zldev,
        ZL_REG_SYNTH_MB_SEM,
        ZL_SYNTH_MB_SEM_RD,
        ZL_REG_SYNTH_MB_MASK,
        1u32 << index,
    );
    if rc != 0 {
        return rc;
    }

    /* The output frequency is determined by the following formula:
     * base * multiplier * numerator / denominator
     *
     * Read registers with these values
     */
    rc = zl3073x_read_u16(zldev, ZL_REG_SYNTH_FREQ_BASE, &mut (*synth).freq_base);
    if rc != 0 {
        return rc;
    }

    rc = zl3073x_read_u32(zldev, ZL_REG_SYNTH_FREQ_MULT, &mut (*synth).freq_mult);
    if rc != 0 {
        return rc;
    }

    rc = zl3073x_read_u16(zldev, ZL_REG_SYNTH_FREQ_M, &mut (*synth).freq_m);
    if rc != 0 {
        return rc;
    }

    rc = zl3073x_read_u16(zldev, ZL_REG_SYNTH_FREQ_N, &mut (*synth).freq_n);
    if rc != 0 {
        return rc;
    }

    /* Check denominator for zero to avoid div by 0 */
    if (*synth).freq_n == 0 {
        dev_err(
            (*zldev).dev,
            "Zero divisor for SYNTH%u retrieved from device\n",
            index,
        );
        return -EINVAL;
    }

    dev_dbg(
        (*zldev).dev,
        "SYNTH%u frequency: %u Hz\n",
        index,
        zl3073x_synth_freq_get(synth),
    );

    rc
}

/**
 * zl3073x_synth_state_get - get current synth state
 * @zldev: pointer to zl3073x_dev structure
 * @index: synth index to get state for
 *
 * Return: pointer to given synth state
 */
pub unsafe fn zl3073x_synth_state_get(
    zldev: *mut zl3073x_dev,
    index: u8,
) -> *const zl3073x_synth {
    &(*zldev).synth[index as usize] as *const zl3073x_synth
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
