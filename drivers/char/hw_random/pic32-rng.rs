// SPDX-License-Identifier: GPL-2.0-only
/*
 * PIC32 RNG driver
 *
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2016 Microchip Technology Inc.  All rights reserved.
 */

// Linux kernel dependencies supplied by other files.

const RNGCON: usize = 0x04;
const TRNGEN: u32 = 1 << 8;
const TRNGMOD: u32 = 1 << 11;
const RNGSEED1: usize = 0x18;
const RNGSEED2: usize = 0x1C;
const RNGRCNT: usize = 0x20;
const RCNT_MASK: u32 = 0x7F;

#[repr(C)]
struct pic32_rng {
    base: *mut core::ffi::c_void,
    rng: hwrng,
}

/*
 * The TRNG can generate up to 24Mbps. This is a timeout that should be safe
 * enough given the instructions in the loop and that the TRNG may not always
 * be at maximum rate.
 */
const RNG_TIMEOUT: u32 = 500;

unsafe fn pic32_rng_init(rng: *mut hwrng) -> i32 {
    let priv_: *mut pic32_rng = container_of!(rng, pic32_rng, rng);

    /* enable TRNG in enhanced mode */
    writel(TRNGEN | TRNGMOD, (*priv_).base.byte_add(RNGCON));
    0
}

unsafe fn pic32_rng_read(
    rng: *mut hwrng,
    buf: *mut core::ffi::c_void,
    _max: usize,
    wait: bool,
) -> i32 {
    let priv_: *mut pic32_rng = container_of!(rng, pic32_rng, rng);
    let data: *mut u64 = buf.cast();
    let mut t: u32;
    let mut timeout: u32 = RNG_TIMEOUT;

    loop {
        t = readl((*priv_).base.byte_add(RNGRCNT)) & RCNT_MASK;
        if t == 64 {
            /* TRNG value comes through the seed registers */
            *data = ((readl((*priv_).base.byte_add(RNGSEED2)) as u64) << 32)
                .wrapping_add(readl((*priv_).base.byte_add(RNGSEED1)) as u64);
            return 8;
        }
        timeout = timeout.wrapping_sub(1);
        if !(wait && timeout != 0) {
            break;
        }
    }

    -EIO
}

unsafe fn pic32_rng_cleanup(rng: *mut hwrng) {
    let priv_: *mut pic32_rng = container_of!(rng, pic32_rng, rng);

    writel(0, (*priv_).base.byte_add(RNGCON));
}

unsafe fn pic32_rng_probe(pdev: *mut platform_device) -> i32 {
    let priv_: *mut pic32_rng;
    let clk: *mut clk;

    priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<pic32_rng>(), GFP_KERNEL);
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!((*priv_).base) {
        return PTR_ERR!((*priv_).base);
    }

    clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR!(clk) {
        return PTR_ERR!(clk);
    }

    (*priv_).rng.name = (*pdev).name;
    (*priv_).rng.init = Some(pic32_rng_init);
    (*priv_).rng.read = Some(pic32_rng_read);
    (*priv_).rng.cleanup = Some(pic32_rng_cleanup);

    devm_hwrng_register(&mut (*pdev).dev, &mut (*priv_).rng)
}

static mut pic32_rng_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"microchip,pic32mzda-rng".as_ptr() },
    of_device_id { /* sentinel */ ..unsafe { core::mem::zeroed() } },
];

MODULE_DEVICE_TABLE!(of, pic32_rng_of_match);

static mut pic32_rng_driver: platform_driver = platform_driver {
    probe: Some(pic32_rng_probe),
    driver: driver {
        name: c"pic32-rng".as_ptr(),
        of_match_table: pic32_rng_of_match.as_ptr(),
    },
};

module_platform_driver!(pic32_rng_driver);

MODULE_LICENSE!(c"GPL");
MODULE_AUTHOR!(c"Joshua Henderson <joshua.henderson@microchip.com>");
MODULE_DESCRIPTION!(c"Microchip PIC32 RNG Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
