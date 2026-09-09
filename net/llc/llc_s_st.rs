// SPDX-License-Identifier: GPL-2.0
/*
 * llc_s_st.c - Defines SAP component state machine transitions.
 *
 * The followed transitions are SAP component state machine transitions
 * which are described in 802.2 LLC protocol standard document.
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *                     2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

/* External declarations are supplied by the corresponding LLC headers. */

/* dummy last-transition indicator; common to all state transition groups
 * last entry for this state
 * all members are zeros, .bss zeroes it
 */
static llc_sap_state_trans_end: llc_sap_state_trans = llc_sap_state_trans {
    ev: 0,
    next_state: 0,
    ev_actions: core::ptr::null(),
};

/* state LLC_SAP_STATE_INACTIVE transition for
 * LLC_SAP_EV_ACTIVATION_REQ event
 */
static llc_sap_inactive_state_actions_1: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_report_status),
    None,
];

static llc_sap_inactive_state_trans_1: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_activation_req,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_inactive_state_actions_1.as_ptr(),
};

/* array of pointers; one to each transition */
static llc_sap_inactive_state_transitions: [*const llc_sap_state_trans; 2] = [
    &llc_sap_inactive_state_trans_1,
    &llc_sap_state_trans_end,
];

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_RX_UI event */
static llc_sap_active_state_actions_1: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_unitdata_ind),
    None,
];

static llc_sap_active_state_trans_1: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_rx_ui,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_1.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_UNITDATA_REQ event */
static llc_sap_active_state_actions_2: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_send_ui),
    None,
];

static llc_sap_active_state_trans_2: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_unitdata_req,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_2.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_XID_REQ event */
static llc_sap_active_state_actions_3: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_send_xid_c),
    None,
];

static llc_sap_active_state_trans_3: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_xid_req,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_3.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_RX_XID_C event */
static llc_sap_active_state_actions_4: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_send_xid_r),
    None,
];

static llc_sap_active_state_trans_4: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_rx_xid_c,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_4.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_RX_XID_R event */
static llc_sap_active_state_actions_5: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_xid_ind),
    None,
];

static llc_sap_active_state_trans_5: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_rx_xid_r,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_5.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_TEST_REQ event */
static llc_sap_active_state_actions_6: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_send_test_c),
    None,
];

static llc_sap_active_state_trans_6: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_test_req,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_6.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_RX_TEST_C event */
static llc_sap_active_state_actions_7: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_send_test_r),
    None,
];

static llc_sap_active_state_trans_7: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_rx_test_c,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_7.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for LLC_SAP_EV_RX_TEST_R event */
static llc_sap_active_state_actions_8: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_test_ind),
    None,
];

static llc_sap_active_state_trans_8: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_rx_test_r,
    next_state: LLC_SAP_STATE_ACTIVE,
    ev_actions: llc_sap_active_state_actions_8.as_ptr(),
};

/* state LLC_SAP_STATE_ACTIVE transition for
 * LLC_SAP_EV_DEACTIVATION_REQ event
 */
static llc_sap_active_state_actions_9: [Option<llc_sap_action_t>; 2] = [
    Some(llc_sap_action_report_status),
    None,
];

static llc_sap_active_state_trans_9: llc_sap_state_trans = llc_sap_state_trans {
    ev: llc_sap_ev_deactivation_req,
    next_state: LLC_SAP_STATE_INACTIVE,
    ev_actions: llc_sap_active_state_actions_9.as_ptr(),
};

/* array of pointers; one to each transition */
static mut llc_sap_active_state_transitions: [*const llc_sap_state_trans; 10] = [
    &llc_sap_active_state_trans_2,
    &llc_sap_active_state_trans_1,
    &llc_sap_active_state_trans_3,
    &llc_sap_active_state_trans_4,
    &llc_sap_active_state_trans_5,
    &llc_sap_active_state_trans_6,
    &llc_sap_active_state_trans_7,
    &llc_sap_active_state_trans_8,
    &llc_sap_active_state_trans_9,
    &llc_sap_state_trans_end,
];

/* SAP state transition table */
static mut llc_sap_state_table: [llc_sap_state; LLC_NR_SAP_STATES] = {
    let mut table = [llc_sap_state {
        curr_state: 0,
        transitions: core::ptr::null(),
    }; LLC_NR_SAP_STATES];
    table[LLC_SAP_STATE_INACTIVE - 1] = llc_sap_state {
        curr_state: LLC_SAP_STATE_INACTIVE,
        transitions: llc_sap_inactive_state_transitions.as_ptr(),
    };
    table[LLC_SAP_STATE_ACTIVE - 1] = llc_sap_state {
        curr_state: LLC_SAP_STATE_ACTIVE,
        transitions: llc_sap_active_state_transitions.as_ptr(),
    };
    table
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
