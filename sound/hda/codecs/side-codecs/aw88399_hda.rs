// SPDX-License-Identifier: GPL-2.0-only
//
// AW88399 HDA side codec driver
//
// Based on cs35l41_hda.c and aw88399.c
//

// C includes removed. External kernel, HDA, and AW88399 symbols are declared
// here as dependencies supplied by the surrounding repository.

use core::ffi::{c_char, c_int, c_void};

const AW88399_HDA_I2C_BASE_ADDR: c_int = 0x34;

const HDA_GEN_PCM_ACT_OPEN: c_int = 0;
const HDA_GEN_PCM_ACT_PREPARE: c_int = 1;
const HDA_GEN_PCM_ACT_CLEANUP: c_int = 2;
const HDA_GEN_PCM_ACT_CLOSE: c_int = 3;

const AW88399_SYNC_START: c_int = 0;
const GFP_KERNEL: c_int = 0;
const GPIOD_OUT_LOW: c_int = 0;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub addr: c_int,
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aw_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aw88399 {
    pub lock: mutex,
    pub reset_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub bsts_unreliable: bool,
    pub aw_pa: *mut aw_device,
    pub fw_needs_reload: bool,
}

#[repr(C)]
pub struct aw88399_hda {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub core: *mut aw88399,
    pub aw_dev: *mut aw_device,
    pub index: c_int,
    pub channel: c_int,
    pub playing: bool,
    pub bsts_unreliable: bool,
    pub acpi_subsystem_id: *const c_char,
}

#[repr(C)]
pub struct hda_component_parent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_component {
    pub dev: *mut device,
    pub name: [c_char; 0],
    pub playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
}

#[repr(C)]
pub struct component_ops {
    pub bind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void) -> c_int>,
    pub unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void)>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct aw88399_prop_model {
    ssid: *const c_char,
    apply_prop: Option<unsafe extern "C" fn(*mut aw88399_hda) -> c_int>,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;

    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;

    fn hda_component_from_index(parent: *mut hda_component_parent, index: c_int) -> *mut hda_component;
    fn component_add(dev: *mut device, ops: *const component_ops) -> c_int;
    fn component_del(dev: *mut device, ops: *const component_ops);

    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn aw88399_hw_reset(core: *mut aw88399);
    fn aw88399_init(core: *mut aw88399, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int;
    fn aw88399_dev_set_channel(core: *mut aw88399, channel: c_int);
    fn aw88399_request_firmware_file(core: *mut aw88399) -> c_int;
    fn aw88399_start(core: *mut aw88399, sync: c_int) -> c_int;
    fn aw88399_stop(aw_dev: *mut aw_device) -> c_int;

    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_void, hrv: c_int) -> *mut acpi_device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn ACPI_HANDLE(dev: *mut device) -> *mut c_void;
    fn acpi_get_subsystem_id(handle: *mut c_void) -> *mut c_char;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_int) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

unsafe extern "C" fn aw88399_hda_playback_hook(dev: *mut device, action: c_int) {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;
    let core = (*aw88399).core;
    let mut ret: c_int = 0;

    dev_dbg((*aw88399).dev, c"Playback action: %d\n".as_ptr(), action);

    match action {
        HDA_GEN_PCM_ACT_OPEN => {
            pm_runtime_get_sync(dev);
            (*aw88399).playing = true;
        }
        HDA_GEN_PCM_ACT_PREPARE => {
            if !core.is_null() {
                aw88399_start(core, AW88399_SYNC_START);
            }
        }
        HDA_GEN_PCM_ACT_CLEANUP => {
            if !(*aw88399).aw_dev.is_null() {
                ret = aw88399_stop((*aw88399).aw_dev);
            }
            if ret != 0 {
                dev_err((*aw88399).dev, c"Failed to stop amplifier: %d\n".as_ptr(), ret);
            }
        }
        HDA_GEN_PCM_ACT_CLOSE => {
            if !(*aw88399).aw_dev.is_null() {
                aw88399_stop((*aw88399).aw_dev);
            }
            (*aw88399).playing = false;
            pm_runtime_mark_last_busy(dev);
            pm_runtime_put_autosuspend(dev);
        }
        _ => {
            dev_warn((*aw88399).dev, c"Unsupported action: %d\n".as_ptr(), action);
        }
    }
}

