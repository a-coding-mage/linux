/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM is `msr`.
// TRACE_INCLUDE_FILE is `msr-trace` and TRACE_INCLUDE_PATH is `asm/`.
// The C header guard is preserved as conditional intent; Rust items are
// normally defined once by the module system.

/*
 * Tracing for x86 model specific registers. Directly maps to the
 * RDMSR/WRMSR instructions.
 */

#[repr(C)]
pub struct MsrTraceEntry {
    pub msr: ::core::ffi::c_uint,
    pub val: u64,
    pub failed: ::core::ffi::c_int,
}

impl MsrTraceEntry {
    #[inline]
    pub const fn new(
        msr: ::core::ffi::c_uint,
        val: u64,
        failed: ::core::ffi::c_int,
    ) -> Self {
        Self { msr, val, failed }
    }

    // Equivalent to TP_printk("%x, value %llx%s", ...).
    #[inline]
    pub fn print(&self) -> (u32, u64, &'static str) {
        (self.msr as u32, self.val, if self.failed != 0 { " #GP" } else { "" })
    }
}

// DECLARE_EVENT_CLASS(msr_trace_class, ...):
// The event class stores the MSR number, value, and failure status above.
// TP_fast_assign performs the corresponding field assignments:
#[inline]
pub unsafe fn msr_trace_class_assign(
    entry: *mut MsrTraceEntry,
    msr: ::core::ffi::c_uint,
    val: u64,
    failed: ::core::ffi::c_int,
) {
    // SAFETY: The caller supplies the event entry pointer, matching the C
    // tracepoint contract.
    (*entry).msr = msr;
    (*entry).val = val;
    (*entry).failed = failed;
}

// DEFINE_EVENT(msr_trace_class, read_msr, ...)
// DEFINE_EVENT(msr_trace_class, write_msr, ...)
// DEFINE_EVENT(msr_trace_class, rdpmc, ...)
// These declarations are supplied by the tracepoint implementation and retain
// the common prototype: (unsigned msr, u64 val, int failed).
extern "C" {
    pub fn read_msr(msr: ::core::ffi::c_uint, val: u64, failed: ::core::ffi::c_int);
    pub fn write_msr(msr: ::core::ffi::c_uint, val: u64, failed: ::core::ffi::c_int);
    pub fn rdpmc(msr: ::core::ffi::c_uint, val: u64, failed: ::core::ffi::c_int);
}

// <linux/tracepoint.h> and <trace/define_trace.h> provide the tracepoint
// machinery in the C build; their dependency intent is retained here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
