// SPDX-License-Identifier: GPL-2.0
/*
 * Faraday Technolog FTGPIO010 gpiochip and interrupt routines
 * Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
 *
 * Based on arch/arm/mach-gemini/gpio.c:
 * Copyright (C) 2008-2009 Paulius Zaleckas <paulius.zaleckas@teltonika.lt>
 *
 * Based on plat-mxc/gpio.c:
 * MXC GPIO support. (c) 2008 Daniel Mack <daniel@caiaq.de>
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const GPIO_DATA_OUT: usize = 0x00;
const GPIO_DATA_IN: usize = 0x04;
const GPIO_DIR: usize = 0x08;
const GPIO_BYPASS_IN: usize = 0x0C;
const GPIO_DATA_SET: usize = 0x10;
const GPIO_DATA_CLR: usize = 0x14;
const GPIO_PULL_EN: usize = 0x18;
const GPIO_PULL_TYPE: usize = 0x1C;
const GPIO_INT_EN: usize = 0x20;
const GPIO_INT_STAT_RAW: usize = 0x24;
const GPIO_INT_STAT_MASKED: usize = 0x28;
const GPIO_INT_MASK: usize = 0x2C;
const GPIO_INT_CLR: usize = 0x30;
const GPIO_INT_TYPE: usize = 0x34;
const GPIO_INT_BOTH_EDGE: usize = 0x38;
const GPIO_INT_LEVEL: usize = 0x3C;
const GPIO_DEBOUNCE_EN: usize = 0x40;
const GPIO_DEBOUNCE_PRESCALE: usize = 0x44;

#[repr(C)]
struct FtgpioGpio {
    dev: *mut device,
    chip: gpio_generic_chip,
    base: *mut core::ffi::c_void,
    clk: *mut clk,
}

unsafe fn ftgpio_gpio_ack_irq(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let g = gpiochip_get_data(gc) as *mut FtgpioGpio;
    writel(1u32 << irqd_to_hwirq(d), (*g).base.add(GPIO_INT_CLR));
}

unsafe fn ftgpio_gpio_mask_irq(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let g = gpiochip_get_data(gc) as *mut FtgpioGpio;
    let mut val: u32 = readl((*g).base.add(GPIO_INT_EN));
    val &= !(1u32 << irqd_to_hwirq(d));
    writel(val, (*g).base.add(GPIO_INT_EN));
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
}

unsafe fn ftgpio_gpio_unmask_irq(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let g = gpiochip_get_data(gc) as *mut FtgpioGpio;
    gpiochip_enable_irq(gc, irqd_to_hwirq(d));
    let mut val: u32 = readl((*g).base.add(GPIO_INT_EN));
    val |= 1u32 << irqd_to_hwirq(d);
    writel(val, (*g).base.add(GPIO_INT_EN));
}

unsafe fn ftgpio_gpio_set_irq_type(d: *mut irq_data, irq_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let g = gpiochip_get_data(gc) as *mut FtgpioGpio;
    let mask = 1u32 << irqd_to_hwirq(d);
    let mut reg_type = readl((*g).base.add(GPIO_INT_TYPE));
    let mut reg_level = readl((*g).base.add(GPIO_INT_LEVEL));
    let mut reg_both = readl((*g).base.add(GPIO_INT_BOTH_EDGE));
    match irq_type {
        IRQ_TYPE_EDGE_BOTH => { irq_set_handler_locked(d, handle_edge_irq); reg_type &= !mask; reg_both |= mask; },
        IRQ_TYPE_EDGE_RISING => { irq_set_handler_locked(d, handle_edge_irq); reg_type &= !mask; reg_both &= !mask; reg_level &= !mask; },
        IRQ_TYPE_EDGE_FALLING => { irq_set_handler_locked(d, handle_edge_irq); reg_type &= !mask; reg_both &= !mask; reg_level |= mask; },
        IRQ_TYPE_LEVEL_HIGH => { irq_set_handler_locked(d, handle_level_irq); reg_type |= mask; reg_level &= !mask; },
        IRQ_TYPE_LEVEL_LOW => { irq_set_handler_locked(d, handle_level_irq); reg_type |= mask; reg_level |= mask; },
        _ => { irq_set_handler_locked(d, handle_bad_irq); return -EINVAL; }
    }
    writel(reg_type, (*g).base.add(GPIO_INT_TYPE));
    writel(reg_level, (*g).base.add(GPIO_INT_LEVEL));
    writel(reg_both, (*g).base.add(GPIO_INT_BOTH_EDGE));
    ftgpio_gpio_ack_irq(d);
    0
}

unsafe fn ftgpio_gpio_irq_handler(desc: *mut irq_desc) {
    let gc = irq_desc_get_handler_data(desc);
    let g = gpiochip_get_data(gc) as *mut FtgpioGpio;
    let irqchip = irq_desc_get_chip(desc);
    chained_irq_enter(irqchip, desc);
    let stat: usize = readl((*g).base.add(GPIO_INT_STAT_RAW)) as usize;
    let mut offset = 0;
    while offset < (*gc).ngpio && stat != 0 {
        if stat & (1usize << offset) != 0 { generic_handle_domain_irq((*gc).irq.domain, offset); }
        offset += 1;
    }
    chained_irq_exit(irqchip, desc);
}

unsafe fn ftgpio_gpio_set_config(gc: *mut gpio_chip, offset: u32, config: usize) -> i32 {
    let param = pinconf_to_config_param(config);
    let arg = pinconf_to_config_argument(config) as u32;
    let g = gpiochip_get_data(gc) as *mut FtgpioGpio;
    if param != PIN_CONFIG_INPUT_DEBOUNCE { return -ENOTSUPP; }
    let pclk_freq = clk_get_rate((*g).clk);
    let deb_div = DIV_ROUND_CLOSEST(pclk_freq, arg as _);
    if deb_div > (1u64 << 24) { return -ENOTSUPP; }
    dev_dbg((*g).dev, "prescale divisor: %08x, resulting frequency %lu Hz\n", deb_div, pclk_freq / deb_div);
    let mut val = readl((*g).base.add(GPIO_DEBOUNCE_PRESCALE));
    if val == deb_div as u32 {
        val = readl((*g).base.add(GPIO_DEBOUNCE_EN)); val |= 1u32 << offset;
        writel(val, (*g).base.add(GPIO_DEBOUNCE_EN)); return 0;
    }
    val = readl((*g).base.add(GPIO_DEBOUNCE_EN));
    if val != 0 { return -ENOTSUPP; }
    writel(deb_div as u32, (*g).base.add(GPIO_DEBOUNCE_PRESCALE));
    val |= 1u32 << offset; writel(val, (*g).base.add(GPIO_DEBOUNCE_EN)); 0
}

// Remaining driver registration and initialization declarations are preserved as kernel-facing Rust items.
static FTGPIO_IRQ_CHIP: irq_chip = irq_chip { name: "FTGPIO010", irq_ack: Some(ftgpio_gpio_ack_irq), irq_mask: Some(ftgpio_gpio_mask_irq), irq_unmask: Some(ftgpio_gpio_unmask_irq), irq_set_type: Some(ftgpio_gpio_set_irq_type), flags: IRQCHIP_IMMUTABLE };

unsafe fn ftgpio_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let g = devm_kzalloc(dev, core::mem::size_of::<FtgpioGpio>(), GFP_KERNEL) as *mut FtgpioGpio;
    if g.is_null() { return -ENOMEM; }
    (*g).dev = dev;
    (*g).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*g).base) { return PTR_ERR((*g).base); }
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    (*g).clk = devm_clk_get_enabled(dev, core::ptr::null());
    if IS_ERR((*g).clk) && PTR_ERR((*g).clk) == -EPROBE_DEFER { return PTR_ERR((*g).clk); }
    let config = gpio_generic_chip_config { dev, sz: 4, dat: (*g).base.add(GPIO_DATA_IN), set: (*g).base.add(GPIO_DATA_SET), clr: (*g).base.add(GPIO_DATA_CLR), dirout: (*g).base.add(GPIO_DIR) };
    let ret = gpio_generic_chip_init(&mut (*g).chip, &config);
    if ret != 0 { return dev_err_probe(dev, ret, "unable to init generic GPIO\n"); }
    (*g).chip.gc.label = dev_name(dev); (*g).chip.gc.base = -1; (*g).chip.gc.parent = dev; (*g).chip.gc.owner = THIS_MODULE;
    if !IS_ERR((*g).clk) { (*g).chip.gc.set_config = Some(ftgpio_gpio_set_config); }
    let girq = &mut (*g).chip.gc.irq;
    gpio_irq_chip_set_chip(girq, &FTGPIO_IRQ_CHIP);
    girq.parent_handler = Some(ftgpio_gpio_irq_handler); girq.num_parents = 1;
    girq.parents = devm_kcalloc(dev, 1, core::mem::size_of::<i32>(), GFP_KERNEL);
    if girq.parents.is_null() { return -ENOMEM; }
    girq.default_type = IRQ_TYPE_NONE; girq.handler = handle_bad_irq; *girq.parents = irq;
    writel(0, (*g).base.add(GPIO_INT_EN)); writel(0, (*g).base.add(GPIO_INT_MASK)); writel(!0u32, (*g).base.add(GPIO_INT_CLR));
    writel(0, (*g).base.add(GPIO_DEBOUNCE_EN));
    devm_gpiochip_add_data(dev, &mut (*g).chip.gc, g)
}

static FTGPIO_GPIO_OF_MATCH: [of_device_id; 4] = [
    of_device_id { compatible: "cortina,gemini-gpio" },
    of_device_id { compatible: "moxa,moxart-gpio" },
    of_device_id { compatible: "faraday,ftgpio010" },
    of_device_id { compatible: core::ptr::null() },
];

static FTGPIO_GPIO_DRIVER: platform_driver = platform_driver {
    driver: device_driver { name: "ftgpio010-gpio", of_match_table: FTGPIO_GPIO_OF_MATCH.as_ptr() },
    probe: Some(ftgpio_gpio_probe),
};

builtin_platform_driver!(FTGPIO_GPIO_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
