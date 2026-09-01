// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

/* C dependencies:
 * linux/device.h, linux/err.h, linux/module.h, linux/pm.h,
 * linux/pm_runtime.h, linux/regmap.h, linux/soundwire/sdw.h,
 * linux/soundwire/sdw_registers.h, linux/soundwire/sdw_type.h,
 * sound/sdca.h, sound/sdca_function.h, sound/sdca_interrupts.h,
 * sound/sdca_regmap.h, and sdca_class.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const CLASS_SDW_ATTACH_TIMEOUT_MS: c_int = 5000;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const REGCACHE_MAPLE: c_uint = 0;

const SDW_SCP_INT1_BUS_CLASH: c_uint = 0;
const SDW_SCP_INT1_PARITY: c_uint = 0;
const SDW_SCP_INT1_IMPL_DEF: c_uint = 0;
const SDW_SCP_SDCA_INT1: c_uint = 0;
const SDW_SCP_SDCA_INT4: c_uint = 0;
const SDW_SCP_SDCA_INTMASK1: c_uint = 0;
const SDW_SCP_SDCA_INTMASK4: c_uint = 0;
const SDW_SDCA_MAX_REGISTER: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdca_irq_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub use_domain_irq: bool,
    pub scp_int1_mask: c_uint,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub irq: c_int,
}

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub class_id: c_uint,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub id_table: *const sdw_device_id,
    pub ops: *const sdw_slave_ops,
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub lock: Option<unsafe extern "C" fn(*mut c_void)>,
    pub unlock: Option<unsafe extern "C" fn(*mut c_void)>,
    pub lock_arg: *mut c_void,
}

#[repr(C)]
pub struct sdca_class_drv {
    pub dev: *mut device,
    pub sdw: *mut sdw_slave,
    pub regmap_lock: mutex,
    pub init_lock: mutex,
    pub boot_work: work_struct,
    pub dev_regmap: *mut regmap,
    pub irq_info: *mut sdca_irq_info,
}

unsafe extern "C" {
    static mut system_long_wq: *mut workqueue_struct;

    fn sdw_slave_read_prop(sdw: *mut sdw_slave);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn sdca_dev_unregister_functions(sdw: *mut sdw_slave);
    fn sdw_slave_wait_for_init(sdw: *mut sdw_slave, timeout_ms: c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn devm_sdca_irq_allocate(
        dev: *mut device,
        map: *mut regmap,
        irq: c_int,
    ) -> *mut sdca_irq_info;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn sdca_dev_register_functions(sdw: *mut sdw_slave) -> c_int;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_put_sync(dev: *mut device);
    fn sdca_lookup_swft(sdw: *mut sdw_slave);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kmemdup(
        dev: *mut device,
        src: *const c_void,
        len: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn devm_regmap_init_sdw(sdw: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn disable_irq(irq: c_int);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn enable_irq(irq: c_int);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn module_sdw_driver(driver: *mut sdw_driver);
}

unsafe extern "C" fn class_read_prop(sdw: *mut sdw_slave) -> c_int {
    let prop: *mut sdw_slave_prop = unsafe { &mut (*sdw).prop };

    unsafe {
        sdw_slave_read_prop(sdw);

        (*prop).use_domain_irq = true;
        (*prop).scp_int1_mask =
            SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY | SDW_SCP_INT1_IMPL_DEF;
    }

    0
}

static CLASS_SDW_OPS: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(class_read_prop),
};

unsafe extern "C" fn class_regmap_lock(data: *mut c_void) {
    let lock: *mut mutex = data.cast();

    unsafe {
        mutex_lock(lock);
    }
}

unsafe extern "C" fn class_regmap_unlock(data: *mut c_void) {
    let lock: *mut mutex = data.cast();

    unsafe {
        mutex_unlock(lock);
    }
}

unsafe extern "C" fn class_dev_regmap_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SDW_SCP_SDCA_INTMASK1..=SDW_SCP_SDCA_INTMASK4 => false,
        _ => true,
    }
}

unsafe extern "C" fn class_dev_regmap_precious(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SDW_SCP_SDCA_INT1..=SDW_SCP_SDCA_INT4
        | SDW_SCP_SDCA_INTMASK1..=SDW_SCP_SDCA_INTMASK4 => false,
        _ => true,
    }
}

static CLASS_DEV_REGMAP_CONFIG: regmap_config = regmap_config {
    name: b"sdca-device\0".as_ptr().cast(),
    reg_bits: 32,
    val_bits: 8,
    max_register: SDW_SDCA_MAX_REGISTER,
    volatile_reg: Some(class_dev_regmap_volatile),
    precious_reg: Some(class_dev_regmap_precious),
    cache_type: REGCACHE_MAPLE,
    lock: Some(class_regmap_lock),
    unlock: Some(class_regmap_unlock),
    lock_arg: ptr::null_mut(),
};

unsafe extern "C" fn class_remove_functions(data: *mut c_void) {
    let drv: *mut sdca_class_drv = data.cast();

    unsafe {
        sdca_dev_unregister_functions((*drv).sdw);
    }
}

unsafe extern "C" fn class_boot_work(work: *mut work_struct) {
    let drv: *mut sdca_class_drv = work.cast();
    let mut ret: c_int;

    unsafe {
        ret = sdw_slave_wait_for_init((*drv).sdw, CLASS_SDW_ATTACH_TIMEOUT_MS);
        if ret != 0 {
            goto_err(drv);
            return;
        }

        regcache_cache_only((*drv).dev_regmap, false);

        (*drv).irq_info = devm_sdca_irq_allocate((*drv).dev, (*drv).dev_regmap, (*(*drv).sdw).irq);
        if IS_ERR((*drv).irq_info.cast()) {
            goto_err(drv);
            return;
        }

        ret = sdca_dev_register_functions((*drv).sdw);
        if ret != 0 {
            goto_err(drv);
            return;
        }

        /* Ensure function drivers are removed before the IRQ is destroyed */
        ret = devm_add_action_or_reset((*drv).dev, class_remove_functions, drv.cast());
        if ret != 0 {
            goto_err(drv);
            return;
        }

        dev_dbg((*drv).dev, b"boot work complete\n\0".as_ptr().cast());

        pm_runtime_mark_last_busy((*drv).dev);
        pm_runtime_put_autosuspend((*drv).dev);
    }
}

