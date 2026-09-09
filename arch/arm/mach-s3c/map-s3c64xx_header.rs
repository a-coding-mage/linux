/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C64XX - Memory map definitions
 */

// C dependencies: "map-base.h" and "map-s3c.h".

/*
 * Post-mux Chip Select Regions Xm0CSn_
 * These may be used by SROM, NAND or CF depending on settings
 */

pub const S3C64XX_PA_XM0CSN0: u32 = 0x10000000;
pub const S3C64XX_PA_XM0CSN1: u32 = 0x18000000;
pub const S3C64XX_PA_XM0CSN2: u32 = 0x20000000;
pub const S3C64XX_PA_XM0CSN3: u32 = 0x28000000;
pub const S3C64XX_PA_XM0CSN4: u32 = 0x30000000;
pub const S3C64XX_PA_XM0CSN5: u32 = 0x38000000;

/* HSMMC units */
pub const fn S3C64XX_PA_HSMMC(x: u32) -> u32 { 0x7C200000u32.wrapping_add(x.wrapping_mul(0x100000)) }
pub const S3C64XX_PA_HSMMC0: u32 = S3C64XX_PA_HSMMC(0);
pub const S3C64XX_PA_HSMMC1: u32 = S3C64XX_PA_HSMMC(1);
pub const S3C64XX_PA_HSMMC2: u32 = S3C64XX_PA_HSMMC(2);

pub const S3C_PA_UART: u32 = 0x7F005000;
pub const S3C_PA_UART0: u32 = S3C_PA_UART + 0x00;
pub const S3C_PA_UART1: u32 = S3C_PA_UART + 0x400;
pub const S3C_PA_UART2: u32 = S3C_PA_UART + 0x800;
pub const S3C_PA_UART3: u32 = S3C_PA_UART + 0xC00;
pub const S3C_UART_OFFSET: u32 = 0x400;

/* See notes on UART VA mapping in debug-macro.S */
pub const fn S3C_VA_UARTx(x: u32) -> u32 {
    S3C_VA_UART + (S3C_PA_UART & 0xfffff) + x * S3C_UART_OFFSET
}

pub const S3C_VA_UART0: u32 = S3C_VA_UARTx(0);
pub const S3C_VA_UART1: u32 = S3C_VA_UARTx(1);
pub const S3C_VA_UART2: u32 = S3C_VA_UARTx(2);
pub const S3C_VA_UART3: u32 = S3C_VA_UARTx(3);

pub const S3C64XX_PA_SROM: u32 = 0x70000000;
pub const S3C64XX_PA_ONENAND0: u32 = 0x70100000;
pub const S3C64XX_PA_ONENAND0_BUF: u32 = 0x20000000;
pub const S3C64XX_SZ_ONENAND0_BUF: u32 = SZ_64M;

/* NAND and OneNAND1 controllers occupy the same register region
   (depending on SoC POP version) */
pub const S3C64XX_PA_ONENAND1: u32 = 0x70200000;
pub const S3C64XX_PA_ONENAND1_BUF: u32 = 0x28000000;
pub const S3C64XX_SZ_ONENAND1_BUF: u32 = SZ_64M;

pub const S3C64XX_PA_NAND: u32 = 0x70200000;
pub const S3C64XX_PA_FB: u32 = 0x77100000;
pub const S3C64XX_PA_USB_HSOTG: u32 = 0x7C000000;
pub const S3C64XX_PA_WATCHDOG: u32 = 0x7E004000;
pub const S3C64XX_PA_RTC: u32 = 0x7E005000;
pub const S3C64XX_PA_KEYPAD: u32 = 0x7E00A000;
pub const S3C64XX_PA_ADC: u32 = 0x7E00B000;
pub const S3C64XX_PA_SYSCON: u32 = 0x7E00F000;
pub const S3C64XX_PA_AC97: u32 = 0x7F001000;
pub const S3C64XX_PA_IIS0: u32 = 0x7F002000;
pub const S3C64XX_PA_IIS1: u32 = 0x7F003000;
pub const S3C64XX_PA_TIMER: u32 = 0x7F006000;
pub const S3C64XX_PA_IIC0: u32 = 0x7F004000;
pub const S3C64XX_PA_SPI0: u32 = 0x7F00B000;
pub const S3C64XX_PA_SPI1: u32 = 0x7F00C000;
pub const S3C64XX_PA_PCM0: u32 = 0x7F009000;
pub const S3C64XX_PA_PCM1: u32 = 0x7F00A000;
pub const S3C64XX_PA_IISV4: u32 = 0x7F00D000;
pub const S3C64XX_PA_IIC1: u32 = 0x7F00F000;

pub const S3C64XX_PA_GPIO: u32 = 0x7F008000;
pub const S3C64XX_SZ_GPIO: u32 = SZ_4K;
pub const S3C64XX_PA_SDRAM: u32 = 0x50000000;
pub const S3C64XX_PA_CFCON: u32 = 0x70300000;
pub const S3C64XX_PA_VIC0: u32 = 0x71200000;
pub const S3C64XX_PA_VIC1: u32 = 0x71300000;
pub const S3C64XX_PA_MODEM: u32 = 0x74108000;
pub const S3C64XX_PA_USBHOST: u32 = 0x74300000;
pub const S3C64XX_PA_USB_HSPHY: u32 = 0x7C100000;

/* compatibility defines. */
pub const S3C_PA_TIMER: u32 = S3C64XX_PA_TIMER;
pub const S3C_PA_HSMMC0: u32 = S3C64XX_PA_HSMMC0;
pub const S3C_PA_HSMMC1: u32 = S3C64XX_PA_HSMMC1;
pub const S3C_PA_HSMMC2: u32 = S3C64XX_PA_HSMMC2;
pub const S3C_PA_IIC: u32 = S3C64XX_PA_IIC0;
pub const S3C_PA_IIC1: u32 = S3C64XX_PA_IIC1;
pub const S3C_PA_NAND: u32 = S3C64XX_PA_NAND;
pub const S3C_PA_ONENAND: u32 = S3C64XX_PA_ONENAND0;
pub const S3C_PA_ONENAND_BUF: u32 = S3C64XX_PA_ONENAND0_BUF;
pub const S3C_SZ_ONENAND_BUF: u32 = S3C64XX_SZ_ONENAND0_BUF;
pub const S3C_PA_FB: u32 = S3C64XX_PA_FB;
pub const S3C_PA_USBHOST: u32 = S3C64XX_PA_USBHOST;
pub const S3C_PA_USB_HSOTG: u32 = S3C64XX_PA_USB_HSOTG;
pub const S3C_PA_RTC: u32 = S3C64XX_PA_RTC;
pub const S3C_PA_WDT: u32 = S3C64XX_PA_WATCHDOG;
pub const S3C_PA_SPI0: u32 = S3C64XX_PA_SPI0;
pub const S3C_PA_SPI1: u32 = S3C64XX_PA_SPI1;

pub const SAMSUNG_PA_ADC: u32 = S3C64XX_PA_ADC;
pub const SAMSUNG_PA_CFCON: u32 = S3C64XX_PA_CFCON;
pub const SAMSUNG_PA_KEYPAD: u32 = S3C64XX_PA_KEYPAD;
pub const SAMSUNG_PA_TIMER: u32 = S3C64XX_PA_TIMER;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
