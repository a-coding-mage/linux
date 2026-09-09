// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright IBM Corp. 2001, 2004
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2001 Intel Corp.
 *
 * This file is part of the SCTP kernel implementation
 *
 * This file converts numerical ID value to alphabetical names for SCTP
 * terms such as chunk type, parameter time, event type, etc.
 */

use core::ffi::c_char;

/* The declarations supplied by <net/sctp/sctp.h> are external dependencies. */

/* These are printable forms of Chunk ID's from section 3.1.  */
static SCTP_CID_TBL: [*const c_char; SCTP_NUM_BASE_CHUNK_TYPES as usize] = [
    c"DATA".as_ptr(), c"INIT".as_ptr(), c"INIT_ACK".as_ptr(), c"SACK".as_ptr(),
    c"HEARTBEAT".as_ptr(), c"HEARTBEAT_ACK".as_ptr(), c"ABORT".as_ptr(),
    c"SHUTDOWN".as_ptr(), c"SHUTDOWN_ACK".as_ptr(), c"ERROR".as_ptr(),
    c"COOKIE_ECHO".as_ptr(), c"COOKIE_ACK".as_ptr(), c"ECN_ECNE".as_ptr(),
    c"ECN_CWR".as_ptr(), c"SHUTDOWN_COMPLETE".as_ptr(),
];

/* Lookup "chunk type" debug name. */
#[no_mangle]
pub unsafe extern "C" fn sctp_cname(cid: sctp_subtype) -> *const c_char {
    if cid.chunk <= SCTP_CID_BASE_MAX {
        return SCTP_CID_TBL[cid.chunk as usize];
    }

    match cid.chunk {
        SCTP_CID_ASCONF => c"ASCONF".as_ptr(),
        SCTP_CID_ASCONF_ACK => c"ASCONF_ACK".as_ptr(),
        SCTP_CID_FWD_TSN => c"FWD_TSN".as_ptr(),
        SCTP_CID_AUTH => c"AUTH".as_ptr(),
        SCTP_CID_RECONF => c"RECONF".as_ptr(),
        SCTP_CID_I_DATA => c"I_DATA".as_ptr(),
        SCTP_CID_I_FWD_TSN => c"I_FWD_TSN".as_ptr(),
        _ => c"unknown chunk".as_ptr(),
    }
}

/* These are printable forms of the states.  */
#[no_mangle]
pub static sctp_state_tbl: [*const c_char; SCTP_STATE_NUM_STATES as usize] = [
    c"STATE_CLOSED".as_ptr(), c"STATE_COOKIE_WAIT".as_ptr(),
    c"STATE_COOKIE_ECHOED".as_ptr(), c"STATE_ESTABLISHED".as_ptr(),
    c"STATE_SHUTDOWN_PENDING".as_ptr(), c"STATE_SHUTDOWN_SENT".as_ptr(),
    c"STATE_SHUTDOWN_RECEIVED".as_ptr(), c"STATE_SHUTDOWN_ACK_SENT".as_ptr(),
];

/* Events that could change the state of an association.  */
#[no_mangle]
pub static sctp_evttype_tbl: [*const c_char; 5] = [
    c"EVENT_T_unknown".as_ptr(), c"EVENT_T_CHUNK".as_ptr(),
    c"EVENT_T_TIMEOUT".as_ptr(), c"EVENT_T_OTHER".as_ptr(),
    c"EVENT_T_PRIMITIVE".as_ptr(),
];

/* Return value of a state function */
#[no_mangle]
pub static sctp_status_tbl: [*const c_char; 9] = [
    c"DISPOSITION_DISCARD".as_ptr(), c"DISPOSITION_CONSUME".as_ptr(),
    c"DISPOSITION_NOMEM".as_ptr(), c"DISPOSITION_DELETE_TCB".as_ptr(),
    c"DISPOSITION_ABORT".as_ptr(), c"DISPOSITION_VIOLATION".as_ptr(),
    c"DISPOSITION_NOT_IMPL".as_ptr(), c"DISPOSITION_ERROR".as_ptr(),
    c"DISPOSITION_BUG".as_ptr(),
];

/* Printable forms of primitives */
static SCTP_PRIMITIVE_TBL: [*const c_char; SCTP_NUM_PRIMITIVE_TYPES as usize] = [
    c"PRIMITIVE_ASSOCIATE".as_ptr(), c"PRIMITIVE_SHUTDOWN".as_ptr(),
    c"PRIMITIVE_ABORT".as_ptr(), c"PRIMITIVE_SEND".as_ptr(),
    c"PRIMITIVE_REQUESTHEARTBEAT".as_ptr(), c"PRIMITIVE_ASCONF".as_ptr(),
];

/* Lookup primitive debug name. */
#[no_mangle]
pub unsafe extern "C" fn sctp_pname(id: sctp_subtype) -> *const c_char {
    if id.primitive <= SCTP_EVENT_PRIMITIVE_MAX {
        return SCTP_PRIMITIVE_TBL[id.primitive as usize];
    }
    c"unknown_primitive".as_ptr()
}

static SCTP_OTHER_TBL: [*const c_char; 2] = [
    c"NO_PENDING_TSN".as_ptr(), c"ICMP_PROTO_UNREACH".as_ptr(),
];

/* Lookup "other" debug name. */
#[no_mangle]
pub unsafe extern "C" fn sctp_oname(id: sctp_subtype) -> *const c_char {
    if id.other <= SCTP_EVENT_OTHER_MAX {
        return SCTP_OTHER_TBL[id.other as usize];
    }
    c"unknown 'other' event".as_ptr()
}

static SCTP_TIMER_TBL: [*const c_char; 12] = [
    c"TIMEOUT_NONE".as_ptr(), c"TIMEOUT_T1_COOKIE".as_ptr(),
    c"TIMEOUT_T1_INIT".as_ptr(), c"TIMEOUT_T2_SHUTDOWN".as_ptr(),
    c"TIMEOUT_T3_RTX".as_ptr(), c"TIMEOUT_T4_RTO".as_ptr(),
    c"TIMEOUT_T5_SHUTDOWN_GUARD".as_ptr(), c"TIMEOUT_HEARTBEAT".as_ptr(),
    c"TIMEOUT_RECONF".as_ptr(), c"TIMEOUT_PROBE".as_ptr(),
    c"TIMEOUT_SACK".as_ptr(), c"TIMEOUT_AUTOCLOSE".as_ptr(),
];

/* Lookup timer debug name. */
#[no_mangle]
pub unsafe extern "C" fn sctp_tname(id: sctp_subtype) -> *const c_char {
    // BUILD_BUG_ON(SCTP_EVENT_TIMEOUT_MAX + 1 != ARRAY_SIZE(sctp_timer_tbl));
    if id.timeout < SCTP_TIMER_TBL.len() {
        return SCTP_TIMER_TBL[id.timeout as usize];
    }
    c"unknown_timer".as_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
