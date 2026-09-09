/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2022, Linaro Ltd. */

/* Translated from common.h. Linux endian and MHI symbols are external dependencies. */

pub const MHIREGLEN: u32 = 0x00;
pub const MHIVER: u32 = 0x08;
pub const MHICFG: u32 = 0x10;
pub const CHDBOFF: u32 = 0x18;
pub const ERDBOFF: u32 = 0x20;
pub const BHIOFF: u32 = 0x28;
pub const BHIEOFF: u32 = 0x2c;
pub const DEBUGOFF: u32 = 0x30;
pub const MHICTRL: u32 = 0x38;
pub const MHISTATUS: u32 = 0x48;
pub const CCABAP_LOWER: u32 = 0x58;
pub const CCABAP_HIGHER: u32 = 0x5c;
pub const ECABAP_LOWER: u32 = 0x60;
pub const ECABAP_HIGHER: u32 = 0x64;
pub const CRCBAP_LOWER: u32 = 0x68;
pub const CRCBAP_HIGHER: u32 = 0x6c;
pub const CRDB_LOWER: u32 = 0x70;
pub const CRDB_HIGHER: u32 = 0x74;
pub const MHICTRLBASE_LOWER: u32 = 0x80;
pub const MHICTRLBASE_HIGHER: u32 = 0x84;
pub const MHICTRLLIMIT_LOWER: u32 = 0x88;
pub const MHICTRLLIMIT_HIGHER: u32 = 0x8c;
pub const MHIDATABASE_LOWER: u32 = 0x98;
pub const MHIDATABASE_HIGHER: u32 = 0x9c;
pub const MHIDATALIMIT_LOWER: u32 = 0xa0;
pub const MHIDATALIMIT_HIGHER: u32 = 0xa4;

pub const BHI_BHIVERSION_MINOR: u32 = 0x00;
pub const BHI_BHIVERSION_MAJOR: u32 = 0x04;
pub const BHI_IMGADDR_LOW: u32 = 0x08;
pub const BHI_IMGADDR_HIGH: u32 = 0x0c;
pub const BHI_IMGSIZE: u32 = 0x10;
pub const BHI_RSVD1: u32 = 0x14;
pub const BHI_IMGTXDB: u32 = 0x18;
pub const BHI_RSVD2: u32 = 0x1c;
pub const BHI_INTVEC: u32 = 0x20;
pub const BHI_RSVD3: u32 = 0x24;
pub const BHI_EXECENV: u32 = 0x28;
pub const BHI_STATUS: u32 = 0x2c;
pub const BHI_ERRCODE: u32 = 0x30;
pub const BHI_ERRDBG1: u32 = 0x34;
pub const BHI_ERRDBG2: u32 = 0x38;
pub const BHI_ERRDBG3: u32 = 0x3c;
pub const BHI_SERIALNU: u32 = 0x40;
pub const BHI_SBLANTIROLLVER: u32 = 0x44;
pub const BHI_NUMSEG: u32 = 0x48;
pub const BHI_RSVD5: u32 = 0xc4;

#[inline] pub const fn BHI_MSMHWID(n: u32) -> u32 { 0x4c + 0x4 * n }
#[inline] pub const fn BHI_OEMPKHASH(n: u32) -> u32 { 0x64 + 0x4 * n }

#[inline] pub const fn BIT(n: u32) -> u32 { 1u32 << n }
#[inline] pub const fn GENMASK(high: u32, low: u32) -> u32 { (!0u32 >> (31 - high)) & (!0u32 << low) }
#[inline] pub const fn FIELD_PREP(mask: u32, value: u32) -> u32 { (value << mask.trailing_zeros()) & mask }
#[inline] pub const fn FIELD_GET(mask: u32, value: u32) -> u32 { (value & mask) >> mask.trailing_zeros() }

