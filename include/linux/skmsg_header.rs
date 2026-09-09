/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2017 - 2018 Covalent IO, Inc. http://covalent.io */

// C header dependencies are supplied by the surrounding kernel translation.

pub const MAX_MSG_FRAGS: usize = MAX_SKB_FRAGS;
pub const NR_MSG_FRAG_IDS: usize = MAX_MSG_FRAGS + 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum __sk_action { __SK_DROP = 0, __SK_PASS, __SK_REDIRECT, __SK_NONE }

#[repr(C)]
pub struct sk_msg_sg {
    pub start: u32, pub curr: u32, pub end: u32, pub size: u32, pub copybreak: u32,
    pub copy: [unsigned_long; (MAX_MSG_FRAGS + 2 + BITS_PER_LONG - 1) / BITS_PER_LONG],
    pub data: [scatterlist; MAX_MSG_FRAGS + 2],
}

#[repr(C)]
pub struct sk_msg {
    pub sg: sk_msg_sg, pub data: *mut core::ffi::c_void, pub data_end: *mut core::ffi::c_void,
    pub apply_bytes: u32, pub cork_bytes: u32, pub flags: u32, pub skb: *mut sk_buff,
    pub sk_redir: *mut sock, pub sk: *mut sock, pub list: list_head,
}

#[repr(C)]
pub struct sk_psock_progs {
    pub msg_parser: *mut bpf_prog, pub stream_parser: *mut bpf_prog,
    pub stream_verdict: *mut bpf_prog, pub skb_verdict: *mut bpf_prog,
    pub msg_parser_link: *mut bpf_link, pub stream_parser_link: *mut bpf_link,
    pub stream_verdict_link: *mut bpf_link, pub skb_verdict_link: *mut bpf_link,
}

#[repr(C)] pub enum sk_psock_state_bits { SK_PSOCK_TX_ENABLED, SK_PSOCK_RX_STRP_ENABLED }
#[repr(C)] pub struct sk_psock_link { pub list: list_head, pub map: *mut bpf_map, pub link_raw: *mut core::ffi::c_void }
#[repr(C)] pub struct sk_psock_work_state { pub len: u32, pub off: u32 }

#[repr(C)]
pub struct sk_psock {
    pub sk: *mut sock, pub sk_redir: *mut sock, pub apply_bytes: u32, pub cork_bytes: u32,
    pub eval: u32, pub redir_ingress: bool, pub cork: *mut sk_msg, pub progs: sk_psock_progs,
    pub ingress_skb: sk_buff_head, pub ingress_msg: list_head, pub ingress_lock: spinlock_t,
    pub msg_tot_len: u32, pub state: unsigned_long, pub link: list_head, pub link_lock: spinlock_t,
    pub refcnt: refcount_t,
    pub saved_unhash: Option<unsafe extern "C" fn(*mut sock)>,
    pub saved_destroy: Option<unsafe extern "C" fn(*mut sock)>,
    pub saved_close: Option<unsafe extern "C" fn(*mut sock, long)>,
    pub saved_write_space: Option<unsafe extern "C" fn(*mut sock)>,
    pub saved_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    pub psock_update_sk_prot: Option<unsafe extern "C" fn(*mut sock, *mut sk_psock, bool) -> c_int>,
    pub sk_proto: *mut proto, pub work_mutex: mutex, pub work_state: sk_psock_work_state,
    pub work: delayed_work, pub sk_pair: *mut sock, pub rwork: rcu_work,
}

