/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-rpc/include/mach/acornfb.h
 *
 *  Copyright (C) 1999 Russell King
 *
 *  AcornFB architecture specific code
 */

// Dependency intent: fb_var_screeninfo, vidc_timing, current_par, and the
// VIDC20_* constants are supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn acornfb_bandwidth(var: *const fb_var_screeninfo) -> u_long {
    (*var).pixclock * 8 / (*var).bits_per_pixel
}

#[inline]
pub unsafe fn acornfb_valid_pixrate(var: *mut fb_var_screeninfo) -> i32 {
    let limit: u_long;

    if (*var).pixclock == 0 {
        return 0;
    }

    /*
     * Limits below are taken from RISC OS bandwidthlimit file
     */
    if current_par.using_vram {
        if current_par.vram_half_sam == 2048 {
            limit = 6578;
        } else {
            limit = 13157;
        }
    } else {
        limit = 26315;
    }

    if acornfb_bandwidth(var) >= limit { 1 } else { 0 }
}

/*
 * Try to find the best PLL parameters for the pixel clock.
 * This algorithm seems to give best predictable results,
 * and produces the same values as detailed in the VIDC20
 * data sheet.
 */
#[inline]
pub fn acornfb_vidc20_find_pll(pixclk: u_int) -> u_int {
    let mut best_r: u_int = 2;
    let mut best_v: u_int = 2;
    let mut best_d: i32 = 0x7fffffff;
    let mut r: u_int = 2;

    while r <= 32 {
        let rr: u_int = 41667 * r;
        let v: u_int = (rr + pixclk / 2) / pixclk;

        if v > 32 || v < 2 {
            r += 1;
            continue;
        }

        let p: u_int = (rr + v / 2) / v;
        let mut d: i32 = pixclk as i32 - p as i32;

        if d < 0 {
            d = -d;
        }

        if d < best_d {
            best_d = d;
            best_v = v - 1;
            best_r = r - 1;
        }

        if d == 0 {
            break;
        }
        r += 1;
    }

    best_v << 8 | best_r
}

#[inline]
pub unsafe fn acornfb_vidc20_find_rates(
    vidc: *mut vidc_timing,
    var: *mut fb_var_screeninfo,
) {
    /* Select pixel-clock divisor to keep PLL in range */
    let mut div: u_int = (*var).pixclock / 9090; /*9921*/

    /* Limit divisor */
    if div == 0 {
        div = 1;
    }
    if div > 8 {
        div = 8;
    }

    /* Encode divisor to VIDC20 setting */
    match div {
        1 => (*vidc).control |= VIDC20_CTRL_PIX_CK,
        2 => (*vidc).control |= VIDC20_CTRL_PIX_CK2,
        3 => (*vidc).control |= VIDC20_CTRL_PIX_CK3,
        4 => (*vidc).control |= VIDC20_CTRL_PIX_CK4,
        5 => (*vidc).control |= VIDC20_CTRL_PIX_CK5,
        6 => (*vidc).control |= VIDC20_CTRL_PIX_CK6,
        7 => (*vidc).control |= VIDC20_CTRL_PIX_CK7,
        8 => (*vidc).control |= VIDC20_CTRL_PIX_CK8,
        _ => {}
    }

    /*
     * With VRAM, the FIFO can be set to the highest possible setting
     * because there are no latency considerations for other memory
     * accesses. However, in 64 bit bus mode the FIFO preload value
     * must not be set to VIDC20_CTRL_FIFO_28 because this will let
     * the FIFO overflow. See VIDC20 manual page 33 (6.0 Setting the
     * FIFO preload value).
     */
    if current_par.using_vram {
        if current_par.vram_half_sam == 2048 {
            (*vidc).control |= VIDC20_CTRL_FIFO_24;
        } else {
            (*vidc).control |= VIDC20_CTRL_FIFO_28;
        }
    } else {
        let bandwidth: u_long = acornfb_bandwidth(var);

        /* Encode bandwidth as VIDC20 setting */
        if bandwidth > 33334 { /* < 30.0MB/s */
            (*vidc).control |= VIDC20_CTRL_FIFO_16;
        } else if bandwidth > 26666 { /* < 37.5MB/s */
            (*vidc).control |= VIDC20_CTRL_FIFO_20;
        } else if bandwidth > 22222 { /* < 45.0MB/s */
            (*vidc).control |= VIDC20_CTRL_FIFO_24;
        } else { /* > 45.0MB/s */
            (*vidc).control |= VIDC20_CTRL_FIFO_28;
        }
    }

    /* Find the PLL values */
    (*vidc).pll_ctl = acornfb_vidc20_find_pll((*var).pixclock / div);
}

#[inline]
pub const fn acornfb_default_control() -> u_int {
    VIDC20_CTRL_PIX_VCLK
}

#[inline]
pub const fn acornfb_default_econtrol() -> u_int {
    VIDC20_ECTL_DAC | VIDC20_ECTL_REG(3)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
