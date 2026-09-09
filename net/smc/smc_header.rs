/* SPDX-License-Identifier: GPL-2.0 */
/* Shared Memory Communications over RDMA (SMC-R) and RoCE. */

/* Linux dependencies from smc.h are supplied by other translated units. */

pub const SMC_V1: u32 = 1;
pub const SMC_V2: u32 = 2;
pub const SMC_RELEASE_0: u32 = 0;
pub const SMC_RELEASE_1: u32 = 1;
pub const SMC_RELEASE: u32 = SMC_RELEASE_1;
pub const SMCPROTO_SMC: u32 = 0;
pub const SMCPROTO_SMC6: u32 = 1;
pub const SMC_AUTOCORKING_DEFAULT_SIZE: u32 = 0x10000;

extern "C" {
    pub static mut smc_proto: proto;
    pub static mut smc_proto6: proto;
    pub static mut smc_v4_hashinfo: smc_hashinfo;
    pub static mut smc_v6_hashinfo: smc_hashinfo;

    pub fn smc_hash_sk(sk: *mut sock) -> i32;
    pub fn smc_unhash_sk(sk: *mut sock);
    pub fn smc_release_cb(sk: *mut sock);
    pub fn smc_release(sock: *mut socket) -> i32;
    pub fn smc_bind(sock: *mut socket, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32;
    pub fn smc_connect(sock: *mut socket, addr: *mut sockaddr_unsized, alen: i32, flags: i32) -> i32;
    pub fn smc_accept(sock: *mut socket, new_sock: *mut socket, arg: *mut proto_accept_arg) -> i32;
    pub fn smc_getname(sock: *mut socket, addr: *mut sockaddr, peer: i32) -> i32;
    pub fn smc_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t;
    pub fn smc_ioctl(sock: *mut socket, cmd: u32, arg: c_ulong) -> i32;
    pub fn smc_listen(sock: *mut socket, backlog: i32) -> i32;
    pub fn smc_shutdown(sock: *mut socket, how: i32) -> i32;
    pub fn smc_setsockopt(sock: *mut socket, level: i32, optname: i32, optval: sockptr_t, optlen: u32) -> i32;
    pub fn smc_getsockopt(sock: *mut socket, level: i32, optname: i32, optval: *mut c_char, optlen: *mut i32) -> i32;
    pub fn smc_sendmsg(sock: *mut socket, msg: *mut msghdr, len: usize) -> isize;
    pub fn smc_recvmsg(sock: *mut socket, msg: *mut msghdr, len: usize, flags: i32) -> isize;
    pub fn smc_splice_read(sock: *mut socket, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: u32) -> isize;
    pub fn smc_sk_init(net: *mut net, sk: *mut sock, protocol: i32);
    pub fn smc_create_clcsk(net: *mut net, sk: *mut sock, family: i32) -> i32;
}

pub const SMC_ACTIVE: i32 = 1;
pub const SMC_INIT: i32 = 2;
pub const SMC_CLOSED: i32 = 7;
pub const SMC_LISTEN: i32 = 10;
pub const SMC_PEERCLOSEWAIT1: i32 = 20;
pub const SMC_PEERCLOSEWAIT2: i32 = 21;
pub const SMC_APPFINCLOSEWAIT: i32 = 24;
pub const SMC_APPCLOSEWAIT1: i32 = 22;
pub const SMC_APPCLOSEWAIT2: i32 = 23;
pub const SMC_PEERFINCLOSEWAIT: i32 = 25;
pub const SMC_PEERABORTWAIT: i32 = 26;
pub const SMC_PROCESSABORT: i32 = 27;
pub const SMC_SPF_EMULATED_ISM_DEV: u32 = 0;
pub const SMC_FEATURE_MASK: u32 = 1 << SMC_SPF_EMULATED_ISM_DEV;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_wr_rx_hdr { pub type_: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_cdc_conn_state_flags { pub bits: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_cdc_producer_flags { pub bits: u8 }

#[repr(C)]
pub union smc_host_cursor {
    pub fields: smc_host_cursor_fields,
    pub acurs: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_host_cursor_fields { pub reserved: u16, pub wrap: u16, pub count: u32 }

#[repr(C, align(8))]
pub struct smc_host_cdc_msg {
    pub common: smc_wr_rx_hdr, pub len: u8, pub seqno: u16, pub token: u32,
    pub prod: smc_host_cursor, pub cons: smc_host_cursor,
    pub prod_flags: smc_cdc_producer_flags, pub conn_state_flags: smc_cdc_conn_state_flags,
    pub reserved: [u8; 18],
}

pub const SMC_URG_VALID: i32 = 1;
pub const SMC_URG_NOTYET: i32 = 2;
pub const SMC_URG_READ: i32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum smc_state { SMC_ACTIVE_STATE = 1, SMC_INIT_STATE = 2, SMC_CLOSED_STATE = 7, SMC_LISTEN_STATE = 10,
    SMC_PEERCLOSEWAIT1_STATE = 20, SMC_PEERCLOSEWAIT2_STATE = 21, SMC_APPFINCLOSEWAIT_STATE = 24,
    SMC_APPCLOSEWAIT1_STATE = 22, SMC_APPCLOSEWAIT2_STATE = 23, SMC_PEERFINCLOSEWAIT_STATE = 25,
    SMC_PEERABORTWAIT_STATE = 26, SMC_PROCESSABORT_STATE = 27 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum smc_supplemental_features { SMC_SPF_EMULATED_ISM_DEV_ENUM = 0 }

#[repr(C)]
pub struct smc_mark_woken { pub woken: bool, pub key: *mut c_void, pub wait_entry: wait_queue_entry_t }

#[repr(C)]
pub struct smc_connection {
    pub alert_node: rb_node, pub lgr: *mut smc_link_group, pub lnk: *mut smc_link,
    pub alert_token_local: u32, pub peer_rmbe_idx: u8, pub peer_rmbe_size: i32,
    pub peer_rmbe_space: atomic_t, pub rtoken_idx: i32, pub sndbuf_desc: *mut smc_buf_desc,
    pub rmb_desc: *mut smc_buf_desc, pub rmbe_size_comp: i32, pub rmbe_update_limit: i32,
    pub local_tx_ctrl: smc_host_cdc_msg, pub local_tx_ctrl_fin: smc_host_cursor,
    pub tx_curs_prep: smc_host_cursor, pub tx_curs_sent: smc_host_cursor, pub tx_curs_fin: smc_host_cursor,
    pub sndbuf_space: atomic_t, pub tx_cdc_seq: u16, pub tx_cdc_seq_fin: u16, pub send_lock: spinlock_t,
    pub cdc_pend_tx_wr: atomic_t, pub cdc_pend_tx_wq: wait_queue_head_t, pub tx_work: delayed_work,
    pub tx_off: u32, pub local_rx_ctrl: smc_host_cdc_msg, pub rx_curs_confirmed: smc_host_cursor,
    pub urg_curs: smc_host_cursor, pub urg_state: i32, pub urg_tx_pend: bool, pub urg_rx_skip_pend: bool,
    pub urg_rx_byte: c_char, pub tx_in_release_sock: bool, pub bytes_to_rcv: atomic_t,
    pub splice_pending: atomic_t, pub close_work: work_struct, pub abort_work: work_struct,
    pub rx_tsklet: tasklet_struct, pub rx_off: u8, pub peer_token: u64, pub killed: u8,
    pub freed: u8, pub out_of_sync: u8,
}

#[repr(C)]
pub union smc_sock_union { pub sk: sock, pub icsk_inet: inet_sock }
#[repr(C)]
pub struct smc_sock {
    pub base: smc_sock_union, pub clcsock: *mut socket,
    pub clcsk_state_change: Option<unsafe extern "C" fn(*mut sock)>,
    pub clcsk_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    pub clcsk_write_space: Option<unsafe extern "C" fn(*mut sock)>,
    pub clcsk_error_report: Option<unsafe extern "C" fn(*mut sock)>,
    pub conn: smc_connection, pub listen_smc: *mut smc_sock, pub connect_work: work_struct,
    pub tcp_listen_work: work_struct, pub smc_listen_work: work_struct, pub accept_q: list_head,
    pub accept_q_lock: spinlock_t, pub limit_smc_hs: bool, pub use_fallback: bool,
    pub fallback_rsn: i32, pub peer_diagnosis: u32, pub queued_smc_hs: atomic_t,
    pub af_ops: inet_connection_sock_af_ops, pub ori_af_ops: *const inet_connection_sock_af_ops,
    pub sockopt_defer_accept: i32, pub wait_close_tx_prepared: u8, pub connect_nonblock: u8,
    pub clcsock_release_lock: mutex,
}

#[inline]
pub unsafe fn smc_init_saved_callbacks(smc: *mut smc_sock) {
    (*smc).clcsk_state_change = None; (*smc).clcsk_data_ready = None;
    (*smc).clcsk_write_space = None; (*smc).clcsk_error_report = None;
}
#[inline]
pub unsafe fn smc_clcsock_user_data(clcsk: *const sock) -> *mut smc_sock {
    ((*clcsk).sk_user_data as usize & !(SK_USER_DATA_NOCOPY as usize)) as *mut smc_sock
}
#[inline]
pub unsafe fn smc_clcsock_replace_cb(target_cb: *mut Option<unsafe extern "C" fn(*mut sock)>, new_cb: Option<unsafe extern "C" fn(*mut sock)>, saved_cb: *mut Option<unsafe extern "C" fn(*mut sock)>) {
    if (*saved_cb).is_none() { *saved_cb = *target_cb; } *target_cb = new_cb;
}
#[inline]
pub unsafe fn smc_clcsock_restore_cb(target_cb: *mut Option<unsafe extern "C" fn(*mut sock)>, saved_cb: *mut Option<unsafe extern "C" fn(*mut sock)>) {
    if (*saved_cb).is_none() { return; } *target_cb = *saved_cb; *saved_cb = None;
}

#[inline]
pub unsafe fn hton24(net: *mut u8, host: u32) {
    let t = host.to_be_bytes(); core::ptr::copy_nonoverlapping(t.as_ptr().add(1), net, 3);
}
#[inline]
pub unsafe fn ntoh24(net: *const u8) -> u32 {
    let mut t = [0u8; 4]; core::ptr::copy_nonoverlapping(net, t.as_mut_ptr().add(1), 3); u32::from_be_bytes(t)
}

extern "C" {
    pub static mut smc_hs_wq: *mut workqueue_struct;
    pub static mut smc_close_wq: *mut workqueue_struct;
    pub static mut local_systemid: [u8; 8];
    pub fn smc_accept_dequeue(parent: *mut sock, new_sock: *mut socket) -> *mut sock;
    pub fn smc_close_non_accepted(sk: *mut sock);
    pub fn smc_fill_gid_list(lgr: *mut smc_link_group, gidlist: *mut smc_gidlist, known_dev: *mut smc_ib_device, known_gid: *mut u8);
    pub fn smc_nl_dump_hs_limitation(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn smc_nl_enable_hs_limitation(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn smc_nl_disable_hs_limitation(skb: *mut sk_buff, info: *mut genl_info) -> i32;
}

pub const SMC_SYSTEMID_LEN: usize = 8;

#[inline]
pub unsafe fn smc_sock_set_flag(sk: *mut sock, flag: sock_flags) { set_bit(flag, &mut (*sk).sk_flags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
