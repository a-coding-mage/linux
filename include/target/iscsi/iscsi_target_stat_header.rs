/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/spinlock.h, and linux/socket.h.

/*
 * For struct iscsi_tiqn->tiqn_wwn default groups
 */
extern "C" {
    pub static iscsi_stat_instance_cit: config_item_type;
    pub static iscsi_stat_sess_err_cit: config_item_type;
    pub static iscsi_stat_tgt_attr_cit: config_item_type;
    pub static iscsi_stat_login_cit: config_item_type;
    pub static iscsi_stat_logout_cit: config_item_type;
}

/*
 * For struct iscsi_session->se_sess default groups
 */
extern "C" {
    pub static iscsi_stat_sess_cit: config_item_type;
}

/* iSCSI session error types */
pub const ISCSI_SESS_ERR_UNKNOWN: u32 = 0;
pub const ISCSI_SESS_ERR_DIGEST: u32 = 1;
pub const ISCSI_SESS_ERR_CXN_TIMEOUT: u32 = 2;
pub const ISCSI_SESS_ERR_PDU_FORMAT: u32 = 3;

/* iSCSI session error stats */
#[repr(C)]
pub struct iscsi_sess_err_stats {
    pub lock: spinlock_t,
    pub digest_errors: u32,
    pub cxn_timeout_errors: u32,
    pub pdu_format_errors: u32,
    pub last_sess_failure_type: u32,
    pub last_sess_fail_rem_name: [core::ffi::c_char; ISCSI_IQN_LEN],
}
// C attribute: ____cacheline_aligned.

/* iSCSI login failure types (sub oids) */
pub const ISCSI_LOGIN_FAIL_OTHER: u32 = 2;
pub const ISCSI_LOGIN_FAIL_REDIRECT: u32 = 3;
pub const ISCSI_LOGIN_FAIL_AUTHORIZE: u32 = 4;
pub const ISCSI_LOGIN_FAIL_AUTHENTICATE: u32 = 5;
pub const ISCSI_LOGIN_FAIL_NEGOTIATE: u32 = 6;

/* iSCSI login stats */
#[repr(C)]
pub struct iscsi_login_stats {
    pub lock: spinlock_t,
    pub accepts: u32,
    pub other_fails: u32,
    pub redirects: u32,
    pub authorize_fails: u32,
    pub authenticate_fails: u32,
    pub negotiate_fails: u32, // used for notifications
    pub last_fail_time: u64, // time stamp (jiffies)
    pub last_fail_type: u32,
    pub last_intr_fail_ip_family: i32,
    pub last_intr_fail_sockaddr: sockaddr_storage,
    pub last_intr_fail_name: [core::ffi::c_char; ISCSI_IQN_LEN],
}
// C attribute: ____cacheline_aligned.

/* iSCSI logout stats */
#[repr(C)]
pub struct iscsi_logout_stats {
    pub lock: spinlock_t,
    pub normal_logouts: u32,
    pub abnormal_logouts: u32,
}
// C attribute: ____cacheline_aligned.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
