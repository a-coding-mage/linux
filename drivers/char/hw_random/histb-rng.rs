// SPDX-License-Identifier: GPL-2.0-or-later OR MIT
/*
 * Copyright (c) 2023 David Yang
 */

// Dependencies supplied by the surrounding kernel Rust environment.

const RNG_CTRL: usize = 0x0;
const RNG_SOURCE: u32 = (1u32 << 2) - 1;
const DROP_ENABLE: u32 = 1u32 << 5;
const POST_PROCESS_ENABLE: u32 = 1u32 << 7;
const POST_PROCESS_DEPTH: u32 = 0xffu32 << 8;
const RNG_NUMBER: usize = 0x4;
const RNG_STAT: usize = 0x8;
const DATA_COUNT: u32 = (1u32 << 3) - 1; // max 4

#[repr(C)]
struct histb_rng_priv {
    rng: hwrng,
    base: *mut core::ffi::c_void,
}

/*
 * Observed:
 * depth = 1 -> ~1ms
 * depth = 255 -> ~16ms
 */
unsafe fn histb_rng_wait(base: *mut core::ffi::c_void) -> i32 {
    let mut val: u32 = 0;
    readl_relaxed_poll_timeout(
        (base as *mut u8).add(RNG_STAT) as *mut core::ffi::c_void,
        &mut val,
        val & DATA_COUNT,
        1000,
        30 * 1000,
    )
}

unsafe fn histb_rng_init(base: *mut core::ffi::c_void, depth: u32) {
    let mut val = readl_relaxed((base as *mut u8).add(RNG_CTRL) as *mut core::ffi::c_void);

    val &= !RNG_SOURCE;
    val |= 2;

    val &= !POST_PROCESS_DEPTH;
    val |= core::cmp::min(depth, 0xffu32) << 8;

    val |= POST_PROCESS_ENABLE;
    val |= DROP_ENABLE;

    writel_relaxed(
        val,
        (base as *mut u8).add(RNG_CTRL) as *mut core::ffi::c_void,
    );
}

unsafe fn histb_rng_read(
    rng: *mut hwrng,
    data: *mut core::ffi::c_void,
    max: usize,
    wait: bool,
) -> isize {
    let priv_: *mut histb_rng_priv = container_of!(rng, histb_rng_priv, rng);
    let base = (*priv_).base;

    let mut i = 0usize;
    while i < max {
        if readl_relaxed((base as *mut u8).add(RNG_STAT) as *mut core::ffi::c_void)
            & DATA_COUNT
            == 0
        {
            if !wait {
                return i as isize;
            }
            if histb_rng_wait(base) != 0 {
                pr_err!("failed to generate random number, generated {}\n", i);
                return if i != 0 { i as isize } else { -ETIMEDOUT as isize };
            }
        }
        *(data.add(i) as *mut u32) =
            readl_relaxed((base as *mut u8).add(RNG_NUMBER) as *mut core::ffi::c_void);
        i += core::mem::size_of::<u32>();
    }

    max as isize
}

unsafe fn histb_rng_get_depth(base: *mut core::ffi::c_void) -> u32 {
    (readl_relaxed((base as *mut u8).add(RNG_CTRL) as *mut core::ffi::c_void)
        & POST_PROCESS_DEPTH)
        >> 8
}

unsafe fn depth_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut i8,
) -> isize {
    let priv_: *mut histb_rng_priv = dev_get_drvdata(dev);
    let base = (*priv_).base;
    sprintf!(buf, "{}\n", histb_rng_get_depth(base))
}

unsafe fn depth_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const i8,
    count: usize,
) -> isize {
    let priv_: *mut histb_rng_priv = dev_get_drvdata(dev);
    let base = (*priv_).base;
    let mut depth: u32 = 0;

    if kstrtouint(buf, 0, &mut depth) != 0 {
        return -ERANGE as isize;
    }

    histb_rng_init(base, depth);
    count as isize
}

static DEVICE_ATTR_RW_depth: device_attribute = device_attr_rw!(depth);

static mut histb_rng_attrs: [*mut attribute; 2] = [
    &DEVICE_ATTR_RW_depth.attr as *const attribute as *mut attribute,
    core::ptr::null_mut(),
];

static histb_rng_groups: attribute_group_array = attribute_groups!(histb_rng);

unsafe fn histb_rng_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let priv_: *mut histb_rng_priv;
    let base: *mut core::ffi::c_void;
    let mut ret: i32;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<histb_rng_priv>(), GFP_KERNEL)
        as *mut histb_rng_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(base) {
        return PTR_ERR!(base);
    }

    histb_rng_init(base, 144);
    if histb_rng_wait(base) != 0 {
        dev_err!(dev, "cannot bring up device\n");
        return -ENODEV;
    }

    (*priv_).base = base;
    (*priv_).rng.name = (*pdev).name;
    (*priv_).rng.read = Some(histb_rng_read);
    ret = devm_hwrng_register(dev, &mut (*priv_).rng);
    if ret != 0 {
        dev_err!(dev, "failed to register hwrng: {}\n", ret);
        return ret;
    }

    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);
    dev_set_drvdata(dev, priv_ as *mut core::ffi::c_void);
    0
}

static histb_rng_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c_str!("hisilicon,histb-rng") },
    of_device_id::default(),
];

static mut histb_rng_driver: platform_driver = platform_driver {
    probe: Some(histb_rng_probe),
    driver: device_driver {
        name: c_str!("histb-rng"),
        of_match_table: histb_rng_of_match.as_ptr(),
        dev_groups: histb_rng_groups.as_ptr(),
    },
};

module_platform_driver!(histb_rng_driver);

MODULE_DESCRIPTION!("Hisilicon STB random number generator driver");
MODULE_LICENSE!("Dual MIT/GPL");
MODULE_AUTHOR!("David Yang <mmyangfl@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
