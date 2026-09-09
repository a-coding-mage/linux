// SPDX-License-Identifier: GPL-2.0
/*
 * Freescale Management Complex (MC) bus driver MSI support
 *
 * Copyright (C) 2015-2016 Freescale Semiconductor, Inc.
 * Author: German Rivera <German.Rivera@freescale.com>
 *
 */

// Dependencies: linux/of_irq.h, linux/irq.h, linux/irqdomain.h,
// linux/msi.h, linux/acpi_iort.h, and fsl-mc-private.h.

unsafe fn fsl_mc_write_msi_msg(msi_desc: *mut msi_desc, msg: *mut msi_msg) {
    let mc_bus_dev: *mut fsl_mc_device = to_fsl_mc_device((*msi_desc).dev);
    let mc_bus: *mut fsl_mc_bus = to_fsl_mc_bus(mc_bus_dev);
    let mc_dev_irq: *mut fsl_mc_device_irq =
        &mut (*mc_bus).irq_resources[(*msi_desc).msi_index as usize];
    let owner_mc_dev: *mut fsl_mc_device = (*mc_dev_irq).mc_dev;
    let mut irq_cfg: dprc_irq_cfg;
    let error: i32;

    (*msi_desc).msg = *msg;

    /*
     * msi_desc->msg.address is 0x0 when this function is invoked in
     * the free_irq() code path. In this case, for the MC, we don't
     * really need to "unprogram" the MSI, so we just return.
     */
    if (*msi_desc).msg.address_lo == 0x0 && (*msi_desc).msg.address_hi == 0x0 {
        return;
    }

    if owner_mc_dev.is_null() {
        return;
    }

    irq_cfg.paddr = ((*msi_desc).msg.address_hi as u64) << 32 |
        (*msi_desc).msg.address_lo as u64;
    irq_cfg.val = (*msi_desc).msg.data;
    irq_cfg.irq_num = (*msi_desc).irq;

    if owner_mc_dev == mc_bus_dev {
        /*
         * IRQ is for the mc_bus_dev's DPRC itself
         */
        error = dprc_set_irq(
            (*mc_bus_dev).mc_io,
            MC_CMD_FLAG_INTR_DIS | MC_CMD_FLAG_PRI,
            (*mc_bus_dev).mc_handle,
            (*mc_dev_irq).dev_irq_index,
            &mut irq_cfg,
        );
        if error < 0 {
            dev_err(
                &mut (*owner_mc_dev).dev,
                "dprc_set_irq() failed: %d\n",
                error,
            );
        }
    } else {
        /*
         * IRQ is for for a child device of mc_bus_dev
         */
        error = dprc_set_obj_irq(
            (*mc_bus_dev).mc_io,
            MC_CMD_FLAG_INTR_DIS | MC_CMD_FLAG_PRI,
            (*mc_bus_dev).mc_handle,
            (*owner_mc_dev).obj_desc.type,
            (*owner_mc_dev).obj_desc.id,
            (*mc_dev_irq).dev_irq_index,
            &mut irq_cfg,
        );
        if error < 0 {
            dev_err(
                &mut (*owner_mc_dev).dev,
                "dprc_obj_set_irq() failed: %d\n",
                error,
            );
        }
    }
}

unsafe fn fsl_mc_get_msi_parent(dev: *mut device) -> *mut irq_domain {
    let mc_dev: *mut fsl_mc_device = to_fsl_mc_device(dev);
    let mut root_dprc_dev: *mut device = core::ptr::null_mut();
    let bus_dev: *mut device;

    fsl_mc_get_root_dprc(dev, &mut root_dprc_dev);
    bus_dev = (*root_dprc_dev).parent;

    if !(*bus_dev).of_node.is_null() {
        of_msi_get_domain(bus_dev, (*bus_dev).of_node, DOMAIN_BUS_NEXUS)
    } else {
        iort_get_device_domain(bus_dev, (*mc_dev).icid, DOMAIN_BUS_NEXUS)
    }
}

unsafe fn fsl_mc_msi_domain_alloc_irqs(dev: *mut device, irq_count: u32) -> i32 {
    let mut error = msi_setup_device_data(dev);
    if error != 0 {
        return error;
    }

    error = platform_device_msi_init_and_alloc_irqs(dev, irq_count, fsl_mc_write_msi_msg);
    if error != 0 {
        dev_err(dev, "Failed to allocate IRQs\n");
    }
    error
}

unsafe fn fsl_mc_msi_domain_free_irqs(dev: *mut device) {
    msi_domain_free_irqs_all(dev, MSI_DEFAULT_DOMAIN);
}

unsafe fn fsl_mc_get_msi_id(dev: *mut device) -> u32 {
    let mc_dev: *mut fsl_mc_device = to_fsl_mc_device(dev);
    let mut root_dprc_dev: *mut device = core::ptr::null_mut();

    fsl_mc_get_root_dprc(dev, &mut root_dprc_dev);

    if !(*(*root_dprc_dev).parent).of_node.is_null() {
        of_msi_xlate(dev, core::ptr::null(), (*mc_dev).icid)
    } else {
        iort_msi_map_id(dev, (*mc_dev).icid)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
