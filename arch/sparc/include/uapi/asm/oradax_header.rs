/* SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note */
/*
 * Copyright (c) 2017, Oracle and/or its affiliates. All rights reserved.
 */

/*
 * Oracle DAX driver API definitions
 */

// Dependency intent: the C header includes <linux/types.h> for __u16, __u32,
// and __u64.

pub const CCB_KILL: i32 = 0;
pub const CCB_INFO: i32 = 1;
pub const CCB_DEQUEUE: i32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dax_command {
    pub command: __u16,   /* CCB_KILL/INFO/DEQUEUE */
    pub ca_offset: __u16, /* offset into mmapped completion area */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccb_kill_result {
    pub action: __u16, /* action taken to kill ccb */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccb_info_result {
    pub state: __u16,    /* state of enqueued ccb */
    pub inst_num: __u16, /* dax instance number of enqueued ccb */
    pub q_num: __u16,    /* queue number of enqueued ccb */
    pub q_pos: __u16,    /* ccb position in queue */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccb_exec_result {
    pub status_data: __u64, /* additional status data (e.g. bad VA) */
    pub status: __u32,      /* one of DAX_SUBMIT_* */
}

#[repr(C)]
pub union ccb_result {
    pub exec: ccb_exec_result,
    pub info: ccb_info_result,
    pub kill: ccb_kill_result,
}

pub const DAX_MMAP_LEN: i32 = 16 * 1024;
pub const DAX_MAX_CCBS: i32 = 15;
pub const DAX_CCB_BUF_MAXLEN: i32 = DAX_MAX_CCBS * 64;
pub const DAX_NAME: &str = "oradax";

/* CCB_EXEC status */
pub const DAX_SUBMIT_OK: i32 = 0;
pub const DAX_SUBMIT_ERR_RETRY: i32 = 1;
pub const DAX_SUBMIT_ERR_WOULDBLOCK: i32 = 2;
pub const DAX_SUBMIT_ERR_BUSY: i32 = 3;
pub const DAX_SUBMIT_ERR_THR_INIT: i32 = 4;
pub const DAX_SUBMIT_ERR_ARG_INVAL: i32 = 5;
pub const DAX_SUBMIT_ERR_CCB_INVAL: i32 = 6;
pub const DAX_SUBMIT_ERR_NO_CA_AVAIL: i32 = 7;
pub const DAX_SUBMIT_ERR_CCB_ARR_MMU_MISS: i32 = 8;
pub const DAX_SUBMIT_ERR_NOMAP: i32 = 9;
pub const DAX_SUBMIT_ERR_NOACCESS: i32 = 10;
pub const DAX_SUBMIT_ERR_TOOMANY: i32 = 11;
pub const DAX_SUBMIT_ERR_UNAVAIL: i32 = 12;
pub const DAX_SUBMIT_ERR_INTERNAL: i32 = 13;

/* CCB_INFO states - must match HV_CCB_STATE_* definitions */
pub const DAX_CCB_COMPLETED: i32 = 0;
pub const DAX_CCB_ENQUEUED: i32 = 1;
pub const DAX_CCB_INPROGRESS: i32 = 2;
pub const DAX_CCB_NOTFOUND: i32 = 3;

/* CCB_KILL actions - must match HV_CCB_KILL_* definitions */
pub const DAX_KILL_COMPLETED: i32 = 0;
pub const DAX_KILL_DEQUEUED: i32 = 1;
pub const DAX_KILL_KILLED: i32 = 2;
pub const DAX_KILL_NOTFOUND: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
