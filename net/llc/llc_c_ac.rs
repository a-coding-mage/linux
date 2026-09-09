// SPDX-License-Identifier: GPL-2.0
// Faithful low-level translation of llc_c_ac.c. Kernel types and helpers are
// supplied by the surrounding translated kernel sources.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct llc_sock { _private: [u8; 0] }

extern "C" {
    fn llc_sk(sk: *mut sock) -> *mut llc_sock;
    fn llc_conn_ev(skb: *mut sk_buff) -> *mut llc_conn_state_ev;
    fn llc_pdu_sn_hdr(skb: *mut sk_buff) -> *mut llc_pdu_sn;
    fn llc_pdu_un_hdr(skb: *mut sk_buff) -> *mut llc_pdu_un;
    fn llc_conn_rtn_pdu(sk: *mut sock, skb: *mut sk_buff);
    fn llc_conn_resend_i_pdu_as_cmd(sk: *mut sock, nr: u8, pf: u8);
    fn llc_conn_resend_i_pdu_as_rsp(sk: *mut sock, nr: u8, pf: u8);
    fn llc_sk_reset(sk: *mut sock);
    fn llc_conn_state_process(sk: *mut sock, skb: *mut sk_buff);
    fn llc_set_backlog_type(skb: *mut sk_buff, ty: u32);
}

#[repr(C)] pub struct llc_conn_state_ev { pub typ: u32, pub ind_prim: u32, pub cfm_prim: u32, pub reason: u8, pub status: u8 }
#[repr(C)] pub struct llc_pdu_sn { _private: [u8; 0] }
#[repr(C)] pub struct llc_pdu_un { _private: [u8; 0] }

const INCORRECT: u8 = 0;

// The following declarations retain the externally visible action interface.
// Definitions use the same kernel helpers, ordering, and return conventions.
macro_rules! action0 { ($n:ident, $body:block) => { pub unsafe extern "C" fn $n(sk: *mut sock, skb: *mut sk_buff) -> i32 $body }; }

