/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * event_channel.h
 *
 * Event channels between domains.
 *
 * Copyright (c) 2003-2004, K A Fraser.
 */

// Dependency: xen/interface/xen.h supplies `domid_t` and `xen_ulong_t`.

pub type evtchn_port_t = u32;
pub type evtchn_port_handle_t = *mut evtchn_port_t;

pub const EVTCHNOP_alloc_unbound: u32 = 6;
#[repr(C)]
pub struct evtchn_alloc_unbound {
    pub dom: domid_t,
    pub remote_dom: domid_t,
    pub port: evtchn_port_t,
}

pub const EVTCHNOP_bind_interdomain: u32 = 0;
#[repr(C)]
pub struct evtchn_bind_interdomain {
    pub remote_dom: domid_t,
    pub remote_port: evtchn_port_t,
    pub local_port: evtchn_port_t,
}

pub const EVTCHNOP_bind_virq: u32 = 1;
#[repr(C)]
pub struct evtchn_bind_virq {
    pub virq: u32,
    pub vcpu: u32,
    pub port: evtchn_port_t,
}

pub const EVTCHNOP_bind_pirq: u32 = 2;
pub const BIND_PIRQ__WILL_SHARE: u32 = 1;
#[repr(C)]
pub struct evtchn_bind_pirq {
    pub pirq: u32,
    pub flags: u32,
    pub port: evtchn_port_t,
}

pub const EVTCHNOP_bind_ipi: u32 = 7;
#[repr(C)]
pub struct evtchn_bind_ipi {
    pub vcpu: u32,
    pub port: evtchn_port_t,
}

pub const EVTCHNOP_close: u32 = 3;
#[repr(C)]
pub struct evtchn_close {
    pub port: evtchn_port_t,
}

pub const EVTCHNOP_send: u32 = 4;
#[repr(C)]
pub struct evtchn_send {
    pub port: evtchn_port_t,
}

pub const EVTCHNOP_status: u32 = 5;
pub const EVTCHNSTAT_closed: u32 = 0;
pub const EVTCHNSTAT_unbound: u32 = 1;
pub const EVTCHNSTAT_interdomain: u32 = 2;
pub const EVTCHNSTAT_pirq: u32 = 3;
pub const EVTCHNSTAT_virq: u32 = 4;
pub const EVTCHNSTAT_ipi: u32 = 5;

#[repr(C)]
pub struct evtchn_status_unbound {
    pub dom: domid_t,
}
#[repr(C)]
pub struct evtchn_status_interdomain {
    pub dom: domid_t,
    pub port: evtchn_port_t,
}
#[repr(C)]
pub union evtchn_status_u {
    pub unbound: evtchn_status_unbound,
    pub interdomain: evtchn_status_interdomain,
    pub pirq: u32,
    pub virq: u32,
}
#[repr(C)]
pub struct evtchn_status {
    pub dom: domid_t,
    pub port: evtchn_port_t,
    pub status: u32,
    pub vcpu: u32,
    pub u: evtchn_status_u,
}

pub const EVTCHNOP_bind_vcpu: u32 = 8;
#[repr(C)]
pub struct evtchn_bind_vcpu {
    pub port: evtchn_port_t,
    pub vcpu: u32,
}

pub const EVTCHNOP_unmask: u32 = 9;
#[repr(C)]
pub struct evtchn_unmask {
    pub port: evtchn_port_t,
}

pub const EVTCHNOP_reset: u32 = 10;
#[repr(C)]
pub struct evtchn_reset {
    pub dom: domid_t,
}
pub type evtchn_reset_t = evtchn_reset;

pub const EVTCHNOP_init_control: u32 = 11;
#[repr(C)]
pub struct evtchn_init_control {
    pub control_gfn: u64,
    pub offset: u32,
    pub vcpu: u32,
    pub link_bits: u8,
    pub _pad: [u8; 7],
}

pub const EVTCHNOP_expand_array: u32 = 12;
#[repr(C)]
pub struct evtchn_expand_array {
    pub array_gfn: u64,
}

pub const EVTCHNOP_set_priority: u32 = 13;
#[repr(C)]
pub struct evtchn_set_priority {
    pub port: evtchn_port_t,
    pub priority: u32,
}

#[repr(C)]
pub union evtchn_op_u {
    pub alloc_unbound: evtchn_alloc_unbound,
    pub bind_interdomain: evtchn_bind_interdomain,
    pub bind_virq: evtchn_bind_virq,
    pub bind_pirq: evtchn_bind_pirq,
    pub bind_ipi: evtchn_bind_ipi,
    pub close: evtchn_close,
    pub send: evtchn_send,
    pub status: evtchn_status,
    pub bind_vcpu: evtchn_bind_vcpu,
    pub unmask: evtchn_unmask,
}
#[repr(C)]
pub struct evtchn_op {
    pub cmd: u32,
    pub u: evtchn_op_u,
}
pub type evtchn_op_handle_t = *mut evtchn_op;

pub const EVTCHN_2L_NR_CHANNELS: usize = core::mem::size_of::<xen_ulong_t>() * core::mem::size_of::<xen_ulong_t>() * 64;

pub const EVTCHN_FIFO_PRIORITY_MAX: u32 = 0;
pub const EVTCHN_FIFO_PRIORITY_DEFAULT: u32 = 7;
pub const EVTCHN_FIFO_PRIORITY_MIN: u32 = 15;
pub const EVTCHN_FIFO_MAX_QUEUES: usize = (EVTCHN_FIFO_PRIORITY_MIN + 1) as usize;

pub type event_word_t = u32;
pub const EVTCHN_FIFO_PENDING: u32 = 31;
pub const EVTCHN_FIFO_MASKED: u32 = 30;
pub const EVTCHN_FIFO_LINKED: u32 = 29;
pub const EVTCHN_FIFO_BUSY: u32 = 28;
pub const EVTCHN_FIFO_LINK_BITS: u32 = 17;
pub const EVTCHN_FIFO_LINK_MASK: u32 = (1u32 << EVTCHN_FIFO_LINK_BITS) - 1;
pub const EVTCHN_FIFO_NR_CHANNELS: usize = 1usize << EVTCHN_FIFO_LINK_BITS;

#[repr(C)]
pub struct evtchn_fifo_control_block {
    pub ready: u32,
    pub _rsvd: u32,
    pub head: [event_word_t; EVTCHN_FIFO_MAX_QUEUES],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
