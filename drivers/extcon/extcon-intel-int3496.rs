// SPDX-License-Identifier: GPL-2.0
/*
 * Intel INT3496 ACPI device extcon driver
 *
 * Copyright (c) 2016 Hans de Goede <hdegoede@redhat.com>
 *
 * Based on android x86 kernel code which is:
 *
 * Copyright (c) 2014, Intel Corporation.
 * Author: David Cohen <david.a.cohen@linux.intel.com>
 */

// Linux kernel dependencies supplied by other translation units.

const INT3496_GPIO_USB_ID: u32 = 0;
const INT3496_GPIO_VBUS_EN: u32 = 1;
const INT3496_GPIO_USB_MUX: u32 = 2;
const DEBOUNCE_TIME: u64 = msecs_to_jiffies(50);

#[repr(C)]
struct int3496_data {
    dev: *mut device,
    edev: *mut extcon_dev,
    work: delayed_work,
    gpio_usb_id: *mut gpio_desc,
    gpio_vbus_en: *mut gpio_desc,
    gpio_usb_mux: *mut gpio_desc,
    vbus_boost: *mut regulator,
    usb_id_irq: i32,
    vbus_boost_enabled: bool,
}

static int3496_cable: [u32; 2] = [EXTCON_USB_HOST, EXTCON_NONE];

static id_gpios: acpi_gpio_params = acpi_gpio_params { arg: INT3496_GPIO_USB_ID, index: 0, active_low: false };
static vbus_gpios: acpi_gpio_params = acpi_gpio_params { arg: INT3496_GPIO_VBUS_EN, index: 0, active_low: false };
static mux_gpios: acpi_gpio_params = acpi_gpio_params { arg: INT3496_GPIO_USB_MUX, index: 0, active_low: false };

static acpi_int3496_default_gpios: [acpi_gpio_mapping; 4] = [
    acpi_gpio_mapping { name: "id-gpios", data: &id_gpios, size: 1, quirks: ACPI_GPIO_QUIRK_NO_IO_RESTRICTION },
    acpi_gpio_mapping { name: "vbus-gpios", data: &vbus_gpios, size: 1, quirks: 0 },
    acpi_gpio_mapping { name: "mux-gpios", data: &mux_gpios, size: 1, quirks: 0 },
    acpi_gpio_mapping { name: core::ptr::null(), data: core::ptr::null(), size: 0, quirks: 0 },
];

unsafe fn int3496_set_vbus_boost(data: *mut int3496_data, enable: bool) {
    let mut ret: i32;
    if IS_ERR_OR_NULL((*data).vbus_boost) { return; }
    if (*data).vbus_boost_enabled == enable { return; }
    if enable { ret = regulator_enable((*data).vbus_boost); }
    else { ret = regulator_disable((*data).vbus_boost); }
    if ret == 0 { (*data).vbus_boost_enabled = enable; }
    else { dev_err((*data).dev, "Error updating Vbus boost regulator: %d\n", ret); }
}

unsafe extern "C" fn int3496_do_usb_id(work: *mut work_struct) {
    let data = container_of!(work, int3496_data, work.work);
    let id: i32 = gpiod_get_value_cansleep((*data).gpio_usb_id);
    // id == 1: PERIPHERAL, id == 0: HOST
    dev_dbg((*data).dev, "Connected %s cable\n", if id != 0 { "PERIPHERAL" } else { "HOST" });
    // Peripheral: set USB mux to peripheral and disable VBUS
    // Host: set USB mux to host and enable VBUS
    if !IS_ERR((*data).gpio_usb_mux) { gpiod_direction_output((*data).gpio_usb_mux, id); }
    if !IS_ERR((*data).gpio_vbus_en) { gpiod_direction_output((*data).gpio_vbus_en, !id); }
    else { int3496_set_vbus_boost(data, !id); }
    extcon_set_state_sync((*data).edev, EXTCON_USB_HOST, !id);
}

