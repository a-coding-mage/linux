// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Broadcom Corporation
 */
/*
 * DESCRIPTION: The Broadcom iProc RNG200 Driver
 */

// Kernel dependencies supplied by the surrounding repository.

const RNG_CTRL_OFFSET: usize = 0x00;
const RNG_CTRL_RNG_RBGEN_MASK: u32 = 0x0000_1fff;
const RNG_CTRL_RNG_RBGEN_ENABLE: u32 = 0x0000_0001;

const RNG_SOFT_RESET_OFFSET: usize = 0x04;
const RNG_SOFT_RESET: u32 = 0x0000_0001;

const RBG_SOFT_RESET_OFFSET: usize = 0x08;
const RBG_SOFT_RESET: u32 = 0x0000_0001;

const RNG_INT_STATUS_OFFSET: usize = 0x18;
const RNG_INT_STATUS_MASTER_FAIL_LOCKOUT_IRQ_MASK: u32 = 0x8000_0000;
const RNG_INT_STATUS_STARTUP_TRANSITIONS_MET_IRQ_MASK: u32 = 0x0002_0000;
const RNG_INT_STATUS_NIST_FAIL_IRQ_MASK: u32 = 0x0000_0020;
const RNG_INT_STATUS_TOTAL_BITS_COUNT_IRQ_MASK: u32 = 0x0000_0001;

const RNG_FIFO_DATA_OFFSET: usize = 0x20;
const RNG_FIFO_COUNT_OFFSET: usize = 0x24;
const RNG_FIFO_COUNT_RNG_FIFO_COUNT_MASK: u32 = 0x0000_00ff;

#[repr(C)]
pub struct IprocRng200Dev {
    pub rng: Hwrng,
    pub base: *mut core::ffi::c_void,
}

// External kernel types and functions are provided by other translation units.
#[repr(C)]
pub struct Hwrng {
    pub name: *const core::ffi::c_char,
    pub read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> isize>,
    pub init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>,
}

unsafe fn to_rng_priv(rng: *mut Hwrng) -> *mut IprocRng200Dev {
    (rng as *mut u8).sub(core::mem::offset_of!(IprocRng200Dev, rng)) as *mut IprocRng200Dev
}

unsafe fn iproc_rng200_enable_set(rng_base: *mut core::ffi::c_void, enable: bool) {
    let mut val = ioread32(rng_base.add(RNG_CTRL_OFFSET));
    val &= !RNG_CTRL_RNG_RBGEN_MASK;
    if enable {
        val |= RNG_CTRL_RNG_RBGEN_ENABLE;
    }
    iowrite32(val, rng_base.add(RNG_CTRL_OFFSET));
}

unsafe fn iproc_rng200_restart(rng_base: *mut core::ffi::c_void) {
    iproc_rng200_enable_set(rng_base, false);
    iowrite32(0xffff_ffff, rng_base.add(RNG_INT_STATUS_OFFSET));

    let mut val = ioread32(rng_base.add(RBG_SOFT_RESET_OFFSET));
    val |= RBG_SOFT_RESET;
    iowrite32(val, rng_base.add(RBG_SOFT_RESET_OFFSET));

    val = ioread32(rng_base.add(RNG_SOFT_RESET_OFFSET));
    val |= RNG_SOFT_RESET;
    iowrite32(val, rng_base.add(RNG_SOFT_RESET_OFFSET));

    val = ioread32(rng_base.add(RNG_SOFT_RESET_OFFSET));
    val &= !RNG_SOFT_RESET;
    iowrite32(val, rng_base.add(RNG_SOFT_RESET_OFFSET));

    val = ioread32(rng_base.add(RBG_SOFT_RESET_OFFSET));
    val &= !RBG_SOFT_RESET;
    iowrite32(val, rng_base.add(RBG_SOFT_RESET_OFFSET));
    iproc_rng200_enable_set(rng_base, true);
}

