// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Handle async block request by crypto hardware engine.
 *
 * Copyright (C) 2016 Linaro, Inc.
 *
 * Author: Baolin Wang <baolin.wang@linaro.org>
 */

// Kernel crypto, device, scheduling, and local internal declarations are
// supplied by the surrounding build.

const CRYPTO_ENGINE_MAX_QLEN: i32 = 10;

#[repr(C)]
pub struct crypto_engine_alg {
    pub base: crypto_alg,
    pub op: crypto_engine_op,
}

unsafe fn crypto_finalize_request(engine: *mut crypto_engine,
                                  req: *mut crypto_async_request,
                                  err: i32) {
    let mut flags: c_ulong = 0;

    if !(*engine).retry_support {
        spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
        if (*engine).cur_req == req {
            (*engine).cur_req = core::ptr::null_mut();
        }
        spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
    }

    lockdep_assert_in_softirq();
    crypto_request_complete(req, err);
    kthread_queue_work((*engine).kworker, &mut (*engine).pump_requests);
}

unsafe fn crypto_pump_requests(engine: *mut crypto_engine, in_kthread: bool) {
    let (mut async_req, mut backlog): (*mut crypto_async_request, *mut crypto_async_request);
    let (mut alg, mut op): (*mut crypto_engine_alg, *mut crypto_engine_op);
    let mut flags: c_ulong = 0;
    let mut ret: i32;

    spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
    if !(*engine).retry_support && !(*engine).cur_req.is_null() { goto_out!(); }
    if crypto_queue_len(&mut (*engine).queue) == 0 || !(*engine).running {
        if !(*engine).busy { goto_out!(); }
        if !in_kthread {
            kthread_queue_work((*engine).kworker, &mut (*engine).pump_requests);
            goto_out!();
        }
        (*engine).busy = false;
        goto_out!();
    }

    loop {
        backlog = crypto_get_backlog(&mut (*engine).queue);
        async_req = crypto_dequeue_request(&mut (*engine).queue);
        if async_req.is_null() { break; }
        if !(*engine).retry_support { (*engine).cur_req = async_req; }
        if !(*engine).busy { (*engine).busy = true; }
        spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);

        alg = container_of_crypto_engine_alg((*async_req).tfm);
        op = &mut (*alg).op;
        ret = ((*op).do_one_request)(engine, async_req);
        if ret < 0 {
            if !(*engine).retry_support || ret != -ENOSPC {
                dev_err((*engine).dev, "Failed to do one request from queue: %d\n", ret);
                crypto_request_complete(async_req, ret);
            } else {
                spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
                crypto_enqueue_request_head(&mut (*engine).queue, async_req);
                kthread_queue_work((*engine).kworker, &mut (*engine).pump_requests);
                spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
                return;
            }
        }
        if !backlog.is_null() { crypto_request_complete(backlog, -EINPROGRESS); }
        if !(*engine).retry_support { return; }
        spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
    }

    spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
}

unsafe fn crypto_pump_work(work: *mut kthread_work) {
    let engine = container_of_crypto_engine(work);
    crypto_pump_requests(engine, true);
}

unsafe fn crypto_transfer_request(engine: *mut crypto_engine,
                                  req: *mut crypto_async_request,
                                  need_pump: bool) -> i32 {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
    if !(*engine).running {
        spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
        return -ESHUTDOWN;
    }
    let ret = crypto_enqueue_request(&mut (*engine).queue, req);
    if !(*engine).busy && need_pump {
        kthread_queue_work((*engine).kworker, &mut (*engine).pump_requests);
    }
    spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
    ret
}

unsafe fn crypto_transfer_request_to_engine(engine: *mut crypto_engine,
                                            req: *mut crypto_async_request) -> i32 {
    crypto_transfer_request(engine, req, true)
}

