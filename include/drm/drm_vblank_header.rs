/*
 * Copyright 2016 Intel Corp.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct drm_device;
#[repr(C)]
pub struct drm_crtc;
#[repr(C)]
pub struct drm_vblank_work;

#[repr(C)]
pub struct drm_pending_vblank_event {
    // Base structure for tracking pending DRM events.
    pub base: drm_pending_event,
    // drm_crtc_index() of the drm_crtc this event is for.
    pub pipe: u32,
    // Frame event should be triggered at.
    pub sequence: u64,
    // Actual event which will be sent to userspace.
    pub event: drm_pending_vblank_event_event,
}

#[repr(C)]
pub union drm_pending_vblank_event_event {
    // DRM event base class.
    pub base: drm_event,
    // Event payload for vblank events.
    pub vbl: drm_event_vblank,
    // Event payload for the MODE_QUEUEU_SEQUENCE IOCTL.
    pub seq: drm_event_crtc_sequence,
}

#[repr(C)]
pub struct drm_vblank_crtc_config {
    // Vblank off delay in ms.
    pub offdelay_ms: i32,
    // Immediate vblank disabling value for this CRTC.
    pub disable_immediate: bool,
}

#[repr(C)]
pub struct drm_vblank_crtc_timer {
    // The vblank's high-resolution timer.
    pub timer: hrtimer,
    // Protects interval.
    pub interval_lock: spinlock_t,
    // Duration between two vblanks.
    pub interval: ktime_t,
    // The timer's CRTC.
    pub crtc: *mut drm_crtc,
}

#[repr(C)]
pub struct drm_vblank_crtc {
    // Pointer to the drm_device.
    pub dev: *mut drm_device,
    // Wait queue for vblank waiters.
    pub queue: wait_queue_head_t,
    // Disable timer for delayed vblank disabling hysteresis logic.
    pub disable_timer: timer_list,
    // Protect vblank count and time.
    pub seqlock: seqlock_t,
    // Current software vblank counter.
    pub count: atomic64_t,
    // Vblank timestamp corresponding to count.
    pub time: ktime_t,
    // Number of users/waiters of the vblank interrupt.
    pub refcount: atomic_t,
    // Protected by drm_device.vbl_lock, used for wraparound handling.
    pub last: u32,
    // Maximum value of the vblank registers for this CRTC.
    pub max_vblank_count: u32,
    // Tracks whether the vblank is disabled due to a modeset.
    pub inmodeset: u32,
    // drm_crtc_index() of the corresponding drm_crtc.
    pub pipe: u32,
    // Frame/Field duration in ns.
    pub framedur_ns: i32,
    // Line duration in ns.
    pub linedur_ns: i32,
    // Cache of the current hardware display mode.
    pub hwmode: drm_display_mode,
    // Stores vblank configuration values for a given CRTC.
    pub config: drm_vblank_crtc_config,
    // Tracks the enabling state of the corresponding drm_crtc.
    pub enabled: bool,
    // The kthread_worker used for executing vblank works.
    pub worker: *mut kthread_worker,
    // A list of scheduled drm_vblank_work items waiting for a future vblank.
    pub pending_work: list_head,
    // Wait queue for completed or cancelled drm_vblank_work items.
    pub work_wait_queue: wait_queue_head_t,
    // Holds the state of the vblank timer.
    pub vblank_timer: drm_vblank_crtc_timer,
}

pub type drm_vblank_get_scanout_position_func = unsafe extern "C" fn(
    crtc: *mut drm_crtc,
    in_vblank_irq: bool,
    vpos: *mut i32,
    hpos: *mut i32,
    stime: *mut ktime_t,
    etime: *mut ktime_t,
    mode: *const drm_display_mode,
) -> bool;

extern "C" {
    pub fn drm_crtc_vblank_crtc(crtc: *mut drm_crtc) -> *mut drm_vblank_crtc;
    pub fn drm_vblank_init(dev: *mut drm_device, num_crtcs: u32) -> i32;
    pub fn drm_dev_has_vblank(dev: *const drm_device) -> bool;
    pub fn drm_crtc_vblank_count(crtc: *mut drm_crtc) -> u64;
    pub fn drm_crtc_vblank_count_and_time(crtc: *mut drm_crtc, vblanktime: *mut ktime_t) -> u64;
    pub fn drm_crtc_next_vblank_start(crtc: *mut drm_crtc, vblanktime: *mut ktime_t) -> i32;
    pub fn drm_crtc_send_vblank_event(crtc: *mut drm_crtc, e: *mut drm_pending_vblank_event);
    pub fn drm_crtc_arm_vblank_event(crtc: *mut drm_crtc, e: *mut drm_pending_vblank_event);
    pub fn drm_vblank_set_event(e: *mut drm_pending_vblank_event, seq: *mut u64, now: *mut ktime_t);
    pub fn drm_handle_vblank(dev: *mut drm_device, pipe: u32) -> bool;
    pub fn drm_crtc_handle_vblank(crtc: *mut drm_crtc) -> bool;
    pub fn drm_crtc_vblank_get(crtc: *mut drm_crtc) -> i32;
    pub fn drm_crtc_vblank_put(crtc: *mut drm_crtc);
    pub fn drm_crtc_wait_one_vblank(crtc: *mut drm_crtc) -> i32;
    pub fn drm_crtc_vblank_off(crtc: *mut drm_crtc);
    pub fn drm_crtc_vblank_reset(crtc: *mut drm_crtc);
    pub fn drm_crtc_vblank_on_config(crtc: *mut drm_crtc, config: *const drm_vblank_crtc_config);
    pub fn drm_crtc_vblank_on(crtc: *mut drm_crtc);
    pub fn drm_crtc_accurate_vblank_count(crtc: *mut drm_crtc) -> u64;
    pub fn drm_crtc_vblank_restore(crtc: *mut drm_crtc);
    pub fn drm_calc_timestamping_constants(crtc: *mut drm_crtc, mode: *const drm_display_mode);
    pub fn drm_crtc_vblank_waitqueue(crtc: *mut drm_crtc) -> *mut wait_queue_head_t;
    pub fn drm_crtc_set_max_vblank_count(crtc: *mut drm_crtc, max_vblank_count: u32);
    pub fn drm_crtc_vblank_start_timer(crtc: *mut drm_crtc) -> i32;
    pub fn drm_crtc_vblank_cancel_timer(crtc: *mut drm_crtc);
    pub fn drm_crtc_vblank_get_vblank_timeout(crtc: *mut drm_crtc, vblank_time: *mut ktime_t);
    pub fn drm_crtc_vblank_helper_get_vblank_timestamp_internal(
        crtc: *mut drm_crtc,
        max_error: *mut i32,
        vblank_time: *mut ktime_t,
        in_vblank_irq: bool,
        get_scanout_position: drm_vblank_get_scanout_position_func,
    ) -> bool;
    pub fn drm_crtc_vblank_helper_get_vblank_timestamp(
        crtc: *mut drm_crtc,
        max_error: *mut i32,
        vblank_time: *mut ktime_t,
        in_vblank_irq: bool,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
