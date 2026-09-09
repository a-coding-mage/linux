/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2017 Oracle.  All rights reserved. */

/* enum ib_event_type, from include/rdma/ib_verbs.h */
#[macro_export]
macro_rules! IB_EVENT_LIST {
    ($m:ident) => {
        $m!(CQ_ERR); $m!(QP_FATAL); $m!(QP_REQ_ERR); $m!(QP_ACCESS_ERR);
        $m!(COMM_EST); $m!(SQ_DRAINED); $m!(PATH_MIG); $m!(PATH_MIG_ERR);
        $m!(DEVICE_FATAL); $m!(PORT_ACTIVE); $m!(PORT_ERR); $m!(LID_CHANGE);
        $m!(PKEY_CHANGE); $m!(SM_CHANGE); $m!(SRQ_ERR); $m!(SRQ_LIMIT_REACHED);
        $m!(QP_LAST_WQE_REACHED); $m!(CLIENT_REREGISTER); $m!(GID_CHANGE);
        $m!(WQ_FATAL);
    };
}

#[macro_export]
macro_rules! rdma_show_ib_event {
    ($x:expr) => { __print_symbolic!($x, IB_EVENT_LIST) };
}

/* enum ib_wc_status type, from include/rdma/ib_verbs.h */
#[macro_export]
macro_rules! IB_WC_STATUS_LIST {
    ($m:ident) => {
        $m!(SUCCESS); $m!(LOC_LEN_ERR); $m!(LOC_QP_OP_ERR); $m!(LOC_EEC_OP_ERR);
        $m!(LOC_PROT_ERR); $m!(WR_FLUSH_ERR); $m!(MW_BIND_ERR); $m!(BAD_RESP_ERR);
        $m!(LOC_ACCESS_ERR); $m!(REM_INV_REQ_ERR); $m!(REM_ACCESS_ERR);
        $m!(REM_OP_ERR); $m!(RETRY_EXC_ERR); $m!(RNR_RETRY_EXC_ERR);
        $m!(LOC_RDD_VIOL_ERR); $m!(REM_INV_RD_REQ_ERR); $m!(REM_ABORT_ERR);
        $m!(INV_EECN_ERR); $m!(INV_EEC_STATE_ERR); $m!(FATAL_ERR);
        $m!(RESP_TIMEOUT_ERR); $m!(GENERAL_ERR);
    };
}

#[macro_export]
macro_rules! rdma_show_wc_status {
    ($x:expr) => { __print_symbolic!($x, IB_WC_STATUS_LIST) };
}

/* enum ib_cm_event_type, from include/rdma/ib_cm.h */
#[macro_export]
macro_rules! IB_CM_EVENT_LIST {
    ($m:ident) => {
        $m!(REQ_ERROR); $m!(REQ_RECEIVED); $m!(REP_ERROR); $m!(REP_RECEIVED);
        $m!(RTU_RECEIVED); $m!(USER_ESTABLISHED); $m!(DREQ_ERROR);
        $m!(DREQ_RECEIVED); $m!(DREP_RECEIVED); $m!(TIMEWAIT_EXIT);
        $m!(MRA_RECEIVED); $m!(REJ_RECEIVED); $m!(LAP_ERROR); $m!(LAP_RECEIVED);
        $m!(APR_RECEIVED); $m!(SIDR_REQ_ERROR); $m!(SIDR_REQ_RECEIVED);
        $m!(SIDR_REP_RECEIVED);
    };
}

#[macro_export]
macro_rules! rdma_show_ib_cm_event {
    ($x:expr) => { __print_symbolic!($x, IB_CM_EVENT_LIST) };
}

/* enum rdma_cm_event_type, from include/rdma/rdma_cm.h */
#[macro_export]
macro_rules! RDMA_CM_EVENT_LIST {
    ($m:ident) => {
        $m!(ADDR_RESOLVED); $m!(ADDR_ERROR); $m!(ROUTE_RESOLVED);
        $m!(ROUTE_ERROR); $m!(CONNECT_REQUEST); $m!(CONNECT_RESPONSE);
        $m!(CONNECT_ERROR); $m!(UNREACHABLE); $m!(REJECTED); $m!(ESTABLISHED);
        $m!(DISCONNECTED); $m!(DEVICE_REMOVAL); $m!(MULTICAST_JOIN);
        $m!(MULTICAST_ERROR); $m!(ADDR_CHANGE); $m!(TIMEWAIT_EXIT);
    };
}

#[macro_export]
macro_rules! rdma_show_cm_event {
    ($x:expr) => { __print_symbolic!($x, RDMA_CM_EVENT_LIST) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