pub unsafe fn crypto_transfer_aead_request_to_engine(engine: *mut crypto_engine, req: *mut aead_request) -> i32 { crypto_transfer_request_to_engine(engine, &mut (*req).base) }
pub unsafe fn crypto_transfer_akcipher_request_to_engine(engine: *mut crypto_engine, req: *mut akcipher_request) -> i32 { crypto_transfer_request_to_engine(engine, &mut (*req).base) }
pub unsafe fn crypto_transfer_hash_request_to_engine(engine: *mut crypto_engine, req: *mut ahash_request) -> i32 { crypto_transfer_request_to_engine(engine, &mut (*req).base) }
pub unsafe fn crypto_transfer_kpp_request_to_engine(engine: *mut crypto_engine, req: *mut kpp_request) -> i32 { crypto_transfer_request_to_engine(engine, &mut (*req).base) }
pub unsafe fn crypto_transfer_skcipher_request_to_engine(engine: *mut crypto_engine, req: *mut skcipher_request) -> i32 { crypto_transfer_request_to_engine(engine, &mut (*req).base) }

pub unsafe fn crypto_finalize_aead_request(e: *mut crypto_engine, r: *mut aead_request, err: i32) { crypto_finalize_request(e, &mut (*r).base, err); }
pub unsafe fn crypto_finalize_akcipher_request(e: *mut crypto_engine, r: *mut akcipher_request, err: i32) { crypto_finalize_request(e, &mut (*r).base, err); }
pub unsafe fn crypto_finalize_hash_request(e: *mut crypto_engine, r: *mut ahash_request, err: i32) { crypto_finalize_request(e, &mut (*r).base, err); }
pub unsafe fn crypto_finalize_kpp_request(e: *mut crypto_engine, r: *mut kpp_request, err: i32) { crypto_finalize_request(e, &mut (*r).base, err); }
pub unsafe fn crypto_finalize_skcipher_request(e: *mut crypto_engine, r: *mut skcipher_request, err: i32) { crypto_finalize_request(e, &mut (*r).base, err); }

pub unsafe fn crypto_engine_start(engine: *mut crypto_engine) -> i32 {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
    if (*engine).running || (*engine).busy {
        spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
        return -EBUSY;
    }
    (*engine).running = true;
    spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
    kthread_queue_work((*engine).kworker, &mut (*engine).pump_requests);
    0
}

pub unsafe fn crypto_engine_stop(engine: *mut crypto_engine) -> i32 {
    let mut flags: c_ulong = 0;
    let mut limit: u32 = 500;
    spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
    while (crypto_queue_len(&mut (*engine).queue) != 0 || (*engine).busy) && limit != 0 {
        limit -= 1;
        spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
        msleep(20);
        spin_lock_irqsave(&mut (*engine).queue_lock, &mut flags);
    }
    let ret = if crypto_queue_len(&mut (*engine).queue) != 0 || (*engine).busy { -EBUSY } else { (*engine).running = false; 0 };
    spin_unlock_irqrestore(&mut (*engine).queue_lock, flags);
    if ret != 0 { dev_warn((*engine).dev, "could not stop engine\n"); }
    ret
}

pub unsafe fn crypto_engine_alloc_init_and_set(dev: *mut device, retry_support: bool, rt: bool, qlen: i32) -> *mut crypto_engine {
    if dev.is_null() { return core::ptr::null_mut(); }
    let engine = devm_kzalloc(dev, core::mem::size_of::<crypto_engine>(), GFP_KERNEL);
    if engine.is_null() { return core::ptr::null_mut(); }
    (*engine).dev = dev; (*engine).rt = rt; (*engine).running = false; (*engine).busy = false;
    (*engine).retry_support = retry_support; (*engine).priv_data = dev;
    snprintf((*engine).name.as_mut_ptr(), (*engine).name.len(), "%s-engine", dev_name(dev));
    spinlock_init(&mut (*engine).queue_lock);
    crypto_init_queue(&mut (*engine).queue, qlen);
    (*engine).kworker = kthread_run_worker(0, "%s", (*engine).name.as_ptr());
    if is_err((*engine).kworker) { dev_err(dev, "failed to create crypto request pump task\n"); return core::ptr::null_mut(); }
    kthread_init_work(&mut (*engine).pump_requests, crypto_pump_work);
    if (*engine).rt { dev_info(dev, "will run requests pump with realtime priority\n"); sched_set_fifo((*engine).kworker); }
    engine
}

