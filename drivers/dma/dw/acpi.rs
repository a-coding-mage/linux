// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2013,2019 Intel Corporation

// Dependencies supplied by the surrounding kernel translation unit:
// linux/acpi.h, linux/acpi_dma.h, and internal.h.

unsafe fn dw_dma_acpi_filter(
    chan: *mut dma_chan,
    param: *mut core::ffi::c_void,
) -> bool {
    let dw: *mut dw_dma = to_dw_dma((*chan).device);
    let data: *mut dw_dma_chip_pdata = dev_get_drvdata((*dw).dma.dev);
    let dma_spec: *mut acpi_dma_spec = param as *mut acpi_dma_spec;
    let slave = dw_dma_slave {
        dma_dev: (*dma_spec).dev,
        src_id: (*dma_spec).slave_id,
        dst_id: (*dma_spec).slave_id,
        m_master: (*data).m_master,
        p_master: (*data).p_master,
    };

    dw_dma_filter(chan, &slave)
}

pub unsafe fn dw_dma_acpi_controller_register(dw: *mut dw_dma) {
    let dev: *mut device = (*dw).dma.dev;
    let mut info: *mut acpi_dma_filter_info;
    let ret: i32;

    if !has_acpi_companion(dev) {
        return;
    }

    info = devm_kzalloc(dev, core::mem::size_of::<acpi_dma_filter_info>(), GFP_KERNEL)
        as *mut acpi_dma_filter_info;
    if info.is_null() {
        return;
    }

    dma_cap_zero((*info).dma_cap);
    dma_cap_set(DMA_SLAVE, (*info).dma_cap);
    (*info).filter_fn = Some(dw_dma_acpi_filter);

    ret = acpi_dma_controller_register(dev, acpi_dma_simple_xlate, info);
    if ret != 0 {
        dev_err(dev, "could not register acpi_dma_controller\n");
    }
}

pub unsafe fn dw_dma_acpi_controller_free(dw: *mut dw_dma) {
    let dev: *mut device = (*dw).dma.dev;

    if !has_acpi_companion(dev) {
        return;
    }

    acpi_dma_controller_free(dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
