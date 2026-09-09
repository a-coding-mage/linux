/* SPDX-License-Identifier: MIT */
/*
 * sched.h -- Scheduler state interactions
 *
 * Rust translation of the Xen public scheduler interface header.
 * The event-channel and Xen ABI types referenced here are supplied externally.
 */

/*
 * Guest Scheduler Operations
 *
 * The SCHEDOP interface provides mechanisms for a guest to interact
 * with the scheduler, including yield, blocking and shutting itself
 * down.
 */

/*
 * The prototype for this hypercall is:
 * long HYPERVISOR_sched_op(enum sched_op cmd, void *arg, ...)
 *
 * @cmd == SCHEDOP_??? (scheduler operation).
 * @arg == Operation-specific extra argument(s), as described below.
 * ...  == Additional Operation-specific extra arguments, described below.
 *
 * Versions of Xen prior to 3.0.2 provided only the following legacy version
 * of this hypercall, supporting only the commands yield, block and shutdown.
 * This legacy version is available to new guests as:
 * long HYPERVISOR_sched_op_compat(enum sched_op cmd, unsigned long arg)
 */

/* Voluntarily yield the CPU. @arg == NULL. */
pub const SCHEDOP_yield: u32 = 0;

/* Block execution of this VCPU until an event is received for processing. */
pub const SCHEDOP_block: u32 = 1;

/* Halt execution of this domain and notify the system controller. */
pub const SCHEDOP_shutdown: u32 = 2;

/* Poll a set of event-channel ports. */
pub const SCHEDOP_poll: u32 = 3;

/* Declare a shutdown for another domain. */
pub const SCHEDOP_remote_shutdown: u32 = 4;

/* Latch a shutdown code. */
pub const SCHEDOP_shutdown_code: u32 = 5;

/* Setup, poke and destroy a domain watchdog timer. */
pub const SCHEDOP_watchdog: u32 = 6;

/* Override the current VCPU affinity. */
pub const SCHEDOP_pin_override: u32 = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_shutdown {
    pub reason: ::core::ffi::c_uint, /* SHUTDOWN_* => shutdown reason */
}

/* DEFINE_GUEST_HANDLE_STRUCT(sched_shutdown); */
pub type sched_shutdown_t = *mut sched_shutdown;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_poll {
    /* GUEST_HANDLE(evtchn_port_t) ports; */
    pub ports: *mut evtchn_port_t,
    pub nr_ports: ::core::ffi::c_uint,
    pub timeout: u64,
}

/* DEFINE_GUEST_HANDLE_STRUCT(sched_poll); */
pub type sched_poll_t = *mut sched_poll;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_remote_shutdown {
    pub domain_id: domid_t,      /* Remote domain ID */
    pub reason: ::core::ffi::c_uint, /* SHUTDOWN_* => shutdown reason */
}

/* DEFINE_GUEST_HANDLE_STRUCT(sched_remote_shutdown); */
pub type sched_remote_shutdown_t = *mut sched_remote_shutdown;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_watchdog {
    pub id: u32,      /* watchdog ID */
    pub timeout: u32, /* timeout */
}

/* DEFINE_GUEST_HANDLE_STRUCT(sched_watchdog); */
pub type sched_watchdog_t = *mut sched_watchdog;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_pin_override {
    pub pcpu: i32,
}

/* DEFINE_GUEST_HANDLE_STRUCT(sched_pin_override); */
pub type sched_pin_override_t = *mut sched_pin_override;

/*
 * Reason codes for SCHEDOP_shutdown. These may be interpreted by control
 * software to determine the appropriate action. For the most part, Xen does
 * not care about the shutdown code.
 */
pub const SHUTDOWN_poweroff: u32 = 0; /* Domain exited normally. Clean up and kill. */
pub const SHUTDOWN_reboot: u32 = 1;   /* Clean up, kill, and then restart. */
pub const SHUTDOWN_suspend: u32 = 2;  /* Clean up, save suspend info, kill. */
pub const SHUTDOWN_crash: u32 = 3;    /* Tell controller we've crashed. */
pub const SHUTDOWN_watchdog: u32 = 4; /* Restart because watchdog time expired. */

/* Domain asked to perform 'soft reset' for it. */
pub const SHUTDOWN_soft_reset: u32 = 5;
pub const SHUTDOWN_MAX: u32 = 5; /* Maximum valid shutdown reason. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
