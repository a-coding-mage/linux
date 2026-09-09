// SPDX-License-Identifier: GPL-2.0
/*
 * ARM APMT table support.
 * Design document number: ARM DEN0117.
 *
 * Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES.
 *
 */

// #define pr_fmt(fmt) "ACPI: APMT: " fmt
// DEV_NAME "arm-cs-arch-pmu"
const DEV_MAX_RESOURCE_COUNT: usize = 3;

/* Root pointer to the mapped APMT table */
static mut apmt_table: *mut acpi_table_header = core::ptr::null_mut();

unsafe fn apmt_init_resources(
    res: *mut resource,
    node: *mut acpi_apmt_node,
) -> i32 {
    let mut irq: i32;
    let mut trigger: i32;
    let mut num_res: i32 = 0;

    (*res.add(num_res as usize)).start = (*node).base_address0;
    (*res.add(num_res as usize)).end = (*node).base_address0.wrapping_add(SZ_4K as u64 - 1);
    (*res.add(num_res as usize)).flags = IORESOURCE_MEM;

    num_res += 1;

    if ((*node).flags & ACPI_APMT_FLAGS_DUAL_PAGE) != 0 {
        (*res.add(num_res as usize)).start = (*node).base_address1;
        (*res.add(num_res as usize)).end = (*node).base_address1.wrapping_add(SZ_4K as u64 - 1);
        (*res.add(num_res as usize)).flags = IORESOURCE_MEM;

        num_res += 1;
    }

    if (*node).ovflw_irq != 0 {
        trigger = (*node).ovflw_irq_flags & ACPI_APMT_OVFLW_IRQ_FLAGS_MODE;
        trigger = if trigger == ACPI_APMT_OVFLW_IRQ_FLAGS_MODE_LEVEL {
            ACPI_LEVEL_SENSITIVE
        } else {
            ACPI_EDGE_SENSITIVE
        };
        irq = acpi_register_gsi(core::ptr::null_mut(), (*node).ovflw_irq, trigger,
                                ACPI_ACTIVE_HIGH);

        if irq <= 0 {
            pr_warn!("APMT could not register gsi hwirq {}\n", irq);
            return num_res;
        }

        (*res.add(num_res as usize)).start = irq as u64;
        (*res.add(num_res as usize)).end = irq as u64;
        (*res.add(num_res as usize)).flags = IORESOURCE_IRQ;

        num_res += 1;
    }

    num_res
}

/**
 * apmt_add_platform_device() - Allocate a platform device for APMT node
 * @node: Pointer to device ACPI APMT node
 * @fwnode: fwnode associated with the APMT node
 *
 * Returns: 0 on success, <0 failure
 */
unsafe fn apmt_add_platform_device(
    node: *mut acpi_apmt_node,
    fwnode: *mut fwnode_handle,
) -> i32 {
    let pdev: *mut platform_device;
    let mut ret: i32;
    let count: i32;
    let uid: i32 = ((*node).id & INT_MAX as u32) as i32;
    let mut res: [resource; DEV_MAX_RESOURCE_COUNT] = core::mem::zeroed();

    if uid as u32 != (*node).id {
        pr_warn!("Unexpectedly large UID 0x{:x}, truncated to 0x{:x}\n", (*node).id, uid);
    }
    pdev = platform_device_alloc(b"arm-cs-arch-pmu\0".as_ptr() as *const i8, uid);
    if pdev.is_null() {
        return -ENOMEM;
    }

    count = apmt_init_resources(res.as_mut_ptr(), node);

    ret = platform_device_add_resources(pdev, res.as_ptr(), count);
    if ret != 0 {
        platform_device_put(pdev);
        return ret;
    }

    /*
     * Add a copy of APMT node pointer to platform_data to be used to
     * retrieve APMT data information.
     */
    ret = platform_device_add_data(pdev, &node as *const _ as *const core::ffi::c_void,
                                   core::mem::size_of::<*mut acpi_apmt_node>());
    if ret != 0 {
        platform_device_put(pdev);
        return ret;
    }

    platform_device_set_fwnode(pdev, fwnode);

    ret = platform_device_add(pdev);
    if ret != 0 {
        platform_device_put(pdev);
        return ret;
    }

    0
}

unsafe fn apmt_init_platform_devices() -> i32 {
    let mut apmt_node: *mut acpi_apmt_node;
    let apmt: *mut acpi_table_apmt;
    let mut fwnode: *mut fwnode_handle;
    let mut offset: u64;
    let end: u64;
    let mut ret: i32;

    /*
     * apmt_table and apmt both point to the start of APMT table, but
     * have different struct types
     */
    apmt = apmt_table as *mut acpi_table_apmt;
    offset = core::mem::size_of::<acpi_table_apmt>() as u64;
    end = (*apmt).header.length as u64;

    while offset < end {
        apmt_node = (apmt as *mut u8).add(offset as usize) as *mut acpi_apmt_node;

        fwnode = acpi_alloc_fwnode_static();
        if fwnode.is_null() {
            return -ENOMEM;
        }

        ret = apmt_add_platform_device(apmt_node, fwnode);
        if ret != 0 {
            acpi_free_fwnode_static(fwnode);
            return ret;
        }

        offset += (*apmt_node).length as u64;
    }

    0
}

unsafe fn acpi_apmt_init() {
    let mut status: acpi_status;
    let mut ret: i32;

    /**
     * APMT table nodes will be used at runtime after the apmt init,
     * so we don't need to call acpi_put_table() to release
     * the APMT table mapping.
     */
    status = acpi_get_table(ACPI_SIG_APMT, 0, &mut apmt_table);

    if ACPI_FAILURE(status) {
        if status != AE_NOT_FOUND {
            let msg: *const i8 = acpi_format_exception(status);
            pr_err!("Failed to get APMT table, {}\n", msg);
        }

        return;
    }

    ret = apmt_init_platform_devices();
    if ret != 0 {
        pr_err!("Failed to initialize APMT platform devices, ret: {}\n", ret);
        acpi_put_table(apmt_table);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
