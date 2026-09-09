/* SPDX-License-Identifier: GPL-2.0 */

/* Equivalent of the C CONFIG_TRACE_IRQFLAGS conditional. */
#[cfg(feature = "CONFIG_TRACE_IRQFLAGS")]
#[repr(C)]
pub struct irqtrace_events {
	pub irq_events: core::ffi::c_uint,
	pub hardirq_enable_ip: core::ffi::c_ulong,
	pub hardirq_disable_ip: core::ffi::c_ulong,
	pub hardirq_enable_event: core::ffi::c_uint,
	pub hardirq_disable_event: core::ffi::c_uint,
	pub softirq_disable_ip: core::ffi::c_ulong,
	pub softirq_enable_ip: core::ffi::c_ulong,
	pub softirq_disable_event: core::ffi::c_uint,
	pub softirq_enable_event: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
