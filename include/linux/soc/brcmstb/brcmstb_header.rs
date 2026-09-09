/* SPDX-License-Identifier: GPL-2.0 */

/// Return the Broadcom SoC identifier portion of a register value.
#[inline]
pub const fn BRCM_ID(reg: u32) -> u32 {
    if reg >> 28 != 0 {
        reg >> 16
    } else {
        reg >> 8
    }
}

#[inline]
pub const fn BRCM_REV(reg: u32) -> u32 {
    reg & 0xff
}

/* Equivalent to the source's IS_ENABLED(CONFIG_SOC_BRCMSTB) condition.
 * Configure `soc_brcmstb` when CONFIG_SOC_BRCMSTB is enabled. */
#[cfg(soc_brcmstb)]
extern "C" {
    pub fn brcmstb_get_family_id() -> u32;
    pub fn brcmstb_get_product_id() -> u32;
}

#[cfg(not(soc_brcmstb))]
#[inline]
pub const fn brcmstb_get_family_id() -> u32 {
    0
}

#[cfg(not(soc_brcmstb))]
#[inline]
pub const fn brcmstb_get_product_id() -> u32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
