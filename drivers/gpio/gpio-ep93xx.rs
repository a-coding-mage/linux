// SPDX-License-Identifier: GPL-2.0
/*
 * Generic EP93xx GPIO handling
 *
 * Copyright (c) 2008 Ryan Mallon
 * Copyright (c) 2011 H Hartley Sweeten <hsweeten@visionengravers.com>
 *
 * Based on code originally from:
 *  linux/arch/arm/mach-ep93xx/core.c
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct ep93xx_gpio_irq_chip {
    base: *mut core::ffi::c_void,
    int_unmasked: u8,
    int_enabled: u8,
    int_type1: u8,
    int_type2: u8,
    int_debounce: u8,
}

#[repr(C)]
struct ep93xx_gpio_chip {
    base: *mut core::ffi::c_void,
    chip: gpio_generic_chip,
    eic: *mut ep93xx_gpio_irq_chip,
}

unsafe fn to_ep93xx_gpio_chip(gc: *mut gpio_chip) -> *mut ep93xx_gpio_chip {
    container_of(to_gpio_generic_chip(gc), core::ptr::null_mut::<ep93xx_gpio_chip>())
}

unsafe fn to_ep93xx_gpio_irq_chip(gc: *mut gpio_chip) -> *mut ep93xx_gpio_irq_chip {
    (*to_ep93xx_gpio_chip(gc)).eic
}

/* Interrupt handling for EP93xx on-chip GPIOs */
const EP93XX_INT_TYPE1_OFFSET: usize = 0x00;
const EP93XX_INT_TYPE2_OFFSET: usize = 0x04;
const EP93XX_INT_EOI_OFFSET: usize = 0x08;
const EP93XX_INT_EN_OFFSET: usize = 0x0c;
const EP93XX_INT_STATUS_OFFSET: usize = 0x10;
const EP93XX_INT_RAW_STATUS_OFFSET: usize = 0x14;
const EP93XX_INT_DEBOUNCE_OFFSET: usize = 0x18;

unsafe fn ep93xx_gpio_update_int_params(eic: *mut ep93xx_gpio_irq_chip) {
    writeb_relaxed(0, (*eic).base.add(EP93XX_INT_EN_OFFSET));
    writeb_relaxed((*eic).int_type2, (*eic).base.add(EP93XX_INT_TYPE2_OFFSET));
    writeb_relaxed((*eic).int_type1, (*eic).base.add(EP93XX_INT_TYPE1_OFFSET));
    writeb_relaxed((*eic).int_unmasked & (*eic).int_enabled,
                   (*eic).base.add(EP93XX_INT_EN_OFFSET));
}

unsafe fn ep93xx_gpio_int_debounce(gc: *mut gpio_chip, offset: c_uint, enable: bool) {
    let eic = to_ep93xx_gpio_irq_chip(gc);
    let port_mask = BIT(offset);
    if enable { (*eic).int_debounce |= port_mask; }
    else { (*eic).int_debounce &= !port_mask; }
    writeb((*eic).int_debounce, (*eic).base.add(EP93XX_INT_DEBOUNCE_OFFSET));
}

unsafe fn ep93xx_gpio_ab_irq_handler(gc: *mut gpio_chip) -> u32 {
    let eic = to_ep93xx_gpio_irq_chip(gc);
    let stat = readb((*eic).base.add(EP93XX_INT_STATUS_OFFSET)) as c_ulong;
    for_each_set_bit!(offset, &stat, 8, {
        generic_handle_domain_irq((*gc).irq.domain, offset);
    });
    stat as u32
}

unsafe extern "C" fn ep93xx_ab_irq_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    IRQ_RETVAL(ep93xx_gpio_ab_irq_handler(dev_id as *mut gpio_chip))
}

unsafe extern "C" fn ep93xx_gpio_f_irq_handler(desc: *mut irq_desc) {
    let irqchip = irq_desc_get_chip(desc);
    let gc = irq_desc_get_handler_data(desc) as *mut gpio_chip;
    let gic = &mut (*gc).irq;
    let parent = irq_desc_get_irq(desc);
    let mut i = 0;
    chained_irq_enter(irqchip, desc);
    while i < gic.num_parents {
        if *gic.parents.add(i) == parent { break; }
        i += 1;
    }
    if i < gic.num_parents { generic_handle_domain_irq(gic.domain, i); }
    chained_irq_exit(irqchip, desc);
}

