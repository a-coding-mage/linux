/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2024 Advanced Micro Devices, Inc. All rights reserved.
 * All Rights Reserved.
 */

// Translated from isp_v4_1_1.c. Kernel and AMDGPU declarations are supplied
// by the surrounding Rust translation environment.

const ISP_PERFORMANCE_STATE_LOW: u32 = 0;
const ISP_PERFORMANCE_STATE_HIGH: u32 = 1;
const ISP_HIGH_PERFORMANC_XCLK: u32 = 788;
const ISP_HIGH_PERFORMANC_ICLK: u32 = 788;

static ISP_4_1_1_INT_SRCID: [u32; MAX_ISP411_INT_SRC as usize] = [
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT9,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT10,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT11,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT12,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT13,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT14,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT15,
    ISP_4_1__SRCID__ISP_RINGBUFFER_WPT16,
];

static mut ISP_GPIO_TABLE: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "amd_isp_capture\0".as_ptr() as *const i8,
    table: [GPIO_LOOKUP("AMDI0030:00", 85, "enable_isp", GPIO_ACTIVE_HIGH), GPIO_LOOKUP_END],
};

static mut ISP_SENSOR_GPIO_TABLE: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "i2c-ov05c10\0".as_ptr() as *const i8,
    table: [GPIO_LOOKUP("amdisp-pinctrl", 0, "enable", GPIO_ACTIVE_HIGH), GPIO_LOOKUP_END],
};

unsafe fn isp_poweroff(genpd: *mut generic_pm_domain) -> i32 {
    let isp = container_of!(genpd, amdgpu_isp, ispgpd);
    let adev = (*isp).adev;
    amdgpu_dpm_set_powergating_by_smu(adev, AMD_IP_BLOCK_TYPE_ISP, true, 0)
}

unsafe fn isp_poweron(genpd: *mut generic_pm_domain) -> i32 {
    let isp = container_of!(genpd, amdgpu_isp, ispgpd);
    let adev = (*isp).adev;
    amdgpu_dpm_set_powergating_by_smu(adev, AMD_IP_BLOCK_TYPE_ISP, false, 0)
}

unsafe fn isp_set_performance_state(genpd: *mut generic_pm_domain, state: u32) -> i32 {
    let isp = container_of!(genpd, amdgpu_isp, ispgpd);
    let adev = (*isp).adev;
    let (iclk, xclk): (u32, u32);
    match state {
        ISP_PERFORMANCE_STATE_HIGH => { xclk = ISP_HIGH_PERFORMANC_XCLK; iclk = ISP_HIGH_PERFORMANC_ICLK; }
        ISP_PERFORMANCE_STATE_LOW => { /* isp runs at default lowest clock-rate on power-on, do nothing */ return 0; }
        _ => return -EINVAL,
    }
    let mut ret = amdgpu_dpm_set_soft_freq_range(adev, PP_ISPXCLK, xclk, 0);
    if ret != 0 { drm_err!(&(*adev).ddev, "failed to set xclk %u to %u: %d\n", xclk, state, ret); return ret; }
    ret = amdgpu_dpm_set_soft_freq_range(adev, PP_ISPICLK, iclk, 0);
    if ret != 0 { drm_err!(&(*adev).ddev, "failed to set iclk %u to %u: %d\n", iclk, state, ret); return ret; }
    0
}

unsafe fn isp_genpd_add_device(dev: *mut device, data: *mut core::ffi::c_void) -> i32 {
    let gpd = data as *mut generic_pm_domain;
    let pdev = container_of!(dev, platform_device, dev);
    let isp = container_of!(gpd, amdgpu_isp, ispgpd);
    let adev = (*isp).adev;
    if pdev.is_null() { return -EINVAL; }
    if (*dev).type_.is_null() || (*(*dev).type_).name.is_null() { drm_dbg!(&(*adev).ddev, "Invalid device type to add\n"); return 0; }
    if strcmp((*(*dev).type_).name, b"mfd_device\0".as_ptr() as *const i8) != 0 { drm_dbg!(&(*adev).ddev, "Invalid isp mfd device %s to add\n", (*(*pdev).mfd_cell).name); return 0; }
    let ret = pm_genpd_add_device(gpd, dev);
    if ret != 0 { drm_err!(&(*adev).ddev, "Failed to add dev %s to genpd %d\n", (*(*pdev).mfd_cell).name, ret); return -ENODEV; }
    dev_pm_syscore_device(dev, true); 0
}

