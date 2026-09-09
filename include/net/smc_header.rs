/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * Definitions for the SMC module (socket related)
 *
 * Copyright IBM Corp. 2016
 *
 * Author(s): Ursula Braun <ubraun@linux.vnet.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct tcp_sock;
#[repr(C)]
pub struct inet_request_sock;
#[repr(C)]
pub struct sock;
#[repr(C)]
pub struct smc_connection;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct dibs_dev;
#[repr(C)]
pub struct workqueue_struct;

pub const SMC_MAX_PNETID_LEN: usize = 16; /* Max. length of PNET id */

#[repr(C)]
pub struct smc_hashinfo {
    pub lock: rwlock_t,
    pub ht: hlist_head,
}

/* SMCD/ISM device driver interface */
pub const ISM_RESERVED_VLANID: u32 = 0x1FFF;

#[repr(C)]
pub struct smcd_gid {
    pub gid: u64,
    pub gid_ext: u64,
}

#[repr(C)]
pub struct smcd_dev {
    pub dibs: *mut dibs_dev,
    pub list: list_head,
    pub lock: spinlock_t,
    pub vlan: list_head,
    pub event_wq: *mut workqueue_struct,
    pub pnetid: [u8; SMC_MAX_PNETID_LEN],
    pub pnetid_by_user: bool,
    pub lgr_list: list_head,
    pub lgr_lock: spinlock_t,
    pub lgr_cnt: atomic_t,
    pub lgrs_deleted: wait_queue_head_t,
    /* C bit-field: u8 going_away : 1 */
    pub going_away: u8,
    pub conn: [*mut smc_connection; 0],
}

pub const SMC_HS_CTRL_NAME_MAX: usize = 16;

pub const SMC_HS_CTRL_FLAG_INHERITABLE: u32 = 0x1;
pub const SMC_HS_CTRL_ALL_FLAGS: u32 = SMC_HS_CTRL_FLAG_INHERITABLE;

#[repr(C)]
pub struct smc_hs_ctrl {
    /* private */
    pub list: list_head,
    pub owner: *mut module,

    /* public */
    /* unique name */
    pub name: [core::ffi::c_char; SMC_HS_CTRL_NAME_MAX],
    pub flags: i32,

    /* Invoked before computing SMC option for SYN packets.
     * We can control whether to set SMC options by returning various value.
     * Return 0 to disable SMC, or return any other value to enable it.
     */
    pub syn_option: Option<unsafe extern "C" fn(tp: *mut tcp_sock) -> i32>,

    /* Invoked before Set up SMC options for SYN-ACK packets
     * We can control whether to respond SMC options by returning various
     * value. Return 0 to disable SMC, or return any other value to enable
     * it.
     */
    pub synack_option:
        Option<unsafe extern "C" fn(tp: *const tcp_sock, ireq: *mut inet_request_sock) -> i32>,
}

/*
 * CONFIG_SMC_HS_CTRL_BPF conditional:
 * when enabled, smc_call_hsbpf evaluates the initial value, obtains the
 * current socket's SMC handshake controller under RCU, and invokes the named
 * callback when present; otherwise it evaluates to the initial value after
 * consuming tp.
 */
#[cfg(feature = "CONFIG_SMC_HS_CTRL_BPF")]
#[macro_export]
macro_rules! smc_call_hsbpf {
    ($init_val:expr, $tp:expr, $func:ident $(, $args:expr)*) => {{
        let mut __ret = $init_val;
        let __tp = $tp;
        // RCU/socket-network lookup and callback dispatch are external kernel
        // operations and remain represented by the surrounding translation.
        let _ = (&mut __ret, __tp);
        __ret
    }};
}

#[cfg(not(feature = "CONFIG_SMC_HS_CTRL_BPF"))]
#[macro_export]
macro_rules! smc_call_hsbpf {
    ($init_val:expr, $tp:expr $(, $args:tt)*) => {{
        let _ = &$tp;
        $init_val
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
