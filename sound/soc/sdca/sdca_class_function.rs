// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct auxiliary_device {
    pub dev: device,
}

#[repr(C)]
pub struct auxiliary_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct auxiliary_driver {
    pub driver: device_driver,
    pub probe: Option<
        unsafe extern "C" fn(*mut auxiliary_device, *const auxiliary_device_id) -> c_int,
    >,
    pub remove: Option<unsafe extern "C" fn(*mut auxiliary_device)>,
    pub id_table: *const auxiliary_device_id,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_format_endian: c_uint,
    pub val_format_endian: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub lock: Option<unsafe extern "C" fn(*mut c_void)>,
    pub unlock: Option<unsafe extern "C" fn(*mut c_void)>,
    pub reg_defaults: *mut reg_default,
    pub num_reg_defaults: c_int,
    pub lock_arg: *mut c_void,
}

#[repr(C)]
pub struct regmap_sdw_mbq_cfg {
    pub mbq_size: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub deferrable: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub retry_us: c_uint,
    pub timeout_us: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub fixup_controls: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_port_config {
    pub num: c_int,
}

#[repr(C)]
pub struct sdca_function_desc {
    pub adr: c_uint,
    pub type_: c_uint,
}

#[repr(C)]
pub struct sdca_function_data {
    pub desc: *mut sdca_function_desc,
    pub busy_max_delay: c_uint,
}

#[repr(C)]
pub struct sdca_dev {
    pub function: sdca_function_data,
}

#[repr(C)]
pub struct sdca_class_drv {
    pub dev_regmap: *mut regmap,
    pub irq_info: *mut c_void,
    pub init_lock: mutex,
    pub regmap_lock: mutex,
    pub sdw: *mut sdw_slave,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct class_function_drv {
    dev: *mut device,
    regmap: *mut regmap,
    core: *mut sdca_class_drv,
    function: *mut sdca_function_data,
    suspended: bool,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGMAP_ENDIAN_LITTLE: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SDW_SDCA_MAX_REGISTER: c_uint = 0;
const SDCA_ENTITY_TYPE_ENTITY_0: c_uint = 0;
const SDCA_CTL_ENTITY_0_FUNCTION_STATUS: c_uint = 0;
const SDCA_CTL_ENTITY_0_FUNCTION_HAS_BEEN_RESET: c_uint = 0x01;
const SDCA_CTL_ENTITY_0_FUNCTION_NEEDS_INITIALIZATION: c_uint = 0x02;
const SDCA_FUNCTION_TYPE_SMART_AMP: c_uint = 0;
const SDCA_FUNCTION_TYPE_SMART_MIC: c_uint = 1;
const SDCA_FUNCTION_TYPE_UAJ: c_uint = 2;
const SDCA_FUNCTION_TYPE_HID: c_uint = 3;
const SDCA_FUNCTION_TYPE_RJ: c_uint = 4;
const DPM_FLAG_NO_DIRECT_COMPLETE: c_uint = 0;

unsafe extern "C" {
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn to_auxiliary_dev(dev: *mut device) -> *mut auxiliary_device;
    fn auxiliary_get_drvdata(auxdev: *mut auxiliary_device) -> *mut c_void;
    fn auxiliary_set_drvdata(auxdev: *mut auxiliary_device, data: *mut c_void);
    fn auxiliary_dev_to_sdca_dev(auxdev: *mut auxiliary_device) -> *mut sdca_dev;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn sdca_regmap_writeable(function: *mut sdca_function_data, reg: c_uint) -> bool;
    fn sdca_regmap_readable(function: *mut sdca_function_data, reg: c_uint) -> bool;
    fn sdca_regmap_volatile(function: *mut sdca_function_data, reg: c_uint) -> bool;
    fn sdca_regmap_mbq_size(function: *mut sdca_function_data, reg: c_uint) -> c_int;
    fn sdca_regmap_deferrable(function: *mut sdca_function_data, reg: c_uint) -> bool;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn snd_sdw_params_to_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        sconfig: *mut sdw_stream_config,
        pconfig: *mut sdw_port_config,
    );
    fn sdw_stream_add_slave(
        sdw: *mut sdw_slave,
        sconfig: *mut sdw_stream_config,
        pconfig: *mut sdw_port_config,
        count: c_uint,
        sdw_stream: *mut sdw_stream_runtime,
    ) -> c_int;
    fn sdw_stream_remove_slave(sdw: *mut sdw_slave, sdw_stream: *mut sdw_stream_runtime) -> c_int;
    fn sdca_asoc_set_constraints(
        dev: *mut device,
        regmap: *mut regmap,
        function: *mut sdca_function_data,
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn sdca_asoc_free_constraints(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai);
    fn sdca_asoc_get_port(
        dev: *mut device,
        regmap: *mut regmap,
        function: *mut sdca_function_data,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn sdca_asoc_hw_params(
        dev: *mut device,
        regmap: *mut regmap,
        function: *mut sdca_function_data,
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn sdca_irq_populate(function: *mut sdca_function_data, component: *mut snd_soc_component, irq_info: *mut c_void) -> c_int;
    fn sdca_irq_cleanup(dev: *mut device, function: *mut sdca_function_data, irq_info: *mut c_void);
    fn sdca_irq_populate_early(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data, irq_info: *mut c_void) -> c_int;
    fn sdca_irq_cleanup_late(dev: *mut device, function: *mut sdca_function_data, irq_info: *mut c_void);
    fn sdca_irq_enable_early(function: *mut sdca_function_data, irq_info: *mut c_void);
    fn sdca_irq_enable(function: *mut sdca_function_data, irq_info: *mut c_void);
    fn sdca_irq_disable(function: *mut sdca_function_data, irq_info: *mut c_void);
    fn sdca_jack_set_jack(irq_info: *mut c_void, jack: *mut snd_soc_jack) -> c_int;
    fn sdca_reset_function(dev: *mut device, function: *mut sdca_function_data, regmap: *mut regmap) -> c_int;
    fn sdca_regmap_write_init(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data) -> c_int;
    fn sdca_fdl_sync(dev: *mut device, function: *mut sdca_function_data, irq_info: *mut c_void) -> c_int;
    fn sdca_regmap_write_defaults(dev: *mut device, regmap: *mut regmap, function: *mut sdca_function_data) -> c_int;
    fn sdca_parse_function(dev: *mut device, function: *mut sdca_function_data) -> c_int;
    fn sdca_regmap_count_constants(dev: *mut device, function: *mut sdca_function_data) -> c_int;
    fn sdca_regmap_populate_constants(dev: *mut device, function: *mut sdca_function_data, defaults: *mut reg_default) -> c_int;
    fn regcache_sort_defaults(defaults: *mut reg_default, ndefaults: c_int);
    fn devm_regmap_init_sdw_mbq_cfg(
        dev: *mut device,
        sdw: *mut sdw_slave,
        config: *mut regmap_config,
        mbq_config: *mut regmap_sdw_mbq_cfg,
    ) -> *mut regmap;
    fn sdca_asoc_populate_component(
        dev: *mut device,
        function: *mut sdca_function_data,
        cmp_drv: *mut snd_soc_component_driver,
        dais: *mut *mut snd_soc_dai_driver,
        num_dais: *mut c_int,
        ops: *const snd_soc_dai_ops,
    ) -> c_int;
    fn dev_pm_set_driver_flags(dev: *mut device, flags: c_uint);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmp_drv: *mut snd_soc_component_driver,
        dais: *mut snd_soc_dai_driver,
        num_dais: c_int,
    ) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn module_auxiliary_driver(driver: *mut auxiliary_driver);
}

fn SDW_SDCA_CTL(adr: c_uint, entity: c_uint, control: c_uint, ch: c_uint) -> c_uint {
    adr | entity | control | ch
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe extern "C" fn class_function_regmap_lock(data: *mut c_void) {
    let lock = data as *mut mutex;

    unsafe { mutex_lock(lock) };
}

unsafe extern "C" fn class_function_regmap_unlock(data: *mut c_void) {
    let lock = data as *mut mutex;

    unsafe { mutex_unlock(lock) };
}

unsafe extern "C" fn class_function_regmap_writeable(dev: *mut device, reg: c_uint) -> bool {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;

    unsafe { sdca_regmap_writeable((*drv).function, reg) }
}

unsafe extern "C" fn class_function_regmap_readable(dev: *mut device, reg: c_uint) -> bool {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;

    unsafe { sdca_regmap_readable((*drv).function, reg) }
}

unsafe extern "C" fn class_function_regmap_volatile(dev: *mut device, reg: c_uint) -> bool {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;

    unsafe { sdca_regmap_volatile((*drv).function, reg) }
}

static class_function_regmap_config: regmap_config = regmap_config {
    name: b"sdca\0".as_ptr() as *const c_char,
    reg_bits: 32,
    val_bits: 32,
    reg_format_endian: REGMAP_ENDIAN_LITTLE,
    val_format_endian: REGMAP_ENDIAN_LITTLE,
    max_register: SDW_SDCA_MAX_REGISTER,
    readable_reg: Some(class_function_regmap_readable),
    writeable_reg: Some(class_function_regmap_writeable),
    volatile_reg: Some(class_function_regmap_volatile),
    cache_type: REGCACHE_MAPLE,
    lock: Some(class_function_regmap_lock),
    unlock: Some(class_function_regmap_unlock),
    reg_defaults: ptr::null_mut(),
    num_reg_defaults: 0,
    lock_arg: ptr::null_mut(),
};

unsafe extern "C" fn class_function_regmap_mbq_size(dev: *mut device, reg: c_uint) -> c_int {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;

    unsafe { sdca_regmap_mbq_size((*drv).function, reg) }
}

unsafe extern "C" fn class_function_regmap_deferrable(dev: *mut device, reg: c_uint) -> bool {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;

    unsafe { sdca_regmap_deferrable((*drv).function, reg) }
}

static class_function_mbq_config: regmap_sdw_mbq_cfg = regmap_sdw_mbq_cfg {
    mbq_size: Some(class_function_regmap_mbq_size),
    deferrable: Some(class_function_regmap_deferrable),
    retry_us: 1000,
    timeout_us: 10000,
};

unsafe extern "C" fn class_function_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drv = unsafe { snd_soc_component_get_drvdata((*dai).component) } as *mut class_function_drv;

    unsafe { sdca_asoc_set_constraints((*drv).dev, (*drv).regmap, (*drv).function, substream, dai) }
}

unsafe extern "C" fn class_function_sdw_add_peripheral(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drv = unsafe { snd_soc_component_get_drvdata((*dai).component) } as *mut class_function_drv;
    let sdw_stream = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let sdw = unsafe { dev_to_sdw_dev((*(*drv).dev).parent) };
    let mut sconfig: sdw_stream_config = unsafe { core::mem::zeroed() };
    let mut pconfig: sdw_port_config = unsafe { core::mem::zeroed() };
    let mut ret: c_int;

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    unsafe { snd_sdw_params_to_config(substream, params, &mut sconfig, &mut pconfig) };

    /*
     * FIXME: As also noted in sdca_asoc_get_port(), currently only
     * a single unshared port is supported for each DAI.
     */
    ret = unsafe { sdca_asoc_get_port((*drv).dev, (*drv).regmap, (*drv).function, dai) };
    if ret < 0 {
        return ret;
    }

    pconfig.num = ret;

    ret = unsafe { sdw_stream_add_slave(sdw, &mut sconfig, &mut pconfig, 1, sdw_stream) };
    if ret != 0 {
        unsafe { dev_err((*drv).dev, b"failed to add sdw stream: %d\n\0".as_ptr() as *const c_char, ret) };
        return ret;
    }

    unsafe { sdca_asoc_hw_params((*drv).dev, (*drv).regmap, (*drv).function, substream, params, dai) }
}

unsafe extern "C" fn class_function_sdw_remove_peripheral(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drv = unsafe { snd_soc_component_get_drvdata((*dai).component) } as *mut class_function_drv;
    let sdw_stream = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let sdw = unsafe { dev_to_sdw_dev((*(*drv).dev).parent) };

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    unsafe { sdw_stream_remove_slave(sdw, sdw_stream) }
}

unsafe extern "C" fn class_function_sdw_set_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut c_void,
    direction: c_int,
) -> c_int {
    unsafe { snd_soc_dai_dma_data_set(dai, direction, sdw_stream) };

    0
}

static class_function_sdw_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(class_function_startup),
    shutdown: Some(sdca_asoc_free_constraints),
    set_stream: Some(class_function_sdw_set_stream),
    hw_params: Some(class_function_sdw_add_peripheral),
    hw_free: Some(class_function_sdw_remove_peripheral),
};

unsafe extern "C" fn class_function_component_fixup_controls(
    component: *mut snd_soc_component,
) -> c_int {
    let drv = unsafe { snd_soc_component_get_drvdata(component) } as *mut class_function_drv;
    let core = unsafe { (*drv).core };

    unsafe { sdca_irq_populate((*drv).function, component, (*core).irq_info) }
}

unsafe extern "C" fn class_function_component_remove(component: *mut snd_soc_component) {
    let drv = unsafe { snd_soc_component_get_drvdata(component) } as *mut class_function_drv;
    let core = unsafe { (*drv).core };

    unsafe { sdca_irq_cleanup((*component).dev, (*drv).function, (*core).irq_info) };
}

unsafe extern "C" fn class_function_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    _d: *mut c_void,
) -> c_int {
    let drv = unsafe { snd_soc_component_get_drvdata(component) } as *mut class_function_drv;
    let core = unsafe { (*drv).core };

    unsafe { sdca_jack_set_jack((*core).irq_info, jack) }
}

static class_function_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    fixup_controls: Some(class_function_component_fixup_controls),
    remove: Some(class_function_component_remove),
    set_jack: None,
    endianness: 1,
};

