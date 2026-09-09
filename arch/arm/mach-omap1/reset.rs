// SPDX-License-Identifier: GPL-2.0
/*
 * OMAP1 reset support
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not implemented here: hardware.h, iomap.h, and common.h.

/* ARM_SYSST bit shifts related to SoC reset sources */
const ARM_SYSST_POR_SHIFT: u32 = 5;
const ARM_SYSST_EXT_RST_SHIFT: u32 = 4;
const ARM_SYSST_ARM_WDRST_SHIFT: u32 = 2;
const ARM_SYSST_GLOB_SWRST_SHIFT: u32 = 1;

/* Standardized reset source bits (across all OMAP SoCs) */
const OMAP_GLOBAL_COLD_RST_SRC_ID_SHIFT: u32 = 0;
const OMAP_GLOBAL_WARM_RST_SRC_ID_SHIFT: u32 = 1;
const OMAP_MPU_WD_RST_SRC_ID_SHIFT: u32 = 3;
const OMAP_EXTWARM_RST_SRC_ID_SHIFT: u32 = 5;

pub unsafe fn omap1_restart(mode: reboot_mode, cmd: *const core::ffi::c_char) {
    /*
     * Workaround for 5912/1611b bug mentioned in sprz209d.pdf p. 28
     * "Global Software Reset Affects Traffic Controller Frequency".
     */
    if cpu_is_omap5912() {
        omap_writew(
            omap_readw(DPLL_CTL) & !(1u16 << 4),
            DPLL_CTL,
        );
        omap_writew(0x8, ARM_RSTCT1);
    }

    omap_writew(1, ARM_RSTCT1);
}

/**
 * omap1_get_reset_sources - return the source of the SoC's last reset
 *
 * Returns bits that represent the last reset source for the SoC.  The
 * format is standardized across OMAPs for use by the OMAP watchdog.
 */
pub unsafe fn omap1_get_reset_sources() -> u32 {
    let mut ret: u32 = 0;
    let rs: u16;

    rs = __raw_readw(OMAP1_IO_ADDRESS(ARM_SYSST));

    if rs & (1u16 << ARM_SYSST_POR_SHIFT) != 0 {
        ret |= 1u32 << OMAP_GLOBAL_COLD_RST_SRC_ID_SHIFT;
    }
    if rs & (1u16 << ARM_SYSST_EXT_RST_SHIFT) != 0 {
        ret |= 1u32 << OMAP_EXTWARM_RST_SRC_ID_SHIFT;
    }
    if rs & (1u16 << ARM_SYSST_ARM_WDRST_SHIFT) != 0 {
        ret |= 1u32 << OMAP_MPU_WD_RST_SRC_ID_SHIFT;
    }
    if rs & (1u16 << ARM_SYSST_GLOB_SWRST_SHIFT) != 0 {
        ret |= 1u32 << OMAP_GLOBAL_WARM_RST_SRC_ID_SHIFT;
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
