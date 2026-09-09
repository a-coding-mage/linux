// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for OLPC XO-1.5 System Control Interrupts (SCI)
 *
 * Copyright (C) 2009-2010 One Laptop per Child
 */

// Dependencies supplied by the surrounding kernel translation.

const DRV_NAME: &[u8] = b"olpc-xo15-sci\0";
const PFX: &[u8] = b"olpc-xo15-sci: \0";

static mut XO15_SCI_GPE: libc::c_ulong = 0;
static mut LID_WAKE_ON_CLOSE: bool = false;

/*
 * The normal ACPI LID wakeup behavior is wake-on-open, but not
 * wake-on-close. This is implemented as standard by the XO-1.5 DSDT.
 *
 * We provide here a sysfs attribute that will additionally enable
 * wake-on-close behavior. This is useful (e.g.) when we opportunistically
 * suspend with the display running; if the lid is then closed, we want to
 * wake up to turn the display off.
 *
 * This is controlled through a custom method in the XO-1.5 DSDT.
 */
unsafe fn set_lid_wake_behavior(wake_on_close: bool) -> libc::c_int {
    let status = acpi_execute_simple_method(
        core::ptr::null_mut(),
        b"\\_SB.PCI0.LID.LIDW\0".as_ptr() as *const libc::c_char,
        wake_on_close,
    );
    if acpi_failure(status) {
        pr_warn(b"olpc-xo15-sci: failed to set lid behavior\n\0".as_ptr());
        return 1;
    }

    LID_WAKE_ON_CLOSE = wake_on_close;
    0
}

unsafe extern "C" fn lid_wake_on_close_show(
    _s: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut libc::c_char,
) -> libc::ssize_t {
    sprintf(buf, b"%u\n\0".as_ptr(), LID_WAKE_ON_CLOSE as libc::c_uint)
}

unsafe extern "C" fn lid_wake_on_close_store(
    _s: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const libc::c_char,
    n: libc::size_t,
) -> libc::ssize_t {
    let mut val: libc::c_uint = 0;
    if sscanf(buf, b"%u\0".as_ptr(), &mut val) != 1 {
        return -libc::EINVAL as libc::ssize_t;
    }

    set_lid_wake_behavior(val != 0);
    n as libc::ssize_t
}

static mut LID_WAKE_ON_CLOSE_ATTR: kobj_attribute = kobj_attribute {
    attr: attribute { name: b"lid_wake_on_close\0".as_ptr() as *const libc::c_char, mode: 0o644 },
    show: Some(lid_wake_on_close_show),
    store: Some(lid_wake_on_close_store),
};

unsafe fn battery_status_changed() {
    let psy = power_supply_get_by_name(b"olpc_battery\0".as_ptr() as *const libc::c_char);
    if !psy.is_null() {
        power_supply_changed(psy);
        power_supply_put(psy);
    }
}

unsafe fn ac_status_changed() {
    let psy = power_supply_get_by_name(b"olpc_ac\0".as_ptr() as *const libc::c_char);
    if !psy.is_null() {
        power_supply_changed(psy);
        power_supply_put(psy);
    }
}

unsafe fn process_sci_queue() {
    let mut data: u16;
    let mut r: libc::c_int;
    loop {
        data = 0;
        r = olpc_ec_sci_query(&mut data);
        if r != 0 || data == 0 { break; }

        pr_debug(b"olpc-xo15-sci: SCI 0x%x received\n\0".as_ptr(), data);
        match data {
            EC_SCI_SRC_BATERR | EC_SCI_SRC_BATSOC | EC_SCI_SRC_BATTERY | EC_SCI_SRC_BATCRIT => battery_status_changed(),
            EC_SCI_SRC_ACPWR => ac_status_changed(),
            _ => {}
        }
    }
    if r != 0 { pr_err(b"olpc-xo15-sci: Failed to clear SCI queue\0".as_ptr()); }
}

