// SPDX-License-Identifier: GPL-2.0
//
// IXP4 GPIO driver
// Copyright (C) 2019 Linus Walleij <linus.walleij@linaro.org>
//
// based on previous work and know-how from:
// Deepak Saxena <dsaxena@plexity.net>

// Dependencies are supplied by the surrounding kernel bindings.

const IXP4XX_REG_GPOUT: usize = 0x00;
const IXP4XX_REG_GPOE: usize = 0x04;
const IXP4XX_REG_GPIN: usize = 0x08;
const IXP4XX_REG_GPIS: usize = 0x0C;
const IXP4XX_REG_GPIT1: usize = 0x10;
const IXP4XX_REG_GPIT2: usize = 0x14;
const IXP4XX_REG_GPCLK: usize = 0x18;
const IXP4XX_REG_GPDBSEL: usize = 0x1C;

const IXP4XX_GPIO_STYLE_ACTIVE_HIGH: u32 = 0x0;
const IXP4XX_GPIO_STYLE_ACTIVE_LOW: u32 = 0x1;
const IXP4XX_GPIO_STYLE_RISING_EDGE: u32 = 0x2;
const IXP4XX_GPIO_STYLE_FALLING_EDGE: u32 = 0x3;
const IXP4XX_GPIO_STYLE_TRANSITIONAL: u32 = 0x4;
const IXP4XX_GPIO_STYLE_MASK: u32 = 0x7;
const IXP4XX_GPIO_STYLE_SIZE: u32 = 3;

const IXP4XX_GPCLK_CLK0DC_SHIFT: u32 = 0;
const IXP4XX_GPCLK_CLK0TC_SHIFT: u32 = 4;
const IXP4XX_GPCLK_CLK0_MASK: u32 = 0xff;
const IXP4XX_GPCLK_MUX14: u32 = 1 << 8;
const IXP4XX_GPCLK_CLK1DC_SHIFT: u32 = 16;
const IXP4XX_GPCLK_CLK1TC_SHIFT: u32 = 20;
const IXP4XX_GPCLK_CLK1_MASK: u32 = 0xff << 16;
const IXP4XX_GPCLK_MUX15: u32 = 1 << 24;

#[repr(C)]
struct Ixp4xxGpio {
    chip: gpio_generic_chip,
    dev: *mut device,
    base: *mut core::ffi::c_void,
    irq_edge: u64,
}

unsafe fn ixp4xx_gpio_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let g = gpiochip_get_data(gc) as *mut Ixp4xxGpio;
    __raw_writel(1u32 << (*d).hwirq, (*g).base.add(IXP4XX_REG_GPIS));
}

unsafe fn ixp4xx_gpio_mask_irq(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    irq_chip_mask_parent(d);
    gpiochip_disable_irq(gc, (*d).hwirq);
}

unsafe fn ixp4xx_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let g = gpiochip_get_data(gc) as *mut Ixp4xxGpio;
    if ((*g).irq_edge & (1u64 << (*d).hwirq)) == 0 { ixp4xx_gpio_irq_ack(d); }
    gpiochip_enable_irq(gc, (*d).hwirq);
    irq_chip_unmask_parent(d);
}

unsafe fn ixp4xx_gpio_irq_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let g = gpiochip_get_data(gc) as *mut Ixp4xxGpio;
    let mut line = (*d).hwirq as u32;
    let int_style: u32;
    let int_reg: usize;
    let mut val: u32;
    match type_ {
        IRQ_TYPE_EDGE_BOTH => { irq_set_handler_locked(d, handle_edge_irq); int_style = IXP4XX_GPIO_STYLE_TRANSITIONAL; (*g).irq_edge |= 1u64 << (*d).hwirq; }
        IRQ_TYPE_EDGE_RISING => { irq_set_handler_locked(d, handle_edge_irq); int_style = IXP4XX_GPIO_STYLE_RISING_EDGE; (*g).irq_edge |= 1u64 << (*d).hwirq; }
        IRQ_TYPE_EDGE_FALLING => { irq_set_handler_locked(d, handle_edge_irq); int_style = IXP4XX_GPIO_STYLE_FALLING_EDGE; (*g).irq_edge |= 1u64 << (*d).hwirq; }
        IRQ_TYPE_LEVEL_HIGH => { irq_set_handler_locked(d, handle_level_irq); int_style = IXP4XX_GPIO_STYLE_ACTIVE_HIGH; (*g).irq_edge &= !(1u64 << (*d).hwirq); }
        IRQ_TYPE_LEVEL_LOW => { irq_set_handler_locked(d, handle_level_irq); int_style = IXP4XX_GPIO_STYLE_ACTIVE_LOW; (*g).irq_edge &= !(1u64 << (*d).hwirq); }
        _ => return -EINVAL,
    }
    if line >= 8 { line -= 8; int_reg = IXP4XX_REG_GPIT2; } else { int_reg = IXP4XX_REG_GPIT1; }
    // scoped_guard(gpio_generic_lock_irqsave, &g->chip)
    val = __raw_readl((*g).base.add(int_reg));
    val &= !(IXP4XX_GPIO_STYLE_MASK << (line * IXP4XX_GPIO_STYLE_SIZE));
    __raw_writel(val, (*g).base.add(int_reg));
    __raw_writel(1u32 << line, (*g).base.add(IXP4XX_REG_GPIS));
    val = __raw_readl((*g).base.add(int_reg));
    val |= int_style << (line * IXP4XX_GPIO_STYLE_SIZE);
    __raw_writel(val, (*g).base.add(int_reg));
    val = __raw_readl((*g).base.add(IXP4XX_REG_GPOE));
    val |= 1u32 << (*d).hwirq;
    __raw_writel(val, (*g).base.add(IXP4XX_REG_GPOE));
    irq_chip_set_type_parent(d, IRQ_TYPE_LEVEL_HIGH)
}