unsafe fn class_function_init_device(drv: *mut class_function_drv, status: c_uint) -> c_int {
    let mut ret: c_int;

    if (status & SDCA_CTL_ENTITY_0_FUNCTION_HAS_BEEN_RESET) == 0 {
        unsafe { dev_dbg((*drv).dev, b"reset function device\n\0".as_ptr() as *const c_char) };

        ret = unsafe { sdca_reset_function((*drv).dev, (*drv).function, (*drv).regmap) };
        if ret != 0 {
            return ret;
        }
    }

    if (status & SDCA_CTL_ENTITY_0_FUNCTION_NEEDS_INITIALIZATION) != 0 {
        unsafe { dev_dbg((*drv).dev, b"write initialisation\n\0".as_ptr() as *const c_char) };

        ret = unsafe { sdca_regmap_write_init((*drv).dev, (*(*drv).core).dev_regmap, (*drv).function) };
        if ret != 0 {
            return ret;
        }
    }

    0
}

unsafe fn class_function_boot(drv: *mut class_function_drv) -> c_int {
    let reg = unsafe {
        SDW_SDCA_CTL(
            (*(*(*drv).function).desc).adr,
            SDCA_ENTITY_TYPE_ENTITY_0,
            SDCA_CTL_ENTITY_0_FUNCTION_STATUS,
            0,
        )
    };
    let mut val: c_uint = 0;
    let mut ret: c_int;

    unsafe { mutex_lock(&mut (*(*drv).core).init_lock) };

    ret = unsafe { regmap_read((*drv).regmap, reg, &mut val) };
    if ret < 0 {
        unsafe { dev_err((*drv).dev, b"failed to read function status: %d\n\0".as_ptr() as *const c_char, ret) };
        unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };
        return ret;
    }

    ret = unsafe { class_function_init_device(drv, val) };
    if ret != 0 {
        unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };
        return ret;
    }

    /* Start FDL process */
    ret = unsafe { sdca_irq_populate_early((*drv).dev, (*drv).regmap, (*drv).function, (*(*drv).core).irq_info) };
    if ret != 0 {
        unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };
        return ret;
    }

    ret = unsafe { sdca_fdl_sync((*drv).dev, (*drv).function, (*(*drv).core).irq_info) };
    if ret != 0 {
        unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };
        return ret;
    }

    ret = unsafe { sdca_regmap_write_defaults((*drv).dev, (*drv).regmap, (*drv).function) };
    if ret != 0 {
        unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };
        return ret;
    }

    ret = unsafe { regmap_write((*drv).regmap, reg, 0xFF) };
    if ret < 0 {
        unsafe { dev_err((*drv).dev, b"failed to clear function status: %d\n\0".as_ptr() as *const c_char, ret) };
        unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };
        return ret;
    }

    unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };

    0
}

