// Translated from fsi_master_ast_cf.h.
// TRACE_SYSTEM: fsi_master_ast_cf
// The Linux tracepoint registration macros are represented below by their
// event payload layouts and assignment helpers.

use core::ffi::c_int;

#[repr(C)]
pub struct FsiMasterAcfCoproCommandEntry {
    pub master_idx: c_int,
    pub op: u32,
}

#[inline]
pub unsafe fn fsi_master_acf_copro_command_assign(
    entry: *mut FsiMasterAcfCoproCommandEntry,
    master_idx: c_int,
    op: u32,
) {
    (*entry).master_idx = master_idx;
    (*entry).op = op;
}

#[repr(C)]
pub struct FsiMasterAcfSendRequestEntry {
    pub master_idx: c_int,
    pub msg: u64,
    pub bits: u8,
    pub rbits: u8,
}

#[inline]
pub unsafe fn fsi_master_acf_send_request_assign(
    entry: *mut FsiMasterAcfSendRequestEntry,
    master_idx: c_int,
    msg: u64,
    bits: u8,
    rbits: u8,
) {
    (*entry).master_idx = master_idx;
    (*entry).msg = msg;
    (*entry).bits = bits;
    (*entry).rbits = rbits;
}

#[repr(C)]
pub struct FsiMasterAcfCoproResponseEntry {
    pub master_idx: c_int,
    pub rtag: u8,
    pub rcrc: u8,
    pub rdata: u32,
    pub crc_ok: bool,
}

#[inline]
pub unsafe fn fsi_master_acf_copro_response_assign(
    entry: *mut FsiMasterAcfCoproResponseEntry,
    master_idx: c_int,
    rtag: u8,
    rcrc: u8,
    rdata: u32,
    crc_ok: bool,
) {
    (*entry).master_idx = master_idx;
    (*entry).rtag = rtag;
    (*entry).rcrc = rcrc;
    // be32_to_cpu(rdata); the conversion is target-endian dependent.
    (*entry).rdata = u32::from_be(rdata);
    (*entry).crc_ok = crc_ok;
}

#[repr(C)]
pub struct FsiMasterAcfCrcRspErrorEntry {
    pub master_idx: c_int,
    pub retries: c_int,
}

#[inline]
pub unsafe fn fsi_master_acf_crc_rsp_error_assign(
    entry: *mut FsiMasterAcfCrcRspErrorEntry,
    master_idx: c_int,
    retries: c_int,
) {
    (*entry).master_idx = master_idx;
    (*entry).retries = retries;
}

#[repr(C)]
pub struct FsiMasterAcfPollResponseBusyEntry {
    pub master_idx: c_int,
    pub busy_count: c_int,
}

#[inline]
pub unsafe fn fsi_master_acf_poll_response_busy_assign(
    entry: *mut FsiMasterAcfPollResponseBusyEntry,
    master_idx: c_int,
    busy_count: c_int,
) {
    (*entry).master_idx = master_idx;
    (*entry).busy_count = busy_count;
}

#[repr(C)]
pub struct FsiMasterAcfCmdAbsAddrEntry {
    pub master_idx: c_int,
    pub addr: u32,
}

#[inline]
pub unsafe fn fsi_master_acf_cmd_abs_addr_assign(
    entry: *mut FsiMasterAcfCmdAbsAddrEntry,
    master_idx: c_int,
    addr: u32,
) {
    (*entry).master_idx = master_idx;
    (*entry).addr = addr;
}

#[repr(C)]
pub struct FsiMasterAcfCmdRelAddrEntry {
    pub master_idx: c_int,
    pub rel_addr: u32,
}

#[inline]
pub unsafe fn fsi_master_acf_cmd_rel_addr_assign(
    entry: *mut FsiMasterAcfCmdRelAddrEntry,
    master_idx: c_int,
    rel_addr: u32,
) {
    (*entry).master_idx = master_idx;
    (*entry).rel_addr = rel_addr;
}

#[repr(C)]
pub struct FsiMasterAcfCmdSameAddrEntry {
    pub master_idx: c_int,
}

#[inline]
pub unsafe fn fsi_master_acf_cmd_same_addr_assign(
    entry: *mut FsiMasterAcfCmdSameAddrEntry,
    master_idx: c_int,
) {
    (*entry).master_idx = master_idx;
}

// TP_printk formats retained from the source:
// fsi-acf%d command %08x
// fsi-acf%d cmd: %016llx/%d/%d
// fsi-acf%d rsp: tag=%04x crc=%04x data=%08x %c\n
// fsi-acf%d CRC error in response retry %d
// fsi-acf%d: device reported busy %d times
// fsi-acf%d: Sending ABS_ADR %06x
// fsi-acf%d: Sending REL_ADR %03x
// fsi-acf%d: Sending SAME_ADR

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
