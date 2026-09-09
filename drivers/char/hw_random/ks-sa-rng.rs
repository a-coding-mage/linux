// SPDX-License-Identifier: GPL-2.0-only
/*
 * Random Number Generator driver for the Keystone SOC
 *
 * Copyright (C) 2016 Texas Instruments Incorporated - https://www.ti.com
 *
 * Authors: Sandeep Nair
 *          Vitaly Andrianov
 */

// Linux kernel dependencies supplied by other translation units.

const SA_CMD_STATUS_OFS: u32 = 0x8;
const SA_CMD_STATUS_REG_TRNG_ENABLE: u32 = 1 << 3;
const TRNG_CNTL_REG_TRNG_ENABLE: u32 = 1 << 10;
const TRNG_STATUS_REG_READY: u32 = 1 << 0;
const TRNG_INTACK_REG_READY: u32 = 1 << 0;
const TRNG_DEF_STARTUP_CYCLES: u32 = 0;
const TRNG_CNTL_REG_STARTUP_CYCLES_SHIFT: u32 = 16;
const TRNG_DEF_MIN_REFILL_CYCLES: u32 = 1;
const TRNG_CFG_REG_MIN_REFILL_CYCLES_SHIFT: u32 = 0;
const TRNG_DEF_MAX_REFILL_CYCLES: u32 = 0;
const TRNG_CFG_REG_MAX_REFILL_CYCLES_SHIFT: u32 = 16;
const TRNG_DEF_CLK_DIV_CYCLES: u32 = 0;
const TRNG_CFG_REG_SAMPLE_DIV_SHIFT: u32 = 8;
const SA_MAX_RNG_DATA_RETRIES: i32 = 5;
const SA_RNG_DATA_RETRY_DELAY: u32 = 5;

#[repr(C)]
pub struct trng_regs {
    pub output_l: u32,
    pub output_h: u32,
    pub status: u32,
    pub intmask: u32,
    pub intack: u32,
    pub control: u32,
    pub config: u32,
}

#[repr(C)]
pub struct ks_sa_rng {
    pub rng: hwrng,
    pub clk: *mut clk,
    pub regmap_cfg: *mut regmap,
    pub reg_rng: *mut trng_regs,
    pub ready_ts: u64,
    pub refill_delay_ns: u32,
}

unsafe fn cycles_to_ns(clk_rate: c_ulong, cycles: u32) -> u32 {
    div_round_up_ull(
        (TRNG_DEF_CLK_DIV_CYCLES as u64 + 1) * 1_000_000_000u64 * cycles as u64,
        clk_rate as u64,
    ) as u32
}

unsafe fn startup_delay_ns(clk_rate: c_ulong) -> u32 {
    if TRNG_DEF_STARTUP_CYCLES == 0 {
        cycles_to_ns(clk_rate, 1 << 24)
    } else {
        cycles_to_ns(clk_rate, 256 * TRNG_DEF_STARTUP_CYCLES)
    }
}

unsafe fn refill_delay_ns(clk_rate: c_ulong) -> u32 {
    if TRNG_DEF_MAX_REFILL_CYCLES == 0 {
        cycles_to_ns(clk_rate, 1 << 24)
    } else {
        cycles_to_ns(clk_rate, 256 * TRNG_DEF_MAX_REFILL_CYCLES)
    }
}