unsafe fn goto_err(drv: *mut sdca_class_drv) {
    unsafe {
        pm_runtime_put_sync((*drv).dev);
    }
}

unsafe extern "C" fn class_sdw_probe(
    sdw: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let dev: *mut device = unsafe { &mut (*sdw).dev };
    let dev_config: *mut regmap_config;
    let drv: *mut sdca_class_drv;
    let ret: c_int;

    unsafe {
        sdca_lookup_swft(sdw);

        drv = devm_kzalloc(dev, mem::size_of::<sdca_class_drv>(), GFP_KERNEL).cast();
        if drv.is_null() {
            return -ENOMEM;
        }

        dev_config = devm_kmemdup(
            dev,
            (&CLASS_DEV_REGMAP_CONFIG as *const regmap_config).cast(),
            mem::size_of::<regmap_config>(),
            GFP_KERNEL,
        )
        .cast();
        if dev_config.is_null() {
            return -ENOMEM;
        }

        (*drv).dev = dev;
        (*drv).sdw = sdw;
        mutex_init(&mut (*drv).regmap_lock);
        mutex_init(&mut (*drv).init_lock);

        dev_set_drvdata((*drv).dev, drv.cast());

        INIT_WORK(&mut (*drv).boot_work, class_boot_work);

        (*dev_config).lock_arg = (&mut (*drv).regmap_lock as *mut mutex).cast();

        (*drv).dev_regmap = devm_regmap_init_sdw(sdw, dev_config);
        if IS_ERR((*drv).dev_regmap.cast()) {
            return dev_err_probe(
                (*drv).dev,
                PTR_ERR((*drv).dev_regmap.cast()),
                b"failed to create device regmap\n\0".as_ptr().cast(),
            );
        }

        regcache_cache_only((*drv).dev_regmap, true);

        pm_runtime_set_autosuspend_delay(dev, 250);
        pm_runtime_use_autosuspend(dev);
        pm_runtime_set_active(dev);
        pm_runtime_get_noresume(dev);

        ret = devm_pm_runtime_enable(dev);
        if ret != 0 {
            return ret;
        }

        queue_work(system_long_wq, &mut (*drv).boot_work);
    }

    0
}

unsafe extern "C" fn class_sdw_remove(sdw: *mut sdw_slave) {
    let dev: *mut device = unsafe { &mut (*sdw).dev };
    let drv: *mut sdca_class_drv = unsafe { dev_get_drvdata(dev).cast() };

    unsafe {
        cancel_work_sync(&mut (*drv).boot_work);
    }
}

