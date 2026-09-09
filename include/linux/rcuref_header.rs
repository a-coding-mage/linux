/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation:
// linux/atomic.h, linux/bug.h, linux/limits.h, linux/lockdep.h,
// linux/preempt.h, and linux/rcupdate.h.

pub const RCUREF_ONEREF: u32 = 0x0000_0000;
pub const RCUREF_MAXREF: u32 = 0x7fff_ffff;
pub const RCUREF_SATURATED: u32 = 0xa000_0000;
pub const RCUREF_RELEASED: u32 = 0xc000_0000;
pub const RCUREF_DEAD: u32 = 0xe000_0000;
pub const RCUREF_NOREF: u32 = 0xffff_ffff;

/// Initialize a rcuref reference count with the given reference count.
#[inline]
pub unsafe fn rcuref_init(ref_: *mut rcuref_t, cnt: u32) {
	atomic_set(&mut (*ref_).refcnt, cnt.wrapping_sub(1));
}

/// Read the number of held reference counts of a rcuref.
#[inline]
pub unsafe fn rcuref_read(ref_: *mut rcuref_t) -> u32 {
	let c: u32 = atomic_read(&(*ref_).refcnt);

	// Return 0 if within the DEAD zone.
	if c >= RCUREF_RELEASED { 0 } else { c.wrapping_add(1) }
}

/// Check if the rcuref has been already marked dead.
#[inline]
pub unsafe fn rcuref_is_dead(ref_: *mut rcuref_t) -> bool {
	let c: u32 = atomic_read(&(*ref_).refcnt);

	c >= RCUREF_RELEASED && c < RCUREF_NOREF
}

extern "C" {
	pub fn rcuref_get_slowpath(ref_: *mut rcuref_t) -> bool;
}

/// Acquire one reference on a rcuref reference count.
#[inline]
pub unsafe fn rcuref_get(ref_: *mut rcuref_t) -> bool {
	// Unconditionally increase the reference count. The saturation and dead
	// zones provide enough tolerance for this.
	if !atomic_add_negative_relaxed(1, &mut (*ref_).refcnt) {
		return true;
	}

	// Handle the cases inside the saturation and dead zones.
	rcuref_get_slowpath(ref_)
}

extern "C" {
	pub fn rcuref_put_slowpath(ref_: *mut rcuref_t, cnt: u32) -> bool;
}

// Internal helper. Do not invoke directly.
#[inline(always)]
pub unsafe fn __rcuref_put(ref_: *mut rcuref_t) -> bool {
	let cnt: i32;

	// RCU_LOCKDEP_WARN(!rcu_read_lock_held() && preemptible(),
	//                  "suspicious rcuref_put_rcusafe() usage");
	// Unconditionally decrease the reference count. The saturation and dead
	// zones provide enough tolerance for this.
	cnt = atomic_sub_return_release(1, &mut (*ref_).refcnt);
	if cnt >= 0 {
		return false;
	}

	// Handle the last reference drop and cases inside the saturation and dead
	// zones.
	rcuref_put_slowpath(ref_, cnt as u32)
}

/// Release one reference for a rcuref reference count, RCU safe.
#[inline]
pub unsafe fn rcuref_put_rcusafe(ref_: *mut rcuref_t) -> bool {
	__rcuref_put(ref_)
}

/// Release one reference for a rcuref reference count.
#[inline]
pub unsafe fn rcuref_put(ref_: *mut rcuref_t) -> bool {
	preempt_disable();
	let released = __rcuref_put(ref_);
	preempt_enable();
	released
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
