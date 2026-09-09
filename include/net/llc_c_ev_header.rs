/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 *                  2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 *
 * Dependency: <net/sock.h>
 */

pub const LLC_CONN_EV_TYPE_SIMPLE: u8 = 1;
pub const LLC_CONN_EV_TYPE_CONDITION: u8 = 2;
pub const LLC_CONN_EV_TYPE_PRIM: u8 = 3;
pub const LLC_CONN_EV_TYPE_PDU: u8 = 4;
pub const LLC_CONN_EV_TYPE_ACK_TMR: u8 = 5;
pub const LLC_CONN_EV_TYPE_P_TMR: u8 = 6;
pub const LLC_CONN_EV_TYPE_REJ_TMR: u8 = 7;
pub const LLC_CONN_EV_TYPE_BUSY_TMR: u8 = 8;
pub const LLC_CONN_EV_TYPE_RPT_STATUS: u8 = 9;
pub const LLC_CONN_EV_TYPE_SENDACK_TMR: u8 = 10;
pub const NBR_CONN_EV: u8 = 5;

/* Connection events which cause state transitions when fully qualified. */
pub const LLC_CONN_EV_CONN_REQ: u8 = 1;
pub const LLC_CONN_EV_CONN_RESP: u8 = 2;
pub const LLC_CONN_EV_DATA_REQ: u8 = 3;
pub const LLC_CONN_EV_DISC_REQ: u8 = 4;
pub const LLC_CONN_EV_RESET_REQ: u8 = 5;
pub const LLC_CONN_EV_RESET_RESP: u8 = 6;
pub const LLC_CONN_EV_LOCAL_BUSY_DETECTED: u8 = 7;
pub const LLC_CONN_EV_LOCAL_BUSY_CLEARED: u8 = 8;
pub const LLC_CONN_EV_RX_BAD_PDU: u8 = 9;
pub const LLC_CONN_EV_RX_DISC_CMD_Pbit_SET_X: u8 = 10;
pub const LLC_CONN_EV_RX_DM_RSP_Fbit_SET_X: u8 = 11;
pub const LLC_CONN_EV_RX_FRMR_RSP_Fbit_SET_X: u8 = 12;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_X: u8 = 13;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_X_UNEXPD_Ns: u8 = 14;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_X_INVAL_Ns: u8 = 15;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_X: u8 = 16;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_X_UNEXPD_Ns: u8 = 17;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_X_INVAL_Ns: u8 = 18;
pub const LLC_CONN_EV_RX_REJ_CMD_Pbit_SET_X: u8 = 19;
pub const LLC_CONN_EV_RX_REJ_RSP_Fbit_SET_X: u8 = 20;
pub const LLC_CONN_EV_RX_RNR_CMD_Pbit_SET_X: u8 = 21;
pub const LLC_CONN_EV_RX_RNR_RSP_Fbit_SET_X: u8 = 22;
pub const LLC_CONN_EV_RX_RR_CMD_Pbit_SET_X: u8 = 23;
pub const LLC_CONN_EV_RX_RR_RSP_Fbit_SET_X: u8 = 24;
pub const LLC_CONN_EV_RX_SABME_CMD_Pbit_SET_X: u8 = 25;
pub const LLC_CONN_EV_RX_UA_RSP_Fbit_SET_X: u8 = 26;
pub const LLC_CONN_EV_RX_XXX_CMD_Pbit_SET_X: u8 = 27;
pub const LLC_CONN_EV_RX_XXX_RSP_Fbit_SET_X: u8 = 28;
pub const LLC_CONN_EV_RX_XXX_YYY: u8 = 29;
pub const LLC_CONN_EV_RX_ZZZ_CMD_Pbit_SET_X_INVAL_Nr: u8 = 30;
pub const LLC_CONN_EV_RX_ZZZ_RSP_Fbit_SET_X_INVAL_Nr: u8 = 31;
pub const LLC_CONN_EV_P_TMR_EXP: u8 = 32;
pub const LLC_CONN_EV_ACK_TMR_EXP: u8 = 33;
pub const LLC_CONN_EV_REJ_TMR_EXP: u8 = 34;
pub const LLC_CONN_EV_BUSY_TMR_EXP: u8 = 35;
pub const LLC_CONN_EV_RX_XXX_CMD_Pbit_SET_1: u8 = 36;
pub const LLC_CONN_EV_RX_XXX_CMD_Pbit_SET_0: u8 = 37;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_0_UNEXPD_Ns: u8 = 38;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_0_UNEXPD_Ns: u8 = 39;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_1_UNEXPD_Ns: u8 = 40;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_1_UNEXPD_Ns: u8 = 41;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_0: u8 = 42;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_0: u8 = 43;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_1: u8 = 44;
pub const LLC_CONN_EV_RX_RR_CMD_Pbit_SET_0: u8 = 45;
pub const LLC_CONN_EV_RX_RR_RSP_Fbit_SET_0: u8 = 46;
pub const LLC_CONN_EV_RX_RR_RSP_Fbit_SET_1: u8 = 47;
pub const LLC_CONN_EV_RX_RR_CMD_Pbit_SET_1: u8 = 48;
pub const LLC_CONN_EV_RX_RNR_CMD_Pbit_SET_0: u8 = 49;
pub const LLC_CONN_EV_RX_RNR_RSP_Fbit_SET_0: u8 = 50;
pub const LLC_CONN_EV_RX_RNR_RSP_Fbit_SET_1: u8 = 51;
pub const LLC_CONN_EV_RX_RNR_CMD_Pbit_SET_1: u8 = 52;
pub const LLC_CONN_EV_RX_REJ_CMD_Pbit_SET_0: u8 = 53;
pub const LLC_CONN_EV_RX_REJ_RSP_Fbit_SET_0: u8 = 54;
pub const LLC_CONN_EV_RX_REJ_CMD_Pbit_SET_1: u8 = 55;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_1: u8 = 56;
pub const LLC_CONN_EV_RX_REJ_RSP_Fbit_SET_1: u8 = 57;
pub const LLC_CONN_EV_RX_XXX_RSP_Fbit_SET_1: u8 = 58;
pub const LLC_CONN_EV_TX_BUFF_FULL: u8 = 59;
pub const LLC_CONN_EV_INIT_P_F_CYCLE: u8 = 100;

