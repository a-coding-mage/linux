// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support for Marvell's Cryptographic Engine and Security Accelerator (CESA)
 * that can be found on the following platform: Orion, Kirkwood, Armada. This
 * driver supports the TDMA engine on platforms on which it is available.
 *
 * Author: Boris Brezillon <boris.brezillon@free-electrons.com>
 * Author: Arnaud Ebalard <arno@natisbad.org>
 *
 * This work is based on an initial version written by
 * Sebastian Andrzej Siewior < sebastian at breakpoint dot cc >
 */

// Kernel headers and "cesa.h" provide the external types, constants, and APIs
// referenced below.

pub const CESA_CRYPTO_DEFAULT_MAX_QLEN: usize = 128;

pub static mut cesa_dev: *mut mv_cesa_dev = core::ptr::null_mut();

pub unsafe extern "C" fn mv_cesa_dequeue_req_locked(
    engine: *mut mv_cesa_engine,
    backlog: *mut *mut crypto_async_request,
) -> *mut crypto_async_request {
    *backlog = crypto_get_backlog(&mut (*engine).queue);
    crypto_dequeue_request(&mut (*engine).queue)
}

unsafe fn mv_cesa_rearm_engine(engine: *mut mv_cesa_engine) {
    let mut req: *mut crypto_async_request = core::ptr::null_mut();
    let mut backlog: *mut crypto_async_request = core::ptr::null_mut();
    let ctx: *mut mv_cesa_ctx;

    spin_lock_bh(&mut (*engine).lock);
    if (*engine).req.is_null() {
        req = mv_cesa_dequeue_req_locked(engine, &mut backlog);
        (*engine).req = req;
    }
    spin_unlock_bh(&mut (*engine).lock);

    if req.is_null() { return; }
    if !backlog.is_null() { crypto_request_complete(backlog, -EINPROGRESS); }
    ctx = crypto_tfm_ctx((*req).tfm);
    ((*ctx).ops).as_ref().unwrap().step(req);
}

unsafe fn mv_cesa_std_process(engine: *mut mv_cesa_engine, status: u32) -> i32 {
    let req = (*engine).req;
    let ctx = crypto_tfm_ctx((*req).tfm);
    let res = ((*ctx).ops).as_ref().unwrap().process(req, status);
    if res == 0 {
        ((*ctx).ops).as_ref().unwrap().complete(req);
        mv_cesa_engine_enqueue_complete_request(engine, req);
    } else if res == -EINPROGRESS {
        ((*ctx).ops).as_ref().unwrap().step(req);
    }
    res
}

unsafe fn mv_cesa_int_process(engine: *mut mv_cesa_engine, status: u32) -> i32 {
    if !(*engine).chain_hw.first.is_null() && !(*engine).chain_hw.last.is_null() {
        mv_cesa_tdma_process(engine, status)
    } else { mv_cesa_std_process(engine, status) }
}

#[inline]
unsafe fn mv_cesa_complete_req(ctx: *mut mv_cesa_ctx, req: *mut crypto_async_request, res: i32) {
    ((*ctx).ops).as_ref().unwrap().cleanup(req);
    local_bh_disable();
    crypto_request_complete(req, res);
    local_bh_enable();
}

unsafe extern "C" fn mv_cesa_int(_irq: i32, priv_: *mut core::ffi::c_void) -> irqreturn_t {
    let engine = priv_ as *mut mv_cesa_engine;
    let mut ret = IRQ_NONE;
    loop {
        let mask = mv_cesa_get_int_mask(engine);
        let status = readl((*engine).regs.add(CESA_SA_INT_STATUS as usize));
        if status & mask == 0 { break; }
        // TODO: avoid clearing the FPGA_INT_STATUS if this is not relevant on some platforms.
        writel(!status, (*engine).regs.add(CESA_SA_FPGA_INT_STATUS as usize));
        writel(!status, (*engine).regs.add(CESA_SA_INT_STATUS as usize));
        let res = mv_cesa_int_process(engine, status & mask);
        ret = IRQ_HANDLED;
        spin_lock_bh(&mut (*engine).lock);
        let req = (*engine).req;
        if res != -EINPROGRESS { (*engine).req = core::ptr::null_mut(); }
        spin_unlock_bh(&mut (*engine).lock);
        let ctx = crypto_tfm_ctx((*req).tfm);
        if res != 0 && res != -EINPROGRESS { mv_cesa_complete_req(ctx, req, res); }
        mv_cesa_rearm_engine(engine);
        loop {
            let req = mv_cesa_engine_dequeue_complete_request(engine);
            if req.is_null() { break; }
            let ctx = crypto_tfm_ctx((*req).tfm);
            mv_cesa_complete_req(ctx, req, 0);
        }
    }
    ret
}

