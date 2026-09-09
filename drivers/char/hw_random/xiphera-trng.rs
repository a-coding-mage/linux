// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2020 Xiphera Ltd. */

// Kernel dependencies supplied by the surrounding Rust environment.
use core::ffi::c_void;

const CONTROL_REG: usize = 0x00000000;
const STATUS_REG: usize = 0x00000004;
const RAND_REG: usize = 0x00000000;

const HOST_TO_TRNG_RESET: u32 = 0x00000001;
const HOST_TO_TRNG_RELEASE_RESET: u32 = 0x00000002;
const HOST_TO_TRNG_ENABLE: u32 = 0x80000000;
const HOST_TO_TRNG_ZEROIZE: u32 = 0x80000004;
const HOST_TO_TRNG_ACK_ZEROIZE: u32 = 0x80000008;
const HOST_TO_TRNG_READ: u32 = 0x8000000F;

/* trng statuses */
const TRNG_ACK_RESET: u32 = 0x000000AC;
const TRNG_SUCCESSFUL_STARTUP: u32 = 0x00000057;
const TRNG_FAILED_STARTUP: u32 = 0x000000FA;
const TRNG_NEW_RAND_AVAILABLE: u32 = 0x000000ED;

#[repr(C)]
pub struct xiphera_trng {
    mem: *mut c_void,
    rng: hwrng,
}

// External kernel declarations supplied by other files.
#[repr(C)]
pub struct hwrng {
    pub name: *const core::ffi::c_char,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut c_void, usize, bool) -> i32>,
    pub quality: i32,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn usleep_range(min: u32, max: u32);
    fn msleep(msecs: u32);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut c_void;
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;

unsafe extern "C" fn xiphera_trng_read(
    rng: *mut hwrng,
    mut buf: *mut c_void,
    mut max: usize,
    _wait: bool,
) -> i32 {
    let trng = (rng as *mut u8).sub(core::mem::offset_of!(xiphera_trng, rng)) as *mut xiphera_trng;
    let mut ret: i32 = 0;

    while max >= core::mem::size_of::<u32>() {
        /* check for data */
        if readl((*trng).mem.add(STATUS_REG)) == TRNG_NEW_RAND_AVAILABLE {
            *(buf as *mut u32) = readl((*trng).mem.add(RAND_REG));
            /*
             * Inform the trng of the read
             * and re-enable it to produce a new random number
             */
            writel(HOST_TO_TRNG_READ, (*trng).mem.add(CONTROL_REG));
            writel(HOST_TO_TRNG_ENABLE, (*trng).mem.add(CONTROL_REG));
            ret += core::mem::size_of::<u32>() as i32;
            buf = buf.add(core::mem::size_of::<u32>());
            max -= core::mem::size_of::<u32>();
        } else {
            break;
        }
    }
    ret
}

unsafe extern "C" fn xiphera_trng_probe(pdev: *mut platform_device) -> i32 {
    let mut trng: *mut xiphera_trng;
    let dev = &mut (*pdev).dev as *mut device;

    trng = devm_kzalloc(dev, core::mem::size_of::<xiphera_trng>(), GFP_KERNEL) as *mut xiphera_trng;
    if trng.is_null() {
        return -ENOMEM;
    }

    (*trng).mem = devm_platform_ioremap_resource(pdev, 0);
    if (*trng).mem as isize == -1 {
        return -1;
    }

    /*
     * the trng needs to be reset first which might not happen in time,
     * hence we incorporate a small delay to ensure proper behaviour
     */
    writel(HOST_TO_TRNG_RESET, (*trng).mem.add(CONTROL_REG));
    usleep_range(100, 200);

    if readl((*trng).mem.add(STATUS_REG)) != TRNG_ACK_RESET {
        /*
         * there is a small chance the trng is just not ready yet,
         * so we try one more time. If the second time fails, we give up
         */
        usleep_range(100, 200);
        if readl((*trng).mem.add(STATUS_REG)) != TRNG_ACK_RESET {
            dev_err(dev, b"failed to reset the trng ip\0".as_ptr() as *const _);
            return -ENODEV;
        }
    }

    /*
     * once again, to ensure proper behaviour we sleep
     * for a while after zeroizing the trng
     */
    writel(HOST_TO_TRNG_RELEASE_RESET, (*trng).mem.add(CONTROL_REG));
    writel(HOST_TO_TRNG_ENABLE, (*trng).mem.add(CONTROL_REG));
    writel(HOST_TO_TRNG_ZEROIZE, (*trng).mem.add(CONTROL_REG));
    msleep(20);

    if readl((*trng).mem.add(STATUS_REG)) != TRNG_SUCCESSFUL_STARTUP {
        /* diagnose the reason for the failure */
        if readl((*trng).mem.add(STATUS_REG)) == TRNG_FAILED_STARTUP {
            dev_err(dev, b"trng ip startup-tests failed\0".as_ptr() as *const _);
            return -ENODEV;
        }
        dev_err(dev, b"startup-tests yielded no response\0".as_ptr() as *const _);
        return -ENODEV;
    }

    writel(HOST_TO_TRNG_ACK_ZEROIZE, (*trng).mem.add(CONTROL_REG));

    (*trng).rng.name = (*pdev).name;
    (*trng).rng.read = Some(xiphera_trng_read);
    (*trng).rng.quality = 900;

    let ret = devm_hwrng_register(dev, &mut (*trng).rng);
    if ret != 0 {
        dev_err(dev, b"failed to register rng device: %d\0".as_ptr() as *const _, ret);
        return ret;
    }

    0
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const core::ffi::c_char,
}

static XIPHERA_TRNG_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"xiphera,xip8001b-trng\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, xiphera_trng_of_match);
// module_platform_driver(xiphera_trng_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Atte Tommiska");
// MODULE_DESCRIPTION("Xiphera FPGA-based true random number generator driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
