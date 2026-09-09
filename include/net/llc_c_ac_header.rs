/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 *                   2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */
/* Connection component state transition actions */
/*
 * Connection state transition actions
 * (Fb = F bit; Pb = P bit; Xb = X bit)
 */

// Dependency supplied by other translated units: linux/types.h

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

pub const LLC_CONN_AC_CLR_REMOTE_BUSY: i32 = 1;
pub const LLC_CONN_AC_CONN_IND: i32 = 2;
pub const LLC_CONN_AC_CONN_CONFIRM: i32 = 3;
pub const LLC_CONN_AC_DATA_IND: i32 = 4;
pub const LLC_CONN_AC_DISC_IND: i32 = 5;
pub const LLC_CONN_AC_RESET_IND: i32 = 6;
pub const LLC_CONN_AC_RESET_CONFIRM: i32 = 7;
pub const LLC_CONN_AC_REPORT_STATUS: i32 = 8;
pub const LLC_CONN_AC_CLR_REMOTE_BUSY_IF_Fb_EQ_1: i32 = 9;
pub const LLC_CONN_AC_STOP_REJ_TMR_IF_DATA_FLAG_EQ_2: i32 = 10;
pub const LLC_CONN_AC_SEND_DISC_CMD_Pb_SET_X: i32 = 11;
pub const LLC_CONN_AC_SEND_DM_RSP_Fb_SET_Pb: i32 = 12;
pub const LLC_CONN_AC_SEND_DM_RSP_Fb_SET_1: i32 = 13;
pub const LLC_CONN_AC_SEND_DM_RSP_Fb_SET_F_FLAG: i32 = 14;
pub const LLC_CONN_AC_SEND_FRMR_RSP_Fb_SET_X: i32 = 15;
pub const LLC_CONN_AC_RESEND_FRMR_RSP_Fb_SET_0: i32 = 16;
pub const LLC_CONN_AC_RESEND_FRMR_RSP_Fb_SET_Pb: i32 = 17;
pub const LLC_CONN_AC_SEND_I_CMD_Pb_SET_1: i32 = 18;
pub const LLC_CONN_AC_RESEND_I_CMD_Pb_SET_1: i32 = 19;
pub const LLC_CONN_AC_RESEND_I_CMD_Pb_SET_1_OR_SEND_RR: i32 = 20;
pub const LLC_CONN_AC_SEND_I_XXX_Xb_SET_0: i32 = 21;
pub const LLC_CONN_AC_RESEND_I_XXX_Xb_SET_0: i32 = 22;
pub const LLC_CONN_AC_RESEND_I_XXX_Xb_SET_0_OR_SEND_RR: i32 = 23;
pub const LLC_CONN_AC_RESEND_I_RSP_Fb_SET_1: i32 = 24;
pub const LLC_CONN_AC_SEND_REJ_CMD_Pb_SET_1: i32 = 25;
pub const LLC_CONN_AC_SEND_REJ_RSP_Fb_SET_1: i32 = 26;
pub const LLC_CONN_AC_SEND_REJ_XXX_Xb_SET_0: i32 = 27;
pub const LLC_CONN_AC_SEND_RNR_CMD_Pb_SET_1: i32 = 28;
pub const LLC_CONN_AC_SEND_RNR_RSP_Fb_SET_1: i32 = 29;
pub const LLC_CONN_AC_SEND_RNR_XXX_Xb_SET_0: i32 = 30;
pub const LLC_CONN_AC_SET_REMOTE_BUSY: i32 = 31;
pub const LLC_CONN_AC_OPTIONAL_SEND_RNR_XXX_Xb_SET_0: i32 = 32;
pub const LLC_CONN_AC_SEND_RR_CMD_Pb_SET_1: i32 = 33;
pub const LLC_CONN_AC_SEND_ACK_CMD_Pb_SET_1: i32 = 34;
pub const LLC_CONN_AC_SEND_RR_RSP_Fb_SET_1: i32 = 35;
pub const LLC_CONN_AC_SEND_ACK_RSP_Fb_SET_1: i32 = 36;
pub const LLC_CONN_AC_SEND_RR_XXX_Xb_SET_0: i32 = 37;
pub const LLC_CONN_AC_SEND_ACK_XXX_Xb_SET_0: i32 = 38;
pub const LLC_CONN_AC_SEND_SABME_CMD_Pb_SET_X: i32 = 39;
pub const LLC_CONN_AC_SEND_UA_RSP_Fb_SET_Pb: i32 = 40;
pub const LLC_CONN_AC_SEND_UA_RSP_Fb_SET_F_FLAG: i32 = 41;
pub const LLC_CONN_AC_S_FLAG_SET_0: i32 = 42;
pub const LLC_CONN_AC_S_FLAG_SET_1: i32 = 43;
pub const LLC_CONN_AC_START_P_TMR: i32 = 44;
pub const LLC_CONN_AC_START_ACK_TMR: i32 = 45;
pub const LLC_CONN_AC_START_REJ_TMR: i32 = 46;
pub const LLC_CONN_AC_START_ACK_TMR_IF_NOT_RUNNING: i32 = 47;
pub const LLC_CONN_AC_STOP_ACK_TMR: i32 = 48;
pub const LLC_CONN_AC_STOP_P_TMR: i32 = 49;
pub const LLC_CONN_AC_STOP_REJ_TMR: i32 = 50;
pub const LLC_CONN_AC_STOP_ALL_TMRS: i32 = 51;
pub const LLC_CONN_AC_STOP_OTHER_TMRS: i32 = 52;
pub const LLC_CONN_AC_UPDATE_Nr_RECEIVED: i32 = 53;
pub const LLC_CONN_AC_UPDATE_P_FLAG: i32 = 54;
pub const LLC_CONN_AC_DATA_FLAG_SET_2: i32 = 55;
pub const LLC_CONN_AC_DATA_FLAG_SET_0: i32 = 56;
pub const LLC_CONN_AC_DATA_FLAG_SET_1: i32 = 57;
pub const LLC_CONN_AC_DATA_FLAG_SET_1_IF_DATA_FLAG_EQ_0: i32 = 58;
pub const LLC_CONN_AC_P_FLAG_SET_0: i32 = 59;
pub const LLC_CONN_AC_P_FLAG_SET_P: i32 = 60;
pub const LLC_CONN_AC_REMOTE_BUSY_SET_0: i32 = 61;
pub const LLC_CONN_AC_RETRY_CNT_SET_0: i32 = 62;
pub const LLC_CONN_AC_RETRY_CNT_INC_BY_1: i32 = 63;
pub const LLC_CONN_AC_Vr_SET_0: i32 = 64;
pub const LLC_CONN_AC_Vr_INC_BY_1: i32 = 65;
pub const LLC_CONN_AC_Vs_SET_0: i32 = 66;
pub const LLC_CONN_AC_Vs_SET_Nr: i32 = 67;
pub const LLC_CONN_AC_F_FLAG_SET_P: i32 = 68;
pub const LLC_CONN_AC_STOP_SENDACK_TMR: i32 = 70;
pub const LLC_CONN_AC_START_SENDACK_TMR_IF_NOT_RUNNING: i32 = 71;

