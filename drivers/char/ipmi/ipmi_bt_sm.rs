// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of ipmi_bt_sm.c. */

// Kernel headers and ipmi_si_sm.h provide the constants, types, logging,
// scheduling, and external symbols referenced below.

const BT_DEBUG_OFF: i32 = 0;
const BT_DEBUG_ENABLE: i32 = 1;
const BT_DEBUG_MSG: i32 = 2;
const BT_DEBUG_STATES: i32 = 4;
static mut bt_debug: i32 = 0;

const BT_NORMAL_TIMEOUT: i64 = 5;
const BT_NORMAL_RETRY_LIMIT: i32 = 2;
const BT_RESET_DELAY: i64 = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bt_states {
    BT_STATE_IDLE = 0,
    BT_STATE_XACTION_START,
    BT_STATE_WRITE_BYTES,
    BT_STATE_WRITE_CONSUME,
    BT_STATE_READ_WAIT,
    BT_STATE_CLEAR_B2H,
    BT_STATE_READ_BYTES,
    BT_STATE_RESET1,
    BT_STATE_RESET2,
    BT_STATE_RESET3,
    BT_STATE_RESTART,
    BT_STATE_PRINTME,
    BT_STATE_LONG_BUSY,
}

#[repr(C)]
pub struct si_sm_data {
    state: bt_states,
    seq: u8,
    io: *mut si_sm_io,
    write_data: [u8; IPMI_MAX_MSG_LENGTH + 2],
    write_count: i32,
    read_data: [u8; IPMI_MAX_MSG_LENGTH + 2],
    read_count: i32,
    truncated: i32,
    timeout: i64,
    error_retries: i32,
    nonzero_status: i32,
    complete: bt_states,
    BT_CAP_req2rsp: i64,
    BT_CAP_retries: i32,
}

const BT_CLR_WR_PTR: u8 = 0x01;
const BT_CLR_RD_PTR: u8 = 0x02;
const BT_H2B_ATN: u8 = 0x04;
const BT_B2H_ATN: u8 = 0x08;
const BT_SMS_ATN: u8 = 0x10;
const BT_OEM0: u8 = 0x20;
const BT_H_BUSY: u8 = 0x40;
const BT_B_BUSY: u8 = 0x80;
const BT_BMC_HWRST: u8 = 0x80;

unsafe fn status(bt: *mut si_sm_data) -> u8 { ((*(*bt).io).inputb)((*bt).io, 0) }
unsafe fn control(bt: *mut si_sm_data, x: u8) { ((*(*bt).io).outputb)((*bt).io, 0, x); }
unsafe fn bmc2host(bt: *mut si_sm_data) -> u8 { ((*(*bt).io).inputb)((*bt).io, 1) }
unsafe fn host2bmc(bt: *mut si_sm_data, x: u8) { ((*(*bt).io).outputb)((*bt).io, 1, x); }
unsafe fn intmask_r(bt: *mut si_sm_data) -> u8 { ((*(*bt).io).inputb)((*bt).io, 2) }
unsafe fn intmask_w(bt: *mut si_sm_data, x: u8) { ((*(*bt).io).outputb)((*bt).io, 2, x); }

unsafe fn state2txt(state: bt_states) -> &'static str {
    match state {
        bt_states::BT_STATE_IDLE => "IDLE", bt_states::BT_STATE_XACTION_START => "XACTION",
        bt_states::BT_STATE_WRITE_BYTES => "WR_BYTES", bt_states::BT_STATE_WRITE_CONSUME => "WR_CONSUME",
        bt_states::BT_STATE_READ_WAIT => "RD_WAIT", bt_states::BT_STATE_CLEAR_B2H => "CLEAR_B2H",
        bt_states::BT_STATE_READ_BYTES => "RD_BYTES", bt_states::BT_STATE_RESET1 => "RESET1",
        bt_states::BT_STATE_RESET2 => "RESET2", bt_states::BT_STATE_RESET3 => "RESET3",
        bt_states::BT_STATE_RESTART => "RESTART", bt_states::BT_STATE_LONG_BUSY => "LONG_BUSY",
        bt_states::BT_STATE_PRINTME => "BAD STATE",
    }
}