pub unsafe extern "C" fn ks_sa_rng_init(rng: *mut hwrng) -> i32 {
    let ks_sa_rng = container_of_hwrng(rng);
    let clk_rate = clk_get_rate((*ks_sa_rng).clk);

    regmap_write_bits((*ks_sa_rng).regmap_cfg, SA_CMD_STATUS_OFS,
                      SA_CMD_STATUS_REG_TRNG_ENABLE,
                      SA_CMD_STATUS_REG_TRNG_ENABLE);

    writel(0, &mut (*(*ks_sa_rng).reg_rng).control);
    let mut value = TRNG_DEF_STARTUP_CYCLES << TRNG_CNTL_REG_STARTUP_CYCLES_SHIFT;
    writel(value, &mut (*(*ks_sa_rng).reg_rng).control);

    value = (TRNG_DEF_MIN_REFILL_CYCLES << TRNG_CFG_REG_MIN_REFILL_CYCLES_SHIFT)
        | (TRNG_DEF_MAX_REFILL_CYCLES << TRNG_CFG_REG_MAX_REFILL_CYCLES_SHIFT)
        | (TRNG_DEF_CLK_DIV_CYCLES << TRNG_CFG_REG_SAMPLE_DIV_SHIFT);
    writel(value, &mut (*(*ks_sa_rng).reg_rng).config);
    writel(0, &mut (*(*ks_sa_rng).reg_rng).intmask);

    value = readl(&(*(*ks_sa_rng).reg_rng).control);
    value |= TRNG_CNTL_REG_TRNG_ENABLE;
    writel(value, &mut (*(*ks_sa_rng).reg_rng).control);

    (*ks_sa_rng).refill_delay_ns = refill_delay_ns(clk_rate);
    (*ks_sa_rng).ready_ts = ktime_get_ns() + startup_delay_ns(clk_rate) as u64;
    0
}

pub unsafe extern "C" fn ks_sa_rng_cleanup(rng: *mut hwrng) {
    let ks_sa_rng = container_of_hwrng(rng);
    writel(0, &mut (*(*ks_sa_rng).reg_rng).control);
    regmap_write_bits((*ks_sa_rng).regmap_cfg, SA_CMD_STATUS_OFS,
                      SA_CMD_STATUS_REG_TRNG_ENABLE, 0);
}

pub unsafe extern "C" fn ks_sa_rng_data_read(rng: *mut hwrng, data: *mut u32) -> i32 {
    let ks_sa_rng = container_of_hwrng(rng);
    *data = readl(&(*(*ks_sa_rng).reg_rng).output_l);
    *data.add(1) = readl(&(*(*ks_sa_rng).reg_rng).output_h);
    writel(TRNG_INTACK_REG_READY, &mut (*(*ks_sa_rng).reg_rng).intack);
    (*ks_sa_rng).ready_ts = ktime_get_ns() + (*ks_sa_rng).refill_delay_ns as u64;
    (core::mem::size_of::<u32>() * 2) as i32
}

pub unsafe extern "C" fn ks_sa_rng_data_present(rng: *mut hwrng, wait: i32) -> i32 {
    let ks_sa_rng = container_of_hwrng(rng);
    let now = ktime_get_ns();
    let mut ready: u32 = 0;
    if wait != 0 && now < (*ks_sa_rng).ready_ts {
        let min_delay = div_round_up(((*ks_sa_rng).ready_ts - now) as u32, 1000);
        usleep_range(min_delay, min_delay + SA_RNG_DATA_RETRY_DELAY);
    }
    let mut j = 0;
    while j < SA_MAX_RNG_DATA_RETRIES {
        ready = readl(&(*(*ks_sa_rng).reg_rng).status) & TRNG_STATUS_REG_READY;
        if ready != 0 || wait == 0 { break; }
        udelay(SA_RNG_DATA_RETRY_DELAY);
        j += 1;
    }
    ready as i32
}

