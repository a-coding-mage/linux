/* SPDX-License-Identifier: GPL-2.0 */
/* Helper functions for scsw access. */

// C dependencies supplied by other translated headers:
// linux/types.h, asm/css_chars.h, asm/dma-types.h, asm/cio.h

#[repr(C, packed)]
pub struct cmd_scsw {
    pub key: u32, pub sctl: u32, pub eswf: u32, pub cc: u32, pub fmt: u32,
    pub pfch: u32, pub isic: u32, pub alcc: u32, pub ssi: u32, pub zcc: u32,
    pub ectl: u32, pub pno: u32, pub res: u32, pub fctl: u32, pub actl: u32,
    pub stctl: u32, pub cpa: u32, pub dstat: u32, pub cstat: u32, pub count: u32,
}

#[repr(C, packed)]
pub struct tm_scsw {
    pub key: u32, pub _reserved0: u32, pub eswf: u32, pub cc: u32, pub fmt: u32,
    pub x: u32, pub q: u32, pub _reserved1: u32, pub ectl: u32, pub pno: u32,
    pub _reserved2: u32, pub fctl: u32, pub actl: u32, pub stctl: u32,
    pub tcw: u32, pub dstat: u32, pub cstat: u32, pub fcxs: u32,
    pub ifob: u32, pub sesq: u32,
}

#[repr(C, packed)]
pub struct eadm_scsw {
    pub key: u32, pub _reserved0: u32, pub eswf: u32, pub cc: u32,
    pub _reserved1: u32, pub ectl: u32, pub _reserved2: u32, pub fctl: u32,
    pub actl: u32, pub stctl: u32, pub aob: u32, pub dstat: u32,
    pub cstat: u32, pub _reserved3: u32,
}

#[repr(C, packed)]
pub union scsw { pub cmd: cmd_scsw, pub tm: tm_scsw, pub eadm: eadm_scsw }

pub const SCSW_FCTL_CLEAR_FUNC: u32 = 0x1;
pub const SCSW_FCTL_HALT_FUNC: u32 = 0x2;
pub const SCSW_FCTL_START_FUNC: u32 = 0x4;
pub const SCSW_ACTL_SUSPENDED: u32 = 0x1;
pub const SCSW_ACTL_DEVACT: u32 = 0x2;
pub const SCSW_ACTL_SCHACT: u32 = 0x4;
pub const SCSW_ACTL_CLEAR_PEND: u32 = 0x8;
pub const SCSW_ACTL_HALT_PEND: u32 = 0x10;
pub const SCSW_ACTL_START_PEND: u32 = 0x20;
pub const SCSW_ACTL_RESUME_PEND: u32 = 0x40;
pub const SCSW_STCTL_STATUS_PEND: u32 = 0x1;
pub const SCSW_STCTL_SEC_STATUS: u32 = 0x2;
pub const SCSW_STCTL_PRIM_STATUS: u32 = 0x4;
pub const SCSW_STCTL_INTER_STATUS: u32 = 0x8;
pub const SCSW_STCTL_ALERT_STATUS: u32 = 0x10;
pub const DEV_STAT_ATTENTION: u32 = 0x80;
pub const DEV_STAT_STAT_MOD: u32 = 0x40;
pub const DEV_STAT_CU_END: u32 = 0x20;
pub const DEV_STAT_BUSY: u32 = 0x10;
pub const DEV_STAT_CHN_END: u32 = 0x08;
pub const DEV_STAT_DEV_END: u32 = 0x04;
pub const DEV_STAT_UNIT_CHECK: u32 = 0x02;
pub const DEV_STAT_UNIT_EXCEP: u32 = 0x01;
pub const SCHN_STAT_PCI: u32 = 0x80;
pub const SCHN_STAT_INCORR_LEN: u32 = 0x40;
pub const SCHN_STAT_PROG_CHECK: u32 = 0x20;
pub const SCHN_STAT_PROT_CHECK: u32 = 0x10;
pub const SCHN_STAT_CHN_DATA_CHK: u32 = 0x08;
pub const SCHN_STAT_CHN_CTRL_CHK: u32 = 0x04;
pub const SCHN_STAT_INTF_CTRL_CHK: u32 = 0x02;
pub const SCHN_STAT_CHAIN_CHECK: u32 = 0x01;
pub const SCSW_SESQ_DEV_NOFCX: u32 = 3;
pub const SCSW_SESQ_PATH_NOFCX: u32 = 4;
pub const SNS0_CMD_REJECT: u32 = 0x80;
// Preserved source spelling (SNS0_CMD_REJEC is supplied externally or is a source typo).
pub const SNS_CMD_REJECT: u32 = SNS0_CMD_REJEC;
pub const SNS0_INTERVENTION_REQ: u32 = 0x40;
pub const SNS0_BUS_OUT_CHECK: u32 = 0x20;
pub const SNS0_EQUIPMENT_CHECK: u32 = 0x10;
pub const SNS0_DATA_CHECK: u32 = 0x08;
pub const SNS0_OVERRUN: u32 = 0x04;
pub const SNS0_INCOMPL_DOMAIN: u32 = 0x01;
pub const SNS1_PERM_ERR: u32 = 0x80;
pub const SNS1_INV_TRACK_FORMAT: u32 = 0x40;
pub const SNS1_EOC: u32 = 0x20;
pub const SNS1_MESSAGE_TO_OPER: u32 = 0x10;
pub const SNS1_NO_REC_FOUND: u32 = 0x08;
pub const SNS1_FILE_PROTECTED: u32 = 0x04;
pub const SNS1_WRITE_INHIBITED: u32 = 0x02;
pub const SNS1_INPRECISE_END: u32 = 0x01;
pub const SNS2_REQ_INH_WRITE: u32 = 0x80;
pub const SNS2_CORRECTABLE: u32 = 0x40;
pub const SNS2_FIRST_LOG_ERR: u32 = 0x20;
pub const SNS2_ENV_DATA_PRESENT: u32 = 0x10;
pub const SNS2_INPRECISE_END: u32 = 0x04;
pub const SNS7_INVALID_ON_SEC: u32 = 0x0e;

