/* SPDX-License-Identifier: GPL-2.0+ */
/* Internal header file for Samsung S3C2410 serial ports (UART0-2). */

/* Translated from serial_s3c.h; values retain the original register layout. */
pub const S3C2410_URXH: u32 = 0x24;
pub const S3C2410_UTXH: u32 = 0x20;
pub const S3C2410_ULCON: u32 = 0x00;
pub const S3C2410_UCON: u32 = 0x04;
pub const S3C2410_UFCON: u32 = 0x08;
pub const S3C2410_UMCON: u32 = 0x0c;
pub const S3C2410_UBRDIV: u32 = 0x28;
pub const S3C2410_UTRSTAT: u32 = 0x10;
pub const S3C2410_UERSTAT: u32 = 0x14;
pub const S3C2410_UFSTAT: u32 = 0x18;
pub const S3C2410_UMSTAT: u32 = 0x1c;

pub const S3C2410_LCON_CFGMASK: u32 = (0xf << 3) | 0x3;
pub const S3C2410_LCON_CS5: u32 = 0; pub const S3C2410_LCON_CS6: u32 = 1;
pub const S3C2410_LCON_CS7: u32 = 2; pub const S3C2410_LCON_CS8: u32 = 3;
pub const S3C2410_LCON_CSMASK: u32 = 3;
pub const S3C2410_LCON_PNONE: u32 = 0; pub const S3C2410_LCON_PEVEN: u32 = 5 << 3;
pub const S3C2410_LCON_PODD: u32 = 4 << 3; pub const S3C2410_LCON_PMASK: u32 = 7 << 3;
pub const S3C2410_LCON_STOPB: u32 = 1 << 2; pub const S3C2410_LCON_IRM: u32 = 1 << 6;

pub const S3C2440_UCON_CLKMASK: u32 = 3 << 10; pub const S3C2440_UCON_CLKSHIFT: u32 = 10;
pub const S3C2440_UCON_PCLK: u32 = 0; pub const S3C2440_UCON_UCLK: u32 = 1 << 10;
pub const S3C2440_UCON_PCLK2: u32 = 2 << 10; pub const S3C2440_UCON_FCLK: u32 = 3 << 10;
pub const S3C2443_UCON_EPLL: u32 = 3 << 10;
pub const S3C6400_UCON_CLKMASK: u32 = 3 << 10; pub const S3C6400_UCON_CLKSHIFT: u32 = 10;
pub const S3C6400_UCON_PCLK: u32 = 0; pub const S3C6400_UCON_PCLK2: u32 = 2 << 10;
pub const S3C6400_UCON_UCLK0: u32 = 1 << 10; pub const S3C6400_UCON_UCLK1: u32 = 3 << 10;
pub const S3C2440_UCON2_FCLK_EN: u32 = 1 << 15;
pub const S3C2440_UCON0_DIVMASK: u32 = 15 << 12; pub const S3C2440_UCON1_DIVMASK: u32 = 15 << 12;
pub const S3C2440_UCON2_DIVMASK: u32 = 7 << 12; pub const S3C2440_UCON_DIVSHIFT: u32 = 12;
pub const S3C2412_UCON_CLKMASK: u32 = 3 << 10; pub const S3C2412_UCON_CLKSHIFT: u32 = 10;
pub const S3C2412_UCON_UCLK: u32 = 1 << 10; pub const S3C2412_UCON_USYSCLK: u32 = 3 << 10;
pub const S3C2412_UCON_PCLK: u32 = 0; pub const S3C2412_UCON_PCLK2: u32 = 2 << 10;
pub const S3C2410_UCON_CLKMASK: u32 = 1 << 10; pub const S3C2410_UCON_CLKSHIFT: u32 = 10;
pub const S3C2410_UCON_UCLK: u32 = 1 << 10; pub const S3C2410_UCON_SBREAK: u32 = 1 << 4;
pub const S3C2410_UCON_TXILEVEL: u32 = 1 << 9; pub const S3C2410_UCON_RXILEVEL: u32 = 1 << 8;
pub const S3C2410_UCON_TXIRQMODE: u32 = 1 << 2; pub const S3C2410_UCON_RXIRQMODE: u32 = 1;
pub const S3C2410_UCON_RXFIFO_TOI: u32 = 1 << 7; pub const S3C2443_UCON_RXERR_IRQEN: u32 = 1 << 6;
pub const S3C2410_UCON_LOOPBACK: u32 = 1 << 5;
pub const S3C2410_UCON_DEFAULT: u32 = S3C2410_UCON_TXILEVEL | S3C2410_UCON_RXILEVEL | S3C2410_UCON_TXIRQMODE | S3C2410_UCON_RXIRQMODE | S3C2410_UCON_RXFIFO_TOI;

