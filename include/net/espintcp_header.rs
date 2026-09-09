/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// net/strparser.h and linux/skmsg.h.

extern "C" {
    pub fn espintcp_init(); // __init

    pub fn espintcp_push_skb(sk: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn espintcp_queue_out(sk: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn tcp_is_ulp_esp(sk: *mut sock) -> bool;
}

#[repr(C)]
pub struct espintcp_msg {
    pub skb: *mut sk_buff,
    pub skmsg: sk_msg,
    pub offset: ::core::ffi::c_int,
    pub len: ::core::ffi::c_int,
}

#[repr(C)]
pub struct espintcp_ctx {
    pub strp: strparser,
    pub ike_queue: sk_buff_head,
    pub out_queue: sk_buff_head,
    pub partial: espintcp_msg,
    pub saved_data_ready: Option<unsafe extern "C" fn(sk: *mut sock)>,
    pub saved_write_space: Option<unsafe extern "C" fn(sk: *mut sock)>,
    pub saved_destruct: Option<unsafe extern "C" fn(sk: *mut sock)>,
    pub work: work_struct,
    pub tx_running: bool,
}

#[inline]
pub unsafe fn espintcp_getctx(sk: *const sock) -> *mut espintcp_ctx {
    let icsk: *const inet_connection_sock = inet_csk(sk);

    /* RCU is only needed for diag */
    (*icsk).icsk_ulp_data as *mut espintcp_ctx
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