unsafe fn bt_init_data(bt: *mut si_sm_data, io: *mut si_sm_io) -> u32 {
    let old_io = (*bt).io;
    core::ptr::write_bytes(bt, 0, 1);
    if old_io != io { (*bt).io = io; (*bt).seq = 0; }
    (*bt).state = bt_states::BT_STATE_IDLE;
    (*bt).complete = bt_states::BT_STATE_IDLE;
    (*bt).BT_CAP_req2rsp = BT_NORMAL_TIMEOUT * USEC_PER_SEC as i64;
    (*bt).BT_CAP_retries = BT_NORMAL_RETRY_LIMIT;
    3
}

unsafe fn force_result(bt: *mut si_sm_data, code: u8) {
    (*bt).read_data[0] = 4; (*bt).read_data[1] = (*bt).write_data[1] | 4;
    (*bt).read_data[2] = (*bt).write_data[2]; (*bt).read_data[3] = (*bt).write_data[3];
    (*bt).read_data[4] = code; (*bt).read_count = 5;
}

unsafe fn bt_start_transaction(bt: *mut si_sm_data, data: *mut u8, size: u32) -> i32 {
    if size < 2 { return IPMI_REQ_LEN_INVALID_ERR; }
    if size > IPMI_MAX_MSG_LENGTH as u32 { return IPMI_REQ_LEN_EXCEEDED_ERR; }
    if (*bt).state == bt_states::BT_STATE_LONG_BUSY { return IPMI_NODE_BUSY_ERR; }
    if (*bt).state != bt_states::BT_STATE_IDLE { return IPMI_NOT_IN_MY_STATE_ERR; }
    (*bt).write_data[0] = (size + 1) as u8; (*bt).write_data[1] = *data;
    (*bt).write_data[2] = (*bt).seq; (*bt).seq = (*bt).seq.wrapping_add(1);
    core::ptr::copy_nonoverlapping(data.add(1), (*bt).write_data.as_mut_ptr().add(3), (size - 1) as usize);
    (*bt).write_count = size as i32 + 2; (*bt).error_retries = 0;
    (*bt).nonzero_status = 0; (*bt).truncated = 0; (*bt).state = bt_states::BT_STATE_XACTION_START;
    (*bt).timeout = (*bt).BT_CAP_req2rsp; force_result(bt, IPMI_ERR_UNSPECIFIED); 0
}

unsafe fn bt_get_result(bt: *mut si_sm_data, data: *mut u8, length: u32) -> i32 {
    let mut msg_len = (*bt).read_count - 2;
    if msg_len < 3 || msg_len > IPMI_MAX_MSG_LENGTH as i32 { force_result(bt, IPMI_ERR_UNSPECIFIED); msg_len = 3; }
    *data = (*bt).read_data[1]; *data.add(1) = (*bt).read_data[3];
    if length < msg_len as u32 || (*bt).truncated != 0 { *data.add(2) = IPMI_ERR_MSG_TRUNCATED; msg_len = 3; }
    else { core::ptr::copy_nonoverlapping((*bt).read_data.as_ptr().add(4), data.add(2), (msg_len - 2) as usize); }
    msg_len
}

