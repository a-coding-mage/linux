/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM page_isolation
// The Linux tracepoint and trace-definition headers are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct TestPagesIsolatedEntry {
    pub start_pfn: core::ffi::c_ulong,
    pub end_pfn: core::ffi::c_ulong,
    pub fin_pfn: core::ffi::c_ulong,
}

impl TestPagesIsolatedEntry {
    #[inline]
    pub const fn new(
        start_pfn: core::ffi::c_ulong,
        end_pfn: core::ffi::c_ulong,
        fin_pfn: core::ffi::c_ulong,
    ) -> Self {
        Self {
            start_pfn,
            end_pfn,
            fin_pfn,
        }
    }

    #[inline]
    pub const fn result(&self) -> &'static str {
        if self.end_pfn <= self.fin_pfn {
            "success"
        } else {
            "fail"
        }
    }
}

/// Translation of the `test_pages_isolated` TRACE_EVENT declaration.
///
/// The original event accepts `start_pfn`, `end_pfn`, and `fin_pfn`, stores
/// them in the trace entry, and prints:
/// `start_pfn=0x%lx end_pfn=0x%lx fin_pfn=0x%lx ret=%s`.
#[macro_export]
macro_rules! test_pages_isolated {
    ($start_pfn:expr, $end_pfn:expr, $fin_pfn:expr) => {{
        $crate::TestPagesIsolatedEntry::new($start_pfn, $end_pfn, $fin_pfn)
    }};
}

// <trace/define_trace.h> is intentionally supplied by the surrounding build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
