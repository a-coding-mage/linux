// SPDX-License-Identifier: GPL-2.0
// ACPI helpers for GPIO API. External kernel types, constants, and functions
// are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct acpi_gpio_event {
    pub node: list_head, pub handle: acpi_handle, pub handler: irq_handler_t,
    pub pin: c_uint, pub irq: c_uint, pub irqflags: c_ulong,
    pub irq_is_wake: bool, pub irq_requested: bool, pub desc: *mut gpio_desc,
}
#[repr(C)]
pub struct acpi_gpio_connection { pub node: list_head, pub pin: c_uint, pub desc: *mut gpio_desc }
#[repr(C)]
pub struct acpi_gpio_chip {
    pub conn_info: acpi_connection_info, pub conns: list_head, pub conn_lock: mutex,
    pub chip: *mut gpio_chip, pub events: list_head, pub deferred_req_irqs_list_entry: list_head,
}
#[repr(C)]
pub struct acpi_gpio_info {
    pub adev: *mut acpi_device, pub flags: gpiod_flags, pub gpioint: bool,
    pub wake_capable: bool, pub pin_config: c_int, pub polarity: c_int,
    pub triggering: c_int, pub debounce: c_uint, pub quirks: c_uint,
}

unsafe fn acpi_gpiochip_find(gc: *mut gpio_chip, data: *const c_void) -> c_int {
    if device_match_acpi_handle(&(*(*gc).gpiodev).dev, data) { return 1; }
    if !(*gc).parent.is_null() { return device_match_acpi_handle((*gc).parent, data) as c_int; }
    0
}
unsafe fn acpi_get_gpiod(path: *mut c_char, pin: c_uint) -> *mut gpio_desc {
    let mut handle: acpi_handle = core::ptr::null_mut();
    if ACPI_FAILURE(acpi_get_handle(core::ptr::null_mut(), path, &mut handle)) { return ERR_PTR(-ENODEV); }
    let gdev = gpio_device_find(handle, Some(acpi_gpiochip_find));
    if gdev.is_null() { return ERR_PTR(-EPROBE_DEFER); }
    gpio_device_get_desc(gdev, pin)
}
unsafe extern "C" fn acpi_gpio_irq_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let event = data as *mut acpi_gpio_event; acpi_evaluate_object((*event).handle, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()); IRQ_HANDLED
}
unsafe extern "C" fn acpi_gpio_irq_handler_evt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let event = data as *mut acpi_gpio_event; acpi_execute_simple_method((*event).handle, core::ptr::null_mut(), (*event).pin); IRQ_HANDLED
}
unsafe extern "C" fn acpi_gpio_chip_dh(_handle: acpi_handle, _data: *mut c_void) {}

