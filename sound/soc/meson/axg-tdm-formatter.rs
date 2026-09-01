// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Translated from axg-tdm-formatter.c. C include dependencies are represented
// as opaque extern types, constants, and functions supplied by the surrounding
// kernel/audio driver code.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const AXG_TDM_NUM_LANES: usize = 4;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const SND_SOC_DAPM_PRE_PMU: c_int = 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 2;
const SND_SOC_DAIFMT_CONT: c_uint = 1 << 4;

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
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
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
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
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
pub struct axg_tdm_iface {
    pub fmt: c_uint,
    pub mclk: *mut clk,
    pub sclk: *mut clk,
    pub lrclk: *mut clk,
}

#[repr(C)]
pub struct axg_tdm_stream {
    pub formatter_list: list_head,
    pub lock: mutex,
    pub iface: *mut axg_tdm_iface,
    pub channels: c_uint,
    pub mask: [u32; AXG_TDM_NUM_LANES],
    pub ready: bool,
    pub clk_enabled: bool,
}

#[repr(C)]
pub struct axg_tdm_formatter_ops {
    pub prepare: unsafe extern "C" fn(
        map: *mut regmap,
        quirks: c_uint,
        stream: *mut axg_tdm_stream,
    ) -> c_int,
    pub enable: unsafe extern "C" fn(map: *mut regmap),
    pub disable: unsafe extern "C" fn(map: *mut regmap),
    pub get_stream: unsafe extern "C" fn(w: *mut snd_soc_dapm_widget) -> *mut axg_tdm_stream,
}

#[repr(C)]
pub struct axg_tdm_formatter_driver {
    pub ops: *const axg_tdm_formatter_ops,
    pub quirks: c_uint,
    pub regmap_cfg: *const regmap_config,
    pub component_drv: *const snd_soc_component_driver,
}

#[repr(C)]
pub struct axg_tdm_formatter {
    pub list: list_head,
    pub stream: *mut axg_tdm_stream,
    pub drv: *const axg_tdm_formatter_driver,
    pub pclk: *mut clk,
    pub sclk: *mut clk,
    pub lrclk: *mut clk,
    pub sclk_sel: *mut clk,
    pub lrclk_sel: *mut clk,
    pub reset: *mut reset_control,
    pub enabled: bool,
    pub map: *mut regmap,
}

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_get_reg_stride(map: *mut regmap) -> c_uint;
    fn reset_control_reset(rstc: *mut reset_control) -> c_int;
    fn axg_tdm_sclk_invert(fmt: c_uint) -> bool;
    fn clk_set_phase(clk: *mut clk, degrees: c_int) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_reset_control_get_optional_exclusive(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn WARN_ON(condition: bool) -> bool;
}

#[inline]
const fn BIT(nr: c_int) -> u32 {
    1u32 << nr
}

