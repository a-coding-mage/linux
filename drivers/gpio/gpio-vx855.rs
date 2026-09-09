// SPDX-License-Identifier: GPL-2.0+
/*
 * Linux GPIOlib driver for the VIA VX855 integrated southbridge GPIO
 *
 * Copyright (C) 2009 VIA Technologies, Inc.
 * Copyright (C) 2010 One Laptop per Child
 * Author: Harald Welte <HaraldWelte@viatech.com>
 * All rights reserved.
 */
// Linux kernel dependencies supplied by the surrounding translation unit.

const MODULE_NAME: &str = "vx855_gpio";

/* The VX855 south bridge has the following GPIO pins:
 *\tGPI 0...13\tGeneral Purpose Input
 *\tGPO 0...12\tGeneral Purpose Output
 *\tGPIO 0...14\tGeneral Purpose I/O (Open-Drain)
 */
const NR_VX855_GPI: usize = 14;
const NR_VX855_GPO: usize = 13;
const NR_VX855_GPIO: usize = 15;
const NR_VX855_GPINO: usize = NR_VX855_GPI + NR_VX855_GPO;
const NR_VX855_GP: usize = NR_VX855_GPI + NR_VX855_GPO + NR_VX855_GPIO;

#[repr(C)]
struct Vx855Gpio {
    gpio: gpio_chip,
    lock: spinlock_t,
    io_gpi: u32,
    io_gpo: u32,
}

/* resolve a GPIx into the corresponding bit position */
#[inline]
unsafe fn gpi_i_bit(i: i32) -> u32 {
    if i < 10 { 1u32 << i } else { 1u32 << (i + 14) }
}

#[inline]
unsafe fn gpo_o_bit(i: i32) -> u32 {
    if i < 11 { 1u32 << i } else { 1u32 << (i + 14) }
}

#[inline]
unsafe fn gpio_i_bit(i: i32) -> u32 {
    if i < 14 { 1u32 << (i + 10) } else { 1u32 << (i + 14) }
}

#[inline]
unsafe fn gpio_o_bit(i: i32) -> u32 {
    if i < 14 { 1u32 << (i + 11) } else { 1u32 << (i + 13) }
}

/* Mapping between numeric GPIO ID and the actual GPIO hardware numbering:
 * 0..13\tGPI 0..13
 * 14..26\tGPO 0..12
 * 27..41\tGPIO 0..14
 */

unsafe fn vx855gpio_direction_input(gpio: *mut gpio_chip, nr: u32) -> i32 {
    let vg = gpiochip_get_data(gpio) as *mut Vx855Gpio;
    let mut flags: c_ulong = 0;
    let mut reg_out: u32;

    /* Real GPI bits are always in input direction */
    if nr < NR_VX855_GPI as u32 { return 0; }
    /* Real GPO bits cannot be put in output direction */
    if nr < NR_VX855_GPINO as u32 { return -EINVAL; }

    /* Open Drain GPIO have to be set to one */
    spin_lock_irqsave(&mut (*vg).lock, &mut flags);
    reg_out = inl((*vg).io_gpo);
    reg_out |= gpio_o_bit((nr - NR_VX855_GPINO as u32) as i32);
    outl(reg_out, (*vg).io_gpo);
    spin_unlock_irqrestore(&mut (*vg).lock, flags);
    0
}

unsafe fn vx855gpio_get(gpio: *mut gpio_chip, nr: u32) -> i32 {
    let vg = gpiochip_get_data(gpio) as *mut Vx855Gpio;
    let reg_in: u32;
    let mut ret = 0;
    if nr < NR_VX855_GPI as u32 {
        reg_in = inl((*vg).io_gpi);
        if reg_in & gpi_i_bit(nr as i32) != 0 { ret = 1; }
    } else if nr < NR_VX855_GPINO as u32 {
        /* GPO don't have an input bit, we need to read it back from the output register */
        reg_in = inl((*vg).io_gpo);
        if reg_in & gpo_o_bit((nr - NR_VX855_GPI as u32) as i32) != 0 { ret = 1; }
    } else {
        reg_in = inl((*vg).io_gpi);
        if reg_in & gpio_i_bit((nr - NR_VX855_GPINO as u32) as i32) != 0 { ret = 1; }
    }
    ret
}

unsafe fn vx855gpio_set(gpio: *mut gpio_chip, nr: u32, val: i32) -> i32 {
    let vg = gpiochip_get_data(gpio) as *mut Vx855Gpio;
    let mut flags: c_ulong = 0;
    let mut reg_out: u32;
    /* True GPI cannot be switched to output mode */
    if nr < NR_VX855_GPI as u32 { return -EPERM; }
    spin_lock_irqsave(&mut (*vg).lock, &mut flags);
    reg_out = inl((*vg).io_gpo);
    if nr < NR_VX855_GPINO as u32 {
        let bit = gpo_o_bit((nr - NR_VX855_GPI as u32) as i32);
        if val != 0 { reg_out |= bit; } else { reg_out &= !bit; }
    } else {
        let bit = gpio_o_bit((nr - NR_VX855_GPINO as u32) as i32);
        if val != 0 { reg_out |= bit; } else { reg_out &= !bit; }
    }
    outl(reg_out, (*vg).io_gpo);
    spin_unlock_irqrestore(&mut (*vg).lock, flags);
    0
}