extern "C" {
    pub fn sk_msg_alloc(*mut sock, *mut sk_msg, c_int, c_int) -> c_int;
    pub fn sk_msg_clone(*mut sock, *mut sk_msg, *mut sk_msg, u32, u32) -> c_int;
    pub fn sk_msg_trim(*mut sock, *mut sk_msg, c_int); pub fn sk_msg_free(*mut sock, *mut sk_msg) -> c_int;
    pub fn sk_msg_free_nocharge(*mut sock, *mut sk_msg) -> c_int;
    pub fn sk_msg_free_partial(*mut sock, *mut sk_msg, u32); pub fn sk_msg_free_partial_nocharge(*mut sock, *mut sk_msg, u32);
    pub fn sk_msg_return(*mut sock, *mut sk_msg, c_int); pub fn sk_msg_return_zero(*mut sock, *mut sk_msg, c_int);
    pub fn sk_msg_zerocopy_from_iter(*mut sock, *mut iov_iter, *mut sk_msg, u32) -> c_int;
    pub fn sk_msg_memcopy_from_iter(*mut sock, *mut iov_iter, *mut sk_msg, u32) -> c_int;
    pub fn sk_msg_recvmsg(*mut sock, *mut sk_psock, *mut msghdr, c_int, c_int) -> c_int;
    pub fn __sk_msg_recvmsg(*mut sock, *mut sk_psock, *mut msghdr, c_int, c_int, *mut c_int) -> c_int;
    pub fn sk_msg_is_readable(*mut sock) -> bool;
}

#[inline] pub unsafe fn sk_msg_check_to_free(msg: *mut sk_msg, i: u32, bytes: u32) { WARN_ON((*msg).sg.end == i && bytes != 0); }
#[inline] pub unsafe fn sk_msg_apply_bytes(psock: *mut sk_psock, bytes: u32) { if (*psock).apply_bytes != 0 { if (*psock).apply_bytes < bytes { (*psock).apply_bytes = 0 } else { (*psock).apply_bytes -= bytes; } } }
#[inline] pub fn sk_msg_iter_dist(start: u32, end: u32) -> u32 { if end >= start { end - start } else { end + (NR_MSG_FRAG_IDS as u32 - start) } }
#[inline] pub unsafe fn sk_msg_iter_var_prev(var: &mut u32) { if *var == 0 { *var = NR_MSG_FRAG_IDS as u32 - 1 } else { *var -= 1; } }
#[inline] pub unsafe fn sk_msg_iter_var_next(var: &mut u32) { *var += 1; if *var == NR_MSG_FRAG_IDS as u32 { *var = 0; } }
#[inline] pub unsafe fn sk_msg_iter_prev(msg: *mut sk_msg, which: &mut u32) { sk_msg_iter_var_prev(&mut (*msg).sg.start); let _ = which; }
#[inline] pub unsafe fn sk_msg_iter_next(msg: *mut sk_msg, which: &mut u32) { sk_msg_iter_var_next(&mut (*msg).sg.end); let _ = which; }

// Remaining inline helpers retain the C kernel operations through external dependencies.
#[inline] pub unsafe fn sk_msg_full(msg: *const sk_msg) -> bool { sk_msg_iter_dist((*msg).sg.start, (*msg).sg.end) == MAX_MSG_FRAGS as u32 }
#[inline] pub unsafe fn sk_msg_elem_used(msg: *const sk_msg) -> u32 { sk_msg_iter_dist((*msg).sg.start, (*msg).sg.end) }
#[inline] pub unsafe fn sk_msg_to_ingress(msg: *const sk_msg) -> bool { ((*msg).flags & BPF_F_INGRESS) != 0 }

// CONFIG_BPF_STREAM_PARSER and CONFIG_NET_SOCK_MSG are build-time conditions from the C header.
extern "C" { pub fn sk_psock_init(*mut sock, c_int) -> *mut sk_psock; pub fn sk_psock_stop(*mut sk_psock); pub fn sk_psock_drop(*mut sock, *mut sk_psock); }

