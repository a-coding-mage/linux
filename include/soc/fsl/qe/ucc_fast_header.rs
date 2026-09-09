/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Internal header file for UCC FAST unit routines. */

/* Dependencies supplied by the surrounding repository are intentionally not
 * implemented here. */

pub const R_E: u32 = 0x8000_0000; /* buffer empty */
pub const R_W: u32 = 0x2000_0000; /* wrap bit */
pub const R_I: u32 = 0x1000_0000; /* interrupt on reception */
pub const R_L: u32 = 0x0800_0000; /* last */
pub const R_F: u32 = 0x0400_0000; /* first */

pub const T_R: u32 = 0x8000_0000; /* ready bit */
pub const T_W: u32 = 0x2000_0000; /* wrap bit */
pub const T_I: u32 = 0x1000_0000; /* interrupt on completion */
pub const T_L: u32 = 0x0800_0000; /* last */

pub const R_E_S: u16 = 0x8000;
pub const R_W_S: u16 = 0x2000;
pub const R_I_S: u16 = 0x1000;
pub const R_L_S: u16 = 0x0800;
pub const R_F_S: u16 = 0x0400;
pub const R_CM_S: u16 = 0x0200;
pub const R_LG_S: u16 = 0x0020;
pub const R_NO_S: u16 = 0x0010;
pub const R_AB_S: u16 = 0x0008;
pub const R_CR_S: u16 = 0x0004;
pub const R_OV_S: u16 = 0x0002;
pub const R_CD_S: u16 = 0x0001;

pub const T_R_S: u16 = 0x8000;
pub const T_W_S: u16 = 0x2000;
pub const T_I_S: u16 = 0x1000;
pub const T_L_S: u16 = 0x0800;
pub const T_TC_S: u16 = 0x0400;
pub const T_TM_S: u16 = 0x0200;
pub const T_UN_S: u16 = 0x0002;
pub const T_CT_S: u16 = 0x0001;

