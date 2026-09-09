/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/include/asm-arm/hardware/serial_amba.h. */

use core::ffi::c_void;

/* UART register offsets. */
pub const UART01X_DR: u32 = 0x00;
pub const UART01X_RSR: u32 = 0x04;
pub const UART01X_ECR: u32 = 0x04;
pub const UART010_LCRH: u32 = 0x08;
pub const ST_UART011_DMAWM: u32 = 0x08;
pub const UART010_LCRM: u32 = 0x0c;
pub const ST_UART011_TIMEOUT: u32 = 0x0c;
pub const UART010_LCRL: u32 = 0x10;
pub const UART010_CR: u32 = 0x14;
pub const UART01X_FR: u32 = 0x18;
pub const UART010_IIR: u32 = 0x1c;
pub const UART010_ICR: u32 = 0x1c;
pub const ST_UART011_LCRH_RX: u32 = 0x1c;
pub const UART01X_ILPR: u32 = 0x20;
pub const UART011_IBRD: u32 = 0x24;
pub const UART011_FBRD: u32 = 0x28;
pub const UART011_LCRH: u32 = 0x2c;
pub const ST_UART011_LCRH_TX: u32 = 0x2c;
pub const UART011_CR: u32 = 0x30;
pub const UART011_IFLS: u32 = 0x34;
pub const UART011_IMSC: u32 = 0x38;
pub const UART011_RIS: u32 = 0x3c;
pub const UART011_MIS: u32 = 0x40;
pub const UART011_ICR: u32 = 0x44;
pub const UART011_DMACR: u32 = 0x48;
pub const ST_UART011_XFCR: u32 = 0x50;
pub const ST_UART011_XON1: u32 = 0x54;
pub const ST_UART011_XON2: u32 = 0x58;
pub const ST_UART011_XOFF1: u32 = 0x5c;
pub const ST_UART011_XOFF2: u32 = 0x60;
pub const ST_UART011_ITCR: u32 = 0x80;
pub const ST_UART011_ITIP: u32 = 0x84;
pub const ST_UART011_ABCR: u32 = 0x100;
pub const ST_UART011_ABIMSC: u32 = 0x15c;

/* ZTE UART register offsets. */
pub const ZX_UART011_DR: u32 = 0x04;
pub const ZX_UART011_FR: u32 = 0x14;
pub const ZX_UART011_IBRD: u32 = 0x24;
pub const ZX_UART011_FBRD: u32 = 0x28;
pub const ZX_UART011_LCRH: u32 = 0x30;
pub const ZX_UART011_CR: u32 = 0x34;
pub const ZX_UART011_IFLS: u32 = 0x38;
pub const ZX_UART011_IMSC: u32 = 0x40;
pub const ZX_UART011_RIS: u32 = 0x44;
pub const ZX_UART011_MIS: u32 = 0x48;
pub const ZX_UART011_ICR: u32 = 0x4c;
pub const ZX_UART011_DMACR: u32 = 0x50;

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }
macro_rules! genmask { ($hi:expr, $lo:expr) => { ((1u32 << ($hi + 1)) - 1) & !((1u32 << $lo) - 1) }; }
macro_rules! field_prep { ($mask:expr, $val:expr) => { (($val << $mask.trailing_zeros()) & $mask) }; }

pub const UART011_DR_OE: u32 = bit!(11); pub const UART011_DR_BE: u32 = bit!(10); pub const UART011_DR_PE: u32 = bit!(9); pub const UART011_DR_FE: u32 = bit!(8);
pub const UART01X_RSR_OE: u32 = bit!(3); pub const UART01X_RSR_BE: u32 = bit!(2); pub const UART01X_RSR_PE: u32 = bit!(1); pub const UART01X_RSR_FE: u32 = bit!(0);
pub const UART011_FR_RI: u32 = bit!(8); pub const UART011_FR_TXFE: u32 = bit!(7); pub const UART011_FR_RXFF: u32 = bit!(6);
pub const UART01X_FR_TXFF: u32 = 1 << 5; pub const UART01X_FR_RXFE: u32 = bit!(4); pub const UART01X_FR_BUSY: u32 = 1 << 3;
pub const UART01X_FR_DCD: u32 = bit!(2); pub const UART01X_FR_DSR: u32 = bit!(1); pub const UART01X_FR_CTS: u32 = bit!(0);
pub const UART01X_FR_TMSK: u32 = UART01X_FR_TXFF + UART01X_FR_BUSY;
pub const ZX_UART01X_FR_BUSY: u32 = bit!(8); pub const ZX_UART01X_FR_DSR: u32 = bit!(3); pub const ZX_UART01X_FR_CTS: u32 = bit!(1); pub const ZX_UART011_FR_RI: u32 = bit!(0);

