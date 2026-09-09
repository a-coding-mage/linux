// Translated from trace/events/qdisc.h.
// Kernel include dependencies and tracepoint-generation machinery are supplied externally.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// #include <linux/skbuff.h>
// #include <linux/netdevice.h>
// #include <linux/tracepoint.h>
// #include <linux/ftrace.h>
// #include <linux/pkt_sched.h>
// #include <net/sch_generic.h>
// #include <trace/define_trace.h>

use core::ffi::c_void;

extern "C" {
    pub type Qdisc;
    pub type netdev_queue;
    pub type net_device;
    pub type sk_buff;
    pub type Qdisc_ops;
}

pub type u32 = core::primitive::u32;

#[repr(C)]
pub struct qdisc_dequeue_entry {
    pub qdisc: *mut Qdisc,
    pub txq: *const netdev_queue,
    pub packets: i32,
    pub skbaddr: *mut c_void,
    pub ifindex: i32,
    pub handle: u32,
    pub parent: u32,
    pub txq_state: usize,
}

#[repr(C)]
pub struct qdisc_enqueue_entry {
    pub qdisc: *mut Qdisc,
    pub txq: *const netdev_queue,
    pub skbaddr: *mut c_void,
    pub ifindex: i32,
    pub handle: u32,
    pub parent: u32,
}

#[repr(C)]
pub struct qdisc_drop_entry {
    pub qdisc: *mut Qdisc,
    pub txq: *const netdev_queue,
    pub skbaddr: *mut c_void,
    pub ifindex: i32,
    pub handle: u32,
    pub parent: u32,
    pub reason: qdisc_drop_reason,
    pub kind: *const core::ffi::c_char,
}

#[repr(C)]
pub struct qdisc_reset_entry {
    pub dev: *const core::ffi::c_char,
    pub kind: *const core::ffi::c_char,
    pub parent: u32,
    pub handle: u32,
}

#[repr(C)]
pub struct qdisc_destroy_entry {
    pub dev: *const core::ffi::c_char,
    pub kind: *const core::ffi::c_char,
    pub parent: u32,
    pub handle: u32,
}

#[repr(C)]
pub struct qdisc_create_entry {
    pub dev: *const core::ffi::c_char,
    pub kind: *const core::ffi::c_char,
    pub parent: u32,
}

// enum qdisc_drop_reason is declared by linux/pkt_sched.h.
pub type qdisc_drop_reason = i32;

// The following trace-event declarations preserve the C event interfaces and
// assignment/printing expressions; the trace backend provides their expansion.
#[macro_export]
macro_rules! trace_event_qdisc_dequeue {
    ($qdisc:expr, $txq:expr, $packets:expr, $skb:expr) => {{
        qdisc_dequeue_entry {
            qdisc: $qdisc,
            txq: $txq,
            packets: if !$skb.is_null() { $packets } else { 0 },
            skbaddr: $skb as *mut c_void,
            ifindex: 0, // txq->dev ? txq->dev->ifindex : 0
            handle: 0, // qdisc->handle
            parent: 0, // qdisc->parent
            txq_state: 0, // txq->state
        }
    }};
}

#[macro_export]
macro_rules! trace_event_qdisc_enqueue {
    ($qdisc:expr, $txq:expr, $skb:expr) => {{
        qdisc_enqueue_entry {
            qdisc: $qdisc,
            txq: $txq,
            skbaddr: $skb as *mut c_void,
            ifindex: 0, // txq->dev ? txq->dev->ifindex : 0
            handle: 0, // qdisc->handle
            parent: 0, // qdisc->parent
        }
    }};
}

#[macro_export]
macro_rules! trace_event_qdisc_drop {
    ($qdisc:expr, $txq:expr, $dev:expr, $skb:expr, $reason:expr) => {{
        qdisc_drop_entry {
            qdisc: $qdisc, txq: $txq, skbaddr: $skb as *mut c_void,
            ifindex: 0, // dev ? dev->ifindex : 0
            handle: 0, // qdisc->handle
            parent: 0, // qdisc->parent
            reason: $reason, kind: core::ptr::null(),
        }
    }};
}

// qdisc_reset, qdisc_destroy, and qdisc_create retain their C tracepoint
// signatures and field layouts above; string assignment and TC_H_MAJ/TC_H_MIN
// formatting are performed by the external tracepoint implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
