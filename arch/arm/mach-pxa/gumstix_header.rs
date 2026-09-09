/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-pxa/include/mach/gumstix.h
 */

/* Dependency supplied by irqs.h: PXA_GPIO_TO_IRQ, GPIO_IN, and GPIO_OUT. */

/* BTRESET - Reset line to Bluetooth module, active low signal. */
pub const GPIO_GUMSTIX_BTRESET: i32 = 7;
pub const GPIO_GUMSTIX_BTRESET_MD: i32 = GPIO_GUMSTIX_BTRESET | GPIO_OUT;

/*
GPIOn - Input from MAX823 (or equiv), normalizing USB +5V into a clean
interrupt signal for determining cable presence. On the gumstix F,
this moves to GPIO17 and GPIO37.
*/

/* GPIOx - Connects to USB D+ and used as a pull-up after GPIOn
has detected a cable insertion; driven low otherwise. */

pub const GPIO_GUMSTIX_USB_GPIOn: i32 = 35;
pub const GPIO_GUMSTIX_USB_GPIOx: i32 = 41;

/* usb state change */
pub const GUMSTIX_USB_INTR_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO_GUMSTIX_USB_GPIOn);

pub const GPIO_GUMSTIX_USB_GPIOn_MD: i32 = GPIO_GUMSTIX_USB_GPIOn | GPIO_IN;
pub const GPIO_GUMSTIX_USB_GPIOx_CON_MD: i32 = GPIO_GUMSTIX_USB_GPIOx | GPIO_OUT;
pub const GPIO_GUMSTIX_USB_GPIOx_DIS_MD: i32 = GPIO_GUMSTIX_USB_GPIOx | GPIO_IN;

/*
 * SD/MMC definitions
 */
pub const GUMSTIX_GPIO_nSD_WP: i32 = 22; /* SD Write Protect */
pub const GUMSTIX_GPIO_nSD_DETECT: i32 = 11; /* MMC/SD Card Detect */
pub const GUMSTIX_IRQ_GPIO_nSD_DETECT: i32 = PXA_GPIO_TO_IRQ(GUMSTIX_GPIO_nSD_DETECT);

/*
 * SMC Ethernet definitions
 * ETH_RST provides a hardware reset line to the ethernet chip
 * ETH is the IRQ line in from the ethernet chip to the PXA
 */
pub const GPIO_GUMSTIX_ETH0_RST: i32 = 80;
pub const GPIO_GUMSTIX_ETH0_RST_MD: i32 = GPIO_GUMSTIX_ETH0_RST | GPIO_OUT;
pub const GPIO_GUMSTIX_ETH1_RST: i32 = 52;
pub const GPIO_GUMSTIX_ETH1_RST_MD: i32 = GPIO_GUMSTIX_ETH1_RST | GPIO_OUT;

pub const GPIO_GUMSTIX_ETH0: i32 = 36;
pub const GPIO_GUMSTIX_ETH0_MD: i32 = GPIO_GUMSTIX_ETH0 | GPIO_IN;
pub const GUMSTIX_ETH0_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO_GUMSTIX_ETH0);
pub const GPIO_GUMSTIX_ETH1: i32 = 27;
pub const GPIO_GUMSTIX_ETH1_MD: i32 = GPIO_GUMSTIX_ETH1 | GPIO_IN;
pub const GUMSTIX_ETH1_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO_GUMSTIX_ETH1);

/* CF reset line */
pub const GPIO8_RESET: i32 = 8;

/* CF slot 0 */
pub const GPIO4_nBVD1: i32 = 4;
pub const GPIO4_nSTSCHG: i32 = GPIO4_nBVD1;
pub const GPIO11_nCD: i32 = 11;
pub const GPIO26_PRDY_nBSY: i32 = 26;
pub const GUMSTIX_S0_nSTSCHG_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO4_nSTSCHG);
pub const GUMSTIX_S0_nCD_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO11_nCD);
pub const GUMSTIX_S0_PRDY_nBSY_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO26_PRDY_nBSY);

/* CF slot 1 */
pub const GPIO18_nBVD1: i32 = 18;
pub const GPIO18_nSTSCHG: i32 = GPIO18_nBVD1;
pub const GPIO36_nCD: i32 = 36;
pub const GPIO27_PRDY_nBSY: i32 = 27;
pub const GUMSTIX_S1_nSTSCHG_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO18_nSTSCHG);
pub const GUMSTIX_S1_nCD_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO36_nCD);
pub const GUMSTIX_S1_PRDY_nBSY_IRQ: i32 = PXA_GPIO_TO_IRQ(GPIO27_PRDY_nBSY);

/* CF GPIO line modes */
pub const GPIO4_nSTSCHG_MD: i32 = GPIO4_nSTSCHG | GPIO_IN;
pub const GPIO8_RESET_MD: i32 = GPIO8_RESET | GPIO_OUT;
pub const GPIO11_nCD_MD: i32 = GPIO11_nCD | GPIO_IN;
pub const GPIO18_nSTSCHG_MD: i32 = GPIO18_nSTSCHG | GPIO_IN;
pub const GPIO26_PRDY_nBSY_MD: i32 = GPIO26_PRDY_nBSY | GPIO_IN;
pub const GPIO27_PRDY_nBSY_MD: i32 = GPIO27_PRDY_nBSY | GPIO_IN;
pub const GPIO36_nCD_MD: i32 = GPIO36_nCD | GPIO_IN;

/* for expansion boards that can't be programatically detected */
extern "C" {
    pub fn am200_init() -> i32;
    pub fn am300_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
