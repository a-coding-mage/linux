// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2011, NVIDIA Corporation.
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct rfkill_gpio_data {
    pub name: *const core::ffi::c_char,
    pub r#type: rfkill_type,
    pub reset_gpio: *mut gpio_desc,
    pub shutdown_gpio: *mut gpio_desc,
    pub rfkill_dev: *mut rfkill,
    pub clk: *mut clk,
    pub clk_enabled: bool,
}

extern "C" {
    fn clk_enable(clk: *mut clk) -> i32;
    fn clk_disable(clk: *mut clk);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: bool);
    fn gpiod_direction_output(desc: *mut gpio_desc, value: bool) -> i32;
    fn devm_acpi_dev_add_driver_gpios(dev: *mut device, mapping: *const acpi_gpio_mapping) -> i32;
    fn acpi_match_device(table: *const acpi_device_id, dev: *mut device) -> *const acpi_device_id;
    fn dmi_check_system(table: *const dmi_system_id) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_of_node(dev: *mut device) -> bool;
    fn device_property_read_string(dev: *mut device, name: *const core::ffi::c_char,
                                   value: *mut *const core::ffi::c_char) -> i32;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn rfkill_find_type(name: *const core::ffi::c_char) -> rfkill_type;
    fn devm_clk_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut clk;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const core::ffi::c_char,
                               flags: u32) -> *mut gpio_desc;
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *const core::ffi::c_void) -> i32;
    fn device_property_present(dev: *mut device, name: *const core::ffi::c_char) -> bool;
    fn rfkill_init_sw_state(dev: *mut rfkill, blocked: bool);
    fn rfkill_alloc(name: *const core::ffi::c_char, dev: *mut device, r#type: rfkill_type,
                    ops: *const rfkill_ops, data: *mut core::ffi::c_void) -> *mut rfkill;
    fn rfkill_register(dev: *mut rfkill) -> i32;
    fn rfkill_unregister(dev: *mut rfkill);
    fn rfkill_destroy(dev: *mut rfkill);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut rfkill_gpio_data;
}

#[repr(C)] pub struct gpio_desc;
#[repr(C)] pub struct rfkill;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct acpi_device_id { pub driver_data: usize }
#[repr(C)] pub struct dmi_system_id;
#[repr(C)] pub struct acpi_gpio_mapping;
#[repr(C)] pub struct rfkill_ops { pub set_block: Option<unsafe extern "C" fn(*mut core::ffi::c_void, bool) -> i32> }
pub type rfkill_type = u32;

unsafe extern "C" fn rfkill_gpio_set_power(data: *mut core::ffi::c_void, blocked: bool) -> i32 {
    let rfkill = data as *mut rfkill_gpio_data;
    if !blocked && !is_err((*rfkill).clk as *const _) && !(*rfkill).clk_enabled {
        let ret = clk_enable((*rfkill).clk);
        if ret != 0 { return ret; }
    }
    gpiod_set_value_cansleep((*rfkill).shutdown_gpio, !blocked);
    gpiod_set_value_cansleep((*rfkill).reset_gpio, !blocked);
    if blocked && !is_err((*rfkill).clk as *const _) && (*rfkill).clk_enabled {
        clk_disable((*rfkill).clk);
    }
    (*rfkill).clk_enabled = !blocked;
    0
}

static RFKILL_GPIO_OPS: rfkill_ops = rfkill_ops { set_block: Some(rfkill_gpio_set_power) };

unsafe extern "C" fn rfkill_gpio_acpi_probe(dev: *mut device, rfkill: *mut rfkill_gpio_data) -> i32 {
    let id = acpi_match_device(core::ptr::null(), dev);
    if id.is_null() { return -19; }
    (*rfkill).r#type = (*id).driver_data as rfkill_type;
    devm_acpi_dev_add_driver_gpios(dev, core::ptr::null())
}