pub unsafe fn crypto_engine_alloc_init(dev: *mut device, rt: bool) -> *mut crypto_engine { crypto_engine_alloc_init_and_set(dev, false, rt, CRYPTO_ENGINE_MAX_QLEN) }

pub unsafe fn crypto_engine_exit(engine: *mut crypto_engine) { if crypto_engine_stop(engine) == 0 { kthread_destroy_worker((*engine).kworker); } }

pub unsafe fn crypto_engine_register_aead(alg: *mut aead_engine_alg) -> i32 { if (*alg).op.do_one_request.is_none() { -EINVAL } else { crypto_register_aead(&mut (*alg).base) } }
pub unsafe fn crypto_engine_unregister_aead(alg: *mut aead_engine_alg) { crypto_unregister_aead(&mut (*alg).base); }
pub unsafe fn crypto_engine_register_ahash(alg: *mut ahash_engine_alg) -> i32 { if (*alg).op.do_one_request.is_none() { -EINVAL } else { crypto_register_ahash(&mut (*alg).base) } }
pub unsafe fn crypto_engine_unregister_ahash(alg: *mut ahash_engine_alg) { crypto_unregister_ahash(&mut (*alg).base); }
pub unsafe fn crypto_engine_register_akcipher(alg: *mut akcipher_engine_alg) -> i32 { if (*alg).op.do_one_request.is_none() { -EINVAL } else { crypto_register_akcipher(&mut (*alg).base) } }
pub unsafe fn crypto_engine_unregister_akcipher(alg: *mut akcipher_engine_alg) { crypto_unregister_akcipher(&mut (*alg).base); }
pub unsafe fn crypto_engine_register_kpp(alg: *mut kpp_engine_alg) -> i32 { if (*alg).op.do_one_request.is_none() { -EINVAL } else { crypto_register_kpp(&mut (*alg).base) } }
pub unsafe fn crypto_engine_unregister_kpp(alg: *mut kpp_engine_alg) { crypto_unregister_kpp(&mut (*alg).base); }
pub unsafe fn crypto_engine_register_skcipher(alg: *mut skcipher_engine_alg) -> i32 { if (*alg).op.do_one_request.is_none() { -EINVAL } else { crypto_register_skcipher(&mut (*alg).base) } }
pub unsafe fn crypto_engine_unregister_skcipher(alg: *mut skcipher_engine_alg) { crypto_unregister_skcipher(&mut (*alg).base); }

// The register-many helpers preserve the C loop and rollback behavior.
macro_rules! register_many { ($name:ident, $one:ident, $undo:ident, $ty:ty) => {
    pub unsafe fn $name(algs: *mut $ty, count: i32) -> i32 { let mut i = 0; while i < count { let ret = $one(algs.add(i as usize)); if ret != 0 { $undo(algs, i); return ret; } i += 1; } 0 }
} }
macro_rules! unregister_many { ($name:ident, $one:ident, $ty:ty) => {
    pub unsafe fn $name(algs: *mut $ty, count: i32) { let mut i = count; while i > 0 { i -= 1; $one(algs.add(i as usize)); } }
} }
register_many!(crypto_engine_register_aeads, crypto_engine_register_aead, crypto_engine_unregister_aeads, aead_engine_alg);
unregister_many!(crypto_engine_unregister_aeads, crypto_engine_unregister_aead, aead_engine_alg);
register_many!(crypto_engine_register_ahashes, crypto_engine_register_ahash, crypto_engine_unregister_ahashes, ahash_engine_alg);
unregister_many!(crypto_engine_unregister_ahashes, crypto_engine_unregister_ahash, ahash_engine_alg);
register_many!(crypto_engine_register_skciphers, crypto_engine_register_skcipher, crypto_engine_unregister_skciphers, skcipher_engine_alg);
unregister_many!(crypto_engine_unregister_skciphers, crypto_engine_unregister_skcipher, skcipher_engine_alg);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
