// SPDX-License-Identifier: GPL-2.0-only

/*
 * ACPI support for platform bus type.
 *
 * Copyright (C) 2015, Linaro Ltd
 * Author: Graeme Gregory <graeme.gregory@linaro.org>
 */

// Dependencies supplied by the Linux ACPI, AMBA, clock, device, resource,
// kernel, module, and local init interfaces are referenced below.

static AMBA_ID_LIST: [acpi_device_id; 3] = [
    acpi_device_id { id: *b"ARMH0061\0", driver_data: 0 }, // PL061 GPIO Device
    acpi_device_id { id: *b"ARMH0330\0", driver_data: 0 }, // ARM DMA Controller DMA-330
    acpi_device_id { id: *b"\0\0\0\0\0\0\0\0", driver_data: 0 },
];

unsafe fn amba_register_dummy_clk() {
    let mut amba_dummy_clk: *mut clk;

    amba_dummy_clk = clk_register_fixed_rate(core::ptr::null_mut(), c"apb_pclk".as_ptr(), core::ptr::null(), 0, 0);
    clk_register_clkdev(amba_dummy_clk, c"apb_pclk".as_ptr(), core::ptr::null());
}

unsafe fn amba_handler_attach(
    adev: *mut acpi_device,
    _id: *const acpi_device_id,
) -> i32 {
    let parent: *mut acpi_device = acpi_dev_parent(adev);
    let mut dev: *mut amba_device;
    let mut rentry: *mut resource_entry;
    let mut resource_list: list_head = core::mem::zeroed();
    let mut address_found = false;
    let mut irq_no = 0;
    let mut ret: i32;

    /* If the ACPI node already has a physical device attached, skip it. */
    if (*adev).physical_node_count != 0 {
        return 0;
    }

    dev = amba_device_alloc(dev_name(&mut (*adev).dev), 0, 0);
    if dev.is_null() {
        dev_err(&mut (*adev).dev, c"%s(): amba_device_alloc() failed\n".as_ptr(), c"amba_handler_attach".as_ptr());
        return -12;
    }

    INIT_LIST_HEAD(&mut resource_list);
    ret = acpi_dev_get_resources(adev, &mut resource_list, None, None);
    if ret < 0 {
        amba_device_put(dev);
        return ret;
    }

    // list_for_each_entry(rentry, &resource_list, node)
    let mut pos = (*resource_list.next).next;
    while pos != &mut resource_list as *mut list_head {
        rentry = container_of!(pos, resource_entry, node);
        match resource_type((*rentry).res) {
            IORESOURCE_MEM => {
                if !address_found {
                    (*dev).res = *(*rentry).res;
                    (*dev).res.name = dev_name(&mut (*dev).dev);
                    address_found = true;
                }
            }
            IORESOURCE_IRQ => {
                if irq_no < AMBA_NR_IRQS {
                    (*dev).irq[irq_no as usize] = (*(*rentry).res).start;
                    irq_no += 1;
                }
            }
            _ => {
                dev_warn(&mut (*adev).dev, c"Invalid resource\n".as_ptr());
            }
        }
        pos = (*pos).next;
    }

    acpi_dev_free_resource_list(&mut resource_list);

    /*
     * If the ACPI node has a parent and that parent has a physical device
     * attached to it, that physical device should be the parent of
     * the amba device we are about to create.
     */
    if !parent.is_null() {
        (*dev).dev.parent = acpi_get_first_physical_node(parent);
    }

    device_set_node(&mut (*dev).dev, acpi_fwnode_handle(adev));

    ret = amba_device_add(dev, &mut iomem_resource);
    if ret != 0 {
        dev_err(&mut (*adev).dev, c"%s(): amba_device_add() failed (%d)\n".as_ptr(), c"amba_handler_attach".as_ptr(), ret);
        amba_device_put(dev);
        return ret;
    }

    1
}

static mut AMBA_HANDLER: acpi_scan_handler = acpi_scan_handler {
    ids: AMBA_ID_LIST.as_ptr(),
    attach: Some(amba_handler_attach),
};

pub unsafe fn acpi_amba_init() {
    amba_register_dummy_clk();
    acpi_scan_add_handler(&mut AMBA_HANDLER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