pub unsafe extern "C" fn ks_sa_rng_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let ks_sa_rng = devm_kzalloc(dev, core::mem::size_of::<ks_sa_rng>(), GFP_KERNEL);
    if ks_sa_rng.is_null() { return -12; }

    (*ks_sa_rng).rng = hwrng_init(
        b"ks_sa_hwrng\0".as_ptr(),
        ks_sa_rng_init,
        ks_sa_rng_data_read,
        ks_sa_rng_data_present,
        ks_sa_rng_cleanup,
    );

    (*ks_sa_rng).reg_rng = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*ks_sa_rng).reg_rng as *mut core::ffi::c_void) {
        return ptr_err((*ks_sa_rng).reg_rng as *mut core::ffi::c_void);
    }
    (*ks_sa_rng).regmap_cfg = syscon_regmap_lookup_by_phandle(
        &mut (*dev).of_node, b"ti,syscon-sa-cfg\0".as_ptr());
    if is_err((*ks_sa_rng).regmap_cfg as *mut core::ffi::c_void) {
        return dev_err_probe(dev, -22, b"syscon_node_to_regmap failed\n\0".as_ptr());
    }
    (*ks_sa_rng).clk = devm_clk_get_enabled(dev, core::ptr::null());
    if is_err((*ks_sa_rng).clk as *mut core::ffi::c_void) {
        return dev_err_probe(dev, ptr_err((*ks_sa_rng).clk as *mut core::ffi::c_void),
                             b"Failed to get clock\n\0".as_ptr());
    }
    pm_runtime_enable(dev);
    let ret = pm_runtime_resume_and_get(dev);
    if ret < 0 {
        pm_runtime_disable(dev);
        return dev_err_probe(dev, ret, b"Failed to enable SA power-domain\n\0".as_ptr());
    }
    let ret = devm_hwrng_register(dev, &mut (*ks_sa_rng).rng);
    if ret != 0 {
        pm_runtime_put_sync(dev);
        pm_runtime_disable(dev);
        return ret;
    }
    0
}

#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
static KS_SA_RNG_DT_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"ti,keystone-rng\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)] pub struct driver { pub name: *const u8, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct platform_driver { pub driver: driver, pub probe: unsafe extern "C" fn(*mut platform_device) -> i32, pub remove: unsafe extern "C" fn(*mut platform_device) }
static mut KS_SA_RNG_DRIVER: platform_driver = platform_driver {
    driver: driver { name: b"ks-sa-rng\0".as_ptr(), of_match_table: KS_SA_RNG_DT_MATCH.as_ptr() },
    probe: ks_sa_rng_probe,
    remove: ks_sa_rng_remove,
};

// module_platform_driver(ks_sa_rng_driver);
// MODULE_DESCRIPTION("Keystone NETCP SA H/W Random Number Generator driver");
// MODULE_AUTHOR("Vitaly Andrianov <vitalya@ti.com>");
// MODULE_LICENSE("GPL");

// Probe, remove, device matching, and module registration retain their Linux
// kernel interfaces; the referenced kernel types and helpers are external.
pub unsafe extern "C" fn ks_sa_rng_remove(pdev: *mut platform_device) {
    pm_runtime_put_sync(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
}

extern "C" {
    fn container_of_hwrng(rng: *mut hwrng) -> *mut ks_sa_rng;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn div_round_up_ull(n: u64, d: u64) -> u64;
    fn div_round_up(n: u32, d: u32) -> u32;
    fn regmap_write_bits(map: *mut regmap, reg: u32, mask: u32, val: u32);
    fn writel(val: u32, addr: *mut u32);
    fn readl(addr: *const u32) -> u32;
    fn ktime_get_ns() -> u64;
    fn usleep_range(min: u32, max: u32);
    fn udelay(usecs: u32);
    fn pm_runtime_put_sync(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut ks_sa_rng;
    fn hwrng_init(name: *const u8, init: unsafe extern "C" fn(*mut hwrng) -> i32,
                  data_read: unsafe extern "C" fn(*mut hwrng, *mut u32) -> i32,
                  data_present: unsafe extern "C" fn(*mut hwrng, i32) -> i32,
                  cleanup: unsafe extern "C" fn(*mut hwrng));
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut trng_regs;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn syscon_regmap_lookup_by_phandle(node: *mut *mut core::ffi::c_void, name: *const u8) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: i32, msg: *const u8) -> i32;
    fn devm_clk_get_enabled(dev: *mut device, id: *const u8) -> *mut clk;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_resume_and_get(dev: *mut device) -> i32;
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> i32;
}

use core::ffi::c_ulong;
#[allow(non_camel_case_types)] pub enum hwrng {}
#[allow(non_camel_case_types)] pub enum clk {}
#[allow(non_camel_case_types)] pub enum regmap {}
#[allow(non_camel_case_types)] pub struct device { pub of_node: *mut core::ffi::c_void, _private: [u8; 0] }
#[allow(non_camel_case_types)] pub struct platform_device { pub dev: device }
const GFP_KERNEL: u32 = 0x08;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