unsafe extern "C" fn class_suspend(dev: *mut device) -> c_int {
    let drv: *mut sdca_class_drv = unsafe { dev_get_drvdata(dev).cast() };
    let ret: c_int;

    unsafe {
        disable_irq((*(*drv).sdw).irq);

        ret = pm_runtime_force_suspend(dev);
        if ret != 0 {
            dev_err(dev, b"failed to force suspend: %d\n\0".as_ptr().cast(), ret);
            return ret;
        }
    }

    0
}

unsafe extern "C" fn class_resume(dev: *mut device) -> c_int {
    let drv: *mut sdca_class_drv = unsafe { dev_get_drvdata(dev).cast() };
    let ret: c_int;

    unsafe {
        ret = pm_runtime_force_resume(dev);
        if ret != 0 {
            dev_err(dev, b"failed to force resume: %d\n\0".as_ptr().cast(), ret);
            return ret;
        }

        enable_irq((*(*drv).sdw).irq);
    }

    0
}

unsafe extern "C" fn class_runtime_suspend(dev: *mut device) -> c_int {
    let drv: *mut sdca_class_drv = unsafe { dev_get_drvdata(dev).cast() };

    /*
     * Whilst the driver doesn't power the chip down here, going into runtime
     * suspend lets the SoundWire bus power down, which means the driver
     * can't communicate with the device any more.
     */
    unsafe {
        regcache_cache_only((*drv).dev_regmap, true);
    }

    0
}

unsafe extern "C" fn class_runtime_resume(dev: *mut device) -> c_int {
    let drv: *mut sdca_class_drv = unsafe { dev_get_drvdata(dev).cast() };
    let ret: c_int;

    unsafe {
        ret = sdw_slave_wait_for_init((*drv).sdw, CLASS_SDW_ATTACH_TIMEOUT_MS);
        if ret != 0 {
            regcache_cache_only((*drv).dev_regmap, true);
            return ret;
        }

        regcache_cache_only((*drv).dev_regmap, false);
        regcache_mark_dirty((*drv).dev_regmap);

        ret = regcache_sync((*drv).dev_regmap);
        if ret != 0 {
            dev_err((*drv).dev, b"failed to restore cache: %d\n\0".as_ptr().cast(), ret);
            regcache_cache_only((*drv).dev_regmap, true);
            return ret;
        }
    }

    0
}

/* SYSTEM_SLEEP_PM_OPS(class_suspend, class_resume)
 * RUNTIME_PM_OPS(class_runtime_suspend, class_runtime_resume, NULL)
 */
static CLASS_PM_OPS: dev_pm_ops = dev_pm_ops {
    suspend: Some(class_suspend),
    resume: Some(class_resume),
    runtime_suspend: Some(class_runtime_suspend),
    runtime_resume: Some(class_runtime_resume),
    runtime_idle: None,
};

/* SDW_SLAVE_ENTRY(0x01FA, 0x4245, 0), etc. */
static CLASS_SDW_ID: [sdw_device_id; 4] = [
    sdw_device_id {
        mfg_id: 0x01FA,
        part_id: 0x4245,
        class_id: 0,
    },
    sdw_device_id {
        mfg_id: 0x01FA,
        part_id: 0x4249,
        class_id: 0,
    },
    sdw_device_id {
        mfg_id: 0x01FA,
        part_id: 0x4747,
        class_id: 0,
    },
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        class_id: 0,
    },
];
/* MODULE_DEVICE_TABLE(sdw, class_sdw_id); */

static mut CLASS_SDW_DRIVER: sdw_driver = sdw_driver {
    driver: device_driver {
        name: b"sdca_class\0".as_ptr().cast(),
        pm: &CLASS_PM_OPS,
    },
    probe: Some(class_sdw_probe),
    remove: Some(class_sdw_remove),
    id_table: CLASS_SDW_ID.as_ptr(),
    ops: &CLASS_SDW_OPS,
};

#[used]
static MODULE_INIT_CLASS_SDW_DRIVER: unsafe extern "C" fn() = {
    unsafe extern "C" fn init() {
        unsafe {
            module_sdw_driver(&mut CLASS_SDW_DRIVER);
        }
    }
    init
};

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("SDCA Class Driver"); */
/* MODULE_IMPORT_NS("SND_SOC_SDCA"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
