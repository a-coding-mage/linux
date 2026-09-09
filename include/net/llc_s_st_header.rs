/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 *                      2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

// C dependencies: <linux/types.h>, <net/llc_s_ac.h>, and <net/llc_s_ev.h>.

pub const LLC_NR_SAP_STATES: usize = 2; /* size of state table */

/* structures and types */
/* SAP state table structure */
#[repr(C)]
pub struct llc_sap_state_trans {
    pub ev: llc_sap_ev_t,
    pub next_state: u8,
    pub ev_actions: *const llc_sap_action_t,
}

#[repr(C)]
pub struct llc_sap_state {
    pub curr_state: u8,
    pub transitions: *const *const llc_sap_state_trans,
}

/* only access to SAP state table */
extern "C" {
    pub static mut llc_sap_state_table: [llc_sap_state; LLC_NR_SAP_STATES];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