pub const S3C64XX_UCON_TXBURST_1: u32 = 0; pub const S3C64XX_UCON_TXBURST_4: u32 = 1 << 20;
pub const S3C64XX_UCON_TXBURST_8: u32 = 2 << 20; pub const S3C64XX_UCON_TXBURST_16: u32 = 3 << 20;
pub const S3C64XX_UCON_TXBURST_MASK: u32 = 0xf << 20; pub const S3C64XX_UCON_RXBURST_1: u32 = 0;
pub const S3C64XX_UCON_RXBURST_4: u32 = 1 << 16; pub const S3C64XX_UCON_RXBURST_8: u32 = 2 << 16;
pub const S3C64XX_UCON_RXBURST_16: u32 = 3 << 16; pub const S3C64XX_UCON_RXBURST_MASK: u32 = 0xf << 16;
pub const S3C64XX_UCON_TIMEOUT_SHIFT: u32 = 12; pub const S3C64XX_UCON_TIMEOUT_MASK: u32 = 0xf << 12;
pub const S3C64XX_UCON_EMPTYINT_EN: u32 = 1 << 11; pub const S3C64XX_UCON_DMASUS_EN: u32 = 1 << 10;
pub const S3C64XX_UCON_TXINT_LEVEL: u32 = 1 << 9; pub const S3C64XX_UCON_RXINT_LEVEL: u32 = 1 << 8;
pub const S3C64XX_UCON_TIMEOUT_EN: u32 = 1 << 7; pub const S3C64XX_UCON_ERRINT_EN: u32 = 1 << 6;
pub const S3C64XX_UCON_TXMODE_DMA: u32 = 2 << 2; pub const S3C64XX_UCON_TXMODE_CPU: u32 = 1 << 2;
pub const S3C64XX_UCON_TXMODE_MASK: u32 = 3 << 2; pub const S3C64XX_UCON_RXMODE_DMA: u32 = 2;
pub const S3C64XX_UCON_RXMODE_CPU: u32 = 1; pub const S3C64XX_UCON_RXMODE_MASK: u32 = 3;

pub const S3C2410_UFCON_FIFOMODE: u32 = 1; pub const S3C2410_UFCON_TXTRIG0: u32 = 0;
pub const S3C2410_UFCON_RXTRIG8: u32 = 1 << 4; pub const S3C2410_UFCON_RXTRIG12: u32 = 2 << 4;
pub const S3C2440_UFCON_RXTRIG1: u32 = 0; pub const S3C2440_UFCON_RXTRIG8: u32 = 1 << 4;
pub const S3C2440_UFCON_RXTRIG16: u32 = 2 << 4; pub const S3C2440_UFCON_RXTRIG32: u32 = 3 << 4;
pub const S3C2440_UFCON_TXTRIG0: u32 = 0; pub const S3C2440_UFCON_TXTRIG16: u32 = 1 << 6;
pub const S3C2440_UFCON_TXTRIG32: u32 = 2 << 6; pub const S3C2440_UFCON_TXTRIG48: u32 = 3 << 6;
pub const S3C2410_UFCON_RESETBOTH: u32 = 3 << 1; pub const S3C2410_UFCON_RESETTX: u32 = 1 << 2;
pub const S3C2410_UFCON_RESETRX: u32 = 1 << 1;
pub const S3C2410_UFCON_DEFAULT: u32 = S3C2410_UFCON_FIFOMODE | S3C2410_UFCON_TXTRIG0 | S3C2410_UFCON_RXTRIG8;
pub const S3C2410_UMCOM_AFC: u32 = 1 << 4; pub const S3C2410_UMCOM_RTS_LOW: u32 = 1;
pub const S3C2412_UMCON_AFC_63: u32 = 0; pub const S3C2412_UMCON_AFC_56: u32 = 1 << 5;
pub const S3C2412_UMCON_AFC_48: u32 = 2 << 5; pub const S3C2412_UMCON_AFC_40: u32 = 3 << 5;
pub const S3C2412_UMCON_AFC_32: u32 = 4 << 5; pub const S3C2412_UMCON_AFC_24: u32 = 5 << 5;
pub const S3C2412_UMCON_AFC_16: u32 = 6 << 5; pub const S3C2412_UMCON_AFC_8: u32 = 7 << 5;

