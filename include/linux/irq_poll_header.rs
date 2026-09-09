/* SPDX-License-Identifier: GPL-2.0 */

// `list_head` is supplied by the surrounding kernel translation.
use core::ffi::{c_int, c_ulong};

pub type irq_poll_fn = unsafe extern "C" fn(*mut irq_poll, c_int) -> c_int;

#[repr(C)]
pub struct irq_poll {
	pub list: list_head,
	pub state: c_ulong,
	pub weight: c_int,
	pub poll: Option<irq_poll_fn>,
}

pub const IRQ_POLL_F_SCHED: c_int = 0;
pub const IRQ_POLL_F_DISABLE: c_int = 1;

unsafe extern "C" {
	pub fn irq_poll_sched(iop: *mut irq_poll);
	pub fn irq_poll_init(iop: *mut irq_poll, weight: c_int, poll: Option<irq_poll_fn>);
	pub fn irq_poll_complete(iop: *mut irq_poll);
	pub fn irq_poll_enable(iop: *mut irq_poll);
	pub fn irq_poll_disable(iop: *mut irq_poll);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
