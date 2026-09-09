/* SPDX-License-Identifier: GPL-2.0 */
/* linux/arch/arm/mach-s3c64xx/include/mach/irqs.h
 *
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      Ben Dooks <ben@simtec.co.uk>
 *      http://armlinux.simtec.co.uk/
 *
 * S3C64XX - IRQ support
 */

/* The C header guard is omitted in Rust. */

pub const S3C_IRQ_OFFSET: i32 = 32;

#[inline]
pub const fn s3c_irq(x: i32) -> i32 { x + S3C_IRQ_OFFSET }

pub const IRQ_VIC0_BASE: i32 = s3c_irq(0);
pub const IRQ_VIC1_BASE: i32 = s3c_irq(32);

#[inline]
pub const fn s3c64xx_irq_vic0(x: i32) -> i32 { IRQ_VIC0_BASE + x }
#[inline]
pub const fn s3c64xx_irq_vic1(x: i32) -> i32 { IRQ_VIC1_BASE + x }

pub const IRQ_EINT0_3: i32 = s3c64xx_irq_vic0(0);
pub const IRQ_EINT4_11: i32 = s3c64xx_irq_vic0(1);
pub const IRQ_RTC_TIC: i32 = s3c64xx_irq_vic0(2);
pub const IRQ_CAMIF_C: i32 = s3c64xx_irq_vic0(3);
pub const IRQ_CAMIF_P: i32 = s3c64xx_irq_vic0(4);
pub const IRQ_CAMIF_MC: i32 = s3c64xx_irq_vic0(5);
pub const IRQ_S3C6410_IIC1: i32 = s3c64xx_irq_vic0(5);
pub const IRQ_S3C6410_IIS: i32 = s3c64xx_irq_vic0(6);
pub const IRQ_S3C6400_CAMIF_MP: i32 = s3c64xx_irq_vic0(6);
pub const IRQ_CAMIF_WE_C: i32 = s3c64xx_irq_vic0(7);
pub const IRQ_S3C6410_G3D: i32 = s3c64xx_irq_vic0(8);
pub const IRQ_S3C6400_CAMIF_WE_P: i32 = s3c64xx_irq_vic0(8);
pub const IRQ_POST0: i32 = s3c64xx_irq_vic0(9);
pub const IRQ_ROTATOR: i32 = s3c64xx_irq_vic0(10);
pub const IRQ_2D: i32 = s3c64xx_irq_vic0(11);
pub const IRQ_TVENC: i32 = s3c64xx_irq_vic0(12);
pub const IRQ_SCALER: i32 = s3c64xx_irq_vic0(13);
pub const IRQ_BATF: i32 = s3c64xx_irq_vic0(14);
pub const IRQ_JPEG: i32 = s3c64xx_irq_vic0(15);
pub const IRQ_MFC: i32 = s3c64xx_irq_vic0(16);
pub const IRQ_SDMA0: i32 = s3c64xx_irq_vic0(17);
pub const IRQ_SDMA1: i32 = s3c64xx_irq_vic0(18);
pub const IRQ_ARM_DMAERR: i32 = s3c64xx_irq_vic0(19);
pub const IRQ_ARM_DMA: i32 = s3c64xx_irq_vic0(20);
pub const IRQ_ARM_DMAS: i32 = s3c64xx_irq_vic0(21);
pub const IRQ_KEYPAD: i32 = s3c64xx_irq_vic0(22);
pub const IRQ_TIMER0_VIC: i32 = s3c64xx_irq_vic0(23);
pub const IRQ_TIMER1_VIC: i32 = s3c64xx_irq_vic0(24);
pub const IRQ_TIMER2_VIC: i32 = s3c64xx_irq_vic0(25);
pub const IRQ_WDT: i32 = s3c64xx_irq_vic0(26);
pub const IRQ_TIMER3_VIC: i32 = s3c64xx_irq_vic0(27);
pub const IRQ_TIMER4_VIC: i32 = s3c64xx_irq_vic0(28);
pub const IRQ_LCD_FIFO: i32 = s3c64xx_irq_vic0(29);
pub const IRQ_LCD_VSYNC: i32 = s3c64xx_irq_vic0(30);
pub const IRQ_LCD_SYSTEM: i32 = s3c64xx_irq_vic0(31);

