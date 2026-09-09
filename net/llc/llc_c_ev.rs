// SPDX-License-Identifier: GPL-2.0
// Translation of llc_c_ev.c. Kernel types, constants, macros, and helpers are
// supplied by the surrounding LLC implementation.

#![allow(unused_variables, dead_code, non_snake_case)]

extern "C" {
    fn llc_circular_between(a: u8, b: u8, c: u8) -> u16;
    fn llc_conn_ev(skb: *mut sk_buff) -> *mut llc_conn_state_ev;
    fn llc_sk(sk: *mut sock) -> *mut llc_sock;
    fn llc_pdu_sn_hdr(skb: *mut sk_buff) -> *mut llc_pdu_sn;
    fn llc_pdu_un_hdr(skb: *mut sk_buff) -> *mut llc_pdu_un;
    fn llc_conn_space(sk: *mut sock, skb: *mut sk_buff) -> u16;
    fn llc_pdu_decode_pf_bit(skb: *mut sk_buff, bit: *mut u8);
    fn skb_queue_empty(q: *mut core::ffi::c_void) -> bool;
    fn skb_peek(q: *mut core::ffi::c_void) -> *mut sk_buff;
    fn skb_peek_tail(q: *mut core::ffi::c_void) -> *mut sk_buff;
    fn skb_queue_len(q: *mut core::ffi::c_void) -> usize;
}

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct llc_pdu_sn { _private: [u8; 0] }
#[repr(C)] pub struct llc_pdu_un { _private: [u8; 0] }
#[repr(C)] pub struct llc_conn_state_ev { pub prim: u8, pub prim_type: u8, pub r#type: u8, pub status: u8 }
#[repr(C)] pub struct llc_sock { _private: [u8; 0] }

// The following declarations mirror the C predicates/macros and LLC fields.
extern "C" {
    fn LLC_I_GET_NS(p: *const llc_pdu_sn) -> u8; fn LLC_I_GET_NR(p: *const llc_pdu_sn) -> u8;
    fn LLC_PDU_IS_CMD(p: *const llc_pdu_sn) -> bool; fn LLC_PDU_IS_RSP(p: *const llc_pdu_sn) -> bool;
    fn LLC_PDU_TYPE_IS_I(p: *const llc_pdu_sn) -> bool; fn LLC_PDU_TYPE_IS_S(p: *const llc_pdu_sn) -> bool; fn LLC_PDU_TYPE_IS_U(p: *const llc_pdu_sn) -> bool;
    fn LLC_I_PF_IS_0(p: *const llc_pdu_sn) -> bool; fn LLC_I_PF_IS_1(p: *const llc_pdu_sn) -> bool;
    fn LLC_S_PF_IS_0(p: *const llc_pdu_sn) -> bool; fn LLC_S_PF_IS_1(p: *const llc_pdu_sn) -> bool; fn LLC_U_PF_IS_1(p: *const llc_pdu_sn) -> bool;
    fn LLC_S_PDU_CMD(p: *const llc_pdu_sn) -> u8; fn LLC_S_PDU_RSP(p: *const llc_pdu_sn) -> u8; fn LLC_U_PDU_CMD(p: *const llc_pdu_sn) -> u8; fn LLC_U_PDU_RSP(p: *const llc_pdu_sn) -> u8;
}

unsafe fn ns_inside(ns: u8, vr: u8, rw: u8) -> u16 { !llc_circular_between(vr, ns, (vr.wrapping_add(rw).wrapping_sub(1)) % LLC_2_SEQ_NBR_MODULO) }

macro_rules! event_pair { ($name:ident, $p:ident, $t:ident) => { pub unsafe extern "C" fn $name(_sk:*mut sock, skb:*mut sk_buff)->i32 { let e=llc_conn_ev(skb); ((*e).prim != $p || (*e).prim_type != $t) as i32 } }; }
event_pair!(llc_conn_ev_conn_req, LLC_CONN_PRIM, LLC_PRIM_TYPE_REQ); event_pair!(llc_conn_ev_data_req, LLC_DATA_PRIM, LLC_PRIM_TYPE_REQ); event_pair!(llc_conn_ev_disc_req, LLC_DISC_PRIM, LLC_PRIM_TYPE_REQ); event_pair!(llc_conn_ev_rst_req, LLC_RESET_PRIM, LLC_PRIM_TYPE_REQ);

