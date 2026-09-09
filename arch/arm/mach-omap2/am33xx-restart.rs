// SPDX-License-Identifier: GPL-2.0-only
/*
 * am33xx-restart.c - Code common to all AM33xx machines.
 */

// Dependency declarations supplied by the surrounding kernel translation.
pub const AM335X_PIN_EMU0: u32 = 0;
pub const AM335X_PIN_EMU1: u32 = 0;

pub type reboot_mode = core::ffi::c_int;

extern "C" {
    fn omap_ctrl_readl(reg: u32) -> u32;
    fn omap_ctrl_writel(val: u32, reg: u32);
    fn mdelay(msecs: u32);
    static mut prm_reboot_mode: reboot_mode;
    fn omap_prm_reset_system();
}

/*
 * Advisory 1.0.36 EMU0 and EMU1: Terminals Must be Pulled High Before
 * ICEPick Samples
 *
 * If EMU0/EMU1 pins have been used as GPIO outputs and actively driving low
 * level, the device might not reboot in normal mode. We are in a bad position
 * to override GPIO state here, so just switch the pins into EMU input mode
 * (that's what reset will do anyway) and wait a bit, because the state will be
 * latched 190 ns after reset.
 */
unsafe fn am33xx_advisory_1_0_36() {
    let emu0: u32 = omap_ctrl_readl(AM335X_PIN_EMU0);
    let emu1: u32 = omap_ctrl_readl(AM335X_PIN_EMU1);

    /* If both pins are in EMU mode, nothing to do */
    if (emu0 & 7 == 0 && emu1 & 7 == 0) {
        return;
    }

    /* Switch GPIO3_7/GPIO3_8 into EMU0/EMU1 modes respectively */
    omap_ctrl_writel(emu0 & !7, AM335X_PIN_EMU0);
    omap_ctrl_writel(emu1 & !7, AM335X_PIN_EMU1);

    /*
     * Give pull-ups time to load the pin/PCB trace capacity.
     * 5 ms shall be enough to load 1 uF (would be huge capacity for these
     * pins) with TI-recommended 4k7 external pull-ups.
     */
    mdelay(5);
}

/**
 * am33xx_restart - trigger a software restart of the SoC
 * @mode: the "reboot mode", see arch/arm/kernel/{setup,process}.c
 * @cmd: passed from the userspace program rebooting the system (if provided)
 *
 * Resets the SoC.  For @cmd, see the 'reboot' syscall in
 * kernel/sys.c.  No return value.
 */
pub unsafe fn am33xx_restart(mode: reboot_mode, _cmd: *const core::ffi::c_char) {
    am33xx_advisory_1_0_36();

    /* TODO: Handle cmd if necessary */
    prm_reboot_mode = mode;

    omap_prm_reset_system();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