unsafe fn isp_genpd_remove_device(dev: *mut device, data: *mut core::ffi::c_void) -> i32 {
    let gpd = data as *mut generic_pm_domain;
    let pdev = container_of!(dev, platform_device, dev);
    let isp = container_of!(gpd, amdgpu_isp, ispgpd);
    let adev = (*isp).adev;
    if pdev.is_null() { return -EINVAL; }
    if (*dev).type_.is_null() || (*(*dev).type_).name.is_null() { drm_dbg!(&(*adev).ddev, "Invalid device type to remove\n"); return 0; }
    if strcmp((*(*dev).type_).name, b"mfd_device\0".as_ptr() as *const i8) != 0 { drm_dbg!(&(*adev).ddev, "Invalid isp mfd device %s to remove\n", (*(*pdev).mfd_cell).name); return 0; }
    let ret = pm_genpd_remove_device(dev);
    if ret != 0 { drm_err!(&(*adev).ddev, "Failed to remove dev from genpd %d\n", ret); return -ENODEV; }
    dev_pm_syscore_device(dev, false); 0
}

unsafe fn isp_suspend_device(dev: *mut device, _data: *mut core::ffi::c_void) -> i32 { pm_runtime_force_suspend(dev) }
unsafe fn isp_resume_device(dev: *mut device, _data: *mut core::ffi::c_void) -> i32 { pm_runtime_force_resume(dev) }

unsafe fn isp_v4_1_1_hw_suspend(isp: *mut amdgpu_isp) -> i32 {
    let r = device_for_each_child((*isp).parent, core::ptr::null_mut(), isp_suspend_device);
    if r != 0 { dev_err!((*isp).parent, "failed to suspend hw devices (%d)\n", r); } r
}

unsafe fn isp_v4_1_1_hw_resume(isp: *mut amdgpu_isp) -> i32 {
    let r = device_for_each_child((*isp).parent, core::ptr::null_mut(), isp_resume_device);
    if r != 0 { dev_err!((*isp).parent, "failed to resume hw device (%d)\n", r); } r
}

