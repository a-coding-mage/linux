// SPDX-License-Identifier: GPL-2.0
/*
 * AMD CDX bus driver MSI support
 *
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Linux kernel headers and "cdx.h" provide the types, constants, and
// functions referenced below.

unsafe fn cdx_msi_write_msg(irq_data: *mut irq_data, msg: *mut msi_msg) {
    let msi_desc: *mut msi_desc = irq_data_get_msi_desc(irq_data);
    let cdx_dev: *mut cdx_device = to_cdx_device((*msi_desc).dev);

    /* We would not operate on msg here rather we wait for irq_bus_sync_unlock()
     * to be called from preemptible task context.
     */
    (*msi_desc).msg = *msg;
    (*cdx_dev).msi_write_pending = true;
}

unsafe fn cdx_msi_write_irq_lock(irq_data: *mut irq_data) {
    let msi_desc: *mut msi_desc = irq_data_get_msi_desc(irq_data);
    let cdx_dev: *mut cdx_device = to_cdx_device((*msi_desc).dev);

    mutex_lock(&mut (*cdx_dev).irqchip_lock);
}

unsafe fn cdx_msi_write_irq_unlock(irq_data: *mut irq_data) {
    let msi_desc: *mut msi_desc = irq_data_get_msi_desc(irq_data);
    let cdx_dev: *mut cdx_device = to_cdx_device((*msi_desc).dev);
    let cdx: *mut cdx_controller = (*cdx_dev).cdx;
    let mut dev_config: cdx_device_config = core::mem::zeroed();

    if !(*cdx_dev).msi_write_pending {
        mutex_unlock(&mut (*cdx_dev).irqchip_lock);
        return;
    }

    (*cdx_dev).msi_write_pending = false;
    mutex_unlock(&mut (*cdx_dev).irqchip_lock);

    dev_config.msi.msi_index = (*msi_desc).msi_index;
    dev_config.msi.data = (*msi_desc).msg.data;
    dev_config.msi.addr = ((*msi_desc).msg.address_hi as u64).wrapping_shl(32)
        | (*msi_desc).msg.address_lo as u64;

    /*
     * dev_configure() is a controller callback which can interact with
     * Firmware or other entities, and can sleep, so invoke this function
     * outside of the mutex held region.
     */
    dev_config.type_ = CDX_DEV_MSI_CONF;
    if let Some(dev_configure) = (*(*cdx).ops).dev_configure {
        dev_configure(cdx, (*cdx_dev).bus_num, (*cdx_dev).dev_num, &mut dev_config);
    }
}

unsafe fn cdx_enable_msi(cdx_dev: *mut cdx_device) -> i32 {
    let cdx: *mut cdx_controller = (*cdx_dev).cdx;
    let mut dev_config: cdx_device_config = core::mem::zeroed();

    dev_config.type_ = CDX_DEV_MSI_ENABLE;
    dev_config.msi_enable = true;
    if let Some(dev_configure) = (*(*cdx).ops).dev_configure {
        return dev_configure(cdx, (*cdx_dev).bus_num, (*cdx_dev).dev_num, &mut dev_config);
    }

    -EOPNOTSUPP
}

unsafe fn cdx_disable_msi(cdx_dev: *mut cdx_device) {
    let cdx: *mut cdx_controller = (*cdx_dev).cdx;
    let mut dev_config: cdx_device_config = core::mem::zeroed();

    dev_config.type_ = CDX_DEV_MSI_ENABLE;
    dev_config.msi_enable = false;
    if let Some(dev_configure) = (*(*cdx).ops).dev_configure {
        dev_configure(cdx, (*cdx_dev).bus_num, (*cdx_dev).dev_num, &mut dev_config);
    }
}

// EXPORT_SYMBOL_GPL(cdx_enable_msi);
// EXPORT_SYMBOL_GPL(cdx_disable_msi);

static mut cdx_msi_irq_chip: irq_chip = irq_chip {
    name: "CDX-MSI\0" as *const str as *const i8,
    irq_mask: Some(irq_chip_mask_parent),
    irq_unmask: Some(irq_chip_unmask_parent),
    irq_eoi: Some(irq_chip_eoi_parent),
    irq_set_affinity: Some(msi_domain_set_affinity),
    irq_write_msi_msg: Some(cdx_msi_write_msg),
    irq_bus_lock: Some(cdx_msi_write_irq_lock),
    irq_bus_sync_unlock: Some(cdx_msi_write_irq_unlock),
};

