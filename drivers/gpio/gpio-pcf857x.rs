// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for pcf857x, pca857x, and pca967x I2C GPIO expanders */

// Kernel dependencies supplied externally.

#[repr(C)]
struct Pcf857x {
    chip: gpio_chip,
    client: *mut i2c_client,
    lock: mutex,
    out: c_uint,
    status: c_uint,
    irq_enabled: c_uint,
    write: Option<unsafe extern "C" fn(*mut i2c_client, c_uint) -> c_int>,
    read: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

static PCF857X_ID: [i2c_device_id; 14] = [
    i2c_device_id { name: "pcf8574", driver_data: 8 },
    i2c_device_id { name: "pcf8574a", driver_data: 8 },
    i2c_device_id { name: "pca8574", driver_data: 8 },
    i2c_device_id { name: "pca9670", driver_data: 8 },
    i2c_device_id { name: "pca9672", driver_data: 8 },
    i2c_device_id { name: "pca9674", driver_data: 8 },
    i2c_device_id { name: "pcf8575", driver_data: 16 },
    i2c_device_id { name: "pca8575", driver_data: 16 },
    i2c_device_id { name: "pca9671", driver_data: 16 },
    i2c_device_id { name: "pca9673", driver_data: 16 },
    i2c_device_id { name: "pca9675", driver_data: 16 },
    i2c_device_id { name: "max7328", driver_data: 8 },
    i2c_device_id { name: "max7329", driver_data: 8 },
    i2c_device_id { name: "", driver_data: 0 },
];

unsafe extern "C" fn i2c_write_le8(client: *mut i2c_client, data: c_uint) -> c_int {
    i2c_smbus_write_byte(client, data)
}
unsafe extern "C" fn i2c_read_le8(client: *mut i2c_client) -> c_int {
    i2c_smbus_read_byte(client)
}
unsafe extern "C" fn i2c_write_le16(client: *mut i2c_client, word: c_uint) -> c_int {
    let buf: u16 = (word as u16).to_le();
    let status = i2c_master_send(client, &buf as *const _ as *const c_char, 2);
    if status < 0 { status } else { 0 }
}
unsafe extern "C" fn i2c_read_le16(client: *mut i2c_client) -> c_int {
    let mut buf: u16 = 0;
    let status = i2c_master_recv(client, &mut buf as *mut _ as *mut c_char, 2);
    if status < 0 { return status; }
    u16::from_le(buf) as c_int
}

unsafe extern "C" fn pcf857x_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let gpio = gpiochip_get_data(chip) as *mut Pcf857x;
    mutex_lock(&mut (*gpio).lock);
    (*gpio).out |= 1u32.wrapping_shl(offset);
    let status = ((*gpio).write.unwrap())((*gpio).client, (*gpio).out);
    mutex_unlock(&mut (*gpio).lock);
    status
}
unsafe extern "C" fn pcf857x_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let gpio = gpiochip_get_data(chip) as *mut Pcf857x;
    let value = ((*gpio).read.unwrap())((*gpio).client);
    if value < 0 { value } else { ((value as u32 & 1u32.wrapping_shl(offset)) != 0) as c_int }
}
unsafe extern "C" fn pcf857x_get_multiple(chip: *mut gpio_chip, mask: *mut c_ulong, bits: *mut c_ulong) -> c_int {
    let gpio = gpiochip_get_data(chip) as *mut Pcf857x;
    let value = ((*gpio).read.unwrap())((*gpio).client);
    if value < 0 { return value; }
    *bits &= !*mask;
    *bits |= value as c_ulong & *mask;
    0
}
unsafe extern "C" fn pcf857x_output(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let gpio = gpiochip_get_data(chip) as *mut Pcf857x;
    let bit = 1u32.wrapping_shl(offset);
    mutex_lock(&mut (*gpio).lock);
    if value != 0 { (*gpio).out |= bit; } else { (*gpio).out &= !bit; }
    let status = ((*gpio).write.unwrap())((*gpio).client, (*gpio).out);
    mutex_unlock(&mut (*gpio).lock);
    status
}
unsafe extern "C" fn pcf857x_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int { pcf857x_output(chip, offset, value) }
unsafe extern "C" fn pcf857x_set_multiple(chip: *mut gpio_chip, mask: *mut c_ulong, bits: *mut c_ulong) -> c_int {
    let gpio = gpiochip_get_data(chip) as *mut Pcf857x;
    mutex_lock(&mut (*gpio).lock);
    (*gpio).out &= !(*mask as c_uint);
    (*gpio).out |= *bits as c_uint & *mask as c_uint;
    let status = ((*gpio).write.unwrap())((*gpio).client, (*gpio).out);
    mutex_unlock(&mut (*gpio).lock);
    status
}