unsafe fn vx855gpio_direction_output(gpio: *mut gpio_chip, nr: u32, val: i32) -> i32 {
    /* True GPI cannot be switched to output mode */
    if nr < NR_VX855_GPI as u32 { return -EINVAL; }
    /* True GPO don't need to be switched to output mode, and GPIO are open-drain, i.e. also need no switching, so all we do is set the level */
    vx855gpio_set(gpio, nr, val);
    0
}

unsafe fn vx855gpio_set_config(gpio: *mut gpio_chip, nr: u32, config: c_ulong) -> i32 {
    let param = pinconf_to_config_param(config);
    /* The GPI cannot be single-ended */
    if nr < NR_VX855_GPI as u32 { return -EINVAL; }
    /* The GPO's are push-pull */
    if nr < NR_VX855_GPINO as u32 {
        if param != PIN_CONFIG_DRIVE_PUSH_PULL { return -ENOTSUPP; }
        return 0;
    }
    /* The GPIO's are open drain */
    if param != PIN_CONFIG_DRIVE_OPEN_DRAIN { return -ENOTSUPP; }
    0
}

static VX855GPIO_NAMES: [&str; NR_VX855_GP] = [
    "VX855_GPI0", "VX855_GPI1", "VX855_GPI2", "VX855_GPI3", "VX855_GPI4", "VX855_GPI5", "VX855_GPI6", "VX855_GPI7", "VX855_GPI8", "VX855_GPI9", "VX855_GPI10", "VX855_GPI11", "VX855_GPI12", "VX855_GPI13",
    "VX855_GPO0", "VX855_GPO1", "VX855_GPO2", "VX855_GPO3", "VX855_GPO4", "VX855_GPO5", "VX855_GPO6", "VX855_GPO7", "VX855_GPO8", "VX855_GPO9", "VX855_GPO10", "VX855_GPO11", "VX855_GPO12",
    "VX855_GPIO0", "VX855_GPIO1", "VX855_GPIO2", "VX855_GPIO3", "VX855_GPIO4", "VX855_GPIO5", "VX855_GPIO6", "VX855_GPIO7", "VX855_GPIO8", "VX855_GPIO9", "VX855_GPIO10", "VX855_GPIO11", "VX855_GPIO12", "VX855_GPIO13", "VX855_GPIO14"
];

unsafe fn vx855gpio_gpio_setup(vg: *mut Vx855Gpio) {
    let c = &mut (*vg).gpio;
    c.label = b"VX855 South Bridge\0".as_ptr() as *const c_char;
    c.owner = THIS_MODULE;
    c.direction_input = Some(vx855gpio_direction_input);
    c.direction_output = Some(vx855gpio_direction_output);
    c.get = Some(vx855gpio_get);
    c.set = Some(vx855gpio_set);
    c.set_config = Some(vx855gpio_set_config);
    c.dbg_show = None;
    c.base = 0;
    c.ngpio = NR_VX855_GP as u32;
    c.can_sleep = false;
    c.names = VX855GPIO_NAMES.as_ptr();
}

/* This platform device is ordinarily registered by the vx855 mfd driver */
unsafe fn vx855gpio_probe(pdev: *mut platform_device) -> i32 {
    let res_gpi = platform_get_resource(pdev, IORESOURCE_IO, 0);
    let res_gpo = platform_get_resource(pdev, IORESOURCE_IO, 1);
    if res_gpi.is_null() || res_gpo.is_null() { return -EBUSY; }
    let vg = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Vx855Gpio>(), GFP_KERNEL) as *mut Vx855Gpio;
    if vg.is_null() { return -ENOMEM; }
    dev_info(&mut (*pdev).dev, "found VX855 GPIO controller\n");
    (*vg).io_gpi = (*res_gpi).start;
    (*vg).io_gpo = (*res_gpo).start;
    spin_lock_init(&mut (*vg).lock);
    /* ACPI may already reserve these regions; ignore failures and continue. */
    if devm_request_region(&mut (*pdev).dev, (*res_gpi).start, resource_size(res_gpi), MODULE_NAME.as_ptr(),) .is_null() { dev_warn(&mut (*pdev).dev, "GPI I/O resource busy, probably claimed by ACPI\n"); }
    if devm_request_region(&mut (*pdev).dev, (*res_gpo).start, resource_size(res_gpo), MODULE_NAME.as_ptr(),) .is_null() { dev_warn(&mut (*pdev).dev, "GPO I/O resource busy, probably claimed by ACPI\n"); }
    vx855gpio_gpio_setup(vg);
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*vg).gpio, vg)
}

// C module metadata and module_platform_driver(vx855gpio_driver) are retained as external kernel registration metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
