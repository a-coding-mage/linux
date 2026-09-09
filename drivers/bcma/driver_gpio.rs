/*
 * Broadcom specific AMBA
 * GPIO driver
 *
 * Copyright 2011, Broadcom Corporation
 * Copyright 2012, Hauke Mehrtens <hauke@hauke-m.de>
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// External Linux kernel declarations and build-time configuration are supplied by other files.

const BCMA_GPIO_MAX_PINS: u32 = 32;

pub static mut bcma_gpio_swnode: software_node = software_node {
    name: "bcma-gpio",
};

unsafe fn bcma_gpio_get_value(chip: *mut gpio_chip, gpio: u32) -> i32 {
    let cc: *mut bcma_drv_cc = gpiochip_get_data(chip);
    return if bcma_chipco_gpio_in(cc, 1u32 << gpio) != 0 { 1 } else { 0 };
}

unsafe fn bcma_gpio_set_value(chip: *mut gpio_chip, gpio: u32, value: i32) -> i32 {
    let cc: *mut bcma_drv_cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_out(cc, 1u32 << gpio, if value != 0 { 1u32 << gpio } else { 0 });
    0
}

unsafe fn bcma_gpio_direction_input(chip: *mut gpio_chip, gpio: u32) -> i32 {
    let cc: *mut bcma_drv_cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_outen(cc, 1u32 << gpio, 0);
    0
}

unsafe fn bcma_gpio_direction_output(chip: *mut gpio_chip, gpio: u32, value: i32) -> i32 {
    let cc: *mut bcma_drv_cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_outen(cc, 1u32 << gpio, 1u32 << gpio);
    bcma_chipco_gpio_out(cc, 1u32 << gpio, if value != 0 { 1u32 << gpio } else { 0 });
    0
}

unsafe fn bcma_gpio_request(chip: *mut gpio_chip, gpio: u32) -> i32 {
    let cc: *mut bcma_drv_cc = gpiochip_get_data(chip);
    bcma_chipco_gpio_control(cc, 1u32 << gpio, 0);
    /* clear pulldown */
    bcma_chipco_gpio_pulldown(cc, 1u32 << gpio, 0);
    /* Set pullup */
    bcma_chipco_gpio_pullup(cc, 1u32 << gpio, 1u32 << gpio);
    0
}

unsafe fn bcma_gpio_free(chip: *mut gpio_chip, gpio: u32) {
    let cc: *mut bcma_drv_cc = gpiochip_get_data(chip);
    /* clear pullup */
    bcma_chipco_gpio_pullup(cc, 1u32 << gpio, 0);
}

/* Preserved from: #if IS_BUILTIN(CONFIG_BCM47XX) || IS_BUILTIN(CONFIG_ARCH_BCM_5301X) */

unsafe fn bcma_gpio_irq_unmask(d: *mut irq_data) {
    let gc: *mut gpio_chip = irq_data_get_irq_chip_data(d);
    let cc: *mut bcma_drv_cc = gpiochip_get_data(gc);
    let gpio: i32 = irqd_to_hwirq(d) as i32;
    let val: u32 = bcma_chipco_gpio_in(cc, BIT(gpio as u32));
    gpiochip_enable_irq(gc, gpio as u32);
    bcma_chipco_gpio_polarity(cc, BIT(gpio as u32), val);
    bcma_chipco_gpio_intmask(cc, BIT(gpio as u32), BIT(gpio as u32));
}

unsafe fn bcma_gpio_irq_mask(d: *mut irq_data) {
    let gc: *mut gpio_chip = irq_data_get_irq_chip_data(d);
    let cc: *mut bcma_drv_cc = gpiochip_get_data(gc);
    let gpio: i32 = irqd_to_hwirq(d) as i32;
    bcma_chipco_gpio_intmask(cc, BIT(gpio as u32), 0);
    gpiochip_disable_irq(gc, gpio as u32);
}

static mut bcma_gpio_irq_chip: irq_chip = irq_chip {
    name: "BCMA-GPIO",
    irq_mask: Some(bcma_gpio_irq_mask),
    irq_unmask: Some(bcma_gpio_irq_unmask),
    flags: IRQCHIP_IMMUTABLE,
    /* GPIOCHIP_IRQ_RESOURCE_HELPERS */
};

unsafe fn bcma_gpio_irq_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let cc: *mut bcma_drv_cc = dev_id as *mut bcma_drv_cc;
    let gc: *mut gpio_chip = &mut (*cc).gpio;
    let val: u32 = bcma_cc_read32(cc, BCMA_CC_GPIOIN);
    let mask: u32 = bcma_cc_read32(cc, BCMA_CC_GPIOIRQ);
    let pol: u32 = bcma_cc_read32(cc, BCMA_CC_GPIOPOL);
    let irqs: usize = ((val ^ pol) & mask) as usize;
    let _ = irq;
    if irqs == 0 { return IRQ_NONE; }
    for gpio in 0..(*gc).ngpio {
        if (irqs & (1usize << gpio)) != 0 {
            generic_handle_domain_irq_safe((*(*gc).irq).domain, gpio);
        }
    }
    bcma_chipco_gpio_polarity(cc, irqs as u32, val & irqs as u32);
    IRQ_HANDLED
}