unsafe extern "C" fn noop(_data: *mut irq_data) {}
unsafe extern "C" fn pcf857x_irq_set_wake(data: *mut irq_data, on: c_uint) -> c_int {
    let gpio = irq_data_get_irq_chip_data(data) as *mut Pcf857x;
    irq_set_irq_wake((*gpio).client.as_ref().unwrap().irq, on)
}
unsafe extern "C" fn pcf857x_irq_enable(data: *mut irq_data) {
    let gpio = irq_data_get_irq_chip_data(data) as *mut Pcf857x;
    let hwirq = irqd_to_hwirq(data);
    gpiochip_enable_irq(&mut (*gpio).chip, hwirq);
    (*gpio).irq_enabled |= 1u32.wrapping_shl(hwirq as u32);
}
unsafe extern "C" fn pcf857x_irq_disable(data: *mut irq_data) {
    let gpio = irq_data_get_irq_chip_data(data) as *mut Pcf857x;
    let hwirq = irqd_to_hwirq(data);
    (*gpio).irq_enabled &= !1u32.wrapping_shl(hwirq as u32);
    gpiochip_disable_irq(&mut (*gpio).chip, hwirq);
}
unsafe extern "C" fn pcf857x_irq_bus_lock(data: *mut irq_data) { let gpio = irq_data_get_irq_chip_data(data) as *mut Pcf857x; mutex_lock(&mut (*gpio).lock); }
unsafe extern "C" fn pcf857x_irq_bus_sync_unlock(data: *mut irq_data) { let gpio = irq_data_get_irq_chip_data(data) as *mut Pcf857x; mutex_unlock(&mut (*gpio).lock); }

unsafe extern "C" fn pcf857x_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let gpio = data as *mut Pcf857x;
    let status = ((*gpio).read.unwrap())((*gpio).client) as c_ulong;
    mutex_lock(&mut (*gpio).lock);
    let change = ((*gpio).status as c_ulong ^ status) & (*gpio).irq_enabled as c_ulong;
    (*gpio).status = status as c_uint;
    mutex_unlock(&mut (*gpio).lock);
    let mut i = 0;
    while i < (*gpio).chip.ngpio {
        if change & (1 as c_ulong).wrapping_shl(i) != 0 {
            handle_nested_irq(irq_find_mapping((*gpio).chip.irq.domain, i));
        }
        i += 1;
    }
    IRQ_HANDLED
}

unsafe extern "C" fn pcf857x_probe(client: *mut i2c_client) -> c_int {
    let gpio = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<Pcf857x>(), GFP_KERNEL) as *mut Pcf857x;
    if gpio.is_null() { return -ENOMEM; }
    mutex_init(&mut (*gpio).lock);
    (*gpio).chip.base = -1;
    (*gpio).chip.can_sleep = true;
    (*gpio).chip.parent = &mut (*client).dev;
    (*gpio).chip.owner = THIS_MODULE;
    (*gpio).chip.get = Some(pcf857x_get);
    (*gpio).chip.get_multiple = Some(pcf857x_get_multiple);
    (*gpio).chip.set = Some(pcf857x_set);
    (*gpio).chip.set_multiple = Some(pcf857x_set_multiple);
    (*gpio).chip.direction_input = Some(pcf857x_input);
    (*gpio).chip.direction_output = Some(pcf857x_output);
    (*gpio).chip.ngpio = i2c_get_match_data(client) as usize;
    if (*gpio).chip.ngpio == 8 {
        (*gpio).write = Some(i2c_write_le8); (*gpio).read = Some(i2c_read_le8);
        if !i2c_check_functionality((*client).adapter, I2C_FUNC_SMBUS_BYTE) { return -EIO; }
        if i2c_smbus_read_byte(client) < 0 { return -EIO; }
    } else if (*gpio).chip.ngpio == 16 {
        (*gpio).write = Some(i2c_write_le16); (*gpio).read = Some(i2c_read_le16);
        if !i2c_check_functionality((*client).adapter, I2C_FUNC_I2C) { return -EIO; }
        if i2c_read_le16(client) < 0 { return -EIO; }
    } else { return -EINVAL; }
    (*gpio).client = client;
    i2c_set_clientdata(client, gpio as *mut c_void);
    (*gpio).out = !0;
    (*gpio).status = ((*gpio).read.unwrap())((*gpio).client) as c_uint;
    devm_gpiochip_add_data(&mut (*client).dev, &mut (*gpio).chip, gpio as *mut c_void)
}
unsafe extern "C" fn pcf857x_shutdown(client: *mut i2c_client) {
    let gpio = i2c_get_clientdata(client) as *mut Pcf857x;
    ((*gpio).write.unwrap())((*gpio).client, (1u32 << (*gpio).chip.ngpio) - 1);
}

// Driver registration and module metadata correspond to the C i2c_driver/module macros.
static mut PCF857X_DRIVER: i2c_driver = i2c_driver { _private: 0 };
unsafe extern "C" fn pcf857x_init() -> c_int { i2c_add_driver(&mut PCF857X_DRIVER) }
unsafe extern "C" fn pcf857x_exit() { i2c_del_driver(&mut PCF857X_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
