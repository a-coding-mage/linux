// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Secure Processor device driver
 *
 * Copyright (C) 2014,2018 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct SpPlatform {
    coherent: ::core::ffi::c_int,
    irq_count: ::core::ffi::c_uint,
}

static DEV_VDATA: [SpDevVdata; 1] = [SpDevVdata {
    bar: 0,
    // CONFIG_CRYPTO_DEV_SP_CCP: ccp_vdata: &ccpv3_platform,
}];

static SP_ACPI_MATCH: [AcpiDeviceId; 2] = [
    AcpiDeviceId { name: *b"AMDI0C00\0", driver_data: &DEV_VDATA[0] as *const _ as KernelUlong },
    AcpiDeviceId { name: [0; 9], driver_data: 0 },
];

static SP_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"amd,ccp-seattle-v1a\0".as_ptr() as *const _,
        data: &DEV_VDATA[0] as *const _ as *const ::core::ffi::c_void,
    },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn sp_get_acpi_version(pdev: *mut PlatformDevice) -> *const SpDevVdata {
    let device = &mut (*pdev).dev;
    let match_entry = acpi_match_device(SP_ACPI_MATCH.as_ptr(), device);
    if !match_entry.is_null() && (*match_entry).driver_data != 0 {
        return (*match_entry).driver_data as *const SpDevVdata;
    }
    core::ptr::null()
}

unsafe fn sp_get_irqs(sp: *mut SpDevice) -> ::core::ffi::c_int {
    let sp_platform = (*sp).dev_specific as *mut SpPlatform;
    let dev = (*sp).dev;
    let pdev = to_platform_device(dev);
    let mut ret: ::core::ffi::c_int;

    (*sp_platform).irq_count = platform_irq_count(pdev) as ::core::ffi::c_uint;

    ret = platform_get_irq(pdev, 0);
    if ret < 0 {
        dev_notice(dev, b"unable to get IRQ (%d)\n\0".as_ptr() as *const _, ret);
        return ret;
    }

    (*sp).psp_irq = ret;
    if (*sp_platform).irq_count == 1 {
        (*sp).ccp_irq = ret;
    } else {
        ret = platform_get_irq(pdev, 1);
        if ret < 0 {
            dev_notice(dev, b"unable to get IRQ (%d)\n\0".as_ptr() as *const _, ret);
            return ret;
        }
        (*sp).ccp_irq = ret;
    }
    0
}

unsafe fn sp_platform_probe(pdev: *mut PlatformDevice) -> ::core::ffi::c_int {
    let dev = &mut (*pdev).dev as *mut Device;
    let mut ret = -ENOMEM;
    let sp = sp_alloc_struct(dev);
    if sp.is_null() { return sp_probe_error(dev, ret); }

    let sp_platform = devm_kzalloc(dev, core::mem::size_of::<SpPlatform>(), GFP_KERNEL)
        as *mut SpPlatform;
    if sp_platform.is_null() { return sp_probe_error(dev, ret); }

    (*sp).dev_specific = sp_platform as *mut _;
    (*sp).dev_vdata = if !(*dev).of_node.is_null() {
        of_device_get_match_data(dev)
    } else {
        sp_get_acpi_version(pdev)
    };
    if (*sp).dev_vdata.is_null() {
        ret = -ENODEV;
        dev_err(dev, b"missing driver data\n\0".as_ptr() as *const _);
        return sp_probe_error(dev, ret);
    }

    (*sp).io_map = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*sp).io_map) {
        ret = ptr_err((*sp).io_map);
        return sp_probe_error(dev, ret);
    }

    let attr = device_get_dma_attr(dev);
    if attr == DEV_DMA_NOT_SUPPORTED {
        dev_err(dev, b"DMA is not supported\0".as_ptr() as *const _);
        return sp_probe_error(dev, ret);
    }
    (*sp_platform).coherent = (attr == DEV_DMA_COHERENT) as ::core::ffi::c_int;
    (*sp).axcache = if (*sp_platform).coherent != 0 { CACHE_WB_NO_ALLOC } else { CACHE_NONE };

    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(48));
    if ret != 0 {
        dev_err(dev, b"dma_set_mask_and_coherent failed (%d)\n\0".as_ptr() as *const _, ret);
        return sp_probe_error(dev, ret);
    }
    ret = sp_get_irqs(sp);
    if ret != 0 { return sp_probe_error(dev, ret); }
    dev_set_drvdata(dev, sp as *mut _);
    ret = sp_init(sp);
    if ret != 0 { return sp_probe_error(dev, ret); }
    dev_notice(dev, b"enabled\n\0".as_ptr() as *const _);
    0
}

unsafe fn sp_probe_error(dev: *mut Device, ret: ::core::ffi::c_int) -> ::core::ffi::c_int {
    dev_notice(dev, b"initialization failed\n\0".as_ptr() as *const _);
    ret
}

unsafe fn sp_platform_remove(pdev: *mut PlatformDevice) {
    let dev = &mut (*pdev).dev as *mut Device;
    let sp = dev_get_drvdata(dev) as *mut SpDevice;
    sp_destroy(sp);
    dev_notice(dev, b"disabled\n\0".as_ptr() as *const _);
}

// CONFIG_PM: preserve the suspend and resume callbacks.
unsafe fn sp_platform_suspend(pdev: *mut PlatformDevice, _state: PmMessage) -> ::core::ffi::c_int {
    sp_suspend(dev_get_drvdata(&mut (*pdev).dev) as *mut SpDevice)
}

unsafe fn sp_platform_resume(pdev: *mut PlatformDevice) -> ::core::ffi::c_int {
    sp_resume(dev_get_drvdata(&mut (*pdev).dev) as *mut SpDevice)
}

static mut SP_PLATFORM_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: b"ccp\0".as_ptr() as *const _,
        acpi_match_table: SP_ACPI_MATCH.as_ptr(),
        of_match_table: SP_OF_MATCH.as_ptr(),
    },
    probe: Some(sp_platform_probe),
    remove: Some(sp_platform_remove),
    // CONFIG_PM: suspend: Some(sp_platform_suspend), resume: Some(sp_platform_resume),
};

unsafe fn sp_platform_init() -> ::core::ffi::c_int {
    platform_driver_register(&mut SP_PLATFORM_DRIVER)
}

unsafe fn sp_platform_exit() {
    platform_driver_unregister(&mut SP_PLATFORM_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