unsafe extern "C" fn aw88399_hda_bind(
    dev: *mut device,
    _master: *mut device,
    master_data: *mut c_void,
) -> c_int {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;
    let parent = master_data as *mut hda_component_parent;
    let comp: *mut hda_component;

    comp = hda_component_from_index(parent, (*aw88399).index);
    if comp.is_null() {
        return -EINVAL;
    }

    if !(*comp).dev.is_null() {
        return -EBUSY;
    }

    (*comp).dev = dev;

    strscpy((*comp).name.as_mut_ptr(), dev_name(dev), (*comp).name.len());

    (*comp).playback_hook = Some(aw88399_hda_playback_hook);

    dev_info(
        (*aw88399).dev,
        c"AW88399 Bound - SSID: %s, channel: %d\n".as_ptr(),
        (*aw88399).acpi_subsystem_id,
        (*aw88399).channel,
    );

    0
}

unsafe extern "C" fn aw88399_hda_unbind(
    dev: *mut device,
    _master: *mut device,
    master_data: *mut c_void,
) {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;
    let parent = master_data as *mut hda_component_parent;
    let comp: *mut hda_component;

    comp = hda_component_from_index(parent, (*aw88399).index);
    if !comp.is_null() && (*comp).dev == dev {
        memset(
            comp as *mut c_void,
            0,
            core::mem::size_of::<hda_component>(),
        );
    }

    dev_dbg((*aw88399).dev, c"Unbound from HDA codec\n".as_ptr());
}

static aw88399_hda_comp_ops: component_ops = component_ops {
    bind: Some(aw88399_hda_bind),
    unbind: Some(aw88399_hda_unbind),
};

unsafe fn aw88399_hda_index_from_i2c(aw88399: *mut aw88399_hda) -> c_int {
    (*to_i2c_client((*aw88399).dev)).addr - AW88399_HDA_I2C_BASE_ADDR
}

unsafe fn aw88399_hda_init(aw88399: *mut aw88399_hda) -> c_int {
    let dev = (*aw88399).dev;
    let i2c = to_i2c_client(dev);
    let core: *mut aw88399;
    let mut ret: c_int;

    core = devm_kzalloc(dev, core::mem::size_of::<aw88399>(), GFP_KERNEL) as *mut aw88399;
    if core.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*core).lock);
    (*core).reset_gpio = (*aw88399).reset_gpio;
    (*core).regmap = (*aw88399).regmap;
    (*core).bsts_unreliable = (*aw88399).bsts_unreliable;

    aw88399_hw_reset(core);

    ret = aw88399_init(core, i2c, (*aw88399).regmap);
    if ret != 0 {
        return ret;
    }

    /* Set channel BEFORE loading firmware so ACF parser sees correct value */
    if !(*core).aw_pa.is_null() {
        aw88399_dev_set_channel(core, (*aw88399).channel);
    }

    ret = aw88399_request_firmware_file(core);
    if ret != 0 {
        return ret;
    }

    (*aw88399).core = core;
    (*aw88399).aw_dev = (*core).aw_pa;

    0
}

unsafe extern "C" fn aw88399_swap_channels(aw88399: *mut aw88399_hda) -> c_int {
    /*
     * Certain Lenovo Legion laptops have their
     * I2C wiring reversed: 0x34 is physically the right speaker,
     * 0x35 is the left. Swap channels to correct L/R assignment.
     * This is a model-specific hardware wiring issue, not a driver bug.
     */
    (*aw88399).channel = 1 - (*aw88399).channel;
    dev_dbg(
        (*aw88399).dev,
        c"Channel swap applied: index %d -> channel %d\n".as_ptr(),
        (*aw88399).index,
        (*aw88399).channel,
    );
    0
}