pub const UART011_CR_CTSEN: u32 = bit!(15); pub const UART011_CR_RTSEN: u32 = bit!(14); pub const UART011_CR_OUT2: u32 = bit!(13); pub const UART011_CR_OUT1: u32 = bit!(12); pub const UART011_CR_RTS: u32 = bit!(11); pub const UART011_CR_DTR: u32 = bit!(10); pub const UART011_CR_RXE: u32 = bit!(9); pub const UART011_CR_TXE: u32 = bit!(8); pub const UART011_CR_LBE: u32 = bit!(7);
pub const UART010_CR_RTIE: u32 = bit!(6); pub const UART010_CR_TIE: u32 = bit!(5); pub const UART010_CR_RIE: u32 = bit!(4); pub const UART010_CR_MSIE: u32 = bit!(3); pub const ST_UART011_CR_OVSFACT: u32 = bit!(3); pub const UART01X_CR_IIRLP: u32 = bit!(2); pub const UART01X_CR_SIREN: u32 = bit!(1); pub const UART01X_CR_UARTEN: u32 = bit!(0);
pub const UART011_LCRH_SPS: u32 = bit!(7); pub const UART01X_LCRH_WLEN_8: u32 = 0x60; pub const UART01X_LCRH_WLEN_7: u32 = 0x40; pub const UART01X_LCRH_WLEN_6: u32 = 0x20; pub const UART01X_LCRH_WLEN_5: u32 = 0; pub const UART01X_LCRH_FEN: u32 = bit!(4); pub const UART01X_LCRH_STP2: u32 = bit!(3); pub const UART01X_LCRH_EPS: u32 = bit!(2); pub const UART01X_LCRH_PEN: u32 = bit!(1); pub const UART01X_LCRH_BRK: u32 = bit!(0);

pub const ST_UART011_DMAWM_RX: u32 = genmask!(5, 3); pub const ST_UART011_DMAWM_TX: u32 = genmask!(2, 0);
pub const ST_UART011_DMAWM_RX_1: u32 = field_prep!(ST_UART011_DMAWM_RX, 0); pub const ST_UART011_DMAWM_RX_2: u32 = field_prep!(ST_UART011_DMAWM_RX, 1); pub const ST_UART011_DMAWM_RX_4: u32 = field_prep!(ST_UART011_DMAWM_RX, 2); pub const ST_UART011_DMAWM_RX_8: u32 = field_prep!(ST_UART011_DMAWM_RX, 3); pub const ST_UART011_DMAWM_RX_16: u32 = field_prep!(ST_UART011_DMAWM_RX, 4); pub const ST_UART011_DMAWM_RX_32: u32 = field_prep!(ST_UART011_DMAWM_RX, 5); pub const ST_UART011_DMAWM_RX_48: u32 = field_prep!(ST_UART011_DMAWM_RX, 6);
pub const ST_UART011_DMAWM_TX_1: u32 = field_prep!(ST_UART011_DMAWM_TX, 0); pub const ST_UART011_DMAWM_TX_2: u32 = field_prep!(ST_UART011_DMAWM_TX, 1); pub const ST_UART011_DMAWM_TX_4: u32 = field_prep!(ST_UART011_DMAWM_TX, 2); pub const ST_UART011_DMAWM_TX_8: u32 = field_prep!(ST_UART011_DMAWM_TX, 3); pub const ST_UART011_DMAWM_TX_16: u32 = field_prep!(ST_UART011_DMAWM_TX, 4); pub const ST_UART011_DMAWM_TX_32: u32 = field_prep!(ST_UART011_DMAWM_TX, 5); pub const ST_UART011_DMAWM_TX_48: u32 = field_prep!(ST_UART011_DMAWM_TX, 6);

