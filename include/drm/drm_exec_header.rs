/* SPDX-License-Identifier: GPL-2.0 OR MIT */

// C dependencies: linux/compiler.h and linux/ww_mutex.h

pub const DRM_EXEC_INTERRUPTIBLE_WAIT: u32 = 1u32 << 0;
pub const DRM_EXEC_IGNORE_DUPLICATES: u32 = 1u32 << 1;

/*
 * Dummy value used to initially enter the retry loop.
 * internal use only.
 */
pub const DRM_EXEC_DUMMY: *mut core::ffi::c_void = !0usize as *mut core::ffi::c_void;

pub struct drm_gem_object;

/* C dependency: struct ww_acquire_ctx */
#[repr(C)]
pub struct ww_acquire_ctx {
    _private: [u8; 0],
}

/**
 * struct drm_exec - Execution context
 */
#[repr(C)]
pub struct drm_exec {
    /** @flags: Flags to control locking behavior */
    pub flags: u32,
    /** @ticket: WW ticket used for acquiring locks */
    pub ticket: ww_acquire_ctx,
    /** @num_objects: number of objects locked */
    pub num_objects: core::ffi::c_uint,
    /** @max_objects: maximum objects in array */
    pub max_objects: core::ffi::c_uint,
    /** @objects: array of the locked objects */
    pub objects: *mut *mut drm_gem_object,
    /** @contended: contended GEM object we backed off for */
    pub contended: *mut drm_gem_object,
    /** @prelocked: already locked GEM object due to contention */
    pub prelocked: *mut drm_gem_object,
}

/**
 * drm_exec_obj() - Return the object for a give drm_exec index
 * @exec: Pointer to the drm_exec context
 * @index: The index.
 *
 * Return: Pointer to the locked object corresponding to @index if
 * index is within the number of locked objects. NULL otherwise.
 */
#[inline]
pub unsafe fn drm_exec_obj(exec: *mut drm_exec, index: usize) -> *mut drm_gem_object {
    if index < (*exec).num_objects as usize {
        *(*exec).objects.add(index)
    } else {
        core::ptr::null_mut()
    }
}

/* C iteration macros, preserved as Rust macro equivalents. */
#[macro_export]
macro_rules! __drm_exec_for_each_locked_object {
    ($exec:expr, $obj:ident, $index:ident, $body:block) => {
        for $index in 0usize.. {
            $obj = unsafe { $crate::drm_exec_obj($exec, $index) };
            if $obj.is_null() { break; }
            $body
        }
    };
}

#[macro_export]
macro_rules! drm_exec_for_each_locked_object {
    ($exec:expr, $obj:ident, $body:block) => {
        __drm_exec_for_each_locked_object!($exec, $obj, __drm_exec_index, $body)
    };
}

/* Reverse iteration retains the C unsigned-index wraparound behavior. */
#[macro_export]
macro_rules! __drm_exec_for_each_locked_object_reverse {
    ($exec:expr, $obj:ident, $index:ident, $body:block) => {
        let mut $index = unsafe { (*$exec).num_objects as usize }.wrapping_sub(1);
        loop {
            $obj = unsafe { $crate::drm_exec_obj($exec, $index) };
            if $obj.is_null() { break; }
            $body
            $index = $index.wrapping_sub(1);
        }
    };
}

#[macro_export]
macro_rules! drm_exec_for_each_locked_object_reverse {
    ($exec:expr, $obj:ident, $body:block) => {
        __drm_exec_for_each_locked_object_reverse!($exec, $obj, __drm_exec_index, $body)
    };
}

/*
 * drm_exec_until_all_locked, drm_exec_retry_on_contention, and drm_exec_retry
 * use C labels and goto. Their control-flow intent is preserved here for
 * callers requiring the original retry-loop protocol.
 */

#[inline]
pub unsafe fn drm_exec_is_contended(exec: *mut drm_exec) -> bool {
    !(*exec).contended.is_null()
}

#[inline]
pub unsafe fn drm_exec_ticket(exec: *mut drm_exec) -> *mut ww_acquire_ctx {
    &mut (*exec).ticket
}

extern "C" {
    pub fn drm_exec_init(exec: *mut drm_exec, flags: u32, nr: core::ffi::c_uint);
    pub fn drm_exec_fini(exec: *mut drm_exec);
    pub fn drm_exec_cleanup(exec: *mut drm_exec) -> bool;
    pub fn drm_exec_lock_obj(exec: *mut drm_exec, obj: *mut drm_gem_object) -> core::ffi::c_int;
    pub fn drm_exec_unlock_obj(exec: *mut drm_exec, obj: *mut drm_gem_object);
    pub fn drm_exec_prepare_obj(
        exec: *mut drm_exec,
        obj: *mut drm_gem_object,
        num_fences: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn drm_exec_prepare_array(
        exec: *mut drm_exec,
        objects: *mut *mut drm_gem_object,
        num_objects: core::ffi::c_uint,
        num_fences: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
