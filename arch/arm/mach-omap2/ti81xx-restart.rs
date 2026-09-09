// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the surrounding kernel translation:
// iomap.h, common.h, control.h, and prm3xxx.h.

use core::ffi::c_char;

const TI81XX_PRM_DEVICE_RSTCTRL: u32 = 0x00a0;
const TI81XX_GLOBAL_RST_COLD: u32 = 1 << 1;

unsafe extern "C" {
    fn omap2_prm_set_mod_reg_bits(bits: u32, module: u32, offset: u32);
}

/**
 * ti81xx_restart - trigger a software restart of the SoC
 * @mode: the "reboot mode", see arch/arm/kernel/{setup,process}.c
 * @cmd: passed from the userspace program rebooting the system (if provided)
 *
 * Resets the SoC.  For @cmd, see the 'reboot' syscall in
 * kernel/sys.c.  No return value.
 *
 * NOTE: Warm reset does not seem to work, may require resetting
 * clocks to bypass mode.
 */
pub unsafe fn ti81xx_restart(mode: reboot_mode, cmd: *const c_char) {
    let _ = mode;
    let _ = cmd;

    unsafe {
        omap2_prm_set_mod_reg_bits(
            TI81XX_GLOBAL_RST_COLD,
            0,
            TI81XX_PRM_DEVICE_RSTCTRL,
        );
    }
    loop {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
