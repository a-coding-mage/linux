// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2010-2012 Broadcom. All rights reserved.
 * Copyright (c) 2013 Lubomir Rintel
 */

// Linux kernel dependencies supplied by the surrounding build.

const RNG_CTRL: u32 = 0x0;
const RNG_STATUS: u32 = 0x4;
const RNG_DATA: u32 = 0x8;
const RNG_INT_MASK: u32 = 0x10;

/* enable rng */
const RNG_RBGEN: u32 = 0x1;

/* the initial numbers generated are "less random" so will be discarded */
const RNG_WARMUP_COUNT: u32 = 0x40000;

const RNG_INT_OFF: u32 = 0x1;

#[repr(C)]
pub struct bcm2835_rng_priv {
    pub rng: hwrng,
    pub base: *mut core::ffi::c_void,
    pub mask_interrupts: bool,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
}

#[repr(C)]
pub struct hwrng {
    pub name: *const core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut hwrng) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut hwrng)>,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const core::ffi::c_char,
}

unsafe fn to_rng_priv(rng: *mut hwrng) -> *mut bcm2835_rng_priv {
    // Equivalent to container_of(rng, struct bcm2835_rng_priv, rng).
    (rng as *mut u8).sub(core::mem::offset_of!(bcm2835_rng_priv, rng)) as *mut bcm2835_rng_priv
}

unsafe fn rng_readl(priv_: *mut bcm2835_rng_priv, offset: u32) -> u32 {
    /* MIPS chips strapped for BE will automagically configure the
     * peripheral registers for CPU-native byte order.
     */
    // Preserve the CONFIG_MIPS && CONFIG_CPU_BIG_ENDIAN conditional intent.
    if cfg!(all(target_arch = "mips", target_endian = "big")) {
        core::ptr::read_volatile(((*priv_).base as *const u8).add(offset as usize) as *const u32)
    } else {
        u32::from_le(core::ptr::read_volatile(
            ((*priv_).base as *const u8).add(offset as usize) as *const u32,
        ))
    }
}

unsafe fn rng_writel(priv_: *mut bcm2835_rng_priv, val: u32, offset: u32) {
    if cfg!(all(target_arch = "mips", target_endian = "big")) {
        core::ptr::write_volatile(
            ((*priv_).base as *mut u8).add(offset as usize) as *mut u32,
            val,
        );
    } else {
        core::ptr::write_volatile(
            ((*priv_).base as *mut u8).add(offset as usize) as *mut u32,
            val.to_le(),
        );
    }
}

unsafe extern "C" fn bcm2835_rng_read(
    rng: *mut hwrng,
    buf: *mut core::ffi::c_void,
    max: usize,
    wait: bool,
) -> i32 {
    let priv_ = to_rng_priv(rng);
    let max_words = (max / core::mem::size_of::<u32>()) as u32;
    let num_words: u32;
    let mut count: u32;

    while (rng_readl(priv_, RNG_STATUS) >> 24) == 0 {
        if !wait {
            return 0;
        }
        hwrng_yield(rng);
    }

    num_words = core::cmp::min(rng_readl(priv_, RNG_STATUS) >> 24, max_words);
    count = 0;
    while count < num_words {
        (buf as *mut u32).add(count as usize).write(rng_readl(priv_, RNG_DATA));
        count += 1;
    }

    (num_words as usize * core::mem::size_of::<u32>()) as i32
}

unsafe extern "C" fn bcm2835_rng_init(rng: *mut hwrng) -> i32 {
    let priv_ = to_rng_priv(rng);
    let mut ret = clk_prepare_enable((*priv_).clk);
    let mut val: u32;

    if ret != 0 {
        return ret;
    }

    ret = reset_control_reset((*priv_).reset);
    if ret != 0 {
        clk_disable_unprepare((*priv_).clk);
        return ret;
    }

    if (*priv_).mask_interrupts {
        /* mask the interrupt */
        val = rng_readl(priv_, RNG_INT_MASK);
        val |= RNG_INT_OFF;
        rng_writel(priv_, val, RNG_INT_MASK);
    }

    /* set warm-up count & enable */
    rng_writel(priv_, RNG_WARMUP_COUNT, RNG_STATUS);
    rng_writel(priv_, RNG_RBGEN, RNG_CTRL);

    ret
}

