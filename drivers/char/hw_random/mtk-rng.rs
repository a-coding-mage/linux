// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Mediatek Hardware Random Number Generator
 *
 * Copyright (C) 2017 Sean Wang <sean.wang@mediatek.com>
 * Copyright (C) 2026 Daniel Golle <daniel@makrotopia.org>
 */

// Linux kernel dependencies supplied by other files.

const RNG_AUTOSUSPEND_TIMEOUT: u32 = 100;
const USEC_POLL: u32 = 2;
const TIMEOUT_POLL: u32 = 60;
const RNG_CTRL: usize = 0x00;
const RNG_EN: u32 = 1 << 0;
const RNG_READY: u32 = 1 << 31;
const RNG_DATA: usize = 0x08;
const MTK_RNG_SMC: usize = 1 << 0;
const MTK_SIP_KERNEL_GET_RND: usize = mtk_sip_smc_cmd(0x550);

#[repr(C)]
struct MtkRng {
    base: *mut core::ffi::c_void,
    clk: *mut core::ffi::c_void,
    rng: Hwrng,
    dev: *mut Device,
    flags: usize,
}

#[repr(C)]
struct Hwrng {
    name: *const core::ffi::c_char,
    quality: u32,
    init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>,
    cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>,
    read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
}

#[repr(C)]
struct Device;
#[repr(C)]
struct PlatformDevice {
    dev: Device,
    name: *const core::ffi::c_char,
}
#[repr(C)]
struct ArmSmcccRes {
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
}

extern "C" {
    fn clk_prepare_enable(clk: *mut core::ffi::c_void) -> i32;
    fn clk_disable_unprepare(clk: *mut core::ffi::c_void);
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn readl_poll_timeout_atomic(addr: *mut u8, value: *mut i32, condition: i32, delay: u32, timeout: u32) -> i32;
    fn pm_runtime_get_sync(dev: *mut Device) -> i32;
    fn pm_runtime_put_sync_autosuspend(dev: *mut Device) -> i32;
    fn arm_smccc_smc(a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize, a7: usize, res: *mut ArmSmcccRes);
    fn mtk_sip_smc_cmd(cmd: usize) -> usize;
    fn dev_get_drvdata(dev: *mut Device) -> *mut core::ffi::c_void;
}

unsafe fn mtk_rng_init(rng: *mut Hwrng) -> i32 {
    let priv_: *mut MtkRng = (rng as *mut u8).sub(core::mem::offset_of!(MtkRng, rng)) as *mut MtkRng;
    let err = clk_prepare_enable((*priv_).clk);
    if err != 0 { return err; }
    let mut val = readl((*priv_).base.add(RNG_CTRL));
    val |= RNG_EN;
    writel(val, (*priv_).base.add(RNG_CTRL));
    0
}

unsafe fn mtk_rng_cleanup(rng: *mut Hwrng) {
    let priv_: *mut MtkRng = (rng as *mut u8).sub(core::mem::offset_of!(MtkRng, rng)) as *mut MtkRng;
    let mut val = readl((*priv_).base.add(RNG_CTRL));
    val &= !RNG_EN;
    writel(val, (*priv_).base.add(RNG_CTRL));
    clk_disable_unprepare((*priv_).clk);
}

unsafe fn mtk_rng_wait_ready(rng: *mut Hwrng, wait: bool) -> bool {
    let priv_: *mut MtkRng = (rng as *mut u8).sub(core::mem::offset_of!(MtkRng, rng)) as *mut MtkRng;
    let mut ready = readl((*priv_).base.add(RNG_CTRL)) as i32;
    if (ready & RNG_READY as i32) == 0 && wait {
        readl_poll_timeout_atomic((*priv_).base.add(RNG_CTRL), &mut ready, ready & RNG_READY as i32, USEC_POLL, TIMEOUT_POLL);
    }
    (ready & RNG_READY as i32) != 0
}

unsafe fn mtk_rng_read(rng: *mut Hwrng, mut buf: *mut core::ffi::c_void, mut max: usize, wait: bool) -> i32 {
    let priv_: *mut MtkRng = (rng as *mut u8).sub(core::mem::offset_of!(MtkRng, rng)) as *mut MtkRng;
    let mut retval: i32 = 0;
    pm_runtime_get_sync((*priv_).dev);
    while max >= core::mem::size_of::<u32>() {
        if !mtk_rng_wait_ready(rng, wait) { break; }
        *(buf as *mut u32) = readl((*priv_).base.add(RNG_DATA));
        retval += core::mem::size_of::<u32>() as i32;
        buf = (buf as *mut u8).add(core::mem::size_of::<u32>()) as *mut core::ffi::c_void;
        max -= core::mem::size_of::<u32>();
    }
    pm_runtime_put_sync_autosuspend((*priv_).dev);
    if retval != 0 || !wait { retval } else { -5 }
}