pub const LLC_CONN_EV_QFY_DATA_FLAG_EQ_1: u8 = 1;
pub const LLC_CONN_EV_QFY_DATA_FLAG_EQ_0: u8 = 2;
pub const LLC_CONN_EV_QFY_DATA_FLAG_EQ_2: u8 = 3;
pub const LLC_CONN_EV_QFY_P_FLAG_EQ_1: u8 = 4;
pub const LLC_CONN_EV_QFY_P_FLAG_EQ_0: u8 = 5;
pub const LLC_CONN_EV_QFY_P_FLAG_EQ_Fbit: u8 = 6;
pub const LLC_CONN_EV_QFY_REMOTE_BUSY_EQ_0: u8 = 7;
pub const LLC_CONN_EV_QFY_RETRY_CNT_LT_N2: u8 = 8;
pub const LLC_CONN_EV_QFY_RETRY_CNT_GTE_N2: u8 = 9;
pub const LLC_CONN_EV_QFY_S_FLAG_EQ_1: u8 = 10;
pub const LLC_CONN_EV_QFY_S_FLAG_EQ_0: u8 = 11;
pub const LLC_CONN_EV_QFY_INIT_P_F_CYCLE: u8 = 12;

#[repr(C)]
pub struct llc_conn_state_ev {
    pub type_: u8,
    pub prim: u8,
    pub prim_type: u8,
    pub reason: u8,
    pub status: u8,
    pub ind_prim: u8,
    pub cfm_prim: u8,
}

/* Types and fields supplied by the net/sock.h dependency. */
pub type llc_conn_ev_t = unsafe extern "C" fn(*mut sock, *mut sk_buff) -> i32;
pub type llc_conn_ev_qfyr_t = unsafe extern "C" fn(*mut sock, *mut sk_buff) -> i32;

#[repr(C)]
pub struct sock { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { pub cb: [u8; 48], _private: [u8; 0] }

pub unsafe fn llc_conn_ev(skb: *mut sk_buff) -> *mut llc_conn_state_ev {
    (*skb).cb.as_mut_ptr() as *mut llc_conn_state_ev
}

/* The sk_rmem_alloc, sk_rcvbuf, and skb truesize layouts are supplied by net/sock.h. */
pub unsafe fn llc_conn_space(_sk: *mut sock, _skb: *mut sk_buff) -> bool {
    // TODO: preserve the atomic_read/field comparison once the dependency layout is available.
    unimplemented!()
}

unsafe extern "C" {
    pub fn llc_conn_ev_conn_req(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_data_req(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_disc_req(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rst_req(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_local_busy_detected(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_local_busy_cleared(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_bad_pdu(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_disc_cmd_pbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_dm_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_frmr_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_cmd_pbit_set_x_inval_ns(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_x_unexpd_ns(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_x_inval_ns(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rej_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_sabme_cmd_pbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_ua_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_xxx_cmd_pbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_xxx_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_zzz_cmd_pbit_set_x_inval_nr(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_zzz_rsp_fbit_set_x_inval_nr(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_p_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_ack_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rej_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_busy_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_xxx_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_xxx_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_cmd_pbit_set_0_unexpd_ns(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_cmd_pbit_set_1_unexpd_ns(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_0_unexpd_ns(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_1_unexpd_ns(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rr_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rr_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rr_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rr_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rnr_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rnr_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rnr_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rnr_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rej_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rej_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rej_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_rej_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_rx_any_frame(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_tx_buffer_full(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_init_p_f_cycle(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_data_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_data_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_data_flag_eq_2(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_p_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_last_frame_eq_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_last_frame_eq_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_p_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_p_flag_eq_f(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_remote_busy_eq_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_remote_busy_eq_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_retry_cnt_lt_n2(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_retry_cnt_gte_n2(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_s_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_s_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_cause_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_cause_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_set_status_conn(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_set_status_disc(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_set_status_failed(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_set_status_remote_busy(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_set_status_refuse(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_set_status_conflict(sk: *mut sock, skb: *mut sk_buff) -> i32;
    pub fn llc_conn_ev_qlfy_set_status_rst_done(sk: *mut sock, skb: *mut sk_buff) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
