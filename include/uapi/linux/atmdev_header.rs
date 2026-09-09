/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* atmdev.h - ATM device driver declarations and various related items */
/* Translated from the C UAPI header. */

// Dependencies supplied by the corresponding Linux UAPI headers:
// linux/atmapi.h, linux/atm.h, linux/atmioc.h

pub const ESI_LEN: usize = 6;

pub const ATM_OC3_PCR: i32 = (155_520_000 / 270 * 260 / 8 / 53);
pub const ATM_25_PCR: i32 = ((25_600_000 / 8 - 8_000) / 54);
pub const ATM_OC12_PCR: i32 = (622_080_000 / 1080 * 1040 / 8 / 53);
pub const ATM_DS3_PCR: i32 = 8_000 * 12;

#[repr(C)]
pub struct atm_aal_stats {
    pub tx: i32,
    pub tx_err: i32,
    pub rx: i32,
    pub rx_err: i32,
    pub rx_drop: i32,
}

#[repr(C)]
pub struct atm_dev_stats {
    pub aal0: atm_aal_stats,
    pub aal34: atm_aal_stats,
    pub aal5: atm_aal_stats,
}
// __ATM_API_ALIGN is supplied by the Linux UAPI environment.

// The following ioctl constants use _IOW and types from linux/atmioc.h,
// linux/atm.h, and linux/atmapi.h.
pub const ATM_GETLINKRATE: usize = _IOW('a', ATMIOC_ITF + 1, atmif_sioc);
pub const ATM_GETNAMES: usize = _IOW('a', ATMIOC_ITF + 3, atm_iobuf);
pub const ATM_GETTYPE: usize = _IOW('a', ATMIOC_ITF + 4, atmif_sioc);
pub const ATM_GETESI: usize = _IOW('a', ATMIOC_ITF + 5, atmif_sioc);
pub const ATM_GETCIRANGE: usize = _IOW('a', ATMIOC_ITF + 10, atmif_sioc);
pub const ATM_SETCIRANGE: usize = _IOW('a', ATMIOC_ITF + 11, atmif_sioc);
pub const ATM_SETESI: usize = _IOW('a', ATMIOC_ITF + 12, atmif_sioc);
pub const ATM_SETESIF: usize = _IOW('a', ATMIOC_ITF + 13, atmif_sioc);
pub const ATM_GETSTAT: usize = _IOW('a', ATMIOC_SARCOM + 0, atmif_sioc);
pub const ATM_GETSTATZ: usize = _IOW('a', ATMIOC_SARCOM + 1, atmif_sioc);
pub const ATM_GETLOOP: usize = _IOW('a', ATMIOC_SARCOM + 2, atmif_sioc);
pub const ATM_SETLOOP: usize = _IOW('a', ATMIOC_SARCOM + 3, atmif_sioc);
pub const ATM_QUERYLOOP: usize = _IOW('a', ATMIOC_SARCOM + 4, atmif_sioc);
pub const ATM_SETSC: usize = _IOW('a', ATMIOC_SPECIAL + 1, i32);
pub const ATM_SETBACKEND: usize = _IOW('a', ATMIOC_SPECIAL + 2, atm_backend_t);
pub const ATM_NEWBACKENDIF: usize = _IOW('a', ATMIOC_SPECIAL + 3, atm_backend_t);

pub const ATM_BACKEND_RAW: i32 = 0;
pub const ATM_BACKEND_PPP: i32 = 1;
pub const ATM_BACKEND_BR2684: i32 = 2;
pub const ATM_ITFTYP_LEN: usize = 8;

pub const __ATM_LM_NONE: i32 = 0;
pub const __ATM_LM_AAL: i32 = 1;
pub const __ATM_LM_ATM: i32 = 2;
pub const __ATM_LM_PHY: i32 = 8;
pub const __ATM_LM_ANALOG: i32 = 16;

#[inline]
pub const fn __ATM_LM_MKLOC(n: i32) -> i32 { n }
#[inline]
pub const fn __ATM_LM_MKRMT(n: i32) -> i32 { n << 8 }
#[inline]
pub const fn __ATM_LM_XTLOC(n: i32) -> i32 { n & 0xff }
#[inline]
pub const fn __ATM_LM_XTRMT(n: i32) -> i32 { (n >> 8) & 0xff }

pub const ATM_LM_NONE: i32 = 0;
pub const ATM_LM_LOC_AAL: i32 = __ATM_LM_MKLOC(__ATM_LM_AAL);
pub const ATM_LM_LOC_ATM: i32 = __ATM_LM_MKLOC(__ATM_LM_ATM);
pub const ATM_LM_LOC_PHY: i32 = __ATM_LM_MKLOC(__ATM_LM_PHY);
pub const ATM_LM_LOC_ANALOG: i32 = __ATM_LM_MKLOC(__ATM_LM_ANALOG);
pub const ATM_LM_RMT_AAL: i32 = __ATM_LM_MKRMT(__ATM_LM_AAL);
pub const ATM_LM_RMT_ATM: i32 = __ATM_LM_MKRMT(__ATM_LM_ATM);
pub const ATM_LM_RMT_PHY: i32 = __ATM_LM_MKRMT(__ATM_LM_PHY);
pub const ATM_LM_RMT_ANALOG: i32 = __ATM_LM_MKRMT(__ATM_LM_ANALOG);

#[repr(C)]
pub struct atm_iobuf {
    pub length: i32,
    pub buffer: *mut core::ffi::c_void,
}

pub const ATM_CI_MAX: i32 = -1;

#[repr(C)]
pub struct atm_cirange {
    pub vpi_bits: i8,
    pub vci_bits: i8,
}

pub const ATM_SC_RX: i32 = 1024;
pub const ATM_SC_TX: i32 = 2048;
pub const ATM_BACKLOG_DEFAULT: i32 = 32;

pub const ATM_MF_IMMED: i32 = 1;
pub const ATM_MF_INC_RSV: i32 = 2;
pub const ATM_MF_INC_SHP: i32 = 4;
pub const ATM_MF_DEC_RSV: i32 = 8;
pub const ATM_MF_DEC_SHP: i32 = 16;
pub const ATM_MF_BWD: i32 = 32;
pub const ATM_MF_SET: i32 = ATM_MF_INC_RSV | ATM_MF_INC_SHP | ATM_MF_DEC_RSV | ATM_MF_DEC_SHP | ATM_MF_BWD;

pub const ATM_VS_IDLE: i32 = 0;
pub const ATM_VS_CONNECTED: i32 = 1;
pub const ATM_VS_CLOSING: i32 = 2;
pub const ATM_VS_LISTEN: i32 = 3;
pub const ATM_VS_INUSE: i32 = 4;
pub const ATM_VS_BOUND: i32 = 5;

pub const ATM_VS2TXT_MAP: [&str; 6] = ["IDLE", "CONNECTED", "CLOSING", "LISTEN", "INUSE", "BOUND"];
pub const ATM_VF2TXT_MAP: [&str; 16] = [
    "ADDR", "READY", "PARTIAL", "REGIS", "RELEASED", "HASQOS", "LISTEN", "META",
    "256", "512", "1024", "2048", "SESSION", "HASSAP", "BOUND", "CLOSE",
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