unsafe extern "C" fn int3496_thread_isr(_irq: i32, priv_: *mut core::ffi::c_void) -> irqreturn_t {
    let data = priv_ as *mut int3496_data;
    mod_delayed_work(system_percpu_wq, &mut (*data).work, DEBOUNCE_TIME);
    IRQ_HANDLED
}

unsafe extern "C" fn int3496_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let mut data: *mut int3496_data;
    let mut ret: i32;
    if has_acpi_companion(dev) {
        ret = devm_acpi_dev_add_driver_gpios(dev, acpi_int3496_default_gpios.as_ptr());
        if ret != 0 { dev_err(dev, "can't add GPIO ACPI mapping\n"); return ret; }
    }
    data = devm_kzalloc(dev, core::mem::size_of::<int3496_data>(), GFP_KERNEL) as *mut int3496_data;
    if data.is_null() { return -ENOMEM; }
    (*data).dev = dev;
    ret = devm_delayed_work_autocancel(dev, &mut (*data).work, int3496_do_usb_id);
    if ret != 0 { return ret; }
    (*data).gpio_usb_id = devm_gpiod_get(dev, "id", GPIOD_IN | GPIOD_FLAGS_BIT_NONEXCLUSIVE);
    if IS_ERR((*data).gpio_usb_id) { ret = PTR_ERR((*data).gpio_usb_id); dev_err(dev, "can't request USB ID GPIO: %d\n", ret); return ret; }
    (*data).usb_id_irq = gpiod_to_irq((*data).gpio_usb_id);
    if (*data).usb_id_irq < 0 { dev_err(dev, "can't get USB ID IRQ: %d\n", (*data).usb_id_irq); return (*data).usb_id_irq; }
    (*data).gpio_vbus_en = devm_gpiod_get(dev, "vbus", GPIOD_ASIS);
    if IS_ERR((*data).gpio_vbus_en) { dev_dbg(dev, "can't request VBUS EN GPIO\n"); (*data).vbus_boost = devm_regulator_get_optional(dev, "vbus"); }
    (*data).gpio_usb_mux = devm_gpiod_get(dev, "mux", GPIOD_ASIS);
    if IS_ERR((*data).gpio_usb_mux) { dev_dbg(dev, "can't request USB MUX GPIO\n"); }
    (*data).edev = devm_extcon_dev_allocate(dev, int3496_cable.as_ptr());
    if IS_ERR((*data).edev) { return -ENOMEM; }
    ret = devm_extcon_dev_register(dev, (*data).edev);
    if ret < 0 { dev_err(dev, "can't register extcon device: %d\n", ret); return ret; }
    ret = devm_request_threaded_irq(dev, (*data).usb_id_irq, None, int3496_thread_isr, IRQF_SHARED | IRQF_ONESHOT | IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING, dev_name(dev), data as *mut core::ffi::c_void);
    if ret < 0 { dev_err(dev, "can't request IRQ for USB ID GPIO: %d\n", ret); return ret; }
    queue_delayed_work(system_percpu_wq, &mut (*data).work, 0);
    flush_delayed_work(&mut (*data).work);
    platform_set_drvdata(pdev, data as *mut core::ffi::c_void);
    0
}

static int3496_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: "INT3496" }, acpi_device_id { id: "" },
];
static int3496_ids: [platform_device_id; 2] = [
    platform_device_id { name: "intel-int3496" }, platform_device_id { name: "" },
];
static mut int3496_driver: platform_driver = platform_driver {
    driver: driver { name: "intel-int3496", acpi_match_table: int3496_acpi_match.as_ptr() },
    probe: Some(int3496_probe), id_table: int3496_ids.as_ptr(),
};

module_platform_driver!(int3496_driver);
// MODULE_DEVICE_TABLE(acpi, int3496_acpi_match);
// MODULE_DEVICE_TABLE(platform, int3496_ids);
// MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
// MODULE_DESCRIPTION("Intel INT3496 ACPI device extcon driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