unsafe extern "C" fn process_sci_queue_work(_work: *mut work_struct) { process_sci_queue(); }

static mut SCI_WORK: work_struct = work_struct::new(process_sci_queue_work);

unsafe extern "C" fn xo15_sci_gpe_handler(
    _gpe_device: acpi_handle, _gpe: u32, _context: *mut libc::c_void,
) -> u32 {
    schedule_work(&mut SCI_WORK);
    ACPI_INTERRUPT_HANDLED | ACPI_REENABLE_GPE
}

unsafe extern "C" fn xo15_sci_probe(pdev: *mut platform_device) -> libc::c_int {
    let device = acpi_companion(&mut (*pdev).dev);
    if device.is_null() { return -libc::ENODEV; }
    let mut tmp: u64 = 0;
    let status = acpi_evaluate_integer((*device).handle, b"_GPE\0".as_ptr() as *const libc::c_char, core::ptr::null_mut(), &mut tmp);
    if acpi_failure(status) { return -libc::EINVAL; }
    XO15_SCI_GPE = tmp as libc::c_ulong;
    let status = acpi_install_gpe_handler(core::ptr::null_mut(), XO15_SCI_GPE, ACPI_GPE_EDGE_TRIGGERED, xo15_sci_gpe_handler, device);
    if acpi_failure(status) { return -libc::ENODEV; }
    dev_info(&mut (*pdev).dev, b"Initialized, GPE = 0x%lx\n\0".as_ptr(), XO15_SCI_GPE);
    let r = sysfs_create_file(&mut (*device).dev.kobj, &mut LID_WAKE_ON_CLOSE_ATTR.attr);
    if r != 0 {
        acpi_remove_gpe_handler(core::ptr::null_mut(), XO15_SCI_GPE, xo15_sci_gpe_handler);
        cancel_work_sync(&mut SCI_WORK);
        return r;
    }
    process_sci_queue();
    olpc_ec_mask_write(EC_SCI_SRC_ALL);
    acpi_enable_gpe(core::ptr::null_mut(), XO15_SCI_GPE);
    if (*device).wakeup.flags.valid { device_init_wakeup(&mut (*pdev).dev, true); }
    0
}

unsafe extern "C" fn xo15_sci_remove(pdev: *mut platform_device) {
    let device = acpi_companion(&mut (*pdev).dev);
    device_init_wakeup(&mut (*pdev).dev, false);
    acpi_disable_gpe(core::ptr::null_mut(), XO15_SCI_GPE);
    acpi_remove_gpe_handler(core::ptr::null_mut(), XO15_SCI_GPE, xo15_sci_gpe_handler);
    cancel_work_sync(&mut SCI_WORK);
    sysfs_remove_file(&mut (*device).dev.kobj, &mut LID_WAKE_ON_CLOSE_ATTR.attr);
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn xo15_sci_resume(_dev: *mut device) -> libc::c_int {
    olpc_ec_mask_write(EC_SCI_SRC_ALL);
    battery_status_changed();
    ac_status_changed();
    0
}

static mut XO15_SCI_PM: simple_dev_pm_ops = simple_dev_pm_ops::new(None, Some(xo15_sci_resume));

static XO15_SCI_DEVICE_IDS: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"XO15EC\0\0", driver_data: 0 },
    acpi_device_id { id: [0; 8], driver_data: 0 },
];

static mut XO15_SCI_DRV: platform_driver = platform_driver {
    probe: Some(xo15_sci_probe),
    remove: Some(xo15_sci_remove),
    driver: driver { name: DRV_NAME.as_ptr() as *const libc::c_char, acpi_match_table: XO15_SCI_DEVICE_IDS.as_ptr(), pm: &XO15_SCI_PM },
};

unsafe extern "C" fn xo15_sci_init() -> libc::c_int {
    platform_driver_register(&mut XO15_SCI_DRV)
}

device_initcall!(xo15_sci_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
