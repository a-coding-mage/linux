/*
 * Copyright (C) 2013 Red Hat
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation.

/**
 * DOC: flip utils
 *
 * Utility to queue up work to run from work-queue context after flip/vblank.
 * Typically this can be used to defer unref of framebuffer's, cursor
 * bo's, etc until after vblank. The APIs are all thread-safe. Moreover,
 * drm_flip_work_commit() can be called in atomic context.
 */

/*
 * drm_flip_func_t - callback function
 *
 * @work: the flip work
 * @val: value queued via drm_flip_work_queue()
 *
 * Callback function to be called for each of the  queue'd work items after
 * drm_flip_work_commit() is called.
 */
pub type drm_flip_func_t = unsafe extern "C" fn(work: *mut drm_flip_work, val: *mut core::ffi::c_void);

/**
 * struct drm_flip_work - flip work queue
 * @name: debug name
 * @func: callback fxn called for each committed item
 * @worker: worker which calls @func
 * @queued: queued tasks
 * @commited: commited tasks
 * @lock: lock to access queued and commited lists
 */
#[repr(C)]
pub struct drm_flip_work {
    pub name: *const core::ffi::c_char,
    pub func: drm_flip_func_t,
    pub worker: work_struct,
    pub queued: list_head,
    pub commited: list_head,
    pub lock: spinlock_t,
}

unsafe extern "C" {
    pub fn drm_flip_work_queue(work: *mut drm_flip_work, val: *mut core::ffi::c_void);
    pub fn drm_flip_work_commit(work: *mut drm_flip_work, wq: *mut workqueue_struct);
    pub fn drm_flip_work_init(
        work: *mut drm_flip_work,
        name: *const core::ffi::c_char,
        func: drm_flip_func_t,
    );
    pub fn drm_flip_work_cleanup(work: *mut drm_flip_work);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
