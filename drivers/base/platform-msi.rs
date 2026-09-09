// SPDX-License-Identifier: GPL-2.0
/*
 * MSI framework for platform devices
 *
 * Copyright (C) 2015 ARM Limited, All Rights Reserved.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 * Copyright (C) 2022 Linutronix GmbH
 */

/* Dependencies supplied by the surrounding kernel translation unit. */

/*
 * This indirection can go when platform_device_msi_init_and_alloc_irqs()
 * is switched to a proper irq_chip::irq_write_msi_msg() callback. Keep it
 * simple for now.
 */
unsafe fn platform_msi_write_msi_msg(d: *mut irq_data, msg: *mut msi_msg) {
    let cb: irq_write_msi_msg_t = (*d).chip_data as irq_write_msi_msg_t;

    cb(irq_data_get_msi_desc(d), msg);
}

unsafe fn platform_msi_set_desc(arg: *mut msi_alloc_info_t, desc: *mut msi_desc) {
    (*arg).desc = desc;
    (*arg).hwirq = (*desc).msi_index;
}

static platform_msi_template: msi_domain_template = msi_domain_template {
    chip: irq_chip {
        name: "pMSI\\0".as_ptr() as *const i8,
        irq_mask: Some(irq_chip_mask_parent),
        irq_unmask: Some(irq_chip_unmask_parent),
        irq_write_msi_msg: Some(platform_msi_write_msi_msg),
        /* The rest is filled in by the platform MSI parent */
        ..unsafe { core::mem::zeroed() }
    },

    ops: msi_domain_ops {
        set_desc: Some(platform_msi_set_desc),
        ..unsafe { core::mem::zeroed() }
    },

    info: msi_domain_info {
        bus_token: DOMAIN_BUS_DEVICE_MSI,
        ..unsafe { core::mem::zeroed() }
    },
};

/**
 * platform_device_msi_init_and_alloc_irqs - Initialize platform device MSI
 *                                             and allocate interrupts for @dev
 * @dev:                The device for which to allocate interrupts
 * @nvec:               The number of interrupts to allocate
 * @write_msi_msg:      Callback to write an interrupt message for @dev
 *
 * Returns:
 * Zero for success, or an error code in case of failure
 *
 * This creates a MSI domain on @dev which has @dev->msi.domain as
 * parent. The parent domain sets up the new domain. The domain has
 * a fixed size of @nvec. The domain is managed by devres and will
 * be removed when the device is removed.
 */
pub unsafe fn platform_device_msi_init_and_alloc_irqs(
    dev: *mut device,
    nvec: u32,
    write_msi_msg: irq_write_msi_msg_t,
) -> i32 {
    let domain: *mut irq_domain = (*dev).msi.domain;

    if domain.is_null() || write_msi_msg.is_none() {
        return -EINVAL;
    }

    /*
     * @write_msi_msg is stored in the resulting msi_domain_info::data.
     * The underlying domain creation mechanism will assign that
     * callback to the resulting irq chip.
     */
    if !msi_create_device_irq_domain(
        dev,
        MSI_DEFAULT_DOMAIN,
        &platform_msi_template,
        nvec,
        core::ptr::null_mut(),
        write_msi_msg,
    ) {
        return -ENODEV;
    }

    msi_domain_alloc_irqs_range(dev, MSI_DEFAULT_DOMAIN, 0, nvec.wrapping_sub(1))
}

/**
 * platform_device_msi_free_irqs_all - Free all interrupts for @dev
 * @dev:    The device for which to free interrupts
 */
pub unsafe fn platform_device_msi_free_irqs_all(dev: *mut device) {
    msi_domain_free_irqs_all(dev, MSI_DEFAULT_DOMAIN);
    msi_remove_device_irq_domain(dev, MSI_DEFAULT_DOMAIN);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
