// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for a 7-segment LED display
 *
 * The decimal point LED present on some devices is currently not
 * supported.
 *
 * Copyright (C) Allied Telesis Labs
 */

// External Linux kernel declarations supplied by the surrounding build.
use core::ffi::c_void;

#[repr(C)]
pub struct linedisp {
    pub map: *mut linedisp_map,
    pub buf: [u8; 1],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_descs {
    pub ndescs: usize,
}

#[repr(C)]
pub struct seg7_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct linedisp_map {
    pub map: linedisp_map_union,
}

#[repr(C)]
pub union linedisp_map_union {
    pub seg7: seg7_map,
}

#[repr(C)]
pub struct linedisp_ops {
    pub get_map_type: Option<unsafe extern "C" fn(*mut linedisp) -> i32>,
    pub update: Option<unsafe extern "C" fn(*mut linedisp)>,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

extern "C" {
    fn bitmap_set_value8(bitmap: *mut u8, value: u8, start: u32);
    fn map_to_seg7(map: *const seg7_map, value: u8) -> u8;
    fn gpiod_multi_set_value_cansleep(descs: *mut gpio_descs, values: *const u8);
    fn init_delayed_work(work: *mut delayed_work, function: unsafe extern "C" fn(*mut work_struct));
    fn schedule_delayed_work(work: *mut delayed_work, delay: u64) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn devm_gpiod_get_array(dev: *mut device, con_id: *const i8, flags: u32) -> *mut gpio_descs;
    fn ptr_err(ptr: *mut gpio_descs) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn linedisp_register(
        linedisp: *mut linedisp,
        dev: *mut device,
        num: u32,
        ops: *const linedisp_ops,
    ) -> i32;
    fn linedisp_unregister(linedisp: *mut linedisp);
}

const LINEDISP_MAP_SEG7: i32 = 0;
const GFP_KERNEL: u32 = 0;
const GPIOD_OUT_LOW: u32 = 0;

#[repr(C)]
pub struct seg_led_priv {
    pub linedisp: linedisp,
    pub work: delayed_work,
    pub segment_gpios: *mut gpio_descs,
}

unsafe extern "C" fn seg_led_update(work: *mut work_struct) {
    let priv_ = (work as *mut u8).sub(core::mem::offset_of!(seg_led_priv, work) + core::mem::offset_of!(delayed_work, work)) as *mut seg_led_priv;
    let linedisp = &mut (*priv_).linedisp;
    let map = (*linedisp).map;
    let mut values = [0u8; 1];

    bitmap_set_value8(values.as_mut_ptr(), map_to_seg7(&(*map).map.seg7, (*linedisp).buf[0]), 0);

    gpiod_multi_set_value_cansleep((*priv_).segment_gpios, values.as_ptr());
}

unsafe extern "C" fn seg_led_linedisp_get_map_type(linedisp: *mut linedisp) -> i32 {
    let priv_ = (linedisp as *mut u8).sub(core::mem::offset_of!(seg_led_priv, linedisp)) as *mut seg_led_priv;

    init_delayed_work(&mut (*priv_).work, seg_led_update);
    LINEDISP_MAP_SEG7
}

unsafe extern "C" fn seg_led_linedisp_update(linedisp: *mut linedisp) {
    let priv_ = (linedisp as *mut u8).sub(core::mem::offset_of!(seg_led_priv, linedisp)) as *mut seg_led_priv;

    schedule_delayed_work(&mut (*priv_).work, 0);
}

static SEG_LED_LINEDISP_OPS: linedisp_ops = linedisp_ops {
    get_map_type: Some(seg_led_linedisp_get_map_type),
    update: Some(seg_led_linedisp_update),
};

unsafe extern "C" fn seg_led_probe(pdev: *mut platform_device) -> i32 {
    let mut priv_: *mut seg_led_priv;
    let dev = &mut (*pdev).dev as *mut device;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<seg_led_priv>(), GFP_KERNEL) as *mut seg_led_priv;
    if priv_.is_null() {
        return -12;
    }

    platform_set_drvdata(pdev, priv_ as *mut c_void);

    static SEGMENT: &[u8] = b"segment\0";
    (*priv_).segment_gpios = devm_gpiod_get_array(dev, SEGMENT.as_ptr() as *const i8, GPIOD_OUT_LOW);
    if (*priv_).segment_gpios as isize < 0 {
        return ptr_err((*priv_).segment_gpios);
    }

    if (*(*priv_).segment_gpios).ndescs < 7 || (*(*priv_).segment_gpios).ndescs > 8 {
        return -22;
    }

    linedisp_register(&mut (*priv_).linedisp, dev, 1, &SEG_LED_LINEDISP_OPS)
}

unsafe extern "C" fn seg_led_remove(pdev: *mut platform_device) {
    let priv_ = platform_get_drvdata(pdev) as *mut seg_led_priv;

    cancel_delayed_work_sync(&mut (*priv_).work);
    linedisp_unregister(&mut (*priv_).linedisp);
}

// Device-tree match table: { .compatible = "gpio-7-segment" }, {}
// MODULE_DEVICE_TABLE(of, seg_led_of_match);
// module_platform_driver(seg_led_driver);
// MODULE_AUTHOR("Chris Packham <chris.packham@alliedtelesis.co.nz>");
// MODULE_DESCRIPTION("7 segment LED driver");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("LINEDISP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