pub const UART010_IIR_RTIS: u32 = bit!(3); pub const UART010_IIR_TIS: u32 = bit!(2); pub const UART010_IIR_RIS: u32 = bit!(1); pub const UART010_IIR_MIS: u32 = bit!(0);
pub const UART011_IFLS_RXIFLSEL: u32 = genmask!(5, 3); pub const UART011_IFLS_TXIFLSEL: u32 = genmask!(2, 0);
pub const UART011_IFLS_RX1_8: u32 = field_prep!(UART011_IFLS_RXIFLSEL, 0); pub const UART011_IFLS_RX2_8: u32 = field_prep!(UART011_IFLS_RXIFLSEL, 1); pub const UART011_IFLS_RX4_8: u32 = field_prep!(UART011_IFLS_RXIFLSEL, 2); pub const UART011_IFLS_RX6_8: u32 = field_prep!(UART011_IFLS_RXIFLSEL, 3); pub const UART011_IFLS_RX7_8: u32 = field_prep!(UART011_IFLS_RXIFLSEL, 4); pub const UART011_IFLS_TX1_8: u32 = field_prep!(UART011_IFLS_TXIFLSEL, 0); pub const UART011_IFLS_TX2_8: u32 = field_prep!(UART011_IFLS_TXIFLSEL, 1); pub const UART011_IFLS_TX4_8: u32 = field_prep!(UART011_IFLS_TXIFLSEL, 2); pub const UART011_IFLS_TX6_8: u32 = field_prep!(UART011_IFLS_TXIFLSEL, 3); pub const UART011_IFLS_TX7_8: u32 = field_prep!(UART011_IFLS_TXIFLSEL, 4);
pub const UART011_IFLS_RX_HALF: u32 = field_prep!(UART011_IFLS_RXIFLSEL, 5); pub const UART011_IFLS_TX_HALF: u32 = field_prep!(UART011_IFLS_TXIFLSEL, 5);

/* Interrupt masks, status, and clear bits share the same bit positions. */
pub const UART011_OEIM: u32 = bit!(10); pub const UART011_BEIM: u32 = bit!(9); pub const UART011_PEIM: u32 = bit!(8); pub const UART011_FEIM: u32 = bit!(7); pub const UART011_RTIM: u32 = bit!(6); pub const UART011_TXIM: u32 = bit!(5); pub const UART011_RXIM: u32 = bit!(4); pub const UART011_DSRMIM: u32 = bit!(3); pub const UART011_DCDMIM: u32 = bit!(2); pub const UART011_CTSMIM: u32 = bit!(1); pub const UART011_RIMIM: u32 = bit!(0);
pub const UART011_OEIS: u32 = bit!(10); pub const UART011_BEIS: u32 = bit!(9); pub const UART011_PEIS: u32 = bit!(8); pub const UART011_FEIS: u32 = bit!(7); pub const UART011_RTIS: u32 = bit!(6); pub const UART011_TXIS: u32 = bit!(5); pub const UART011_RXIS: u32 = bit!(4); pub const UART011_DSRMIS: u32 = bit!(3); pub const UART011_DCDMIS: u32 = bit!(2); pub const UART011_CTSMIS: u32 = bit!(1); pub const UART011_RIMIS: u32 = bit!(0);
pub const UART011_OEIC: u32 = bit!(10); pub const UART011_BEIC: u32 = bit!(9); pub const UART011_PEIC: u32 = bit!(8); pub const UART011_FEIC: u32 = bit!(7); pub const UART011_RTIC: u32 = bit!(6); pub const UART011_TXIC: u32 = bit!(5); pub const UART011_RXIC: u32 = bit!(4); pub const UART011_DSRMIC: u32 = bit!(3); pub const UART011_DCDMIC: u32 = bit!(2); pub const UART011_CTSMIC: u32 = bit!(1); pub const UART011_RIMIC: u32 = bit!(0);
pub const UART011_DMAONERR: u32 = bit!(2); pub const UART011_TXDMAE: u32 = bit!(1); pub const UART011_RXDMAE: u32 = bit!(0);
pub const UART01X_RSR_ANY: u32 = UART01X_RSR_OE | UART01X_RSR_BE | UART01X_RSR_PE | UART01X_RSR_FE;
pub const UART01X_FR_MODEM_ANY: u32 = UART01X_FR_DCD | UART01X_FR_DSR | UART01X_FR_CTS;

#[repr(C)]
pub struct amba_device;
#[repr(C)]
pub struct dma_chan;

#[repr(C)]
pub struct amba_pl010_data {
    pub set_mctrl: Option<unsafe extern "C" fn(dev: *mut amba_device, base: *mut c_void, mctrl: u32)>,
}

#[repr(C)]
pub struct amba_pl011_data {
    pub dma_filter: Option<unsafe extern "C" fn(chan: *mut dma_chan, filter_param: *mut c_void) -> bool>,
    pub dma_rx_param: *mut c_void,
    pub dma_tx_param: *mut c_void,
    pub dma_rx_poll_enable: bool,
    pub dma_rx_poll_rate: u32,
    pub dma_rx_poll_timeout: u32,
    pub init: Option<unsafe extern "C" fn()>,
    pub exit: Option<unsafe extern "C" fn()>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
