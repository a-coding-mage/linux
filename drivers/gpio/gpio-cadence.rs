// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2017-2018 Cadence
 * Copyright (C) 2025 Axiado Corporation.
 *
 * Authors:
 *  Jan Kotas <jank@cadence.com>
 *  Boris Brezillon <boris.brezillon@free-electrons.com>
 */

// Linux kernel dependencies supplied by surrounding bindings.

const CDNS_GPIO_BYPASS_MODE: usize = 0x00;
const CDNS_GPIO_DIRECTION_MODE: usize = 0x04;
const CDNS_GPIO_OUTPUT_EN: usize = 0x08;
const CDNS_GPIO_OUTPUT_VALUE: usize = 0x0c;
const CDNS_GPIO_INPUT_VALUE: usize = 0x10;
const CDNS_GPIO_IRQ_MASK: usize = 0x14;
const CDNS_GPIO_IRQ_EN: usize = 0x18;
const CDNS_GPIO_IRQ_DIS: usize = 0x1c;
const CDNS_GPIO_IRQ_STATUS: usize = 0x20;
const CDNS_GPIO_IRQ_TYPE: usize = 0x24;
const CDNS_GPIO_IRQ_VALUE: usize = 0x28;
const CDNS_GPIO_IRQ_ANY_EDGE: usize = 0x2c;

#[repr(C)]
pub struct cdns_gpio_quirks { pub skip_init: bool }

#[repr(C)]
pub struct cdns_gpio_chip {
    pub gen_gc: gpio_generic_chip,
    pub regs: *mut core::ffi::c_void,
    pub bypass_orig: u32,
    pub quirks: *const cdns_gpio_quirks,
}

static CDNS_DEFAULT_QUIRKS: cdns_gpio_quirks = cdns_gpio_quirks { skip_init: false };
static AX3000_GPIO_QUIRKS: cdns_gpio_quirks = cdns_gpio_quirks { skip_init: true };

unsafe fn cdns_gpio_request(chip: *mut gpio_chip, offset: u32) -> i32 {
    let cgpio = gpiochip_get_data(chip);
    let _guard = gpio_generic_lock_guard((*cgpio).gen_gc);
    let v = ioread32((*cgpio).regs.add(CDNS_GPIO_BYPASS_MODE)) & !(1u32 << offset);
    iowrite32(v, (*cgpio).regs.add(CDNS_GPIO_BYPASS_MODE)); 0
}

unsafe fn cdns_gpio_free(chip: *mut gpio_chip, offset: u32) {
    let cgpio = gpiochip_get_data(chip);
    let _guard = gpio_generic_lock_guard((*cgpio).gen_gc);
    let v = ioread32((*cgpio).regs.add(CDNS_GPIO_BYPASS_MODE))
        | ((1u32 << offset) & (*cgpio).bypass_orig);
    iowrite32(v, (*cgpio).regs.add(CDNS_GPIO_BYPASS_MODE));
}

unsafe fn cdns_gpio_irq_mask(d: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(d); let cgpio = gpiochip_get_data(chip);
    iowrite32(1u32 << (*d).hwirq, (*cgpio).regs.add(CDNS_GPIO_IRQ_DIS));
    gpiochip_disable_irq(chip, irqd_to_hwirq(d));
}

unsafe fn cdns_gpio_irq_unmask(d: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(d); let cgpio = gpiochip_get_data(chip);
    gpiochip_enable_irq(chip, irqd_to_hwirq(d));
    iowrite32(1u32 << (*d).hwirq, (*cgpio).regs.add(CDNS_GPIO_IRQ_EN));
}