unsafe fn isp_v4_1_1_hw_init(isp: *mut amdgpu_isp) -> i32 {
    let adev = (*isp).adev;
    if (*adev).rmmio_size == 0 || (*adev).rmmio_size < 0x5289 { return -EINVAL; }
    let mut acpi_dev: *mut acpi_device = core::ptr::null_mut();
    let mut r = amdgpu_acpi_get_isp4_dev(&mut acpi_dev);
    if r != 0 { drm_dbg!(&(*adev).ddev, "Invalid isp platform detected (%d)", r); return 0; }
    if strcmp(acpi_device_hid(acpi_dev), b"OMNI5C10\0".as_ptr() as *const i8) == 0 { gpiod_add_lookup_table(&mut ISP_GPIO_TABLE); gpiod_add_lookup_table(&mut ISP_SENSOR_GPIO_TABLE); }
    let isp_base = (*adev).rmmio_base;
    (*isp).ispgpd.name = b"ISP_v_4_1_1\0".as_ptr() as *const i8;
    (*isp).ispgpd.power_off = Some(isp_poweroff); (*isp).ispgpd.power_on = Some(isp_poweron); (*isp).ispgpd.set_performance_state = Some(isp_set_performance_state);
    r = pm_genpd_init(&mut (*isp).ispgpd, core::ptr::null_mut(), true); if r != 0 { drm_err!(&(*adev).ddev, "failed to initialize genpd (%d)\n", r); return -EINVAL; }
    (*isp).isp_cell = kzalloc_objs::<mfd_cell>(3); if (*isp).isp_cell.is_null() { r = -ENOMEM; drm_err!(&(*adev).ddev, "isp mfd cell alloc failed (%d)\n", r); return r; }
    let num_res = MAX_ISP411_MEM_RES + MAX_ISP411_INT_SRC;
    (*isp).isp_res = kzalloc_objs::<resource>(num_res as usize); if (*isp).isp_res.is_null() { r = -ENOMEM; drm_err!(&(*adev).ddev, "isp mfd resource alloc failed (%d)\n", r); return r; }
    (*isp).isp_pdata = kzalloc_obj::<isp_platform_data>(); if (*isp).isp_pdata.is_null() { r = -ENOMEM; drm_err!(&(*adev).ddev, "isp platform data alloc failed (%d)\n", r); return r; }
    (*(*isp).isp_pdata).adev = adev as *mut core::ffi::c_void; (*(*isp).isp_pdata).asic_type = (*adev).asic_type; (*(*isp).isp_pdata).base_rmmio_size = (*adev).rmmio_size;
    (*(*isp).isp_res.add(0)).name = b"isp_4_1_1_reg\0".as_ptr() as *const i8; (*(*isp).isp_res.add(0)).flags = IORESOURCE_MEM; (*(*isp).isp_res.add(0)).start = isp_base; (*(*isp).isp_res.add(0)).end = isp_base + ISP_REGS_OFFSET_END;
    (*(*isp).isp_res.add(1)).name = b"isp_4_1_1_phy0_reg\0".as_ptr() as *const i8; (*(*isp).isp_res.add(1)).flags = IORESOURCE_MEM; (*(*isp).isp_res.add(1)).start = isp_base + ISP411_PHY0_OFFSET; (*(*isp).isp_res.add(1)).end = isp_base + ISP411_PHY0_OFFSET + ISP411_PHY0_SIZE;
    for (idx, int_idx) in (MAX_ISP411_MEM_RES..num_res).zip(0..) { let res = &mut *(*isp).isp_res.add(idx as usize); res.name = b"isp_4_1_1_irq\0".as_ptr() as *const i8; res.flags = IORESOURCE_IRQ; res.start = amdgpu_irq_create_mapping(adev, ISP_4_1_1_INT_SRCID[int_idx as usize]); res.end = res.start; }
    let amd_camera_node = (*acpi_dev).driver_data as *const software_node;
    let isp4_node = software_node_find_by_name(amd_camera_node, b"isp4\0".as_ptr() as *const i8);
    let cell = (*isp).isp_cell;
    (*cell.add(0)).name = b"amd_isp_capture\0".as_ptr() as *const i8;
    (*cell.add(0)).num_resources = num_res; (*cell.add(0)).resources = (*isp).isp_res;
    (*cell.add(0)).platform_data = (*isp).isp_pdata as *mut core::ffi::c_void; (*cell.add(0)).swnode = isp4_node; (*cell.add(0)).pdata_size = core::mem::size_of::<isp_platform_data>();
    (*isp).isp_i2c_res = kzalloc_objs::<resource>(1); (*isp).isp_gpio_res = kzalloc_objs::<resource>(1);
    if (*isp).isp_i2c_res.is_null() || (*isp).isp_gpio_res.is_null() { return -ENOMEM; }
    (*(*isp).isp_i2c_res).name = b"isp_i2c0_reg\0".as_ptr() as *const i8; (*(*isp).isp_i2c_res).flags = IORESOURCE_MEM; (*(*isp).isp_i2c_res).start = isp_base + ISP411_I2C0_OFFSET; (*(*isp).isp_i2c_res).end = isp_base + ISP411_I2C0_OFFSET + ISP411_I2C0_SIZE;
    (*cell.add(1)).name = b"amd_isp_i2c_designware\0".as_ptr() as *const i8; (*cell.add(1)).num_resources = 1; (*cell.add(1)).resources = (*isp).isp_i2c_res; (*cell.add(1)).platform_data = (*isp).isp_pdata as *mut core::ffi::c_void; (*cell.add(1)).pdata_size = core::mem::size_of::<isp_platform_data>();
    (*(*isp).isp_gpio_res).name = b"isp_gpio_reg\0".as_ptr() as *const i8; (*(*isp).isp_gpio_res).flags = IORESOURCE_MEM; (*(*isp).isp_gpio_res).start = isp_base + ISP411_GPIO_SENSOR_OFFSET; (*(*isp).isp_gpio_res).end = isp_base + ISP411_GPIO_SENSOR_OFFSET + ISP411_GPIO_SENSOR_SIZE;
    (*cell.add(2)).name = b"amdisp-pinctrl\0".as_ptr() as *const i8; (*cell.add(2)).num_resources = 1; (*cell.add(2)).resources = (*isp).isp_gpio_res; (*cell.add(2)).platform_data = (*isp).isp_pdata as *mut core::ffi::c_void; (*cell.add(2)).pdata_size = core::mem::size_of::<isp_platform_data>();
    r = mfd_add_hotplug_devices((*isp).parent, (*isp).isp_cell, 2); if r != 0 { return r; }
    r = device_for_each_child((*isp).parent, &mut (*isp).ispgpd as *mut _ as *mut _, isp_genpd_add_device); if r != 0 { return r; }
    r = mfd_add_hotplug_devices((*isp).parent, (*isp).isp_cell.add(2), 1); if r != 0 { return r; } 0
}

unsafe fn isp_v4_1_1_hw_fini(isp: *mut amdgpu_isp) -> i32 { device_for_each_child((*isp).parent, core::ptr::null_mut(), isp_genpd_remove_device); mfd_remove_devices((*isp).parent); kfree((*isp).isp_res as *mut _); kfree((*isp).isp_cell as *mut _); kfree((*isp).isp_pdata as *mut _); kfree((*isp).isp_i2c_res as *mut _); kfree((*isp).isp_gpio_res as *mut _); 0 }

static ISP_V4_1_1_FUNCS: isp_funcs = isp_funcs { hw_init: Some(isp_v4_1_1_hw_init), hw_fini: Some(isp_v4_1_1_hw_fini), hw_suspend: Some(isp_v4_1_1_hw_suspend), hw_resume: Some(isp_v4_1_1_hw_resume) };

pub unsafe fn isp_v4_1_1_set_isp_funcs(isp: *mut amdgpu_isp) { (*isp).funcs = &ISP_V4_1_1_FUNCS; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
