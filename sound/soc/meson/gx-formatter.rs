// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2026 BayLibre, SAS.
// Author: Valerio Setti <vsetti@baylibre.com>

// Translated from Linux C implementation source. External kernel, ALSA SoC,
// regmap, list, mutex, allocator, and gx-formatter header symbols are declared
// here as dependencies supplied by other translation units.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gx_iface {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gx_stream {
    pub formatter_list: list_head,
    pub lock: mutex,
    pub iface: *mut gx_iface,
    pub ready: bool,
}

#[repr(C)]
pub struct gx_formatter_ops {
    pub prepare: Option<
        unsafe extern "C" fn(*mut regmap, c_uint, *mut gx_stream) -> c_int,
    >,
    pub enable: Option<unsafe extern "C" fn(*mut regmap)>,
    pub disable: Option<unsafe extern "C" fn(*mut regmap)>,
    pub get_stream: unsafe extern "C" fn(*mut snd_soc_dapm_widget) -> *mut gx_stream,
}

#[repr(C)]
pub struct gx_formatter_driver {
    pub ops: *const gx_formatter_ops,
    pub quirks: c_uint,
    pub regmap_cfg: *const regmap_config,
    pub component_drv: *const snd_soc_component_driver,
}

#[repr(C)]
pub struct gx_formatter {
    pub list: list_head,
    pub stream: *mut gx_stream,
    pub drv: *const gx_formatter_driver,
    pub enabled: bool,
    pub map: *mut regmap,
}

const GFP_KERNEL: c_uint = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SND_SOC_DAPM_PRE_PMU: c_int = 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 2;

unsafe extern "C" {
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);

    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn __list_for_each_entry_first(head: *mut list_head) -> *mut gx_formatter;
    fn __list_for_each_entry_next(pos: *mut gx_formatter) -> *mut gx_formatter;
    fn __list_for_each_entry_done(pos: *mut gx_formatter, head: *mut list_head) -> bool;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);

    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn WARN_ON(condition: bool) -> bool;

    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

unsafe fn gx_formatter_enable(formatter: *mut gx_formatter) -> c_int {
    let ret: c_int;

    /* Do nothing if the formatter is already enabled */
    if unsafe { (*formatter).enabled } {
        return 0;
    }

    /* Setup the stream parameter in the formatter */
    if let Some(prepare) = unsafe { (*(*(*formatter).drv).ops).prepare } {
        ret = unsafe {
            prepare(
                (*formatter).map,
                (*(*formatter).drv).quirks,
                (*formatter).stream,
            )
        };
        if ret != 0 {
            return ret;
        }
    }

    /* Finally, actually enable the formatter */
    if let Some(enable) = unsafe { (*(*(*formatter).drv).ops).enable } {
        unsafe {
            enable((*formatter).map);
        }
    }

    unsafe {
        (*formatter).enabled = true;
    }

    0
}

unsafe fn gx_formatter_disable(formatter: *mut gx_formatter) {
    /* Do nothing if the formatter is already disabled */
    if unsafe { !(*formatter).enabled } {
        return;
    }

    if let Some(disable) = unsafe { (*(*(*formatter).drv).ops).disable } {
        unsafe {
            disable((*formatter).map);
        }
    }

    unsafe {
        (*formatter).enabled = false;
    }
}

unsafe fn gx_formatter_attach(formatter: *mut gx_formatter) -> c_int {
    let ts: *mut gx_stream = unsafe { (*formatter).stream };
    let mut ret: c_int = 0;

    unsafe {
        mutex_lock(&mut (*ts).lock);
    }

    /* Catch up if the stream is already running when we attach */
    if unsafe { (*ts).ready } {
        ret = unsafe { gx_formatter_enable(formatter) };
        if ret != 0 {
            unsafe {
                pr_err(c"failed to enable formatter\n".as_ptr());
            }
            goto_out(formatter, ts, ret);
            return ret;
        }
    }

    unsafe {
        list_add_tail(&mut (*formatter).list, &mut (*ts).formatter_list);
    }

    unsafe {
        mutex_unlock(&mut (*ts).lock);
    }
    ret
}

unsafe fn goto_out(_formatter: *mut gx_formatter, ts: *mut gx_stream, _ret: c_int) {
    unsafe {
        mutex_unlock(&mut (*ts).lock);
    }
}

unsafe fn gx_formatter_detach(formatter: *mut gx_formatter) {
    let ts: *mut gx_stream = unsafe { (*formatter).stream };

    if ts.is_null() {
        return;
    }

    unsafe {
        mutex_lock(&mut (*ts).lock);
        list_del(&mut (*formatter).list);
        mutex_unlock(&mut (*ts).lock);

        gx_formatter_disable(formatter);
    }
}

unsafe fn gx_formatter_power_up(
    formatter: *mut gx_formatter,
    w: *mut snd_soc_dapm_widget,
) -> c_int {
    let ts: *mut gx_stream = unsafe { ((*(*(*formatter).drv).ops).get_stream)(w) };
    let ret: c_int;

    /*
     * If we don't get a stream at this stage, it would mean that the
     * widget is powering up but is not attached to any backend DAI.
     * It should not happen, ever !
     */
    if unsafe { WARN_ON(ts.is_null()) } {
        return -ENODEV;
    }

    unsafe {
        (*formatter).stream = ts;
        INIT_LIST_HEAD(&mut (*formatter).list);
    }
    ret = unsafe { gx_formatter_attach(formatter) };
    if ret != 0 {
        return ret;
    }

    0
}