pub const S3C2410_UFSTAT_TXFULL: u32 = 1 << 9; pub const S3C2410_UFSTAT_RXFULL: u32 = 1 << 8;
pub const S3C2410_UFSTAT_TXMASK: u32 = 15 << 4; pub const S3C2410_UFSTAT_TXSHIFT: u32 = 4;
pub const S3C2410_UFSTAT_RXMASK: u32 = 15; pub const S3C2410_UFSTAT_RXSHIFT: u32 = 0;
pub const S3C2440_UFSTAT_TXFULL: u32 = 1 << 14; pub const S3C2440_UFSTAT_RXFULL: u32 = 1 << 6;
pub const S3C2440_UFSTAT_TXSHIFT: u32 = 8; pub const S3C2440_UFSTAT_RXSHIFT: u32 = 0;
pub const S3C2440_UFSTAT_TXMASK: u32 = 63 << 8; pub const S3C2440_UFSTAT_RXMASK: u32 = 63;
pub const S3C2410_UTRSTAT_TIMEOUT: u32 = 1 << 3; pub const S3C2410_UTRSTAT_TXE: u32 = 1 << 2;
pub const S3C2410_UTRSTAT_TXFE: u32 = 1 << 1; pub const S3C2410_UTRSTAT_RXDR: u32 = 1;
pub const S3C2410_UERSTAT_OVERRUN: u32 = 1; pub const S3C2410_UERSTAT_FRAME: u32 = 1 << 2;
pub const S3C2410_UERSTAT_BREAK: u32 = 1 << 3; pub const S3C2443_UERSTAT_PARITY: u32 = 1 << 1;
pub const S3C2410_UERSTAT_ANY: u32 = S3C2410_UERSTAT_OVERRUN | S3C2410_UERSTAT_FRAME | S3C2410_UERSTAT_BREAK;
pub const S3C2410_UMSTAT_CTS: u32 = 1; pub const S3C2410_UMSTAT_DeltaCTS: u32 = 1 << 2;
pub const S3C2443_DIVSLOT: u32 = 0x2c;

pub const S3C64XX_UINTP: u32 = 0x30; pub const S3C64XX_UINTSP: u32 = 0x34; pub const S3C64XX_UINTM: u32 = 0x38;
pub const S3C64XX_UINTM_RXD: u32 = 0; pub const S3C64XX_UINTM_ERROR: u32 = 1; pub const S3C64XX_UINTM_TXD: u32 = 2;
pub const S3C64XX_UINTM_RXD_MSK: u32 = 1; pub const S3C64XX_UINTM_ERR_MSK: u32 = 1 << 1; pub const S3C64XX_UINTM_TXD_MSK: u32 = 1 << 2;