unsafe extern "C" fn bcm2835_rng_cleanup(rng: *mut hwrng) {
    let priv_ = to_rng_priv(rng);

    /* disable rng hardware */
    rng_writel(priv_, 0, RNG_CTRL);
    clk_disable_unprepare((*priv_).clk);
}

#[repr(C)]
pub struct bcm2835_rng_of_data {
    pub mask_interrupts: bool,
}

static NSP_RNG_OF_DATA: bcm2835_rng_of_data = bcm2835_rng_of_data {
    mask_interrupts: true,
};

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

static BCM2835_RNG_OF_MATCH: &[of_device_id] = &[
    of_device_id { compatible: b"brcm,bcm2835-rng\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { compatible: b"brcm,bcm-nsp-rng\0".as_ptr() as *const _, data: &NSP_RNG_OF_DATA as *const _ as *const _ },
    of_device_id { compatible: b"brcm,bcm5301x-rng\0".as_ptr() as *const _, data: &NSP_RNG_OF_DATA as *const _ as *const _ },
    of_device_id { compatible: b"brcm,bcm6368-rng\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn bcm2835_rng_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<bcm2835_rng_priv>(), GFP_KERNEL);
    if priv_.is_null() {
        return -12;
    }

    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*priv_).base) {
        return ptr_err((*priv_).base);
    }

    (*priv_).clk = devm_clk_get_optional(dev, core::ptr::null());
    if is_err((*priv_).clk as *mut core::ffi::c_void) {
        return ptr_err((*priv_).clk as *mut core::ffi::c_void);
    }

    (*priv_).reset = devm_reset_control_get_optional_exclusive(dev, core::ptr::null());
    if is_err((*priv_).reset as *mut core::ffi::c_void) {
        return ptr_err((*priv_).reset as *mut core::ffi::c_void);
    }

    (*priv_).rng.name = (*pdev).name;
    (*priv_).rng.init = Some(bcm2835_rng_init);
    (*priv_).rng.read = Some(bcm2835_rng_read);
    (*priv_).rng.cleanup = Some(bcm2835_rng_cleanup);

    if dev_of_node(dev) {
        /* Check for rng init function, execute it */
        let of_data = of_device_get_match_data(dev) as *const bcm2835_rng_of_data;
        if !of_data.is_null() {
            (*priv_).mask_interrupts = (*of_data).mask_interrupts;
        }
    }

    /* register driver */
    let err = devm_hwrng_register(dev, &mut (*priv_).rng);
    if err != 0 {
        dev_err(dev, b"hwrng registration failed\n\0".as_ptr() as *const _);
    } else {
        dev_info(dev, b"hwrng registered\n\0".as_ptr() as *const _);
    }
    err
}

#[repr(C)]
pub struct platform_device_id {
    pub name: *const core::ffi::c_char,
}

static BCM2835_RNG_DEVTYPE: &[platform_device_id] = &[
    platform_device_id { name: b"bcm2835-rng\0".as_ptr() as *const _ },
    platform_device_id { name: b"bcm63xx-rng\0".as_ptr() as *const _ },
    platform_device_id { name: core::ptr::null() },
];

// The platform driver object and module registration are supplied by the kernel
// module framework; metadata mirrors MODULE_DEVICE_TABLE, MODULE_AUTHOR,
// MODULE_DESCRIPTION, and MODULE_LICENSE from the C source.

extern "C" {
    static GFP_KERNEL: u32;
    fn hwrng_yield(rng: *mut hwrng);
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn reset_control_reset(reset: *mut reset_control) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut bcm2835_rng_priv;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn devm_clk_get_optional(dev: *mut device, id: *const core::ffi::c_char) -> *mut clk;
    fn devm_reset_control_get_optional_exclusive(dev: *mut device, id: *const core::ffi::c_char) -> *mut reset_control;
    fn dev_of_node(dev: *mut device) -> bool;
    fn of_device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char);
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
