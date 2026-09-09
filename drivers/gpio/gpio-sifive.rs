// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 SiFive
 */

// Linux headers and build-time module infrastructure are supplied by the kernel
// Rust environment.

const SIFIVE_GPIO_INPUT_VAL: usize = 0x00;
const SIFIVE_GPIO_INPUT_EN: usize = 0x04;
const SIFIVE_GPIO_OUTPUT_EN: usize = 0x08;
const SIFIVE_GPIO_OUTPUT_VAL: usize = 0x0C;
const SIFIVE_GPIO_RISE_IE: usize = 0x18;
const SIFIVE_GPIO_RISE_IP: usize = 0x1C;
const SIFIVE_GPIO_FALL_IE: usize = 0x20;
const SIFIVE_GPIO_FALL_IP: usize = 0x24;
const SIFIVE_GPIO_HIGH_IE: usize = 0x28;
const SIFIVE_GPIO_HIGH_IP: usize = 0x2C;
const SIFIVE_GPIO_LOW_IE: usize = 0x30;
const SIFIVE_GPIO_LOW_IP: usize = 0x34;
const SIFIVE_GPIO_OUTPUT_XOR: usize = 0x40;

const SIFIVE_GPIO_MAX: usize = 32;

#[repr(C)]
struct SifiveGpio {
    base: *mut core::ffi::c_void,
    gen_gc: gpio_generic_chip,
    regs: *mut regmap,
    irq_state: usize,
    trigger: [u32; SIFIVE_GPIO_MAX],
    irq_number: [u32; SIFIVE_GPIO_MAX],
}

unsafe fn sifive_gpio_set_ie(chip: *mut SifiveGpio, offset: u32) {
    // guard(gpio_generic_lock_irqsave)(&chip->gen_gc);
    let trigger = if ((*chip).irq_state & (1usize << offset)) != 0 {
        (*chip).trigger[offset as usize]
    } else {
        0
    };
    regmap_update_bits((*chip).regs, SIFIVE_GPIO_RISE_IE as u32, 1u32 << offset,
        if (trigger & IRQ_TYPE_EDGE_RISING) != 0 { 1u32 << offset } else { 0 });
    regmap_update_bits((*chip).regs, SIFIVE_GPIO_FALL_IE as u32, 1u32 << offset,
        if (trigger & IRQ_TYPE_EDGE_FALLING) != 0 { 1u32 << offset } else { 0 });
    regmap_update_bits((*chip).regs, SIFIVE_GPIO_HIGH_IE as u32, 1u32 << offset,
        if (trigger & IRQ_TYPE_LEVEL_HIGH) != 0 { 1u32 << offset } else { 0 });
    regmap_update_bits((*chip).regs, SIFIVE_GPIO_LOW_IE as u32, 1u32 << offset,
        if (trigger & IRQ_TYPE_LEVEL_LOW) != 0 { 1u32 << offset } else { 0 });
}

unsafe fn sifive_gpio_irq_set_type(d: *mut irq_data, trigger: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = gpiochip_get_data(gc);
    let offset = irqd_to_hwirq(d) as i32;
    if offset < 0 || offset >= (*gc).ngpio as i32 { return -EINVAL; }
    (*chip).trigger[offset as usize] = trigger;
    sifive_gpio_set_ie(chip, offset as u32);
    0
}

unsafe fn sifive_gpio_irq_enable(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = gpiochip_get_data(gc);
    let hwirq = irqd_to_hwirq(d);
    let offset = hwirq % SIFIVE_GPIO_MAX as u32;
    let bit = 1u32 << offset;
    gpiochip_enable_irq(gc, hwirq);
    irq_chip_enable_parent(d);
    ((*gc).direction_input)(gc, offset);
    // scoped_guard(gpio_generic_lock_irqsave, &chip->gen_gc)
    regmap_write((*chip).regs, SIFIVE_GPIO_RISE_IP as u32, bit);
    regmap_write((*chip).regs, SIFIVE_GPIO_FALL_IP as u32, bit);
    regmap_write((*chip).regs, SIFIVE_GPIO_HIGH_IP as u32, bit);
    regmap_write((*chip).regs, SIFIVE_GPIO_LOW_IP as u32, bit);
    (*chip).irq_state |= 1usize << offset;
    sifive_gpio_set_ie(chip, offset);
}

unsafe fn sifive_gpio_irq_disable(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = gpiochip_get_data(gc);
    let hwirq = irqd_to_hwirq(d);
    let offset = hwirq % SIFIVE_GPIO_MAX as u32;
    (*chip).irq_state &= !(1usize << offset);
    sifive_gpio_set_ie(chip, offset);
    irq_chip_disable_parent(d);
    gpiochip_disable_irq(gc, hwirq);
}

unsafe fn sifive_gpio_irq_eoi(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = gpiochip_get_data(gc);
    let offset = irqd_to_hwirq(d) % SIFIVE_GPIO_MAX as u32;
    let bit = 1u32 << offset;
    // scoped_guard(gpio_generic_lock_irqsave, &chip->gen_gc)
    regmap_write((*chip).regs, SIFIVE_GPIO_RISE_IP as u32, bit);
    regmap_write((*chip).regs, SIFIVE_GPIO_FALL_IP as u32, bit);
    regmap_write((*chip).regs, SIFIVE_GPIO_HIGH_IP as u32, bit);
    regmap_write((*chip).regs, SIFIVE_GPIO_LOW_IP as u32, bit);
    irq_chip_eoi_parent(d);
}

