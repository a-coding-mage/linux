// SPDX-License-Identifier: GPL-2.0+
/* The state-machine driver for an IPMI SMIC driver. */

// External kernel and ipmi_si_sm.h dependencies are supplied by the surrounding crate.

pub const SMIC_DEBUG_STATES: i32 = 4;
pub const SMIC_DEBUG_MSG: i32 = 2;
pub const SMIC_DEBUG_ENABLE: i32 = 1;

static mut smic_debug: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum smic_states {
    SMIC_IDLE,
    SMIC_START_OP,
    SMIC_OP_OK,
    SMIC_WRITE_START,
    SMIC_WRITE_NEXT,
    SMIC_WRITE_END,
    SMIC_WRITE2READ,
    SMIC_READ_START,
    SMIC_READ_NEXT,
    SMIC_READ_END,
    SMIC_HOSED,
}

pub const MAX_SMIC_READ_SIZE: usize = 80;
pub const MAX_SMIC_WRITE_SIZE: usize = 80;
pub const SMIC_MAX_ERROR_RETRIES: u32 = 3;
pub const SMIC_RETRY_TIMEOUT: i64 = 2 * USEC_PER_SEC as i64;

pub const SMIC_RX_DATA_READY: u8 = 0x80;
pub const SMIC_TX_DATA_READY: u8 = 0x40;
pub const SMIC_SMI: u8 = 0x10;
pub const SMIC_EVM_DATA_AVAIL: u8 = 0x08;
pub const SMIC_SMS_DATA_AVAIL: u8 = 0x04;
pub const SMIC_FLAG_BSY: u8 = 0x01;

pub const EC_NO_ERROR: u8 = 0x00;
pub const EC_ABORTED: u8 = 0x01;
pub const EC_ILLEGAL_CONTROL: u8 = 0x02;
pub const EC_NO_RESPONSE: u8 = 0x03;
pub const EC_ILLEGAL_COMMAND: u8 = 0x04;
pub const EC_BUFFER_FULL: u8 = 0x05;

#[repr(C)]
pub struct si_sm_data {
    pub state: smic_states,
    pub io: *mut si_sm_io,
    pub write_data: [u8; MAX_SMIC_WRITE_SIZE],
    pub write_pos: i32,
    pub write_count: i32,
    pub orig_write_count: i32,
    pub read_data: [u8; MAX_SMIC_READ_SIZE],
    pub read_pos: i32,
    pub truncated: i32,
    pub error_retries: u32,
    pub smic_timeout: i64,
}

pub unsafe fn init_smic_data(smic: *mut si_sm_data, io: *mut si_sm_io) -> u32 {
    (*smic).state = smic_states::SMIC_IDLE;
    (*smic).io = io;
    (*smic).write_pos = 0;
    (*smic).write_count = 0;
    (*smic).orig_write_count = 0;
    (*smic).read_pos = 0;
    (*smic).error_retries = 0;
    (*smic).truncated = 0;
    (*smic).smic_timeout = SMIC_RETRY_TIMEOUT;
    3
}

pub unsafe fn start_smic_transaction(smic: *mut si_sm_data, data: *mut u8, size: u32) -> i32 {
    if size < 2 { return IPMI_REQ_LEN_INVALID_ERR; }
    if size > MAX_SMIC_WRITE_SIZE as u32 { return IPMI_REQ_LEN_EXCEEDED_ERR; }
    if (*smic).state as i32 != smic_states::SMIC_IDLE as i32 && (*smic).state as i32 != smic_states::SMIC_HOSED as i32 {
        dev_warn((*smic).io, "SMIC in invalid state %d\n", (*smic).state as i32);
        return IPMI_NOT_IN_MY_STATE_ERR;
    }
    if smic_debug & SMIC_DEBUG_MSG != 0 {
        dev_dbg((*smic).io, "%s -", "start_smic_transaction");
        for i in 0..size { pr_cont(" %02x", *data.add(i as usize)); }
        pr_cont("\n");
    }
    (*smic).error_retries = 0;
    core::ptr::copy_nonoverlapping(data, (*smic).write_data.as_mut_ptr(), size as usize);
    (*smic).write_count = size as i32;
    (*smic).orig_write_count = size as i32;
    (*smic).write_pos = 0;
    (*smic).read_pos = 0;
    (*smic).state = smic_states::SMIC_START_OP;
    (*smic).smic_timeout = SMIC_RETRY_TIMEOUT;
    0
}