unsafe extern "C" fn rfkill_gpio_probe(pdev: *mut platform_device) -> i32 {
    let rfkill = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<rfkill_gpio_data>(), 0) as *mut rfkill_gpio_data;
    if rfkill.is_null() { return -12; }
    let mut type_name: *const core::ffi::c_char = core::ptr::null();
    let (name_property, type_property) = if dev_of_node(&mut (*pdev).dev) {
        (c"label".as_ptr(), c"radio-type".as_ptr())
    } else { (c"name".as_ptr(), c"type".as_ptr()) };
    device_property_read_string(&mut (*pdev).dev, name_property, &mut (*rfkill).name);
    device_property_read_string(&mut (*pdev).dev, type_property, &mut type_name);
    if (*rfkill).name.is_null() { (*rfkill).name = dev_name(&mut (*pdev).dev); }
    (*rfkill).r#type = rfkill_find_type(type_name);
    if !acpi_match_device(core::ptr::null(), &mut (*pdev).dev).is_null() {
        let ret = rfkill_gpio_acpi_probe(&mut (*pdev).dev, rfkill);
        if ret != 0 { return ret; }
    }
    (*rfkill).clk = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
    (*rfkill).reset_gpio = devm_gpiod_get_optional(&mut (*pdev).dev, c"reset".as_ptr(), 0);
    if is_err((*rfkill).reset_gpio as *const _) { return ptr_err((*rfkill).reset_gpio as *const _); }
    (*rfkill).shutdown_gpio = devm_gpiod_get_optional(&mut (*pdev).dev, c"shutdown".as_ptr(), 0);
    if is_err((*rfkill).shutdown_gpio as *const _) { return ptr_err((*rfkill).shutdown_gpio as *const _); }
    if (*rfkill).reset_gpio.is_null() && (*rfkill).shutdown_gpio.is_null() { return -22; }
    let mut ret = gpiod_direction_output((*rfkill).reset_gpio, true);
    if ret != 0 { return ret; }
    ret = gpiod_direction_output((*rfkill).shutdown_gpio, true);
    if ret != 0 { return ret; }
    (*rfkill).rfkill_dev = rfkill_alloc((*rfkill).name, &mut (*pdev).dev, (*rfkill).r#type, &RFKILL_GPIO_OPS, rfkill as *mut _);
    if (*rfkill).rfkill_dev.is_null() { return -12; }
    if device_property_present(&mut (*pdev).dev, c"default-blocked".as_ptr()) { rfkill_init_sw_state((*rfkill).rfkill_dev, true); }
    ret = rfkill_register((*rfkill).rfkill_dev);
    if ret < 0 { rfkill_destroy((*rfkill).rfkill_dev); return ret; }
    platform_set_drvdata(pdev, rfkill as *mut _);
    0
}

unsafe extern "C" fn rfkill_gpio_remove(pdev: *mut platform_device) {
    let rfkill = platform_get_drvdata(pdev);
    rfkill_unregister((*rfkill).rfkill_dev);
    rfkill_destroy((*rfkill).rfkill_dev);
}

// DMI deny table: Lenovo Yoga Tab 3 Pro YT3-X90, bogus "BCM4752" device in DSDT.
static RFKILL_GPIO_DENY_TABLE: [dmi_system_id; 1] = [dmi_system_id {}];

#[cfg(feature = "acpi")]
static RFKILL_ACPI_MATCH: [acpi_device_id; 3] = [
    acpi_device_id { driver_data:  gps_type() },
    acpi_device_id { driver_data:  gps_type() },
    acpi_device_id { driver_data: 0 },
];

const fn gps_type() -> rfkill_type { 3 }

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }

static RFKILL_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"rfkill-gpio".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    name: *const core::ffi::c_char,
}

static RFKILL_GPIO_DRIVER: platform_driver = platform_driver {
    probe: Some(rfkill_gpio_probe),
    remove: Some(rfkill_gpio_remove),
    name: c"rfkill_gpio".as_ptr(),
};

// module_platform_driver(rfkill_gpio_driver);
// MODULE_DESCRIPTION("gpio rfkill");
// MODULE_AUTHOR("NVIDIA");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