unsafe fn sifive_gpio_irq_set_affinity(data: *mut irq_data, dest: *const cpumask, force: bool) -> i32 {
    if !(*data).parent_data.is_null() { irq_chip_set_affinity_parent(data, dest, force) } else { -EINVAL }
}

#[repr(C)]
struct irq_chip {
    name: *const u8,
    irq_set_type: Option<unsafe fn(*mut irq_data, u32) -> i32>,
    irq_enable: Option<unsafe fn(*mut irq_data)>,
    irq_disable: Option<unsafe fn(*mut irq_data)>,
    irq_eoi: Option<unsafe fn(*mut irq_data)>,
    irq_set_affinity: Option<unsafe fn(*mut irq_data, *const cpumask, bool) -> i32>,
}

static SIFIVE_GPIO_IRQCHIP: irq_chip = irq_chip {
    name: b"sifive-gpio\0".as_ptr(),
    irq_set_type: Some(sifive_gpio_irq_set_type),
    irq_enable: Some(sifive_gpio_irq_enable),
    irq_disable: Some(sifive_gpio_irq_disable),
    irq_eoi: Some(sifive_gpio_irq_eoi),
    irq_set_affinity: Some(sifive_gpio_irq_set_affinity),
};

unsafe fn sifive_gpio_child_to_parent_hwirq(gc: *mut gpio_chip, child: u32, _child_type: u32,
                                             parent: *mut u32, parent_type: *mut u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let d = irq_get_irq_data((*chip).irq_number[child as usize]);
    *parent_type = IRQ_TYPE_NONE;
    *parent = irqd_to_hwirq(d);
    0
}

unsafe fn sifive_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let chip = devm_kzalloc(dev, core::mem::size_of::<SifiveGpio>(), GFP_KERNEL)
        as *mut SifiveGpio;
    if chip.is_null() { return -ENOMEM; }
    (*chip).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*chip).base) {
        dev_err(dev, b"failed to allocate device memory\0".as_ptr());
        return ptr_err((*chip).base);
    }
    (*chip).regs = devm_regmap_init_mmio(dev, (*chip).base, &SIFIVE_GPIO_REGMAP_CONFIG);
    if is_err((*chip).regs as *mut core::ffi::c_void) { return ptr_err((*chip).regs as *mut _); }

    let mut ngpio: i32 = 0;
    while (ngpio as usize) < SIFIVE_GPIO_MAX {
        let ret = platform_get_irq_optional(pdev, ngpio as u32);
        if ret < 0 { break; }
        (*chip).irq_number[ngpio as usize] = ret as u32;
        ngpio += 1;
    }
    if ngpio == 0 {
        dev_err(dev, b"no IRQ found\0".as_ptr());
        return -ENODEV;
    }
    // The check above ensures at least one parent IRQ is valid. Assume all
    // parent IRQs belong to the same domain.
    let parent = irq_get_irq_data((*chip).irq_number[0]);
    let _parent_domain = (*parent).domain;

    let config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: (*chip).base.add(SIFIVE_GPIO_INPUT_VAL),
        set: (*chip).base.add(SIFIVE_GPIO_OUTPUT_VAL),
        dirout: (*chip).base.add(SIFIVE_GPIO_OUTPUT_EN),
        dirin: (*chip).base.add(SIFIVE_GPIO_INPUT_EN),
        flags: GPIO_GENERIC_READ_OUTPUT_REG_SET,
    };
    let ret = gpio_generic_chip_init(&mut (*chip).gen_gc, &config);
    if ret != 0 {
        dev_err(dev, b"unable to init generic GPIO\0".as_ptr());
        return ret;
    }
    regmap_write((*chip).regs, SIFIVE_GPIO_RISE_IE as u32, 0);
    regmap_write((*chip).regs, SIFIVE_GPIO_FALL_IE as u32, 0);
    regmap_write((*chip).regs, SIFIVE_GPIO_HIGH_IE as u32, 0);
    regmap_write((*chip).regs, SIFIVE_GPIO_LOW_IE as u32, 0);
    (*chip).irq_state = 0;
    (*chip).gen_gc.gc.base = -1;
    (*chip).gen_gc.gc.ngpio = ngpio as u32;
    (*chip).gen_gc.gc.label = dev_name(dev);
    (*chip).gen_gc.gc.parent = dev;
    (*chip).gen_gc.gc.owner = THIS_MODULE;
    let girq = &mut (*chip).gen_gc.gc.irq;
    gpio_irq_chip_set_chip(girq, &SIFIVE_GPIO_IRQCHIP);
    (*girq).fwnode = dev_fwnode(dev);
    (*girq).parent_domain = (*parent).domain;
    (*girq).child_to_parent_hwirq = Some(sifive_gpio_child_to_parent_hwirq);
    (*girq).handler = handle_bad_irq;
    (*girq).default_type = IRQ_TYPE_NONE;
    gpiochip_add_data(&mut (*chip).gen_gc.gc, chip)
}

static SIFIVE_GPIO_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, disable_locking: true,
};

// MODULE_DEVICE_TABLE(of, sifive_gpio_match);
// module_platform_driver(sifive_gpio_driver)
// MODULE_AUTHOR("Yash Shah <yash.shah@sifive.com>");
// MODULE_DESCRIPTION("SiFive GPIO driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