unsafe fn reset_flags(bt: *mut si_sm_data) { if status(bt) & BT_H_BUSY != 0 { control(bt, BT_H_BUSY); } control(bt, BT_CLR_WR_PTR); control(bt, BT_SMS_ATN); intmask_w(bt, BT_BMC_HWRST); }
unsafe fn drain_bmc2host(bt: *mut si_sm_data) { if status(bt) & BT_B2H_ATN == 0 { return; } control(bt, BT_H_BUSY); control(bt, BT_B2H_ATN); let _ = status(bt); control(bt, BT_B2H_ATN); control(bt, BT_CLR_RD_PTR); let n = bmc2host(bt); for _ in 0..n { let _ = bmc2host(bt); } control(bt, BT_H_BUSY); }
unsafe fn write_all_bytes(bt: *mut si_sm_data) { for i in 0..(*bt).write_count as usize { host2bmc(bt, (*bt).write_data[i]); } }
unsafe fn read_all_bytes(bt: *mut si_sm_data) -> i32 {
    (*bt).read_data[0] = bmc2host(bt); (*bt).read_count = (*bt).read_data[0] as i32;
    if (*bt).read_count < 4 || (*bt).read_count >= IPMI_MAX_MSG_LENGTH as i32 { (*bt).truncated = 1; return 1; }
    for i in 1..=(*bt).read_count as usize { (*bt).read_data[i] = bmc2host(bt); } (*bt).read_count += 1;
    if (*bt).read_data[3] == (*bt).write_data[3] && (*bt).read_data[2] == (*bt).write_data[2] && ((*bt).read_data[1] & 0xf8) == ((*bt).write_data[1] & 0xf8) { 1 } else { 0 }
}

unsafe fn error_recovery(bt: *mut si_sm_data, status0: u8, mut code: u8) -> si_sm_result {
    (*bt).timeout = (*bt).BT_CAP_req2rsp; (*bt).error_retries += 1;
    if (*bt).error_retries < (*bt).BT_CAP_retries { (*bt).state = bt_states::BT_STATE_RESTART; return SI_SM_CALL_WITHOUT_DELAY; }
    if (*bt).nonzero_status == 0 { } else if (*bt).seq <= ((*bt).BT_CAP_retries & 0xff) as u8 { (*bt).state = bt_states::BT_STATE_RESET1; return SI_SM_CALL_WITHOUT_DELAY; }
    (*bt).state = bt_states::BT_STATE_IDLE;
    if code == IPMI_TIMEOUT_ERR && status0 & BT_B_BUSY != 0 { code = IPMI_NODE_BUSY_ERR; (*bt).state = bt_states::BT_STATE_LONG_BUSY; }
    force_result(bt, code); SI_SM_TRANSACTION_COMPLETE
}

