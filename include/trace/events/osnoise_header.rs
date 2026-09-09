/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/osnoise.h.
// The Linux tracepoint macros below are represented by their C-layout payloads
// and assignment/printing contracts; referenced kernel symbols remain external.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct osnoise_sample {
    pub runtime: u64,       /* runtime */
    pub noise: u64,         /* noise */
    pub max_sample: u64,    /* max single noise sample */
    pub hw_count: c_int,    /* # HW (incl. hypervisor) interference */
    pub nmi_count: c_int,   /* # NMIs during this sample */
    pub irq_count: c_int,   /* # IRQs during this sample */
    pub softirq_count: c_int, /* # softirqs during this sample */
    pub thread_count: c_int, /* # threads during this sample */
}

#[cfg(CONFIG_TIMERLAT_TRACER)]
#[repr(C)]
pub struct timerlat_sample {
    pub timer_latency: u64, /* timer_latency */
    pub seqnum: u32,        /* unique sequence */
    pub context: c_int,     /* timer context */
}

#[repr(C)]
pub struct osnoise_sample_entry {
    pub runtime: u64,
    pub noise: u64,
    pub max_sample: u64,
    pub hw_count: c_int,
    pub irq_count: c_int,
    pub nmi_count: c_int,
    pub softirq_count: c_int,
    pub thread_count: c_int,
}

#[cfg(CONFIG_TIMERLAT_TRACER)]
#[repr(C)]
pub struct timerlat_sample_entry {
    pub timer_latency: u64,
    pub seqnum: u32,
    pub context: c_int,
}

#[repr(C)]
pub struct thread_noise_entry {
    pub comm: [c_char; TASK_COMM_LEN],
    pub start: u64,
    pub duration: u64,
    pub pid: pid_t,
}

#[repr(C)]
pub struct softirq_noise_entry {
    pub start: u64,
    pub duration: u64,
    pub vector: c_int,
}

#[repr(C)]
pub struct irq_noise_entry {
    pub start: u64,
    pub duration: u64,
    pub desc: *const c_char,
    pub vector: c_int,
}

#[repr(C)]
pub struct nmi_noise_entry {
    pub start: u64,
    pub duration: u64,
}

#[repr(C)]
pub struct sample_threshold_entry {
    pub start: u64,
    pub duration: u64,
    pub interference: u64,
}

// TRACE_EVENT(osnoise_sample):
// TP_PROTO(struct osnoise_sample *s)
#[inline]
pub unsafe fn osnoise_sample_assign(entry: *mut osnoise_sample_entry, s: *const osnoise_sample) {
    (*entry).runtime = (*s).runtime;
    (*entry).noise = (*s).noise;
    (*entry).max_sample = (*s).max_sample;
    (*entry).hw_count = (*s).hw_count;
    (*entry).irq_count = (*s).irq_count;
    (*entry).nmi_count = (*s).nmi_count;
    (*entry).softirq_count = (*s).softirq_count;
    (*entry).thread_count = (*s).thread_count;
}

#[cfg(CONFIG_TIMERLAT_TRACER)]
#[inline]
pub unsafe fn timerlat_sample_assign(entry: *mut timerlat_sample_entry, s: *const timerlat_sample) {
    (*entry).timer_latency = (*s).timer_latency;
    (*entry).seqnum = (*s).seqnum;
    (*entry).context = (*s).context;
}

// TRACE_EVENT(thread_noise): comm is copied from t->comm; pid, start, and
// duration are assigned directly. The trace printk format is:
// "%8s:%d start %llu.%09u duration %llu ns".
// TRACE_EVENT(softirq_noise): printk uses show_softirq_name(vector) and the
// same start/duration format.
// TRACE_EVENT(irq_noise): desc is assigned as a trace string, then vector,
// start, and duration are assigned; printk uses "%s:%d ...".
// TRACE_EVENT(nmi_noise): printk uses "start %llu.%09u duration %llu ns".
// TRACE_EVENT(sample_threshold): printk uses
// "start %llu.%09u duration %llu ns interference %llu".

extern "C" {
    pub type pid_t;
    pub const TASK_COMM_LEN: usize;
    pub fn show_softirq_name(vector: c_int) -> *const c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
