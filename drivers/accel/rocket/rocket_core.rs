// SPDX-License-Identifier: GPL-2.0-only
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// Dependencies supplied by the surrounding kernel and driver sources:
// linux/clk.h, linux/delay.h, linux/dev_printk.h, linux/dma-mapping.h,
// linux/err.h, linux/iommu.h, linux/platform_device.h, linux/pm_runtime.h,
// linux/reset.h, rocket_core.h, and rocket_job.h.

extern "C" {
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn devm_reset_control_bulk_get_exclusive(
        dev: *mut device,
        num_rstcs: usize,
        rstcs: *mut reset_control_bulk_data,
    ) -> i32;
    fn devm_clk_bulk_get(dev: *mut device, num_clks: usize, clks: *mut clk_bulk_data) -> i32;
    fn devm_platform_ioremap_resource_byname(
        pdev: *mut platform_device,
        name: *const i8,
    ) -> *mut core::ffi::c_void;
    fn dma_set_max_seg_size(dev: *mut device, size: u32);
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn iommu_group_get(dev: *mut device) -> *mut iommu_group;
    fn iommu_group_put(group: *mut iommu_group);
    fn rocket_job_init(core: *mut rocket_core) -> i32;
    fn rocket_job_fini(core: *mut rocket_core);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: i32);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_resume_and_get(dev: *mut device) -> i32;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_dont_use_autosuspend(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn reset_control_bulk_assert(num_rstcs: usize, rstcs: *mut reset_control_bulk_data) -> i32;
    fn reset_control_bulk_deassert(num_rstcs: usize, rstcs: *mut reset_control_bulk_data) -> i32;
    fn udelay(usecs: u64);
    fn rocket_pc_readl(core: *mut rocket_core, reg: u32) -> u32;
}

// Types and constants are declared by the corresponding driver/kernel headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reset_control_bulk_data {
    pub id: *const i8,
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_bulk_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iommu_group {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rocket_core {
    pub dev: *mut device,
    pub resets: [reset_control_bulk_data; 2],
    pub clks: [clk_bulk_data; 0],
    pub pc_iomem: *mut core::ffi::c_void,
    pub cna_iomem: *mut core::ffi::c_void,
    pub core_iomem: *mut core::ffi::c_void,
    pub iommu_group: *mut iommu_group,
    pub index: i32,
}

const VERSION: u32 = 0;
const VERSION_NUM: u32 = 0;

pub unsafe fn rocket_core_init(core: *mut rocket_core) -> i32 {
    let dev = (*core).dev;
    let pdev = to_platform_device(dev);
    let mut version: u32;
    let mut err: i32 = 0;

    (*core).resets[0].id = b"srst_a\0".as_ptr() as *const i8;
    (*core).resets[1].id = b"srst_h\0".as_ptr() as *const i8;
    err = devm_reset_control_bulk_get_exclusive(dev, (*core).resets.len(), (*core).resets.as_mut_ptr());
    if err != 0 {
        return err;
    }

    err = devm_clk_bulk_get(dev, (*core).clks.len(), (*core).clks.as_mut_ptr());
    if err != 0 {
        return err;
    }

    (*core).pc_iomem = devm_platform_ioremap_resource_byname(pdev, b"pc\0".as_ptr() as *const i8);
    if (*core).pc_iomem as isize < 0 {
        return (*core).pc_iomem as isize as i32;
    }

    (*core).cna_iomem = devm_platform_ioremap_resource_byname(pdev, b"cna\0".as_ptr() as *const i8);
    if (*core).cna_iomem as isize < 0 {
        return (*core).cna_iomem as isize as i32;
    }

    (*core).core_iomem = devm_platform_ioremap_resource_byname(pdev, b"core\0".as_ptr() as *const i8);
    if (*core).core_iomem as isize < 0 {
        return (*core).core_iomem as isize as i32;
    }

    dma_set_max_seg_size(dev, u32::MAX);
    err = dma_set_mask_and_coherent(dev, (1u64 << 40).wrapping_sub(1));
    if err != 0 {
        return err;
    }

    (*core).iommu_group = iommu_group_get(dev);
    err = rocket_job_init(core);
    if err != 0 {
        iommu_group_put((*core).iommu_group);
        (*core).iommu_group = core::ptr::null_mut();
        return err;
    }

    pm_runtime_use_autosuspend(dev);
    /*
     * As this NPU will be most often used as part of a media pipeline that
     * ends presenting in a display, choose 50 ms (~3 frames at 60Hz) as an
     * autosuspend delay as that will keep the device powered up while the
     * pipeline is running.
     */
    pm_runtime_set_autosuspend_delay(dev, 50);
    pm_runtime_enable(dev);

    err = pm_runtime_resume_and_get(dev);
    if err != 0 {
        rocket_core_fini(core);
        return err;
    }

    version = rocket_pc_readl(core, VERSION);
    version = version.wrapping_add(rocket_pc_readl(core, VERSION_NUM) & 0xffff);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_put_autosuspend(dev);
    0
}

pub unsafe fn rocket_core_fini(core: *mut rocket_core) {
    pm_runtime_dont_use_autosuspend((*core).dev);
    pm_runtime_disable((*core).dev);
    iommu_group_put((*core).iommu_group);
    (*core).iommu_group = core::ptr::null_mut();
    rocket_job_fini(core);
}

pub unsafe fn rocket_core_reset(core: *mut rocket_core) {
    reset_control_bulk_assert((*core).resets.len(), (*core).resets.as_mut_ptr());
    udelay(10);
    reset_control_bulk_deassert((*core).resets.len(), (*core).resets.as_mut_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
