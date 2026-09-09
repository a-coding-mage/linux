//! Rust translation of `trace/events/preemptirq.h`.
//!
//! The tracepoint registration and formatting machinery referenced by the
//! original header is provided by the surrounding kernel tracepoint system.

#[cfg(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS")]
#[repr(C)]
pub struct PreemptirqEntry {
    pub caller_offs: i32,
    pub parent_offs: i32,
}

#[cfg(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS")]
extern "C" {
    static _stext: u8;
}

#[cfg(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS")]
#[inline]
pub unsafe fn preemptirq_assign_entry(
    entry: *mut PreemptirqEntry,
    ip: usize,
    parent_ip: usize,
) {
    // TP_fast_assign: offsets are signed 32-bit values relative to _stext.
    (*entry).caller_offs = (ip.wrapping_sub((&_stext as *const u8) as usize)) as i32;
    (*entry).parent_offs =
        (parent_ip.wrapping_sub((&_stext as *const u8) as usize)) as i32;
}

#[cfg(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS")]
#[inline]
pub unsafe fn preemptirq_caller_address(entry: *const PreemptirqEntry) -> *const u8 {
    ((&_stext as *const u8 as usize)
        .wrapping_add((*entry).caller_offs as isize as usize)) as *const u8
}

#[cfg(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS")]
#[inline]
pub unsafe fn preemptirq_parent_address(entry: *const PreemptirqEntry) -> *const u8 {
    ((&_stext as *const u8 as usize)
        .wrapping_add((*entry).parent_offs as isize as usize)) as *const u8
}

// DEFINE_EVENT(preemptirq_template, irq_disable/irq_enable)
// and DEFINE_EVENT(preemptirq_template, preempt_disable/preempt_enable)
// are tracepoint declarations supplied by the kernel tracepoint subsystem.

#[cfg(all(
    feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS",
    feature = "CONFIG_TRACE_IRQFLAGS"
))]
extern "C" {
    pub fn trace_irq_disable(ip: usize, parent_ip: usize);
    pub fn trace_irq_enable(ip: usize, parent_ip: usize);
}

#[cfg(all(
    feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS",
    not(feature = "CONFIG_TRACE_IRQFLAGS")
))]
#[inline]
pub fn trace_irq_disable(_ip: usize, _parent_ip: usize) {}

#[cfg(all(
    feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS",
    not(feature = "CONFIG_TRACE_IRQFLAGS")
))]
#[inline]
pub fn trace_irq_enable(_ip: usize, _parent_ip: usize) {}

#[cfg(all(
    feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS",
    feature = "CONFIG_TRACE_PREEMPT_TOGGLE"
))]
extern "C" {
    pub fn trace_preempt_disable(ip: usize, parent_ip: usize);
    pub fn trace_preempt_enable(ip: usize, parent_ip: usize);
}

#[cfg(all(
    feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS",
    not(feature = "CONFIG_TRACE_PREEMPT_TOGGLE")
))]
#[inline]
pub fn trace_preempt_disable(_ip: usize, _parent_ip: usize) {}

#[cfg(all(
    feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS",
    not(feature = "CONFIG_TRACE_PREEMPT_TOGGLE")
))]
#[inline]
pub fn trace_preempt_enable(_ip: usize, _parent_ip: usize) {}

// !CONFIG_PREEMPTIRQ_TRACEPOINTS: the original variadic trace macros expand
// to no-ops. These Rust functions provide the corresponding call surface.
#[cfg(not(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS"))]
#[inline]
pub fn trace_irq_enable(_ip: usize, _parent_ip: usize) {}

#[cfg(not(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS"))]
#[inline]
pub fn trace_irq_disable(_ip: usize, _parent_ip: usize) {}

#[cfg(not(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS"))]
#[inline]
pub fn trace_preempt_enable(_ip: usize, _parent_ip: usize) {}

#[cfg(not(feature = "CONFIG_PREEMPTIRQ_TRACEPOINTS"))]
#[inline]
pub fn trace_preempt_disable(_ip: usize, _parent_ip: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