unsafe fn cdns_gpio_irq_set_type(d: *mut irq_data, irq_type: u32) -> i32 {
    let chip = irq_data_get_irq_chip_data(d); let cgpio = gpiochip_get_data(chip);
    let _guard = gpio_generic_lock_guard((*cgpio).gen_gc);
    let mask = 1u32 << (*d).hwirq;
    let mut value = ioread32((*cgpio).regs.add(CDNS_GPIO_IRQ_VALUE)) & !mask;
    let mut int_type = ioread32((*cgpio).regs.add(CDNS_GPIO_IRQ_TYPE)) & !mask;
    /*
     * Interrupt polarity and trigger behaviour:
     * (0, 0) falling edge, (0, 1) rising edge,
     * (1, 0) low level, (1, 1) high level.
     */
    let mut any = ioread32((*cgpio).regs.add(CDNS_GPIO_IRQ_ANY_EDGE)) & !mask;
    if irq_type == IRQ_TYPE_LEVEL_HIGH { int_type |= mask; value |= mask; }
    else if irq_type == IRQ_TYPE_LEVEL_LOW { int_type |= mask; }
    else if irq_type == IRQ_TYPE_EDGE_RISING { value |= mask; }
    else if irq_type == IRQ_TYPE_EDGE_FALLING { /* value remains cleared */ }
    else if irq_type == IRQ_TYPE_EDGE_BOTH { any |= mask; }
    else { return -EINVAL; }
    iowrite32(value, (*cgpio).regs.add(CDNS_GPIO_IRQ_VALUE));
    iowrite32(int_type, (*cgpio).regs.add(CDNS_GPIO_IRQ_TYPE));
    iowrite32(any, (*cgpio).regs.add(CDNS_GPIO_IRQ_ANY_EDGE)); 0
}

unsafe fn cdns_gpio_irq_handler(desc: *mut irq_desc) {
    let chip = irq_desc_get_handler_data(desc); let cgpio = gpiochip_get_data(chip);
    let irqchip = irq_desc_get_chip(desc); chained_irq_enter(irqchip, desc);
    let status = ioread32((*cgpio).regs.add(CDNS_GPIO_IRQ_STATUS))
        & !ioread32((*cgpio).regs.add(CDNS_GPIO_IRQ_MASK));
    for hwirq in 0..(*chip).ngpio {
        if status & (1u32 << hwirq) != 0 {
            generic_handle_domain_irq((*chip).irq.domain, hwirq);
        }
    }
    chained_irq_exit(irqchip, desc);
}

static CDNS_GPIO_IRQCHIP: irq_chip = irq_chip {
    name: "cdns-gpio", irq_mask: Some(cdns_gpio_irq_mask),
    irq_unmask: Some(cdns_gpio_irq_unmask),
    irq_set_type: Some(cdns_gpio_irq_set_type), flags: IRQCHIP_IMMUTABLE,
};

