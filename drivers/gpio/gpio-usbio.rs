// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2025 Intel Corporation.
 * Copyright (c) 2025 Red Hat, Inc.
 */

#[repr(C)]
struct usbio_gpio_bank {
    config: [u8; USBIO_GPIOSPERBANK as usize],
    bitmap: u32,
}

#[repr(C)]
struct usbio_gpio {
    config_mutex: mutex, /* Protects banks[x].config */
    banks: [usbio_gpio_bank; USBIO_MAX_GPIOBANKS as usize],
    gc: gpio_chip,
    adev: *mut auxiliary_device,
}

static usbio_gpio_acpi_hids: [acpi_device_id; 7] = [
    acpi_device_id { id: *b"INTC1007\0", driver_data: 0 }, /* MTL */
    acpi_device_id { id: *b"INTC10B2\0", driver_data: 0 }, /* ARL */
    acpi_device_id { id: *b"INTC10B5\0", driver_data: 0 }, /* LNL */
    acpi_device_id { id: *b"INTC10D1\0", driver_data: 0 }, /* MTL-CVF */
    acpi_device_id { id: *b"INTC10E2\0", driver_data: 0 }, /* PTL */
    acpi_device_id { id: *b"INTC1116\0", driver_data: 0 }, /* NVL */
    acpi_device_id { id: [0; 9], driver_data: 0 },
];

unsafe fn usbio_gpio_get_bank_and_pin(
    gc: *mut gpio_chip,
    offset: c_uint,
    bank_ret: *mut *mut usbio_gpio_bank,
    pin_ret: *mut c_uint,
) {
    let gpio = gpiochip_get_data(gc) as *mut usbio_gpio;
    let dev = &mut (*(*gpio).adev).dev as *mut device;
    let bank = &mut (*gpio).banks[(offset / USBIO_GPIOSPERBANK) as usize] as *mut usbio_gpio_bank;
    let pin = offset % USBIO_GPIOSPERBANK;

    if !((*bank).bitmap & BIT(pin)) != 0 {
        /* The FW bitmap sometimes is invalid, warn and continue */
        dev_warn_once(dev, FW_BUG "GPIO %u is not in FW pins bitmap\n", offset);
    }

    *bank_ret = bank;
    *pin_ret = pin;
}

unsafe extern "C" fn usbio_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    let mut bank: *mut usbio_gpio_bank = core::ptr::null_mut();
    let mut pin = 0;
    usbio_gpio_get_bank_and_pin(gc, offset, &mut bank, &mut pin);
    let cfg = (*bank).config[pin as usize] & USBIO_GPIO_PINMOD_MASK;
    if cfg == USBIO_GPIO_PINMOD_OUTPUT { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe extern "C" fn usbio_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    let gpio = gpiochip_get_data(gc) as *mut usbio_gpio;
    let mut bank: *mut usbio_gpio_bank = core::ptr::null_mut();
    let mut pin = 0;
    let mut gbuf: usbio_gpio_rw = core::mem::zeroed();
    usbio_gpio_get_bank_and_pin(gc, offset, &mut bank, &mut pin);
    gbuf.bankid = offset / USBIO_GPIOSPERBANK;
    gbuf.pincount = 1;
    gbuf.pin = pin;
    let ret = usbio_control_msg((*gpio).adev, USBIO_PKTTYPE_GPIO, USBIO_GPIOCMD_READ,
        &mut gbuf as *mut _ as *mut _, core::mem::size_of::<usbio_gpio_rw>() - core::mem::size_of::<u32>(),
        &mut gbuf as *mut _ as *mut _, core::mem::size_of::<usbio_gpio_rw>());
    if ret != core::mem::size_of::<usbio_gpio_rw>() as c_int { return if ret < 0 { ret } else { -EPROTO }; }
    ((le32_to_cpu(gbuf.value) >> pin) & 1) as c_int
}

unsafe extern "C" fn usbio_gpio_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let gpio = gpiochip_get_data(gc) as *mut usbio_gpio;
    let mut bank: *mut usbio_gpio_bank = core::ptr::null_mut();
    let mut pin = 0;
    let mut gbuf: usbio_gpio_rw = core::mem::zeroed();
    usbio_gpio_get_bank_and_pin(gc, offset, &mut bank, &mut pin);
    gbuf.bankid = offset / USBIO_GPIOSPERBANK;
    gbuf.pincount = 1;
    gbuf.pin = pin;
    gbuf.value = cpu_to_le32((value << pin) as u32);
    usbio_control_msg((*gpio).adev, USBIO_PKTTYPE_GPIO, USBIO_GPIOCMD_WRITE,
        &mut gbuf as *mut _ as *mut _, core::mem::size_of::<usbio_gpio_rw>(), core::ptr::null_mut(), 0)
}

/* The remaining callback and driver declarations retain the same control flow and ABI. */
unsafe extern "C" fn usbio_gpio_direction_input(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    usbio_gpio_update_config(gc, offset, USBIO_GPIO_PINMOD_MASK, USBIO_GPIO_SET_PINMOD(USBIO_GPIO_PINMOD_INPUT))
}

unsafe extern "C" fn usbio_gpio_direction_output(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let ret = usbio_gpio_update_config(gc, offset, USBIO_GPIO_PINMOD_MASK, USBIO_GPIO_SET_PINMOD(USBIO_GPIO_PINMOD_OUTPUT));
    if ret != 0 { return ret; }
    usbio_gpio_set(gc, offset, value)
}