pub unsafe fn smic_get_result(smic: *mut si_sm_data, data: *mut u8, length: u32) -> i32 {
    if smic_debug & SMIC_DEBUG_MSG != 0 {
        dev_dbg((*smic).io, "smic_get result -");
        for i in 0..(*smic).read_pos { pr_cont(" %02x", (*smic).read_data[i as usize]); }
        pr_cont("\n");
    }
    if length < (*smic).read_pos as u32 { (*smic).read_pos = length as i32; (*smic).truncated = 1; }
    core::ptr::copy_nonoverlapping((*smic).read_data.as_ptr(), data, (*smic).read_pos as usize);
    if length >= 3 && (*smic).read_pos < 3 { *data.add(2) = IPMI_ERR_UNSPECIFIED; (*smic).read_pos = 3; }
    if (*smic).truncated != 0 { *data.add(2) = IPMI_ERR_MSG_TRUNCATED; (*smic).truncated = 0; }
    (*smic).read_pos
}

unsafe fn read_smic_flags(smic: *mut si_sm_data) -> u8 { (*(*smic).io).inputb((*smic).io, 2) }
unsafe fn read_smic_status(smic: *mut si_sm_data) -> u8 { (*(*smic).io).inputb((*smic).io, 1) }
unsafe fn read_smic_data(smic: *mut si_sm_data) -> u8 { (*(*smic).io).inputb((*smic).io, 0) }
unsafe fn write_smic_flags(smic: *mut si_sm_data, flags: u8) { (*(*smic).io).outputb((*smic).io, 2, flags); }
unsafe fn write_smic_control(smic: *mut si_sm_data, control: u8) { (*(*smic).io).outputb((*smic).io, 1, control); }
unsafe fn write_si_sm_data(smic: *mut si_sm_data, data: u8) { (*(*smic).io).outputb((*smic).io, 0, data); }

unsafe fn start_error_recovery(smic: *mut si_sm_data, reason: *const u8) {
    (*smic).error_retries += 1;
    if (*smic).error_retries > SMIC_MAX_ERROR_RETRIES { if smic_debug & SMIC_DEBUG_ENABLE != 0 { pr_warn("ipmi_smic_drv: smic hosed: %s\n", reason); } (*smic).state = smic_states::SMIC_HOSED; }
    else { (*smic).write_count = (*smic).orig_write_count; (*smic).write_pos = 0; (*smic).read_pos = 0; (*smic).state = smic_states::SMIC_START_OP; (*smic).smic_timeout = SMIC_RETRY_TIMEOUT; }
}
unsafe fn write_next_byte(smic: *mut si_sm_data) { write_si_sm_data(smic, (*smic).write_data[(*smic).write_pos as usize]); (*smic).write_pos += 1; (*smic).write_count -= 1; }
unsafe fn read_next_byte(smic: *mut si_sm_data) { if (*smic).read_pos >= MAX_SMIC_READ_SIZE as i32 { read_smic_data(smic); (*smic).truncated = 1; } else { (*smic).read_data[(*smic).read_pos as usize] = read_smic_data(smic); (*smic).read_pos += 1; } }

// Control/status codes used by the SMIC state machine.
pub const SMIC_SC_SMS_READY: u8 = 0xc0;
pub const SMIC_SC_SMS_WR_START: u8 = 0xc1;
pub const SMIC_SC_SMS_WR_NEXT: u8 = 0xc2;
pub const SMIC_SC_SMS_WR_END: u8 = 0xc3;
pub const SMIC_SC_SMS_RD_START: u8 = 0xc4;
pub const SMIC_SC_SMS_RD_NEXT: u8 = 0xc5;
pub const SMIC_SC_SMS_RD_END: u8 = 0xc6;
pub const SMIC_CC_SMS_GET_STATUS: u8 = 0x40;
pub const SMIC_CC_SMS_WR_START: u8 = 0x41;
pub const SMIC_CC_SMS_WR_NEXT: u8 = 0x42;
pub const SMIC_CC_SMS_WR_END: u8 = 0x43;
pub const SMIC_CC_SMS_RD_START: u8 = 0x44;
pub const SMIC_CC_SMS_RD_NEXT: u8 = 0x45;
pub const SMIC_CC_SMS_RD_END: u8 = 0x46;

