// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) Intel Corporation, 2022
// Copyright Everest Semiconductor Co.,Ltd

// C includes translated as external dependency expectations:
// linux/module.h, linux/acpi.h, and "es83xx-dsm-common.h".

/* UUID ("a9800c04-e016-343e-41f4-6bcce70f4332") */
#[repr(C)]
pub struct guid_t {
    pub b: [u8; 16],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type acpi_handle = *mut core::ffi::c_void;

#[repr(C)]
pub struct acpi_object_integer {
    pub value: u64,
}

#[repr(C)]
pub union acpi_object_data {
    pub integer: acpi_object_integer,
}

#[repr(C)]
pub struct acpi_object {
    pub type_: u32,
    pub data: acpi_object_data,
}

const ACPI_TYPE_INTEGER: u32 = 1;
const ENOENT: i32 = 2;
const EINVAL: i32 = 22;

const ES83xx_DSM_REVID: u64 = 1;

static es83xx_dsm_guid: guid_t = guid_t {
    b: [
        0x04, 0x0c, 0x80, 0xa9, 0x16, 0xe0, 0x3e, 0x34, 0x41, 0xf4, 0x6b, 0xcc, 0xe7, 0x0f,
        0x43, 0x32,
    ],
};

extern "C" {
    fn ACPI_HANDLE(dev: *mut device) -> acpi_handle;
    fn acpi_evaluate_dsm(
        handle: acpi_handle,
        guid: *const guid_t,
        rev: u64,
        func: u64,
        argv4: *mut core::ffi::c_void,
    ) -> *mut acpi_object;
    fn ACPI_FREE(obj: *mut acpi_object);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

extern "C" {
    static PLATFORM_MAINMIC_TYPE_ARG: i32;
    static PLATFORM_HPMIC_TYPE_ARG: i32;
    static PLATFORM_SPK_TYPE_ARG: i32;
    static PLATFORM_HPDET_INV_ARG: i32;
    static PLATFORM_PCM_TYPE_ARG: i32;
    static PLATFORM_MIC_DE_POP_ARG: i32;
}

#[no_mangle]
pub unsafe extern "C" fn es83xx_dsm(
    dev: *mut device,
    arg: core::ffi::c_int,
    value: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let dhandle: acpi_handle;
    let obj: *mut acpi_object;
    let mut ret: core::ffi::c_int = 0;

    dhandle = ACPI_HANDLE(dev);
    if dhandle.is_null() {
        return -ENOENT;
    }

    obj = acpi_evaluate_dsm(
        dhandle,
        &es83xx_dsm_guid as *const guid_t,
        ES83xx_DSM_REVID,
        arg as u64,
        core::ptr::null_mut(),
    );
    if obj.is_null() {
        dev_err(
            dev,
            b"%s: acpi_evaluate_dsm() failed\n\0".as_ptr() as *const core::ffi::c_char,
            b"es83xx_dsm\0".as_ptr() as *const core::ffi::c_char,
        );
        ret = -EINVAL;
        return ret;
    }

    if (*obj).type_ != ACPI_TYPE_INTEGER {
        dev_err(
            dev,
            b"%s: object is not ACPI_TYPE_INTEGER\n\0".as_ptr() as *const core::ffi::c_char,
            b"es83xx_dsm\0".as_ptr() as *const core::ffi::c_char,
        );
        ret = -EINVAL;
        ACPI_FREE(obj);
        return ret;
    }

    *value = (*obj).data.integer.value as core::ffi::c_int;
    ACPI_FREE(obj);
    ret
}

// EXPORT_SYMBOL_GPL(es83xx_dsm);

#[no_mangle]
pub unsafe extern "C" fn es83xx_dsm_dump(dev: *mut device) -> core::ffi::c_int {
    let mut value: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int;

    ret = es83xx_dsm(dev, PLATFORM_MAINMIC_TYPE_ARG, &mut value);
    if ret < 0 {
        return ret;
    }
    dev_info(
        dev,
        b"PLATFORM_MAINMIC_TYPE %#x\n\0".as_ptr() as *const core::ffi::c_char,
        value,
    );

    ret = es83xx_dsm(dev, PLATFORM_HPMIC_TYPE_ARG, &mut value);
    if ret < 0 {
        return ret;
    }
    dev_info(
        dev,
        b"PLATFORM_HPMIC_TYPE %#x\n\0".as_ptr() as *const core::ffi::c_char,
        value,
    );

    ret = es83xx_dsm(dev, PLATFORM_SPK_TYPE_ARG, &mut value);
    if ret < 0 {
        return ret;
    }
    dev_info(
        dev,
        b"PLATFORM_SPK_TYPE %#x\n\0".as_ptr() as *const core::ffi::c_char,
        value,
    );

    ret = es83xx_dsm(dev, PLATFORM_HPDET_INV_ARG, &mut value);
    if ret < 0 {
        return ret;
    }
    dev_info(
        dev,
        b"PLATFORM_HPDET_INV %#x\n\0".as_ptr() as *const core::ffi::c_char,
        value,
    );

    ret = es83xx_dsm(dev, PLATFORM_PCM_TYPE_ARG, &mut value);
    if ret < 0 {
        return ret;
    }
    dev_info(
        dev,
        b"PLATFORM_PCM_TYPE %#x\n\0".as_ptr() as *const core::ffi::c_char,
        value,
    );

    ret = es83xx_dsm(dev, PLATFORM_MIC_DE_POP_ARG, &mut value);
    if ret < 0 {
        return ret;
    }
    dev_info(
        dev,
        b"PLATFORM_MIC_DE_POP %#x\n\0".as_ptr() as *const core::ffi::c_char,
        value,
    );

    0
}

// EXPORT_SYMBOL_GPL(es83xx_dsm_dump);

// MODULE_DESCRIPTION("Everest Semi ES83xx DSM helpers");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
