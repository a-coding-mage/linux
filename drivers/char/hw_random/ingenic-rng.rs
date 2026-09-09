// SPDX-License-Identifier: GPL-2.0
/*
 * Ingenic Random Number Generator driver
 * Copyright (c) 2017 PrasannaKumar Muralidharan <prasannatsmkumar@gmail.com>
 * Copyright (c) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
 */

// Required Linux kernel types, helpers, and constants are supplied externally.

const RNG_REG_ERNG_OFFSET: usize = 0x0;
const RNG_REG_RNG_OFFSET: usize = 0x4;

const ERNG_READY: u32 = 1u32 << 31;
const ERNG_ENABLE: u32 = 1u32 << 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum IngenicRngVersion {
    ID_JZ4780,
    ID_X1000,
}

// External kernel definitions.
#[repr(C)]
pub struct Hwrng {
    pub name: *const core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>,
    pub read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
}

#[repr(C)]
pub struct Device;
#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
    pub name: *const core::ffi::c_char,
}
#[repr(C)]
pub struct OfDeviceId;
#[repr(C)]
pub struct PlatformDriver;

type IoMem = core::ffi::c_void;

#[repr(C)]
struct IngenicRng {
    version: IngenicRngVersion,
    base: *mut IoMem,
    rng: Hwrng,
}

unsafe extern "C" {
    fn writel(value: u32, address: *mut IoMem);
    fn readl(address: *mut IoMem) -> u32;
    fn readl_poll_timeout(address: *mut IoMem, value: *mut u32, condition: u32, delay: u32, timeout: u32) -> i32;
    fn udelay(usecs: u32);
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut IoMem;
    fn of_device_get_match_data(dev: *mut Device) -> *const core::ffi::c_void;
    fn hwrng_register(rng: *mut Hwrng) -> i32;
    fn hwrng_unregister(rng: *mut Hwrng);
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut IngenicRng;
    fn pr_err(format: *const core::ffi::c_char, ...);
    fn dev_err(dev: *mut Device, format: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut Device, format: *const core::ffi::c_char, ...);
}

const GFP_KERNEL: u32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ETIMEDOUT: i32 = 110;

unsafe extern "C" fn ingenic_rng_init(rng: *mut Hwrng) -> i32 {
    let priv_ = (rng as *mut u8).sub(core::mem::offset_of!(IngenicRng, rng)) as *mut IngenicRng;
    writel(ERNG_ENABLE, (*priv_).base.add(RNG_REG_ERNG_OFFSET));
    0
}

unsafe extern "C" fn ingenic_rng_cleanup(rng: *mut Hwrng) {
    let priv_ = (rng as *mut u8).sub(core::mem::offset_of!(IngenicRng, rng)) as *mut IngenicRng;
    writel(0, (*priv_).base.add(RNG_REG_ERNG_OFFSET));
}

unsafe extern "C" fn ingenic_rng_read(rng: *mut Hwrng, buf: *mut core::ffi::c_void, _max: usize, _wait: bool) -> i32 {
    let priv_ = (rng as *mut u8).sub(core::mem::offset_of!(IngenicRng, rng)) as *mut IngenicRng;
    let data = buf as *mut u32;
    let mut status = 0u32;
    let ret: i32;

    if (*priv_).version >= IngenicRngVersion::ID_X1000 {
        ret = readl_poll_timeout((*priv_).base.add(RNG_REG_ERNG_OFFSET), &mut status, status & ERNG_READY, 10, 1000);
        if ret == -ETIMEDOUT {
            pr_err(c"%s: Wait for RNG data ready timeout\n".as_ptr(), c"ingenic_rng_read".as_ptr());
            return ret;
        }
    } else {
        /* A delay is required so that the current RNG data is not bit shifted
         * version of previous RNG data which could happen if random data is
         * read continuously from this device. */
        udelay(20);
    }

    *data = readl((*priv_).base.add(RNG_REG_RNG_OFFSET));
    4
}

unsafe extern "C" fn ingenic_rng_probe(pdev: *mut PlatformDevice) -> i32 {
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<IngenicRng>(), GFP_KERNEL) as *mut IngenicRng;
    if priv_.is_null() { return -ENOMEM; }

    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if (*priv_).base.is_null() {
        pr_err(c"%s: Failed to map RNG registers\n".as_ptr(), c"ingenic_rng_probe".as_ptr());
        return -EINVAL;
    }

    (*priv_).version = *(of_device_get_match_data(&mut (*pdev).dev) as *const IngenicRngVersion);
    (*priv_).rng.name = (*pdev).name;
    (*priv_).rng.init = Some(ingenic_rng_init);
    (*priv_).rng.cleanup = Some(ingenic_rng_cleanup);
    (*priv_).rng.read = Some(ingenic_rng_read);

    let ret = hwrng_register(&mut (*priv_).rng);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Failed to register hwrng\n".as_ptr());
        return ret;
    }
    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);
    dev_info(&mut (*pdev).dev, c"Ingenic RNG driver registered\n".as_ptr());
    0
}

unsafe extern "C" fn ingenic_rng_remove(pdev: *mut PlatformDevice) {
    let priv_ = platform_get_drvdata(pdev);
    hwrng_unregister(&mut (*priv_).rng);
    writel(0, (*priv_).base.add(RNG_REG_ERNG_OFFSET));
}

// Device-match table and platform-driver registration are provided by the kernel integration.
// MODULE_DEVICE_TABLE(of, ingenic_rng_of_match);
// module_platform_driver(ingenic_rng_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("PrasannaKumar Muralidharan <prasannatsmkumar@gmail.com>");
// MODULE_AUTHOR("周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>");
// MODULE_DESCRIPTION("Ingenic Random Number Generator driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
