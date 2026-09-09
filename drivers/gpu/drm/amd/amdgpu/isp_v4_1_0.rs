/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2024 Advanced Micro Devices, Inc. All rights reserved.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// amdgpu.h, isp_v4_1_0.h

static ISP_4_1_0_INT_SRCID: [::core::ffi::c_uint; MAX_ISP410_INT_SRC] = [
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT9,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT10,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT11,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT12,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT13,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT14,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT15,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT16,
];

unsafe fn isp_v4_1_0_hw_init(isp: *mut amdgpu_isp) -> ::core::ffi::c_int {
    let adev = (*isp).adev;
    let mut idx: ::core::ffi::c_int;
    let mut int_idx: ::core::ffi::c_int;
    let mut num_res: ::core::ffi::c_int;
    let mut r: ::core::ffi::c_int;
    let isp_base: u64;
    let cleanup = |r: ::core::ffi::c_int| {
        kfree((*isp).isp_pdata as *mut ::core::ffi::c_void);
        kfree((*isp).isp_res as *mut ::core::ffi::c_void);
        kfree((*isp).isp_cell as *mut ::core::ffi::c_void);
        kfree((*isp).isp_i2c_res as *mut ::core::ffi::c_void);
        kfree((*isp).isp_gpio_res as *mut ::core::ffi::c_void);
        r
    };

    if (*adev).rmmio_size == 0 || (*adev).rmmio_size < 0x5289 {
        return -EINVAL;
    }

    isp_base = (*adev).rmmio_base;

    (*isp).isp_cell = kzalloc_objs::<mfd_cell>(3);
    if (*isp).isp_cell.is_null() {
        r = -ENOMEM;
        drm_err(&(*adev).ddev, "%s: isp mfd cell alloc failed\n", __func__);
        return cleanup(r);
    }

    num_res = MAX_ISP410_MEM_RES + MAX_ISP410_INT_SRC;
    (*isp).isp_res = kzalloc_objs::<resource>(num_res as usize);
    if (*isp).isp_res.is_null() {
        r = -ENOMEM;
        drm_err(&(*adev).ddev, "%s: isp mfd res alloc failed\n", __func__);
        return cleanup(r);
    }

    (*isp).isp_pdata = kzalloc_obj::<isp_platform_data>();
    if (*isp).isp_pdata.is_null() {
        r = -ENOMEM;
        drm_err(&(*adev).ddev, "%s: isp platform data alloc failed\n", __func__);
        return cleanup(r);
    }

    /* initialize isp platform data */
    (*(*isp).isp_pdata).adev = adev as *mut ::core::ffi::c_void;
    (*(*isp).isp_pdata).asic_type = (*adev).asic_type;
    (*(*isp).isp_pdata).base_rmmio_size = (*adev).rmmio_size;

    (*(*isp).isp_res.add(0)).name = "isp_4_1_0_reg";
    (*(*isp).isp_res.add(0)).flags = IORESOURCE_MEM;
    (*(*isp).isp_res.add(0)).start = isp_base;
    (*(*isp).isp_res.add(0)).end = isp_base + ISP_REGS_OFFSET_END;

    (*(*isp).isp_res.add(1)).name = "isp_4_1_phy0_reg";
    (*(*isp).isp_res.add(1)).flags = IORESOURCE_MEM;
    (*(*isp).isp_res.add(1)).start = isp_base + ISP410_PHY0_OFFSET;
    (*(*isp).isp_res.add(1)).end = isp_base + ISP410_PHY0_OFFSET + ISP410_PHY0_SIZE;

    idx = MAX_ISP410_MEM_RES;
    int_idx = 0;
    while idx < num_res {
        (*(*isp).isp_res.add(idx as usize)).name = "isp_4_1_0_irq";
        (*(*isp).isp_res.add(idx as usize)).flags = IORESOURCE_IRQ;
        (*(*isp).isp_res.add(idx as usize)).start =
            amdgpu_irq_create_mapping(adev, ISP_4_1_0_INT_SRCID[int_idx as usize]);
        (*(*isp).isp_res.add(idx as usize)).end = (*(*isp).isp_res.add(idx as usize)).start;
        idx += 1;
        int_idx += 1;
    }

    (*(*isp).isp_cell.add(0)).name = "amd_isp_capture";
    (*(*isp).isp_cell.add(0)).num_resources = num_res;
    (*(*isp).isp_cell.add(0)).resources = (*isp).isp_res;
    (*(*isp).isp_cell.add(0)).platform_data = (*isp).isp_pdata;
    (*(*isp).isp_cell.add(0)).pdata_size = ::core::mem::size_of::<isp_platform_data>();

    /* initialize isp i2c platform data */
    (*isp).isp_i2c_res = kzalloc_objs::<resource>(1);
    if (*isp).isp_i2c_res.is_null() {
        r = -ENOMEM;
        drm_err(&(*adev).ddev, "%s: isp mfd res alloc failed\n", __func__);
        return cleanup(r);
    }
    (*(*isp).isp_i2c_res).name = "isp_i2c0_reg";
    (*(*isp).isp_i2c_res).flags = IORESOURCE_MEM;
    (*(*isp).isp_i2c_res).start = isp_base + ISP410_I2C0_OFFSET;
    (*(*isp).isp_i2c_res).end = isp_base + ISP410_I2C0_OFFSET + ISP410_I2C0_SIZE;
    (*(*isp).isp_cell.add(1)).name = "amd_isp_i2c_designware";
    (*(*isp).isp_cell.add(1)).num_resources = 1;
    (*(*isp).isp_cell.add(1)).resources = (*isp).isp_i2c_res;
    (*(*isp).isp_cell.add(1)).platform_data = (*isp).isp_pdata;
    (*(*isp).isp_cell.add(1)).pdata_size = ::core::mem::size_of::<isp_platform_data>();

    /* initialize isp gpiochip platform data */
    (*isp).isp_gpio_res = kzalloc_objs::<resource>(1);
    if (*isp).isp_gpio_res.is_null() {
        r = -ENOMEM;
        drm_err(&(*adev).ddev, "%s: isp gpio res alloc failed\n", __func__);
        goto failure;
    }
    (*(*isp).isp_gpio_res).name = "isp_gpio_reg";
    (*(*isp).isp_gpio_res).flags = IORESOURCE_MEM;
    (*(*isp).isp_gpio_res).start = isp_base + ISP410_GPIO_SENSOR_OFFSET;
    (*(*isp).isp_gpio_res).end = isp_base + ISP410_GPIO_SENSOR_OFFSET + ISP410_GPIO_SENSOR_SIZE;
    (*(*isp).isp_cell.add(2)).name = "amdisp-pinctrl";
    (*(*isp).isp_cell.add(2)).num_resources = 1;
    (*(*isp).isp_cell.add(2)).resources = (*isp).isp_gpio_res;
    (*(*isp).isp_cell.add(2)).platform_data = (*isp).isp_pdata;
    (*(*isp).isp_cell.add(2)).pdata_size = ::core::mem::size_of::<isp_platform_data>();

    r = mfd_add_hotplug_devices((*isp).parent, (*isp).isp_cell, 3);
    if r != 0 {
        drm_err(&(*adev).ddev, "%s: add mfd hotplug device failed\n", __func__);
        goto failure;
    }
    return 0;

    cleanup(r)
}

unsafe fn isp_v4_1_0_hw_fini(isp: *mut amdgpu_isp) -> ::core::ffi::c_int {
    mfd_remove_devices((*isp).parent);
    kfree((*isp).isp_res as *mut ::core::ffi::c_void);
    kfree((*isp).isp_cell as *mut ::core::ffi::c_void);
    kfree((*isp).isp_pdata as *mut ::core::ffi::c_void);
    kfree((*isp).isp_i2c_res as *mut ::core::ffi::c_void);
    kfree((*isp).isp_gpio_res as *mut ::core::ffi::c_void);
    0
}

static ISP_V4_1_0_FUNCS: isp_funcs = isp_funcs {
    hw_init: Some(isp_v4_1_0_hw_init),
    hw_fini: Some(isp_v4_1_0_hw_fini),
};

unsafe fn isp_v4_1_0_set_isp_funcs(isp: *mut amdgpu_isp) {
    (*isp).funcs = &ISP_V4_1_0_FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
