/* SPDX-License-Identifier: MIT */

// Dependency: <linux/kthread.h>

use core::ffi::c_int;

pub struct drm_crtc;
pub struct drm_vblank_crtc;
pub struct kthread_work;
pub struct list_head;

/**
 * struct drm_vblank_work - A delayed work item which delays until a target
 * vblank passes, and then executes at realtime priority outside of IRQ
 * context.
 *
 * See also:
 * drm_vblank_work_schedule()
 * drm_vblank_work_init()
 * drm_vblank_work_cancel_sync()
 * drm_vblank_work_flush()
 * drm_vblank_work_flush_all()
 */
#[repr(C)]
pub struct drm_vblank_work {
	/**
	 * @base: The base &kthread_work item which will be executed by
	 * &drm_vblank_crtc.worker. Drivers should not interact with this
	 * directly, and instead rely on drm_vblank_work_init() to initialize
	 * this.
	 */
	pub base: kthread_work,

	/**
	 * @vblank: A pointer to &drm_vblank_crtc this work item belongs to.
	 */
	pub vblank: *mut drm_vblank_crtc,

	/**
	 * @count: The target vblank this work will execute on. Drivers should
	 * not modify this value directly, and instead use
	 * drm_vblank_work_schedule()
	 */
	pub count: u64,

	/**
	 * @cancelling: The number of drm_vblank_work_cancel_sync() calls that
	 * are currently running. A work item cannot be rescheduled until all
	 * calls have finished.
	 */
	pub cancelling: c_int,

	/**
	 * @node: The position of this work item in
	 * &drm_vblank_crtc.pending_work.
	 */
	pub node: list_head,
}

/**
 * to_drm_vblank_work - Retrieve the respective &drm_vblank_work item from a
 * &kthread_work
 * @_work: The &kthread_work embedded inside a &drm_vblank_work
 */
#[macro_export]
macro_rules! to_drm_vblank_work {
	($work:expr) => {
		container_of!($work, $crate::drm_vblank_work, base)
	};
}

extern "C" {
	pub fn drm_vblank_work_schedule(
		work: *mut drm_vblank_work,
		count: u64,
		nextonmiss: bool,
	) -> c_int;
	pub fn drm_vblank_work_init(
		work: *mut drm_vblank_work,
		crtc: *mut drm_crtc,
		func: Option<unsafe extern "C" fn(work: *mut kthread_work)>,
	);
	pub fn drm_vblank_work_cancel_sync(work: *mut drm_vblank_work) -> bool;
	pub fn drm_vblank_work_flush(work: *mut drm_vblank_work);
	pub fn drm_vblank_work_flush_all(crtc: *mut drm_crtc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
