// SPDX-License-Identifier: GPL-2.0+
/*
 * caam - Freescale FSL CAAM support for hw_random
 *
 * Copyright 2011 Freescale Semiconductor, Inc.
 * Copyright 2018-2019, 2023 NXP
 *
 * Based on caamalg.c crypto API driver.
 */

const CAAM_RNG_MAX_FIFO_STORE_SIZE: usize = 16;
const CAAM_RNG_DESC_LEN: usize = CAAM_CMD_SZ + CAAM_CMD_SZ + CAAM_CMD_SZ + CAAM_PTR_SZ_MAX;

#[repr(C)]
struct caam_rng_ctx {
    rng: hwrng,
    jrdev: *mut device,
    ctrldev: *mut device,
    desc_async: *mut core::ffi::c_void,
    desc_sync: *mut core::ffi::c_void,
    worker: work_struct,
    fifo: kfifo,
}

#[repr(C)]
struct caam_rng_job_ctx {
    done: *mut completion,
    err: *mut i32,
}

unsafe fn to_caam_rng_ctx(r: *mut hwrng) -> *mut caam_rng_ctx {
    (*r).priv_ as *mut caam_rng_ctx
}

unsafe extern "C" fn caam_rng_done(
    jrdev: *mut device,
    _desc: *mut u32,
    err: u32,
    context: *mut core::ffi::c_void,
) {
    let jctx = context as *mut caam_rng_job_ctx;
    if err != 0 {
        *(*jctx).err = caam_jr_strstatus(jrdev, err);
    }
    complete((*jctx).done);
}

unsafe fn caam_init_desc(desc: *mut u32, dst_dma: dma_addr_t) -> *mut u32 {
    init_job_desc(desc, 0);
    append_operation(desc, OP_ALG_ALGSEL_RNG | OP_TYPE_CLASS1_ALG | OP_ALG_PR_ON);
    append_fifo_store(desc, dst_dma, CAAM_RNG_MAX_FIFO_STORE_SIZE, FIFOST_TYPE_RNGSTORE);
    print_hex_dump_debug(
        b"rng job desc@: \0".as_ptr() as *const i8,
        DUMP_PREFIX_ADDRESS,
        16,
        4,
        desc as *const core::ffi::c_void,
        desc_bytes(desc),
        1,
    );
    desc
}

unsafe fn caam_rng_read_one(
    jrdev: *mut device,
    dst: *mut core::ffi::c_void,
    _len: i32,
    desc: *mut core::ffi::c_void,
    done: *mut completion,
) -> i32 {
    let len = CAAM_RNG_MAX_FIFO_STORE_SIZE as i32;
    let mut ret = 0i32;
    let mut jctx = caam_rng_job_ctx { done, err: &mut ret };
    let dst_dma = dma_map_single(jrdev, dst, len as usize, DMA_FROM_DEVICE);
    if dma_mapping_error(jrdev, dst_dma) {
        dev_err(jrdev, b"unable to map destination memory\n\0".as_ptr() as *const i8);
        return -ENOMEM;
    }
    init_completion(done);
    let mut err = caam_jr_enqueue(
        jrdev,
        caam_init_desc(desc as *mut u32, dst_dma),
        caam_rng_done,
        &mut jctx as *mut _ as *mut core::ffi::c_void,
    );
    if err == -EINPROGRESS {
        wait_for_completion(done);
        err = 0;
    }
    dma_unmap_single(jrdev, dst_dma, len as usize, DMA_FROM_DEVICE);
    if err != 0 { err } else if ret != 0 { ret } else { len }
}

unsafe fn caam_rng_fill_async(ctx: *mut caam_rng_ctx) {
    let mut sg = [core::mem::zeroed::<scatterlist>(); 1];
    let mut done = core::mem::zeroed::<completion>();
    sg_init_table(sg.as_mut_ptr(), 1);
    let nents = kfifo_dma_in_prepare(&mut (*ctx).fifo, sg.as_mut_ptr(), 1, CAAM_RNG_MAX_FIFO_STORE_SIZE);
    if nents == 0 { return; }
    let len = caam_rng_read_one((*ctx).jrdev, sg_virt(&mut sg[0]), sg[0].length as i32, (*ctx).desc_async, &mut done);
    if len < 0 { return; }
    kfifo_dma_in_finish(&mut (*ctx).fifo, len as usize);
}

unsafe extern "C" fn caam_rng_worker(work: *mut work_struct) {
    let ctx = container_of!(work, caam_rng_ctx, worker);
    caam_rng_fill_async(ctx);
}

unsafe extern "C" fn caam_read(rng: *mut hwrng, dst: *mut core::ffi::c_void, max: usize, wait: bool) -> i32 {
    let ctx = to_caam_rng_ctx(rng);
    if wait {
        let mut done = core::mem::zeroed::<completion>();
        return caam_rng_read_one((*ctx).jrdev, dst, max as i32, (*ctx).desc_sync, &mut done);
    }
    let out = kfifo_out(&mut (*ctx).fifo, dst, max);
    if kfifo_is_empty(&(*ctx).fifo) { schedule_work(&mut (*ctx).worker); }
    out as i32
}