unsafe fn gx_formatter_power_down(formatter: *mut gx_formatter) {
    unsafe {
        gx_formatter_detach(formatter);
        (*formatter).stream = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gx_formatter_event(
    w: *mut snd_soc_dapm_widget,
    _control: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let c: *mut snd_soc_component;
    let formatter: *mut gx_formatter;
    let mut ret: c_int = 0;

    c = unsafe { snd_soc_dapm_to_component((*w).dapm) };

    if unsafe { !(*w).priv_.is_null() } {
        formatter = unsafe { (*w).priv_ as *mut gx_formatter };
    } else {
        formatter = unsafe { snd_soc_component_get_drvdata(c) as *mut gx_formatter };
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            ret = unsafe { gx_formatter_power_up(formatter, w) };
        }

        SND_SOC_DAPM_PRE_PMD => {
            unsafe {
                gx_formatter_power_down(formatter);
            }
        }

        _ => {
            unsafe {
                dev_err((*c).dev, c"Unexpected event %d\n".as_ptr(), event);
            }
            return -EINVAL;
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gx_formatter_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let drv: *const gx_formatter_driver;
    let formatter: *mut gx_formatter;
    let regs: *mut c_void;

    drv = unsafe { of_device_get_match_data(dev) as *const gx_formatter_driver };
    if drv.is_null() {
        unsafe {
            dev_err(dev, c"failed to match device\n".as_ptr());
        }
        return -ENODEV;
    }

    formatter = unsafe {
        devm_kzalloc(dev, core::mem::size_of::<gx_formatter>(), GFP_KERNEL) as *mut gx_formatter
    };
    if formatter.is_null() {
        return -ENOMEM;
    }
    unsafe {
        platform_set_drvdata(pdev, formatter as *mut c_void);
        (*formatter).drv = drv;
    }

    regs = unsafe { devm_platform_ioremap_resource(pdev, 0) };
    if unsafe { IS_ERR(regs) } {
        return unsafe { PTR_ERR(regs) as c_int };
    }

    unsafe {
        (*formatter).map = devm_regmap_init_mmio(dev, regs, (*drv).regmap_cfg);
    }
    if unsafe { IS_ERR((*formatter).map as *const c_void) } {
        unsafe {
            dev_err(
                dev,
                c"failed to init regmap: %ld\n".as_ptr(),
                PTR_ERR((*formatter).map as *const c_void),
            );
        }
        return unsafe { PTR_ERR((*formatter).map as *const c_void) as c_int };
    }

    unsafe { devm_snd_soc_register_component(dev, (*drv).component_drv, ptr::null(), 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gx_formatter_create(
    dev: *mut device,
    w: *mut snd_soc_dapm_widget,
    drv: *const gx_formatter_driver,
    regmap: *mut regmap,
) -> c_int {
    let formatter: *mut gx_formatter;

    formatter = unsafe {
        devm_kzalloc(dev, core::mem::size_of::<gx_formatter>(), GFP_KERNEL) as *mut gx_formatter
    };
    if formatter.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*formatter).drv = drv;
        (*formatter).map = regmap;

        (*w).priv_ = formatter as *mut c_void;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gx_stream_start(ts: *mut gx_stream) -> c_int {
    let mut formatter: *mut gx_formatter;
    let mut ret: c_int = 0;

    unsafe {
        mutex_lock(&mut (*ts).lock);
    }

    /* Start all the formatters attached to the stream */
    unsafe {
        formatter = __list_for_each_entry_first(&mut (*ts).formatter_list);
        while !__list_for_each_entry_done(formatter, &mut (*ts).formatter_list) {
            ret = gx_formatter_enable(formatter);
            if ret != 0 {
                pr_err(c"failed to enable formatter\n".as_ptr());
                break;
            }
            formatter = __list_for_each_entry_next(formatter);
        }
    }

    if ret == 0 {
        unsafe {
            (*ts).ready = true;
        }
    }

    unsafe {
        mutex_unlock(&mut (*ts).lock);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gx_stream_stop(ts: *mut gx_stream) {
    let mut formatter: *mut gx_formatter;

    unsafe {
        mutex_lock(&mut (*ts).lock);

        (*ts).ready = false;
    }

    /* Stop all the formatters attached to the stream */
    unsafe {
        formatter = __list_for_each_entry_first(&mut (*ts).formatter_list);
        while !__list_for_each_entry_done(formatter, &mut (*ts).formatter_list) {
            gx_formatter_disable(formatter);
            formatter = __list_for_each_entry_next(formatter);
        }
    }

    unsafe {
        mutex_unlock(&mut (*ts).lock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gx_stream_alloc(iface: *mut gx_iface) -> *mut gx_stream {
    let ts: *mut gx_stream;

    ts = unsafe { kzalloc(core::mem::size_of::<gx_stream>(), GFP_KERNEL) as *mut gx_stream };
    if !ts.is_null() {
        unsafe {
            INIT_LIST_HEAD(&mut (*ts).formatter_list);
            mutex_init(&mut (*ts).lock);
            (*ts).iface = iface;
        }
    }

    ts
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gx_stream_free(ts: *mut gx_stream) {
    /*
     * If the list is not empty, it would mean that one of the formatter
     * widget is still powered and attached to the interface while we
     * are removing the TDM DAI. It should not be possible
     */
    unsafe {
        WARN_ON(list_empty(&(*ts).formatter_list) == 0);
        mutex_destroy(&mut (*ts).lock);
        kfree(ts as *mut c_void);
    }
}

// MODULE_DESCRIPTION("Amlogic GX formatter driver");
// MODULE_AUTHOR("Valerio Setti <vsetti@baylibre.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
