// SPDX-License-Identifier: GPL-2.0
/*
 * sl3516-ce-rng.c - hardware cryptographic offloader for SL3516 SoC.
 *
 * Copyright (C) 2021 Corentin Labbe <clabbe@baylibre.com>
 *
 * This file handle the RNG found in the SL3516 crypto engine
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
pub const IPSEC_RAND_NUM_REG: usize = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hwrng {
    pub name: *const u8,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut c_void, usize, bool) -> isize>,
    pub quality: u32,
}

#[repr(C)]
pub struct sl3516_ce_dev {
    pub dev: *mut device,
    pub base: *mut u8,
    pub trng: hwrng,
    #[cfg(feature = "CONFIG_CRYPTO_DEV_SL3516_DEBUG")]
    pub hwrng_stat_req: u64,
    #[cfg(feature = "CONFIG_CRYPTO_DEV_SL3516_DEBUG")]
    pub hwrng_stat_bytes: u64,
}

extern "C" {
    fn pm_runtime_get_sync(dev: *mut device) -> i32;
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_put(dev: *mut device);
    fn readl(addr: *mut u8) -> u32;
    fn hwrng_register(rng: *mut hwrng) -> i32;
    fn hwrng_unregister(rng: *mut hwrng);
    fn dev_err(dev: *mut device, fmt: *const u8);
    fn sl3516_ce_rng_container_of(rng: *mut hwrng) -> *mut sl3516_ce_dev;
}

unsafe extern "C" fn sl3516_ce_rng_read(
    rng: *mut hwrng,
    buf: *mut c_void,
    max: usize,
    _wait: bool,
) -> isize {
    let ce: *mut sl3516_ce_dev = sl3516_ce_rng_container_of(rng);
    let mut data = buf as *mut u32;
    let mut read: usize = 0;
    let err: i32;

    #[cfg(feature = "CONFIG_CRYPTO_DEV_SL3516_DEBUG")]
    {
        (*ce).hwrng_stat_req += 1;
        (*ce).hwrng_stat_bytes += max as u64;
    }

    err = pm_runtime_get_sync((*ce).dev);
    if err < 0 {
        pm_runtime_put_noidle((*ce).dev);
        return err as isize;
    }

    while read < max {
        *data = readl((*ce).base.add(IPSEC_RAND_NUM_REG));
        data = data.add(1);
        read += 4;
    }

    pm_runtime_put((*ce).dev);

    read as isize
}

pub unsafe extern "C" fn sl3516_ce_rng_register(ce: *mut sl3516_ce_dev) -> i32 {
    let ret: i32;

    (*ce).trng.name = b"SL3516 Crypto Engine RNG\0".as_ptr();
    (*ce).trng.read = Some(sl3516_ce_rng_read);
    (*ce).trng.quality = 700;

    ret = hwrng_register(&mut (*ce).trng);
    if ret != 0 {
        dev_err((*ce).dev, b"Fail to register the RNG\n\0".as_ptr());
    }
    ret
}

pub unsafe extern "C" fn sl3516_ce_rng_unregister(ce: *mut sl3516_ce_dev) {
    hwrng_unregister(&mut (*ce).trng);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