unsafe extern "C" fn iproc_rng200_read(rng: *mut Hwrng, mut buf: *mut core::ffi::c_void, max: usize, wait: bool) -> isize {
    let priv_ = &mut *to_rng_priv(rng);
    let mut num_remaining = max as u32;
    let mut num_resets = 0u32;
    const MAX_RESETS_PER_READ: u32 = 1;
    const MAX_IDLE_TIME: u64 = 1 * HZ;
    let mut idle_endtime = jiffies().wrapping_add(MAX_IDLE_TIME);

    while num_remaining > 0 && time_before(jiffies(), idle_endtime) {
        let status = ioread32(priv_.base.add(RNG_INT_STATUS_OFFSET));
        if status & (RNG_INT_STATUS_MASTER_FAIL_LOCKOUT_IRQ_MASK | RNG_INT_STATUS_NIST_FAIL_IRQ_MASK) != 0 {
            if num_resets >= MAX_RESETS_PER_READ { return (max - num_remaining as usize) as isize; }
            iproc_rng200_restart(priv_.base);
            num_resets += 1;
        }
        if ioread32(priv_.base.add(RNG_FIFO_COUNT_OFFSET)) & RNG_FIFO_COUNT_RNG_FIFO_COUNT_MASK > 0 {
            if num_remaining >= core::mem::size_of::<u32>() as u32 {
                *(buf as *mut u32) = ioread32(priv_.base.add(RNG_FIFO_DATA_OFFSET));
                buf = buf.add(core::mem::size_of::<u32>());
                num_remaining -= core::mem::size_of::<u32>() as u32;
            } else {
                let rnd_number = ioread32(priv_.base.add(RNG_FIFO_DATA_OFFSET));
                core::ptr::copy_nonoverlapping((&rnd_number as *const u32) as *const u8, buf as *mut u8, num_remaining as usize);
                buf = buf.add(num_remaining as usize);
                num_remaining = 0;
            }
            idle_endtime = jiffies().wrapping_add(MAX_IDLE_TIME);
        } else {
            if !wait { return (max - num_remaining as usize) as isize; }
            usleep_range(core::cmp::min(num_remaining * 10, 500), 500);
        }
    }
    (max - num_remaining as usize) as isize
}

unsafe extern "C" fn iproc_rng200_init(rng: *mut Hwrng) -> i32 {
    iproc_rng200_enable_set((*to_rng_priv(rng)).base, true);
    0
}

unsafe extern "C" fn iproc_rng200_cleanup(rng: *mut Hwrng) {
    iproc_rng200_enable_set((*to_rng_priv(rng)).base, false);
}

#[repr(C)]
pub struct PlatformDevice { pub dev: Device }
#[repr(C)]
pub struct Device;

unsafe extern "C" fn iproc_rng200_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<IprocRng200Dev>());
    if priv_.is_null() { return -12; }
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if (*priv_).base as isize == -1 {
        dev_err(dev, b"failed to remap rng regs\0".as_ptr() as *const core::ffi::c_char);
        return -1;
    }
    dev_set_drvdata(dev, priv_ as *mut core::ffi::c_void);
    (*priv_).rng.name = b"iproc-rng200\0".as_ptr() as *const core::ffi::c_char;
    (*priv_).rng.read = Some(iproc_rng200_read);
    (*priv_).rng.init = Some(iproc_rng200_init);
    (*priv_).rng.cleanup = Some(iproc_rng200_cleanup);
    let ret = devm_hwrng_register(dev, &mut (*priv_).rng);
    if ret != 0 {
        dev_err(dev, b"hwrng registration failed\0".as_ptr() as *const core::ffi::c_char);
        return ret;
    }
    dev_info(dev, b"hwrng registered\0".as_ptr() as *const core::ffi::c_char);
    0
}

unsafe extern "C" fn iproc_rng200_suspend(dev: *mut Device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut IprocRng200Dev;
    iproc_rng200_cleanup(&mut (*priv_).rng);
    0
}

unsafe extern "C" fn iproc_rng200_resume(dev: *mut Device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut IprocRng200Dev;
    iproc_rng200_init(&mut (*priv_).rng);
    0
}

#[repr(C)]
pub struct DevPmOps {
    pub suspend: Option<unsafe extern "C" fn(*mut Device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut Device) -> i32>,
}
static iproc_rng200_pm_ops: DevPmOps = DevPmOps {
    suspend: Some(iproc_rng200_suspend),
    resume: Some(iproc_rng200_resume),
};

#[repr(C)]
pub struct OfDeviceId { pub compatible: *const core::ffi::c_char }
static iproc_rng200_of_match: [OfDeviceId; 5] = [
    OfDeviceId { compatible: b"brcm,bcm2711-rng200\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"brcm,bcm7211-rng200\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"brcm,bcm7278-rng200\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"brcm,iproc-rng200\0".as_ptr() as *const _ },
    OfDeviceId { compatible: core::ptr::null() },
];

#[repr(C)]
pub struct PlatformDriver { pub name: *const core::ffi::c_char, pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32> }
static iproc_rng200_driver: PlatformDriver = PlatformDriver {
    name: b"iproc-rng200\0".as_ptr() as *const _, probe: Some(iproc_rng200_probe)
};

// The remaining platform-driver registration declarations are supplied by the kernel bindings.
extern "C" {
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn jiffies() -> u64;
    fn time_before(a: u64, b: u64) -> bool;
    fn usleep_range(min: u32, max: u32);
    fn devm_kzalloc(dev: *mut Device, size: usize) -> *mut IprocRng200Dev;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut Device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut Device) -> *mut core::ffi::c_void;
    fn devm_hwrng_register(dev: *mut Device, rng: *mut Hwrng) -> i32;
    fn dev_err(dev: *mut Device, fmt: *const core::ffi::c_char);
    fn dev_info(dev: *mut Device, fmt: *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
