// SPDX-License-Identifier: GPL-2.0-only
/*
 * dptf_pch_fivr:  DPTF PCH FIVR Participant driver
 * Copyright (c) 2020, Intel Corporation.
 */

// Translated from the Linux kernel C implementation. Kernel and ACPI symbols
// referenced below are supplied by the surrounding build environment.

#[repr(C)]
pub struct pch_fivr_resp {
    pub status: u64,
    pub result: u64,
}

unsafe fn pch_fivr_read(
    handle: acpi_handle,
    method: *mut core::ffi::c_char,
    fivr_resp: *mut pch_fivr_resp,
) -> i32 {
    let mut resp = acpi_buffer {
        length: core::mem::size_of::<pch_fivr_resp>(),
        pointer: fivr_resp.cast(),
    };
    let mut buffer = acpi_buffer {
        length: ACPI_ALLOCATE_BUFFER,
        pointer: core::ptr::null_mut(),
    };
    let mut format = acpi_buffer {
        length: core::mem::size_of::<[u8; 3]>(),
        pointer: b"NN\0".as_ptr() as *mut core::ffi::c_void,
    };
    let mut obj: *mut acpi_object;
    let mut status: acpi_status;
    let mut ret: i32 = -EFAULT;

    status = acpi_evaluate_object(handle, method, core::ptr::null(), &mut buffer);
    if ACPI_FAILURE(status) {
        return ret;
    }

    obj = buffer.pointer.cast();
    if obj.is_null() || (*obj).type_ != ACPI_TYPE_PACKAGE {
        return release_buffer(&mut buffer, ret);
    }

    status = acpi_extract_package(obj, &mut format, &mut resp);
    if ACPI_FAILURE(status) {
        return release_buffer(&mut buffer, ret);
    }

    if (*fivr_resp).status != 0 {
        return release_buffer(&mut buffer, ret);
    }

    ret = 0;
    release_buffer(&mut buffer, ret)
}

unsafe fn release_buffer(buffer: *mut acpi_buffer, ret: i32) -> i32 {
    ACPI_FREE((*buffer).pointer);
    ret
}

unsafe fn freq_mhz_low_clock_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    pch_fivr_show(dev, buf, b"GFC0\0".as_ptr() as *mut core::ffi::c_char)
}

unsafe fn freq_mhz_high_clock_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    pch_fivr_show(dev, buf, b"GFC1\0".as_ptr() as *mut core::ffi::c_char)
}

unsafe fn ssc_clock_info_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    pch_fivr_show(dev, buf, b"GEMI\0".as_ptr() as *mut core::ffi::c_char)
}

unsafe fn fivr_switching_freq_mhz_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    pch_fivr_show(dev, buf, b"GFCS\0".as_ptr() as *mut core::ffi::c_char)
}

unsafe fn fivr_switching_fault_status_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> ssize_t {
    pch_fivr_show(dev, buf, b"GFFS\0".as_ptr() as *mut core::ffi::c_char)
}

unsafe fn pch_fivr_show(
    dev: *mut device,
    buf: *mut core::ffi::c_char,
    method: *mut core::ffi::c_char,
) -> ssize_t {
    let acpi_dev: *mut acpi_device = dev_get_drvdata(dev).cast();
    let mut fivr_resp = pch_fivr_resp { status: 0, result: 0 };
    let status = pch_fivr_read((*acpi_dev).handle, method, &mut fivr_resp);

    if status != 0 {
        return status as ssize_t;
    }

    sprintf(buf, b"%llu\n\0".as_ptr() as *const core::ffi::c_char, fivr_resp.result)
}

unsafe fn freq_mhz_low_clock_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const core::ffi::c_char,
    count: size_t,
) -> ssize_t {
    pch_fivr_store(dev, buf, count, b"RFC0\0".as_ptr() as *mut core::ffi::c_char)
}

unsafe fn freq_mhz_high_clock_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const core::ffi::c_char,
    count: size_t,
) -> ssize_t {
    pch_fivr_store(dev, buf, count, b"RFC1\0".as_ptr() as *mut core::ffi::c_char)
}

unsafe fn pch_fivr_store(
    dev: *mut device,
    buf: *const core::ffi::c_char,
    count: size_t,
    method: *mut core::ffi::c_char,
) -> ssize_t {
    let acpi_dev: *mut acpi_device = dev_get_drvdata(dev).cast();
    let mut val: u32 = 0;

    if kstrtouint(buf, 0, &mut val) < 0 {
        return -EINVAL as ssize_t;
    }

    let status = acpi_execute_simple_method((*acpi_dev).handle, method, val);
    if ACPI_SUCCESS(status) {
        return count as ssize_t;
    }

    -EINVAL as ssize_t
}

static mut fivr_attrs: [*mut attribute; 6] = [
    core::ptr::addr_of_mut!(dev_attr_freq_mhz_low_clock.attr),
    core::ptr::addr_of_mut!(dev_attr_freq_mhz_high_clock.attr),
    core::ptr::addr_of_mut!(dev_attr_ssc_clock_info.attr),
    core::ptr::addr_of_mut!(dev_attr_fivr_switching_freq_mhz.attr),
    core::ptr::addr_of_mut!(dev_attr_fivr_switching_fault_status.attr),
    core::ptr::null_mut(),
];

static pch_fivr_attribute_group: attribute_group = attribute_group {
    attrs: unsafe { core::ptr::addr_of_mut!(fivr_attrs).cast() },
    name: b"pch_fivr_switch_frequency\0".as_ptr() as *const core::ffi::c_char,
};

unsafe fn pch_fivr_add(pdev: *mut platform_device) -> i32 {
    let acpi_dev = ACPI_COMPANION(&mut (*pdev).dev);
    if acpi_dev.is_null() {
        return -ENODEV;
    }

    let mut ptype: u64 = 0;
    let status = acpi_evaluate_integer((*acpi_dev).handle, b"PTYP\0".as_ptr() as _, core::ptr::null(), &mut ptype);
    if ACPI_FAILURE(status) || ptype != 0x05 {
        return -ENODEV;
    }

    let result = sysfs_create_group(&mut (*pdev).dev.kobj, &pch_fivr_attribute_group);
    if result != 0 {
        return result;
    }

    platform_set_drvdata(pdev, acpi_dev.cast());
    0
}

unsafe fn pch_fivr_remove(pdev: *mut platform_device) {
    sysfs_remove_group(&mut (*pdev).dev.kobj, &pch_fivr_attribute_group);
}

static pch_fivr_device_ids: [acpi_device_id; 7] = [
    acpi_device_id { id: *b"INTC1045\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1049\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC1064\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC106B\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10A3\0", driver_data: 0 },
    acpi_device_id { id: *b"INTC10D7\0", driver_data: 0 },
    acpi_device_id { id: *b"\0", driver_data: 0 },
];

static mut pch_fivr_driver: platform_driver = platform_driver {
    probe: Some(pch_fivr_add),
    remove: Some(pch_fivr_remove),
    driver: driver {
        name: b"dptf_pch_fivr\0".as_ptr() as _,
        acpi_match_table: pch_fivr_device_ids.as_ptr(),
    },
};

// module_platform_driver!(pch_fivr_driver);
// MODULE_DEVICE_TABLE(acpi, pch_fivr_device_ids);
// MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("ACPI DPTF PCH FIVR driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
