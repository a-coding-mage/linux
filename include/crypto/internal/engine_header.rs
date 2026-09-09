/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Crypto engine API
 *
 * Copyright (c) 2016 Baolin Wang <baolin.wang@linaro.org>
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the corresponding kernel crypto and Linux headers:
// crypto/algapi.h, crypto/engine.h, linux/kthread.h,
// linux/spinlock_types.h, and linux/types.h.

pub const ENGINE_NAME_LEN: usize = 30;

// struct device;

/*
 * struct crypto_engine - crypto hardware engine
 * @name: the engine name
 * @busy: request pump is busy
 * @running: the engine is on working
 * @retry_support: indication that the hardware allows re-execution
 * of a failed backlog request
 * crypto-engine, in head position to keep order
 * @rt: whether this queue is set to run as a realtime task
 * @list: link with the global crypto engine list
 * @queue_lock: spinlock to synchronise access to request queue
 * @queue: the crypto queue of the engine
 * @kworker: kthread worker struct for request pump
 * @pump_requests: work struct for scheduling work to the request pump
 * @priv_data: the engine private data
 * @cur_req: the current request which is on processing
 */
#[repr(C)]
pub struct crypto_engine {
    pub name: [core::ffi::c_char; ENGINE_NAME_LEN],
    pub busy: bool,
    pub running: bool,

    pub retry_support: bool,
    pub rt: bool,

    pub list: list_head,
    pub queue_lock: spinlock_t,
    pub queue: crypto_queue,
    pub dev: *mut device,

    pub kworker: *mut kthread_worker,
    pub pump_requests: kthread_work,

    pub priv_data: *mut core::ffi::c_void,
    pub cur_req: *mut crypto_async_request,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
