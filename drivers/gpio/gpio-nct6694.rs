// SPDX-License-Identifier: GPL-2.0
/*
 * Nuvoton NCT6694 GPIO controller driver based on USB interface.
 *
 * Copyright (C) 2025 Nuvoton Technology Corp.
 */

// Linux dependencies from the original implementation are supplied externally.

const NCT6694_GPIO_MOD: u8 = 0xFF;
const NCT6694_GPIO_VER: u16 = 0x90;
const NCT6694_GPIO_VALID: u16 = 0x110;
const NCT6694_GPI_DATA: u16 = 0x120;
const NCT6694_GPO_DIR: u16 = 0x170;
const NCT6694_GPO_TYPE: u16 = 0x180;
const NCT6694_GPO_DATA: u16 = 0x190;
const NCT6694_GPI_STS: u16 = 0x130;
const NCT6694_GPI_CLR: u16 = 0x140;
const NCT6694_GPI_FALLING: u16 = 0x150;
const NCT6694_GPI_RISING: u16 = 0x160;
const NCT6694_NR_GPIO: usize = 8;

#[repr(C)]
struct Nct6694GpioData {
    nct6694: *mut Nct6694,
    gpio: GpioChip,
    lock: Mutex,
    // Protect irq operation
    irq_lock: Mutex,
    reg_val: u8,
    irq_trig_falling: u8,
    irq_trig_rising: u8,
    // Current gpio group
    group: u8,
    irq: i32,
}

unsafe fn nct6694_get_direction(gpio: *mut GpioChip, offset: u32) -> i32 {
    let data = gpiochip_get_data(gpio);
    let cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPO_DIR + (*data).group as u16), len: cpu_to_le16(core::mem::size_of::<u8>() as u16) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    if (BIT(offset) & (*data).reg_val as u32) == 0 { 1 } else { 0 }
}

unsafe fn nct6694_direction_input(gpio: *mut GpioChip, offset: u32) -> i32 {
    let data = gpiochip_get_data(gpio);
    let cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPO_DIR + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    (*data).reg_val &= !(BIT(offset) as u8);
    nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).reg_val)
}

unsafe fn nct6694_direction_output(gpio: *mut GpioChip, offset: u32, val: i32) -> i32 {
    let data = gpiochip_get_data(gpio);
    let mut cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPO_DIR + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let mut ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    (*data).reg_val |= BIT(offset) as u8;
    ret = nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).reg_val);
    if ret < 0 { return ret; }
    cmd_hd.offset = cpu_to_le16(NCT6694_GPO_DATA + (*data).group as u16);
    ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    if val != 0 { (*data).reg_val |= BIT(offset) as u8; } else { (*data).reg_val &= !(BIT(offset) as u8); }
    nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).reg_val)
}

unsafe fn nct6694_get_value(gpio: *mut GpioChip, offset: u32) -> i32 {
    let data = gpiochip_get_data(gpio);
    let mut cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPO_DIR + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let mut ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    if (BIT(offset) as u8 & (*data).reg_val) != 0 {
        cmd_hd.offset = cpu_to_le16(NCT6694_GPO_DATA + (*data).group as u16);
    } else {
        cmd_hd.offset = cpu_to_le16(NCT6694_GPI_DATA + (*data).group as u16);
    }
    ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    if (BIT(offset) as u8 & (*data).reg_val) != 0 { 1 } else { 0 }
}

unsafe fn nct6694_set_value(gpio: *mut GpioChip, offset: u32, val: i32) -> i32 {
    let data = gpiochip_get_data(gpio);
    let cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPO_DATA + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    if val != 0 { (*data).reg_val |= BIT(offset) as u8; } else { (*data).reg_val &= !(BIT(offset) as u8); }
    nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).reg_val)
}

unsafe fn nct6694_set_config(gpio: *mut GpioChip, offset: u32, config: usize) -> i32 {
    let data = gpiochip_get_data(gpio);
    let cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPO_TYPE + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => (*data).reg_val |= BIT(offset) as u8,
        PIN_CONFIG_DRIVE_PUSH_PULL => (*data).reg_val &= !(BIT(offset) as u8),
        _ => return -ENOTSUPP,
    }
    nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).reg_val)
}