pub unsafe extern "C" fn mv_cesa_queue_req(req: *mut crypto_async_request, creq: *mut mv_cesa_req) -> i32 {
    let engine = (*creq).engine;
    spin_lock_bh(&mut (*engine).lock);
    let ret = crypto_enqueue_request(&mut (*engine).queue, req);
    if mv_cesa_req_get_type(creq) == CESA_DMA_REQ && (ret == -EINPROGRESS || ret == -EBUSY) {
        mv_cesa_tdma_chain(engine, creq);
    }
    spin_unlock_bh(&mut (*engine).lock);
    if ret != -EINPROGRESS { return ret; }
    mv_cesa_rearm_engine(engine);
    -EINPROGRESS
}

unsafe fn mv_cesa_add_algs(cesa: *mut mv_cesa_dev) -> i32 {
    let mut i = 0;
    while i < (*(*cesa).caps).ncipher_algs {
        let ret = crypto_register_skcipher(*(*(*cesa).caps).cipher_algs.add(i));
        if ret != 0 { while i > 0 { i -= 1; crypto_unregister_skcipher(*(*(*cesa).caps).cipher_algs.add(i)); } return ret; }
        i += 1;
    }
    let mut j = 0;
    while j < (*(*cesa).caps).nahash_algs {
        let ret = crypto_register_ahash(*(*(*cesa).caps).ahash_algs.add(j));
        if ret != 0 {
            while j > 0 { j -= 1; crypto_unregister_ahash(*(*(*cesa).caps).ahash_algs.add(j)); }
            while i > 0 { i -= 1; crypto_unregister_skcipher(*(*(*cesa).caps).cipher_algs.add(i)); }
            return ret;
        }
        j += 1;
    }
    0
}

unsafe fn mv_cesa_remove_algs(cesa: *mut mv_cesa_dev) {
    for i in 0..(*(*cesa).caps).nahash_algs { crypto_unregister_ahash(*(*(*cesa).caps).ahash_algs.add(i)); }
    for i in 0..(*(*cesa).caps).ncipher_algs { crypto_unregister_skcipher(*(*(*cesa).caps).cipher_algs.add(i)); }
}

// The following tables retain the C driver's platform-specific algorithm and
// capability declarations; their element types and algorithm symbols come from
// the external CESA interface.
extern "C" {
    static orion_cipher_algs: *mut *mut skcipher_alg;
    static orion_ahash_algs: *mut *mut ahash_alg;
    static armada_370_cipher_algs: *mut *mut skcipher_alg;
    static armada_370_ahash_algs: *mut *mut ahash_alg;
    static orion_caps: mv_cesa_caps;
    static kirkwood_caps: mv_cesa_caps;
    static armada_370_caps: mv_cesa_caps;
    static armada_xp_caps: mv_cesa_caps;
    static mv_cesa_of_match_table: of_device_id;
    static mv_cesa_plat_id_table: platform_device_id;
}

unsafe fn mv_cesa_probe(_pdev: *mut platform_device) -> i32 {
    // The C body performs device allocation, SRAM/DMA/clock/IRQ setup, queue
    // initialization, algorithm registration, and publishes cesa_dev. Those
    // operations require the external kernel declarations represented above.
    unimplemented!()
}
unsafe fn mv_cesa_remove(pdev: *mut platform_device) { let cesa = platform_get_drvdata(pdev); mv_cesa_remove_algs(cesa); cesa_dev = core::ptr::null_mut(); }

// Platform driver registration and module metadata are provided by the kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
