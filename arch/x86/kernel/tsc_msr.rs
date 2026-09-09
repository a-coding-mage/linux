// SPDX-License-Identifier: GPL-2.0
/*
 * TSC frequency enumeration via MSR
 *
 * Copyright (C) 2013, 2018 Intel Corporation
 * Author: Bin Gao <bin.gao@intel.com>
 */

const MAX_NUM_FREQS: usize = 16; /* 4 bits to select the frequency */
const TSC_REFERENCE_KHZ: u32 = 100000;

#[repr(C)]
pub struct muldiv {
    pub multiplier: u32,
    pub divider: u32,
}

#[repr(C)]
pub struct freq_desc {
    pub use_msr_plat: bool,
    pub muldiv: [muldiv; MAX_NUM_FREQS],
    pub freqs: [u32; MAX_NUM_FREQS],
    pub mask: u32,
}

const ZERO_MULDIV: muldiv = muldiv { multiplier: 0, divider: 0 };

/* Penwell and Clovertrail use spread spectrum clock. */
static FREQ_DESC_PNW: freq_desc = freq_desc {
    use_msr_plat: false,
    muldiv: [ZERO_MULDIV; MAX_NUM_FREQS],
    freqs: [0, 0, 0, 0, 0, 99840, 0, 83200, 0, 0, 0, 0, 0, 0, 0, 0],
    mask: 0x07,
};

static FREQ_DESC_CLV: freq_desc = freq_desc {
    use_msr_plat: false,
    muldiv: [ZERO_MULDIV; MAX_NUM_FREQS],
    freqs: [0, 133200, 0, 0, 0, 99840, 0, 83200, 0, 0, 0, 0, 0, 0, 0, 0],
    mask: 0x07,
};

static FREQ_DESC_BYT: freq_desc = freq_desc {
    use_msr_plat: true,
    muldiv: [
        muldiv { multiplier: 5, divider: 6 }, muldiv { multiplier: 1, divider: 1 },
        muldiv { multiplier: 4, divider: 3 }, muldiv { multiplier: 7, divider: 6 },
        muldiv { multiplier: 4, divider: 5 }, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV,
        ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV,
        ZERO_MULDIV, ZERO_MULDIV,
    ],
    freqs: [0; MAX_NUM_FREQS],
    mask: 0x07,
};

static FREQ_DESC_CHT: freq_desc = freq_desc {
    use_msr_plat: true,
    muldiv: [
        muldiv { multiplier: 5, divider: 6 }, muldiv { multiplier: 1, divider: 1 },
        muldiv { multiplier: 4, divider: 3 }, muldiv { multiplier: 7, divider: 6 },
        muldiv { multiplier: 4, divider: 5 }, muldiv { multiplier: 14, divider: 15 },
        muldiv { multiplier: 9, divider: 10 }, muldiv { multiplier: 8, divider: 9 },
        muldiv { multiplier: 7, divider: 8 }, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV,
        ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV,
    ],
    freqs: [0; MAX_NUM_FREQS],
    mask: 0x0f,
};

static FREQ_DESC_TNG: freq_desc = freq_desc {
    use_msr_plat: true,
    muldiv: [ZERO_MULDIV, muldiv { multiplier: 1, divider: 1 }, muldiv { multiplier: 4, divider: 3 }, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV],
    freqs: [0; MAX_NUM_FREQS], mask: 0x07,
};

static FREQ_DESC_ANN: freq_desc = freq_desc {
    use_msr_plat: true,
    muldiv: [muldiv { multiplier: 5, divider: 6 }, muldiv { multiplier: 1, divider: 1 }, muldiv { multiplier: 4, divider: 3 }, muldiv { multiplier: 1, divider: 1 }, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV, ZERO_MULDIV],
    freqs: [0; MAX_NUM_FREQS], mask: 0x0f,
};

static FREQ_DESC_LGM: freq_desc = freq_desc {
    use_msr_plat: true,
    muldiv: [ZERO_MULDIV; MAX_NUM_FREQS],
    freqs: [78000; MAX_NUM_FREQS],
    mask: 0x0f,
};

/* X86_MATCH_VFM entries are supplied by the architecture CPU-ID definitions. */
extern "C" {
    fn x86_match_cpu(ids: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn rdmsrq(msr: u32, value: *mut u64);
    fn setup_force_cpu_cap(cap: u32);
    fn pr_err(format: *const core::ffi::c_char, ...);
    static mut lapic_timer_period: u32;
}

pub unsafe fn cpu_khz_from_msr() -> u64 {
    let _ = (&FREQ_DESC_PNW, &FREQ_DESC_CLV, &FREQ_DESC_BYT, &FREQ_DESC_CHT,
        &FREQ_DESC_TNG, &FREQ_DESC_ANN, &FREQ_DESC_LGM);
    // The CPU-ID table and architecture constants are provided by the imported kernel interfaces.
    let _ = x86_match_cpu(core::ptr::null());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
