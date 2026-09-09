// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015-2018, Intel Corporation.
 * Copyright (c) 2021, IBM Corp.
 */

// Linux headers and the kcs_bmc interfaces are supplied by other translation units.

/* Implement both the device and client interfaces here */

/* Record registered devices and drivers */
static mut kcs_bmc_lock: core::ffi::c_void = core::ffi::c_void {};
static mut kcs_bmc_devices: core::ffi::c_void = core::ffi::c_void {};
static mut kcs_bmc_drivers: core::ffi::c_void = core::ffi::c_void {};

/* Consumer data access */

pub unsafe fn kcs_bmc_read_data(kcs_bmc: *mut kcs_bmc_device) -> u8 {
    ((*(*kcs_bmc).ops).io_inputb)(kcs_bmc, (*kcs_bmc).ioreg.idr)
}

pub unsafe fn kcs_bmc_write_data(kcs_bmc: *mut kcs_bmc_device, data: u8) {
    ((*(*kcs_bmc).ops).io_outputb)(kcs_bmc, (*kcs_bmc).ioreg.odr, data);
}

pub unsafe fn kcs_bmc_read_status(kcs_bmc: *mut kcs_bmc_device) -> u8 {
    ((*(*kcs_bmc).ops).io_inputb)(kcs_bmc, (*kcs_bmc).ioreg.str_)
}

pub unsafe fn kcs_bmc_write_status(kcs_bmc: *mut kcs_bmc_device, data: u8) {
    ((*(*kcs_bmc).ops).io_outputb)(kcs_bmc, (*kcs_bmc).ioreg.str_, data);
}

pub unsafe fn kcs_bmc_update_status(kcs_bmc: *mut kcs_bmc_device, mask: u8, val: u8) {
    ((*(*kcs_bmc).ops).io_updateb)(kcs_bmc, (*kcs_bmc).ioreg.str_, mask, val);
}

pub unsafe fn kcs_bmc_handle_event(kcs_bmc: *mut kcs_bmc_device) -> irqreturn_t {
    let mut client: *mut kcs_bmc_client;
    let mut rc: irqreturn_t = IRQ_NONE;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*kcs_bmc).lock, &mut flags);
    client = (*kcs_bmc).client;
    if !client.is_null() {
        rc = ((*(*client).ops).event)(client);
    }
    spin_unlock_irqrestore(&mut (*kcs_bmc).lock, flags);
    rc
}

pub unsafe fn kcs_bmc_enable_device(
    kcs_bmc: *mut kcs_bmc_device,
    client: *mut kcs_bmc_client,
) -> i32 {
    let rc: i32;
    spin_lock_irq(&mut (*kcs_bmc).lock);
    if !(*kcs_bmc).client.is_null() {
        rc = -EBUSY;
    } else {
        let mask: u8 = KCS_BMC_EVENT_TYPE_IBF;
        (*kcs_bmc).client = client;
        kcs_bmc_update_event_mask(kcs_bmc, mask, mask);
        rc = 0;
    }
    spin_unlock_irq(&mut (*kcs_bmc).lock);
    rc
}

pub unsafe fn kcs_bmc_disable_device(
    kcs_bmc: *mut kcs_bmc_device,
    client: *mut kcs_bmc_client,
) {
    spin_lock_irq(&mut (*kcs_bmc).lock);
    if client == (*kcs_bmc).client {
        let mask: u8 = KCS_BMC_EVENT_TYPE_IBF | KCS_BMC_EVENT_TYPE_OBE;
        kcs_bmc_update_event_mask(kcs_bmc, mask, 0);
        (*kcs_bmc).client = core::ptr::null_mut();
    }
    spin_unlock_irq(&mut (*kcs_bmc).lock);
}

pub unsafe fn kcs_bmc_add_device(kcs_bmc: *mut kcs_bmc_device) -> i32 {
    let mut error: i32 = 0;
    spin_lock_init(&mut (*kcs_bmc).lock);
    (*kcs_bmc).client = core::ptr::null_mut();
    mutex_lock(&mut kcs_bmc_lock);
    list_add(&mut (*kcs_bmc).entry, &mut kcs_bmc_devices);
    /* list_for_each_entry(drv, &kcs_bmc_drivers, entry) */
    for drv in kcs_bmc_drivers_iter_mut() {
        let rc = ((*(*drv).ops).add_device)(kcs_bmc);
        if rc != 0 {
            dev_err((*kcs_bmc).dev, "Failed to add chardev for KCS channel %d: %d", (*kcs_bmc).channel, rc);
            error = rc;
        }
    }
    mutex_unlock(&mut kcs_bmc_lock);
    error
}

pub unsafe fn kcs_bmc_remove_device(kcs_bmc: *mut kcs_bmc_device) {
    mutex_lock(&mut kcs_bmc_lock);
    list_del(&mut (*kcs_bmc).entry);
    /* list_for_each_entry(drv, &kcs_bmc_drivers, entry) */
    for drv in kcs_bmc_drivers_iter_mut() {
        let rc = ((*(*drv).ops).remove_device)(kcs_bmc);
        if rc != 0 { dev_err((*kcs_bmc).dev, "Failed to remove chardev for KCS channel %d: %d", (*kcs_bmc).channel, rc); }
    }
    mutex_unlock(&mut kcs_bmc_lock);
}

pub unsafe fn kcs_bmc_register_driver(drv: *mut kcs_bmc_driver) {
    mutex_lock(&mut kcs_bmc_lock);
    list_add(&mut (*drv).entry, &mut kcs_bmc_drivers);
    /* list_for_each_entry(kcs_bmc, &kcs_bmc_devices, entry) */
    for kcs_bmc in kcs_bmc_devices_iter_mut() {
        let rc = ((*(*drv).ops).add_device)(kcs_bmc);
        if rc != 0 { dev_err((*kcs_bmc).dev, "Failed to add driver for KCS channel %d: %d", (*kcs_bmc).channel, rc); }
    }
    mutex_unlock(&mut kcs_bmc_lock);
}

pub unsafe fn kcs_bmc_unregister_driver(drv: *mut kcs_bmc_driver) {
    mutex_lock(&mut kcs_bmc_lock);
    list_del(&mut (*drv).entry);
    /* list_for_each_entry(kcs_bmc, &kcs_bmc_devices, entry) */
    for kcs_bmc in kcs_bmc_devices_iter_mut() {
        let rc = ((*(*drv).ops).remove_device)(kcs_bmc);
        if rc != 0 { dev_err((*kcs_bmc).dev, "Failed to remove driver for KCS channel %d: %d", (*kcs_bmc).channel, rc); }
    }
    mutex_unlock(&mut kcs_bmc_lock);
}

pub unsafe fn kcs_bmc_update_event_mask(kcs_bmc: *mut kcs_bmc_device, mask: u8, events: u8) {
    ((*(*kcs_bmc).ops).irq_mask_update)(kcs_bmc, mask, events);
}

// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Haiyue Wang <haiyue.wang@linux.intel.com>");
// MODULE_AUTHOR("Andrew Jeffery <andrew@aj.id.au>");
// MODULE_DESCRIPTION("KCS BMC to handle the IPMI request from system software");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
