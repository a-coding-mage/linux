// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 DENX Software Engineering, GmbH
 *
 * Pulled from code:
 * Portions copyright (C) 2003 Russell King, PXA MMCI Driver
 * Portions copyright (C) 2004-2005 Pierre Ossman, W83L51xD SD/MMC driver
 *
 * Copyright 2008 Embedded Alley Solutions, Inc.
 * Copyright 2009-2011 Freescale Semiconductor, Inc.
 */

pub unsafe fn mxs_ssp_set_clk_rate(ssp: *mut mxs_ssp, rate: u32) {
    let mut ssp_clk: u32;
    let mut ssp_sck: u32;
    let mut clock_divide: u32;
    let mut clock_rate: u32;
    let mut val: u32;

    ssp_clk = clk_get_rate((*ssp).clk);

    clock_divide = 2;
    loop {
        clock_rate = ssp_clk.div_ceil(rate.wrapping_mul(clock_divide));
        clock_rate = if clock_rate > 0 { clock_rate - 1 } else { 0 };
        if clock_rate <= 255 {
            break;
        }
        if clock_divide > 254 {
            break;
        }
        clock_divide = clock_divide.wrapping_add(2);
    }

    if clock_divide > 254 {
        dev_err(
            (*ssp).dev,
            "%s: cannot set clock to %d\n\0",
            "mxs_ssp_set_clk_rate\0".as_ptr(),
            rate,
        );
        return;
    }

    ssp_sck = ssp_clk / clock_divide / (1 + clock_rate);

    val = readl((*ssp).base.add(HW_SSP_TIMING(ssp)));
    val &= !(BM_SSP_TIMING_CLOCK_DIVIDE | BM_SSP_TIMING_CLOCK_RATE);
    val |= BF_SSP(clock_divide, TIMING_CLOCK_DIVIDE);
    val |= BF_SSP(clock_rate, TIMING_CLOCK_RATE);
    writel(val, (*ssp).base.add(HW_SSP_TIMING(ssp)));

    (*ssp).clk_rate = ssp_sck;

    dev_dbg(
        (*ssp).dev,
        "%s: clock_divide %d, clock_rate %d, ssp_clk %d, rate_actual %d, rate_requested %d\n\0",
        "mxs_ssp_set_clk_rate\0".as_ptr(),
        clock_divide,
        clock_rate,
        ssp_clk,
        ssp_sck,
        rate,
    );
}

// EXPORT_SYMBOL_GPL(mxs_ssp_set_clk_rate);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