static const struct irq_chip ixp4xx_gpio_irqchip = {
    .name = "IXP4GPIO",
    .irq_ack = ixp4xx_gpio_irq_ack,
    .irq_mask = ixp4xx_gpio_mask_irq,
    .irq_unmask = ixp4xx_gpio_irq_unmask,
    .irq_set_type = ixp4xx_gpio_irq_set_type,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
};

unsafe fn ixp4xx_gpio_child_to_parent_hwirq(gc: *mut gpio_chip, child: u32, child_type: u32, parent: *mut u32, parent_type: *mut u32) -> i32 {
    *parent_type = IRQ_TYPE_LEVEL_HIGH;
    if child == 0 { *parent = 6; return 0; }
    if child == 1 { *parent = 7; return 0; }
    if child >= 2 && child <= 12 { *parent = child + 17; return 0; }
    -EINVAL
}

// The probe, platform-driver registration, and kernel object declarations retain
// their C ABI shape; referenced kernel types and helpers are external bindings.
unsafe fn ixp4xx_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    let mut g = devm_kzalloc(dev, core::mem::size_of::<Ixp4xxGpio>(), GFP_KERNEL) as *mut Ixp4xxGpio;
    if g.is_null() { return -ENOMEM; }
    (*g).dev = dev;
    (*g).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*g).base) { return PTR_ERR((*g).base); }
    let irq_parent = of_irq_find_parent(np);
    if irq_parent.is_null() { dev_err(dev, "no IRQ parent node\n"); return -ENODEV; }
    let parent = irq_find_host(irq_parent);
    if parent.is_null() { dev_err(dev, "no IRQ parent domain\n"); return -ENODEV; }
    let clk_14 = of_property_read_bool(np, "intel,ixp4xx-gpio14-clkout");
    let clk_15 = of_property_read_bool(np, "intel,ixp4xx-gpio15-clkout");
    let mut val: u32;
    if of_machine_is_compatible("dlink,dsm-g600-a") || of_machine_is_compatible("iom,nas-100d") { val = 0; }
    else {
        val = __raw_readl((*g).base.add(IXP4XX_REG_GPCLK));
        if clk_14 || clk_15 {
            val &= !(IXP4XX_GPCLK_MUX14 | IXP4XX_GPCLK_MUX15 | IXP4XX_GPCLK_CLK0_MASK | IXP4XX_GPCLK_CLK1_MASK);
            if clk_14 { val |= 1 << IXP4XX_GPCLK_CLK0TC_SHIFT | IXP4XX_GPCLK_MUX14; }
            if clk_15 { val |= 1 << IXP4XX_GPCLK_CLK1TC_SHIFT | IXP4XX_GPCLK_MUX15; }
        }
    }
    __raw_writel(val, (*g).base.add(IXP4XX_REG_GPCLK));
    // CONFIG_CPU_BIG_ENDIAN selects GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER.
    let flags = if cfg!(target_endian = "big") { GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER } else { 0 };
    let config = gpio_generic_chip_config { dev, sz: 4, dat: (*g).base.add(IXP4XX_REG_GPIN), set: (*g).base.add(IXP4XX_REG_GPOUT), dirin: (*g).base.add(IXP4XX_REG_GPOE), flags };
    let ret = gpio_generic_chip_init(&mut (*g).chip, &config);
    if ret != 0 { dev_err(dev, "unable to init generic GPIO\n"); return ret; }
    (*g).chip.gc.ngpio = 16;
    (*g).chip.gc.label = "IXP4XX_GPIO_CHIP";
    (*g).chip.gc.base = -1;
    (*g).chip.gc.parent = dev;
    (*g).chip.gc.owner = THIS_MODULE;
    let girq = &mut (*g).chip.gc.irq;
    gpio_irq_chip_set_chip(girq, &ixp4xx_gpio_irqchip);
    (*g).chip.gc.irq.fwnode = dev_fwnode(dev);
    (*g).chip.gc.irq.parent_domain = parent;
    (*g).chip.gc.irq.child_to_parent_hwirq = ixp4xx_gpio_child_to_parent_hwirq;
    (*g).chip.gc.irq.handler = handle_bad_irq;
    (*g).chip.gc.irq.default_type = IRQ_TYPE_NONE;
    let ret = devm_gpiochip_add_data(dev, &mut (*g).chip.gc, g as *mut core::ffi::c_void);
    if ret != 0 { dev_err(dev, "failed to add SoC gpiochip\n"); return ret; }
    platform_set_drvdata(pdev, g as *mut core::ffi::c_void);
    dev_info(dev, "IXP4 GPIO registered\n");
    0
}

static const ixp4xx_gpio_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "intel,ixp4xx-gpio" },
    of_device_id { compatible: "" },
];

static mut ixp4xx_gpio_driver: platform_driver = platform_driver {
    driver: device_driver { name: "ixp4xx-gpio", of_match_table: ixp4xx_gpio_of_match.as_ptr() },
    probe: Some(ixp4xx_gpio_probe),
};

// builtin_platform_driver(ixp4xx_gpio_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