/* Convert an msi_desc to a unique identifier within the domain. */
unsafe fn cdx_domain_calc_hwirq(dev: *mut cdx_device, desc: *mut msi_desc) -> irq_hw_number_t {
    ((*dev).msi_dev_id as irq_hw_number_t).wrapping_shl(10) | (*desc).msi_index as irq_hw_number_t
}

unsafe fn cdx_msi_set_desc(arg: *mut msi_alloc_info_t, desc: *mut msi_desc) {
    (*arg).desc = desc;
    (*arg).hwirq = cdx_domain_calc_hwirq(to_cdx_device((*desc).dev), desc);
}

unsafe fn cdx_msi_prepare(
    msi_domain: *mut irq_domain,
    dev: *mut device,
    nvec: i32,
    info: *mut msi_alloc_info_t,
) -> i32 {
    let mut msi_spec: of_phandle_args = core::mem::zeroed();
    let cdx_dev: *mut cdx_device = to_cdx_device(dev);
    let parent: *mut device = (*(*cdx_dev).cdx).dev;
    let msi_info: *mut msi_domain_info;
    let ret: i32;

    ret = of_map_msi_id((*parent).of_node, (*cdx_dev).msi_dev_id, core::ptr::null_mut(), &mut msi_spec);
    if ret != 0 {
        dev_err(dev, "of_map_msi_id failed for MSI: %d\n", ret);
        return ret;
    }
    of_node_put(msi_spec.np);

    // GENERIC_MSI_DOMAIN_OPS: preserve the conditional scratchpad assignment.
    (*info).scratchpad[0].ul = msi_spec.args[0];

    msi_info = msi_get_domain_info((*msi_domain).parent);
    ((*msi_info).ops).msi_prepare.unwrap()((*msi_domain).parent, dev, nvec, info)
}

static mut cdx_msi_ops: msi_domain_ops = msi_domain_ops {
    msi_prepare: Some(cdx_msi_prepare),
    set_desc: Some(cdx_msi_set_desc),
};

static mut cdx_msi_domain_info: msi_domain_info = msi_domain_info {
    ops: &raw mut cdx_msi_ops,
    chip: &raw mut cdx_msi_irq_chip,
    flags: MSI_FLAG_USE_DEF_DOM_OPS | MSI_FLAG_USE_DEF_CHIP_OPS |
        MSI_FLAG_ALLOC_SIMPLE_MSI_DESCS | MSI_FLAG_FREE_MSI_DESCS,
};

unsafe fn cdx_msi_domain_init(dev: *mut device) -> *mut irq_domain {
    let np: *mut device_node = (*dev).of_node;
    let fwnode_handle: *mut fwnode_handle;
    let cdx_msi_domain: *mut irq_domain;
    let parent_node: *mut device_node;
    let parent: *mut irq_domain;

    fwnode_handle = of_fwnode_handle(np);
    parent_node = of_parse_phandle(np, "msi-map\0" as *const str as *const i8, 1);
    if parent_node.is_null() {
        dev_err(dev, "msi-map not present on cdx controller\n");
        return core::ptr::null_mut();
    }

    parent = irq_find_matching_fwnode(of_fwnode_handle(parent_node), DOMAIN_BUS_NEXUS);
    of_node_put(parent_node);
    if parent.is_null() || msi_get_domain_info(parent).is_null() {
        dev_err(dev, "unable to locate ITS domain\n");
        return core::ptr::null_mut();
    }

    cdx_msi_domain = msi_create_irq_domain(fwnode_handle, &raw mut cdx_msi_domain_info, parent);
    if cdx_msi_domain.is_null() {
        dev_err(dev, "unable to create CDX-MSI domain\n");
        return core::ptr::null_mut();
    }

    dev_dbg(dev, "CDX-MSI domain created\n");
    cdx_msi_domain
}

// EXPORT_SYMBOL_NS_GPL(cdx_msi_domain_init, "CDX_BUS_CONTROLLER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