// Bit-field extraction helpers retain the C field values in the packed representations.
extern "C" { pub static css_general_characteristics: CssGeneralCharacteristics; }
#[repr(C)] pub struct CssGeneralCharacteristics { pub fcx: bool }

#[inline] pub unsafe fn scsw_is_tm(s: *mut scsw) -> i32 { (css_general_characteristics.fcx && (*s).tm.x == 1) as i32 }
macro_rules! accessor { ($name:ident, $field:ident) => { #[inline] pub unsafe fn $name(s: *mut scsw) -> u32 { if scsw_is_tm(s) != 0 { (*s).tm.$field } else { (*s).cmd.$field } } }; }
accessor!(scsw_key, key); accessor!(scsw_eswf, eswf); accessor!(scsw_cc, cc);
accessor!(scsw_ectl, ectl); accessor!(scsw_pno, pno); accessor!(scsw_fctl, fctl);
accessor!(scsw_actl, actl); accessor!(scsw_stctl, stctl); accessor!(scsw_dstat, dstat);
accessor!(scsw_cstat, cstat);

#[inline] pub unsafe fn scsw_cmd_is_valid_key(s: *mut scsw) -> i32 { ((*s).cmd.fctl & SCSW_FCTL_START_FUNC) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_sctl(s: *mut scsw) -> i32 { ((*s).cmd.fctl & SCSW_FCTL_START_FUNC) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_eswf(s: *mut scsw) -> i32 { ((*s).cmd.stctl & SCSW_STCTL_STATUS_PEND) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_cc(s: *mut scsw) -> i32 { (((*s).cmd.fctl & SCSW_FCTL_START_FUNC) != 0 && ((*s).cmd.stctl & SCSW_STCTL_STATUS_PEND) != 0) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_fmt(s: *mut scsw) -> i32 { ((*s).cmd.fctl & SCSW_FCTL_START_FUNC) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_pfch(s: *mut scsw) -> i32 { ((*s).cmd.fctl & SCSW_FCTL_START_FUNC) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_isic(s: *mut scsw) -> i32 { ((*s).cmd.fctl & SCSW_FCTL_START_FUNC) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_alcc(s: *mut scsw) -> i32 { ((*s).cmd.fctl & SCSW_FCTL_START_FUNC) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_ssi(s: *mut scsw) -> i32 { ((*s).cmd.fctl & SCSW_FCTL_START_FUNC) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_zcc(s: *mut scsw) -> i32 { (((*s).cmd.fctl & SCSW_FCTL_START_FUNC) != 0 && ((*s).cmd.stctl & SCSW_STCTL_INTER_STATUS) != 0) as i32 }

#[inline] pub unsafe fn valid_ectl(st: u32) -> i32 { ((st & SCSW_STCTL_STATUS_PEND) != 0 && (st & SCSW_STCTL_ALERT_STATUS) != 0 && (st & SCSW_STCTL_INTER_STATUS) == 0) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_ectl(s: *mut scsw) -> i32 { valid_ectl((*s).cmd.stctl) }
#[inline] pub unsafe fn valid_pno(f: u32, st: u32, a: u32) -> i32 { if f == 0 || st & SCSW_STCTL_STATUS_PEND == 0 { 0 } else if st & SCSW_STCTL_INTER_STATUS == 0 || a & SCSW_ACTL_SUSPENDED != 0 { 1 } else { 0 } }
#[inline] pub unsafe fn scsw_cmd_is_valid_pno(s: *mut scsw) -> i32 { valid_pno((*s).cmd.fctl, (*s).cmd.stctl, (*s).cmd.actl) }
#[inline] pub unsafe fn scsw_cmd_is_valid_fctl(_: *mut scsw) -> i32 { 1 }
#[inline] pub unsafe fn scsw_cmd_is_valid_actl(_: *mut scsw) -> i32 { 1 }
#[inline] pub unsafe fn scsw_cmd_is_valid_stctl(_: *mut scsw) -> i32 { 1 }
#[inline] pub unsafe fn scsw_cmd_is_valid_dstat(s: *mut scsw) -> i32 { (((*s).cmd.stctl & SCSW_STCTL_STATUS_PEND) != 0 && (*s).cmd.cc != 3) as i32 }
#[inline] pub unsafe fn scsw_cmd_is_valid_cstat(s: *mut scsw) -> i32 { scsw_cmd_is_valid_dstat(s) }

macro_rules! tm_valid_start { ($n:ident, $e:expr) => { #[inline] pub unsafe fn $n(s: *mut scsw) -> i32 { $e } }; }
tm_valid_start!(scsw_tm_is_valid_key, (((*s).tm.fctl & SCSW_FCTL_START_FUNC) != 0) as i32);
tm_valid_start!(scsw_tm_is_valid_eswf, (((*s).tm.stctl & SCSW_STCTL_STATUS_PEND) != 0) as i32);
tm_valid_start!(scsw_tm_is_valid_cc, (((*s).tm.fctl & SCSW_FCTL_START_FUNC) != 0 && (*s).tm.stctl & SCSW_STCTL_STATUS_PEND != 0) as i32);
tm_valid_start!(scsw_tm_is_valid_fmt, 1); tm_valid_start!(scsw_tm_is_valid_x, 1); tm_valid_start!(scsw_tm_is_valid_q, 1);
#[inline] pub unsafe fn scsw_tm_is_valid_ectl(s: *mut scsw) -> i32 { valid_ectl((*s).tm.stctl) }
#[inline] pub unsafe fn scsw_tm_is_valid_pno(s: *mut scsw) -> i32 { valid_pno((*s).tm.fctl, (*s).tm.stctl, (*s).tm.actl) }
tm_valid_start!(scsw_tm_is_valid_fctl, 1); tm_valid_start!(scsw_tm_is_valid_actl, 1); tm_valid_start!(scsw_tm_is_valid_stctl, 1);
#[inline] pub unsafe fn scsw_tm_is_valid_dstat(s: *mut scsw) -> i32 { (((*s).tm.stctl & SCSW_STCTL_STATUS_PEND) != 0 && (*s).tm.cc != 3) as i32 }
#[inline] pub unsafe fn scsw_tm_is_valid_cstat(s: *mut scsw) -> i32 { scsw_tm_is_valid_dstat(s) }
#[inline] pub unsafe fn scsw_tm_is_valid_fcxs(_: *mut scsw) -> i32 { 1 }
#[inline] pub unsafe fn scsw_tm_is_valid_schxs(s: *mut scsw) -> i32 { ((*s).tm.cstat & (SCHN_STAT_PROG_CHECK | SCHN_STAT_INTF_CTRL_CHK | SCHN_STAT_PROT_CHECK | SCHN_STAT_CHN_DATA_CHK)) as i32 }

macro_rules! dispatch_valid { ($n:ident, $t:ident, $c:ident) => { #[inline] pub unsafe fn $n(s: *mut scsw) -> i32 { if scsw_is_tm(s) != 0 { $t(s) } else { $c(s) } } }; }
dispatch_valid!(scsw_is_valid_actl, scsw_tm_is_valid_actl, scsw_cmd_is_valid_actl);
dispatch_valid!(scsw_is_valid_cc, scsw_tm_is_valid_cc, scsw_cmd_is_valid_cc);
dispatch_valid!(scsw_is_valid_cstat, scsw_tm_is_valid_cstat, scsw_cmd_is_valid_cstat);
dispatch_valid!(scsw_is_valid_dstat, scsw_tm_is_valid_dstat, scsw_cmd_is_valid_dstat);
dispatch_valid!(scsw_is_valid_ectl, scsw_tm_is_valid_ectl, scsw_cmd_is_valid_ectl);
dispatch_valid!(scsw_is_valid_eswf, scsw_tm_is_valid_eswf, scsw_cmd_is_valid_eswf);
dispatch_valid!(scsw_is_valid_fctl, scsw_tm_is_valid_fctl, scsw_cmd_is_valid_fctl);
dispatch_valid!(scsw_is_valid_key, scsw_tm_is_valid_key, scsw_cmd_is_valid_key);
dispatch_valid!(scsw_is_valid_pno, scsw_tm_is_valid_pno, scsw_cmd_is_valid_pno);
dispatch_valid!(scsw_is_valid_stctl, scsw_tm_is_valid_stctl, scsw_cmd_is_valid_stctl);

#[inline] pub unsafe fn scsw_cmd_is_solicited(s: *mut scsw) -> i32 { (((*s).cmd.cc != 0) || ((*s).cmd.stctl != (SCSW_STCTL_STATUS_PEND | SCSW_STCTL_ALERT_STATUS))) as i32 }
#[inline] pub unsafe fn scsw_tm_is_solicited(s: *mut scsw) -> i32 { (((*s).tm.cc != 0) || ((*s).tm.stctl != (SCSW_STCTL_STATUS_PEND | SCSW_STCTL_ALERT_STATUS))) as i32 }
#[inline] pub unsafe fn scsw_is_solicited(s: *mut scsw) -> i32 { if scsw_is_tm(s) != 0 { scsw_tm_is_solicited(s) } else { scsw_cmd_is_solicited(s) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
