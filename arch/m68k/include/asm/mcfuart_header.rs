/* SPDX-License-Identifier: GPL-2.0 */
/* ColdFire internal UART support defines. */
/* C dependencies: linux/serial_core.h and linux/platform_device.h. */

#[repr(C)]
pub struct mcf_platform_uart {
    pub mapbase: ::core::ffi::c_ulong,
    pub membase: *mut ::core::ffi::c_void,
    pub irq: ::core::ffi::c_uint,
    pub uartclk: ::core::ffi::c_uint,
}

pub const MCFUART_UMR: u32 = 0x00;
pub const MCFUART_USR: u32 = 0x04;
pub const MCFUART_UCSR: u32 = 0x04;
pub const MCFUART_UCR: u32 = 0x08;
pub const MCFUART_URB: u32 = 0x0c;
pub const MCFUART_UTB: u32 = 0x0c;
pub const MCFUART_UIPCR: u32 = 0x10;
pub const MCFUART_UACR: u32 = 0x10;
pub const MCFUART_UISR: u32 = 0x14;
pub const MCFUART_UIMR: u32 = 0x14;
pub const MCFUART_UBG1: u32 = 0x18;
pub const MCFUART_UBG2: u32 = 0x1c;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_UTF: u32 = 0x28;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_URF: u32 = 0x2c;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_UFPD: u32 = 0x30;
#[cfg(any(feature = "CONFIG_M5206", feature = "CONFIG_M5206e", feature = "CONFIG_M5249", feature = "CONFIG_M525x", feature = "CONFIG_M5307", feature = "CONFIG_M5407"))]
pub const MCFUART_UIVR: u32 = 0x30;
pub const MCFUART_UIPR: u32 = 0x34;
pub const MCFUART_UOP1: u32 = 0x38;
pub const MCFUART_UOP0: u32 = 0x3c;

pub const MCFUART_MR1_RXRTS: u32 = 0x80;
pub const MCFUART_MR1_RXIRQFULL: u32 = 0x40;
pub const MCFUART_MR1_RXIRQRDY: u32 = 0x00;
pub const MCFUART_MR1_RXERRBLOCK: u32 = 0x20;
pub const MCFUART_MR1_RXERRCHAR: u32 = 0x00;
pub const MCFUART_MR1_PARITYNONE: u32 = 0x10;
pub const MCFUART_MR1_PARITYEVEN: u32 = 0x00;
pub const MCFUART_MR1_PARITYODD: u32 = 0x04;
pub const MCFUART_MR1_PARITYSPACE: u32 = 0x08;
pub const MCFUART_MR1_PARITYMARK: u32 = 0x0c;
pub const MCFUART_MR1_CS5: u32 = 0x00;
pub const MCFUART_MR1_CS6: u32 = 0x01;
pub const MCFUART_MR1_CS7: u32 = 0x02;
pub const MCFUART_MR1_CS8: u32 = 0x03;

pub const MCFUART_MR2_LOOPBACK: u32 = 0x80;
pub const MCFUART_MR2_REMOTELOOP: u32 = 0xc0;
pub const MCFUART_MR2_AUTOECHO: u32 = 0x40;
pub const MCFUART_MR2_TXRTS: u32 = 0x20;
pub const MCFUART_MR2_TXCTS: u32 = 0x10;
pub const MCFUART_MR2_STOP1: u32 = 0x07;
pub const MCFUART_MR2_STOP15: u32 = 0x08;
pub const MCFUART_MR2_STOP2: u32 = 0x0f;

pub const MCFUART_USR_RXBREAK: u32 = 0x80;
pub const MCFUART_USR_RXFRAMING: u32 = 0x40;
pub const MCFUART_USR_RXPARITY: u32 = 0x20;
pub const MCFUART_USR_RXOVERRUN: u32 = 0x10;
pub const MCFUART_USR_TXEMPTY: u32 = 0x08;
pub const MCFUART_USR_TXREADY: u32 = 0x04;
pub const MCFUART_USR_RXFULL: u32 = 0x02;
pub const MCFUART_USR_RXREADY: u32 = 0x01;
pub const MCFUART_USR_RXERR: u32 = MCFUART_USR_RXBREAK | MCFUART_USR_RXFRAMING | MCFUART_USR_RXPARITY | MCFUART_USR_RXOVERRUN;

pub const MCFUART_UCSR_RXCLKTIMER: u32 = 0xd0;
pub const MCFUART_UCSR_RXCLKEXT16: u32 = 0xe0;
pub const MCFUART_UCSR_RXCLKEXT1: u32 = 0xf0;
pub const MCFUART_UCSR_TXCLKTIMER: u32 = 0x0d;
pub const MCFUART_UCSR_TXCLKEXT16: u32 = 0x0e;
pub const MCFUART_UCSR_TXCLKEXT1: u32 = 0x0f;

pub const MCFUART_UCR_CMDNULL: u32 = 0x00;
pub const MCFUART_UCR_CMDRESETMRPTR: u32 = 0x10;
pub const MCFUART_UCR_CMDRESETRX: u32 = 0x20;
pub const MCFUART_UCR_CMDRESETTX: u32 = 0x30;
pub const MCFUART_UCR_CMDRESETERR: u32 = 0x40;
pub const MCFUART_UCR_CMDRESETBREAK: u32 = 0x50;
pub const MCFUART_UCR_CMDBREAKSTART: u32 = 0x60;
pub const MCFUART_UCR_CMDBREAKSTOP: u32 = 0x70;
pub const MCFUART_UCR_TXNULL: u32 = 0x00;
pub const MCFUART_UCR_TXENABLE: u32 = 0x04;
pub const MCFUART_UCR_TXDISABLE: u32 = 0x08;
pub const MCFUART_UCR_RXNULL: u32 = 0x00;
pub const MCFUART_UCR_RXENABLE: u32 = 0x01;
pub const MCFUART_UCR_RXDISABLE: u32 = 0x02;

pub const MCFUART_UIPCR_CTSCOS: u32 = 0x10;
pub const MCFUART_UIPCR_CTS: u32 = 0x01;
pub const MCFUART_UIPR_CTS: u32 = 0x01;
pub const MCFUART_UOP_RTS: u32 = 0x01;
pub const MCFUART_UACR_IEC: u32 = 0x01;
pub const MCFUART_UIR_COS: u32 = 0x80;
pub const MCFUART_UIR_DELTABREAK: u32 = 0x04;
pub const MCFUART_UIR_RXREADY: u32 = 0x02;
pub const MCFUART_UIR_TXREADY: u32 = 0x01;

#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_UTF_TXB: u32 = 0x1f;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_UTF_FULL: u32 = 0x20;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_UTF_TXS: u32 = 0xc0;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_URF_RXB: u32 = 0x1f;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_URF_FULL: u32 = 0x20;
#[cfg(feature = "CONFIG_M5272")]
pub const MCFUART_URF_RXS: u32 = 0xc0;

#[cfg(feature = "CONFIG_M54xx")]
pub const MCFUART_TXFIFOSIZE: u32 = 512;
#[cfg(all(not(feature = "CONFIG_M54xx"), feature = "CONFIG_M5272"))]
pub const MCFUART_TXFIFOSIZE: u32 = 25;
#[cfg(all(not(feature = "CONFIG_M54xx"), not(feature = "CONFIG_M5272")))]
pub const MCFUART_TXFIFOSIZE: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
