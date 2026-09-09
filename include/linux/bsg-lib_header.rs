/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  BSG helper library
 *
 *  Copyright (C) 2008   James Smart, Emulex Corporation
 *  Copyright (C) 2011   Red Hat, Inc.  All rights reserved.
 *  Copyright (C) 2011   Mike Christie
 */

/* C dependency: <linux/blkdev.h> */

use core::ffi::{c_char, c_int, c_void};

pub struct bsg_job;
pub struct request;
pub struct device;
pub struct scatterlist;
pub struct request_queue;
pub struct kref;
pub struct bio;
pub struct queue_limits;

/* enum blk_eh_timer_return is supplied by the translated blkdev dependency. */
pub type blk_eh_timer_return = c_int;

pub type bsg_job_fn = unsafe extern "C" fn(*mut bsg_job) -> c_int;
pub type bsg_timeout_fn = unsafe extern "C" fn(*mut request) -> blk_eh_timer_return;

#[repr(C)]
pub struct bsg_buffer {
    pub payload_len: u32,
    pub sg_cnt: c_int,
    pub sg_list: *mut scatterlist,
}

#[repr(C)]
pub struct bsg_job {
    pub dev: *mut device,

    pub kref: kref,

    pub timeout: u32,

    /* Transport/driver specific request/reply structs */
    pub request: *mut c_void,
    pub reply: *mut c_void,

    pub request_len: u32,
    pub reply_len: u32,
    /*
     * On entry : reply_len indicates the buffer size allocated for
     * the reply.
     *
     * Upon completion : the message handler must set reply_len
     *  to indicates the size of the reply to be returned to the
     *  caller.
     */

    /* DMA payloads for the request/response */
    pub request_payload: bsg_buffer,
    pub reply_payload: bsg_buffer,

    pub result: c_int,
    pub reply_payload_rcv_len: u32,

    /* BIDI support */
    pub bidi_rq: *mut request,
    pub bidi_bio: *mut bio,

    pub dd_data: *mut c_void, /* Used for driver-specific storage */
}

unsafe extern "C" {
    pub fn bsg_job_done(
        job: *mut bsg_job,
        result: c_int,
        reply_payload_rcv_len: u32,
    );
    pub fn bsg_setup_queue(
        dev: *mut device,
        name: *const c_char,
        lim: *mut queue_limits,
        job_fn: Option<bsg_job_fn>,
        timeout: Option<bsg_timeout_fn>,
        dd_job_size: c_int,
    ) -> *mut request_queue;
    pub fn bsg_remove_queue(q: *mut request_queue);
    pub fn bsg_job_put(job: *mut bsg_job);
    /* __must_check */
    pub fn bsg_job_get(job: *mut bsg_job) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