pub const S5PV210_UCON_CLKMASK: u32 = 1 << 10; pub const S5PV210_UCON_CLKSHIFT: u32 = 10;
pub const S5PV210_UCON_PCLK: u32 = 0; pub const S5PV210_UCON_UCLK: u32 = 1 << 10;
pub const S5PV210_UFCON_TXTRIG0: u32 = 0; pub const S5PV210_UFCON_TXTRIG4: u32 = 1 << 8;
pub const S5PV210_UFCON_TXTRIG8: u32 = 2 << 8; pub const S5PV210_UFCON_TXTRIG16: u32 = 3 << 8;
pub const S5PV210_UFCON_TXTRIG32: u32 = 4 << 8; pub const S5PV210_UFCON_TXTRIG64: u32 = 5 << 8;
pub const S5PV210_UFCON_TXTRIG128: u32 = 6 << 8; pub const S5PV210_UFCON_TXTRIG256: u32 = 7 << 8;
pub const S5PV210_UFCON_RXTRIG1: u32 = 0; pub const S5PV210_UFCON_RXTRIG4: u32 = 1 << 4;
pub const S5PV210_UFCON_RXTRIG8: u32 = 2 << 4; pub const S5PV210_UFCON_RXTRIG16: u32 = 3 << 4;
pub const S5PV210_UFCON_RXTRIG32: u32 = 4 << 4; pub const S5PV210_UFCON_RXTRIG64: u32 = 5 << 4;
pub const S5PV210_UFCON_RXTRIG128: u32 = 6 << 4; pub const S5PV210_UFCON_RXTRIG256: u32 = 7 << 4;
pub const S5PV210_UFSTAT_TXFULL: u32 = 1 << 24; pub const S5PV210_UFSTAT_RXFULL: u32 = 1 << 8;
pub const S5PV210_UFSTAT_TXMASK: u32 = 255 << 16; pub const S5PV210_UFSTAT_TXSHIFT: u32 = 16;
pub const S5PV210_UFSTAT_RXMASK: u32 = 255; pub const S5PV210_UFSTAT_RXSHIFT: u32 = 0;
pub const S3C2410_UCON_CLKSEL0: u32 = 1; pub const S3C2410_UCON_CLKSEL1: u32 = 1 << 1;
pub const S3C2410_UCON_CLKSEL2: u32 = 1 << 2; pub const S3C2410_UCON_CLKSEL3: u32 = 1 << 3;
pub const S5PV210_UCON_DEFAULT: u32 = S3C2410_UCON_TXILEVEL | S3C2410_UCON_RXILEVEL | S3C2410_UCON_TXIRQMODE | S3C2410_UCON_RXIRQMODE | S3C2410_UCON_RXFIFO_TOI | S3C2443_UCON_RXERR_IRQEN;
pub const S5PV210_UFCON_DEFAULT: u32 = S3C2410_UFCON_FIFOMODE | S5PV210_UFCON_TXTRIG4 | S5PV210_UFCON_RXTRIG4;

pub const APPLE_S5L_UCON_RXTO_ENA: u32 = 9; pub const APPLE_S5L_UCON_RXTO_LEGACY_ENA: u32 = 11;
pub const APPLE_S5L_UCON_RXTHRESH_ENA: u32 = 12; pub const APPLE_S5L_UCON_TXTHRESH_ENA: u32 = 13;
pub const APPLE_S5L_UCON_RXTO_ENA_MSK: u32 = 1 << APPLE_S5L_UCON_RXTO_ENA;
pub const APPLE_S5L_UCON_RXTO_LEGACY_ENA_MSK: u32 = 1 << APPLE_S5L_UCON_RXTO_LEGACY_ENA;
pub const APPLE_S5L_UCON_RXTHRESH_ENA_MSK: u32 = 1 << APPLE_S5L_UCON_RXTHRESH_ENA;
pub const APPLE_S5L_UCON_TXTHRESH_ENA_MSK: u32 = 1 << APPLE_S5L_UCON_TXTHRESH_ENA;
pub const APPLE_S5L_UCON_DEFAULT: u32 = S3C2410_UCON_TXIRQMODE | S3C2410_UCON_RXIRQMODE | S3C2410_UCON_RXFIFO_TOI;
pub const APPLE_S5L_UCON_MASK: u32 = APPLE_S5L_UCON_RXTO_ENA_MSK | APPLE_S5L_UCON_RXTO_LEGACY_ENA_MSK | APPLE_S5L_UCON_RXTHRESH_ENA_MSK | APPLE_S5L_UCON_TXTHRESH_ENA_MSK;
pub const APPLE_S5L_UTRSTAT_RXTO_LEGACY: u32 = 1 << 3; pub const APPLE_S5L_UTRSTAT_RXTHRESH: u32 = 1 << 4;
pub const APPLE_S5L_UTRSTAT_TXTHRESH: u32 = 1 << 5; pub const APPLE_S5L_UTRSTAT_RXTO: u32 = 1 << 9;
pub const APPLE_S5L_UTRSTAT_ALL_FLAGS: u32 = ((1 << (9 - 3 + 1)) - 1) << 3;

/* The C header's non-assembler structure depends on linux/serial_core.h. */
#[repr(C)]
pub struct s3c2410_uartcfg {
    pub hwport: u8,
    pub unused: u8,
    pub flags: u16,
    pub uart_flags: upf_t,
    pub clk_sel: u32,
    pub has_fracval: u32,
    pub ucon: libc::c_ulong,
    pub ulcon: libc::c_ulong,
    pub ufcon: libc::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