unsafe extern "C" fn caam_cleanup(rng: *mut hwrng) {
    let ctx = to_caam_rng_ctx(rng);
    flush_work(&mut (*ctx).worker);
    caam_jr_free((*ctx).jrdev);
    kfifo_free(&mut (*ctx).fifo);
}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_RNG_TEST)]
unsafe fn test_len(rng: *mut hwrng, mut len: usize, wait: bool) {
    let ctx = to_caam_rng_ctx(rng);
    let dev = (*ctx).ctrldev;
    let buf = kcalloc(CAAM_RNG_MAX_FIFO_STORE_SIZE, core::mem::size_of::<u8>(), GFP_KERNEL);
    if buf.is_null() { return; }
    while len > 0 {
        let read_len = ((*rng).read.unwrap())(rng, buf as *mut _, len, wait);
        if read_len < 0 || (read_len == 0 && wait) {
            dev_err(dev, b"RNG Read FAILED received %d bytes\n\0".as_ptr() as *const i8, read_len);
            kfree(buf as *mut _);
            return;
        }
        print_hex_dump_debug(b"random bytes@: \0".as_ptr() as *const i8, DUMP_PREFIX_ADDRESS, 16, 4, buf as *const _, read_len as usize, 1);
        len -= read_len as usize;
    }
    kfree(buf as *mut _);
}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_RNG_TEST)]
unsafe fn test_mode_once(rng: *mut hwrng, wait: bool) { test_len(rng, 32, wait); test_len(rng, 64, wait); test_len(rng, 128, wait); }

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_RNG_TEST)]
unsafe fn self_test(rng: *mut hwrng) { pr_info(b"Executing RNG SELF-TEST with wait\n\0".as_ptr() as *const i8); test_mode_once(rng, true); }

unsafe extern "C" fn caam_init(rng: *mut hwrng) -> i32 {
    let ctx = to_caam_rng_ctx(rng);
    let mut err;
    (*ctx).desc_sync = devm_kzalloc((*ctx).ctrldev, CAAM_RNG_DESC_LEN, GFP_KERNEL);
    if (*ctx).desc_sync.is_null() { return -ENOMEM; }
    (*ctx).desc_async = devm_kzalloc((*ctx).ctrldev, CAAM_RNG_DESC_LEN, GFP_KERNEL);
    if (*ctx).desc_async.is_null() { return -ENOMEM; }
    if kfifo_alloc(&mut (*ctx).fifo, ALIGN(CAAM_RNG_MAX_FIFO_STORE_SIZE, dma_get_cache_alignment()), GFP_KERNEL) != 0 { return -ENOMEM; }
    INIT_WORK!(&mut (*ctx).worker, caam_rng_worker);
    (*ctx).jrdev = caam_jr_alloc();
    err = PTR_ERR_OR_ZERO((*ctx).jrdev);
    if err != 0 { kfifo_free(&mut (*ctx).fifo); pr_err(b"Job Ring Device allocation for transform failed\n\0".as_ptr() as *const i8); return err; }
    caam_rng_fill_async(ctx);
    0
}

unsafe extern "C" { fn caam_rng_init(ctrldev: *mut device) -> i32; }

unsafe fn caam_rng_exit(ctrldev: *mut device) { devres_release_group(ctrldev, caam_rng_init); }

#[no_mangle]
pub unsafe extern "C" fn caam_rng_init(ctrldev: *mut device) -> i32 {
    let priv_ = dev_get_drvdata(ctrldev) as *mut caam_drv_private;
    let rng_inst = if (*priv_).era < 10 { (rd_reg32((*(*priv_).jr[0]).perfmon.cha_num_ls) & CHA_ID_LS_RNG_MASK) >> CHA_ID_LS_RNG_SHIFT } else { rd_reg32((*(*priv_).jr[0]).vreg.rng) & CHA_VER_NUM_MASK };
    if rng_inst == 0 { return 0; }
    if !devres_open_group(ctrldev, caam_rng_init, GFP_KERNEL) { return -ENOMEM; }
    let ctx = devm_kzalloc(ctrldev, core::mem::size_of::<caam_rng_ctx>(), GFP_KERNEL) as *mut caam_rng_ctx;
    if ctx.is_null() { return -ENOMEM; }
    (*ctx).ctrldev = ctrldev;
    (*ctx).rng.name = b"rng-caam\0".as_ptr() as *const i8;
    (*ctx).rng.init = Some(caam_init); (*ctx).rng.cleanup = Some(caam_cleanup); (*ctx).rng.read = Some(caam_read); (*ctx).rng.priv_ = ctx as usize;
    dev_info(ctrldev, b"registering rng-caam\n\0".as_ptr() as *const i8);
    let ret = devm_hwrng_register(ctrldev, &mut (*ctx).rng);
    if ret != 0 { caam_rng_exit(ctrldev); return ret; }
    #[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_RNG_TEST)] self_test(&mut (*ctx).rng);
    devres_close_group(ctrldev, caam_rng_init);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