unsafe extern "C" fn usbio_gpio_update_config(gc: *mut gpio_chip, offset: c_uint, mask: u8, value: u8) -> c_int {
    let gpio = gpiochip_get_data(gc) as *mut usbio_gpio;
    let mut bank: *mut usbio_gpio_bank = core::ptr::null_mut();
    let mut pin = 0;
    let mut gbuf: usbio_gpio_init = core::mem::zeroed();
    usbio_gpio_get_bank_and_pin(gc, offset, &mut bank, &mut pin);
    let _guard = mutex_guard(&mut (*gpio).config_mutex);
    (*bank).config[pin as usize] &= !mask;
    (*bank).config[pin as usize] |= value;
    gbuf.bankid = offset / USBIO_GPIOSPERBANK;
    gbuf.config = (*bank).config[pin as usize];
    gbuf.pincount = 1;
    gbuf.pin = pin;
    usbio_control_msg((*gpio).adev, USBIO_PKTTYPE_GPIO, USBIO_GPIOCMD_INIT,
        &mut gbuf as *mut _ as *mut _, core::mem::size_of::<usbio_gpio_init>(), core::ptr::null_mut(), 0)
}

unsafe extern "C" fn usbio_gpio_set_config(gc: *mut gpio_chip, offset: c_uint, config: c_ulong) -> c_int {
    let value = match pinconf_to_config_param(config) {
        PIN_CONFIG_BIAS_PULL_PIN_DEFAULT => USBIO_GPIO_SET_PINCFG(USBIO_GPIO_PINCFG_DEFAULT),
        PIN_CONFIG_BIAS_PULL_UP => USBIO_GPIO_SET_PINCFG(USBIO_GPIO_PINCFG_PULLUP),
        PIN_CONFIG_BIAS_PULL_DOWN => USBIO_GPIO_SET_PINCFG(USBIO_GPIO_PINCFG_PULLDOWN),
        PIN_CONFIG_DRIVE_PUSH_PULL => USBIO_GPIO_SET_PINCFG(USBIO_GPIO_PINCFG_PUSHPULL),
        _ => return -ENOTSUPP,
    };
    usbio_gpio_update_config(gc, offset, USBIO_GPIO_PINCFG_MASK, value)
}

unsafe extern "C" fn usbio_gpio_probe(adev: *mut auxiliary_device, _adev_id: *const auxiliary_device_id) -> c_int {
    let dev = &mut (*adev).dev as *mut device;
    let bank_desc = dev_get_platdata(dev) as *mut usbio_gpio_bank_desc;
    if bank_desc.is_null() { return -EINVAL; }
    let gpio = devm_kzalloc(dev, core::mem::size_of::<usbio_gpio>(), GFP_KERNEL) as *mut usbio_gpio;
    if gpio.is_null() { return -ENOMEM; }
    let ret = devm_mutex_init(dev, &mut (*gpio).config_mutex);
    if ret != 0 { return ret; }
    (*gpio).adev = adev;
    usbio_acpi_bind(adev, usbio_gpio_acpi_hids.as_ptr());
    let mut bank = 0;
    while bank < USBIO_MAX_GPIOBANKS as c_int && (*bank_desc.add(bank as usize)).bmap != 0 {
        (*gpio).banks[bank as usize].bitmap = le32_to_cpu((*bank_desc.add(bank as usize)).bmap);
        bank += 1;
    }
    (*gpio).gc.label = if !ACPI_COMPANION(dev).is_null() { acpi_dev_name(ACPI_COMPANION(dev)) } else { dev_name(dev) };
    (*gpio).gc.parent = dev;
    (*gpio).gc.owner = THIS_MODULE;
    (*gpio).gc.get_direction = Some(usbio_gpio_get_direction);
    (*gpio).gc.direction_input = Some(usbio_gpio_direction_input);
    (*gpio).gc.direction_output = Some(usbio_gpio_direction_output);
    (*gpio).gc.get = Some(usbio_gpio_get);
    (*gpio).gc.set = Some(usbio_gpio_set);
    (*gpio).gc.set_config = Some(usbio_gpio_set_config);
    (*gpio).gc.base = -1;
    (*gpio).gc.ngpio = bank as c_uint * USBIO_GPIOSPERBANK;
    (*gpio).gc.can_sleep = true;
    let ret = devm_gpiochip_add_data(dev, &mut (*gpio).gc, gpio as *mut _);
    if ret != 0 { return ret; }
    if has_acpi_companion(dev) { acpi_dev_clear_dependencies(ACPI_COMPANION(dev)); }
    0
}

static usbio_gpio_driver: auxiliary_driver = auxiliary_driver {
    name: USBIO_GPIO_CLIENT,
    probe: Some(usbio_gpio_probe),
    id_table: usbio_gpio_id_table.as_ptr(),
};

static usbio_gpio_id_table: [auxiliary_device_id; 2] = [
    auxiliary_device_id { name: *b"usbio.usbio-gpio\0" },
    auxiliary_device_id { name: [0; 17] },
];

module_auxiliary_driver!(usbio_gpio_driver);
MODULE_DESCRIPTION!("Intel USBIO GPIO driver");
MODULE_AUTHOR!("Israel Cepeda <israel.a.cepeda.lopez@intel.com>");
MODULE_AUTHOR!("Hans de Goede <hansg@kernel.org>");
MODULE_LICENSE!("GPL");
MODULE_IMPORT_NS!("USBIO");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
