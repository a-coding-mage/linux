// SPDX-License-Identifier: GPL-2.0
/*
 * Platform driver for the Synopsys DesignWare DMA Controller
 *
 * Copyright (C) 2007-2008 Atmel Corporation
 * Copyright (C) 2010-2011 ST Microelectronics
 * Copyright (C) 2013 Intel Corporation
 *
 * Some parts of this driver are derived from the original dw_dmac.
 */

// Linux kernel headers and "internal.h" provide the types, constants, and
// external functions referenced by this translation.

#[cfg(CONFIG_OF)]
static DW_DMA_OF_ID_TABLE: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"snps,dma-spear1340\\0".as_ptr(), data: &DW_DMA_CHIP_PDATA as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: b"renesas,rzn1-dma\\0".as_ptr(), data: &DW_DMA_CHIP_PDATA as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[cfg(CONFIG_ACPI)]
static DW_DMA_ACPI_ID_TABLE: [AcpiDeviceId; 7] = [
    AcpiDeviceId { id: *b"INTL9C60\\0", driver_data: &DW_DMA_CHIP_PDATA as *const _ as usize },
    AcpiDeviceId { id: *b"80862286\\0", driver_data: &DW_DMA_CHIP_PDATA as *const _ as usize },
    AcpiDeviceId { id: *b"808622C0\\0", driver_data: &DW_DMA_CHIP_PDATA as *const _ as usize },
    // Elkhart Lake iDMA 32-bit (PSE DMA)
    AcpiDeviceId { id: *b"80864BB4\\0", driver_data: &XBAR_CHIP_PDATA as *const _ as usize },
    AcpiDeviceId { id: *b"80864BB5\\0", driver_data: &XBAR_CHIP_PDATA as *const _ as usize },
    AcpiDeviceId { id: *b"80864BB6\\0", driver_data: &XBAR_CHIP_PDATA as *const _ as usize },
    AcpiDeviceId { id: [0; 9], driver_data: 0 },
];

unsafe fn dw_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let match_data = device_get_match_data(dev);
    if match_data.is_null() { return -ENODEV; }

    let data = devm_kmemdup(dev, match_data, core::mem::size_of::<DwDmaChipPdata>(), GFP_KERNEL);
    if data.is_null() { return -ENOMEM; }

    let chip = devm_kzalloc(dev, core::mem::size_of::<DwDmaChip>(), GFP_KERNEL) as *mut DwDmaChip;
    if chip.is_null() { return -ENOMEM; }

    (*chip).irq = platform_get_irq(pdev, 0);
    if (*chip).irq < 0 { return (*chip).irq; }

    (*chip).regs = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*chip).regs) { return ptr_err((*chip).regs); }

    let mut ret = dma_coerce_mask_and_coherent(dev, dma_bit_mask(32));
    if ret != 0 { return ret; }

    if (*data).pdata.is_null() { (*data).pdata = dev_get_platdata(dev); }
    if (*data).pdata.is_null() { (*data).pdata = dw_dma_parse_dt(pdev); }

    (*chip).dev = dev;
    (*chip).id = (*pdev).id;
    (*chip).pdata = (*data).pdata;
    (*data).chip = chip;

    (*chip).clk = devm_clk_get_optional((*chip).dev, b"hclk\\0".as_ptr() as *const i8);
    if is_err((*chip).clk) { return ptr_err((*chip).clk); }
    ret = clk_prepare_enable((*chip).clk);
    if ret != 0 { return ret; }

    pm_runtime_enable(dev);
    ret = ((*data).probe)(chip);
    if ret != 0 {
        pm_runtime_disable(dev);
        clk_disable_unprepare((*chip).clk);
        return ret;
    }
    platform_set_drvdata(pdev, data as *mut core::ffi::c_void);
    dw_dma_of_controller_register((*chip).dw);
    dw_dma_acpi_controller_register((*chip).dw);
    0
}

unsafe fn dw_remove(pdev: *mut PlatformDevice) {
    let data = platform_get_drvdata(pdev) as *mut DwDmaChipPdata;
    let chip = (*data).chip;
    dw_dma_acpi_controller_free((*chip).dw);
    dw_dma_of_controller_free((*chip).dw);
    let ret = ((*data).remove)(chip);
    if ret != 0 { dev_warn((*chip).dev, b"can't remove device properly: %d\\n\\0".as_ptr() as *const i8, ret); }
    pm_runtime_disable(&mut (*pdev).dev);
    clk_disable_unprepare((*chip).clk);
}

unsafe fn dw_shutdown(pdev: *mut PlatformDevice) {
    let data = platform_get_drvdata(pdev) as *mut DwDmaChipPdata;
    let chip = (*data).chip;
    pm_runtime_get_sync((*chip).dev);
    do_dw_dma_disable(chip);
    pm_runtime_put_sync_suspend((*chip).dev);
    clk_disable_unprepare((*chip).clk);
}

unsafe fn dw_suspend_late(dev: *mut Device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut DwDmaChipPdata;
    let chip = (*data).chip;
    do_dw_dma_disable(chip);
    clk_disable_unprepare((*chip).clk);
    0
}

unsafe fn dw_resume_early(dev: *mut Device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut DwDmaChipPdata;
    let chip = (*data).chip;
    let ret = clk_prepare_enable((*chip).clk);
    if ret != 0 { return ret; }
    do_dw_dma_enable(chip)
}

static mut DW_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(dw_probe), remove: Some(dw_remove), shutdown: Some(dw_shutdown),
    driver: DeviceDriver {
        name: b"dw_dmac\\0".as_ptr() as *const i8,
        pm: Some(DevicePmOps { suspend_late: Some(dw_suspend_late), resume_early: Some(dw_resume_early) }),
        #[cfg(CONFIG_OF)] of_match_table: Some(&DW_DMA_OF_ID_TABLE),
        #[cfg(CONFIG_ACPI)] acpi_match_table: Some(&DW_DMA_ACPI_ID_TABLE),
    },
};

unsafe fn dw_init() -> i32 { platform_driver_register(&mut DW_DRIVER) }
unsafe fn dw_exit() { platform_driver_unregister(&mut DW_DRIVER); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
