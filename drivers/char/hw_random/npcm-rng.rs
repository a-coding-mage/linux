// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Nuvoton Technology corporation.

// Dependencies supplied by the kernel environment:
// linux/kernel.h, linux/module.h, linux/io.h, linux/iopoll.h, linux/init.h,
// linux/random.h, linux/err.h, linux/of.h, linux/platform_device.h,
// linux/hw_random.h, linux/delay.h, linux/pm_runtime.h

const NPCM_RNGCS_REG: usize = 0x00; // Control and status register
const NPCM_RNGD_REG: usize = 0x04; // Data register
const NPCM_RNGMODE_REG: usize = 0x08; // Mode register

const NPCM_RNG_CLK_SET_62_5MHZ: u32 = 1 << 2; // 60-80 MHz
const NPCM_RNG_CLK_SET_25MHZ: u32 = (0xf << 3) & (0x1f << 3); // 20-25 MHz
const NPCM_RNG_DATA_VALID: u32 = 1 << 1;
const NPCM_RNG_ENABLE: u32 = 1 << 0;
const NPCM_RNG_M1ROSEL: u32 = 1 << 1;

const NPCM_RNG_TIMEOUT_USEC: u32 = 20000;
const NPCM_RNG_POLL_USEC: u32 = 1000;

#[repr(C)]
pub struct npcm_rng {
    pub base: *mut core::ffi::c_void,
    pub rng: hwrng,
    pub dev: *mut device,
    pub clkp: u32,
}

extern "C" {
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn readb(address: *mut core::ffi::c_void) -> u8;
    fn readb_poll_timeout(
        address: *mut core::ffi::c_void,
        value: *mut u8,
        condition: bool,
        delay_us: u32,
        timeout_us: u32,
    ) -> i32;
    fn pm_runtime_get_sync(dev: *mut device) -> i32;
    fn pm_runtime_put_sync_autosuspend(dev: *mut device) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: i32);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn of_device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> i32;
    fn devm_hwrng_unregister(dev: *mut device, rng: *mut hwrng);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn EIO() -> i32;
    fn ENOMEM() -> i32;
}

#[repr(C)]
pub struct hwrng {
    pub name: *const core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut hwrng) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut hwrng)>,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
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

unsafe fn npcm_rng_init(rng: *mut hwrng) -> i32 {
    let priv_ = (rng as *mut u8).sub(core::mem::offset_of!(npcm_rng, rng)) as *mut npcm_rng;
    writel((*priv_).clkp | NPCM_RNG_ENABLE, (*priv_).base.add(NPCM_RNGCS_REG));
    0
}

unsafe fn npcm_rng_cleanup(rng: *mut hwrng) {
    let priv_ = (rng as *mut u8).sub(core::mem::offset_of!(npcm_rng, rng)) as *mut npcm_rng;
    writel((*priv_).clkp, (*priv_).base.add(NPCM_RNGCS_REG));
}

unsafe fn npcm_rng_read(
    rng: *mut hwrng,
    mut buf: *mut core::ffi::c_void,
    mut max: usize,
    wait: bool,
) -> i32 {
    let priv_ = (rng as *mut u8).sub(core::mem::offset_of!(npcm_rng, rng)) as *mut npcm_rng;
    let mut retval: i32 = 0;
    let mut ready: u8 = 0;

    pm_runtime_get_sync((*priv_).dev);
    while max != 0 {
        if wait {
            if readb_poll_timeout(
                (*priv_).base.add(NPCM_RNGCS_REG),
                &mut ready,
                (ready as u32 & NPCM_RNG_DATA_VALID) != 0,
                NPCM_RNG_POLL_USEC,
                NPCM_RNG_TIMEOUT_USEC,
            ) != 0 {
                break;
            }
        } else if (readb((*priv_).base.add(NPCM_RNGCS_REG)) as u32 & NPCM_RNG_DATA_VALID) == 0 {
            break;
        }

        *(buf as *mut u8) = readb((*priv_).base.add(NPCM_RNGD_REG));
        retval += 1;
        buf = buf.add(1);
        max -= 1;
    }

    pm_runtime_put_sync_autosuspend((*priv_).dev);
    if retval != 0 || !wait { retval } else { -EIO() }
}

unsafe fn npcm_rng_probe(pdev: *mut platform_device) -> i32 {
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<npcm_rng>(), 0) as *mut npcm_rng;
    if priv_.is_null() { return -ENOMEM(); }
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut core::ffi::c_void);
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, 100);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    (*priv_).rng.init = Some(npcm_rng_init);
    (*priv_).rng.cleanup = Some(npcm_rng_cleanup);
    (*priv_).rng.name = (*pdev).name;
    (*priv_).rng.read = Some(npcm_rng_read);
    (*priv_).dev = &mut (*pdev).dev;
    (*priv_).clkp = of_device_get_match_data(&mut (*pdev).dev) as usize as u32;
    writel(NPCM_RNG_M1ROSEL, (*priv_).base.add(NPCM_RNGMODE_REG));
    let ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*priv_).rng);
    if ret != 0 {
        pm_runtime_disable(&mut (*pdev).dev);
        pm_runtime_set_suspended(&mut (*pdev).dev);
        return ret;
    }
    0
}

unsafe fn npcm_rng_remove(pdev: *mut platform_device) {
    let priv_ = platform_get_drvdata(pdev) as *mut npcm_rng;
    devm_hwrng_unregister(&mut (*pdev).dev, &mut (*priv_).rng);
    pm_runtime_disable(&mut (*pdev).dev);
    pm_runtime_set_suspended(&mut (*pdev).dev);
}

unsafe fn npcm_rng_runtime_suspend(dev: *mut device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut npcm_rng;
    npcm_rng_cleanup(&mut (*priv_).rng);
    0
}

unsafe fn npcm_rng_runtime_resume(dev: *mut device) -> i32 {
    let priv_ = dev_get_drvdata(dev) as *mut npcm_rng;
    npcm_rng_init(&mut (*priv_).rng)
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[no_mangle]
pub static rng_dt_id: [of_device_id; 3] = [
    of_device_id { compatible: b"nuvoton,npcm750-rng\0".as_ptr() as *const _, data: NPCM_RNG_CLK_SET_25MHZ as usize as *const _ },
    of_device_id { compatible: b"nuvoton,npcm845-rng\0".as_ptr() as *const _, data: NPCM_RNG_CLK_SET_62_5MHZ as usize as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[no_mangle]
pub static npcm_rng_driver: platform_driver = platform_driver {
    probe: Some(npcm_rng_probe),
    remove: Some(npcm_rng_remove),
};

// module_platform_driver(npcm_rng_driver)
// MODULE_DEVICE_TABLE(of, rng_dt_id)
// MODULE_DESCRIPTION("Nuvoton NPCM Random Number Generator Driver")
// MODULE_AUTHOR("Tomer Maimon <tomer.maimon@nuvoton.com>")
// MODULE_LICENSE("GPL v2")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
