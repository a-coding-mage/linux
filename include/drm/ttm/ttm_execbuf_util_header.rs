/**************************************************************************
 *
 * Copyright (c) 2006-2009 VMware, Inc., Palo Alto, CA., USA
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 **************************************************************************/
/*
 * Authors: Thomas Hellstrom <thellstrom-at-vmware-dot-com>
 */

// External declarations corresponding to Linux list and synchronization types.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub enum ww_acquire_ctx {}
pub enum dma_fence {}
pub enum ttm_buffer_object {}

/**
 * struct ttm_validate_buffer
 *
 * @head:           list head for thread-private list.
 * @bo:             refcounted buffer object pointer.
 * @num_shared:     How many shared fences we want to add.
 */
#[repr(C)]
pub struct ttm_validate_buffer {
    pub head: list_head,
    pub bo: *mut ttm_buffer_object,
    pub num_shared: ::core::ffi::c_uint,
}

/**
 * ttm_eu_backoff_reservation
 *
 * @ticket:   ww_acquire_ctx from reserve call
 * @list:     thread private list of ttm_validate_buffer structs.
 *
 * Undoes all buffer validation reservations for bos pointed to by
 * the list entries.
 */
extern "C" {
    pub fn ttm_eu_backoff_reservation(
        ticket: *mut ww_acquire_ctx,
        list: *mut list_head,
    );

    /**
     * ttm_eu_reserve_buffers
     *
     * @ticket:  [out] ww_acquire_ctx filled in by call, or NULL if only
     *           non-blocking reserves should be tried.
     * @list:    thread private list of ttm_validate_buffer structs.
     * @intr:    should the wait be interruptible
     * @dups:    [out] optional list of duplicates.
     *
     * Tries to reserve bos pointed to by the list entries.
     * If the function returns 0, all buffers are marked as "unfenced",
     * taken off the lru lists and are not synced for write CPU usage.
     *
     * If the function detects a deadlock due to multiple threads trying to
     * reserve the same buffers in reverse order, all threads except one will
     * back off and retry. This function may sleep while waiting for
     * CPU write reservations to be cleared, and for other threads to
     * unreserve their buffers.
     *
     * If intr is set to true, this function may return -ERESTARTSYS if the
     * calling process receives a signal while waiting. In that case, no
     * buffers on the list will be reserved upon return.
     *
     * If dups is non NULL all buffers already reserved by the current thread
     * (e.g. duplicates) are added to this list, otherwise -EALREADY is returned
     * on the first already reserved buffer and all buffers from the list are
     * unreserved again.
     *
     * Buffers reserved by this function should be unreserved by
     * a call to either ttm_eu_backoff_reservation() or
     * ttm_eu_fence_buffer_objects() when command submission is complete or
     * has failed.
     */
    pub fn ttm_eu_reserve_buffers(
        ticket: *mut ww_acquire_ctx,
        list: *mut list_head,
        intr: bool,
        dups: *mut list_head,
    ) -> ::core::ffi::c_int;

    /**
     * ttm_eu_fence_buffer_objects
     *
     * @ticket:      ww_acquire_ctx from reserve call
     * @list:        thread private list of ttm_validate_buffer structs.
     * @fence:       The new exclusive fence for the buffers.
     *
     * This function should be called when command submission is complete, and
     * it will add a new sync object to bos pointed to by entries on @list.
     * It also unreserves all buffers, putting them on lru lists.
     *
     */
    pub fn ttm_eu_fence_buffer_objects(
        ticket: *mut ww_acquire_ctx,
        list: *mut list_head,
        fence: *mut dma_fence,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
