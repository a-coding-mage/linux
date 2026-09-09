/* SPDX-License-Identifier: GPL-2.0 */

/*
 * TRACE_SYSTEM exceptions
 *
 * The original header uses the Linux tracepoint declaration macros.  The
 * following Rust items preserve the event payload and the two event
 * declarations without providing implementations for the external tracepoint
 * machinery.
 */

use core::ffi::c_ulong;

/// Opaque register state supplied by the architecture-specific code.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/// External equivalent of `instruction_pointer(regs)`.
extern "C" {
    pub fn instruction_pointer(regs: *mut pt_regs) -> c_ulong;
}

/// Payload declared by the `exceptions` trace event class.
#[repr(C)]
pub struct ExceptionsEntry {
    pub address: c_ulong,
    pub ip: c_ulong,
    pub error_code: c_ulong,
}

impl ExceptionsEntry {
    /// Equivalent to the `TP_fast_assign` block.
    #[inline]
    pub unsafe fn assign(
        &mut self,
        address: c_ulong,
        regs: *mut pt_regs,
        error_code: c_ulong,
    ) {
        self.address = address;
        self.ip = instruction_pointer(regs);
        self.error_code = error_code;
    }

    /// Equivalent to the `TP_printk` format and arguments.
    #[inline]
    pub fn printk_arguments(&self) -> (c_ulong, c_ulong, c_ulong) {
        (self.address, self.ip, self.error_code)
    }
}

/// `TP_printk("address=%ps ip=%ps error_code=0x%lx", ...)`.
pub const EXCEPTIONS_PRINTK_FORMAT: &str =
    "address=%ps ip=%ps error_code=0x%lx";

extern "C" {
    /// Declaration equivalent to `DEFINE_EVENT(exceptions, page_fault_user, ...)`.
    pub fn page_fault_user(
        address: c_ulong,
        regs: *mut pt_regs,
        error_code: c_ulong,
    );

    /// Declaration equivalent to `DEFINE_EVENT(exceptions, page_fault_kernel, ...)`.
    pub fn page_fault_kernel(
        address: c_ulong,
        regs: *mut pt_regs,
        error_code: c_ulong,
    );
}

/* The original include is intentionally represented as an external
 * dependency; its trace-definition implementation is outside this file. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