unsafe fn container_of_axg_tdm_formatter_list(ptr: *mut list_head) -> *mut axg_tdm_formatter {
    (ptr as *mut u8).sub(core::mem::offset_of!(axg_tdm_formatter, list)) as *mut axg_tdm_formatter
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_formatter_set_channel_masks(
    map: *mut regmap,
    ts: *mut axg_tdm_stream,
    mut offset: c_uint,
) -> c_int {
    let mut ch: c_uint = (*ts).channels;
    let mut val: [u32; AXG_TDM_NUM_LANES] = [0; AXG_TDM_NUM_LANES];
    let mut i: c_int;
    let mut j: usize;
    let mut k: c_int;

    /*
     * We need to mimick the slot distribution used by the HW to keep the
     * channel placement consistent regardless of the number of channel
     * in the stream. This is why the odd algorithm below is used.
     */
    memset(
        val.as_mut_ptr() as *mut c_void,
        0,
        size_of::<u32>() * AXG_TDM_NUM_LANES,
    );

    /*
     * Distribute the channels of the stream over the available slots
     * of each TDM lane. We need to go over the 32 slots ...
     */
    i = 0;
    while i < 32 && ch != 0 {
        /* ... of all the lanes ... */
        j = 0;
        while j < AXG_TDM_NUM_LANES {
            /* ... then distribute the channels in pairs */
            k = 0;
            while k < 2 {
                if (BIT(i + k) & (*ts).mask[j]) != 0 && ch != 0 {
                    val[j] |= BIT(i + k);
                    ch -= 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 2;
    }

    /*
     * If we still have channel left at the end of the process, it means
     * the stream has more channels than we can accommodate and we should
     * have caught this earlier.
     */
    if WARN_ON(ch != 0) {
        pr_err(c"channel mask error\n".as_ptr());
        return -EINVAL;
    }

    i = 0;
    while (i as usize) < AXG_TDM_NUM_LANES {
        regmap_write(map, offset, val[i as usize]);
        offset = offset.wrapping_add(regmap_get_reg_stride(map));
        i += 1;
    }

    0
}
// EXPORT_SYMBOL_GPL(axg_tdm_formatter_set_channel_masks);

unsafe fn axg_tdm_formatter_enable(formatter: *mut axg_tdm_formatter) -> c_int {
    let ts: *mut axg_tdm_stream = (*formatter).stream;
    let invert: bool;
    let mut ret: c_int;

    /* Do nothing if the formatter is already enabled */
    if (*formatter).enabled {
        return 0;
    }

    /*
     * On the g12a (and possibly other SoCs), when a stream using
     * multiple lanes is restarted, it will sometimes not start
     * from the first lane, but randomly from another used one.
     * The result is an unexpected and random channel shift.
     *
     * The hypothesis is that an HW counter is not properly reset
     * and the formatter simply starts on the lane it stopped
     * before. Unfortunately, there does not seems to be a way to
     * reset this through the registers of the block.
     *
     * However, the g12a has indenpendent reset lines for each audio
     * devices. Using this reset before each start solves the issue.
     */
    ret = reset_control_reset((*formatter).reset);
    if ret != 0 {
        return ret;
    }

    /*
     * If sclk is inverted, it means the bit should latched on the
     * rising edge which is what our HW expects. If not, we need to
     * invert it before the formatter.
     */
    invert = axg_tdm_sclk_invert((*(*ts).iface).fmt);
    ret = clk_set_phase((*formatter).sclk, if invert { 0 } else { 180 });
    if ret != 0 {
        return ret;
    }

    /* Setup the stream parameter in the formatter */
    ret = ((*(*(*formatter).drv).ops).prepare)(
        (*formatter).map,
        (*(*formatter).drv).quirks,
        (*formatter).stream,
    );
    if ret != 0 {
        return ret;
    }

    /* Enable the signal clocks feeding the formatter */
    ret = clk_prepare_enable((*formatter).sclk);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*formatter).lrclk);
    if ret != 0 {
        clk_disable_unprepare((*formatter).sclk);
        return ret;
    }

    /* Finally, actually enable the formatter */
    ((*(*(*formatter).drv).ops).enable)((*formatter).map);
    (*formatter).enabled = true;

    0
}

unsafe fn axg_tdm_formatter_disable(formatter: *mut axg_tdm_formatter) {
    /* Do nothing if the formatter is already disabled */
    if !(*formatter).enabled {
        return;
    }

    ((*(*(*formatter).drv).ops).disable)((*formatter).map);
    clk_disable_unprepare((*formatter).lrclk);
    clk_disable_unprepare((*formatter).sclk);
    (*formatter).enabled = false;
}

unsafe fn axg_tdm_formatter_attach(formatter: *mut axg_tdm_formatter) -> c_int {
    let ts: *mut axg_tdm_stream = (*formatter).stream;
    let mut ret: c_int = 0;

    mutex_lock(&mut (*ts).lock);

    /* Catch up if the stream is already running when we attach */
    if (*ts).ready {
        ret = axg_tdm_formatter_enable(formatter);
        if ret != 0 {
            pr_err(c"failed to enable formatter\n".as_ptr());
            mutex_unlock(&mut (*ts).lock);
            return ret;
        }
    }

    list_add_tail(&mut (*formatter).list, &mut (*ts).formatter_list);

    mutex_unlock(&mut (*ts).lock);
    ret
}

unsafe fn axg_tdm_formatter_dettach(formatter: *mut axg_tdm_formatter) {
    let ts: *mut axg_tdm_stream = (*formatter).stream;

    mutex_lock(&mut (*ts).lock);
    list_del(&mut (*formatter).list);
    mutex_unlock(&mut (*ts).lock);

    axg_tdm_formatter_disable(formatter);
}

unsafe fn axg_tdm_formatter_power_up(
    formatter: *mut axg_tdm_formatter,
    w: *mut snd_soc_dapm_widget,
) -> c_int {
    let ts: *mut axg_tdm_stream = ((*(*(*formatter).drv).ops).get_stream)(w);
    let mut ret: c_int;

    /*
     * If we don't get a stream at this stage, it would mean that the
     * widget is powering up but is not attached to any backend DAI.
     * It should not happen, ever !
     */
    if WARN_ON(ts.is_null()) {
        return -ENODEV;
    }

    /* Clock our device */
    ret = clk_prepare_enable((*formatter).pclk);
    if ret != 0 {
        return ret;
    }

    /* Reparent the bit clock to the TDM interface */
    ret = clk_set_parent((*formatter).sclk_sel, (*(*ts).iface).sclk);
    if ret != 0 {
        clk_disable_unprepare((*formatter).pclk);
        return ret;
    }

    /* Reparent the sample clock to the TDM interface */
    ret = clk_set_parent((*formatter).lrclk_sel, (*(*ts).iface).lrclk);
    if ret != 0 {
        clk_disable_unprepare((*formatter).pclk);
        return ret;
    }

    (*formatter).stream = ts;
    ret = axg_tdm_formatter_attach(formatter);
    if ret != 0 {
        clk_disable_unprepare((*formatter).pclk);
        return ret;
    }

    0
}

unsafe fn axg_tdm_formatter_power_down(formatter: *mut axg_tdm_formatter) {
    axg_tdm_formatter_dettach(formatter);
    clk_disable_unprepare((*formatter).pclk);
    (*formatter).stream = ptr::null_mut();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_formatter_event(
    w: *mut snd_soc_dapm_widget,
    _control: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let c: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let formatter: *mut axg_tdm_formatter =
        snd_soc_component_get_drvdata(c) as *mut axg_tdm_formatter;
    let mut ret: c_int = 0;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            ret = axg_tdm_formatter_power_up(formatter, w);
        }
        SND_SOC_DAPM_PRE_PMD => {
            axg_tdm_formatter_power_down(formatter);
        }
        _ => {
            dev_err((*c).dev, c"Unexpected event %d\n".as_ptr(), event);
            return -EINVAL;
        }
    }

    ret
}
// EXPORT_SYMBOL_GPL(axg_tdm_formatter_event);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_formatter_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let drv: *const axg_tdm_formatter_driver;
    let formatter: *mut axg_tdm_formatter;
    let regs: *mut c_void;

    drv = of_device_get_match_data(dev) as *const axg_tdm_formatter_driver;
    if drv.is_null() {
        dev_err(dev, c"failed to match device\n".as_ptr());
        return -ENODEV;
    }

    formatter = devm_kzalloc(dev, size_of::<axg_tdm_formatter>(), GFP_KERNEL)
        as *mut axg_tdm_formatter;
    if formatter.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, formatter as *mut c_void);
    (*formatter).drv = drv;

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs) as c_int;
    }

    (*formatter).map = devm_regmap_init_mmio(dev, regs, (*drv).regmap_cfg);
    if IS_ERR((*formatter).map as *const c_void) {
        dev_err(
            dev,
            c"failed to init regmap: %ld\n".as_ptr(),
            PTR_ERR((*formatter).map as *const c_void),
        );
        return PTR_ERR((*formatter).map as *const c_void) as c_int;
    }

    /* Peripharal clock */
    (*formatter).pclk = devm_clk_get(dev, c"pclk".as_ptr());
    if IS_ERR((*formatter).pclk as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*formatter).pclk as *const c_void),
            c"failed to get pclk\n".as_ptr(),
        );
    }

    /* Formatter bit clock */
    (*formatter).sclk = devm_clk_get(dev, c"sclk".as_ptr());
    if IS_ERR((*formatter).sclk as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*formatter).sclk as *const c_void),
            c"failed to get sclk\n".as_ptr(),
        );
    }

    /* Formatter sample clock */
    (*formatter).lrclk = devm_clk_get(dev, c"lrclk".as_ptr());
    if IS_ERR((*formatter).lrclk as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*formatter).lrclk as *const c_void),
            c"failed to get lrclk\n".as_ptr(),
        );
    }

    /* Formatter bit clock input multiplexer */
    (*formatter).sclk_sel = devm_clk_get(dev, c"sclk_sel".as_ptr());
    if IS_ERR((*formatter).sclk_sel as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*formatter).sclk_sel as *const c_void),
            c"failed to get sclk_sel\n".as_ptr(),
        );
    }

    /* Formatter sample clock input multiplexer */
    (*formatter).lrclk_sel = devm_clk_get(dev, c"lrclk_sel".as_ptr());
    if IS_ERR((*formatter).lrclk_sel as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*formatter).lrclk_sel as *const c_void),
            c"failed to get lrclk_sel\n".as_ptr(),
        );
    }

    /* Formatter dedicated reset line */
    (*formatter).reset = devm_reset_control_get_optional_exclusive(dev, ptr::null());
    if IS_ERR((*formatter).reset as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*formatter).reset as *const c_void),
            c"failed to get reset\n".as_ptr(),
        );
    }

    devm_snd_soc_register_component(dev, (*drv).component_drv, ptr::null_mut(), 0)
}
// EXPORT_SYMBOL_GPL(axg_tdm_formatter_probe);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_stream_start(ts: *mut axg_tdm_stream) -> c_int {
    let mut formatter: *mut axg_tdm_formatter;
    let mut ret: c_int = 0;

    mutex_lock(&mut (*ts).lock);
    (*ts).ready = true;

    /* Start all the formatters attached to the stream */
    let mut pos: *mut list_head = (*ts).formatter_list.next;
    while pos != &mut (*ts).formatter_list {
        formatter = container_of_axg_tdm_formatter_list(pos);
        ret = axg_tdm_formatter_enable(formatter);
        if ret != 0 {
            pr_err(c"failed to start tdm stream\n".as_ptr());
            mutex_unlock(&mut (*ts).lock);
            return ret;
        }
        pos = (*pos).next;
    }

    mutex_unlock(&mut (*ts).lock);
    ret
}
// EXPORT_SYMBOL_GPL(axg_tdm_stream_start);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_stream_stop(ts: *mut axg_tdm_stream) {
    let mut formatter: *mut axg_tdm_formatter;

    mutex_lock(&mut (*ts).lock);
    (*ts).ready = false;

    /* Stop all the formatters attached to the stream */
    let mut pos: *mut list_head = (*ts).formatter_list.next;
    while pos != &mut (*ts).formatter_list {
        formatter = container_of_axg_tdm_formatter_list(pos);
        axg_tdm_formatter_disable(formatter);
        pos = (*pos).next;
    }

    mutex_unlock(&mut (*ts).lock);
}
// EXPORT_SYMBOL_GPL(axg_tdm_stream_stop);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_stream_alloc(iface: *mut axg_tdm_iface) -> *mut axg_tdm_stream {
    let ts: *mut axg_tdm_stream;

    ts = kzalloc(size_of::<axg_tdm_stream>(), GFP_KERNEL) as *mut axg_tdm_stream;
    if !ts.is_null() {
        (*ts).formatter_list.next = &mut (*ts).formatter_list;
        (*ts).formatter_list.prev = &mut (*ts).formatter_list;
        mutex_init(&mut (*ts).lock);
        (*ts).iface = iface;
    }

    ts
}
// EXPORT_SYMBOL_GPL(axg_tdm_stream_alloc);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_stream_free(ts: *mut axg_tdm_stream) {
    /*
     * If the list is not empty, it would mean that one of the formatter
     * widget is still powered and attached to the interface while we
     * are removing the TDM DAI. It should not be possible
     */
    WARN_ON(list_empty(&(*ts).formatter_list) == 0);
    mutex_destroy(&mut (*ts).lock);
    kfree(ts as *mut c_void);
}
// EXPORT_SYMBOL_GPL(axg_tdm_stream_free);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn axg_tdm_stream_set_cont_clocks(
    ts: *mut axg_tdm_stream,
    fmt: c_uint,
) -> c_int {
    let mut ret: c_int = 0;

    if (fmt & SND_SOC_DAIFMT_CONT) != 0 {
        /* Clock are already enabled - skipping */
        if (*ts).clk_enabled {
            return 0;
        }

        ret = clk_prepare_enable((*(*ts).iface).mclk);
        if ret != 0 {
            return ret;
        }

        ret = clk_prepare_enable((*(*ts).iface).sclk);
        if ret != 0 {
            clk_disable_unprepare((*(*ts).iface).mclk);
            (*ts).clk_enabled = false;
            return ret;
        }

        ret = clk_prepare_enable((*(*ts).iface).lrclk);
        if ret != 0 {
            clk_disable_unprepare((*(*ts).iface).sclk);
            clk_disable_unprepare((*(*ts).iface).mclk);
            (*ts).clk_enabled = false;
            return ret;
        }

        (*ts).clk_enabled = true;
        return 0;
    }

    /* Clocks are already disabled - skipping */
    if !(*ts).clk_enabled {
        return 0;
    }

    clk_disable_unprepare((*(*ts).iface).lrclk);
    clk_disable_unprepare((*(*ts).iface).sclk);
    clk_disable_unprepare((*(*ts).iface).mclk);
    (*ts).clk_enabled = false;
    ret
}
// EXPORT_SYMBOL_GPL(axg_tdm_stream_set_cont_clocks);

// MODULE_DESCRIPTION("Amlogic AXG TDM formatter driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