pub type llc_conn_action_t = unsafe extern "C" fn(*mut sock, *mut sk_buff) -> i32;

extern "C" {
    pub fn llc_conn_ac_clear_remote_busy(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_conn_ind(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_conn_confirm(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_data_ind(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_disc_ind(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_rst_ind(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_rst_confirm(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_clear_remote_busy_if_f_eq_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_stop_rej_tmr_if_data_flag_eq_2(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_disc_cmd_p_set_x(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_dm_rsp_f_set_p(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_dm_rsp_f_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_frmr_rsp_f_set_x(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_resend_frmr_rsp_f_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_resend_frmr_rsp_f_set_p(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_i_cmd_p_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_i_xxx_x_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_resend_i_xxx_x_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_resend_i_xxx_x_set_0_or_send_rr(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_resend_i_rsp_f_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rej_cmd_p_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rej_rsp_f_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rej_xxx_x_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rnr_cmd_p_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rnr_rsp_f_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rnr_xxx_x_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_remote_busy(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_opt_send_rnr_xxx_x_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rr_cmd_p_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rr_rsp_f_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_ack_rsp_f_set_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_rr_xxx_x_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_ack_xxx_x_set_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_sabme_cmd_p_set_x(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_ua_rsp_f_set_p(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_s_flag_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_s_flag_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_start_p_timer(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_start_ack_timer(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_start_rej_timer(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_start_ack_tmr_if_not_running(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_stop_ack_timer(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_stop_p_timer(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_stop_rej_timer(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_stop_all_timers(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_stop_other_timers(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_upd_nr_received(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_inc_tx_win_size(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_dec_tx_win_size(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_upd_p_flag(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_data_flag_2(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_data_flag_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_data_flag_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_data_flag_1_if_data_flag_eq_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_p_flag_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_remote_busy_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_retry_cnt_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_cause_flag_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_cause_flag_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_inc_retry_cnt_by_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_vr_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_inc_vr_by_1(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_vs_0(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_set_vs_nr(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_rst_vs(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_upd_vs(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_disc(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_reset(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_disc_confirm(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_circular_between(a: u8, b: u8, c: u8) -> u8;
    pub fn llc_conn_ac_send_ack_if_needed(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_adjust_npta_by_rr(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_adjust_npta_by_rnr(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_rst_sendack_flag(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_ac_send_i_as_ack(_: *mut sock, _: *mut sk_buff) -> i32;
    pub fn llc_conn_busy_tmr_cb(t: *mut timer_list);
    pub fn llc_conn_pf_cycle_tmr_cb(t: *mut timer_list);
    pub fn llc_conn_ack_tmr_cb(t: *mut timer_list);
    pub fn llc_conn_rej_tmr_cb(t: *mut timer_list);
    pub fn llc_conn_set_p_flag(sk: *mut sock, value: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