pub const BHI_TXDB_SEQNUM_BMSK: u32 = GENMASK(29, 0);
pub const BHI_TXDB_SEQNUM_SHFT: u32 = 0;
pub const BHI_STATUS_MASK: u32 = GENMASK(31, 30);
pub const BHI_STATUS_ERROR: u32 = 0x03;
pub const BHI_STATUS_SUCCESS: u32 = 0x02;
pub const BHI_STATUS_RESET: u32 = 0x00;

pub const BHIE_MSMSOCID_OFFS: u32 = 0x00;
pub const BHIE_TXVECADDR_LOW_OFFS: u32 = 0x2c;
pub const BHIE_TXVECADDR_HIGH_OFFS: u32 = 0x30;
pub const BHIE_TXVECSIZE_OFFS: u32 = 0x34;
pub const BHIE_TXVECDB_OFFS: u32 = 0x3c;
pub const BHIE_TXVECSTATUS_OFFS: u32 = 0x44;
pub const BHIE_RXVECADDR_LOW_OFFS: u32 = 0x60;
pub const BHIE_RXVECADDR_HIGH_OFFS: u32 = 0x64;
pub const BHIE_RXVECSIZE_OFFS: u32 = 0x68;
pub const BHIE_RXVECDB_OFFS: u32 = 0x70;
pub const BHIE_RXVECSTATUS_OFFS: u32 = 0x78;

pub const MHICFG_NHWER_MASK: u32 = GENMASK(31, 24);
pub const MHICFG_NER_MASK: u32 = GENMASK(23, 16);
pub const MHICFG_NHWCH_MASK: u32 = GENMASK(15, 8);
pub const MHICFG_NCH_MASK: u32 = GENMASK(7, 0);
pub const MHICTRL_MHISTATE_MASK: u32 = GENMASK(15, 8);
pub const MHICTRL_RESET_MASK: u32 = BIT(1);
pub const MHISTATUS_MHISTATE_MASK: u32 = GENMASK(15, 8);
pub const MHISTATUS_SYSERR_MASK: u32 = BIT(2);
pub const MHISTATUS_READY_MASK: u32 = BIT(0);

pub const BHIE_TXVECDB_SEQNUM_BMSK: u32 = GENMASK(29, 0);
pub const BHIE_TXVECDB_SEQNUM_SHFT: u32 = 0;
pub const BHIE_TXVECSTATUS_SEQNUM_BMSK: u32 = GENMASK(29, 0);
pub const BHIE_TXVECSTATUS_SEQNUM_SHFT: u32 = 0;
pub const BHIE_TXVECSTATUS_STATUS_BMSK: u32 = GENMASK(31, 30);
pub const BHIE_TXVECSTATUS_STATUS_SHFT: u32 = 30;
pub const BHIE_TXVECSTATUS_STATUS_RESET: u32 = 0;
pub const BHIE_TXVECSTATUS_STATUS_XFER_COMPL: u32 = 2;
pub const BHIE_TXVECSTATUS_STATUS_ERROR: u32 = 3;
pub const BHIE_RXVECDB_SEQNUM_BMSK: u32 = GENMASK(29, 0);
pub const BHIE_RXVECDB_SEQNUM_SHFT: u32 = 0;
pub const BHIE_RXVECSTATUS_SEQNUM_BMSK: u32 = GENMASK(29, 0);
pub const BHIE_RXVECSTATUS_SEQNUM_SHFT: u32 = 0;
pub const BHIE_RXVECSTATUS_STATUS_BMSK: u32 = GENMASK(31, 30);
pub const BHIE_RXVECSTATUS_STATUS_SHFT: u32 = 30;
pub const BHIE_RXVECSTATUS_STATUS_RESET: u32 = 0;
pub const BHIE_RXVECSTATUS_STATUS_XFER_COMPL: u32 = 2;
pub const BHIE_RXVECSTATUS_STATUS_ERROR: u32 = 3;
pub const EV_CTX_RESERVED_MASK: u32 = GENMASK(7,0);
pub const EV_CTX_INTMODC_MASK: u32 = GENMASK(15,8);
pub const EV_CTX_INTMODT_MASK: u32 = GENMASK(31,16);
pub const CHAN_CTX_CHSTATE_MASK: u32 = GENMASK(7,0);
pub const CHAN_CTX_BRSTMODE_MASK: u32 = GENMASK(9,8);
pub const CHAN_CTX_POLLCFG_MASK: u32 = GENMASK(15,10);
pub const CHAN_CTX_RESERVED_MASK: u32 = GENMASK(31,16);

