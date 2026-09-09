/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel Implementation
 * (C) Copyright IBM Corp. 2001, 2004
 * Copyright (C) 1999-2001 Cisco, Motorola
 *
 * These are the definitions needed for the command object.
 */

// Dependencies supplied by the SCTP constants and structs headers are expected
// to be available in the containing translation unit.

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum sctp_verb {
    SCTP_CMD_NOP = 0,
    SCTP_CMD_NEW_ASOC,
    SCTP_CMD_DELETE_TCB,
    SCTP_CMD_NEW_STATE,
    SCTP_CMD_REPORT_TSN,
    SCTP_CMD_GEN_SACK,
    SCTP_CMD_PROCESS_SACK,
    SCTP_CMD_GEN_INIT_ACK,
    SCTP_CMD_PEER_INIT,
    SCTP_CMD_GEN_COOKIE_ECHO,
    SCTP_CMD_CHUNK_ULP,
    SCTP_CMD_EVENT_ULP,
    SCTP_CMD_REPLY,
    SCTP_CMD_SEND_PKT,
    SCTP_CMD_RETRAN,
    SCTP_CMD_ECN_CE,
    SCTP_CMD_ECN_ECNE,
    SCTP_CMD_ECN_CWR,
    SCTP_CMD_TIMER_START,
    SCTP_CMD_TIMER_START_ONCE,
    SCTP_CMD_TIMER_RESTART,
    SCTP_CMD_TIMER_STOP,
    SCTP_CMD_INIT_CHOOSE_TRANSPORT,
    SCTP_CMD_INIT_COUNTER_RESET,
    SCTP_CMD_INIT_COUNTER_INC,
    SCTP_CMD_INIT_RESTART,
    SCTP_CMD_COOKIEECHO_RESTART,
    SCTP_CMD_INIT_FAILED,
    SCTP_CMD_REPORT_DUP,
    SCTP_CMD_STRIKE,
    SCTP_CMD_HB_TIMERS_START,
    SCTP_CMD_HB_TIMER_UPDATE,
    SCTP_CMD_HB_TIMERS_STOP,
    SCTP_CMD_PROBE_TIMER_UPDATE,
    SCTP_CMD_TRANSPORT_HB_SENT,
    SCTP_CMD_TRANSPORT_IDLE,
    SCTP_CMD_TRANSPORT_ON,
    SCTP_CMD_REPORT_ERROR,
    SCTP_CMD_REPORT_BAD_TAG,
    SCTP_CMD_PROCESS_CTSN,
    SCTP_CMD_ASSOC_FAILED,
    SCTP_CMD_DISCARD_PACKET,
    SCTP_CMD_GEN_SHUTDOWN,
    SCTP_CMD_PURGE_OUTQUEUE,
    SCTP_CMD_SETUP_T2,
    SCTP_CMD_RTO_PENDING,
    SCTP_CMD_PART_DELIVER,
    SCTP_CMD_RENEGE,
    SCTP_CMD_SETUP_T4,
    SCTP_CMD_PROCESS_OPERR,
    SCTP_CMD_REPORT_FWDTSN,
    SCTP_CMD_PROCESS_FWDTSN,
    SCTP_CMD_CLEAR_INIT_TAG,
    SCTP_CMD_DEL_NON_PRIMARY,
    SCTP_CMD_T3_RTX_TIMERS_STOP,
    SCTP_CMD_FORCE_PRIM_RETRAN,
    SCTP_CMD_SET_SK_ERR,
    SCTP_CMD_ASSOC_CHANGE,
    SCTP_CMD_ADAPTATION_IND,
    SCTP_CMD_PEER_NO_AUTH,
    SCTP_CMD_ASSOC_SHKEY,
    SCTP_CMD_T1_RETRAN,
    SCTP_CMD_UPDATE_INITTAG,
    SCTP_CMD_SEND_MSG,
    SCTP_CMD_PURGE_ASCONF_QUEUE,
    SCTP_CMD_SET_ASOC,
    SCTP_CMD_LAST,
}

pub const SCTP_MAX_NUM_COMMANDS: usize = 20;

#[repr(C)]
pub union sctp_arg {
    pub zero_all: *mut core::ffi::c_void,
    pub i32: __s32,
    pub u32: __u32,
    pub be32: __be32,
    pub u16: __u16,
    pub u8: __u8,
    pub error: core::ffi::c_int,
    pub err: __be16,
    pub state: sctp_state,
    pub to: sctp_event_timeout,
    pub chunk: *mut sctp_chunk,
    pub asoc: *mut sctp_association,
    pub transport: *mut sctp_transport,
    pub bp: *mut sctp_bind_addr,
    pub init: *mut sctp_init_chunk,
    pub ulpevent: *mut sctp_ulpevent,
    pub packet: *mut sctp_packet,
    pub sackh: *mut sctp_sackhdr,
    pub msg: *mut sctp_datamsg,
}

