// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI helpers for DMA request / controller
 *
 * Based on of-dma.c
 *
 * Copyright (C) 2013, Intel Corporation
 * Authors: Andy Shevchenko <andriy.she<PRIVATE_PERSON>.shevchenko@linux.intel.com>
 *          Mika Westerberg <mika.westerberg@linux.intel.com>
 */

// The declarations supplied by the Linux kernel headers are external dependencies.

static mut acpi_dma_list: list_head = LIST_HEAD_INIT(acpi_dma_list);
static mut acpi_dma_lock: mutex = DEFINE_MUTEX_INIT();

/**
 * acpi_dma_parse_resource_group - match device and parse resource group
 * @grp: CSRT resource group
 * @adev: ACPI device to match with
 * @adma: struct acpi_dma of the given DMA controller
 */
unsafe fn acpi_dma_parse_resource_group(
    grp: *const acpi_csrt_group,
    adev: *mut acpi_device,
    adma: *mut acpi_dma,
) -> i32 {
    let si: *const acpi_csrt_shared_info;
    let mut resource_list: list_head = list_head::default();
    let mut rentry: *mut resource_entry;
    let mut mem: resource_size_t = 0;
    let mut irq: resource_size_t = 0;
    let mut ret: i32;

    if (*grp).shared_info_length != core::mem::size_of::<acpi_csrt_shared_info>() {
        return -ENODEV;
    }

    INIT_LIST_HEAD(&mut resource_list);
    ret = acpi_dev_get_resources(adev, &mut resource_list, None, None);
    if ret <= 0 {
        return 0;
    }

    list_for_each_entry!(rentry, &mut resource_list, node, {
        if resource_type((*rentry).res) == IORESOURCE_MEM {
            mem = (*rentry).res.start;
        } else if resource_type((*rentry).res) == IORESOURCE_IRQ {
            irq = (*rentry).res.start;
        }
    });

    acpi_dev_free_resource_list(&mut resource_list);

    // Consider initial zero values as resource not found
    if mem == 0 && irq == 0 {
        return 0;
    }

    si = (grp.add(1)) as *const acpi_csrt_shared_info;

    // Match device by MMIO
    if (*si).mmio_base_low != lower_32_bits(mem)
        || (*si).mmio_base_high != upper_32_bits(mem)
    {
        return 0;
    }

    /*
     * acpi_gsi_to_irq() can't be used because some platforms do not save
     * registered IRQs in the MP table. Instead we just try to register
     * the GSI, which is the core part of the above mentioned function.
     */
    ret = acpi_register_gsi(
        core::ptr::null_mut(),
        (*si).gsi_interrupt,
        (*si).interrupt_mode,
        (*si).interrupt_polarity,
    );
    if ret < 0 {
        return 0;
    }

    // Match device by Linux vIRQ
    if ret as resource_size_t != irq {
        return 0;
    }

    dev_dbg!(&(*adev).dev, "matches with %.4s%04X (rev %u)\n", &(*grp).vendor_id, (*grp).device_id, (*grp).revision);

    // Check if the request line range is available
    if (*si).base_request_line == 0 && (*si).num_handshake_signals == 0 {
        return 0;
    }

    // Set up DMA mask based on value from CSRT
    ret = dma_coerce_mask_and_coherent(
        &mut (*adev).dev,
        DMA_BIT_MASK((*si).dma_address_width),
    );
    if ret != 0 {
        return 0;
    }

    (*adma).base_request_line = (*si).base_request_line;
    (*adma).end_request_line = (*si).base_request_line
        + (*si).num_handshake_signals
        - 1;

    dev_dbg!(&(*adev).dev, "request line base: 0x%04x end: 0x%04x\n", (*adma).base_request_line, (*adma).end_request_line);

    1
}

/** Parse CSRT to extract additional DMA resources. */
unsafe fn acpi_dma_parse_csrt(adev: *mut acpi_device, adma: *mut acpi_dma) {
    let mut grp: *mut acpi_csrt_group;
    let end: *mut acpi_csrt_group;
    let mut csrt: *mut acpi_table_csrt = core::ptr::null_mut();
    let mut status: acpi_status;
    let mut ret: i32;

    status = acpi_get_table(ACPI_SIG_CSRT, 0, &mut csrt as *mut _ as *mut *mut acpi_table_header);
    if ACPI_FAILURE(status) {
        if status != AE_NOT_FOUND {
            dev_warn!(&(*adev).dev, "failed to get the CSRT table\n");
        }
        return;
    }

    grp = (csrt.add(1)) as *mut acpi_csrt_group;
    end = ((csrt as *mut u8).add((*csrt).header.length as usize)) as *mut acpi_csrt_group;

    while grp < end {
        ret = acpi_dma_parse_resource_group(grp, adev, adma);
        if ret < 0 {
            dev_warn!(&(*adev).dev, "error in parsing resource group\n");
            break;
        }
        grp = ((grp as *mut u8).add((*grp).length as usize)) as *mut acpi_csrt_group;
    }

    acpi_put_table(csrt as *mut acpi_table_header);
}

pub unsafe fn acpi_dma_controller_register(
    dev: *mut device,
    acpi_dma_xlate: Option<unsafe extern "C" fn(*mut acpi_dma_spec, *mut acpi_dma) -> *mut dma_chan>,
    data: *mut core::ffi::c_void,
) -> i32 {
    let adev: *mut acpi_device;
    let adma: *mut acpi_dma;

    if dev.is_null() || acpi_dma_xlate.is_none() {
        return -EINVAL;
    }

    adev = ACPI_COMPANION(dev);
    if adev.is_null() {
        return -EINVAL;
    }

    adma = kzalloc_obj::<acpi_dma>();
    if adma.is_null() {
        return -ENOMEM;
    }

    (*adma).dev = dev;
    (*adma).acpi_dma_xlate = acpi_dma_xlate;
    (*adma).data = data;

    acpi_dma_parse_csrt(adev, adma);

    mutex_lock(&mut acpi_dma_lock);
    list_add_tail(&mut (*adma).dma_controllers, &mut acpi_dma_list);
    mutex_unlock(&mut acpi_dma_lock);

    0
}

pub unsafe fn acpi_dma_controller_free(dev: *mut device) -> i32 {
    let mut adma: *mut acpi_dma;

    if dev.is_null() {
        return -EINVAL;
    }

    mutex_lock(&mut acpi_dma_lock);

    list_for_each_entry!(adma, &mut acpi_dma_list, dma_controllers, {
        if (*adma).dev == dev {
            list_del(&mut (*adma).dma_controllers);
            mutex_unlock(&mut acpi_dma_lock);
            kfree(adma as *mut core::ffi::c_void);
            return 0;
        }
    });

    mutex_unlock(&mut acpi_dma_lock);
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