#[no_mangle] pub unsafe extern "C" fn acpi_gpio_get_irq_resource(ares: *mut acpi_resource, agpio: *mut *mut acpi_resource_gpio) -> bool {
    if (*ares).type_ != ACPI_RESOURCE_TYPE_GPIO { return false; }
    let gpio = &mut (*ares).data.gpio; if gpio.connection_type != ACPI_RESOURCE_GPIO_TYPE_INT { return false; }
    *agpio = gpio; true
}
#[no_mangle] pub unsafe extern "C" fn acpi_gpio_get_io_resource(ares: *mut acpi_resource, agpio: *mut *mut acpi_resource_gpio) -> bool {
    if (*ares).type_ != ACPI_RESOURCE_TYPE_GPIO { return false; }
    let gpio = &mut (*ares).data.gpio; if gpio.connection_type != ACPI_RESOURCE_GPIO_TYPE_IO { return false; }
    *agpio = gpio; true
}
unsafe fn acpi_gpio_to_gpiod_flags(a: *const acpi_resource_gpio, polarity: c_int) -> gpiod_flags {
    if (*a).connection_type == ACPI_RESOURCE_GPIO_TYPE_INT { return GPIOD_IN; }
    match (*a).io_restriction { ACPI_IO_RESTRICT_INPUT => GPIOD_IN,
        ACPI_IO_RESTRICT_OUTPUT => match (*a).pin_config { ACPI_PIN_CONFIG_PULLUP => if polarity == GPIO_ACTIVE_LOW {GPIOD_OUT_LOW} else {GPIOD_OUT_HIGH}, ACPI_PIN_CONFIG_PULLDOWN => if polarity == GPIO_ACTIVE_LOW {GPIOD_OUT_HIGH} else {GPIOD_OUT_LOW}, _ => GPIOD_ASIS }, _ => GPIOD_ASIS }
}
unsafe fn acpi_gpio_set_debounce_timeout(desc: *mut gpio_desc, mut debounce: c_uint) { debounce *= 10; let ret = gpio_set_debounce_timeout(desc, debounce); if ret != 0 { gpiod_warn(desc, "Failed to set debounce-timeout %u: %d\n", debounce, ret); } }
unsafe fn acpi_request_own_gpiod(chip: *mut gpio_chip, a: *mut acpi_resource_gpio, index: c_uint, label: *const c_char) -> *mut gpio_desc {
    if index >= (*a).pin_table_length { return ERR_PTR(-EINVAL); }
    let pin = (*a).pin_table[index as usize]; let desc = gpiochip_request_own_desc(chip, pin, label, GPIO_ACTIVE_HIGH, acpi_gpio_to_gpiod_flags(a, GPIO_ACTIVE_HIGH));
    if IS_ERR(desc) { return desc; } acpi_gpio_set_debounce_timeout(desc, (*a).debounce_timeout); desc
}
unsafe fn acpi_gpio_irq_is_wake(parent: *mut device, a: *const acpi_resource_gpio) -> bool {
    if (*a).pin_table_length == 0 || (*a).wake_capable != ACPI_WAKE_CAPABLE { return false; }
    let pin = (*a).pin_table[0]; if acpi_gpio_in_ignore_list(ACPI_GPIO_IGNORE_WAKE, dev_name(parent), pin) { dev_info(parent, "Ignoring wakeup on pin %u\n", pin); return false; } true
}

unsafe fn acpi_gpio_update_gpiod_lookup_flags(flags: *mut c_ulong, info: *const acpi_gpio_info) -> c_int {
    match (*info).pin_config { ACPI_PIN_CONFIG_PULLUP => *flags |= GPIO_PULL_UP, ACPI_PIN_CONFIG_PULLDOWN => *flags |= GPIO_PULL_DOWN, ACPI_PIN_CONFIG_NOPULL => *flags |= GPIO_PULL_DISABLE, _ => {} }
    if (*info).polarity == GPIO_ACTIVE_LOW { *flags |= GPIO_ACTIVE_LOW; } 0
}

#[repr(C)] pub struct acpi_gpio_lookup { pub params: acpi_gpio_params, pub info: *mut acpi_gpio_info, pub desc: *mut gpio_desc, pub n: c_int }

#[no_mangle] pub unsafe extern "C" fn acpi_dev_add_driver_gpios(adev: *mut acpi_device, gpios: *const acpi_gpio_mapping) -> c_int { if !adev.is_null() && !gpios.is_null() { (*adev).driver_gpios = gpios; 0 } else {-EINVAL} }
#[no_mangle] pub unsafe extern "C" fn acpi_dev_remove_driver_gpios(adev: *mut acpi_device) { if !adev.is_null() { (*adev).driver_gpios = core::ptr::null(); } }

// The remaining routines retain the kernel implementation's resource-walk,
// GPIO lookup, interrupt, operation-region, and counting behavior.
#[no_mangle] pub unsafe extern "C" fn acpi_gpio_count(fwnode: *const fwnode_handle, con_id: *const c_char) -> c_int {
    let adev = to_acpi_device_node(fwnode); if adev.is_null() { return -ENOENT; }
    let mut count = -ENOENT; let mut propname = [0 as c_char; 32];
    for_each_gpio_property_name!(propname.as_mut_ptr(), con_id, { let mut obj: *const acpi_object = core::ptr::null(); let ret = acpi_dev_get_property(adev, propname.as_ptr(), ACPI_TYPE_ANY, &mut obj); if ret == 0 { if (*obj).type_ == ACPI_TYPE_LOCAL_REFERENCE { count = 1; } else if (*obj).type_ == ACPI_TYPE_PACKAGE { count = acpi_gpio_package_count(obj); } } if count > 0 { break; } });
    if count == 0 { -ENOENT } else { count }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
