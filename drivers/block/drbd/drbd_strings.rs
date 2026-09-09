// SPDX-License-Identifier: GPL-2.0-only
/*
  drbd.h

  This file is part of DRBD by Philipp Reisner and Lars Ellenberg.

  Copyright (C) 2003-2008, LINBIT Information Technologies GmbH.
  Copyright (C) 2003-2008, Philipp Reisner <philipp.reisner@linbit.com>.
  Copyright (C) 2003-2008, Lars Ellenberg <lars.ellenberg@linbit.com>.
*/

use core::ffi::c_char;

// The enum constants below are supplied by the translated DRBD headers.
static DRBD_CONN_S_NAMES: &[(isize, &[u8])] = &[
    (C_STANDALONE as isize, b"StandAlone\0"),
    (C_DISCONNECTING as isize, b"Disconnecting\0"),
    (C_UNCONNECTED as isize, b"Unconnected\0"),
    (C_TIMEOUT as isize, b"Timeout\0"),
    (C_BROKEN_PIPE as isize, b"BrokenPipe\0"),
    (C_NETWORK_FAILURE as isize, b"NetworkFailure\0"),
    (C_PROTOCOL_ERROR as isize, b"ProtocolError\0"),
    (C_WF_CONNECTION as isize, b"WFConnection\0"),
    (C_WF_REPORT_PARAMS as isize, b"WFReportParams\0"),
    (C_TEAR_DOWN as isize, b"TearDown\0"),
    (C_CONNECTED as isize, b"Connected\0"),
    (C_STARTING_SYNC_S as isize, b"StartingSyncS\0"),
    (C_STARTING_SYNC_T as isize, b"StartingSyncT\0"),
    (C_WF_BITMAP_S as isize, b"WFBitMapS\0"),
    (C_WF_BITMAP_T as isize, b"WFBitMapT\0"),
    (C_WF_SYNC_UUID as isize, b"WFSyncUUID\0"),
    (C_SYNC_SOURCE as isize, b"SyncSource\0"),
    (C_SYNC_TARGET as isize, b"SyncTarget\0"),
    (C_PAUSED_SYNC_S as isize, b"PausedSyncS\0"),
    (C_PAUSED_SYNC_T as isize, b"PausedSyncT\0"),
    (C_VERIFY_S as isize, b"VerifyS\0"),
    (C_VERIFY_T as isize, b"VerifyT\0"),
    (C_AHEAD as isize, b"Ahead\0"),
    (C_BEHIND as isize, b"Behind\0"),
];

static DRBD_ROLE_S_NAMES: &[(isize, &[u8])] = &[
    (R_PRIMARY as isize, b"Primary\0"),
    (R_SECONDARY as isize, b"Secondary\0"),
    (R_UNKNOWN as isize, b"Unknown\0"),
];

static DRBD_DISK_S_NAMES: &[(isize, &[u8])] = &[
    (D_DISKLESS as isize, b"Diskless\0"),
    (D_ATTACHING as isize, b"Attaching\0"),
    (D_FAILED as isize, b"Failed\0"),
    (D_NEGOTIATING as isize, b"Negotiating\0"),
    (D_INCONSISTENT as isize, b"Inconsistent\0"),
    (D_OUTDATED as isize, b"Outdated\0"),
    (D_UNKNOWN as isize, b"DUnknown\0"),
    (D_CONSISTENT as isize, b"Consistent\0"),
    (D_UP_TO_DATE as isize, b"UpToDate\0"),
];

static DRBD_STATE_SW_ERRORS: &[(isize, &[u8])] = &[
    (-SS_TWO_PRIMARIES as isize, b"Multiple primaries not allowed by config\0"),
    (-SS_NO_UP_TO_DATE_DISK as isize, b"Need access to UpToDate data\0"),
    (-SS_NO_LOCAL_DISK as isize, b"Can not resync without local disk\0"),
    (-SS_NO_REMOTE_DISK as isize, b"Can not resync without remote disk\0"),
    (-SS_CONNECTED_OUTDATES as isize, b"Refusing to be Outdated while Connected\0"),
    (-SS_PRIMARY_NOP as isize, b"Refusing to be Primary while peer is not outdated\0"),
    (-SS_RESYNC_RUNNING as isize, b"Can not start OV/resync since it is already active\0"),
    (-SS_ALREADY_STANDALONE as isize, b"Can not disconnect a StandAlone device\0"),
    (-SS_CW_FAILED_BY_PEER as isize, b"State change was refused by peer node\0"),
    (-SS_IS_DISKLESS as isize, b"Device is diskless, the requested operation requires a disk\0"),
    (-SS_DEVICE_IN_USE as isize, b"Device is held open by someone\0"),
    (-SS_NO_NET_CONFIG as isize, b"Have no net/connection configuration\0"),
    (-SS_NO_VERIFY_ALG as isize, b"Need a verify algorithm to start online verify\0"),
    (-SS_NEED_CONNECTION as isize, b"Need a connection to start verify or resync\0"),
    (-SS_NOT_SUPPORTED as isize, b"Peer does not support protocol\0"),
    (-SS_LOWER_THAN_OUTDATED as isize, b"Disk state is lower than outdated\0"),
    (-SS_IN_TRANSIENT_STATE as isize, b"In transient state, retry after next state change\0"),
    (-SS_CONCURRENT_ST_CHG as isize, b"Concurrent state changes detected and aborted\0"),
    (-SS_OUTDATE_WO_CONN as isize, b"Need a connection for a graceful disconnect/outdate peer\0"),
    (-SS_O_VOL_PEER_PRI as isize, b"Other vol primary on peer not allowed by config\0"),
];

fn lookup(table: &[(isize, &[u8])], value: isize) -> *const c_char {
    for &(key, string) in table {
        if key == value { return string.as_ptr() as *const c_char; }
    }
    b"TOO_LARGE\0".as_ptr() as *const c_char
}

pub unsafe extern "C" fn drbd_conn_str(s: isize) -> *const c_char {
    if s > C_BEHIND as isize { b"TOO_LARGE\0".as_ptr() as *const c_char } else { lookup(DRBD_CONN_S_NAMES, s) }
}

pub unsafe extern "C" fn drbd_role_str(s: isize) -> *const c_char {
    if s > R_SECONDARY as isize { b"TOO_LARGE\0".as_ptr() as *const c_char } else { lookup(DRBD_ROLE_S_NAMES, s) }
}

pub unsafe extern "C" fn drbd_disk_str(s: isize) -> *const c_char {
    if s > D_UP_TO_DATE as isize { b"TOO_LARGE\0".as_ptr() as *const c_char } else { lookup(DRBD_DISK_S_NAMES, s) }
}

pub unsafe extern "C" fn drbd_set_st_err_str(err: isize) -> *const c_char {
    if err <= SS_AFTER_LAST_ERROR as isize {
        b"TOO_SMALL\0".as_ptr() as *const c_char
    } else if err > SS_TWO_PRIMARIES as isize {
        b"TOO_LARGE\0".as_ptr() as *const c_char
    } else {
        lookup(DRBD_STATE_SW_ERRORS, -err)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
