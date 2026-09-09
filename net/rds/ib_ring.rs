/*
 * Copyright (c) 2006 Oracle.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Dependencies supplied by the kernel/RDS headers are intentionally external.

extern "C" {
    static mut rds_ib_ring_empty_wait: rds_wait_queue_head;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn atomic_read(v: *const rds_atomic_t) -> i32;
    fn atomic_add(i: u32, v: *mut rds_atomic_t);
    fn waitqueue_active(wq: *mut rds_wait_queue_head) -> bool;
    fn wake_up(wq: *mut rds_wait_queue_head);
    fn rdsdebug(fmt: *const core::ffi::c_char, ...);
    fn rds_bug_on(condition: bool);
}

// Types declared by the included RDS headers.
#[repr(C)]
pub struct rds_ib_work_ring {
    pub w_nr: u32,
    pub w_alloc_ptr: u32,
    pub w_free_ptr: u32,
    pub w_alloc_ctr: u32,
    pub w_free_ctr: rds_atomic_t,
}
pub struct rds_atomic_t;
pub struct rds_wait_queue_head;

// Locking for IB rings follows the locking contract documented in ib_ring.c.

pub unsafe fn rds_ib_ring_init(ring: *mut rds_ib_work_ring, nr: u32) {
    memset(ring.cast(), 0, core::mem::size_of::<rds_ib_work_ring>());
    (*ring).w_nr = nr;
    rdsdebug(b"ring %p nr %u\0".as_ptr().cast(), ring, (*ring).w_nr);
}

#[inline]
unsafe fn __rds_ib_ring_used(ring: *mut rds_ib_work_ring) -> u32 {
    // This assumes that atomic_t has at least as many bits as u32.
    let diff = (*ring).w_alloc_ctr.wrapping_sub(atomic_read(&(*ring).w_free_ctr) as u32);
    rds_bug_on(diff > (*ring).w_nr);
    diff
}

pub unsafe fn rds_ib_ring_resize(ring: *mut rds_ib_work_ring, nr: u32) {
    // We only ever get called from the connection setup code, prior to creating the QP.
    rds_bug_on(__rds_ib_ring_used(ring) != 0);
    (*ring).w_nr = nr;
}

unsafe fn __rds_ib_ring_empty(ring: *mut rds_ib_work_ring) -> bool {
    __rds_ib_ring_used(ring) == 0
}

pub unsafe fn rds_ib_ring_alloc(ring: *mut rds_ib_work_ring, val: u32, pos: *mut u32) -> u32 {
    let mut ret = 0u32;
    let avail = (*ring).w_nr - __rds_ib_ring_used(ring);

    rdsdebug(b"ring %p val %u next %u free %u\0".as_ptr().cast(), ring, val,
             (*ring).w_alloc_ptr, avail);

    if val != 0 && avail != 0 {
        ret = core::cmp::min(val, avail);
        *pos = (*ring).w_alloc_ptr;
        (*ring).w_alloc_ptr = ((*ring).w_alloc_ptr + ret) % (*ring).w_nr;
        (*ring).w_alloc_ctr = (*ring).w_alloc_ctr.wrapping_add(ret);
    }
    ret
}

pub unsafe fn rds_ib_ring_free(ring: *mut rds_ib_work_ring, val: u32) {
    (*ring).w_free_ptr = ((*ring).w_free_ptr + val) % (*ring).w_nr;
    atomic_add(val, &mut (*ring).w_free_ctr);
    if __rds_ib_ring_empty(ring) && waitqueue_active(&mut rds_ib_ring_empty_wait) {
        wake_up(&mut rds_ib_ring_empty_wait);
    }
}

pub unsafe fn rds_ib_ring_unalloc(ring: *mut rds_ib_work_ring, val: u32) {
    (*ring).w_alloc_ptr = ((*ring).w_alloc_ptr - val) % (*ring).w_nr;
    (*ring).w_alloc_ctr = (*ring).w_alloc_ctr.wrapping_sub(val);
}

pub unsafe fn rds_ib_ring_empty(ring: *mut rds_ib_work_ring) -> i32 {
    __rds_ib_ring_empty(ring) as i32
}

pub unsafe fn rds_ib_ring_low(ring: *mut rds_ib_work_ring) -> i32 {
    (__rds_ib_ring_used(ring) <= ((*ring).w_nr >> 1)) as i32
}

// Returns the oldest allocated ring entry. This will be the next one freed.
pub unsafe fn rds_ib_ring_oldest(ring: *mut rds_ib_work_ring) -> u32 {
    (*ring).w_free_ptr
}

// Returns the number of completed work requests.
pub unsafe fn rds_ib_ring_completed(ring: *mut rds_ib_work_ring, wr_id: u32, oldest: u32) -> u32 {
    let ret;
    if oldest <= wr_id {
        ret = wr_id - oldest + 1;
    } else {
        ret = (*ring).w_nr - oldest + wr_id + 1;
    }
    rdsdebug(b"ring %p ret %u wr_id %u oldest %u\0".as_ptr().cast(), ring, ret, wr_id, oldest);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
