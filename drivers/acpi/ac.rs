// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  acpi_ac.c - ACPI AC Adapter Driver (Revision: 27)
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 */

// Kernel dependencies supplied by the surrounding repository.

const ACPI_AC_FILE_STATE: &str = "state";
const ACPI_AC_NOTIFY_STATUS: u32 = 0x80;
const ACPI_AC_STATUS_OFFLINE: u8 = 0x00;
const ACPI_AC_STATUS_ONLINE: u8 = 0x01;
const ACPI_AC_STATUS_UNKNOWN: u8 = 0xff;

static ac_device_ids: [acpi_device_id; 2] = [
    acpi_device_id { id: "ACPI0003", driver_data: 0 },
    acpi_device_id { id: "", driver_data: 0 },
];

// MODULE_AUTHOR("Paul Diefenbaugh");
// MODULE_DESCRIPTION("ACPI AC Adapter Driver");
// MODULE_LICENSE("GPL");

static mut ac_sleep_before_get_state_ms: i32 = 0;
static mut ac_only: i32 = 0;

#[repr(C)]
struct acpi_ac {
    charger: *mut power_supply,
    charger_desc: power_supply_desc,
    device: *mut acpi_device,
    state: u64,
    battery_nb: notifier_block,
}

unsafe fn acpi_ac_get_state(ac: *mut acpi_ac) -> i32 {
    let mut status = AE_OK;
    if ac.is_null() { return -EINVAL; }
    if ac_only != 0 {
        (*ac).state = 1;
        return 0;
    }
    status = acpi_evaluate_integer((*(*ac).device).handle, "_PSR", core::ptr::null_mut(), &mut (*ac).state);
    if ACPI_FAILURE(status) {
        acpi_handle_info((*(*ac).device).handle, "Error reading AC Adapter state: %s\n", acpi_format_exception(status));
        (*ac).state = ACPI_AC_STATUS_UNKNOWN as u64;
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn get_ac_property(psy: *mut power_supply, psp: power_supply_property, val: *mut power_supply_propval) -> i32 {
    let ac = power_supply_get_drvdata(psy) as *mut acpi_ac;
    if ac.is_null() { return -ENODEV; }
    if acpi_ac_get_state(ac) != 0 { return -ENODEV; }
    match psp {
        POWER_SUPPLY_PROP_ONLINE => (*val).intval = (*ac).state as i32,
        _ => return -EINVAL,
    }
    0
}

static ac_props: [power_supply_property; 1] = [POWER_SUPPLY_PROP_ONLINE];

unsafe extern "C" fn acpi_ac_notify(handle: acpi_handle, event: u32, data: *mut core::ffi::c_void) {
    let ac = data as *mut acpi_ac;
    let adev = (*ac).device;
    match event {
        ACPI_AC_NOTIFY_STATUS | ACPI_NOTIFY_BUS_CHECK | ACPI_NOTIFY_DEVICE_CHECK => {
            if ac_sleep_before_get_state_ms > 0 { msleep(ac_sleep_before_get_state_ms as u32); }
            acpi_ac_get_state(ac);
            acpi_bus_generate_netlink_event(ACPI_AC_CLASS, dev_name(&(*adev).dev), event, (*ac).state as u32);
            acpi_notifier_call_chain(ACPI_AC_CLASS, acpi_device_bid(adev), event, (*ac).state as u32);
            power_supply_changed((*ac).charger);
        }
        _ => {
            acpi_handle_debug((*adev).handle, "Unsupported event [0x%x]\n", event);
            acpi_ac_notify(handle, ACPI_AC_NOTIFY_STATUS, data);
        }
    }
}

unsafe extern "C" fn acpi_ac_battery_notify(nb: *mut notifier_block, _action: usize, data: *mut core::ffi::c_void) -> i32 {
    let ac = container_of!(nb, acpi_ac, battery_nb);
    let event = data as *mut acpi_bus_event;
    if strcmp((*event).device_class, ACPI_BATTERY_CLASS) == 0 && (*event).type_ == ACPI_BATTERY_NOTIFY_STATUS {
        acpi_ac_get_state(ac);
    }
    NOTIFY_OK
}

unsafe extern "C" fn thinkpad_e530_quirk(_d: *const dmi_system_id) -> i32 {
    ac_sleep_before_get_state_ms = 1000;
    0
}

unsafe extern "C" fn ac_only_quirk(_d: *const dmi_system_id) -> i32 {
    ac_only = 1;
    0
}

// Please keep this list alphabetically sorted.
static ac_dmi_table: [dmi_system_id; 3] = [
    dmi_system_id { callback: Some(ac_only_quirk), matches: &[DMI_MATCH(DMI_PRODUCT_NAME, "GK45")] },
    dmi_system_id { callback: Some(thinkpad_e530_quirk), matches: &[DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"), DMI_MATCH(DMI_PRODUCT_NAME, "32597CG")] },
    dmi_system_id::default(),
];

unsafe extern "C" fn acpi_ac_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let adev = ACPI_COMPANION(dev);
    if adev.is_null() { return -ENODEV; }
    let ac = devm_kzalloc(dev, core::mem::size_of::<acpi_ac>(), GFP_KERNEL) as *mut acpi_ac;
    if ac.is_null() { return -ENOMEM; }
    (*ac).device = adev;
    platform_set_drvdata(pdev, ac as *mut _);
    let result = acpi_ac_get_state(ac);
    if result != 0 { return result; }
    let mut psy_cfg = power_supply_config::default();
    psy_cfg.drv_data = ac as *mut _;
    (*ac).charger_desc.name = acpi_device_bid(adev);
    (*ac).charger_desc.type_ = POWER_SUPPLY_TYPE_MAINS;
    (*ac).charger_desc.properties = ac_props.as_ptr();
    (*ac).charger_desc.num_properties = ac_props.len();
    (*ac).charger_desc.get_property = Some(get_ac_property);
    (*ac).charger = devm_power_supply_register(dev, &mut (*ac).charger_desc, &psy_cfg);
    if IS_ERR((*ac).charger) { return PTR_ERR((*ac).charger); }
    pr_info!("AC Adapter [%s] (%s-line)\n", acpi_device_bid(adev), str_on_off((*ac).state));
    let result = devm_acpi_install_notify_handler(dev, ACPI_ALL_NOTIFY, Some(acpi_ac_notify), ac as *mut _);
    if result != 0 { return result; }
    (*ac).battery_nb.notifier_call = Some(acpi_ac_battery_notify);
    register_acpi_notifier(&mut (*ac).battery_nb);
    0
}

unsafe extern "C" fn acpi_ac_remove(pdev: *mut platform_device) {
    let ac = platform_get_drvdata(pdev) as *mut acpi_ac;
    unregister_acpi_notifier(&mut (*ac).battery_nb);
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn acpi_ac_resume(dev: *mut device) -> i32 {
    let ac = dev_get_drvdata(dev) as *mut acpi_ac;
    let old_state = (*ac).state as u32;
    if acpi_ac_get_state(ac) != 0 { return 0; }
    if old_state != (*ac).state as u32 { power_supply_changed((*ac).charger); }
    0
}

static mut acpi_ac_pm: dev_pm_ops = SIMPLE_DEV_PM_OPS!(None, Some(acpi_ac_resume));

static mut acpi_ac_driver: platform_driver = platform_driver {
    probe: Some(acpi_ac_probe),
    remove: Some(acpi_ac_remove),
    driver: device_driver {
        name: "ac",
        acpi_match_table: ac_device_ids.as_ptr(),
        pm: &acpi_ac_pm,
    },
};

unsafe extern "C" fn acpi_ac_init() -> i32 {
    if acpi_disabled || acpi_quirk_skip_acpi_ac_and_battery() { return -ENODEV; }
    dmi_check_system(ac_dmi_table.as_ptr());
    if platform_driver_register(&mut acpi_ac_driver) < 0 { return -ENODEV; }
    0
}

unsafe extern "C" fn acpi_ac_exit() { platform_driver_unregister(&mut acpi_ac_driver); }

// module_init(acpi_ac_init);
// module_exit(acpi_ac_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