unsafe extern "C" fn ep93xx_gpio_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let eic = to_ep93xx_gpio_irq_chip(gc);
    let port_mask = BIT(irqd_to_hwirq(d));
    if irqd_get_trigger_type(d) == IRQ_TYPE_EDGE_BOTH {
        (*eic).int_type2 ^= port_mask;
        ep93xx_gpio_update_int_params(eic);
    }
    writeb(port_mask, (*eic).base.add(EP93XX_INT_EOI_OFFSET));
}

unsafe extern "C" fn ep93xx_gpio_irq_mask_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let eic = to_ep93xx_gpio_irq_chip(gc);
    let hwirq = irqd_to_hwirq(d);
    let port_mask = BIT(hwirq);
    if irqd_get_trigger_type(d) == IRQ_TYPE_EDGE_BOTH { (*eic).int_type2 ^= port_mask; }
    (*eic).int_unmasked &= !port_mask;
    ep93xx_gpio_update_int_params(eic);
    writeb(port_mask, (*eic).base.add(EP93XX_INT_EOI_OFFSET));
    gpiochip_disable_irq(gc, hwirq);
}

unsafe extern "C" fn ep93xx_gpio_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let eic = to_ep93xx_gpio_irq_chip(gc);
    let hwirq = irqd_to_hwirq(d);
    (*eic).int_unmasked &= !BIT(hwirq);
    ep93xx_gpio_update_int_params(eic);
    gpiochip_disable_irq(gc, hwirq);
}

unsafe extern "C" fn ep93xx_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let eic = to_ep93xx_gpio_irq_chip(gc);
    let hwirq = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, hwirq);
    (*eic).int_unmasked |= BIT(hwirq);
    ep93xx_gpio_update_int_params(eic);
}

/* gpio_int_type1 selects level (0) or edge (1); gpio_int_type2 selects low/falling (0) or high/rising (1). */
unsafe extern "C" fn ep93xx_gpio_irq_type(d: *mut irq_data, type_: c_uint) -> c_int {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let eic = to_ep93xx_gpio_irq_chip(gc);
    let hwirq = irqd_to_hwirq(d);
    let port_mask = BIT(hwirq);
    (*gc).direction_input.unwrap()(gc, hwirq);
    let handler;
    match type_ {
        IRQ_TYPE_EDGE_RISING => { (*eic).int_type1 |= port_mask; (*eic).int_type2 |= port_mask; handler = handle_edge_irq; }
        IRQ_TYPE_EDGE_FALLING => { (*eic).int_type1 |= port_mask; (*eic).int_type2 &= !port_mask; handler = handle_edge_irq; }
        IRQ_TYPE_LEVEL_HIGH => { (*eic).int_type1 &= !port_mask; (*eic).int_type2 |= port_mask; handler = handle_level_irq; }
        IRQ_TYPE_LEVEL_LOW => { (*eic).int_type1 &= !port_mask; (*eic).int_type2 &= !port_mask; handler = handle_level_irq; }
        IRQ_TYPE_EDGE_BOTH => {
            (*eic).int_type1 |= port_mask;
            if (*gc).get.unwrap()(gc, hwirq) != 0 { (*eic).int_type2 &= !port_mask; } else { (*eic).int_type2 |= port_mask; }
            handler = handle_edge_irq;
        }
        _ => return -EINVAL,
    }
    irq_set_handler_locked(d, handler);
    (*eic).int_enabled |= port_mask;
    ep93xx_gpio_update_int_params(eic);
    0
}

unsafe extern "C" fn ep93xx_gpio_set_config(gc: *mut gpio_chip, offset: c_uint, config: c_ulong) -> c_int {
    if pinconf_to_config_param(config) != PIN_CONFIG_INPUT_DEBOUNCE { return -ENOTSUPP; }
    ep93xx_gpio_int_debounce(gc, offset, pinconf_to_config_argument(config) != 0);
    0
}

unsafe extern "C" fn ep93xx_irq_print_chip(data: *mut irq_data, p: *mut seq_file) {
    let gc = irq_data_get_irq_chip_data(data) as *mut gpio_chip;
    seq_puts(p, dev_name((*gc).parent));
}

static gpio_eic_irq_chip: irq_chip = irq_chip {
    name: "ep93xx-gpio-eic", irq_ack: Some(ep93xx_gpio_irq_ack),
    irq_mask: Some(ep93xx_gpio_irq_mask), irq_unmask: Some(ep93xx_gpio_irq_unmask),
    irq_mask_ack: Some(ep93xx_gpio_irq_mask_ack), irq_set_type: Some(ep93xx_gpio_irq_type),
    irq_print_chip: Some(ep93xx_irq_print_chip), flags: IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS
};

