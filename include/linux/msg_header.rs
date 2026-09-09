/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/list.h and uapi/linux/msg.h

/* one msg_msg structure for each message */
#[repr(C)]
pub struct msg_msg {
    pub m_list: list_head,
    pub m_type: core::ffi::c_long,
    pub m_ts: usize, /* message text size */
    pub next: *mut msg_msgseg,
    pub security: *mut core::ffi::c_void,
    /* the actual message follows immediately */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