#[repr(u32)]
pub enum mhi_pkt_type { MHI_PKT_TYPE_INVALID=0, MHI_PKT_TYPE_NOOP_CMD=1, MHI_PKT_TYPE_TRANSFER=2, MHI_PKT_TYPE_COALESCING=8, MHI_PKT_TYPE_RESET_CHAN_CMD=0x10, MHI_PKT_TYPE_STOP_CHAN_CMD=0x11, MHI_PKT_TYPE_START_CHAN_CMD=0x12, MHI_PKT_TYPE_STATE_CHANGE_EVENT=0x20, MHI_PKT_TYPE_CMD_COMPLETION_EVENT=0x21, MHI_PKT_TYPE_TX_EVENT=0x22, MHI_PKT_TYPE_RSC_TX_EVENT=0x28, MHI_PKT_TYPE_EE_EVENT=0x40, MHI_PKT_TYPE_TSYNC_EVENT=0x48, MHI_PKT_TYPE_BW_REQ_EVENT=0x50, MHI_PKT_TYPE_STALE_EVENT=0x51 }
#[repr(u32)]
pub enum mhi_ev_ccs { MHI_EV_CC_INVALID=0, MHI_EV_CC_SUCCESS=1, MHI_EV_CC_EOT=2, MHI_EV_CC_OVERFLOW=3, MHI_EV_CC_EOB=4, MHI_EV_CC_OOB=5, MHI_EV_CC_DB_MODE=6, MHI_EV_CC_UNDEFINED_ERR=0x10, MHI_EV_CC_BAD_TRE=0x11 }
#[repr(u32)]
pub enum mhi_ch_state { MHI_CH_STATE_DISABLED, MHI_CH_STATE_ENABLED, MHI_CH_STATE_RUNNING, MHI_CH_STATE_SUSPENDED, MHI_CH_STATE_STOP, MHI_CH_STATE_ERROR }
#[repr(u32)]
pub enum mhi_cmd_type { MHI_CMD_NOP=1, MHI_CMD_RESET_CHAN=16, MHI_CMD_STOP_CHAN=17, MHI_CMD_START_CHAN=18 }
pub const MHI_TRE_TYPE_TRANSFER: u32 = 2;
pub const MHI_SC_EV_PTR: u32 = 0;
pub const MHI_EE_EV_PTR: u32 = 0;

