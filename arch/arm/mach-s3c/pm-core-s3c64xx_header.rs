/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      Ben Dooks <ben@simtec.co.uk>
 *      http://armlinux.simtec.co.uk/
 *
 * S3C64XX - PM core support for arch/arm/plat-s3c/pm.c
 */

// Dependencies supplied by the translated Linux platform headers:
// linux/serial_s3c.h, linux/delay.h, regs-gpio.h, regs-clock.h, and map.h.

extern "C" {
    fn __raw_readl(addr: u32) -> u32;
    fn __raw_writel(value: u32, addr: u32);
}

pub unsafe fn s3c_pm_debug_init_uart() {
}

pub unsafe fn s3c_pm_arch_prepare_irqs() {
    /* VIC should have already been taken care of */

    /* clear any pending EINT0 interrupts */
    __raw_writel(__raw_readl(S3C64XX_EINT0PEND), S3C64XX_EINT0PEND);
}

pub unsafe fn s3c_pm_arch_stop_clocks() {
}

pub unsafe fn s3c_pm_arch_show_resume_irqs() {
}

/* make these defines, we currently do not have any need to change
 * the IRQ wake controls depending on the CPU we are running on */
#[cfg(feature = "CONFIG_PM_SLEEP")]
pub const s3c_irqwake_eintallow: u32 = (1u32 << 28) - 1;
#[cfg(feature = "CONFIG_PM_SLEEP")]
pub const s3c_irqwake_intallow: u32 = !0u32;
#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
pub const s3c_irqwake_eintallow: u32 = 0;
#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
pub const s3c_irqwake_intallow: u32 = 0;

pub unsafe fn s3c_pm_restored_gpios() {
    /* ensure sleep mode has been cleared from the system */

    __raw_writel(0, S3C64XX_SLPEN);
}

pub unsafe fn samsung_pm_saved_gpios() {
    /* turn on the sleep mode and keep it there, as it seems that during
     * suspend the xCON registers get re-set and thus you can end up with
     * problems between going to sleep and resuming.
     */

    __raw_writel(S3C64XX_SLPEN_USE_xSLP, S3C64XX_SLPEN);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
