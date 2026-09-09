// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding build: linux/stddef.h,
// linux/kbuild.h, and pm.h.

// Build-time condition and kbuild DEFINE/offsetof facilities are preserved
// here as their Rust equivalents; `at91_pm_data` is supplied by pm.h.

#[allow(non_camel_case_types)]
use crate::at91_pm_data;

#[allow(dead_code)]
fn main() -> i32 {
    DEFINE!(PM_DATA_PMC, core::mem::offset_of!(at91_pm_data, pmc));
    DEFINE!(PM_DATA_RAMC0, core::mem::offset_of!(at91_pm_data, ramc[0]));
    DEFINE!(PM_DATA_RAMC1, core::mem::offset_of!(at91_pm_data, ramc[1]));
    DEFINE!(
        PM_DATA_RAMC_PHY,
        core::mem::offset_of!(at91_pm_data, ramc_phy)
    );
    DEFINE!(PM_DATA_MEMCTRL, core::mem::offset_of!(at91_pm_data, memctrl));
    DEFINE!(PM_DATA_MODE, core::mem::offset_of!(at91_pm_data, mode));
    DEFINE!(PM_DATA_SHDWC, core::mem::offset_of!(at91_pm_data, shdwc));
    DEFINE!(PM_DATA_SFRBU, core::mem::offset_of!(at91_pm_data, sfrbu));
    DEFINE!(
        PM_DATA_PMC_MCKR_OFFSET,
        core::mem::offset_of!(at91_pm_data, pmc_mckr_offset)
    );
    DEFINE!(
        PM_DATA_PMC_VERSION,
        core::mem::offset_of!(at91_pm_data, pmc_version)
    );
    DEFINE!(PM_DATA_PMC_MCKS, core::mem::offset_of!(at91_pm_data, pmc_mcks));

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
