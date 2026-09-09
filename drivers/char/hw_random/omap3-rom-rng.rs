/*
 * omap3-rom-rng.c - RNG driver for TI OMAP3 CPU family
 *
 * Copyright (C) 2009 Nokia Corporation
 * Author: Juha Yrjola <juha.yrjola@solidboot.com>
 *
 * Copyright (C) 2013 Pali Rohár <pali@kernel.org>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_void;

type U32 = u32;
type SizeT = usize;

#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut c_void,
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

pub type RomRngCall = unsafe extern "C" fn(ptr: U32, count: U32, flag: U32) -> U32;
pub type HwrngRead = unsafe extern "C" fn(rng: *mut Hwrng, data: *mut c_void, max: SizeT, wait: bool) -> i32;

#[repr(C)]
pub struct Hwrng {
    pub priv_: u64,
    pub name: *const u8,
    pub read: Option<HwrngRead>,
    pub quality: u32,
}

#[repr(C)]
pub struct OmapRomRng {
    pub clk: *mut Clk,
    pub dev: *mut Device,
    pub ops: Hwrng,
    pub rom_rng_call: Option<RomRngCall>,
}

const RNG_RESET: U32 = 0x01;
const RNG_GEN_PRNG_HW_INIT: U32 = 0x02;
const RNG_GEN_HW: U32 = 0x08;

extern "C" {
    fn pm_runtime_get_sync(dev: *mut Device) -> i32;
    fn pm_runtime_put_noidle(dev: *mut Device);
    fn virt_to_phys(addr: *mut c_void) -> U32;
    fn pm_runtime_put_autosuspend(dev: *mut Device);
    fn dev_get_drvdata(dev: *mut Device) -> *mut OmapRomRng;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn clk_prepare_enable(clk: *mut Clk) -> i32;
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn pm_runtime_dont_use_autosuspend(dev: *mut Device);
    fn pm_runtime_disable(dev: *mut Device);
    fn devm_kzalloc(dev: *mut Device, size: SizeT, flags: u32) -> *mut OmapRomRng;
    fn of_device_get_match_data(dev: *mut Device) -> Option<HwrngRead>;
    fn dev_set_drvdata(dev: *mut Device, data: *mut OmapRomRng);
    fn devm_clk_get(dev: *mut Device, name: *const u8) -> *mut Clk;
    fn is_err<T>(ptr: *mut T) -> bool;
    fn ptr_err<T>(ptr: *mut T) -> i32;
    fn pm_runtime_enable(dev: *mut Device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut Device, delay: i32);
    fn pm_runtime_use_autosuspend(dev: *mut Device);
    fn devm_add_action_or_reset(dev: *mut Device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> i32;
    fn devm_hwrng_register(dev: *mut Device, rng: *mut Hwrng) -> i32;
}

unsafe extern "C" fn omap3_rom_rng_read(rng: *mut Hwrng, data: *mut c_void, _max: SizeT, _w: bool) -> i32 {
    let ddata = (*rng).priv_ as *mut OmapRomRng;
    let mut r = pm_runtime_get_sync((*ddata).dev);
    if r < 0 {
        pm_runtime_put_noidle((*ddata).dev);
        return r;
    }

    let ptr = virt_to_phys(data);
    r = ((*ddata).rom_rng_call.unwrap())(ptr, 4, RNG_GEN_HW) as i32;
    if r != 0 { r = -22; } else { r = 4; }
    pm_runtime_put_autosuspend((*ddata).dev);
    r
}

unsafe extern "C" fn omap_rom_rng_runtime_suspend(dev: *mut Device) -> i32 {
    let ddata = dev_get_drvdata(dev);
    let r = ((*ddata).rom_rng_call.unwrap())(0, 0, RNG_RESET);
    if r != 0 { dev_err(dev, b"reset failed: %d\0".as_ptr(), r); }
    clk_disable_unprepare((*ddata).clk);
    0
}

unsafe extern "C" fn omap_rom_rng_runtime_resume(dev: *mut Device) -> i32 {
    let ddata = dev_get_drvdata(dev);
    let mut r = clk_prepare_enable((*ddata).clk);
    if r < 0 { return r; }
    r = ((*ddata).rom_rng_call.unwrap())(0, 0, RNG_GEN_PRNG_HW_INIT) as i32;
    if r != 0 {
        clk_disable_unprepare((*ddata).clk);
        dev_err(dev, b"HW init failed: %d\0".as_ptr(), r);
        return -5;
    }
    0
}

unsafe extern "C" fn omap_rom_rng_finish(data: *mut c_void) {
    let ddata = data as *mut OmapRomRng;
    pm_runtime_dont_use_autosuspend((*ddata).dev);
    pm_runtime_disable((*ddata).dev);
}

unsafe extern "C" fn omap3_rom_rng_probe(pdev: *mut PlatformDevice) -> i32 {
    let ddata = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<OmapRomRng>(), 0);
    if ddata.is_null() { return -12; }
    (*ddata).dev = &mut (*pdev).dev;
    (*ddata).ops.priv_ = ddata as u64;
    (*ddata).ops.name = b"omap3-rom\0".as_ptr();
    (*ddata).ops.read = of_device_get_match_data((*ddata).dev);
    (*ddata).ops.quality = 900;
    if (*ddata).ops.read.is_none() {
        dev_err((*pdev).dev_ptr(), b"missing rom code handler\n\0".as_ptr());
        return -19;
    }
    dev_set_drvdata((*ddata).dev, ddata);
    (*ddata).rom_rng_call = if (*pdev).dev.platform_data.is_null() {
        None
    } else {
        Some(*((*pdev).dev.platform_data as *const RomRngCall))
    };
    if (*ddata).rom_rng_call.is_none() {
        dev_err((*ddata).dev, b"rom_rng_call is NULL\n\0".as_ptr());
        return -22;
    }
    (*ddata).clk = devm_clk_get((*ddata).dev, b"ick\0".as_ptr());
    if is_err((*ddata).clk) {
        dev_err((*ddata).dev, b"unable to get RNG clock\n\0".as_ptr());
        return ptr_err((*ddata).clk);
    }
    pm_runtime_enable(&mut (*pdev).dev);
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, 500);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    let ret = devm_add_action_or_reset((*ddata).dev, omap_rom_rng_finish, ddata as *mut c_void);
    if ret != 0 { return ret; }
    devm_hwrng_register((*ddata).dev, &mut (*ddata).ops)
}

#[repr(C)]
pub struct OfDeviceId { pub compatible: *const u8, pub data: Option<HwrngRead> }

static OMAP_ROM_RNG_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"nokia,n900-rom-rng\0".as_ptr(), data: Some(omap3_rom_rng_read) },
    OfDeviceId { compatible: core::ptr::null(), data: None },
];

#[repr(C)]
pub struct DevPmOps {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut Device) -> i32>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut Device) -> i32>,
}

#[repr(C)]
pub struct PlatformDriver {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
    pub pm: *const DevPmOps,
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
}

static OMAP_ROM_RNG_PM_OPS: DevPmOps = DevPmOps {
    runtime_suspend: Some(omap_rom_rng_runtime_suspend),
    runtime_resume: Some(omap_rom_rng_runtime_resume),
};

static mut OMAP3_ROM_RNG_DRIVER: PlatformDriver = PlatformDriver {
    name: b"omap3-rom-rng\0".as_ptr(),
    of_match_table: OMAP_ROM_RNG_MATCH.as_ptr(),
    pm: &OMAP_ROM_RNG_PM_OPS,
    probe: Some(omap3_rom_rng_probe),
};

// Equivalent to module_platform_driver(omap3_rom_rng_driver).
// MODULE_ALIAS("platform:omap3-rom-rng");
// MODULE_AUTHOR("Juha Yrjola");
// MODULE_AUTHOR("Pali Rohár <pali@kernel.org>");
// MODULE_DESCRIPTION("RNG driver for TI OMAP3 CPU family");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