unsafe fn ep93xx_setup_irqs(pdev: *mut platform_device, egc: *mut ep93xx_gpio_chip) -> c_int {
    let gc = &mut (*egc).chip.gc;
    let dev = &mut (*pdev).dev;
    let girq = &mut gc.irq;
    let intr = devm_platform_ioremap_resource_byname(pdev, "intr");
    if IS_ERR(intr) { return PTR_ERR(intr); }
    gc.set_config = Some(ep93xx_gpio_set_config);
    (*egc).eic = devm_kzalloc(dev, core::mem::size_of::<ep93xx_gpio_irq_chip>(), GFP_KERNEL);
    if (*egc).eic.is_null() { return -ENOMEM; }
    (*(*egc).eic).base = intr;
    gpio_irq_chip_set_chip(girq, &gpio_eic_irq_chip);
    girq.num_parents = platform_irq_count(pdev);
    if girq.num_parents == 0 { return -EINVAL; }
    girq.parents = devm_kcalloc(dev, girq.num_parents, core::mem::size_of::<c_uint>(), GFP_KERNEL);
    if girq.parents.is_null() { return -ENOMEM; }
    if girq.num_parents == 1 {
        let irq = platform_get_irq(pdev, 0);
        if irq < 0 { return irq; }
        let ret = devm_request_irq(dev, irq, Some(ep93xx_ab_irq_handler), IRQF_SHARED, gc.label, gc);
        if ret != 0 { return dev_err_probe(dev, ret, "requesting IRQ: %d\n", irq); }
        *girq.parents = irq as c_uint;
    } else {
        girq.parent_handler = Some(ep93xx_gpio_f_irq_handler);
        for i in 0..girq.num_parents {
            let irq = platform_get_irq_optional(pdev, i);
            if irq >= 0 { *girq.parents.add(i) = irq as c_uint; }
        }
        girq.map = girq.parents;
    }
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = Some(handle_bad_irq);
    0
}

unsafe extern "C" fn ep93xx_gpio_probe(pdev: *mut platform_device) -> c_int {
    let egc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<ep93xx_gpio_chip>(), GFP_KERNEL) as *mut ep93xx_gpio_chip;
    if egc.is_null() { return -ENOMEM; }
    let data = devm_platform_ioremap_resource_byname(pdev, "data");
    if IS_ERR(data) { return PTR_ERR(data); }
    let dir = devm_platform_ioremap_resource_byname(pdev, "dir");
    if IS_ERR(dir) { return PTR_ERR(dir); }
    let config = gpio_generic_chip_config { dev: &mut (*pdev).dev, sz: 1, dat: data, dirout: dir };
    let ret = gpio_generic_chip_init(&mut (*egc).chip, &config);
    if ret != 0 { return dev_err_probe(&mut (*pdev).dev, ret, "unable to init generic GPIO\n"); }
    let gc = &mut (*egc).chip.gc;
    gc.label = dev_name(&mut (*pdev).dev);
    if platform_irq_count(pdev) > 0 {
        dev_dbg(&mut (*pdev).dev, "setting up irqs for %s\n", dev_name(&mut (*pdev).dev));
        let ret = ep93xx_setup_irqs(pdev, egc);
        if ret != 0 { dev_err_probe(&mut (*pdev).dev, ret, "setup irqs failed"); }
    }
    devm_gpiochip_add_data(&mut (*pdev).dev, gc, egc)
}

static ep93xx_gpio_match: [of_device_id; 2] = [
    of_device_id { compatible: "cirrus,ep9301-gpio" }, of_device_id { /* sentinel */ }
];
static ep93xx_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: "gpio-ep93xx", of_match_table: ep93xx_gpio_match.as_ptr() },
    probe: Some(ep93xx_gpio_probe),
};

unsafe fn ep93xx_gpio_init() -> c_int { platform_driver_register(&ep93xx_gpio_driver) }
postcore_initcall!(ep93xx_gpio_init);
MODULE_AUTHOR!("Ryan Mallon <ryan@bluewatersys.com> H Hartley Sweeten <hsweeten@visionengravers.com>");
MODULE_DESCRIPTION!("EP93XX GPIO driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
