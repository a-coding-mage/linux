/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 *		2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

/* Dependencies supplied by the corresponding translated headers:
 * net/llc_c_ac.h and net/llc_c_ev.h
 */

/* Connection component state management */
/* connection states */
pub const LLC_CONN_OUT_OF_SVC: u32 = 0; /* prior to allocation */

pub const LLC_CONN_STATE_ADM: u32 = 1; /* disc, initial state */
pub const LLC_CONN_STATE_SETUP: u32 = 2; /* disconnected state */
pub const LLC_CONN_STATE_NORMAL: u32 = 3; /* connected state */
pub const LLC_CONN_STATE_BUSY: u32 = 4; /* connected state */
pub const LLC_CONN_STATE_REJ: u32 = 5; /* connected state */
pub const LLC_CONN_STATE_AWAIT: u32 = 6; /* connected state */
pub const LLC_CONN_STATE_AWAIT_BUSY: u32 = 7; /* connected state */
pub const LLC_CONN_STATE_AWAIT_REJ: u32 = 8; /* connected state */
pub const LLC_CONN_STATE_D_CONN: u32 = 9; /* disconnected state */
pub const LLC_CONN_STATE_RESET: u32 = 10; /* disconnected state */
pub const LLC_CONN_STATE_ERROR: u32 = 11; /* disconnected state */
pub const LLC_CONN_STATE_TEMP: u32 = 12; /* disconnected state */

pub const NBR_CONN_STATES: u32 = 12; /* size of state table */
pub const NO_STATE_CHANGE: u32 = 100;

/* Connection state table structure */
#[repr(C)]
pub struct llc_conn_state_trans {
    pub ev: llc_conn_ev_t,
    pub next_state: u8,
    pub ev_qualifiers: *const llc_conn_ev_qfyr_t,
    pub ev_actions: *const llc_conn_action_t,
}

#[repr(C)]
pub struct llc_conn_state {
    pub current_state: u8,
    pub transitions: *const *const llc_conn_state_trans,
}

extern "C" {
    pub static mut llc_conn_state_table: [llc_conn_state; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
