// SPDX-License-Identifier: GPL-2.0+
/*
 * ipmi_kcs_sm.c
 *
 * State machine for handling IPMI KCS interfaces.
 *
 * Author: MontaVista Software, Inc.
 *         Corey Minyard <minyard@mvista.com>
 *         source@mvista.com
 *
 * Copyright 2002 MontaVista Software Inc.
 */

/* This state machine is taken from the state machine in the IPMI spec,
 * pretty much verbatim.  If you have questions about the states, see
 * that document.
 */

// External kernel/IPMI symbols supplied by the surrounding translation unit.
const KCS_DEBUG_STATES: i32 = 4;
const KCS_DEBUG_MSG: i32 = 2;
const KCS_DEBUG_ENABLE: i32 = 1;
static mut kcs_debug: i32 = 0;

#[repr(C)]
pub struct si_sm_io {
    pub inputb: unsafe extern "C" fn(*mut si_sm_io, u32) -> u8,
    pub outputb: unsafe extern "C" fn(*mut si_sm_io, u32, u8),
    pub dev: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kcs_states {
    KCS_IDLE,
    KCS_START_OP,
    KCS_WAIT_WRITE_START,
    KCS_WAIT_WRITE,
    KCS_WAIT_WRITE_END,
    KCS_WAIT_READ,
    KCS_ERROR0,
    KCS_ERROR1,
    KCS_ERROR2,
    KCS_ERROR3,
    KCS_HOSED,
}

const MAX_KCS_READ_SIZE: usize = IPMI_MAX_MSG_LENGTH as usize;
const MAX_KCS_WRITE_SIZE: usize = IPMI_MAX_MSG_LENGTH as usize;
const IBF_RETRY_TIMEOUT: i64 = 5 * USEC_PER_SEC as i64;
const OBF_RETRY_TIMEOUT: i64 = 5 * USEC_PER_SEC as i64;
const MAX_ERROR_RETRIES: u32 = 2;
const ERROR0_OBF_WAIT_JIFFIES: u64 = 2 * HZ as u64;

#[repr(C)]
pub struct si_sm_data {
    state: kcs_states,
    io: *mut si_sm_io,
    write_data: [u8; MAX_KCS_WRITE_SIZE],
    write_pos: i32,
    write_count: i32,
    orig_write_count: i32,
    read_data: [u8; MAX_KCS_READ_SIZE],
    read_pos: i32,
    truncated: i32,
    error_retries: u32,
    ibf_timeout: i64,
    obf_timeout: i64,
    error0_timeout: u64,
}

const KCS_GET_STATUS_ABORT: u8 = 0x60;
const KCS_WRITE_START: u8 = 0x61;
const KCS_WRITE_END: u8 = 0x62;
const KCS_READ_BYTE: u8 = 0x68;
const KCS_IDLE_STATE: u8 = 0;
const KCS_READ_STATE: u8 = 1;
const KCS_WRITE_STATE: u8 = 2;
const KCS_ERROR_STATE: u8 = 3;

#[inline] unsafe fn read_status(kcs: *mut si_sm_data) -> u8 { ((*(*kcs).io).inputb)((*kcs).io, 1) }
#[inline] unsafe fn read_data(kcs: *mut si_sm_data) -> u8 { ((*(*kcs).io).inputb)((*kcs).io, 0) }
#[inline] unsafe fn write_cmd(kcs: *mut si_sm_data, data: u8) { ((*(*kcs).io).outputb)((*kcs).io, 1, data); }
#[inline] unsafe fn write_data(kcs: *mut si_sm_data, data: u8) { ((*(*kcs).io).outputb)((*kcs).io, 0, data); }
#[inline] fn get_status_state(status: u8) -> u8 { (status >> 6) & 3 }
#[inline] fn get_status_atn(status: u8) -> u8 { status & 4 }
#[inline] fn get_status_ibf(status: u8) -> u8 { status & 2 }
#[inline] fn get_status_obf(status: u8) -> u8 { status & 1 }

unsafe fn init_kcs_data(kcs: *mut si_sm_data, io: *mut si_sm_io) -> u32 {
    (*kcs).state = kcs_states::KCS_IDLE; (*kcs).io = io;
    (*kcs).write_pos = 0; (*kcs).write_count = 0; (*kcs).orig_write_count = 0;
    (*kcs).read_pos = 0; (*kcs).error_retries = 0; (*kcs).truncated = 0;
    (*kcs).ibf_timeout = IBF_RETRY_TIMEOUT; (*kcs).obf_timeout = OBF_RETRY_TIMEOUT;
    2
}

unsafe fn write_next_byte(kcs: *mut si_sm_data) { write_data(kcs, (*kcs).write_data[(*kcs).write_pos as usize]); (*kcs).write_pos += 1; (*kcs).write_count -= 1; }
unsafe fn start_error_recovery(kcs: *mut si_sm_data, _reason: *const i8) {
    (*kcs).error_retries += 1;
    if (*kcs).error_retries > MAX_ERROR_RETRIES { (*kcs).state = kcs_states::KCS_HOSED; }
    else { (*kcs).error0_timeout = jiffies().wrapping_add(ERROR0_OBF_WAIT_JIFFIES); (*kcs).state = kcs_states::KCS_ERROR0; }
}
unsafe fn read_next_byte(kcs: *mut si_sm_data) {
    if (*kcs).read_pos as usize >= MAX_KCS_READ_SIZE { read_data(kcs); (*kcs).truncated = 1; }
    else { (*kcs).read_data[(*kcs).read_pos as usize] = read_data(kcs); (*kcs).read_pos += 1; }
    write_data(kcs, KCS_READ_BYTE);
}
unsafe fn check_ibf(kcs: *mut si_sm_data, status: u8, time: i64) -> i32 {
    if get_status_ibf(status) != 0 { (*kcs).ibf_timeout -= time; if (*kcs).ibf_timeout < 0 { start_error_recovery(kcs, core::ptr::null()); (*kcs).ibf_timeout = IBF_RETRY_TIMEOUT; return 1; } return 0; }
    (*kcs).ibf_timeout = IBF_RETRY_TIMEOUT; 1
}
unsafe fn check_obf(kcs: *mut si_sm_data, status: u8, time: i64) -> i32 {
    if get_status_obf(status) == 0 { (*kcs).obf_timeout -= time; if (*kcs).obf_timeout < 0 { (*kcs).obf_timeout = OBF_RETRY_TIMEOUT; start_error_recovery(kcs, core::ptr::null()); return 1; } return 0; }
    (*kcs).obf_timeout = OBF_RETRY_TIMEOUT; 1
}
unsafe fn clear_obf(kcs: *mut si_sm_data, status: u8) { if get_status_obf(status) != 0 { read_data(kcs); } }

unsafe fn restart_kcs_transaction(kcs: *mut si_sm_data) { (*kcs).write_count = (*kcs).orig_write_count; (*kcs).write_pos = 0; (*kcs).read_pos = 0; (*kcs).state = kcs_states::KCS_WAIT_WRITE_START; (*kcs).ibf_timeout = IBF_RETRY_TIMEOUT; (*kcs).obf_timeout = OBF_RETRY_TIMEOUT; write_cmd(kcs, KCS_WRITE_START); }

unsafe fn start_kcs_transaction(kcs: *mut si_sm_data, data: *const u8, size: u32) -> i32 {
    if size < 2 { return IPMI_REQ_LEN_INVALID_ERR; } if size as usize > MAX_KCS_WRITE_SIZE { return IPMI_REQ_LEN_EXCEEDED_ERR; }
    if !matches!((*kcs).state, kcs_states::KCS_IDLE | kcs_states::KCS_HOSED) { return IPMI_NOT_IN_MY_STATE_ERR; }
    core::ptr::copy_nonoverlapping(data, (*kcs).write_data.as_mut_ptr(), size as usize);
    (*kcs).error_retries = 0; (*kcs).write_count = size as i32; (*kcs).orig_write_count = size as i32; (*kcs).write_pos = 0; (*kcs).read_pos = 0; (*kcs).state = kcs_states::KCS_START_OP; (*kcs).ibf_timeout = IBF_RETRY_TIMEOUT; (*kcs).obf_timeout = OBF_RETRY_TIMEOUT; 0
}

unsafe fn get_kcs_result(kcs: *mut si_sm_data, data: *mut u8, length: u32) -> i32 {
    if length < (*kcs).read_pos as u32 { (*kcs).read_pos = length as i32; (*kcs).truncated = 1; }
    core::ptr::copy_nonoverlapping((*kcs).read_data.as_ptr(), data, (*kcs).read_pos as usize);
    if length >= 3 && (*kcs).read_pos < 3 { *data.add(2) = IPMI_ERR_UNSPECIFIED; (*kcs).read_pos = 3; }
    if (*kcs).truncated != 0 { *data.add(2) = IPMI_ERR_MSG_TRUNCATED; (*kcs).truncated = 0; }
    (*kcs).read_pos
}

unsafe fn kcs_event(kcs: *mut si_sm_data, time: i64) -> si_sm_result {
    let mut status = read_status(kcs);
    if check_ibf(kcs, status, time) == 0 { return SI_SM_CALL_WITH_DELAY; }
    let state = get_status_state(status);
    match (*kcs).state {
        kcs_states::KCS_IDLE => { clear_obf(kcs, status); if get_status_atn(status) != 0 { return SI_SM_ATTN; } return SI_SM_IDLE; }
        kcs_states::KCS_START_OP => { if state != KCS_IDLE_STATE { start_error_recovery(kcs, core::ptr::null()); } else { clear_obf(kcs, status); write_cmd(kcs, KCS_WRITE_START); (*kcs).state = kcs_states::KCS_WAIT_WRITE_START; } }
        kcs_states::KCS_WAIT_WRITE_START => { if state != KCS_WRITE_STATE { start_error_recovery(kcs, core::ptr::null()); } else { read_data(kcs); if (*kcs).write_count == 1 { write_cmd(kcs, KCS_WRITE_END); (*kcs).state = kcs_states::KCS_WAIT_WRITE_END; } else { write_next_byte(kcs); (*kcs).state = kcs_states::KCS_WAIT_WRITE; } } }
        kcs_states::KCS_WAIT_WRITE => { if state != KCS_WRITE_STATE { start_error_recovery(kcs, core::ptr::null()); } else { clear_obf(kcs, status); if (*kcs).write_count == 1 { write_cmd(kcs, KCS_WRITE_END); (*kcs).state = kcs_states::KCS_WAIT_WRITE_END; } else { write_next_byte(kcs); } } }
        kcs_states::KCS_WAIT_WRITE_END => { if state != KCS_WRITE_STATE { start_error_recovery(kcs, core::ptr::null()); } else { clear_obf(kcs, status); write_next_byte(kcs); (*kcs).state = kcs_states::KCS_WAIT_READ; } }
        kcs_states::KCS_WAIT_READ => {
            if state != KCS_READ_STATE && state != KCS_IDLE_STATE { start_error_recovery(kcs, core::ptr::null()); }
            else if state == KCS_READ_STATE { if check_obf(kcs, status, time) == 0 { return SI_SM_CALL_WITH_DELAY; } read_next_byte(kcs); }
            else { clear_obf(kcs, status); (*kcs).orig_write_count = 0; (*kcs).state = kcs_states::KCS_IDLE; return SI_SM_TRANSACTION_COMPLETE; }
        }
        kcs_states::KCS_ERROR0 => { clear_obf(kcs, status); status = read_status(kcs); if get_status_obf(status) != 0 && time_before(jiffies(), (*kcs).error0_timeout) { return SI_SM_CALL_WITH_TICK_DELAY; } write_cmd(kcs, KCS_GET_STATUS_ABORT); (*kcs).state = kcs_states::KCS_ERROR1; }
        kcs_states::KCS_ERROR1 => { clear_obf(kcs, status); write_data(kcs, 0); (*kcs).state = kcs_states::KCS_ERROR2; }
        kcs_states::KCS_ERROR2 => { if state != KCS_READ_STATE { start_error_recovery(kcs, core::ptr::null()); } else if check_obf(kcs, status, time) != 0 { clear_obf(kcs, status); write_data(kcs, KCS_READ_BYTE); (*kcs).state = kcs_states::KCS_ERROR3; } else { return SI_SM_CALL_WITH_DELAY; } }
        kcs_states::KCS_ERROR3 => { if state != KCS_IDLE_STATE { start_error_recovery(kcs, core::ptr::null()); } else if check_obf(kcs, status, time) == 0 { return SI_SM_CALL_WITH_DELAY; } else { clear_obf(kcs, status); if (*kcs).orig_write_count != 0 { restart_kcs_transaction(kcs); } else { (*kcs).state = kcs_states::KCS_IDLE; return SI_SM_TRANSACTION_COMPLETE; } } }
        kcs_states::KCS_HOSED => {}
    }
    if matches!((*kcs).state, kcs_states::KCS_HOSED) { init_kcs_data(kcs, (*kcs).io); return SI_SM_HOSED; }
    SI_SM_CALL_WITHOUT_DELAY
}

// Build-time kernel symbols and result enums are supplied by dependent files.
unsafe fn kcs_size() -> i32 { core::mem::size_of::<si_sm_data>() as i32 }
unsafe fn kcs_detect(kcs: *mut si_sm_data) -> i32 { if read_status(kcs) == 0xff { 1 } else { 0 } }
unsafe fn kcs_cleanup(_kcs: *mut si_sm_data) {}

#[repr(C)]
pub struct si_sm_handlers {
    pub init_data: unsafe fn(*mut si_sm_data, *mut si_sm_io) -> u32,
    pub start_transaction: unsafe fn(*mut si_sm_data, *const u8, u32) -> i32,
    pub get_result: unsafe fn(*mut si_sm_data, *mut u8, u32) -> i32,
    pub event: unsafe fn(*mut si_sm_data, i64) -> si_sm_result,
    pub detect: unsafe fn(*mut si_sm_data) -> i32,
    pub cleanup: unsafe fn(*mut si_sm_data),
    pub size: unsafe fn() -> i32,
}

pub static kcs_smi_handlers: si_sm_handlers = si_sm_handlers {
    init_data: init_kcs_data,
    start_transaction: start_kcs_transaction,
    get_result: get_kcs_result,
    event: kcs_event,
    detect: kcs_detect,
    cleanup: kcs_cleanup,
    size: kcs_size,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