unsafe extern "C" fn class_function_probe(
    auxdev: *mut auxiliary_device,
    _aux_dev_id: *const auxiliary_device_id,
) -> c_int {
    let dev = unsafe { &mut (*auxdev).dev as *mut device };
    let core = unsafe { dev_get_drvdata((*dev).parent) } as *mut sdca_class_drv;
    let sdev = unsafe { auxiliary_dev_to_sdca_dev(auxdev) };
    let mut cmp_drv: *mut snd_soc_component_driver;
    let mut dais: *mut snd_soc_dai_driver = ptr::null_mut();
    let mut drv: *mut class_function_drv;
    let mut mbq_config: *mut regmap_sdw_mbq_cfg;
    let mut config: *mut regmap_config;
    let mut defaults: *mut reg_default;
    let mut ndefaults: c_int;
    let mut num_dais: c_int = 0;
    let mut ret: c_int;

    drv = unsafe { devm_kzalloc(dev, core::mem::size_of::<class_function_drv>(), GFP_KERNEL) } as *mut class_function_drv;
    if drv.is_null() {
        return -ENOMEM;
    }

    cmp_drv = unsafe {
        devm_kmemdup(
            dev,
            &class_function_component_drv as *const _ as *const c_void,
            core::mem::size_of::<snd_soc_component_driver>(),
            GFP_KERNEL,
        )
    } as *mut snd_soc_component_driver;
    if cmp_drv.is_null() {
        return -ENOMEM;
    }

    config = unsafe {
        devm_kmemdup(
            dev,
            &class_function_regmap_config as *const _ as *const c_void,
            core::mem::size_of::<regmap_config>(),
            GFP_KERNEL,
        )
    } as *mut regmap_config;
    if config.is_null() {
        return -ENOMEM;
    }

    mbq_config = unsafe {
        devm_kmemdup(
            dev,
            &class_function_mbq_config as *const _ as *const c_void,
            core::mem::size_of::<regmap_sdw_mbq_cfg>(),
            GFP_KERNEL,
        )
    } as *mut regmap_sdw_mbq_cfg;
    if mbq_config.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*drv).dev = dev;
        (*drv).core = core;
        (*drv).function = &mut (*sdev).function;
    }

    ret = unsafe { sdca_parse_function(dev, (*drv).function) };
    if ret != 0 {
        return ret;
    }

    ndefaults = unsafe { sdca_regmap_count_constants(dev, (*drv).function) };
    if ndefaults < 0 {
        return ndefaults;
    }

    defaults = unsafe { devm_kcalloc(dev, ndefaults, core::mem::size_of::<reg_default>(), GFP_KERNEL) } as *mut reg_default;
    if defaults.is_null() {
        return -ENOMEM;
    }

    ret = unsafe { sdca_regmap_populate_constants(dev, (*drv).function, defaults) };
    if ret < 0 {
        return ret;
    }

    unsafe { regcache_sort_defaults(defaults, ndefaults) };

    unsafe { auxiliary_set_drvdata(auxdev, drv as *mut c_void) };

    unsafe {
        (*config).reg_defaults = defaults;
        (*config).num_reg_defaults = ndefaults;
        (*config).lock_arg = &mut (*core).regmap_lock as *mut _ as *mut c_void;
    }

    unsafe {
        if (*(*drv).function).busy_max_delay != 0 {
            (*mbq_config).timeout_us = (*(*drv).function).busy_max_delay;
            (*mbq_config).retry_us = core::cmp::max(
                (*(*drv).function).busy_max_delay / 10,
                (*mbq_config).retry_us,
            );
        }
    }

    unsafe {
        (*drv).regmap = devm_regmap_init_sdw_mbq_cfg(dev, (*core).sdw, config, mbq_config);
    }
    if unsafe { IS_ERR((*drv).regmap) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*drv).regmap),
                b"failed to create regmap\0".as_ptr() as *const c_char,
            )
        };
    }

    unsafe {
        match (*(*(*drv).function).desc).type_ {
            SDCA_FUNCTION_TYPE_UAJ | SDCA_FUNCTION_TYPE_RJ => {
                (*cmp_drv).set_jack = Some(class_function_set_jack);
            }
            _ => {}
        }
    }

    ret = unsafe {
        sdca_asoc_populate_component(
            dev,
            (*drv).function,
            cmp_drv,
            &mut dais,
            &mut num_dais,
            &class_function_sdw_ops,
        )
    };
    if ret != 0 {
        return ret;
    }

    unsafe { dev_pm_set_driver_flags(dev, DPM_FLAG_NO_DIRECT_COMPLETE) };

    unsafe { pm_runtime_set_autosuspend_delay(dev, 200) };
    unsafe { pm_runtime_use_autosuspend(dev) };
    unsafe { pm_runtime_set_active(dev) };
    unsafe { pm_runtime_get_noresume(dev) };

    ret = unsafe { devm_pm_runtime_enable(dev) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { class_function_boot(drv) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { devm_snd_soc_register_component(dev, cmp_drv, dais, num_dais) };
    if ret != 0 {
        return unsafe {
            dev_err_probe(
                dev,
                ret,
                b"failed to register component\n\0".as_ptr() as *const c_char,
            )
        };
    }

    unsafe { pm_runtime_mark_last_busy(dev) };
    unsafe { pm_runtime_put_autosuspend(dev) };

    0
}

unsafe extern "C" fn class_function_remove(auxdev: *mut auxiliary_device) {
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;

    unsafe { sdca_irq_cleanup_late((*drv).dev, (*drv).function, (*(*drv).core).irq_info) };
}

unsafe extern "C" fn class_function_runtime_suspend(dev: *mut device) -> c_int {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;

    /*
     * Whilst the driver doesn't power the chip down here, going into
     * runtime suspend means the driver can't be sure the bus won't
     * power down which would prevent communication with the device.
     */
    unsafe { regcache_cache_only((*drv).regmap, true) };

    0
}

unsafe extern "C" fn class_function_runtime_resume(dev: *mut device) -> c_int {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;
    let mut ret: c_int;

    unsafe { mutex_lock(&mut (*(*drv).core).init_lock) };

    unsafe { regcache_mark_dirty((*drv).regmap) };
    unsafe { regcache_cache_only((*drv).regmap, false) };

    unsafe {
        if (*drv).suspended {
            let reg = SDW_SDCA_CTL(
                (*(*(*drv).function).desc).adr,
                SDCA_ENTITY_TYPE_ENTITY_0,
                SDCA_CTL_ENTITY_0_FUNCTION_STATUS,
                0,
            );
            let mut val: c_uint = 0;

            ret = regmap_read((*drv).regmap, reg, &mut val);
            if ret < 0 {
                dev_err((*drv).dev, b"failed to read function status: %d\n\0".as_ptr() as *const c_char, ret);
                regcache_cache_only((*drv).regmap, true);
                mutex_unlock(&mut (*(*drv).core).init_lock);
                return ret;
            }

            ret = class_function_init_device(drv, val);
            if ret != 0 {
                regcache_cache_only((*drv).regmap, true);
                mutex_unlock(&mut (*(*drv).core).init_lock);
                return ret;
            }

            sdca_irq_enable_early((*drv).function, (*(*drv).core).irq_info);

            ret = sdca_fdl_sync((*drv).dev, (*drv).function, (*(*drv).core).irq_info);
            if ret != 0 {
                regcache_cache_only((*drv).regmap, true);
                mutex_unlock(&mut (*(*drv).core).init_lock);
                return ret;
            }

            sdca_irq_enable((*drv).function, (*(*drv).core).irq_info);

            ret = regmap_write((*drv).regmap, reg, 0xFF);
            if ret < 0 {
                dev_err((*drv).dev, b"failed to clear function status: %d\n\0".as_ptr() as *const c_char, ret);
                regcache_cache_only((*drv).regmap, true);
                mutex_unlock(&mut (*(*drv).core).init_lock);
                return ret;
            }

            (*drv).suspended = false;
        }
    }

    ret = unsafe { regcache_sync((*drv).regmap) };
    if ret != 0 {
        unsafe { dev_err((*drv).dev, b"failed to restore register cache: %d\n\0".as_ptr() as *const c_char, ret) };
        unsafe { regcache_cache_only((*drv).regmap, true) };
        unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };
        return ret;
    }

    unsafe { mutex_unlock(&mut (*(*drv).core).init_lock) };

    0
}

