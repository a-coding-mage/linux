/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Flags for SCSI devices that need special treatment
 */

/* Only scan LUN 0 */
pub const BLIST_NOLUN: blist_flags_t = 1u64 << 0;
/* Known to have LUNs, force scanning.
 * DEPRECATED: Use max_luns=N */
pub const BLIST_FORCELUN: blist_flags_t = 1u64 << 1;
/* Flag for broken handshaking */
pub const BLIST_BORKEN: blist_flags_t = 1u64 << 2;
/* unlock by special command */
pub const BLIST_KEY: blist_flags_t = 1u64 << 3;
/* Do not use LUNs in parallel */
pub const BLIST_SINGLELUN: blist_flags_t = 1u64 << 4;
/* Buggy Tagged Command Queuing */
pub const BLIST_NOTQ: blist_flags_t = 1u64 << 5;
/* Non consecutive LUN numbering */
pub const BLIST_SPARSELUN: blist_flags_t = 1u64 << 6;
/* Avoid LUNS >= 5 */
pub const BLIST_MAX5LUN: blist_flags_t = 1u64 << 7;
/* Treat as (removable) CD-ROM */
pub const BLIST_ISROM: blist_flags_t = 1u64 << 8;
/* LUNs past 7 on a SCSI-2 device */
pub const BLIST_LARGELUN: blist_flags_t = 1u64 << 9;
/* override additional length field */
pub const BLIST_INQUIRY_36: blist_flags_t = 1u64 << 10;
/* ignore MEDIA CHANGE unit attention after resuming from runtime suspend */
pub const BLIST_IGN_MEDIA_CHANGE: blist_flags_t = 1u64 << 11;
/* do not do automatic start on add */
pub const BLIST_NOSTARTONADD: blist_flags_t = 1u64 << 12;
/* do not ask for VPD page size first on some broken targets */
pub const BLIST_NO_VPD_SIZE: blist_flags_t = 1u64 << 13;
pub const __BLIST_UNUSED_14: blist_flags_t = 1u64 << 14;
pub const __BLIST_UNUSED_15: blist_flags_t = 1u64 << 15;
pub const __BLIST_UNUSED_16: blist_flags_t = 1u64 << 16;
/* try REPORT_LUNS even for SCSI-2 devs (if HBA supports more than 8 LUNs) */
pub const BLIST_REPORTLUN2: blist_flags_t = 1u64 << 17;
/* don't try REPORT_LUNS scan (SCSI-3 devs) */
pub const BLIST_NOREPORTLUN: blist_flags_t = 1u64 << 18;
/* don't use PREVENT-ALLOW commands */
pub const BLIST_NOT_LOCKABLE: blist_flags_t = 1u64 << 19;
/* device is actually for RAID config */
pub const BLIST_NO_ULD_ATTACH: blist_flags_t = 1u64 << 20;
/* select without ATN */
pub const BLIST_SELECT_NO_ATN: blist_flags_t = 1u64 << 21;
/* retry HARDWARE_ERROR */
pub const BLIST_RETRY_HWERROR: blist_flags_t = 1u64 << 22;
/* maximum 512 sector cdb length */
pub const BLIST_MAX_512: blist_flags_t = 1u64 << 23;
pub const __BLIST_UNUSED_24: blist_flags_t = 1u64 << 24;
/* Disable T10 PI (DIF) */
pub const BLIST_NO_DIF: blist_flags_t = 1u64 << 25;
/* Ignore SBC-3 VPD pages */
pub const BLIST_SKIP_VPD_PAGES: blist_flags_t = 1u64 << 26;
pub const __BLIST_UNUSED_27: blist_flags_t = 1u64 << 27;
/* Attempt to read VPD pages */
pub const BLIST_TRY_VPD_PAGES: blist_flags_t = 1u64 << 28;
/* don't try to issue RSOC */
pub const BLIST_NO_RSOC: blist_flags_t = 1u64 << 29;
/* maximum 1024 sector cdb length */
pub const BLIST_MAX_1024: blist_flags_t = 1u64 << 30;
/* Use UNMAP limit for WRITE SAME */
pub const BLIST_UNMAP_LIMIT_WS: blist_flags_t = 1u64 << 31;
/* Always retry ABORTED_COMMAND with Internal Target Failure */
pub const BLIST_RETRY_ITF: blist_flags_t = 1u64 << 32;
/* Always retry ABORTED_COMMAND with ASC 0xc1 */
pub const BLIST_RETRY_ASC_C1: blist_flags_t = 1u64 << 33;
/* Do not query the IO Advice Hints Grouping mode page */
pub const BLIST_SKIP_IO_HINTS: blist_flags_t = 1u64 << 34;

pub const __BLIST_LAST_USED: blist_flags_t = BLIST_SKIP_IO_HINTS;

pub const __BLIST_HIGH_UNUSED: blist_flags_t =
    !(BLIST_SKIP_IO_HINTS | (BLIST_SKIP_IO_HINTS - 1u64));
pub const __BLIST_UNUSED_MASK: blist_flags_t =
    __BLIST_UNUSED_14 |
    __BLIST_UNUSED_15 |
    __BLIST_UNUSED_16 |
    __BLIST_UNUSED_24 |
    __BLIST_UNUSED_27 |
    __BLIST_HIGH_UNUSED;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
