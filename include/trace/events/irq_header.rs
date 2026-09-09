/* SPDX-License-Identifier: GPL-2.0 */
// Translation of trace/events/irq.h.  The Linux tracepoint framework and the
// SOFTIRQ constants are supplied by other translation units.

use core::ffi::c_void;

#[repr(C)]
pub struct irqaction {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct softirq_action {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tasklet_struct {
    _private: [u8; 0],
}

// SOFTIRQ_NAME_LIST expands to the kernel's softirq enumeration in the
// original header.  Keep the expansion order and spelling here for users of
// the tracepoint representation.
pub const SOFTIRQ_NAME_LIST: &[(&str, u32)] = &[
    ("HI", HI_SOFTIRQ),
    ("TIMER", TIMER_SOFTIRQ),
    ("NET_TX", NET_TX_SOFTIRQ),
    ("NET_RX", NET_RX_SOFTIRQ),
    ("BLOCK", BLOCK_SOFTIRQ),
    ("IRQ_POLL", IRQ_POLL_SOFTIRQ),
    ("TASKLET", TASKLET_SOFTIRQ),
    ("SCHED", SCHED_SOFTIRQ),
    ("HRTIMER", HRTIMER_SOFTIRQ),
    ("RCU", RCU_SOFTIRQ),
];

extern "C" {
    static HI_SOFTIRQ: u32;
    static TIMER_SOFTIRQ: u32;
    static NET_TX_SOFTIRQ: u32;
    static NET_RX_SOFTIRQ: u32;
    static BLOCK_SOFTIRQ: u32;
    static IRQ_POLL_SOFTIRQ: u32;
    static TASKLET_SOFTIRQ: u32;
    static SCHED_SOFTIRQ: u32;
    static HRTIMER_SOFTIRQ: u32;
    static RCU_SOFTIRQ: u32;
}

pub fn show_softirq_name(val: u32) -> Option<&'static str> {
    SOFTIRQ_NAME_LIST.iter().find(|(_, number)| *number == val).map(|(name, _)| *name)
}

#[repr(C)]
pub struct irq_handler_entry {
    pub irq: core::ffi::c_int,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct irq_handler_exit {
    pub irq: core::ffi::c_int,
    pub ret: core::ffi::c_int,
}

#[repr(C)]
pub struct softirq {
    pub vec: u32,
}

#[repr(C)]
pub struct tasklet {
    pub tasklet: *mut c_void,
    pub func: *mut c_void,
}

// TRACE_EVENT(irq_handler_entry): called immediately before the irq action
// handler.  The tracepoint registration is provided by the trace subsystem.
pub unsafe fn irq_handler_entry(irq: core::ffi::c_int, action: *mut irqaction) -> irq_handler_entry {
    irq_handler_entry { irq, name: (*action).name }
}

// TRACE_EVENT(irq_handler_exit): called immediately after the irq action handler returns.
pub fn irq_handler_exit(irq: core::ffi::c_int, ret: core::ffi::c_int) -> irq_handler_exit {
    irq_handler_exit { irq, ret }
}

// DECLARE_EVENT_CLASS(softirq), and DEFINE_EVENT for softirq_entry,
// softirq_exit, and softirq_raise.
pub fn softirq_event(vec_nr: u32) -> softirq {
    softirq { vec: vec_nr }
}

pub fn softirq_entry(vec_nr: u32) -> softirq { softirq_event(vec_nr) }
pub fn softirq_exit(vec_nr: u32) -> softirq { softirq_event(vec_nr) }
pub fn softirq_raise(vec_nr: u32) -> softirq { softirq_event(vec_nr) }

// DECLARE_EVENT_CLASS(tasklet), and DEFINE_EVENT for tasklet_entry and
// tasklet_exit.
pub fn tasklet_event(t: *mut tasklet_struct, func: *mut c_void) -> tasklet {
    tasklet { tasklet: t.cast(), func }
}

pub fn tasklet_entry(t: *mut tasklet_struct, func: *mut c_void) -> tasklet {
    tasklet_event(t, func)
}

pub fn tasklet_exit(t: *mut tasklet_struct, func: *mut c_void) -> tasklet {
    tasklet_event(t, func)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