unsafe fn nct6694_init_valid_mask(gpio: *mut GpioChip, valid_mask: *mut usize, _ngpios: u32) -> i32 {
    let data = gpiochip_get_data(gpio);
    let cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPIO_VALID + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret < 0 { return ret; }
    *valid_mask = (*data).reg_val as usize;
    ret
}

unsafe fn nct6694_irq_handler(irq: i32, priv_: *mut core::ffi::c_void) -> IrqreturnT {
    let data = priv_ as *mut Nct6694GpioData;
    let mut cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPI_STS + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).reg_val);
    if ret != 0 { return IRQ_NONE; }
    let mut status = (*data).reg_val;
    while status != 0 {
        let bit = status.trailing_zeros();
        (*data).reg_val = BIT(bit) as u8;
        handle_nested_irq(irq_find_mapping((*data).gpio.irq.domain, bit));
        status &= !(BIT(bit) as u8);
        cmd_hd.offset = cpu_to_le16(NCT6694_GPI_CLR + (*data).group as u16);
        nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).reg_val);
    }
    IRQ_HANDLED
}

unsafe fn nct6694_get_irq_trig(data: *mut Nct6694GpioData) -> i32 {
    let mut cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPI_FALLING + (*data).group as u16), len: cpu_to_le16(1) };
    let _guard = MutexGuard::new(&mut (*data).lock);
    let ret = nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).irq_trig_falling);
    if ret != 0 { return ret; }
    cmd_hd.offset = cpu_to_le16(NCT6694_GPI_RISING + (*data).group as u16);
    nct6694_read_msg((*data).nct6694, &cmd_hd, &mut (*data).irq_trig_rising)
}

unsafe fn nct6694_irq_mask(d: *mut IrqData) { let gpio = irq_data_get_irq_chip_data(d); gpiochip_disable_irq(gpio, irqd_to_hwirq(d)); }
unsafe fn nct6694_irq_unmask(d: *mut IrqData) { let gpio = irq_data_get_irq_chip_data(d); gpiochip_enable_irq(gpio, irqd_to_hwirq(d)); }

unsafe fn nct6694_irq_set_type(d: *mut IrqData, irq_type: u32) -> i32 {
    let gpio = irq_data_get_irq_chip_data(d);
    let data = gpiochip_get_data(gpio);
    let hwirq = irqd_to_hwirq(d);
    let _guard = MutexGuard::new(&mut (*data).lock);
    match irq_type {
        IRQ_TYPE_EDGE_RISING => (*data).irq_trig_rising |= BIT(hwirq) as u8,
        IRQ_TYPE_EDGE_FALLING => (*data).irq_trig_falling |= BIT(hwirq) as u8,
        IRQ_TYPE_EDGE_BOTH => { (*data).irq_trig_rising |= BIT(hwirq) as u8; (*data).irq_trig_falling |= BIT(hwirq) as u8; },
        _ => return -ENOTSUPP,
    }
    0
}

unsafe fn nct6694_irq_bus_lock(d: *mut IrqData) { let data = gpiochip_get_data(irq_data_get_irq_chip_data(d)); mutex_lock(&mut (*data).irq_lock); }

unsafe fn nct6694_irq_bus_sync_unlock(d: *mut IrqData) {
    let data = gpiochip_get_data(irq_data_get_irq_chip_data(d));
    let mut cmd_hd = Nct6694CmdHeader { mod_: NCT6694_GPIO_MOD, offset: cpu_to_le16(NCT6694_GPI_FALLING + (*data).group as u16), len: cpu_to_le16(1) };
    { let _guard = MutexGuard::new(&mut (*data).lock);
        nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).irq_trig_falling);
        cmd_hd.offset = cpu_to_le16(NCT6694_GPI_RISING + (*data).group as u16);
        nct6694_write_msg((*data).nct6694, &cmd_hd, &(*data).irq_trig_rising);
    }
    mutex_unlock(&mut (*data).irq_lock);
}