unsafe fn mtk_rng_read_smc(_rng: *mut Hwrng, mut buf: *mut core::ffi::c_void, mut max: usize, wait: bool) -> i32 {
    let mut res = ArmSmcccRes { a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0 };
    let mut retval: i32 = 0;
    while max >= core::mem::size_of::<u32>() {
        arm_smccc_smc(MTK_SIP_KERNEL_GET_RND, 0, 0, 0, 0, 0, 0, 0, &mut res);
        if res.a0 != 0 { break; }
        *(buf as *mut u32) = res.a1 as u32;
        retval += core::mem::size_of::<u32>() as i32;
        buf = (buf as *mut u8).add(core::mem::size_of::<u32>()) as *mut core::ffi::c_void;
        max -= core::mem::size_of::<u32>();
    }
    if retval != 0 || !wait { retval } else { -5 }
}

// The remaining probe, runtime-PM, device-match, driver-registration, and module
// declarations require Linux kernel framework types and macros supplied externally.
// Their C control flow is represented by the declarations below.

#[allow(dead_code)]
unsafe fn mtk_rng_runtime_suspend(dev: *mut Device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut MtkRng;
    mtk_rng_cleanup(&mut (*priv_).rng);
    0
}

#[allow(dead_code)]
unsafe fn mtk_rng_runtime_resume(dev: *mut Device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut MtkRng;
    mtk_rng_init(&mut (*priv_).rng)
}

// Framework operations below are external kernel dependencies; these declarations
// preserve the original probe's externally visible entry point and data table.
extern "C" {
    fn mtk_rng_probe(pdev: *mut PlatformDevice) -> i32;
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
}

#[repr(C)]
struct DevPmOps {
    runtime_suspend: Option<unsafe extern "C" fn(*mut Device) -> i32>,
    runtime_resume: Option<unsafe extern "C" fn(*mut Device) -> i32>,
}

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    name: *const core::ffi::c_char,
    pm: *const DevPmOps,
    of_match_table: *const OfDeviceId,
}

// C's CONFIG_PM branches select either the runtime PM operations or NULL.
#[cfg(feature = "CONFIG_PM")]
static MTK_RNG_PM_OPS: DevPmOps = DevPmOps {
    runtime_suspend: Some(mtk_rng_runtime_suspend),
    runtime_resume: Some(mtk_rng_runtime_resume),
};

#[cfg(not(feature = "CONFIG_PM"))]
static MTK_RNG_PM_OPS: *const DevPmOps = core::ptr::null();

// Equivalent of the const-compatible device match table.  The terminating entry
// has null fields, as in the original C initializer.
static MTK_RNG_MATCH: [OfDeviceId; 6] = [
    OfDeviceId { compatible: b"mediatek,mt7623-rng\0".as_ptr() as *const _, data: core::ptr::null() },
    OfDeviceId { compatible: b"mediatek,mt7981-rng\0".as_ptr() as *const _, data: MTK_RNG_SMC as *const core::ffi::c_void },
    OfDeviceId { compatible: b"mediatek,mt7986-rng\0".as_ptr() as *const _, data: core::ptr::null() },
    OfDeviceId { compatible: b"mediatek,mt7987-rng\0".as_ptr() as *const _, data: MTK_RNG_SMC as *const core::ffi::c_void },
    OfDeviceId { compatible: b"mediatek,mt7988-rng\0".as_ptr() as *const _, data: MTK_RNG_SMC as *const core::ffi::c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

// `module_platform_driver(mtk_rng_driver)` registers this driver at module load.
static MTK_RNG_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(mtk_rng_probe),
    name: b"MTK_RNG_DEV\0".as_ptr() as *const _,
    pm: core::ptr::null(), // MTK_RNG_PM_OPS; selected by CONFIG_PM in the kernel build.
    of_match_table: MTK_RNG_MATCH.as_ptr(),
};

// MODULE_DESCRIPTION("Mediatek Random Number Generator Driver");
// MODULE_AUTHOR("Sean Wang <sean.wang@mediatek.com>");
// MODULE_AUTHOR("Daniel Golle <daniel@makrotopia.org>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
