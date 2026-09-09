// SPDX-License-Identifier: GPL-2.0
/*
 * sun8i-ce-trng.c - hardware cryptographic offloader for
 * Allwinner H3/A64/H5/H2+/H6/R40 SoC
 *
 * Copyright (C) 2015-2020 Corentin Labbe <clabbe@baylibre.com>
 *
 * This file handle the TRNG
 *
 * You could find a link for the datasheet in Documentation/arch/arm/sunxi.rst
 */
// Dependencies supplied by the surrounding kernel translation.
use crate::*;

/*
 * Note that according to the algorithm ID, 2 versions of the TRNG exists,
 * The first present in H3/H5/R40/A64 and the second present in H6.
 * This file adds support for both, but only the second is working
 * reliabily according to rngtest.
 */

pub unsafe extern "C" fn sun8i_ce_trng_read(
    rng: *mut hwrng,
    data: *mut core::ffi::c_void,
    max: usize,
    _wait: bool,
) -> isize {
    let ce: *mut sun8i_ce_dev = container_of!(rng, sun8i_ce_dev, trng);
    let mut dma_dst: dma_addr_t;
    let mut err: i32 = 0;
    let flow: i32 = 3;
    let mut todo: u32;
    let chan: *mut sun8i_ce_flow;
    let cet: *mut ce_task;
    let mut common: u32;
    let d: *mut core::ffi::c_void;

    // round the data length to a multiple of 32
    todo = (max as u32).wrapping_add(32);
    todo = todo.wrapping_sub(todo % 32);

    d = kzalloc(todo as usize, GFP_KERNEL | GFP_DMA);
    if d.is_null() {
        return -ENOMEM as isize;
    }

    #[cfg(CONFIG_CRYPTO_DEV_SUN8I_CE_DEBUG)]
    {
        (*ce).hwrng_stat_req = (*ce).hwrng_stat_req.wrapping_add(1);
        (*ce).hwrng_stat_bytes = (*ce).hwrng_stat_bytes.wrapping_add(todo as u64);
    }

    dma_dst = dma_map_single((*ce).dev, d, todo as usize, DMA_FROM_DEVICE);
    if dma_mapping_error((*ce).dev, dma_dst) {
        dev_err((*ce).dev, "Cannot DMA MAP DST\n");
        err = -EFAULT;
        goto err_dst;
    }

    err = pm_runtime_resume_and_get((*ce).dev);
    if err < 0 {
        goto err_pm;
    }

    mutex_lock(&mut (*ce).rnglock);
    chan = &mut (*ce).chanlist[flow as usize];

    cet = &mut (*chan).tl[0];
    core::ptr::write_bytes(cet, 0, 1);

    (*cet).t_id = cpu_to_le32(flow as u32);
    common = (*(*ce).variant).trng | CE_COMM_INT;
    (*cet).t_common_ctl = cpu_to_le32(common);

    // recent CE (H6) need length in bytes, in word otherwise
    if (*(*ce).variant).trng_t_dlen_in_bytes {
        (*cet).t_dlen = cpu_to_le32(todo);
    } else {
        (*cet).t_dlen = cpu_to_le32(todo / 4);
    }

    (*cet).t_sym_ctl = 0;
    (*cet).t_asym_ctl = 0;

    (*cet).t_dst[0].addr = desc_addr_val_le32(ce, dma_dst);
    (*cet).t_dst[0].len = cpu_to_le32(todo / 4);

    err = sun8i_ce_run_task(ce, 3, "TRNG");
    mutex_unlock(&mut (*ce).rnglock);

    pm_runtime_put((*ce).dev);

err_pm:
    dma_unmap_single((*ce).dev, dma_dst, todo as usize, DMA_FROM_DEVICE);

    if err == 0 {
        core::ptr::copy_nonoverlapping(d as *const u8, data as *mut u8, max);
        err = max as i32;
    }
err_dst:
    kfree_sensitive(d);
    err as isize
}

pub unsafe extern "C" fn sun8i_ce_hwrng_register(ce: *mut sun8i_ce_dev) -> i32 {
    let mut ret: i32;

    if (*(*ce).variant).trng == CE_ID_NOTSUPP {
        dev_info((*ce).dev, "TRNG not supported\n");
        return 0;
    }
    (*ce).trng.name = "sun8i Crypto Engine TRNG";
    (*ce).trng.read = Some(sun8i_ce_trng_read);

    ret = hwrng_register(&mut (*ce).trng);
    if ret != 0 {
        dev_err((*ce).dev, "Fail to register the TRNG\n");
    }
    ret
}

pub unsafe extern "C" fn sun8i_ce_hwrng_unregister(ce: *mut sun8i_ce_dev) {
    if (*(*ce).variant).trng == CE_ID_NOTSUPP {
        return;
    }
    hwrng_unregister(&mut (*ce).trng);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
