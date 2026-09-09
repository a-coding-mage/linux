/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This supports machine-specific differences in how the PXA2xx
 * USB Device Controller (UDC) is wired.
 *
 * It is set in linux/arch/arm/mach-pxa/<machine>.c or in
 * linux/arch/mach-ixp4xx/<machine>.c and used in
 * the probe routine of linux/drivers/usb/gadget/pxa2xx_udc.c
 */

/* CONFIG_PXA27x conditional declaration. */
#[cfg(CONFIG_PXA27x)]
unsafe extern "C" {
    pub fn pxa27x_clear_otgph();
}

#[cfg(not(CONFIG_PXA27x))]
#[inline(always)]
pub fn pxa27x_clear_otgph() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