pub const UCC_FAST_RX_ALIGN: u32 = 4;
pub const UCC_FAST_MRBLR_ALIGNMENT: u32 = 4;
pub const UCC_FAST_VIRT_FIFO_REGS_ALIGNMENT: u32 = 8;
pub const UCC_FAST_URFS_MIN_VAL: u32 = 0x88;
pub const UCC_FAST_RECEIVE_VIRTUAL_FIFO_SIZE_FUDGE_FACTOR: u32 = 8;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_channel_protocol_mode {
    UCC_FAST_PROTOCOL_MODE_HDLC = 0x00000000,
    UCC_FAST_PROTOCOL_MODE_RESERVED01 = 0x00000001,
    UCC_FAST_PROTOCOL_MODE_RESERVED_QMC = 0x00000002,
    UCC_FAST_PROTOCOL_MODE_RESERVED02 = 0x00000003,
    UCC_FAST_PROTOCOL_MODE_RESERVED_UART = 0x00000004,
    UCC_FAST_PROTOCOL_MODE_RESERVED03 = 0x00000005,
    UCC_FAST_PROTOCOL_MODE_RESERVED_EX_MAC_1 = 0x00000006,
    UCC_FAST_PROTOCOL_MODE_RESERVED_EX_MAC_2 = 0x00000007,
    UCC_FAST_PROTOCOL_MODE_RESERVED_BISYNC = 0x00000008,
    UCC_FAST_PROTOCOL_MODE_RESERVED04 = 0x00000009,
    UCC_FAST_PROTOCOL_MODE_ATM = 0x0000000A,
    UCC_FAST_PROTOCOL_MODE_RESERVED05 = 0x0000000B,
    UCC_FAST_PROTOCOL_MODE_ETHERNET = 0x0000000C,
    UCC_FAST_PROTOCOL_MODE_RESERVED06 = 0x0000000D,
    UCC_FAST_PROTOCOL_MODE_POS = 0x0000000E,
    UCC_FAST_PROTOCOL_MODE_RESERVED07 = 0x0000000F,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_transparent_txrx { UCC_FAST_GUMR_TRANSPARENT_TTX_TRX_NORMAL = 0x00000000, UCC_FAST_GUMR_TRANSPARENT_TTX_TRX_TRANSPARENT = 0x18000000 }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_diag_mode { UCC_FAST_DIAGNOSTIC_NORMAL = 0, UCC_FAST_DIAGNOSTIC_LOCAL_LOOP_BACK = 0x40000000, UCC_FAST_DIAGNOSTIC_AUTO_ECHO = 0x80000000, UCC_FAST_DIAGNOSTIC_LOOP_BACK_AND_ECHO = 0xC0000000 }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_sync_len { UCC_FAST_SYNC_LEN_NOT_USED = 0, UCC_FAST_SYNC_LEN_AUTOMATIC = 0x00004000, UCC_FAST_SYNC_LEN_8_BIT = 0x00008000, UCC_FAST_SYNC_LEN_16_BIT = 0x0000C000 }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_ready_to_send { UCC_FAST_SEND_IDLES_BETWEEN_FRAMES = 0, UCC_FAST_SEND_FLAGS_BETWEEN_FRAMES = 0x00002000 }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_rx_decoding_method { UCC_FAST_RX_ENCODING_NRZ = 0, UCC_FAST_RX_ENCODING_NRZI = 0x00000800, UCC_FAST_RX_ENCODING_RESERVED0 = 0x00001000, UCC_FAST_RX_ENCODING_RESERVED1 = 0x00001800 }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_tx_encoding_method { UCC_FAST_TX_ENCODING_NRZ = 0, UCC_FAST_TX_ENCODING_NRZI = 0x00000100, UCC_FAST_TX_ENCODING_RESERVED0 = 0x00000200, UCC_FAST_TX_ENCODING_RESERVED1 = 0x00000300 }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ucc_fast_transparent_tcrc { UCC_FAST_16_BIT_CRC = 0, UCC_FAST_CRC_RESERVED0 = 0x00000040, UCC_FAST_32_BIT_CRC = 0x00000080, UCC_FAST_CRC_RESERVED1 = 0x000000C0 }

#[repr(C)]
pub struct ucc_fast_info {
    pub ucc_num: i32, pub tdm_num: i32, pub rx_clock: qe_clock, pub tx_clock: qe_clock,
    pub rx_sync: qe_clock, pub tx_sync: qe_clock, pub regs: resource_size_t, pub irq: i32,
    pub uccm_mask: u32, pub brkpt_support: i32, pub grant_support: i32, pub tsa: i32,
    pub cdp: i32, pub cds: i32, pub ctsp: i32, pub ctss: i32, pub tci: i32, pub txsy: i32,
    pub rtsm: i32, pub revd: i32, pub rsyn: i32, pub max_rx_buf_length: u16, pub urfs: u16,
    pub urfet: u16, pub urfset: u16, pub utfs: u16, pub utfet: u16, pub utftt: u16, pub ufpt: u16,
    pub mode: ucc_fast_channel_protocol_mode, pub ttx_trx: ucc_fast_transparent_txrx,
    pub tenc: ucc_fast_tx_encoding_method, pub renc: ucc_fast_rx_decoding_method,
    pub tcrc: ucc_fast_transparent_tcrc, pub synl: ucc_fast_sync_len,
}

#[repr(C)]
pub struct ucc_fast_private {
    pub uf_info: *mut ucc_fast_info,
    pub uf_regs: *mut ucc_fast,
    pub p_ucce: *mut u32,
    pub p_uccm: *mut u32,
    #[cfg(CONFIG_UGETH_TX_ON_DEMAND)]
    pub p_utodr: *mut u16,
    pub enabled_tx: i32, pub enabled_rx: i32, pub stopped_tx: i32, pub stopped_rx: i32,
    pub ucc_fast_tx_virtual_fifo_base_offset: i32,
    pub ucc_fast_rx_virtual_fifo_base_offset: i32,
    #[cfg(STATISTICS)]
    pub tx_frames: u32,
    #[cfg(STATISTICS)]
    pub rx_frames: u32,
    #[cfg(STATISTICS)]
    pub tx_discarded: u32,
    #[cfg(STATISTICS)]
    pub rx_discarded: u32,
    pub mrblr: u16,
}

extern "C" {
    pub fn ucc_fast_init(uf_info: *mut ucc_fast_info, uccf_ret: *mut *mut ucc_fast_private) -> i32;
    pub fn ucc_fast_free(uccf: *mut ucc_fast_private);
    pub fn ucc_fast_enable(uccf: *mut ucc_fast_private, mode: comm_dir);
    pub fn ucc_fast_disable(uccf: *mut ucc_fast_private, mode: comm_dir);
    pub fn ucc_fast_irq(uccf: *mut ucc_fast_private);
    pub fn ucc_fast_transmit_on_demand(uccf: *mut ucc_fast_private);
    pub fn ucc_fast_get_qe_cr_subblock(uccf_num: i32) -> u32;
    pub fn ucc_fast_dump_regs(uccf: *mut ucc_fast_private);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