#[macro_export] macro_rules! MHI_TRE_CMD_NOOP_DWORD1 { () => { $crate::FIELD_PREP($crate::GENMASK(23,16), $crate::MHI_CMD_NOP as u32) } }
#[macro_export] macro_rules! MHI_TRE_CMD_RESET_DWORD1 { ($chid:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24), $chid) | $crate::FIELD_PREP($crate::GENMASK(23,16), $crate::MHI_CMD_RESET_CHAN as u32) } }
#[macro_export] macro_rules! MHI_TRE_CMD_STOP_DWORD1 { ($chid:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24), $chid) | $crate::FIELD_PREP($crate::GENMASK(23,16), $crate::MHI_CMD_STOP_CHAN as u32) } }
#[macro_export] macro_rules! MHI_TRE_CMD_START_DWORD1 { ($chid:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24), $chid) | $crate::FIELD_PREP($crate::GENMASK(23,16), $crate::MHI_CMD_START_CHAN as u32) } }
#[macro_export] macro_rules! MHI_TRE_EV_DWORD0 { ($code:expr,$len:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24),$code) | $crate::FIELD_PREP($crate::GENMASK(15,0),$len) } }
#[macro_export] macro_rules! MHI_TRE_EV_DWORD1 { ($chid:expr,$typ:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24),$chid) | $crate::FIELD_PREP($crate::GENMASK(23,16),$typ) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_CODE { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(31,24),$crate::MHI_TRE_GET_DWORD!($tre,0)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_LEN { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(15,0),$crate::MHI_TRE_GET_DWORD!($tre,0)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_CHID { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(31,24),$crate::MHI_TRE_GET_DWORD!($tre,1)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_TYPE { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(23,16),$crate::MHI_TRE_GET_DWORD!($tre,1)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_STATE { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(31,24),$crate::MHI_TRE_GET_DWORD!($tre,0)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_EXECENV { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(31,24),$crate::MHI_TRE_GET_DWORD!($tre,0)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_SEQ { ($tre:expr) => { $crate::MHI_TRE_GET_DWORD!($tre,0) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_TIME { ($tre:expr) => { $crate::MHI_TRE_GET_EV_PTR!($tre) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_COOKIE { ($tre:expr) => { ($crate::MHI_TRE_GET_EV_PTR!($tre) as u32) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_VEID { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(23,16),$crate::MHI_TRE_GET_DWORD!($tre,0)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_LINKSPEED { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(31,24),$crate::MHI_TRE_GET_DWORD!($tre,1)) } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_LINKWIDTH { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(7,0),$crate::MHI_TRE_GET_DWORD!($tre,0)) } }
#[macro_export] macro_rules! MHI_SC_EV_DWORD0 { ($state:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24),$state) } }
#[macro_export] macro_rules! MHI_SC_EV_DWORD1 { ($typ:expr) => { $crate::FIELD_PREP($crate::GENMASK(23,16),$typ) } }
#[macro_export] macro_rules! MHI_EE_EV_DWORD0 { ($ee:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24),$ee) } }
#[macro_export] macro_rules! MHI_EE_EV_DWORD1 { ($typ:expr) => { $crate::FIELD_PREP($crate::GENMASK(23,16),$typ) } }
#[macro_export] macro_rules! MHI_CC_EV_PTR { ($ptr:expr) => { $ptr } }
#[macro_export] macro_rules! MHI_CC_EV_DWORD0 { ($code:expr) => { $crate::FIELD_PREP($crate::GENMASK(31,24),$code) } }
#[macro_export] macro_rules! MHI_CC_EV_DWORD1 { ($typ:expr) => { $crate::FIELD_PREP($crate::GENMASK(23,16),$typ) } }
#[macro_export] macro_rules! MHI_TRE_DATA_DWORD0 { ($len:expr) => { $crate::FIELD_PREP($crate::GENMASK(15,0),$len) } }
#[macro_export] macro_rules! MHI_TRE_DATA_DWORD1 { ($bei:expr,$ieot:expr,$ieob:expr,$chain:expr) => { $crate::FIELD_PREP($crate::GENMASK(23,16),$crate::MHI_TRE_TYPE_TRANSFER) | $crate::FIELD_PREP($crate::BIT(10),$bei) | $crate::FIELD_PREP($crate::BIT(9),$ieot) | $crate::FIELD_PREP($crate::BIT(8),$ieob) | $crate::FIELD_PREP($crate::BIT(0),$chain) } }
#[macro_export] macro_rules! MHI_RSCTRE_DATA_PTR { ($ptr:expr,$len:expr) => { $crate::FIELD_PREP((!0u64) << 48,$len) | $ptr } }
#[macro_export] macro_rules! MHI_RSCTRE_DATA_DWORD0 { ($cookie:expr) => { $cookie } }
pub const MHI_RSCTRE_DATA_DWORD1: u32 = FIELD_PREP(GENMASK(23,16), 8);

