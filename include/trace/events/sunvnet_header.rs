/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM: sunvnet
// The Linux tracepoint and trace-definition headers are external dependencies.

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VnetRxOneEntry {
    pub lsid: ::core::ffi::c_int,
    pub rsid: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub needs_ack: ::core::ffi::c_int,
}

pub const VNET_RX_ONE_PRINTK: &str =
    "(%x:%x) walk_rx_one index %d; needs_ack %d";

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VnetTxStoppedAckTemplateEntry {
    pub lsid: ::core::ffi::c_int,
    pub rsid: ::core::ffi::c_int,
    pub ack_end: ::core::ffi::c_int,
    pub npkts: ::core::ffi::c_int,
}

pub const VNET_TX_STOPPED_ACK_PRINTK: &str =
    "(%x:%x) stopped ack for %d; npkts %d";

// DEFINE_EVENT(vnet_tx_stopped_ack_template, vnet_tx_send_stopped_ack,
//              TP_PROTO(int lsid, int rsid, int ack_end, int npkts),
//              TP_ARGS(lsid, rsid, ack_end, npkts));
// DEFINE_EVENT(vnet_tx_stopped_ack_template, vnet_tx_defer_stopped_ack,
//              TP_PROTO(int lsid, int rsid, int ack_end, int npkts),
//              TP_ARGS(lsid, rsid, ack_end, npkts));
// DEFINE_EVENT(vnet_tx_stopped_ack_template, vnet_tx_pending_stopped_ack,
//              TP_PROTO(int lsid, int rsid, int ack_end, int npkts),
//              TP_ARGS(lsid, rsid, ack_end, npkts));

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VnetRxStoppedAckEntry {
    pub lsid: ::core::ffi::c_int,
    pub rsid: ::core::ffi::c_int,
    pub end: ::core::ffi::c_int,
}

pub const VNET_RX_STOPPED_ACK_PRINTK: &str =
    "(%x:%x) stopped ack for index %d";

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VnetTxTriggerEntry {
    pub lsid: ::core::ffi::c_int,
    pub rsid: ::core::ffi::c_int,
    pub start: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
}

pub const VNET_TX_TRIGGER_PRINTK: &str =
    "(%x:%x) Tx trigger for %d sent with err %d %s";

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VnetSkipTxTriggerEntry {
    pub lsid: ::core::ffi::c_int,
    pub rsid: ::core::ffi::c_int,
    pub last: ::core::ffi::c_int,
}

pub const VNET_SKIP_TX_TRIGGER_PRINTK: &str =
    "(%x:%x) Skip Tx trigger. Last trigger sent was %d";

// The original TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT invocations
// define tracepoint registration and dispatch through the external Linux
// tracing framework. Their TP_PROTO and TP_ARGS signatures are:
//   vnet_rx_one(int lsid, int rsid, int index, int needs_ack)
//   vnet_tx_stopped_ack_template(int lsid, int rsid, int ack_end, int npkts)
//   vnet_rx_stopped_ack(int lsid, int rsid, int end)
//   vnet_tx_trigger(int lsid, int rsid, int start, int err)
//   vnet_skip_tx_trigger(int lsid, int rsid, int last)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