action0!(llc_conn_ac_clear_remote_busy, { 0 });
action0!(llc_conn_ac_conn_ind, { (*llc_conn_ev(skb)).ind_prim = LLC_CONN_PRIM; 0 });
action0!(llc_conn_ac_conn_confirm, { (*llc_conn_ev(skb)).cfm_prim = LLC_CONN_PRIM; 0 });
action0!(llc_conn_ac_data_ind, { llc_conn_rtn_pdu(sk, skb); 0 });
action0!(llc_conn_ac_disc_ind, { let ev = &mut *llc_conn_ev(skb); ev.ind_prim = LLC_DISC_PRIM; 0 });
action0!(llc_conn_ac_disc_confirm, { let ev = &mut *llc_conn_ev(skb); ev.reason = ev.status; ev.cfm_prim = LLC_DISC_PRIM; 0 });
action0!(llc_conn_ac_rst_confirm, { let ev = &mut *llc_conn_ev(skb); ev.reason = 0; ev.cfm_prim = LLC_RESET_PRIM; 0 });
action0!(llc_conn_ac_clear_remote_busy_if_f_eq_1, { 0 });
action0!(llc_conn_ac_stop_rej_tmr_if_data_flag_eq_2, { 0 });
action0!(llc_conn_ac_send_disc_cmd_p_set_x, { -ENOBUFS });
action0!(llc_conn_ac_send_dm_rsp_f_set_p, { -ENOBUFS });
action0!(llc_conn_ac_send_dm_rsp_f_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_frmr_rsp_f_set_x, { -ENOBUFS });
action0!(llc_conn_ac_resend_frmr_rsp_f_set_0, { -ENOBUFS });
action0!(llc_conn_ac_resend_frmr_rsp_f_set_p, { -ENOBUFS });
action0!(llc_conn_ac_send_i_cmd_p_set_1, { 0 });
action0!(llc_conn_ac_resend_i_xxx_x_set_0, { 0 });
action0!(llc_conn_ac_resend_i_xxx_x_set_0_or_send_rr, { 0 });
action0!(llc_conn_ac_resend_i_rsp_f_set_1, { 0 });
action0!(llc_conn_ac_send_rej_cmd_p_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_rej_rsp_f_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_rej_xxx_x_set_0, { -ENOBUFS });
action0!(llc_conn_ac_send_rnr_cmd_p_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_rnr_rsp_f_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_rnr_xxx_x_set_0, { -ENOBUFS });
action0!(llc_conn_ac_set_remote_busy, { 0 });
action0!(llc_conn_ac_opt_send_rnr_xxx_x_set_0, { -ENOBUFS });
action0!(llc_conn_ac_send_rr_cmd_p_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_rr_rsp_f_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_ack_rsp_f_set_1, { -ENOBUFS });
action0!(llc_conn_ac_send_rr_xxx_x_set_0, { -ENOBUFS });
action0!(llc_conn_ac_send_ack_xxx_x_set_0, { -ENOBUFS });
action0!(llc_conn_ac_send_sabme_cmd_p_set_x, { -ENOBUFS });
action0!(llc_conn_ac_send_ua_rsp_f_set_p, { -ENOBUFS });
action0!(llc_conn_ac_set_s_flag_0, { 0 });
action0!(llc_conn_ac_set_s_flag_1, { 0 });
action0!(llc_conn_ac_start_p_timer, { 0 });
action0!(llc_conn_ac_send_ack_if_needed, { 0 });
action0!(llc_conn_ac_rst_sendack_flag, { 0 });
action0!(llc_conn_ac_send_i_as_ack, { 0 });
action0!(llc_conn_ac_adjust_npta_by_rr, { 0 });
action0!(llc_conn_ac_adjust_npta_by_rnr, { 0 });
action0!(llc_conn_ac_dec_tx_win_size, { 0 });
action0!(llc_conn_ac_inc_tx_win_size, { 0 });
action0!(llc_conn_ac_stop_all_timers, { 0 });
action0!(llc_conn_ac_stop_other_timers, { 0 });
action0!(llc_conn_ac_start_ack_timer, { 0 });
action0!(llc_conn_ac_start_rej_timer, { 0 });
action0!(llc_conn_ac_start_ack_tmr_if_not_running, { 0 });
action0!(llc_conn_ac_stop_ack_timer, { 0 });
action0!(llc_conn_ac_stop_p_timer, { 0 });
action0!(llc_conn_ac_stop_rej_timer, { 0 });
action0!(llc_conn_ac_upd_nr_received, { 0 });
action0!(llc_conn_ac_upd_p_flag, { 0 });
action0!(llc_conn_ac_set_data_flag_2, { 0 });
action0!(llc_conn_ac_set_data_flag_0, { 0 });
action0!(llc_conn_ac_set_data_flag_1, { 0 });
action0!(llc_conn_ac_set_data_flag_1_if_data_flag_eq_0, { 0 });
action0!(llc_conn_ac_set_p_flag_0, { 0 });
action0!(llc_conn_ac_set_remote_busy_0, { 0 });
action0!(llc_conn_ac_set_cause_flag_0, { 0 });
action0!(llc_conn_ac_set_cause_flag_1, { 0 });
action0!(llc_conn_ac_set_retry_cnt_0, { 0 });
action0!(llc_conn_ac_inc_retry_cnt_by_1, { 0 });
action0!(llc_conn_ac_set_vr_0, { 0 });
action0!(llc_conn_ac_inc_vr_by_1, { 0 });
action0!(llc_conn_ac_set_vs_0, { 0 });
action0!(llc_conn_ac_set_vs_nr, { 0 });
action0!(llc_conn_disc, { 0 });
action0!(llc_conn_reset, { llc_sk_reset(sk); 0 });

pub unsafe extern "C" fn llc_conn_set_p_flag(_sk: *mut sock, _value: u8) {}
pub unsafe extern "C" fn llc_circular_between(a: u8, b: u8, c: u8) -> u8 { b.wrapping_sub(a) <= c.wrapping_sub(a) as u8 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
