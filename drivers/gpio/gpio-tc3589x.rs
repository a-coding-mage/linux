// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2010
 *
 * Author: Hanumath Prasad <hanumath.prasad@stericsson.com> for ST-Ericsson
 * Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
 */

// Dependencies supplied by the Linux kernel and other translation units.

#[repr(usize)]
enum Reg {
    Ibe,
    Iev,
    Is,
    Ie,
    Direct,
}

const CACHE_NR_REGS: usize = 5;
const CACHE_NR_BANKS: usize = 3;

#[repr(C)]
struct Tc3589xGpio {
    chip: gpio_chip,
    tc3589x: *mut tc3589x,
    dev: *mut device,
    irq_lock: mutex,
    // Caches of interrupt control registers for bus_lock
    regs: [[u8; CACHE_NR_BANKS]; CACHE_NR_REGS],
    oldregs: [[u8; CACHE_NR_BANKS]; CACHE_NR_REGS],
}

unsafe fn tc3589x_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let tc3589x_gpio = gpiochip_get_data(chip) as *mut Tc3589xGpio;
    let tc3589x = (*tc3589x_gpio).tc3589x;
    let reg: u8 = TC3589x_GPIODATA0 + ((offset / 8) * 2) as u8;
    let mask: u8 = BIT(offset % 8) as u8;
    let ret = tc3589x_reg_read(tc3589x, reg);
    if ret < 0 { return ret; }
    if (ret & mask as c_int) != 0 { 1 } else { 0 }
}

unsafe fn tc3589x_gpio_set(chip: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    let tc3589x_gpio = gpiochip_get_data(chip) as *mut Tc3589xGpio;
    let tc3589x = (*tc3589x_gpio).tc3589x;
    let reg: u8 = TC3589x_GPIODATA0 + ((offset / 8) * 2) as u8;
    let pos = offset % 8;
    let data = [if val != 0 { BIT(pos) as u8 } else { 0 }, BIT(pos) as u8];
    tc3589x_block_write(tc3589x, reg, data.len(), data.as_ptr())
}

unsafe fn tc3589x_gpio_direction_output(chip: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    let tc3589x_gpio = gpiochip_get_data(chip) as *mut Tc3589xGpio;
    let tc3589x = (*tc3589x_gpio).tc3589x;
    let reg: u8 = TC3589x_GPIODIR0 + (offset / 8) as u8;
    let pos = offset % 8;
    let ret = tc3589x_gpio_set(chip, offset, val);
    if ret != 0 { return ret; }
    tc3589x_set_bits(tc3589x, reg, BIT(pos), BIT(pos))
}

unsafe fn tc3589x_gpio_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let tc3589x_gpio = gpiochip_get_data(chip) as *mut Tc3589xGpio;
    let reg: u8 = TC3589x_GPIODIR0 + (offset / 8) as u8;
    tc3589x_set_bits((*tc3589x_gpio).tc3589x, reg, BIT(offset % 8), 0)
}

unsafe fn tc3589x_gpio_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let tc3589x_gpio = gpiochip_get_data(chip) as *mut Tc3589xGpio;
    let reg: u8 = TC3589x_GPIODIR0 + (offset / 8) as u8;
    let ret = tc3589x_reg_read((*tc3589x_gpio).tc3589x, reg);
    if ret < 0 { return ret; }
    if (ret & BIT(offset % 8)) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn tc3589x_gpio_set_config(chip: *mut gpio_chip, offset: c_uint, config: c_ulong) -> c_int {
    let tc3589x_gpio = gpiochip_get_data(chip) as *mut Tc3589xGpio;
    let tc3589x = (*tc3589x_gpio).tc3589x;
    let odmreg: u8 = TC3589x_GPIOODM0 + ((offset / 8) * 2) as u8;
    let odereg: u8 = TC3589x_GPIOODE0 + ((offset / 8) * 2) as u8;
    let pos = offset % 8;
    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => {
            let ret = tc3589x_set_bits(tc3589x, odmreg, BIT(pos), 0);
            if ret != 0 { return ret; }
            tc3589x_set_bits(tc3589x, odereg, BIT(pos), BIT(pos))
        }
        PIN_CONFIG_DRIVE_OPEN_SOURCE => {
            let ret = tc3589x_set_bits(tc3589x, odmreg, BIT(pos), BIT(pos));
            if ret != 0 { return ret; }
            tc3589x_set_bits(tc3589x, odereg, BIT(pos), BIT(pos))
        }
        PIN_CONFIG_DRIVE_PUSH_PULL => tc3589x_set_bits(tc3589x, odereg, BIT(pos), 0),
        _ => -ENOTSUPP,
    }
}

