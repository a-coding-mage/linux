/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology, Inc.
 *              2001, 2002 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */
/* Dependencies supplied by the corresponding Linux networking headers. */

pub const LLC_EVENT: ::core::ffi::c_int = 1;
pub const LLC_PACKET: ::core::ffi::c_int = 2;

pub const LLC2_P_TIME: ::core::ffi::c_int = 2;
pub const LLC2_ACK_TIME: ::core::ffi::c_int = 1;
pub const LLC2_REJ_TIME: ::core::ffi::c_int = 3;
pub const LLC2_BUSY_TIME: ::core::ffi::c_int = 3;

#[repr(C)]
pub struct llc_timer {
    pub timer: timer_list,
    pub expire: ::core::ffi::c_ulong, /* timer expire time */
}

#[repr(C)]
pub struct llc_sock {
    /* struct sock must be the first member of llc_sock */
    pub sk: sock,
    pub addr: sockaddr_llc, /* address sock is bound to */
    pub state: u8, /* state of connection */
    pub sap: *mut llc_sap, /* pointer to parent SAP */
    pub laddr: llc_addr, /* lsap/mac pair */
    pub daddr: llc_addr, /* dsap/mac pair */
    pub dev: *mut net_device, /* device to send to remote */
    pub dev_tracker: netdevice_tracker,
    pub copied_seq: u32, /* head of yet unread data */
    pub retry_count: u8, /* number of retries */
    pub ack_must_be_send: u8,
    pub first_pdu_Ns: u8,
    pub npta: u8,
    pub ack_timer: llc_timer,
    pub pf_cycle_timer: llc_timer,
    pub rej_sent_timer: llc_timer,
    pub busy_state_timer: llc_timer, /* ind busy clr at remote LLC */
    pub vS: u8, /* seq# next in-seq I-PDU tx'd */
    pub vR: u8, /* seq# next in-seq I-PDU rx'd */
    pub n2: u32, /* max nbr re-tx's for timeout */
    pub n1: u32, /* max nbr octets in I PDU */
    pub k: u8, /* tx window size; max = 127 */
    pub rw: u8, /* rx window size; max = 127 */
    pub p_flag: u8, /* state flags */
    pub f_flag: u8,
    pub s_flag: u8,
    pub data_flag: u8,
    pub remote_busy_flag: u8,
    pub cause_flag: u8,
    pub pdu_unack_q: sk_buff_head, /* PUDs sent/waiting ack */
    pub link: u16, /* network layer link number */
    pub X: u8, /* a temporary variable */
    pub ack_pf: u8, /* this flag indicates what is the P-bit of acknowledge */
    pub failed_data_req: u8, /* recognize that already exist a failed llc_data_req_handler (tx_buffer_full or unacceptable state */
    pub dec_step: u8,
    pub inc_cntr: u8,
    pub dec_cntr: u8,
    pub connect_step: u8,
    pub last_nr: u8, /* NR of last pdu received */
    pub rx_pdu_hdr: u32, /* used for saving header of last pdu received and caused sending FRMR. Used for resending FRMR */
    pub cmsg_flags: u32,
    pub dev_hash_node: hlist_node,
}

#[inline]
pub unsafe fn llc_sk(sk: *const sock) -> *mut llc_sock {
    sk as *mut llc_sock
}

#[inline]
pub unsafe fn llc_set_backlog_type(skb: *mut sk_buff, type_: i8) {
    (*skb).cb[core::mem::size_of_val(&(*skb).cb) - 1] = type_;
}

#[inline]
pub unsafe fn llc_backlog_type(skb: *mut sk_buff) -> i8 {
    (*skb).cb[core::mem::size_of_val(&(*skb).cb) - 1]
}

extern "C" {
    pub fn llc_sk_alloc(net: *mut net, family: ::core::ffi::c_int, priority: gfp_t,
                        prot: *mut proto, kern: ::core::ffi::c_int) -> *mut sock;
    pub fn llc_sk_stop_all_timers(sk: *mut sock, sync: bool);
    pub fn llc_sk_free(sk: *mut sock);
    pub fn llc_sk_reset(sk: *mut sock);

    /* Access to a connection */
    pub fn llc_conn_state_process(sk: *mut sock, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn llc_conn_send_pdu(sk: *mut sock, skb: *mut sk_buff);
    pub fn llc_conn_rtn_pdu(sk: *mut sock, skb: *mut sk_buff);
    pub fn llc_conn_resend_i_pdu_as_cmd(sk: *mut sock, nr: u8, first_p_bit: u8);
    pub fn llc_conn_resend_i_pdu_as_rsp(sk: *mut sock, nr: u8, first_f_bit: u8);
    pub fn llc_conn_remove_acked_pdus(sk: *mut sock, nr: u8, how_many_unacked: *mut u16) -> ::core::ffi::c_int;
    pub fn llc_lookup_established(sap: *mut llc_sap, daddr: *mut llc_addr,
                                  laddr: *mut llc_addr, net: *const net) -> *mut sock;
    pub fn llc_sap_add_socket(sap: *mut llc_sap, sk: *mut sock);
    pub fn llc_sap_remove_socket(sap: *mut llc_sap, sk: *mut sock);

    pub fn llc_data_accept_state(state: u8) -> u8;
    pub fn llc_build_offset_table();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
