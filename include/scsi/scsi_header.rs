/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header file contains public constants and structures used by
 * the SCSI initiator code.
 *
 * C dependencies supplied by other headers are intentionally referenced but
 * not redefined here.
 */

pub struct scsi_cmnd;

pub const SCSI_DEFAULT_EH_TIMEOUT: i32 = 10 * HZ;

/* DIX-capable adapters effectively support infinite chaining for the
 * protection information scatterlist. */
pub const SCSI_MAX_PROT_SG_SEGMENTS: u32 = 0xFFFF;

/* Special value for scanning all possible channels, ids, or luns. */
pub const SCAN_WILD_CARD: u32 = !0;

/* Standard mode-select header prepended to all mode-select commands. */
#[repr(C)]
pub struct ccs_modesel_head {
    pub _r1: u8,
    pub medium: u8,
    pub _r2: u8,
    pub block_desc_length: u8,
    pub density: u8,
    pub number_blocks_hi: u8,
    pub number_blocks_med: u8,
    pub number_blocks_lo: u8,
    pub _r3: u8,
    pub block_length_hi: u8,
    pub block_length_med: u8,
    pub block_length_lo: u8,
}

/* The Well Known LUNS (SAM-3) in our int representation of a LUN. */
pub const SCSI_W_LUN_BASE: u64 = 0xc100;
pub const SCSI_W_LUN_REPORT_LUNS: u64 = SCSI_W_LUN_BASE + 1;
pub const SCSI_W_LUN_ACCESS_CONTROL: u64 = SCSI_W_LUN_BASE + 2;
pub const SCSI_W_LUN_TARGET_LOG_PAGE: u64 = SCSI_W_LUN_BASE + 3;

#[inline]
pub fn scsi_is_wlun(lun: u64) -> i32 {
    if (lun & 0xff00) == SCSI_W_LUN_BASE { 1 } else { 0 }
}

#[inline]
pub fn scsi_status_is_check_condition(mut status: i32) -> bool {
    if status < 0 {
        return false;
    }
    status &= 0xfe;
    status == SAM_STAT_CHECK_CONDITION
}

/* Extended message codes. */
pub const EXTENDED_MODIFY_DATA_POINTER: u32 = 0x00;
pub const EXTENDED_SDTR: u32 = 0x01;
pub const EXTENDED_EXTENDED_IDENTIFY: u32 = 0x02;
pub const EXTENDED_WDTR: u32 = 0x03;
pub const EXTENDED_PPR: u32 = 0x04;
pub const EXTENDED_MODIFY_BIDI_DATA_PTR: u32 = 0x05;

/* Internal return values. */
pub const NEEDS_RETRY: i32 = 0x2001;
pub const SUCCESS: i32 = 0x2002;
pub const FAILED: i32 = 0x2003;
pub const QUEUED: i32 = 0x2004;
pub const SOFT_ERROR: i32 = 0x2005;
pub const ADD_TO_MLQUEUE: i32 = 0x2006;
pub const TIMEOUT_ERROR: i32 = 0x2007;
pub const SCSI_RETURN_NOT_HANDLED: i32 = 0x2008;
pub const FAST_IO_FAIL: i32 = 0x2009;

/* Status values returned by queuecommand if a command has not been queued. */
pub const SCSI_MLQUEUE_HOST_BUSY: i32 = 0x1055;
pub const SCSI_MLQUEUE_DEVICE_BUSY: i32 = 0x1056;
pub const SCSI_MLQUEUE_EH_RETRY: i32 = 0x1057;
pub const SCSI_MLQUEUE_TARGET_BUSY: i32 = 0x1058;

#[inline]
pub fn status_byte(result: i32) -> i32 { result & 0xff }
#[inline]
pub fn host_byte(result: i32) -> i32 { (result >> 16) & 0xff }
#[inline]
pub fn sense_class(sense: i32) -> i32 { (sense >> 4) & 0x7 }
#[inline]
pub fn sense_error(sense: i32) -> i32 { sense & 0xf }
#[inline]
pub fn sense_valid(sense: i32) -> i32 { sense & 0x80 }

/* Default timeouts. */
pub const FORMAT_UNIT_TIMEOUT: i32 = 2 * 60 * 60 * HZ;
pub const START_STOP_TIMEOUT: i32 = 60 * HZ;
pub const MOVE_MEDIUM_TIMEOUT: i32 = 5 * 60 * HZ;
pub const READ_ELEMENT_STATUS_TIMEOUT: i32 = 5 * 60 * HZ;
pub const READ_DEFECT_DATA_TIMEOUT: i32 = 60 * HZ;

pub const IDENTIFY_BASE: i32 = 0x80;
#[inline]
pub fn IDENTIFY(can_disconnect: bool, lun: i32) -> i32 {
    IDENTIFY_BASE | if can_disconnect { 0x40 } else { 0 } | (lun & 0x07)
}

pub const SCSI_UNKNOWN: i32 = 0;
pub const SCSI_1: i32 = 1;
pub const SCSI_1_CCS: i32 = 2;
pub const SCSI_2: i32 = 3;
pub const SCSI_3: i32 = 4;
pub const SCSI_SPC_2: i32 = 5;
pub const SCSI_SPC_3: i32 = 6;
pub const SCSI_SPC_4: i32 = 7;
pub const SCSI_SPC_5: i32 = 8;
pub const SCSI_SPC_6: i32 = 14;

/* INQ peripheral qualifiers. */
pub const SCSI_INQ_PQ_CON: i32 = 0x00;
pub const SCSI_INQ_PQ_NOT_CON: i32 = 0x01;
pub const SCSI_INQ_PQ_NOT_CAP: i32 = 0x03;

/* SCSI-specific ioctl commands. */
pub const SCSI_IOCTL_GET_IDLUN: u32 = 0x5382;
pub const SCSI_IOCTL_PROBE_HOST: u32 = 0x5385;
pub const SCSI_IOCTL_GET_BUS_NUMBER: u32 = 0x5386;
pub const SCSI_IOCTL_GET_PCI: u32 = 0x5387;

#[inline]
pub fn scsi_status_is_good(mut status: i32) -> bool {
    if status < 0 { return false; }
    if host_byte(status) == DID_NO_CONNECT { return false; }
    /* Ignore reserved bit 0, following SCSI-2 behaviour. */
    status &= 0xfe;
    status == SAM_STAT_GOOD ||
        status == SAM_STAT_CONDITION_MET ||
        status == SAM_STAT_INTERMEDIATE ||
        status == SAM_STAT_INTERMEDIATE_CONDITION_MET ||
        status == SAM_STAT_COMMAND_TERMINATED
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