static CDNS_OF_IDS: [of_device_id; 3] = [
    of_device_id { compatible: "axiado,ax3000-gpio", data: &AX3000_GPIO_QUIRKS },
    of_device_id { compatible: "cdns,gpio-r1p02", data: &CDNS_DEFAULT_QUIRKS },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn cdns_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config = gpio_generic_chip_config::default();
    let cgpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<cdns_gpio_chip>(),
        GFP_KERNEL) as *mut cdns_gpio_chip;
    if cgpio.is_null() { return -ENOMEM; }
    (*cgpio).regs = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*cgpio).regs) { return ptr_err((*cgpio).regs); }
    let mut num_gpios = 32u32;
    of_property_read_u32((*pdev).dev.of_node, "ngpios", &mut num_gpios);
    if num_gpios > 32 { dev_err(&(*pdev).dev, "ngpios must be less or equal 32\\n"); return -EINVAL; }
    (*cgpio).quirks = device_get_match_data(&(*pdev).dev);
    if (*cgpio).quirks.is_null() { (*cgpio).quirks = &CDNS_DEFAULT_QUIRKS; }
    let dir_prev = ioread32((*cgpio).regs.add(CDNS_GPIO_DIRECTION_MODE));
    if !(*(*cgpio).quirks).skip_init {
        iowrite32(u32::MAX >> (32 - num_gpios), (*cgpio).regs.add(CDNS_GPIO_DIRECTION_MODE));
    }
    config.dev = &mut (*pdev).dev; config.sz = 4;
    config.dat = (*cgpio).regs.add(CDNS_GPIO_INPUT_VALUE);
    config.set = (*cgpio).regs.add(CDNS_GPIO_OUTPUT_VALUE);
    config.dirin = (*cgpio).regs.add(CDNS_GPIO_DIRECTION_MODE);
    config.flags = GPIO_GENERIC_READ_OUTPUT_REG_SET;
    let mut ret = gpio_generic_chip_init(&mut (*cgpio).gen_gc, &config);
    if ret != 0 { iowrite32(dir_prev, (*cgpio).regs.add(CDNS_GPIO_DIRECTION_MODE)); return ret; }
    (*cgpio).gen_gc.gc.label = dev_name(&(*pdev).dev);
    (*cgpio).gen_gc.gc.ngpio = num_gpios; (*cgpio).gen_gc.gc.parent = &mut (*pdev).dev;
    (*cgpio).gen_gc.gc.base = -1; (*cgpio).gen_gc.gc.owner = THIS_MODULE;
    (*cgpio).gen_gc.gc.request = Some(cdns_gpio_request); (*cgpio).gen_gc.gc.free = Some(cdns_gpio_free);
    let clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if is_err(clk) { ret = ptr_err(clk); iowrite32(dir_prev, (*cgpio).regs.add(CDNS_GPIO_DIRECTION_MODE)); return ret; }
    let irq = platform_get_irq(pdev, 0);
    if irq >= 0 {
        let girq = &mut (*cgpio).gen_gc.gc.irq;
        gpio_irq_chip_set_chip(girq, &CDNS_GPIO_IRQCHIP);
        girq.parent_handler = Some(cdns_gpio_irq_handler); girq.num_parents = 1;
        girq.parents = devm_kcalloc(&mut (*pdev).dev, 1, core::mem::size_of::<i32>(), GFP_KERNEL);
        if girq.parents.is_null() { iowrite32(dir_prev, (*cgpio).regs.add(CDNS_GPIO_DIRECTION_MODE)); return -ENOMEM; }
        girq.parents[0] = irq; girq.default_type = IRQ_TYPE_NONE; girq.handler = Some(handle_level_irq);
    }
    ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*cgpio).gen_gc.gc, cgpio);
    if ret < 0 { iowrite32(dir_prev, (*cgpio).regs.add(CDNS_GPIO_DIRECTION_MODE)); return ret; }
    (*cgpio).bypass_orig = ioread32((*cgpio).regs.add(CDNS_GPIO_BYPASS_MODE));
    if !(*(*cgpio).quirks).skip_init {
        iowrite32(u32::MAX >> (32 - num_gpios), (*cgpio).regs.add(CDNS_GPIO_OUTPUT_EN));
        iowrite32(0, (*cgpio).regs.add(CDNS_GPIO_BYPASS_MODE));
    }
    platform_set_drvdata(pdev, cgpio); 0
}

unsafe fn cdns_gpio_remove(pdev: *mut platform_device) {
    let cgpio = platform_get_drvdata(pdev) as *mut cdns_gpio_chip;
    iowrite32((*cgpio).bypass_orig, (*cgpio).regs.add(CDNS_GPIO_BYPASS_MODE));
}

static mut CDNS_GPIO_DRIVER: platform_driver = platform_driver {
    driver: driver { name: "cdns-gpio", of_match_table: &CDNS_OF_IDS },
    probe: Some(cdns_gpio_probe), remove: Some(cdns_gpio_remove),
};

// module_platform_driver(cdns_gpio_driver);
// MODULE_AUTHOR("Jan Kotas <jank@cadence.com>");
// MODULE_DESCRIPTION("Cadence GPIO driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:cdns-gpio");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
