/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM signal
// The Linux signal, scheduler, tracepoint, and trace definition headers are
// dependencies of this translation and are intentionally not reimplemented.

/*
 * TP_STORE_SIGINFO(__entry, info)
 *
 * The C macro stores errno/code as follows:
 *   SEND_SIG_NOINFO => errno = 0, code = SI_USER
 *   SEND_SIG_PRIV   => errno = 0, code = SI_KERNEL
 *   otherwise       => errno = info->si_errno, code = info->si_code
 */

#[cfg(not(feature = "trace_header_multi_read"))]
#[repr(i32)]
pub enum TraceSignal {
    Delivered,
    Ignored,
    AlreadyPending,
    OverflowFail,
    LoseInfo,
}

/// signal_generate - called when a signal is generated
///
/// Current process sends a `sig` signal to `task` with `info` siginfo.  When
/// `info` is SEND_SIG_NOINFO or SEND_SIG_PRIV it is not a pointer: those cases
/// mean SI_USER and SI_KERNEL respectively.
#[repr(C)]
pub struct SignalGenerateEntry {
    pub sig: core::ffi::c_int,
    pub errno: core::ffi::c_int,
    pub code: core::ffi::c_int,
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
    pub pid: pid_t,
    pub group: core::ffi::c_int,
    pub result: core::ffi::c_int,
}

/// signal_deliver - called when a signal is delivered
///
/// A `sig` signal is delivered to the current process with `info` siginfo and
/// handled by `ka`.  Some generated signals may be lost, ignored, or modified
/// before reaching this tracepoint.
#[repr(C)]
pub struct SignalDeliverEntry {
    pub sig: core::ffi::c_int,
    pub errno: core::ffi::c_int,
    pub code: core::ffi::c_int,
    pub sa_handler: core::ffi::c_ulong,
    pub sa_flags: core::ffi::c_ulong,
}

// The following declarations are the direct Rust representation of the two
// TRACE_EVENT definitions.  Their registration, fast-assignment actions, and
// TP_printk formats are supplied by the tracepoint implementation.
pub const SIGNAL_GENERATE_PROTO: &str =
    "(int sig, struct kernel_siginfo *info, struct task_struct *task, int group, int result)";
pub const SIGNAL_GENERATE_PRINTK: &str =
    "sig=%d errno=%d code=%d comm=%s pid=%d grp=%d res=%d";
pub const SIGNAL_DELIVER_PROTO: &str =
    "(int sig, struct kernel_siginfo *info, struct k_sigaction *ka)";
pub const SIGNAL_DELIVER_PRINTK: &str =
    "sig=%d errno=%d code=%d sa_handler=%lx sa_flags=%lx";

// External kernel types/constants referenced by the header are provided by
// the corresponding translated dependency files.
extern "C" {
    static TASK_COMM_LEN: usize;
}

type pid_t = core::ffi::c_int;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