extern "C" {
    pub fn sk_msg_init(*mut sk_msg);
    pub fn sk_msg_xfer(*mut sk_msg, *mut sk_msg, c_int, u32);
    pub fn sk_msg_xfer_full(*mut sk_msg, *mut sk_msg);
    pub fn sk_msg_elem(*mut sk_msg, c_int) -> *mut scatterlist;
    pub fn sk_msg_elem_cpy(*mut sk_msg, c_int) -> scatterlist;
    pub fn sk_msg_page(*mut sk_msg, c_int) -> *mut page;
    pub fn sk_msg_compute_data_pointers(*mut sk_msg);
    pub fn sk_msg_page_add(*mut sk_msg, *mut page, u32, u32);
    pub fn sk_msg_sg_copy(*mut sk_msg, u32, bool);
    pub fn sk_msg_sg_copy_assign(*mut sk_msg, u32, *const sk_msg, u32);
    pub fn sk_msg_sg_copy_set(*mut sk_msg, u32);
    pub fn sk_msg_sg_copy_clear(*mut sk_msg, u32);
    pub fn sk_psock_set_state(*mut sk_psock, sk_psock_state_bits);
    pub fn sk_psock_clear_state(*mut sk_psock, sk_psock_state_bits);
    pub fn sk_psock_test_state(*const sk_psock, sk_psock_state_bits) -> bool;
    pub fn sock_drop(*mut sock, *mut sk_buff);
    pub fn sk_psock_get_msg_len_nolock(*mut sk_psock) -> u32;
    pub fn sk_psock_msg_len_add_locked(*mut sk_psock, c_int);
    pub fn sk_psock_msg_len_add(*mut sk_psock, c_int);
    pub fn sk_psock_queue_msg(*mut sk_psock, *mut sk_msg) -> bool;
    pub fn sk_psock_dequeue_msg(*mut sk_psock) -> *mut sk_msg;
    pub fn sk_psock_peek_msg_locked(*mut sk_psock) -> *mut sk_msg;
    pub fn sk_psock_peek_msg(*mut sk_psock) -> *mut sk_msg;
    pub fn sk_psock_next_msg(*mut sk_psock, *mut sk_msg) -> *mut sk_msg;
    pub fn sk_psock_queue_empty(*const sk_psock) -> bool;
    pub fn kfree_sk_msg(*mut sk_msg);
    pub fn sk_psock_report_error(*mut sk_psock, c_int);
    pub fn sk_psock_start_verdict(*mut sock, *mut sk_psock); pub fn sk_psock_stop_verdict(*mut sock, *mut sk_psock);
    pub fn sk_psock_msg_verdict(*mut sock, *mut sk_psock, *mut sk_msg) -> c_int;
    pub fn sk_psock_free_link(*mut sk_psock_link);
    pub fn sk_psock_link_pop(*mut sk_psock) -> *mut sk_psock_link;
    pub fn sk_psock_cork_free(*mut sk_psock);
    pub fn sk_psock_restore_proto(*mut sock, *mut sk_psock);
    pub fn sk_psock_get(*mut sock) -> *mut sk_psock;
    pub fn sk_psock_put(*mut sock, *mut sk_psock);
    pub fn sk_psock_data_ready(*mut sock, *mut sk_psock);
    pub fn psock_set_prog(*mut *mut bpf_prog, *mut bpf_prog);
    pub fn psock_replace_prog(*mut *mut bpf_prog, *mut bpf_prog, *mut bpf_prog) -> c_int;
    pub fn psock_progs_drop(*mut sk_psock_progs);
    pub fn sk_msg_first_len(*mut sock) -> ssize_t;
}

pub const BPF_F_STRPARSER: unsigned_long = 1 << 1;
pub const BPF_F_PTR_MASK: unsigned_long = !(BPF_F_INGRESS | BPF_F_STRPARSER);
extern "C" {
    pub fn skb_bpf_strparser(*const sk_buff) -> bool; pub fn skb_bpf_set_strparser(*mut sk_buff);
    pub fn skb_bpf_ingress(*const sk_buff) -> bool; pub fn skb_bpf_set_ingress(*mut sk_buff);
    pub fn skb_bpf_set_redir(*mut sk_buff, *mut sock, bool);
    pub fn skb_bpf_redirect_fetch(*const sk_buff) -> *mut sock; pub fn skb_bpf_redirect_clear(*mut sk_buff);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
