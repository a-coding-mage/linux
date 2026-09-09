// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Synopsys, Inc. and/or its affiliates.

// Dependencies supplied by the Linux kernel headers:
// linux/array_size.h, linux/regmap.h, linux/i3c/device.h,
// linux/i3c/master.h, linux/module.h

use core::ffi::c_void;

extern "C" {
    fn dev_to_i3cdev(dev: *mut device) -> *mut i3c_device;
    fn i3c_device_do_xfers(
        i3c: *mut i3c_device,
        xfers: *mut i3c_xfer,
        nxfers: usize,
        mode: u32,
    ) -> i32;
    fn __regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        context: *mut c_void,
        config: *const regmap_config,
        lock_key: *mut lock_class_key,
        lock_name: *const i8,
    ) -> *mut regmap;
    fn __devm_regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        context: *mut c_void,
        config: *const regmap_config,
        lock_key: *mut lock_class_key,
        lock_name: *const i8,
    ) -> *mut regmap;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i3c_device {
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
pub struct lock_class_key {
    _private: [u8; 0],
}

#[repr(C)]
pub union i3c_xfer_data {
    pub out: *const c_void,
    pub r#in: *mut c_void,
}

#[repr(C)]
pub struct i3c_xfer {
    pub rnw: bool,
    pub len: usize,
    pub data: i3c_xfer_data,
}

#[repr(C)]
pub struct regmap_bus {
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> i32>,
    pub read: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const c_void,
            usize,
            *mut c_void,
            usize,
        ) -> i32,
    >,
}

const I3C_SDR: u32 = 0;

unsafe extern "C" fn regmap_i3c_write(
    context: *mut c_void,
    data: *const c_void,
    count: usize,
) -> i32 {
    let dev = context as *mut device;
    let i3c = dev_to_i3cdev(dev);
    let mut xfers = [i3c_xfer {
        rnw: false,
        len: count,
        data: i3c_xfer_data { out: data },
    }];

    i3c_device_do_xfers(i3c, xfers.as_mut_ptr(), xfers.len(), I3C_SDR)
}

unsafe extern "C" fn regmap_i3c_read(
    context: *mut c_void,
    reg: *const c_void,
    reg_size: usize,
    val: *mut c_void,
    val_size: usize,
) -> i32 {
    let dev = context as *mut device;
    let i3c = dev_to_i3cdev(dev);
    let mut xfers: [i3c_xfer; 2] = [
        i3c_xfer {
            rnw: false,
            len: reg_size,
            data: i3c_xfer_data { out: reg },
        },
        i3c_xfer {
            rnw: true,
            len: val_size,
            data: i3c_xfer_data { r#in: val },
        },
    ];

    i3c_device_do_xfers(i3c, xfers.as_mut_ptr(), xfers.len(), I3C_SDR)
}

static REGMAP_I3C: regmap_bus = regmap_bus {
    write: Some(regmap_i3c_write),
    read: Some(regmap_i3c_read),
};

#[no_mangle]
pub unsafe extern "C" fn __regmap_init_i3c(
    i3c: *mut i3c_device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const i8,
) -> *mut regmap {
    __regmap_init(
        &mut (*i3c).dev,
        &REGMAP_I3C,
        &mut (*i3c).dev as *mut device as *mut c_void,
        config,
        lock_key,
        lock_name,
    )
}

#[no_mangle]
pub unsafe extern "C" fn __devm_regmap_init_i3c(
    i3c: *mut i3c_device,
    config: *const regmap_config,
    lock_key: *mut lock_class_key,
    lock_name: *const i8,
) -> *mut regmap {
    __devm_regmap_init(
        &mut (*i3c).dev,
        &REGMAP_I3C,
        &mut (*i3c).dev as *mut device as *mut c_void,
        config,
        lock_key,
        lock_name,
    )
}

// EXPORT_SYMBOL_GPL(__regmap_init_i3c);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_i3c);
// MODULE_AUTHOR("Vitor Soares <vitor.soares@synopsys.com>");
// MODULE_DESCRIPTION("regmap I3C Module");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
