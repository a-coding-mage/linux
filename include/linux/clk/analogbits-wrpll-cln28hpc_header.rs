/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018-2019 SiFive, Inc.
 * Wesley Terpstra
 * Paul Walmsley
 */

/* DIVQ_VALUES: number of valid DIVQ values */
pub const DIVQ_VALUES: usize = 6;

/*
 * Bit definitions for struct wrpll_cfg.flags
 *
 * WRPLL_FLAGS_BYPASS_FLAG: if set, the PLL is either in bypass, or should be
 *	programmed to enter bypass
 * WRPLL_FLAGS_RESET_FLAG: if set, the PLL is in reset
 * WRPLL_FLAGS_INT_FEEDBACK_FLAG: if set, the PLL is configured for internal
 *	feedback mode
 * WRPLL_FLAGS_EXT_FEEDBACK_FLAG: if set, the PLL is configured for external
 *	feedback mode (not yet supported by this driver)
 */
pub const WRPLL_FLAGS_BYPASS_SHIFT: u32 = 0;
pub const WRPLL_FLAGS_BYPASS_MASK: u8 = 1u8 << WRPLL_FLAGS_BYPASS_SHIFT;
pub const WRPLL_FLAGS_RESET_SHIFT: u32 = 1;
pub const WRPLL_FLAGS_RESET_MASK: u8 = 1u8 << WRPLL_FLAGS_RESET_SHIFT;
pub const WRPLL_FLAGS_INT_FEEDBACK_SHIFT: u32 = 2;
pub const WRPLL_FLAGS_INT_FEEDBACK_MASK: u8 = 1u8 << WRPLL_FLAGS_INT_FEEDBACK_SHIFT;
pub const WRPLL_FLAGS_EXT_FEEDBACK_SHIFT: u32 = 3;
pub const WRPLL_FLAGS_EXT_FEEDBACK_MASK: u8 = 1u8 << WRPLL_FLAGS_EXT_FEEDBACK_SHIFT;

/**
 * struct wrpll_cfg - WRPLL configuration values
 * @divr: reference divider value (6 bits), as presented to the PLL signals
 * @divf: feedback divider value (9 bits), as presented to the PLL signals
 * @divq: output divider value (3 bits), as presented to the PLL signals
 * @flags: PLL configuration flags.  See above for more information
 * @range: PLL loop filter range.  See below for more information
 * @output_rate_cache: cached output rates, swept across DIVQ
 * @parent_rate: PLL refclk rate for which values are valid
 * @max_r: maximum possible R divider value, given @parent_rate
 * @init_r: initial R divider value to start the search from
 *
 * @divr, @divq, @divq, @range represent what the PLL expects to see
 * on its input signals.  Thus @divr and @divf are the actual divisors
 * minus one.  @divq is a power-of-two divider; for example, 1 =
 * divide-by-2 and 6 = divide-by-64.  0 is an invalid @divq value.
 *
 * When initially passing a struct wrpll_cfg record, the
 * record should be zero-initialized with the exception of the @flags
 * field.  The only flag bits that need to be set are either
 * WRPLL_FLAGS_INT_FEEDBACK or WRPLL_FLAGS_EXT_FEEDBACK.
 */
#[repr(C)]
pub struct wrpll_cfg {
    pub divr: u8,
    pub divq: u8,
    pub range: u8,
    pub flags: u8,
    pub divf: u16,
    /* private: */
    pub output_rate_cache: [u32; DIVQ_VALUES],
    pub parent_rate: usize,
    pub max_r: u8,
    pub init_r: u8,
}

extern "C" {
    pub fn wrpll_configure_for_rate(
        c: *mut wrpll_cfg,
        target_rate: u32,
        parent_rate: usize,
    ) -> ::core::ffi::c_int;

    pub fn wrpll_calc_max_lock_us(c: *const wrpll_cfg) -> ::core::ffi::c_uint;

    pub fn wrpll_calc_output_rate(c: *const wrpll_cfg, parent_rate: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