unsafe extern "C" fn aw88399_skip_bsts_check(aw88399: *mut aw88399_hda) -> c_int {
    /*
     * BSTS (boost-finished) status bit does not reliably report on
     * some hardware. On certain Lenovo Legion laptops, both amps
     * report BSTS=0 (boost not finished) during normal playback
     * despite clean audio output. Skip BSTS in the startup status
     * check to avoid false init failures.
     */
    (*aw88399).bsts_unreliable = true;
    dev_dbg((*aw88399).dev, c"BSTS status check disabled\n".as_ptr());
    0
}

unsafe extern "C" fn aw88399_apply_legion_quirks(aw88399: *mut aw88399_hda) -> c_int {
    aw88399_swap_channels(aw88399);
    aw88399_skip_bsts_check(aw88399);
    0
}

static aw88399_prop_model_table: [aw88399_prop_model; 9] = [
    aw88399_prop_model { ssid: c"17AA3906".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: c"17AA3907".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: c"17AA3927".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: c"17AA3928".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: c"17AA3936".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: c"17AA3937".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: c"17AA3938".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: c"17AA3939".as_ptr(), apply_prop: Some(aw88399_apply_legion_quirks) },
    aw88399_prop_model { ssid: core::ptr::null(), apply_prop: None },
];

unsafe fn aw88399_hda_acpi_probe(aw88399: *mut aw88399_hda) -> c_int {
    let mut adev: *mut acpi_device;
    let sub: *mut c_char;
    let mut model: *const aw88399_prop_model;

    (*aw88399).index = aw88399_hda_index_from_i2c(aw88399);
    (*aw88399).channel = (*aw88399).index;
    (*aw88399).acpi_subsystem_id = core::ptr::null();

    adev = acpi_dev_get_first_match_dev(c"AWDZ8399".as_ptr(), core::ptr::null(), -1);
    if adev.is_null() {
        dev_err(
            (*aw88399).dev,
            c"Failed to find an ACPI device for AWDZ8399\n".as_ptr(),
        );
        return -ENODEV;
    }

    // C used: struct device *physdev __free(put_device) =
    //     get_device(acpi_get_first_physical_node(adev));
    let physdev = get_device(acpi_get_first_physical_node(adev));
    acpi_dev_put(adev);
    if physdev.is_null() {
        return -ENODEV;
    }

    sub = acpi_get_subsystem_id(ACPI_HANDLE(physdev));
    if IS_ERR_OR_NULL(sub as *const c_void) {
        put_device(physdev);
        return 0;
    }

    (*aw88399).acpi_subsystem_id = devm_kstrdup((*aw88399).dev, sub, GFP_KERNEL);
    kfree(sub as *mut c_void);
    if (*aw88399).acpi_subsystem_id.is_null() {
        put_device(physdev);
        return -ENOMEM;
    }

    model = aw88399_prop_model_table.as_ptr();
    while !(*model).ssid.is_null() {
        if strcasecmp((*model).ssid, (*aw88399).acpi_subsystem_id) == 0 {
            dev_info(
                (*aw88399).dev,
                c"Applying properties for SSID %s\n".as_ptr(),
                (*aw88399).acpi_subsystem_id,
            );
            let ret = ((*model).apply_prop.unwrap())(aw88399);
            put_device(physdev);
            return ret;
        }
        model = model.add(1);
    }

    put_device(physdev);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_hda_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let aw88399: *mut aw88399_hda;
    let mut ret: c_int;

    aw88399 = devm_kzalloc(dev, core::mem::size_of::<aw88399_hda>(), GFP_KERNEL) as *mut aw88399_hda;
    if aw88399.is_null() {
        return -ENOMEM;
    }

    if IS_ERR(regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR(regmap as *const c_void), c"Failed to obtain regmap\n".as_ptr());
    }

    (*aw88399).dev = dev;
    (*aw88399).regmap = regmap;
    dev_set_drvdata(dev, aw88399 as *mut c_void);

    (*aw88399).reset_gpio = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*aw88399).reset_gpio as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*aw88399).reset_gpio as *const c_void),
            c"Failed to get reset GPIO\n".as_ptr(),
        );
    }

    ret = aw88399_hda_acpi_probe(aw88399);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"ACPI probe failed\n".as_ptr());
    }

    ret = aw88399_hda_init(aw88399);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Chip initialization failed\n".as_ptr());
    }

    /* Enable runtime PM */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);

    ret = component_add(dev, &aw88399_hda_comp_ops);
    if ret != 0 {
        pm_runtime_disable(dev);
        return dev_err_probe(dev, ret, c"Failed to register component\n".as_ptr());
    }

    dev_info(dev, c"AW88399 HDA side codec registered successfully\n".as_ptr());

    0
}
// EXPORT_SYMBOL_NS_GPL(aw88399_hda_probe, "SND_HDA_SCODEC_AW88399");