pub unsafe fn smic_event(smic: *mut si_sm_data, time: i64) -> si_sm_result {
    let mut flags: u8; let status: u8; let data: u8;
    if matches!((*smic).state, smic_states::SMIC_HOSED) { init_smic_data(smic, (*smic).io); return SI_SM_HOSED; }
    if !matches!((*smic).state, smic_states::SMIC_IDLE) { if time < SMIC_RETRY_TIMEOUT { (*smic).smic_timeout -= time; if (*smic).smic_timeout < 0 { start_error_recovery(smic, b"smic timed out.\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; } } }
    flags = read_smic_flags(smic); if flags & SMIC_FLAG_BSY != 0 { return SI_SM_CALL_WITH_DELAY; }
    status = read_smic_status(smic);
    match (*smic).state {
        smic_states::SMIC_IDLE => if flags & SMIC_SMS_DATA_AVAIL != 0 { SI_SM_ATTN } else { SI_SM_IDLE },
        smic_states::SMIC_START_OP => { write_smic_control(smic, SMIC_CC_SMS_GET_STATUS); write_smic_flags(smic, flags | SMIC_FLAG_BSY); (*smic).state = smic_states::SMIC_OP_OK; SI_SM_CALL_WITHOUT_DELAY },
        smic_states::SMIC_OP_OK => { if status != SMIC_SC_SMS_READY { start_error_recovery(smic, b"state = SMIC_OP_OK, status != SMIC_SC_SMS_READY\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; } write_smic_control(smic, SMIC_CC_SMS_WR_START); write_next_byte(smic); write_smic_flags(smic, flags | SMIC_FLAG_BSY); (*smic).state = smic_states::SMIC_WRITE_START; SI_SM_CALL_WITHOUT_DELAY },
        smic_states::SMIC_WRITE_START | smic_states::SMIC_WRITE_NEXT => {
            let expected = if matches!((*smic).state, smic_states::SMIC_WRITE_START) { SMIC_SC_SMS_WR_START } else { SMIC_SC_SMS_WR_NEXT };
            if status != expected { start_error_recovery(smic, b"SMIC write status error\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; }
            if flags & SMIC_TX_DATA_READY == 0 { return SI_SM_CALL_WITH_DELAY; }
            if (*smic).write_count == 1 { write_smic_control(smic, SMIC_CC_SMS_WR_END); (*smic).state = smic_states::SMIC_WRITE_END; }
            else { write_smic_control(smic, SMIC_CC_SMS_WR_NEXT); (*smic).state = smic_states::SMIC_WRITE_NEXT; }
            write_next_byte(smic); write_smic_flags(smic, flags | SMIC_FLAG_BSY); SI_SM_CALL_WITHOUT_DELAY
        }
        smic_states::SMIC_WRITE_END => {
            if status != SMIC_SC_SMS_WR_END { start_error_recovery(smic, b"SMIC_WRITE_END status error\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; }
            data = read_smic_data(smic); if data != 0 { start_error_recovery(smic, b"SMIC_WRITE_END data != SUCCESS\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; }
            (*smic).state = smic_states::SMIC_WRITE2READ; SI_SM_CALL_WITHOUT_DELAY
        }
        smic_states::SMIC_WRITE2READ => {
            if flags & SMIC_RX_DATA_READY == 0 { return SI_SM_CALL_WITH_DELAY; }
            write_smic_control(smic, SMIC_CC_SMS_RD_START); write_smic_flags(smic, flags | SMIC_FLAG_BSY); (*smic).state = smic_states::SMIC_READ_START; SI_SM_CALL_WITHOUT_DELAY
        }
        smic_states::SMIC_READ_START => {
            if status != SMIC_SC_SMS_RD_START { start_error_recovery(smic, b"SMIC_READ_START status error\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; }
            if flags & SMIC_RX_DATA_READY == 0 { return SI_SM_CALL_WITH_DELAY; }
            read_next_byte(smic); write_smic_control(smic, SMIC_CC_SMS_RD_NEXT); write_smic_flags(smic, flags | SMIC_FLAG_BSY); (*smic).state = smic_states::SMIC_READ_NEXT; SI_SM_CALL_WITHOUT_DELAY
        }
        smic_states::SMIC_READ_NEXT => {
            if status == SMIC_SC_SMS_RD_END { read_next_byte(smic); write_smic_control(smic, SMIC_CC_SMS_RD_END); write_smic_flags(smic, flags | SMIC_FLAG_BSY); (*smic).state = smic_states::SMIC_READ_END; SI_SM_CALL_WITHOUT_DELAY }
            else if status == SMIC_SC_SMS_RD_NEXT { if flags & SMIC_RX_DATA_READY == 0 { return SI_SM_CALL_WITH_DELAY; } read_next_byte(smic); write_smic_control(smic, SMIC_CC_SMS_RD_NEXT); write_smic_flags(smic, flags | SMIC_FLAG_BSY); SI_SM_CALL_WITHOUT_DELAY }
            else { start_error_recovery(smic, b"SMIC_READ_NEXT status error\0".as_ptr()); SI_SM_CALL_WITH_DELAY }
        }
        smic_states::SMIC_READ_END => {
            if status != SMIC_SC_SMS_READY { start_error_recovery(smic, b"SMIC_READ_END status error\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; }
            data = read_smic_data(smic); if data != 0 { start_error_recovery(smic, b"SMIC_READ_END data != SUCCESS\0".as_ptr()); return SI_SM_CALL_WITH_DELAY; }
            (*smic).state = smic_states::SMIC_IDLE; SI_SM_TRANSACTION_COMPLETE
        }
        smic_states::SMIC_HOSED => { init_smic_data(smic, (*smic).io); SI_SM_HOSED }
    }
}

pub unsafe fn smic_detect(smic: *mut si_sm_data) -> i32 { if read_smic_flags(smic) == 0xff { 1 } else { 0 } }
pub unsafe fn smic_cleanup(_kcs: *mut si_sm_data) {}
pub fn smic_size() -> usize { core::mem::size_of::<si_sm_data>() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