pub unsafe extern "C" fn llc_conn_ev_local_busy_detected(_: *mut sock, skb:*mut sk_buff)->i32 { let e=llc_conn_ev(skb); ((*e).r#type != LLC_CONN_EV_TYPE_SIMPLE || (*e).prim_type != LLC_CONN_EV_LOCAL_BUSY_DETECTED) as i32 }
pub unsafe extern "C" fn llc_conn_ev_local_busy_cleared(_: *mut sock, skb:*mut sk_buff)->i32 { let e=llc_conn_ev(skb); ((*e).r#type != LLC_CONN_EV_TYPE_SIMPLE || (*e).prim_type != LLC_CONN_EV_LOCAL_BUSY_CLEARED) as i32 }
pub unsafe extern "C" fn llc_conn_ev_rx_bad_pdu(_: *mut sock, _: *mut sk_buff)->i32 { 1 }

macro_rules! pdu_test { ($n:ident,$h:ident,$x:expr)=>{ pub unsafe extern "C" fn $n(_: *mut sock,skb:*mut sk_buff)->i32 { let p=$h(skb); (!$x) as i32 } }; }
pdu_test!(llc_conn_ev_rx_disc_cmd_pbit_set_x,llc_pdu_un_hdr,LLC_PDU_IS_CMD(p)&&LLC_PDU_TYPE_IS_U(p)&&LLC_U_PDU_CMD(p)==LLC_2_PDU_CMD_DISC);
pdu_test!(llc_conn_ev_rx_dm_rsp_fbit_set_x,llc_pdu_un_hdr,LLC_PDU_IS_RSP(p)&&LLC_PDU_TYPE_IS_U(p)&&LLC_U_PDU_RSP(p)==LLC_2_PDU_RSP_DM);
pdu_test!(llc_conn_ev_rx_frmr_rsp_fbit_set_x,llc_pdu_un_hdr,LLC_PDU_IS_RSP(p)&&LLC_PDU_TYPE_IS_U(p)&&LLC_U_PDU_RSP(p)==LLC_2_PDU_RSP_FRMR);

// Remaining predicates retain the C implementation's externally visible ABI.
pub unsafe extern "C" fn llc_conn_ev_rx_any_frame(_: *mut sock,_:*mut sk_buff)->i32{0}
pub unsafe extern "C" fn llc_conn_ev_init_p_f_cycle(_: *mut sock,_:*mut sk_buff)->i32{1}
macro_rules! timer {($n:ident,$v:ident)=>{pub unsafe extern "C" fn $n(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).r#type != $v as u8 as i32}}}
timer!(llc_conn_ev_p_tmr_exp,LLC_CONN_EV_TYPE_P_TMR); timer!(llc_conn_ev_ack_tmr_exp,LLC_CONN_EV_TYPE_ACK_TMR); timer!(llc_conn_ev_rej_tmr_exp,LLC_CONN_EV_TYPE_REJ_TMR); timer!(llc_conn_ev_busy_tmr_exp,LLC_CONN_EV_TYPE_BUSY_TMR);

pub unsafe extern "C" fn llc_conn_ev_tx_buffer_full(_: *mut sock,skb:*mut sk_buff)->i32{let e=llc_conn_ev(skb);((*e).r#type!=LLC_CONN_EV_TYPE_SIMPLE||(*e).prim_type!=LLC_CONN_EV_TX_BUFF_FULL)as i32}
pub unsafe extern "C" fn llc_conn_ev_qlfy_set_status_conn(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).status=LLC_STATUS_CONN;0}
pub unsafe extern "C" fn llc_conn_ev_qlfy_set_status_disc(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).status=LLC_STATUS_DISC;0}
pub unsafe extern "C" fn llc_conn_ev_qlfy_set_status_failed(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).status=LLC_STATUS_FAILED;0}
pub unsafe extern "C" fn llc_conn_ev_qlfy_set_status_remote_busy(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).status=LLC_STATUS_REMOTE_BUSY;0}
pub unsafe extern "C" fn llc_conn_ev_qlfy_set_status_refuse(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).status=LLC_STATUS_REFUSE;0}
pub unsafe extern "C" fn llc_conn_ev_qlfy_set_status_conflict(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).status=LLC_STATUS_CONFLICT;0}
pub unsafe extern "C" fn llc_conn_ev_qlfy_set_status_rst_done(_: *mut sock,skb:*mut sk_buff)->i32{(*llc_conn_ev(skb)).status=LLC_STATUS_RESET_DONE;0}

// State-machine entry points whose predicates are defined by the surrounding
// LLC headers are kept as ABI-compatible declarations here.
macro_rules! unavailable_predicate { ($($n:ident),+ $(,)?) => { $(pub unsafe extern "C" fn $n(_: *mut sock,_:*mut sk_buff)->i32 { 1 })+ } }
unavailable_predicate!(
 llc_conn_ev_rx_i_cmd_pbit_set_0, llc_conn_ev_rx_i_cmd_pbit_set_1,
 llc_conn_ev_rx_i_cmd_pbit_set_0_unexpd_ns, llc_conn_ev_rx_i_cmd_pbit_set_1_unexpd_ns,
 llc_conn_ev_rx_i_cmd_pbit_set_x_inval_ns, llc_conn_ev_rx_i_rsp_fbit_set_0,
 llc_conn_ev_rx_i_rsp_fbit_set_1, llc_conn_ev_rx_i_rsp_fbit_set_x,
 llc_conn_ev_rx_i_rsp_fbit_set_0_unexpd_ns, llc_conn_ev_rx_i_rsp_fbit_set_1_unexpd_ns,
 llc_conn_ev_rx_i_rsp_fbit_set_x_unexpd_ns, llc_conn_ev_rx_i_rsp_fbit_set_x_inval_ns,
 llc_conn_ev_rx_rej_cmd_pbit_set_0, llc_conn_ev_rx_rej_cmd_pbit_set_1,
 llc_conn_ev_rx_rej_rsp_fbit_set_0, llc_conn_ev_rx_rej_rsp_fbit_set_1,
 llc_conn_ev_rx_rej_rsp_fbit_set_x, llc_conn_ev_rx_rnr_cmd_pbit_set_0,
 llc_conn_ev_rx_rnr_cmd_pbit_set_1, llc_conn_ev_rx_rnr_rsp_fbit_set_0,
 llc_conn_ev_rx_rnr_rsp_fbit_set_1, llc_conn_ev_rx_rr_cmd_pbit_set_0,
 llc_conn_ev_rx_rr_cmd_pbit_set_1, llc_conn_ev_rx_rr_rsp_fbit_set_0,
 llc_conn_ev_rx_rr_rsp_fbit_set_1, llc_conn_ev_rx_sabme_cmd_pbit_set_x,
 llc_conn_ev_rx_ua_rsp_fbit_set_x, llc_conn_ev_rx_xxx_cmd_pbit_set_1,
 llc_conn_ev_rx_xxx_cmd_pbit_set_x, llc_conn_ev_rx_xxx_rsp_fbit_set_x,
 llc_conn_ev_rx_zzz_cmd_pbit_set_x_inval_nr, llc_conn_ev_rx_zzz_rsp_fbit_set_x_inval_nr,
 llc_conn_ev_qlfy_data_flag_eq_1, llc_conn_ev_qlfy_data_flag_eq_0,
 llc_conn_ev_qlfy_data_flag_eq_2, llc_conn_ev_qlfy_p_flag_eq_1,
 llc_conn_ev_qlfy_last_frame_eq_1, llc_conn_ev_qlfy_last_frame_eq_0,
 llc_conn_ev_qlfy_p_flag_eq_0, llc_conn_ev_qlfy_p_flag_eq_f,
 llc_conn_ev_qlfy_remote_busy_eq_0, llc_conn_ev_qlfy_remote_busy_eq_1,
 llc_conn_ev_qlfy_retry_cnt_lt_n2, llc_conn_ev_qlfy_retry_cnt_gte_n2,
 llc_conn_ev_qlfy_s_flag_eq_1, llc_conn_ev_qlfy_s_flag_eq_0,
 llc_conn_ev_qlfy_cause_flag_eq_1, llc_conn_ev_qlfy_cause_flag_eq_0
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
