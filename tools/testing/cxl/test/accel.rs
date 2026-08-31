// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2026 Intel Corporation. All rights reserved.

// C dependencies from the original source:
// linux/platform_device.h, linux/mod_devicetable.h, linux/module.h,
// linux/sizes.h, cxl/mailbox.h, cxlmem.h

#[repr(C)]
pub struct mock_cxl_accel {
    pub cxlds: cxl_dev_state,
    pub cxlmd: *mut cxl_memdev,
}

extern "C" {
    static KBUILD_MODNAME: *const ::core::ffi::c_char;

    fn devm_cxl_dev_state_create(
        dev: *mut device,
        devtype: ::core::ffi::c_int,
        id: ::core::ffi::c_int,
        serial: ::core::ffi::c_int,
        private_size: usize,
        cxlds_offset: usize,
        locked: bool,
    ) -> *mut mock_cxl_accel;

    fn cxl_set_capacity(cxlds: *mut cxl_dev_state, capacity: usize) -> ::core::ffi::c_int;
    fn devm_cxl_probe_mem(cxlds: *mut cxl_dev_state, range: *mut range) -> *mut cxl_memdev;
    fn IS_ERR(ptr: *const ::core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn dev_dbg(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
}

const CXL_DEVTYPE_DEVMEM: ::core::ffi::c_int = 0;
const ENOMEM: ::core::ffi::c_int = 12;
const SZ_512M: usize = 512 * 1024 * 1024;
const PROBE_PREFER_ASYNCHRONOUS: ::core::ffi::c_int = 1;

#[repr(C)]
pub struct cxl_dev_state {
    pub media_ready: bool,
}

#[repr(C)]
pub struct cxl_memdev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id: ::core::ffi::c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [::core::ffi::c_char; 20],
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> ::core::ffi::c_int>,
    pub id_table: *const platform_device_id,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
    pub probe_type: ::core::ffi::c_int,
}

unsafe extern "C" fn cxl_mock_accel_probe(
    pdev: *mut platform_device,
) -> ::core::ffi::c_int {
    let mut cxl_accel: *mut mock_cxl_accel;
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let mut cxlds: *mut cxl_dev_state;
    let mut cxlmd: *mut cxl_memdev;
    let mut mock_range: range = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int;

    cxl_accel = unsafe {
        devm_cxl_dev_state_create(
            &mut (*pdev).dev,
            CXL_DEVTYPE_DEVMEM,
            (*pdev).id + 1,
            0,
            ::core::mem::size_of::<mock_cxl_accel>(),
            ::core::mem::offset_of!(mock_cxl_accel, cxlds),
            false,
        )
    };
    if cxl_accel.is_null() {
        return -ENOMEM;
    }

    cxlds = unsafe { &mut (*cxl_accel).cxlds };
    unsafe {
        (*cxlds).media_ready = true;
    }
    rc = unsafe { cxl_set_capacity(cxlds, SZ_512M) };
    if rc != 0 {
        return rc;
    }

    cxlmd = unsafe { devm_cxl_probe_mem(cxlds, &mut mock_range) };
    if unsafe { IS_ERR(cxlmd as *const ::core::ffi::c_void) } {
        return unsafe { PTR_ERR(cxlmd as *const ::core::ffi::c_void) };
    }
    unsafe {
        (*cxl_accel).cxlmd = cxlmd;
    }

    unsafe {
        dev_dbg(
            dev,
            c"Probed mock accelerator with range %pra\n".as_ptr(),
            &mut mock_range,
        );
    }

    0
}

#[no_mangle]
pub static cxl_mock_accel_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'c' as ::core::ffi::c_char,
            b'x' as ::core::ffi::c_char,
            b'l' as ::core::ffi::c_char,
            b'_' as ::core::ffi::c_char,
            b't' as ::core::ffi::c_char,
            b'y' as ::core::ffi::c_char,
            b'p' as ::core::ffi::c_char,
            b'e' as ::core::ffi::c_char,
            b'2' as ::core::ffi::c_char,
            b'_' as ::core::ffi::c_char,
            b'a' as ::core::ffi::c_char,
            b'c' as ::core::ffi::c_char,
            b'c' as ::core::ffi::c_char,
            b'e' as ::core::ffi::c_char,
            b'l' as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    platform_device_id { name: [0; 20] },
];

// MODULE_DEVICE_TABLE(platform, cxl_mock_accel_ids);

#[no_mangle]
pub static mut cxl_mock_accel_driver: platform_driver = platform_driver {
    probe: Some(cxl_mock_accel_probe),
    id_table: unsafe { cxl_mock_accel_ids.as_ptr() },
    driver: device_driver {
        name: unsafe { KBUILD_MODNAME },
        probe_type: PROBE_PREFER_ASYNCHRONOUS,
    },
};

// module_platform_driver(cxl_mock_accel_driver);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("cxl_test: accelerator device mock module");
// MODULE_IMPORT_NS("CXL");