unsafe fn bt_event(bt: *mut si_sm_data, time: i64) -> si_sm_result {
    let status0 = status(bt); (*bt).nonzero_status |= status0 as i32;
    if ((*bt).state as i32) < BT_STATE_WRITE_BYTES as i32 && status0 & BT_B2H_ATN != 0 { drain_bmc2host(bt); return SI_SM_CALL_WITH_DELAY; }
    if (*bt).state != bt_states::BT_STATE_IDLE && (*bt).state as i32 < BT_STATE_PRINTME as i32 { (*bt).timeout -= time; if (*bt).timeout < 0 && (*bt).state as i32 < BT_STATE_RESET1 as i32 { return error_recovery(bt, status0, IPMI_TIMEOUT_ERR); } }
    match (*bt).state {
        bt_states::BT_STATE_IDLE => { if status0 & BT_SMS_ATN != 0 { control(bt, BT_SMS_ATN); return SI_SM_ATTN; } if status0 & BT_H_BUSY != 0 { control(bt, BT_H_BUSY); } SI_SM_IDLE }
        bt_states::BT_STATE_XACTION_START => { if status0 & (BT_B_BUSY|BT_H2B_ATN) != 0 { return SI_SM_CALL_WITH_DELAY; } if status(bt)&BT_H_BUSY != 0 { control(bt,BT_H_BUSY); } (*bt).state=bt_states::BT_STATE_WRITE_BYTES; SI_SM_CALL_WITHOUT_DELAY }
        bt_states::BT_STATE_WRITE_BYTES => { if status0&BT_H_BUSY!=0 {control(bt,BT_H_BUSY);} control(bt,BT_CLR_WR_PTR); write_all_bytes(bt); control(bt,BT_H2B_ATN); (*bt).state=bt_states::BT_STATE_WRITE_CONSUME; SI_SM_CALL_WITHOUT_DELAY }
        bt_states::BT_STATE_WRITE_CONSUME => { if status0&(BT_B_BUSY|BT_H2B_ATN)!=0 {return SI_SM_CALL_WITH_DELAY;} (*bt).state=bt_states::BT_STATE_READ_WAIT; SI_SM_CALL_WITHOUT_DELAY }
        bt_states::BT_STATE_READ_WAIT => { if status0&BT_B2H_ATN==0{return SI_SM_CALL_WITH_DELAY;} control(bt,BT_H_BUSY); control(bt,BT_B2H_ATN); (*bt).state=bt_states::BT_STATE_CLEAR_B2H; SI_SM_CALL_WITHOUT_DELAY }
        bt_states::BT_STATE_CLEAR_B2H => { if status0&BT_B2H_ATN!=0 {control(bt,BT_B2H_ATN);return SI_SM_CALL_WITH_DELAY;} (*bt).state=bt_states::BT_STATE_READ_BYTES; SI_SM_CALL_WITHOUT_DELAY }
        bt_states::BT_STATE_READ_BYTES => { if status0&BT_H_BUSY==0 {control(bt,BT_H_BUSY);} control(bt,BT_CLR_RD_PTR); let ok=read_all_bytes(bt); control(bt,BT_H_BUSY); if ok==0 {(*bt).state=bt_states::BT_STATE_READ_WAIT;return SI_SM_CALL_WITHOUT_DELAY;} (*bt).state=(*bt).complete; if (*bt).state==bt_states::BT_STATE_IDLE {SI_SM_TRANSACTION_COMPLETE} else {SI_SM_CALL_WITHOUT_DELAY} }
        bt_states::BT_STATE_LONG_BUSY => { if status0&BT_B_BUSY==0 {reset_flags(bt);bt_init_data(bt,(*bt).io);} SI_SM_CALL_WITH_DELAY }
        bt_states::BT_STATE_RESET1 => {reset_flags(bt);drain_bmc2host(bt);(*bt).state=bt_states::BT_STATE_RESET2;SI_SM_CALL_WITH_DELAY}
        bt_states::BT_STATE_RESET2 => {control(bt,BT_CLR_WR_PTR);host2bmc(bt,3);host2bmc(bt,0x18);host2bmc(bt,42);host2bmc(bt,3);control(bt,BT_H2B_ATN);(*bt).timeout=BT_RESET_DELAY*USEC_PER_SEC as i64;(*bt).state=bt_states::BT_STATE_RESET3;SI_SM_CALL_WITH_DELAY}
        bt_states::BT_STATE_RESET3 => {if (*bt).timeout>0{return SI_SM_CALL_WITH_DELAY;}drain_bmc2host(bt);(*bt).state=bt_states::BT_STATE_RESTART;SI_SM_CALL_WITH_DELAY}
        bt_states::BT_STATE_RESTART => {(*bt).read_count=0;(*bt).nonzero_status=0;(*bt).timeout=(*bt).BT_CAP_req2rsp;(*bt).state=bt_states::BT_STATE_XACTION_START;SI_SM_CALL_WITH_DELAY}
        _ => error_recovery(bt,status0,IPMI_ERR_UNSPECIFIED),
    }
}
unsafe fn bt_detect(bt: *mut si_sm_data) -> i32 { if status(bt)==0xff && intmask_r(bt)==0xff{return 1;} reset_flags(bt); 0 }
unsafe fn bt_cleanup(_bt: *mut si_sm_data) {}
fn bt_size() -> usize { core::mem::size_of::<si_sm_data>() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
