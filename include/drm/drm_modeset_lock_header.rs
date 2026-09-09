/*
 * Copyright (C) 2014 Red Hat
 * Author: Rob Clark <robdclark@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: ww_acquire_ctx, ww_mutex, list_head, depot_stack_handle_t,
// WARN_ON, list_empty, ww_mutex_is_locked, lockdep_assert_held, mutex_lock,
// mutex_unlock, drm_drv_uses_atomic_modeset, BIT, and EDEADLK.

pub struct drm_modeset_lock;

/**
 * struct drm_modeset_acquire_ctx - locking context (see ww_acquire_ctx)
 * @ww_ctx: base acquire ctx
 * @contended: used internally for -EDEADLK handling
 * @stack_depot: used internally for contention debugging
 * @locked: list of held locks
 * @trylock_only: trylock mode used in atomic contexts/panic notifiers
 * @interruptible: whether interruptible locking should be used.
 *
 * Each thread competing for a set of locks must use one acquire
 * ctx.  And if any lock fxn returns -EDEADLK, it must backoff and
 * retry.
 */
#[repr(C)]
pub struct drm_modeset_acquire_ctx {
    pub ww_ctx: ww_acquire_ctx,
    pub contended: *mut drm_modeset_lock,
    pub stack_depot: depot_stack_handle_t,
    pub locked: list_head,
    pub trylock_only: bool,
    pub interruptible: bool,
}

/**
 * struct drm_modeset_lock - used for locking modeset resources.
 * @mutex: resource locking
 * @head: used to hold its place on &drm_atomi_state.locked list when
 *    part of an atomic update
 *
 * Used for locking CRTCs and other modeset resources.
 */
#[repr(C)]
pub struct drm_modeset_lock {
    pub mutex: ww_mutex,
    pub head: list_head,
}

pub const DRM_MODESET_ACQUIRE_INTERRUPTIBLE: u32 = BIT(0);

extern "C" {
    pub fn drm_modeset_acquire_init(ctx: *mut drm_modeset_acquire_ctx, flags: u32);
    pub fn drm_modeset_acquire_fini(ctx: *mut drm_modeset_acquire_ctx);
    pub fn drm_modeset_drop_locks(ctx: *mut drm_modeset_acquire_ctx);
    pub fn drm_modeset_backoff(ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_modeset_lock_init(lock: *mut drm_modeset_lock);
}

/** drm_modeset_lock_fini - cleanup lock
 * @lock: lock to cleanup
 */
#[inline]
pub unsafe fn drm_modeset_lock_fini(lock: *mut drm_modeset_lock) {
    WARN_ON(!list_empty(&(*lock).head));
}

/** drm_modeset_is_locked - equivalent to mutex_is_locked()
 * @lock: lock to check
 */
#[inline]
pub unsafe fn drm_modeset_is_locked(lock: *mut drm_modeset_lock) -> bool {
    ww_mutex_is_locked(&(*lock).mutex)
}

/** drm_modeset_lock_assert_held - equivalent to lockdep_assert_held()
 * @lock: lock to check
 */
#[inline]
pub unsafe fn drm_modeset_lock_assert_held(lock: *mut drm_modeset_lock) {
    lockdep_assert_held(&(*lock).mutex.base);
}

extern "C" {
    pub fn drm_modeset_lock(lock: *mut drm_modeset_lock, ctx: *mut drm_modeset_acquire_ctx) -> i32;
    pub fn drm_modeset_lock_single_interruptible(lock: *mut drm_modeset_lock) -> i32;
    pub fn drm_modeset_unlock(lock: *mut drm_modeset_lock);
}

pub struct drm_device;
pub struct drm_crtc;
pub struct drm_plane;

extern "C" {
    pub fn drm_modeset_lock_all(dev: *mut drm_device);
    pub fn drm_modeset_unlock_all(dev: *mut drm_device);
    pub fn drm_warn_on_modeset_not_all_locked(dev: *mut drm_device);
    pub fn drm_modeset_lock_all_ctx(dev: *mut drm_device, ctx: *mut drm_modeset_acquire_ctx) -> i32;
}

// Helper to acquire modeset locks using a local context. The C macro's labels
// and caller-provided ret variable are preserved as a Rust macro interface.
#[macro_export]
macro_rules! DRM_MODESET_LOCK_ALL_BEGIN {
    ($dev:expr, $ctx:expr, $flags:expr, $ret:expr) => {
        if !drm_drv_uses_atomic_modeset($dev) {
            mutex_lock(&(*$dev).mode_config.mutex);
        }
        drm_modeset_acquire_init(&mut $ctx, $flags);
        'modeset_lock_retry: loop {
            $ret = drm_modeset_lock_all_ctx($dev, &mut $ctx);
            if $ret != 0 {
                break 'modeset_lock_retry;
            }
            break;
        }
    };
}

// Helper to release and clean up modeset locks. The caller must provide the
// surrounding retry/failure control flow when translating the C macro use.
#[macro_export]
macro_rules! DRM_MODESET_LOCK_ALL_END {
    ($dev:expr, $ctx:expr, $ret:expr) => {
        if $ret == -EDEADLK {
            $ret = drm_modeset_backoff(&mut $ctx);
            if $ret == 0 {
                continue;
            }
        }
        drm_modeset_drop_locks(&mut $ctx);
        drm_modeset_acquire_fini(&mut $ctx);
        if !drm_drv_uses_atomic_modeset($dev) {
            mutex_unlock(&(*$dev).mode_config.mutex);
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
