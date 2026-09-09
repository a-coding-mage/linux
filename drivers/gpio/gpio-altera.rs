// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Altera Corporation
 * Based on gpio-mpc8xxx.c
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

const ALTERA_GPIO_MAX_NGPIO: u32 = 32;
const ALTERA_GPIO_DATA: usize = 0x0;
const ALTERA_GPIO_DIR: usize = 0x4;
const ALTERA_GPIO_IRQ_MASK: usize = 0x8;
const ALTERA_GPIO_EDGE_CAP: usize = 0xc;

#[repr(C)]
pub struct altera_gpio_chip {
    pub chip: gpio_generic_chip,
    pub regs: *mut core::ffi::c_void,
    pub gpio_lock: raw_spinlock_t,
    pub interrupt_trigger: i32,
}

unsafe fn altera_gpio_irq_unmask(d: *mut irq_data) {
    let gc: *mut gpio_chip = irq_data_get_irq_chip_data(d);
    let altera_gc: *mut altera_gpio_chip = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    let mut intmask: u32;

    gpiochip_enable_irq(gc, irqd_to_hwirq(d));

    raw_spin_lock_irqsave(&mut (*altera_gc).gpio_lock, &mut flags);
    intmask = readl((*altera_gc).regs.add(ALTERA_GPIO_IRQ_MASK));
    /* Set ALTERA_GPIO_IRQ_MASK bit to unmask */
    intmask |= 1u32.wrapping_shl(irqd_to_hwirq(d) as u32);
    writel(intmask, (*altera_gc).regs.add(ALTERA_GPIO_IRQ_MASK));
    raw_spin_unlock_irqrestore(&mut (*altera_gc).gpio_lock, flags);
}

unsafe fn altera_gpio_irq_mask(d: *mut irq_data) {
    let gc: *mut gpio_chip = irq_data_get_irq_chip_data(d);
    let altera_gc: *mut altera_gpio_chip = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    let mut intmask: u32;

    raw_spin_lock_irqsave(&mut (*altera_gc).gpio_lock, &mut flags);
    intmask = readl((*altera_gc).regs.add(ALTERA_GPIO_IRQ_MASK));
    /* Clear ALTERA_GPIO_IRQ_MASK bit to mask */
    intmask &= !(1u32.wrapping_shl(irqd_to_hwirq(d) as u32));
    writel(intmask, (*altera_gc).regs.add(ALTERA_GPIO_IRQ_MASK));
    raw_spin_unlock_irqrestore(&mut (*altera_gc).gpio_lock, flags);

    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
}

/*
 * This controller's IRQ type is synthesized in hardware, so this function
 * just checks if the requested set_type matches the synthesized IRQ type
 */
unsafe fn altera_gpio_irq_set_type(d: *mut irq_data, irq_type: c_uint) -> c_int {
    let gc: *mut gpio_chip = irq_data_get_irq_chip_data(d);
    let altera_gc: *mut altera_gpio_chip = gpiochip_get_data(gc);

    if irq_type == IRQ_TYPE_NONE {
        irq_set_handler_locked(d, handle_bad_irq);
        return 0;
    }
    if irq_type == (*altera_gc).interrupt_trigger as c_uint {
        if irq_type == IRQ_TYPE_LEVEL_HIGH {
            irq_set_handler_locked(d, handle_level_irq);
        } else {
            irq_set_handler_locked(d, handle_simple_irq);
        }
        return 0;
    }
    irq_set_handler_locked(d, handle_bad_irq);
    -EINVAL
}

unsafe fn altera_gpio_irq_startup(d: *mut irq_data) -> c_uint {
    altera_gpio_irq_unmask(d);
    0
}

unsafe fn altera_gpio_irq_edge_handler(desc: *mut irq_desc) {
    let gc: *mut gpio_chip = irq_desc_get_handler_data(desc);
    let altera_gc: *mut altera_gpio_chip = gpiochip_get_data(gc);
    let irqdomain: *mut irq_domain = (*gc).irq.domain;
    let chip: *mut irq_chip = irq_desc_get_chip(desc);
    let mut status: c_ulong;
    let mut i: c_int;

    chained_irq_enter(chip, desc);

    loop {
        status = (readl((*altera_gc).regs.add(ALTERA_GPIO_EDGE_CAP))
            & readl((*altera_gc).regs.add(ALTERA_GPIO_IRQ_MASK))) as c_ulong;
        if status == 0 {
            break;
        }
        writel(status as u32, (*altera_gc).regs.add(ALTERA_GPIO_EDGE_CAP));
        i = 0;
        while (i as u32) < (*gc).ngpio {
            if status & (1u64 << (i as u32)) != 0 {
                generic_handle_domain_irq(irqdomain, i as c_uint);
            }
            i += 1;
        }
    }

    chained_irq_exit(chip, desc);
}

unsafe fn altera_gpio_irq_leveL_high_handler(desc: *mut irq_desc) {
    let gc: *mut gpio_chip = irq_desc_get_handler_data(desc);
    let altera_gc: *mut altera_gpio_chip = gpiochip_get_data(gc);
    let irqdomain: *mut irq_domain = (*gc).irq.domain;
    let chip: *mut irq_chip = irq_desc_get_chip(desc);
    let mut status: c_ulong;
    let mut i: c_int;

    chained_irq_enter(chip, desc);

    status = (readl((*altera_gc).regs.add(ALTERA_GPIO_DATA))
        & readl((*altera_gc).regs.add(ALTERA_GPIO_IRQ_MASK))) as c_ulong;

    i = 0;
    while (i as u32) < (*gc).ngpio {
        if status & (1u64 << (i as u32)) != 0 {
            generic_handle_domain_irq(irqdomain, i as c_uint);
        }
        i += 1;
    }

    chained_irq_exit(chip, desc);
}