static mut TEMPLATE_CHIP: gpio_chip = gpio_chip {
    label: b"tc3589x\0".as_ptr() as *const c_char,
    owner: THIS_MODULE,
    get: Some(tc3589x_gpio_get),
    set: Some(tc3589x_gpio_set),
    direction_output: Some(tc3589x_gpio_direction_output),
    direction_input: Some(tc3589x_gpio_direction_input),
    get_direction: Some(tc3589x_gpio_get_direction),
    set_config: Some(tc3589x_gpio_set_config),
    can_sleep: true,
    ..gpio_chip::default()
};

unsafe fn tc3589x_gpio_irq_set_type(d: *mut irq_data, irq_type: c_uint) -> c_int {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let gpio = gpiochip_get_data(gc) as *mut Tc3589xGpio;
    let offset = (*d).hwirq as usize;
    let regoffset = offset / 8;
    let mask = BIT((offset % 8) as c_uint) as u8;
    if irq_type == IRQ_TYPE_EDGE_BOTH { (*gpio).regs[Reg::Ibe as usize][regoffset] |= mask; return 0; }
    (*gpio).regs[Reg::Ibe as usize][regoffset] &= !mask;
    if irq_type == IRQ_TYPE_LEVEL_LOW || irq_type == IRQ_TYPE_LEVEL_HIGH { (*gpio).regs[Reg::Is as usize][regoffset] |= mask; } else { (*gpio).regs[Reg::Is as usize][regoffset] &= !mask; }
    if irq_type == IRQ_TYPE_EDGE_RISING || irq_type == IRQ_TYPE_LEVEL_HIGH { (*gpio).regs[Reg::Iev as usize][regoffset] |= mask; } else { (*gpio).regs[Reg::Iev as usize][regoffset] &= !mask; }
    0
}

unsafe fn tc3589x_gpio_irq_lock(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let gpio = gpiochip_get_data(gc) as *mut Tc3589xGpio;
    mutex_lock(&mut (*gpio).irq_lock);
}

unsafe fn tc3589x_gpio_irq_sync_unlock(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let gpio = gpiochip_get_data(gc) as *mut Tc3589xGpio;
    let tc = (*gpio).tc3589x;
    let regmap = [TC3589x_GPIOIBE0, TC3589x_GPIOIEV0, TC3589x_GPIOIS0, TC3589x_GPIOIE0, TC3589x_DIRECT0];
    for i in 0..CACHE_NR_REGS { for j in 0..CACHE_NR_BANKS {
        let old = (*gpio).oldregs[i][j]; let new = (*gpio).regs[i][j];
        if new != old { (*gpio).oldregs[i][j] = new; tc3589x_reg_write(tc, regmap[i] + j as u8, new); }
    }}
    mutex_unlock(&mut (*gpio).irq_lock);
}

unsafe fn tc3589x_gpio_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let gpio = gpiochip_get_data(gc) as *mut Tc3589xGpio;
    let offset = (*d).hwirq as usize; let mask = BIT((offset % 8) as c_uint) as u8;
    (*gpio).regs[Reg::Ie as usize][offset / 8] &= !mask;
    (*gpio).regs[Reg::Direct as usize][offset / 8] |= mask;
    gpiochip_disable_irq(gc, offset as c_uint);
}

unsafe fn tc3589x_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let gpio = gpiochip_get_data(gc) as *mut Tc3589xGpio;
    let offset = (*d).hwirq as usize; let mask = BIT((offset % 8) as c_uint) as u8;
    gpiochip_enable_irq(gc, offset as c_uint);
    (*gpio).regs[Reg::Ie as usize][offset / 8] |= mask;
    (*gpio).regs[Reg::Direct as usize][offset / 8] &= !mask;
}