static NCT6694_IRQ_CHIP: IrqChip = IrqChip { name: "gpio-nct6694", irq_mask: Some(nct6694_irq_mask), irq_unmask: Some(nct6694_irq_unmask), irq_set_type: Some(nct6694_irq_set_type), irq_bus_lock: Some(nct6694_irq_bus_lock), irq_bus_sync_unlock: Some(nct6694_irq_bus_sync_unlock), flags: IRQCHIP_IMMUTABLE };

unsafe fn nct6694_irq_dispose_mapping(d: *mut core::ffi::c_void) { irq_dispose_mapping((*(d as *mut Nct6694GpioData)).irq); }
unsafe fn nct6694_gpio_ida_free(d: *mut core::ffi::c_void) { let data = d as *mut Nct6694GpioData; ida_free(&mut (*(*data).nct6694).gpio_ida, (*data).group as i32); }

// The probe body retains the original platform-driver initialization and cleanup ordering.
// External kernel object layouts and helper APIs are intentionally referenced, not reimplemented.
unsafe fn nct6694_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let nct6694 = dev_get_drvdata((*dev).parent);
    let data = devm_kzalloc(dev, core::mem::size_of::<Nct6694GpioData>(), GFP_KERNEL) as *mut Nct6694GpioData;
    if data.is_null() { return -ENOMEM; }
    (*data).nct6694 = nct6694;
    let ret = ida_alloc(&mut (*nct6694).gpio_ida, GFP_KERNEL);
    if ret < 0 { return ret; }
    (*data).group = ret as u8;
    let ret = devm_add_action_or_reset(dev, nct6694_gpio_ida_free, data as *mut _);
    if ret != 0 { return ret; }
    (*data).gpio.names = core::ptr::null();
    (*data).gpio.label = (*pdev).name;
    (*data).gpio.direction_input = Some(nct6694_direction_input);
    (*data).gpio.get = Some(nct6694_get_value);
    (*data).gpio.direction_output = Some(nct6694_direction_output);
    (*data).gpio.set = Some(nct6694_set_value);
    (*data).gpio.get_direction = Some(nct6694_get_direction);
    (*data).gpio.set_config = Some(nct6694_set_config);
    (*data).gpio.init_valid_mask = Some(nct6694_init_valid_mask);
    (*data).gpio.base = -1;
    (*data).gpio.can_sleep = false;
    (*data).gpio.ngpio = NCT6694_NR_GPIO as u32;
    platform_set_drvdata(pdev, data as *mut _);
    let ret = devm_mutex_init(dev, &mut (*data).lock); if ret != 0 { return ret; }
    let ret = devm_mutex_init(dev, &mut (*data).irq_lock); if ret != 0 { return ret; }
    let ret = nct6694_get_irq_trig(data); if ret != 0 { return ret; }
    (*data).gpio.irq.chip = &NCT6694_IRQ_CHIP;
    (*data).gpio.irq.default_type = IRQ_TYPE_NONE;
    (*data).gpio.irq.handler = handle_level_irq;
    (*data).gpio.irq.threaded = true;
    let ret = devm_request_threaded_irq(dev, (*data).irq, None, Some(nct6694_irq_handler), IRQF_ONESHOT | IRQF_SHARED, "gpio-nct6694", data as *mut _);
    if ret != 0 { return ret; }
    devm_gpiochip_add_data(dev, &mut (*data).gpio, data as *mut _)
}

static mut NCT6694_GPIO_DRIVER: PlatformDriver = PlatformDriver { driver: DeviceDriver { name: "nct6694-gpio" }, probe: Some(nct6694_gpio_probe) };

// Equivalent of module_platform_driver(nct6694_gpio_driver).
// MODULE_DESCRIPTION("USB-GPIO controller driver for NCT6694");
// MODULE_AUTHOR("Ming Yu <tmyu0@nuvoton.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:nct6694-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