#[repr(C)]
pub struct mhi_event_ctxt { pub intmod:u32, pub ertype:u32, pub msivec:u32, pub rbase:u64, pub rlen:u64, pub rp:u64, pub wp:u64 }
#[repr(C)]
pub struct mhi_chan_ctxt { pub chcfg:u32, pub chtype:u32, pub erindex:u32, pub rbase:u64, pub rlen:u64, pub rp:u64, pub wp:u64 }
#[repr(C)]
pub struct mhi_cmd_ctxt { pub reserved0:u32, pub reserved1:u32, pub reserved2:u32, pub rbase:u64, pub rlen:u64, pub rp:u64, pub wp:u64 }
#[repr(C)]
pub struct mhi_ring_element { pub ptr:u64, pub dword:[u32; 2] }

/* The following macro interfaces preserve the original endian conversions and bitfield operations. */
#[macro_export] macro_rules! MHI_TRE_CMD_NOOP_PTR { () => { 0 } }
#[macro_export] macro_rules! MHI_TRE_CMD_NOOP_DWORD0 { () => { 0 } }
#[macro_export] macro_rules! MHI_TRE_CMD_RESET_PTR { () => { 0 } }
#[macro_export] macro_rules! MHI_TRE_CMD_RESET_DWORD0 { () => { 0 } }
#[macro_export] macro_rules! MHI_TRE_CMD_STOP_PTR { () => { 0 } }
#[macro_export] macro_rules! MHI_TRE_CMD_STOP_DWORD0 { () => { 0 } }
#[macro_export] macro_rules! MHI_TRE_CMD_START_PTR { () => { 0 } }
#[macro_export] macro_rules! MHI_TRE_CMD_START_DWORD0 { () => { 0 } }

#[macro_export] macro_rules! MHI_TRE_GET_DWORD { ($tre:expr, $word:expr) => { ($tre).dword[$word] } }
#[macro_export] macro_rules! MHI_TRE_GET_CMD_CHID { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(31,24), $crate::MHI_TRE_GET_DWORD!($tre,1)) } }
#[macro_export] macro_rules! MHI_TRE_GET_CMD_TYPE { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(23,16), $crate::MHI_TRE_GET_DWORD!($tre,1)) } }
#[macro_export] macro_rules! MHI_TRE_EV_PTR { ($ptr:expr) => { $ptr } }
#[macro_export] macro_rules! MHI_TRE_GET_EV_PTR { ($tre:expr) => { ($tre).ptr } }
#[macro_export] macro_rules! MHI_TRE_DATA_PTR { ($ptr:expr) => { $ptr } }
#[macro_export] macro_rules! MHI_TRE_DATA_GET_PTR { ($tre:expr) => { ($tre).ptr } }
#[macro_export] macro_rules! MHI_TRE_DATA_GET_LEN { ($tre:expr) => { $crate::FIELD_GET($crate::GENMASK(15,0), $crate::MHI_TRE_GET_DWORD!($tre,0)) } }
#[macro_export] macro_rules! MHI_TRE_DATA_GET_CHAIN { ($tre:expr) => { $crate::FIELD_GET($crate::BIT(0), $crate::MHI_TRE_GET_DWORD!($tre,1)) != 0 } }
#[macro_export] macro_rules! MHI_TRE_DATA_GET_IEOB { ($tre:expr) => { $crate::FIELD_GET($crate::BIT(8), $crate::MHI_TRE_GET_DWORD!($tre,1)) != 0 } }
#[macro_export] macro_rules! MHI_TRE_DATA_GET_IEOT { ($tre:expr) => { $crate::FIELD_GET($crate::BIT(9), $crate::MHI_TRE_GET_DWORD!($tre,1)) != 0 } }
#[macro_export] macro_rules! MHI_TRE_DATA_GET_BEI { ($tre:expr) => { $crate::FIELD_GET($crate::BIT(10), $crate::MHI_TRE_GET_DWORD!($tre,1)) != 0 } }

/* mhi_state is supplied by the MHI dependency. */
pub unsafe fn mhi_state_str(state: u32) -> &'static str {
    match state { 0 => "RESET", 1 => "READY", 2 => "M0", 3 => "M1", 4 => "M2", 5 => "M3", 6 => "M3_FAST", 7 => "BHI", 8 => "SYS ERROR", _ => "Unknown state" }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