unsafe fn bcma_gpio_irq_init(cc: *mut bcma_drv_cc) -> i32 {
    let chip: *mut gpio_chip = &mut (*cc).gpio;
    let girq: *mut gpio_irq_chip = &mut (*chip).irq;
    if (*(*cc).core).bus.hosttype != BCMA_HOSTTYPE_SOC { return 0; }
    let hwirq = bcma_core_irq((*cc).core, 0);
    let err = request_irq(hwirq, Some(bcma_gpio_irq_handler), IRQF_SHARED, "gpio", cc as *mut _);
    if err != 0 { return err; }
    bcma_chipco_gpio_intmask(cc, !0u32, 0);
    bcma_cc_set32(cc, BCMA_CC_IRQMASK, BCMA_CC_IRQ_GPIO);
    gpio_irq_chip_set_chip(girq, &mut bcma_gpio_irq_chip);
    /* This will let us handle the parent IRQ in the driver */
    (*girq).parent_handler = None;
    (*girq).num_parents = 0;
    (*girq).parents = core::ptr::null_mut();
    (*girq).default_type = IRQ_TYPE_NONE;
    (*girq).handler = Some(handle_simple_irq);
    0
}

unsafe fn bcma_gpio_irq_exit(cc: *mut bcma_drv_cc) {
    if (*(*cc).core).bus.hosttype != BCMA_HOSTTYPE_SOC { return; }
    bcma_cc_mask32(cc, BCMA_CC_IRQMASK, !BCMA_CC_IRQ_GPIO);
    free_irq(bcma_core_irq((*cc).core, 0), cc as *mut _);
}

pub unsafe fn bcma_gpio_init(cc: *mut bcma_drv_cc) -> i32 {
    let bus: *mut bcma_bus = (*(*cc).core).bus;
    let chip: *mut gpio_chip = &mut (*cc).gpio;
    (*chip).label = "bcma_gpio";
    (*chip).owner = THIS_MODULE;
    (*chip).request = Some(bcma_gpio_request);
    (*chip).free = Some(bcma_gpio_free);
    (*chip).get = Some(bcma_gpio_get_value);
    (*chip).set = Some(bcma_gpio_set_value);
    (*chip).direction_input = Some(bcma_gpio_direction_input);
    (*chip).direction_output = Some(bcma_gpio_direction_output);
    (*chip).parent = (*bus).dev;
    let mut err: i32;
    /* Register software node only for the host SoC bus, unless there is already a firmware node assigned. */
    if (*bus).hosttype == BCMA_HOSTTYPE_SOC && !dev_fwnode(&(*(*cc).core).dev) {
        err = software_node_register(&mut bcma_gpio_swnode);
        if err != 0 { return err; }
        (*chip).fwnode = software_node_fwnode(&bcma_gpio_swnode);
    } else { (*chip).fwnode = dev_fwnode(&(*(*cc).core).dev); }
    (*chip).ngpio = match (*bus).chipinfo.id {
        BCMA_CHIP_ID_BCM4707 | BCMA_CHIP_ID_BCM5357 | BCMA_CHIP_ID_BCM53572 |
        BCMA_CHIP_ID_BCM53573 | BCMA_CHIP_ID_BCM47094 => 32,
        _ => 16,
    };
    if IS_BUILTIN(CONFIG_BCM47XX) || (*(*cc).core).bus.hosttype == BCMA_HOSTTYPE_SOC {
        (*chip).base = (*bus).num * BCMA_GPIO_MAX_PINS as i32;
    } else { (*chip).base = -1; }
    err = bcma_gpio_irq_init(cc);
    if err != 0 { return err; }
    err = gpiochip_add_data(chip, cc as *mut _);
    if err != 0 { bcma_gpio_irq_exit(cc); return err; }
    0
}

pub unsafe fn bcma_gpio_unregister(cc: *mut bcma_drv_cc) -> i32 {
    bcma_gpio_irq_exit(cc);
    gpiochip_remove(&mut (*cc).gpio);
    if (*(*cc).core).bus.hosttype == BCMA_HOSTTYPE_SOC && !(*cc).gpio.fwnode.is_null() && is_software_node((*cc).gpio.fwnode) {
        software_node_unregister(&bcma_gpio_swnode);
        (*cc).gpio.fwnode = core::ptr::null_mut();
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