static altera_gpio_irq_chip: irq_chip = irq_chip {
    name: "altera-gpio",
    irq_mask: Some(altera_gpio_irq_mask),
    irq_unmask: Some(altera_gpio_irq_unmask),
    irq_set_type: Some(altera_gpio_irq_set_type),
    irq_startup: Some(altera_gpio_irq_startup),
    irq_shutdown: Some(altera_gpio_irq_mask),
    flags: IRQCHIP_IMMUTABLE,
    ..GPIOCHIP_IRQ_RESOURCE_HELPERS
};

unsafe fn altera_gpio_probe(pdev: *mut platform_device) -> c_int {
    let mut config: gpio_generic_chip_config;
    let dev: *mut device = &mut (*pdev).dev;
    let mut reg: c_int;
    let mut ret: c_int;
    let altera_gc: *mut altera_gpio_chip;
    let chip: *mut gpio_generic_chip;
    let gc: *mut gpio_chip;
    let girq: *mut gpio_irq_chip;
    let mapped_irq: c_int;

    altera_gc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<altera_gpio_chip>(), GFP_KERNEL);
    if altera_gc.is_null() {
        return -ENOMEM;
    }

    raw_spin_lock_init(&mut (*altera_gc).gpio_lock);

    (*altera_gc).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*altera_gc).regs) {
        return dev_err_probe(dev, PTR_ERR((*altera_gc).regs), "failed to ioremap memory resource\n");
    }

    chip = &mut (*altera_gc).chip;

    config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: (*altera_gc).regs.add(ALTERA_GPIO_DATA),
        set: (*altera_gc).regs.add(ALTERA_GPIO_DATA),
        dirout: (*altera_gc).regs.add(ALTERA_GPIO_DIR),
    };

    ret = gpio_generic_chip_init(chip, &mut config);
    if ret != 0 {
        return dev_err_probe(dev, ret, "unable to init generic GPIO\n");
    }

    gc = &mut (*chip).gc;

    if device_property_read_u32(dev, "altr,ngpio", &mut reg) != 0 {
        /* By default assume maximum ngpio */
        (*gc).ngpio = ALTERA_GPIO_MAX_NGPIO;
    } else {
        (*gc).ngpio = reg as u32;
    }

    if (*gc).ngpio > ALTERA_GPIO_MAX_NGPIO {
        dev_warn(&mut (*pdev).dev, "ngpio is greater than %d, defaulting to %d\n", ALTERA_GPIO_MAX_NGPIO, ALTERA_GPIO_MAX_NGPIO);
        (*gc).ngpio = ALTERA_GPIO_MAX_NGPIO;
    }

    (*gc).base = -1;
    (*gc).label = devm_kasprintf(dev, GFP_KERNEL, "%pfw", dev_fwnode(dev));
    if (*gc).label.is_null() {
        return -ENOMEM;
    }

    mapped_irq = platform_get_irq_optional(pdev, 0);
    if mapped_irq < 0 {
        goto skip_irq;
    }

    if device_property_read_u32(dev, "altr,interrupt-type", &mut reg) != 0 {
        dev_err(&mut (*pdev).dev, "altr,interrupt-type value not set in device tree\n");
        return -EINVAL;
    }
    (*altera_gc).interrupt_trigger = reg;

    girq = &mut (*gc).irq;
    gpio_irq_chip_set_chip(girq, &altera_gpio_irq_chip);

    if (*altera_gc).interrupt_trigger as c_uint == IRQ_TYPE_LEVEL_HIGH {
        (*girq).parent_handler = Some(altera_gpio_irq_leveL_high_handler);
    } else {
        (*girq).parent_handler = Some(altera_gpio_irq_edge_handler);
    }
    (*girq).num_parents = 1;
    (*girq).parents = devm_kcalloc(&mut (*pdev).dev, 1, core::mem::size_of::<c_uint>(), GFP_KERNEL);
    if (*girq).parents.is_null() {
        return -ENOMEM;
    }
    (*girq).default_type = IRQ_TYPE_NONE;
    (*girq).handler = Some(handle_bad_irq);
    *(*girq).parents = mapped_irq as c_uint;

skip_irq:
    ret = devm_gpiochip_add_data(dev, gc, altera_gc);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "Failed adding memory mapped gpiochip\n");
        return ret;
    }

    0
}

static altera_gpio_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "altr,pio-1.0" },
    of_device_id { ..Default::default() },
];

static mut altera_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "altera_gpio",
        of_match_table: altera_gpio_of_match.as_ptr(),
        ..Default::default()
    },
    probe: Some(altera_gpio_probe),
    ..Default::default()
};

unsafe fn altera_gpio_init() -> c_int {
    platform_driver_register(&mut altera_gpio_driver)
}

unsafe fn altera_gpio_exit() {
    platform_driver_unregister(&mut altera_gpio_driver);
}

// Equivalent registration and module metadata supplied by the kernel build environment.
subsys_initcall!(altera_gpio_init);
module_exit!(altera_gpio_exit);
module_author!("Tien Hock Loh <thloh@altera.com>");
module_description!("Altera GPIO driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