unsafe extern "C" fn class_function_suspend(dev: *mut device) -> c_int {
    let auxdev = unsafe { to_auxiliary_dev(dev) };
    let drv = unsafe { auxiliary_get_drvdata(auxdev) } as *mut class_function_drv;
    let mut ret: c_int;

    unsafe { (*drv).suspended = true };

    /* Ensure runtime resume runs on resume */
    ret = unsafe { pm_runtime_resume_and_get(dev) };
    if ret != 0 {
        unsafe { dev_err(dev, b"failed to resume for suspend: %d\n\0".as_ptr() as *const c_char, ret) };
        return ret;
    }

    unsafe { sdca_irq_disable((*drv).function, (*(*drv).core).irq_info) };

    ret = unsafe { pm_runtime_force_suspend(dev) };
    if ret != 0 {
        unsafe { dev_err(dev, b"failed to force suspend: %d\n\0".as_ptr() as *const c_char, ret) };
        return ret;
    }

    unsafe { pm_runtime_put_noidle(dev) };

    0
}

unsafe extern "C" fn class_function_resume(dev: *mut device) -> c_int {
    let mut ret: c_int;

    ret = unsafe { pm_runtime_force_resume(dev) };
    if ret != 0 {
        unsafe { dev_err(dev, b"failed to force resume: %d\n\0".as_ptr() as *const c_char, ret) };
        return ret;
    }

    0
}