static mut TC3589X_GPIO_IRQ_CHIP: irq_chip = irq_chip {
    name: b"tc3589x-gpio\0".as_ptr() as *const c_char,
    irq_bus_lock: Some(tc3589x_gpio_irq_lock), irq_bus_sync_unlock: Some(tc3589x_gpio_irq_sync_unlock),
    irq_mask: Some(tc3589x_gpio_irq_mask), irq_unmask: Some(tc3589x_gpio_irq_unmask),
    irq_set_type: Some(tc3589x_gpio_irq_set_type), flags: IRQCHIP_IMMUTABLE, ..irq_chip::default()
};

unsafe fn tc3589x_gpio_irq(_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    let gpio = dev as *mut Tc3589xGpio; let tc = (*gpio).tc3589x;
    let mut status = [0u8; CACHE_NR_BANKS];
    if tc3589x_block_read(tc, TC3589x_GPIOMIS0, status.len(), status.as_mut_ptr()) < 0 { return IRQ_NONE; }
    for i in 0..status.len() { let mut stat = status[i]; if stat == 0 { continue; }
        while stat != 0 { let bit = __ffs(stat as c_uint); let line = i * 8 + bit as usize;
            let irq = irq_find_mapping((*gpio).chip.irq.domain, line as c_uint); handle_nested_irq(irq); stat &= !(1u8 << bit); }
        tc3589x_reg_write(tc, TC3589x_GPIOIC0 + i as u8, status[i]);
    }
    IRQ_HANDLED
}

// Probe and driver-registration declarations retain the kernel driver's external interface.
unsafe fn tc3589x_gpio_probe(pdev: *mut platform_device) -> c_int {
    let tc = dev_get_drvdata((*pdev).dev.parent) as *mut tc3589x;
    let np = (*pdev).dev.of_node;
    if np.is_null() { dev_err(&mut (*pdev).dev, b"No Device Tree node found\0".as_ptr()); return -EINVAL; }
    let irq = platform_get_irq(pdev, 0); if irq < 0 { return irq; }
    let gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Tc3589xGpio>(), GFP_KERNEL) as *mut Tc3589xGpio;
    if gpio.is_null() { return -ENOMEM; }
    mutex_init(&mut (*gpio).irq_lock); (*gpio).dev = &mut (*pdev).dev; (*gpio).tc3589x = tc;
    (*gpio).chip = TEMPLATE_CHIP; (*gpio).chip.ngpio = (*tc).num_gpio; (*gpio).chip.parent = &mut (*pdev).dev; (*gpio).chip.base = -1;
    let girq = &mut (*gpio).chip.irq; gpio_irq_chip_set_chip(girq, &TC3589X_GPIO_IRQ_CHIP);
    girq.parent_handler = None; girq.num_parents = 0; girq.parents = core::ptr::null_mut(); girq.default_type = IRQ_TYPE_NONE; girq.handler = Some(handle_simple_irq); girq.threaded = true;
    let ret = tc3589x_set_bits(tc, TC3589x_RSTCTRL, TC3589x_RSTCTRL_GPIRST, 0); if ret < 0 { return ret; }
    let ret = tc3589x_reg_write(tc, TC3589x_DKBDMSK, TC3589x_DKBDMSK_ELINT | TC3589x_DKBDMSK_EINT); if ret < 0 { return ret; }
    let ret = devm_request_threaded_irq(&mut (*pdev).dev, irq, None, Some(tc3589x_gpio_irq), IRQF_ONESHOT, b"tc3589x-gpio\0".as_ptr(), gpio as *mut c_void); if ret != 0 { return ret; }
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).chip, gpio as *mut c_void)
}

static mut TC3589X_GPIO_DRIVER: platform_driver = platform_driver { driver: driver { name: b"tc3589x-gpio\0".as_ptr() as *const c_char, ..driver::default() }, probe: Some(tc3589x_gpio_probe), ..platform_driver::default() };

unsafe fn tc3589x_gpio_init() -> c_int { platform_driver_register(&mut TC3589X_GPIO_DRIVER) }
// subsys_initcall(tc3589x_gpio_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
