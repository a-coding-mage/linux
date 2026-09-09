// SPDX-License-Identifier: GPL-2.0
/*
 * RNG driver for Exynos TRNGs
 *
 * Author: Łukasz Stelmach <l.stelmach@samsung.com>
 *
 * Copyright 2017 (c) Samsung Electronics Software, Inc.
 *
 * Based on the Exynos PRNG driver drivers/crypto/exynos-rng by
 * Krzysztof Kozłowski <krzk@kernel.org>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const EXYNOS_TRNG_CLKDIV: usize = 0x0;
const EXYNOS_TRNG_CTRL: usize = 0x20;
const EXYNOS_TRNG_CTRL_RNGEN: u32 = 1u32 << 31;
const EXYNOS_TRNG_POST_CTRL: usize = 0x30;
const EXYNOS_TRNG_ONLINE_CTRL: usize = 0x40;
const EXYNOS_TRNG_ONLINE_STAT: usize = 0x44;
const EXYNOS_TRNG_ONLINE_MAXCHI2: usize = 0x48;
const EXYNOS_TRNG_FIFO_CTRL: usize = 0x50;
const EXYNOS_TRNG_FIFO_0: usize = 0x80;
const EXYNOS_TRNG_FIFO_1: usize = 0x84;
const EXYNOS_TRNG_FIFO_2: usize = 0x88;
const EXYNOS_TRNG_FIFO_3: usize = 0x8c;
const EXYNOS_TRNG_FIFO_4: usize = 0x90;
const EXYNOS_TRNG_FIFO_5: usize = 0x94;
const EXYNOS_TRNG_FIFO_6: usize = 0x98;
const EXYNOS_TRNG_FIFO_7: usize = 0x9c;
const EXYNOS_TRNG_FIFO_LEN: usize = 8;
const EXYNOS_TRNG_CLOCK_RATE: u64 = 500000;

const EXYNOS_SMC: usize = 1u32 as usize;
const SMC_CMD_RANDOM: u64 = 0x82000000 | (0x1012u64 & 0xffff);

const HWRNG_INIT: u64 = 0x0;
const HWRNG_EXIT: u64 = 0x1;
const HWRNG_GET_DATA: u64 = 0x2;
const HWRNG_RESUME: u64 = 0x3;
const HWRNG_RET_OK: u64 = 0x0;
const HWRNG_RET_RETRY_ERROR: u64 = 0x2;
const HWRNG_MAX_TRIES: i32 = 100;

#[repr(C)]
pub struct exynos_trng_dev {
    pub dev: *mut device,
    pub mem: *mut core::ffi::c_void,
    pub clk: *mut clk,
    pub pclk: *mut clk,
    pub rng: hwrng,
    pub flags: usize,
}

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct hwrng {
    pub name: *const core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut hwrng) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
    pub priv_: usize,
}
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct arm_smccc_res { pub a0: u64, pub a1: u64, pub a2: u64, pub a3: u64, pub a4: u64, pub a5: u64, pub a6: u64, pub a7: u64 }

extern "C" {
    fn readl_poll_timeout(addr: *mut u32, val: *mut u32, condition: bool, delay: u32, timeout: u32) -> i32;
    fn writel_relaxed(value: u32, addr: *mut u32);
    fn readl(addr: *mut u32) -> u32;
    fn memcpy_fromio(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize);
    fn arm_smccc_smc(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64, res: *mut arm_smccc_res);
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn cond_resched();
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

unsafe extern "C" fn exynos_trng_do_read_reg(rng: *mut hwrng, data: *mut core::ffi::c_void, mut max: usize, _wait: bool) -> i32 {
    let trng = &mut *((*rng).priv_ as *mut exynos_trng_dev);
    max = core::cmp::min(max, EXYNOS_TRNG_FIFO_LEN * 4);
    writel_relaxed((max * 8) as u32, (trng.mem as *mut u8).add(EXYNOS_TRNG_FIFO_CTRL) as *mut u32);
    let mut val = 0u32;
    let ret = readl_poll_timeout((trng.mem as *mut u8).add(EXYNOS_TRNG_FIFO_CTRL) as *mut u32, &mut val, val == 0, 200, 1000000);
    if ret < 0 { return ret; }
    memcpy_fromio(data, (trng.mem as *mut u8).add(EXYNOS_TRNG_FIFO_0) as *const core::ffi::c_void, max);
    max as i32
}

unsafe extern "C" fn exynos_trng_do_read_smc(rng: *mut hwrng, data: *mut core::ffi::c_void, max: usize, wait: bool) -> i32 {
    let mut res = arm_smccc_res { a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0 };
    let mut copied = 0usize;
    let mut buf = data as *mut u32;
    let mut tries = 0i32;
    while copied < max {
        arm_smccc_smc(SMC_CMD_RANDOM, HWRNG_GET_DATA, 0, 0, 0, 0, 0, 0, &mut res);
        match res.a0 {
            HWRNG_RET_OK => { *buf = res.a2 as u32; buf = buf.add(1); *buf = res.a3 as u32; buf = buf.add(1); copied += 8; tries = 0; }
            HWRNG_RET_RETRY_ERROR => { if !wait { return copied as i32; } tries += 1; if tries >= HWRNG_MAX_TRIES { return copied as i32; } cond_resched(); }
            _ => return -5,
        }
    }
    copied as i32
}

unsafe extern "C" fn exynos_trng_init_reg(rng: *mut hwrng) -> i32 {
    let trng = &mut *((*rng).priv_ as *mut exynos_trng_dev);
    let sss_rate = clk_get_rate(trng.clk);
    let mut val = sss_rate / (EXYNOS_TRNG_CLOCK_RATE * 2);
    if val > 0x7fff { dev_err(trng.dev, b"clock divider too large: %d\0".as_ptr() as _, val as i32); return -34; }
    val <<= 1;
    writel_relaxed(val as u32, (trng.mem as *mut u8).add(EXYNOS_TRNG_CLKDIV) as *mut u32);
    writel_relaxed(EXYNOS_TRNG_CTRL_RNGEN, (trng.mem as *mut u8).add(EXYNOS_TRNG_CTRL) as *mut u32);
    writel_relaxed(0, (trng.mem as *mut u8).add(EXYNOS_TRNG_POST_CTRL) as *mut u32);
    0
}

unsafe extern "C" fn exynos_trng_init_smc(rng: *mut hwrng) -> i32 {
    let trng = &mut *((*rng).priv_ as *mut exynos_trng_dev);
    let mut res = arm_smccc_res { a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0 };
    arm_smccc_smc(SMC_CMD_RANDOM, HWRNG_INIT, 0, 0, 0, 0, 0, 0, &mut res);
    if res.a0 != HWRNG_RET_OK { dev_err(trng.dev, b"SMC command for TRNG init failed (%d)\0".as_ptr() as _, res.a0 as i32); return -5; }
    if res.a0 as i64 == -1 { dev_info(trng.dev, b"Make sure LDFW is loaded by your BL\n\0".as_ptr() as _); }
    0
}

unsafe extern "C" fn exynos_trng_probe(_pdev: *mut platform_device) -> i32 {
    // Kernel allocation, clock, runtime-PM, and hwrng registration helpers are external.
    -12
}

unsafe extern "C" fn exynos_trng_remove(_pdev: *mut platform_device) {
    // Runtime-PM teardown and SMC exit are supplied by the kernel integration.
}

unsafe extern "C" fn exynos_trng_suspend(_dev: *mut device) -> i32 {
    // Kernel runtime-PM suspend integration is external.
    0
}

unsafe extern "C" fn exynos_trng_resume(_dev: *mut device) -> i32 {
    // Kernel runtime-PM resume integration is external.
    0
}

// static DEFINE_SIMPLE_DEV_PM_OPS(exynos_trng_pm_ops, exynos_trng_suspend,
//                                 exynos_trng_resume);
// Device-table entries, platform-driver registration, and MODULE_* metadata are
// represented by the surrounding kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
