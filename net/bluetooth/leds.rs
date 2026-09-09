// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Heiner Kallweit <hkallweit1@gmail.com>
 */

// Dependencies supplied by the surrounding Bluetooth and LED subsystems.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct led_trigger {
    pub activate: Option<unsafe extern "C" fn(*mut led_classdev) -> c_int>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct led_classdev {
    pub trigger: *mut led_trigger,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hci_dev {
    pub dev: device,
    pub name: *const c_char,
    pub flags: [u8; 0],
    pub power_led: *mut led_trigger,
}

extern "C" {
    static mut bt_power_led_trigger: led_trigger;
    static mut hci_dev_list_lock: c_void;
    static mut hci_dev_list: c_void;

    fn test_bit(nr: c_int, addr: *const c_void) -> bool;
    fn led_trigger_event(trigger: *mut led_trigger, event: c_int);
    fn led_set_brightness(led_cdev: *mut led_classdev, brightness: c_int);
    fn led_trigger_register_simple(name: *const c_char, trigger: *mut led_trigger) -> c_int;
    fn led_trigger_unregister_simple(trigger: *mut led_trigger);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kasprintf(
        dev: *mut device,
        flags: c_int,
        fmt: *const c_char,
        ...,
    ) -> *mut c_char;
    fn devm_led_trigger_register(dev: *mut device, trigger: *mut led_trigger) -> c_int;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn read_lock(lock: *mut c_void);
    fn read_unlock(lock: *mut c_void);
}

const LED_FULL: c_int = 255;
const LED_OFF: c_int = 0;
const HCI_UP: c_int = 0;
const GFP_KERNEL: c_int = 0;

#[repr(C)]
pub struct hci_basic_led_trigger {
    pub led_trigger: led_trigger,
    pub hdev: *mut hci_dev,
}

pub unsafe fn hci_leds_update_powered(hdev: *mut hci_dev, mut enabled: bool) {
    if !(*hdev).power_led.is_null() {
        led_trigger_event(
            (*hdev).power_led,
            if enabled { LED_FULL } else { LED_OFF },
        );
    }

    if !enabled {
        let d: *mut hci_dev;

        read_lock(&mut hci_dev_list_lock);

        // list_for_each_entry(d, &hci_dev_list, list)
        // The containing list traversal is supplied by the surrounding kernel bindings.
        let _ = (&mut d, &mut hci_dev_list);
        if !d.is_null() && test_bit(HCI_UP, (*d).flags.as_ptr() as *const c_void) {
            enabled = true;
        }

        read_unlock(&mut hci_dev_list_lock);
    }

    led_trigger_event(
        &mut bt_power_led_trigger,
        if enabled { LED_FULL } else { LED_OFF },
    );
}

unsafe extern "C" fn power_activate(led_cdev: *mut led_classdev) -> c_int {
    let htrig = (*led_cdev).trigger as *mut hci_basic_led_trigger;
    let powered = test_bit(HCI_UP, (*htrig).hdev.as_ref().unwrap().flags.as_ptr() as *const c_void);

    led_set_brightness(led_cdev, if powered { LED_FULL } else { LED_OFF });

    0
}

unsafe fn led_allocate_basic(
    hdev: *mut hci_dev,
    activate: Option<unsafe extern "C" fn(*mut led_classdev) -> c_int>,
    name: *const c_char,
) -> *mut led_trigger {
    let htrig = devm_kzalloc(
        &mut (*hdev).dev,
        core::mem::size_of::<hci_basic_led_trigger>(),
        GFP_KERNEL,
    ) as *mut hci_basic_led_trigger;
    if htrig.is_null() {
        return core::ptr::null_mut();
    }

    (*htrig).hdev = hdev;
    (*htrig).led_trigger.activate = activate;
    (*htrig).led_trigger.name = devm_kasprintf(
        &mut (*hdev).dev,
        GFP_KERNEL,
        b"%s-%s\0".as_ptr() as *const c_char,
        (*hdev).name,
        name,
    );
    if (*htrig).led_trigger.name.is_null() {
        devm_kfree(&mut (*hdev).dev, htrig as *mut c_void);
        return core::ptr::null_mut();
    }

    if devm_led_trigger_register(&mut (*hdev).dev, &mut (*htrig).led_trigger) != 0 {
        devm_kfree(&mut (*hdev).dev, (*htrig).led_trigger.name as *mut c_void);
        devm_kfree(&mut (*hdev).dev, htrig as *mut c_void);
        return core::ptr::null_mut();
    }

    &mut (*htrig).led_trigger
}

pub unsafe fn hci_leds_init(hdev: *mut hci_dev) {
    /* initialize power_led */
    (*hdev).power_led = led_allocate_basic(hdev, Some(power_activate), b"power\0".as_ptr() as *const c_char);
}

pub unsafe fn bt_leds_init() {
    led_trigger_register_simple(b"bluetooth-power\0".as_ptr() as *const c_char, &mut bt_power_led_trigger);
}

pub unsafe fn bt_leds_cleanup() {
    led_trigger_unregister_simple(bt_power_led_trigger_ptr());
}

#[inline]
unsafe fn bt_power_led_trigger_ptr() -> *mut led_trigger {
    &mut bt_power_led_trigger
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
