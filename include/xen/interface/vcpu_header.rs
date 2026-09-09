/* SPDX-License-Identifier: MIT */
/*
 * vcpu.h
 *
 * VCPU initialisation, query, and hotplug.
 *
 * Copyright (c) 2005, Keir Fraser <keir@xensource.com>
 */

/*
 * Prototype for this hypercall is:
 *     int vcpu_op(int cmd, int vcpuid, void *extra_args)
 * @cmd        == VCPUOP_??? (VCPU operation).
 * @vcpuid     == VCPU to operate on.
 * @extra_args == Operation-specific extra arguments (NULL if none).
 */

pub const VCPUOP_initialise: i32 = 0;
pub const VCPUOP_up: i32 = 1;
pub const VCPUOP_down: i32 = 2;
pub const VCPUOP_is_up: i32 = 3;
pub const VCPUOP_get_runstate_info: i32 = 4;

#[repr(C)]
pub struct vcpu_runstate_info {
    /* VCPU's current state (RUNSTATE_*). */
    pub state: i32,
    /* When was current state entered (system time, ns)? */
    pub state_entry_time: u64,
    /* Update indicator set in state_entry_time. */
    pub time: [u64; 4],
}

pub const XEN_RUNSTATE_UPDATE: u64 = 1u64 << 63;

pub const RUNSTATE_running: i32 = 0;
pub const RUNSTATE_runnable: i32 = 1;
pub const RUNSTATE_blocked: i32 = 2;
pub const RUNSTATE_offline: i32 = 3;

pub const VCPUOP_register_runstate_memory_area: i32 = 5;

#[repr(C)]
pub union vcpu_register_runstate_memory_area_addr {
    pub h: *mut vcpu_runstate_info,
    pub v: *mut vcpu_runstate_info,
    pub p: u64,
}

#[repr(C)]
pub struct vcpu_register_runstate_memory_area {
    pub addr: vcpu_register_runstate_memory_area_addr,
}

pub const VCPUOP_set_periodic_timer: i32 = 6;
pub const VCPUOP_stop_periodic_timer: i32 = 7;

#[repr(C)]
pub struct vcpu_set_periodic_timer {
    pub period_ns: u64,
}

pub const VCPUOP_set_singleshot_timer: i32 = 8;
pub const VCPUOP_stop_singleshot_timer: i32 = 9;

#[repr(C)]
pub struct vcpu_set_singleshot_timer {
    pub timeout_abs_ns: u64,
    pub flags: u32,
}

pub const _VCPU_SSHOTTMR_future: u32 = 0;
pub const VCPU_SSHOTTMR_future: u32 = 1u32 << _VCPU_SSHOTTMR_future;

pub const VCPUOP_register_vcpu_info: i32 = 10;

#[repr(C)]
pub struct vcpu_register_vcpu_info {
    pub mfn: u64,
    pub offset: u32,
    pub rsvd: u32,
}

pub const VCPUOP_send_nmi: i32 = 11;
pub const VCPUOP_get_physid: i32 = 12;

#[repr(C)]
pub struct vcpu_get_physid {
    pub phys_id: u64,
}

#[inline]
pub const fn xen_vcpu_physid_to_x86_apicid(physid: u64) -> u32 {
    physid as u32
}

#[inline]
pub const fn xen_vcpu_physid_to_x86_acpiid(physid: u64) -> u32 {
    (physid >> 32) as u32
}

pub const VCPUOP_register_vcpu_time_memory_area: i32 = 13;

/* vcpu_time_info is declared by the corresponding shared ABI header. */
#[repr(C)]
pub union vcpu_register_time_memory_area_addr {
    pub h: *mut vcpu_time_info,
    pub v: *mut pvclock_vcpu_time_info,
    pub p: u64,
}

#[repr(C)]
pub struct vcpu_register_time_memory_area {
    pub addr: vcpu_register_time_memory_area_addr,
}

/* Guest-handle declarations from DEFINE_GUEST_HANDLE_STRUCT are represented
 * by the corresponding ABI structures above; handle aliases are external
 * dependencies supplied by the surrounding Xen interface. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