static class_function_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(class_function_suspend),
    resume: Some(class_function_resume),
    runtime_suspend: Some(class_function_runtime_suspend),
    runtime_resume: Some(class_function_runtime_resume),
};

static class_function_id_table: [auxiliary_device_id; 6] = [
    auxiliary_device_id {
        name: b"snd_soc_sdca.smart_amp\0".as_ptr() as *const c_char,
        driver_data: SDCA_FUNCTION_TYPE_SMART_AMP as c_ulong,
    },
    auxiliary_device_id {
        name: b"snd_soc_sdca.smart_mic\0".as_ptr() as *const c_char,
        driver_data: SDCA_FUNCTION_TYPE_SMART_MIC as c_ulong,
    },
    auxiliary_device_id {
        name: b"snd_soc_sdca.uaj\0".as_ptr() as *const c_char,
        driver_data: SDCA_FUNCTION_TYPE_UAJ as c_ulong,
    },
    auxiliary_device_id {
        name: b"snd_soc_sdca.hid\0".as_ptr() as *const c_char,
        driver_data: SDCA_FUNCTION_TYPE_HID as c_ulong,
    },
    auxiliary_device_id {
        name: b"snd_soc_sdca.rj\0".as_ptr() as *const c_char,
        driver_data: SDCA_FUNCTION_TYPE_RJ as c_ulong,
    },
    auxiliary_device_id {
        name: ptr::null(),
        driver_data: 0,
    },
];

/* MODULE_DEVICE_TABLE(auxiliary, class_function_id_table); */

static mut class_function_drv: auxiliary_driver = auxiliary_driver {
    driver: device_driver {
        name: b"sdca_function\0".as_ptr() as *const c_char,
        pm: &class_function_pm_ops,
    },
    probe: Some(class_function_probe),
    remove: Some(class_function_remove),
    id_table: class_function_id_table.as_ptr(),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_module() -> c_int {
    unsafe { module_auxiliary_driver(&mut class_function_drv) };
    0
}

/*
 * MODULE_LICENSE("GPL");
 * MODULE_DESCRIPTION("SDCA Class Function Driver");
 * MODULE_IMPORT_NS("SND_SOC_SDCA");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