#[inline]
pub fn SCTP_I32(arg: __s32) -> sctp_arg { sctp_arg { i32: arg } }
#[inline]
pub fn SCTP_U32(arg: __u32) -> sctp_arg { sctp_arg { u32: arg } }
#[inline]
pub fn SCTP_BE32(arg: __be32) -> sctp_arg { sctp_arg { be32: arg } }
#[inline]
pub fn SCTP_U16(arg: __u16) -> sctp_arg { sctp_arg { u16: arg } }
#[inline]
pub fn SCTP_U8(arg: __u8) -> sctp_arg { sctp_arg { u8: arg } }
#[inline]
pub fn SCTP_ERROR(arg: core::ffi::c_int) -> sctp_arg { sctp_arg { error: arg } }
#[inline]
pub fn SCTP_PERR(arg: __be16) -> sctp_arg { sctp_arg { err: arg } }
#[inline]
pub fn SCTP_STATE(arg: sctp_state) -> sctp_arg { sctp_arg { state: arg } }
#[inline]
pub fn SCTP_TO(arg: sctp_event_timeout) -> sctp_arg { sctp_arg { to: arg } }
#[inline]
pub fn SCTP_CHUNK(arg: *mut sctp_chunk) -> sctp_arg { sctp_arg { chunk: arg } }
#[inline]
pub fn SCTP_ASOC(arg: *mut sctp_association) -> sctp_arg { sctp_arg { asoc: arg } }
#[inline]
pub fn SCTP_TRANSPORT(arg: *mut sctp_transport) -> sctp_arg { sctp_arg { transport: arg } }
#[inline]
pub fn SCTP_BA(arg: *mut sctp_bind_addr) -> sctp_arg { sctp_arg { bp: arg } }
#[inline]
pub fn SCTP_PEER_INIT(arg: *mut sctp_init_chunk) -> sctp_arg { sctp_arg { init: arg } }
#[inline]
pub fn SCTP_ULPEVENT(arg: *mut sctp_ulpevent) -> sctp_arg { sctp_arg { ulpevent: arg } }
#[inline]
pub fn SCTP_PACKET(arg: *mut sctp_packet) -> sctp_arg { sctp_arg { packet: arg } }
#[inline]
pub fn SCTP_SACKH(arg: *mut sctp_sackhdr) -> sctp_arg { sctp_arg { sackh: arg } }
#[inline]
pub fn SCTP_DATAMSG(arg: *mut sctp_datamsg) -> sctp_arg { sctp_arg { msg: arg } }

#[inline]
pub fn SCTP_FORCE() -> sctp_arg { SCTP_I32(1) }
#[inline]
pub fn SCTP_NOFORCE() -> sctp_arg { SCTP_I32(0) }
#[inline]
pub fn SCTP_NULL() -> sctp_arg { sctp_arg { zero_all: core::ptr::null_mut() } }

#[repr(C)]
pub struct sctp_cmd {
    pub obj: sctp_arg,
    pub verb: sctp_verb,
}

#[repr(C)]
pub struct sctp_cmd_seq {
    pub cmds: [sctp_cmd; SCTP_MAX_NUM_COMMANDS],
    pub last_used_slot: *mut sctp_cmd,
    pub next_cmd: *mut sctp_cmd,
}

#[inline]
pub unsafe fn sctp_init_cmd_seq(seq: *mut sctp_cmd_seq) -> core::ffi::c_int {
    (*seq).last_used_slot = (*seq).cmds.as_mut_ptr().add(SCTP_MAX_NUM_COMMANDS);
    (*seq).next_cmd = (*seq).last_used_slot;
    1
}

#[inline]
pub unsafe fn sctp_add_cmd_sf(seq: *mut sctp_cmd_seq, verb: sctp_verb, obj: sctp_arg) {
    let cmd = (*seq).last_used_slot.sub(1);
    // Corresponds to the kernel BUG_ON(cmd < seq->cmds).
    BUG_ON!(cmd < (*seq).cmds.as_mut_ptr());
    (*cmd).verb = verb;
    (*cmd).obj = obj;
    (*seq).last_used_slot = cmd;
}

#[inline]
pub unsafe fn sctp_next_cmd(seq: *mut sctp_cmd_seq) -> *mut sctp_cmd {
    if (*seq).next_cmd <= (*seq).last_used_slot {
        return core::ptr::null_mut();
    }
    (*seq).next_cmd = (*seq).next_cmd.sub(1);
    (*seq).next_cmd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