extern "C" {
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_hda_remove(dev: *mut device) {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;

    pm_runtime_disable(dev);

    if !(*aw88399).aw_dev.is_null() {
        aw88399_stop((*aw88399).aw_dev);
    }

    component_del(dev, &aw88399_hda_comp_ops);

    dev_dbg((*aw88399).dev, c"AW88399 HDA side codec removed\n".as_ptr());
}
// EXPORT_SYMBOL_NS_GPL(aw88399_hda_remove, "SND_HDA_SCODEC_AW88399");

unsafe extern "C" fn aw88399_hda_runtime_suspend(dev: *mut device) -> c_int {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;

    dev_dbg((*aw88399).dev, c"Runtime suspend\n".as_ptr());

    if !(*aw88399).aw_dev.is_null() && (*aw88399).playing {
        aw88399_stop((*aw88399).aw_dev);
    }

    0
}

unsafe extern "C" fn aw88399_hda_runtime_resume(dev: *mut device) -> c_int {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;

    dev_dbg((*aw88399).dev, c"Runtime resume\n".as_ptr());

    if !(*aw88399).core.is_null() && !(*aw88399).aw_dev.is_null() && (*aw88399).playing {
        aw88399_start((*aw88399).core, AW88399_SYNC_START);
    }

    0
}

unsafe extern "C" fn aw88399_hda_system_suspend(dev: *mut device) -> c_int {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;
    let ret: c_int;

    dev_dbg((*aw88399).dev, c"System suspend\n".as_ptr());

    if !(*aw88399).aw_dev.is_null() && (*aw88399).playing {
        aw88399_stop((*aw88399).aw_dev);
    }

    if !(*aw88399).core.is_null() {
        (*(*aw88399).core).fw_needs_reload = true;
    }

    ret = pm_runtime_force_suspend(dev);
    if ret != 0 {
        dev_err((*aw88399).dev, c"Runtime force suspend failed: %d\n".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn aw88399_hda_system_resume(dev: *mut device) -> c_int {
    let aw88399 = dev_get_drvdata(dev) as *mut aw88399_hda;
    let ret: c_int;

    dev_dbg((*aw88399).dev, c"System resume\n".as_ptr());

    if !(*aw88399).aw_dev.is_null() {
        aw88399_hw_reset((*aw88399).core);
    }

    ret = pm_runtime_force_resume(dev);
    if ret != 0 {
        dev_err((*aw88399).dev, c"Runtime force resume failed: %d\n".as_ptr(), ret);
    }

    ret
}

#[no_mangle]
pub static aw88399_hda_pm_ops: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(aw88399_hda_runtime_suspend, aw88399_hda_runtime_resume, NULL)
    runtime_suspend: Some(aw88399_hda_runtime_suspend),
    runtime_resume: Some(aw88399_hda_runtime_resume),
    runtime_idle: None,
    // SYSTEM_SLEEP_PM_OPS(aw88399_hda_system_suspend, aw88399_hda_system_resume)
    suspend: Some(aw88399_hda_system_suspend),
    resume: Some(aw88399_hda_system_resume),
};
// EXPORT_SYMBOL_NS_GPL(aw88399_hda_pm_ops, "SND_HDA_SCODEC_AW88399");

// MODULE_DESCRIPTION("AW88399 HDA driver");
// MODULE_AUTHOR("Yakov Till <yakov.till@gmail.com>");
// MODULE_AUTHOR("Marco Giunta <marco_giunta@outlook.it>");
// MODULE_LICENSE("GPL");
// MODULE_FIRMWARE("aw88399_acf.bin");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