pub const IRQ_EINT12_19: i32 = s3c64xx_irq_vic1(0);
pub const IRQ_EINT20_27: i32 = s3c64xx_irq_vic1(1);
pub const IRQ_PCM0: i32 = s3c64xx_irq_vic1(2);
pub const IRQ_PCM1: i32 = s3c64xx_irq_vic1(3);
pub const IRQ_AC97: i32 = s3c64xx_irq_vic1(4);
pub const IRQ_UART0: i32 = s3c64xx_irq_vic1(5);
pub const IRQ_UART1: i32 = s3c64xx_irq_vic1(6);
pub const IRQ_UART2: i32 = s3c64xx_irq_vic1(7);
pub const IRQ_UART3: i32 = s3c64xx_irq_vic1(8);
pub const IRQ_DMA0: i32 = s3c64xx_irq_vic1(9);
pub const IRQ_DMA1: i32 = s3c64xx_irq_vic1(10);
pub const IRQ_ONENAND0: i32 = s3c64xx_irq_vic1(11);
pub const IRQ_ONENAND1: i32 = s3c64xx_irq_vic1(12);
pub const IRQ_NFC: i32 = s3c64xx_irq_vic1(13);
pub const IRQ_CFCON: i32 = s3c64xx_irq_vic1(14);
pub const IRQ_USBH: i32 = s3c64xx_irq_vic1(15);
pub const IRQ_SPI0: i32 = s3c64xx_irq_vic1(16);
pub const IRQ_SPI1: i32 = s3c64xx_irq_vic1(17);
pub const IRQ_IIC: i32 = s3c64xx_irq_vic1(18);
pub const IRQ_HSItx: i32 = s3c64xx_irq_vic1(19);
pub const IRQ_HSIrx: i32 = s3c64xx_irq_vic1(20);
pub const IRQ_RESERVED: i32 = s3c64xx_irq_vic1(21);
pub const IRQ_MSM: i32 = s3c64xx_irq_vic1(22);
pub const IRQ_HOSTIF: i32 = s3c64xx_irq_vic1(23);
pub const IRQ_HSMMC0: i32 = s3c64xx_irq_vic1(24);
pub const IRQ_HSMMC1: i32 = s3c64xx_irq_vic1(25);
pub const IRQ_HSMMC2: i32 = IRQ_SPI1;
pub const IRQ_OTG: i32 = s3c64xx_irq_vic1(26);
pub const IRQ_IRDA: i32 = s3c64xx_irq_vic1(27);
pub const IRQ_RTC_ALARM: i32 = s3c64xx_irq_vic1(28);
pub const IRQ_SEC: i32 = s3c64xx_irq_vic1(29);
pub const IRQ_PENDN: i32 = s3c64xx_irq_vic1(30);
pub const IRQ_TC: i32 = IRQ_PENDN;
pub const IRQ_ADC: i32 = s3c64xx_irq_vic1(31);

pub const IRQ_IIC1: i32 = IRQ_S3C6410_IIC1;

pub const S3C_IRQ_EINT_BASE: i32 = s3c_irq(64 + 5);
#[inline]
pub const fn s3c_eint(x: i32) -> i32 { x + S3C_IRQ_EINT_BASE }
#[inline]
pub const fn irq_eint(x: i32) -> i32 { s3c_eint(x) }
#[inline]
pub const fn irq_eint_bit(x: i32) -> i32 { x - s3c_eint(0) }

pub const IRQ_EINT_GROUP1_NR: i32 = 15;
pub const IRQ_EINT_GROUP2_NR: i32 = 8;
pub const IRQ_EINT_GROUP3_NR: i32 = 5;
pub const IRQ_EINT_GROUP4_NR: i32 = 14;
pub const IRQ_EINT_GROUP5_NR: i32 = 7;
pub const IRQ_EINT_GROUP6_NR: i32 = 10;
pub const IRQ_EINT_GROUP7_NR: i32 = 16;
pub const IRQ_EINT_GROUP8_NR: i32 = 15;
pub const IRQ_EINT_GROUP9_NR: i32 = 9;

pub const IRQ_EINT_GROUP_BASE: i32 = s3c_eint(28);
pub const IRQ_EINT_GROUP1_BASE: i32 = IRQ_EINT_GROUP_BASE + 0x00;
pub const IRQ_EINT_GROUP2_BASE: i32 = IRQ_EINT_GROUP1_BASE + IRQ_EINT_GROUP1_NR;
pub const IRQ_EINT_GROUP3_BASE: i32 = IRQ_EINT_GROUP2_BASE + IRQ_EINT_GROUP2_NR;
pub const IRQ_EINT_GROUP4_BASE: i32 = IRQ_EINT_GROUP3_BASE + IRQ_EINT_GROUP3_NR;
pub const IRQ_EINT_GROUP5_BASE: i32 = IRQ_EINT_GROUP4_BASE + IRQ_EINT_GROUP4_NR;
pub const IRQ_EINT_GROUP6_BASE: i32 = IRQ_EINT_GROUP5_BASE + IRQ_EINT_GROUP5_NR;
pub const IRQ_EINT_GROUP7_BASE: i32 = IRQ_EINT_GROUP6_BASE + IRQ_EINT_GROUP6_NR;
pub const IRQ_EINT_GROUP8_BASE: i32 = IRQ_EINT_GROUP7_BASE + IRQ_EINT_GROUP7_NR;
pub const IRQ_EINT_GROUP9_BASE: i32 = IRQ_EINT_GROUP8_BASE + IRQ_EINT_GROUP8_NR;

/* Equivalent of the C token-pasting IRQ_EINT_GROUP(group, no) macro. */
#[macro_export]
macro_rules! IRQ_EINT_GROUP {
    (1, $no:expr) => { $crate::IRQ_EINT_GROUP1_BASE + ($no) };
    (2, $no:expr) => { $crate::IRQ_EINT_GROUP2_BASE + ($no) };
    (3, $no:expr) => { $crate::IRQ_EINT_GROUP3_BASE + ($no) };
    (4, $no:expr) => { $crate::IRQ_EINT_GROUP4_BASE + ($no) };
    (5, $no:expr) => { $crate::IRQ_EINT_GROUP5_BASE + ($no) };
    (6, $no:expr) => { $crate::IRQ_EINT_GROUP6_BASE + ($no) };
    (7, $no:expr) => { $crate::IRQ_EINT_GROUP7_BASE + ($no) };
    (8, $no:expr) => { $crate::IRQ_EINT_GROUP8_BASE + ($no) };
    (9, $no:expr) => { $crate::IRQ_EINT_GROUP9_BASE + ($no) };
}

/* Some boards have their own IRQs behind this. */
pub const IRQ_BOARD_START: i32 = IRQ_EINT_GROUP9_BASE + IRQ_EINT_GROUP9_NR + 1;
pub const S3C64XX_NR_IRQS: i32 = IRQ_BOARD_START;

pub const IRQ_ONENAND: i32 = IRQ_ONENAND0;
pub const IRQ_I2S0: i32 = IRQ_S3C6410_IIS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
