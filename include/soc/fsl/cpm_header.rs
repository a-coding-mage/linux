/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies corresponding to the original Linux headers are supplied externally. */

#[repr(C)]
pub struct spi_pram {
    pub rbase: u16,
    pub tbase: u16,
    pub rfcr: u8,
    pub tfcr: u8,
    pub mrblr: u16,
    pub rstate: u32,
    pub rdp: u32,
    pub rbptr: u16,
    pub rbc: u16,
    pub rxtmp: u32,
    pub tstate: u32,
    pub tdp: u32,
    pub tbptr: u16,
    pub tbc: u16,
    pub txtmp: u32,
    pub res: u32,
    pub rpbase: u16,
    pub res1: u16,
}

#[repr(C, packed)]
pub struct usb_ctlr {
    pub usb_usmod: u8,
    pub usb_usadr: u8,
    pub usb_uscom: u8,
    pub res1: [u8; 1],
    pub usb_usep: [u16; 4],
    pub res2: [u8; 4],
    pub usb_usber: u16,
    pub res3: [u8; 2],
    pub usb_usbmr: u16,
    pub res4: [u8; 1],
    pub usb_usbs: u8,
    /* Fields down below are QE-only */
    pub usb_ussft: u16,
    pub res5: [u8; 2],
    pub usb_usfrn: u16,
    pub res6: [u8; 0x22],
}

/* Function code bits, usually generic to devices. */
#[cfg(feature = "CONFIG_CPM1")]
pub const CPMFCR_GBL: u8 = 0x00;
#[cfg(feature = "CONFIG_CPM1")]
pub const CPMFCR_TC2: u8 = 0x00;
#[cfg(feature = "CONFIG_CPM1")]
pub const CPMFCR_DTB: u8 = 0x00;
#[cfg(feature = "CONFIG_CPM1")]
pub const CPMFCR_BDB: u8 = 0x00;
#[cfg(not(feature = "CONFIG_CPM1"))]
pub const CPMFCR_GBL: u8 = 0x20;
#[cfg(not(feature = "CONFIG_CPM1"))]
pub const CPMFCR_TC2: u8 = 0x04;
#[cfg(not(feature = "CONFIG_CPM1"))]
pub const CPMFCR_DTB: u8 = 0x02;
#[cfg(not(feature = "CONFIG_CPM1"))]
pub const CPMFCR_BDB: u8 = 0x01;
pub const CPMFCR_EB: u8 = 0x10;

pub const CPM_CR_INIT_TRX: u16 = 0x0000;
pub const CPM_CR_INIT_RX: u16 = 0x0001;
pub const CPM_CR_INIT_TX: u16 = 0x0002;
pub const CPM_CR_HUNT_MODE: u16 = 0x0003;
pub const CPM_CR_STOP_TX: u16 = 0x0004;
pub const CPM_CR_GRA_STOP_TX: u16 = 0x0005;
pub const CPM_CR_RESTART_TX: u16 = 0x0006;
pub const CPM_CR_CLOSE_RX_BD: u16 = 0x0007;
pub const CPM_CR_SET_GADDR: u16 = 0x0008;
pub const CPM_CR_SET_TIMER: u16 = 0x0008;
pub const CPM_CR_STOP_IDMA: u16 = 0x000b;

#[repr(C)]
pub struct cpm_buf_desc {
    pub cbd_sc: u16,
    pub cbd_datlen: u16,
    pub cbd_bufaddr: u32,
}
pub type cbd_t = cpm_buf_desc;

pub const BD_SC_EMPTY: u16 = 0x8000;
pub const BD_SC_READY: u16 = 0x8000;
pub const BD_SC_WRAP: u16 = 0x2000;
pub const BD_SC_INTRPT: u16 = 0x1000;
pub const BD_SC_LAST: u16 = 0x0800;
pub const BD_SC_TC: u16 = 0x0400;
pub const BD_SC_CM: u16 = 0x0200;
pub const BD_SC_ID: u16 = 0x0100;
pub const BD_SC_P: u16 = 0x0100;
pub const BD_SC_BR: u16 = 0x0020;
pub const BD_SC_FR: u16 = 0x0010;
pub const BD_SC_PR: u16 = 0x0008;
pub const BD_SC_NAK: u16 = 0x0004;
pub const BD_SC_OV: u16 = 0x0002;
pub const BD_SC_UN: u16 = 0x0002;
pub const BD_SC_CD: u16 = 0x0001;
pub const BD_SC_CL: u16 = 0x0001;

pub const BD_ENET_RX_EMPTY: u16 = 0x8000;
pub const BD_ENET_RX_WRAP: u16 = 0x2000;
pub const BD_ENET_RX_INTR: u16 = 0x1000;
pub const BD_ENET_RX_LAST: u16 = 0x0800;
pub const BD_ENET_RX_FIRST: u16 = 0x0400;
pub const BD_ENET_RX_MISS: u16 = 0x0100;
pub const BD_ENET_RX_BC: u16 = 0x0080;
pub const BD_ENET_RX_MC: u16 = 0x0040;
pub const BD_ENET_RX_LG: u16 = 0x0020;
pub const BD_ENET_RX_NO: u16 = 0x0010;
pub const BD_ENET_RX_SH: u16 = 0x0008;
pub const BD_ENET_RX_CR: u16 = 0x0004;
pub const BD_ENET_RX_OV: u16 = 0x0002;
pub const BD_ENET_RX_CL: u16 = 0x0001;
pub const BD_ENET_RX_STATS: u16 = 0x01ff;

pub const BD_ENET_TX_READY: u16 = 0x8000;
pub const BD_ENET_TX_PAD: u16 = 0x4000;
pub const BD_ENET_TX_WRAP: u16 = 0x2000;
pub const BD_ENET_TX_INTR: u16 = 0x1000;
pub const BD_ENET_TX_LAST: u16 = 0x0800;
pub const BD_ENET_TX_TC: u16 = 0x0400;
pub const BD_ENET_TX_DEF: u16 = 0x0200;
pub const BD_ENET_TX_HB: u16 = 0x0100;
pub const BD_ENET_TX_LC: u16 = 0x0080;
pub const BD_ENET_TX_RL: u16 = 0x0040;
pub const BD_ENET_TX_RCMASK: u16 = 0x003c;
pub const BD_ENET_TX_UN: u16 = 0x0002;
pub const BD_ENET_TX_CSL: u16 = 0x0001;
pub const BD_ENET_TX_STATS: u16 = 0x03ff;

pub const BD_SCC_TX_LAST: u16 = 0x0800;
pub const BD_I2C_START: u16 = 0x0400;

#[cfg(feature = "CONFIG_CPM")]
extern "C" {
    pub fn cpm_command(command: u32, opcode: u8) -> i32;
}

#[cfg(not(feature = "CONFIG_CPM"))]
#[inline]
pub fn cpm_command(_command: u32, _opcode: u8) -> i32 {
    -38
}

extern "C" {
    pub fn cpm2_gpiochip_add32(dev: *mut device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
