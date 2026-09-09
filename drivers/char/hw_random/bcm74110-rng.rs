// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2024 Broadcom
 */

// Dependency intent from the C source: Linux module, kernel, I/O, delay,
// platform-device, random, and hardware-rng interfaces are supplied externally.

const HOST_REV_ID: usize = 0x00;
const HOST_FIFO_DEPTH: usize = 0x04;
const HOST_FIFO_COUNT: usize = 0x08;
const HOST_FIFO_THRESHOLD: usize = 0x0c;
const HOST_FIFO_DATA: usize = 0x10;

const HOST_FIFO_COUNT_MASK: u32 = 0xffff;

/* Delay range in microseconds */
const FIFO_DELAY_MIN_US: u32 = 3;
const FIFO_DELAY_MAX_US: u32 = 7;
const FIFO_DELAY_MAX_COUNT: u32 = 10;

#[repr(C)]
pub struct bcm74110_priv {
    pub base: *mut core::ffi::c_void,
}

#[inline]
unsafe fn bcm74110_rng_fifo_count(mem: *mut core::ffi::c_void) -> u32 {
    (readl_relaxed(mem) & HOST_FIFO_COUNT_MASK) as u32
}

unsafe fn bcm74110_rng_read(
    rng: *mut hwrng,
    buf: *mut core::ffi::c_void,
    max: usize,
    wait: bool,
) -> usize {
    let priv_: *mut bcm74110_priv = (*rng).priv_ as *mut bcm74110_priv;
    let fc_addr = ((*priv_).base as *mut u8).add(HOST_FIFO_COUNT);
    let fd_addr = ((*priv_).base as *mut u8).add(HOST_FIFO_DATA);
    let mut underrun_count: u32 = 0;
    let max_words: u32 = (max / core::mem::size_of::<u32>()) as u32;
    let mut num_words: u32;
    let mut i: u32;

    /*
     * We need to check how many words are available in the RNG FIFO. If
     * there aren't any, we need to wait for some to become available.
     */
    loop {
        num_words = bcm74110_rng_fifo_count(fc_addr as *mut core::ffi::c_void);
        if num_words != 0 {
            break;
        }
        if !wait {
            return 0;
        }
        /*
         * As a precaution, limit how long we wait. If the FIFO doesn't
         * refill within the allotted time, return 0 (=no data) to the
         * caller.
         */
        if likely(underrun_count < FIFO_DELAY_MAX_COUNT) {
            usleep_range(FIFO_DELAY_MIN_US, FIFO_DELAY_MAX_US);
        } else {
            return 0;
        }
        underrun_count = underrun_count.wrapping_add(1);
    }
    if num_words > max_words {
        num_words = max_words;
    }

    /* Bail early if we run out of random numbers unexpectedly */
    i = 0;
    while i < num_words
        && bcm74110_rng_fifo_count(fc_addr as *mut core::ffi::c_void) > 0
    {
        *((buf as *mut u32).add(i as usize)) =
            readl_relaxed(fd_addr as *mut core::ffi::c_void);
        i = i.wrapping_add(1);
    }

    (i as usize) * core::mem::size_of::<u32>()
}

#[repr(C)]
pub struct hwrng {
    pub read: Option<unsafe extern "C" fn(
        rng: *mut hwrng,
        buf: *mut core::ffi::c_void,
        max: usize,
        wait: bool,
    ) -> usize>,
    pub name: *const core::ffi::c_char,
    pub priv_: usize,
}

static mut bcm74110_hwrng: hwrng = hwrng {
    read: Some(bcm74110_rng_read),
    name: core::ptr::null(),
    priv_: 0,
};

unsafe fn bcm74110_rng_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let priv_: *mut bcm74110_priv;
    let mut rc: i32;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<bcm74110_priv>(), GFP_KERNEL);
    if priv_.is_null() {
        return -ENOMEM;
    }

    bcm74110_hwrng.name = (*pdev).name;
    bcm74110_hwrng.priv_ = priv_ as usize;

    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*priv_).base) {
        return ptr_err((*priv_).base);
    }

    rc = devm_hwrng_register(dev, &raw mut bcm74110_hwrng);
    if rc != 0 {
        dev_err(dev, c"hwrng registration failed (%d)\n", rc);
    } else {
        dev_info(dev, c"hwrng registered\n");
    }

    rc
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
}

static bcm74110_rng_match: [of_device_id; 2] = [
    of_device_id { compatible: c"brcm,bcm74110-rng".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    driver: driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

#[repr(C)]
struct driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
}

static mut bcm74110_rng_driver: platform_driver = platform_driver {
    driver: driver {
        name: KBUILD_MODNAME,
        of_match_table: bcm74110_rng_match.as_ptr(),
    },
    probe: Some(bcm74110_rng_probe),
};

// MODULE_DEVICE_TABLE(of, bcm74110_rng_match);
// module_platform_driver(bcm74110_rng_driver);
// MODULE_AUTHOR("Markus Mayer <mmayer@broadcom.com>");
// MODULE_DESCRIPTION("BCM 74110 Random Number Generator (RNG) driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
