// SPDX-License-Identifier: GPL-2.0
/*
 * Toshiba Visconti GPIO Support
 *
 * (C) Copyright 2020 Toshiba Electronic Devices & Storage Corporation
 * (C) Copyright 2020 TOSHIBA CORPORATION
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// Linux kernel dependencies supplied by other translation units.

/* register offset */
const GPIO_DIR: usize = 0x00;
const GPIO_IDATA: usize = 0x08;
const GPIO_ODATA: usize = 0x10;
const GPIO_OSET: usize = 0x18;
const GPIO_OCLR: usize = 0x20;
const GPIO_INTMODE: usize = 0x30;

const BASE_HW_IRQ: u32 = 24;

#[repr(C)]
struct visconti_gpio {
    base: *mut core::ffi::c_void,
    lock: spinlock_t, /* protect gpio register */
    chip: gpio_generic_chip,
    dev: *mut device,
}

unsafe fn visconti_gpio_irq_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let priv_ = gpiochip_get_data(gc);
    let offset = irqd_to_hwirq(d) as u32;
    let bit = 1u32.wrapping_shl(offset);
    let mut intc_type = IRQ_TYPE_EDGE_RISING;
    let mut intmode: u32;
    let mut odata: u32;
    let mut ret: i32 = 0;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*priv_).lock, &mut flags);

    odata = readl((*priv_).base.add(GPIO_ODATA));
    intmode = readl((*priv_).base.add(GPIO_INTMODE));

    match type_ {
        IRQ_TYPE_EDGE_RISING => {
            odata &= !bit;
            intmode &= !bit;
        }
        IRQ_TYPE_EDGE_FALLING => {
            odata |= bit;
            intmode &= !bit;
        }
        IRQ_TYPE_EDGE_BOTH => {
            intmode |= bit;
        }
        IRQ_TYPE_LEVEL_HIGH => {
            intc_type = IRQ_TYPE_LEVEL_HIGH;
            odata &= !bit;
            intmode &= !bit;
        }
        IRQ_TYPE_LEVEL_LOW => {
            intc_type = IRQ_TYPE_LEVEL_HIGH;
            odata |= bit;
            intmode &= !bit;
        }
        _ => {
            ret = -EINVAL;
            goto_err: {
                spin_unlock_irqrestore(&mut (*priv_).lock, flags);
                return ret;
            }
        }
    }

    writel(odata, (*priv_).base.add(GPIO_ODATA));
    writel(intmode, (*priv_).base.add(GPIO_INTMODE));
    irq_set_irq_type(offset, intc_type);

    ret = irq_chip_set_type_parent(d, type_);
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
    ret
}

unsafe fn visconti_gpio_child_to_parent_hwirq(
    _gc: *mut gpio_chip,
    child: u32,
    _child_type: u32,
    parent: *mut u32,
    parent_type: *mut u32,
) -> i32 {
    /* Interrupts 0..15 mapped to interrupts 24..39 on the GIC */
    if child < 16 {
        /* All these interrupts are level high in the CPU */
        *parent_type = IRQ_TYPE_LEVEL_HIGH;
        *parent = child + BASE_HW_IRQ;
        return 0;
    }
    -EINVAL
}

unsafe fn visconti_gpio_populate_parent_fwspec(
    chip: *mut gpio_chip,
    gfwspec: *mut gpio_irq_fwspec,
    parent_hwirq: u32,
    parent_type: u32,
) -> i32 {
    let fwspec = &mut (*gfwspec).fwspec;

    fwspec.fwnode = (*(*chip).irq.parent_domain).fwnode;
    fwspec.param_count = 3;
    fwspec.param[0] = 0;
    fwspec.param[1] = parent_hwirq;
    fwspec.param[2] = parent_type;

    0
}

unsafe fn visconti_gpio_mask_irq(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);

    irq_chip_mask_parent(d);
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
}

unsafe fn visconti_gpio_unmask_irq(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);

    gpiochip_enable_irq(gc, irqd_to_hwirq(d));
    irq_chip_unmask_parent(d);
}

unsafe fn visconti_gpio_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    let gc = irq_data_get_irq_chip_data(d);
    let priv_ = gpiochip_get_data(gc);

    seq_puts(p, dev_name((*priv_).dev));
}

static visconti_gpio_irq_chip: irq_chip = irq_chip {
    irq_mask: Some(visconti_gpio_mask_irq),
    irq_unmask: Some(visconti_gpio_unmask_irq),
    irq_eoi: Some(irq_chip_eoi_parent),
    irq_set_type: Some(visconti_gpio_irq_set_type),
    irq_print_chip: Some(visconti_gpio_irq_print_chip),
    flags: IRQCHIP_SET_TYPE_MASKED | IRQCHIP_MASK_ON_SUSPEND | IRQCHIP_IMMUTABLE,
    // GPIOCHIP_IRQ_RESOURCE_HELPERS
};

unsafe fn visconti_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config: gpio_generic_chip_config;
    let dev = &mut (*pdev).dev;
    let mut priv_: *mut visconti_gpio;
    let girq: *mut gpio_irq_chip;
    let mut parent: *mut irq_domain;
    let irq_parent: *mut device_node;
    let mut ret: i32;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<visconti_gpio>(), GFP_KERNEL) as *mut visconti_gpio;
    if priv_.is_null() {
        return -ENOMEM;
    }

    spin_lock_init(&mut (*priv_).lock);
    (*priv_).dev = dev;

    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*priv_).base) {
        return PTR_ERR((*priv_).base);
    }

    irq_parent = of_irq_find_parent((*dev).of_node);
    if irq_parent.is_null() {
        dev_err(dev, "No IRQ parent node\n");
        return -ENODEV;
    }

    parent = irq_find_host(irq_parent);
    of_node_put(irq_parent);
    if parent.is_null() {
        dev_err(dev, "No IRQ parent domain\n");
        return -ENODEV;
    }

    config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: (*priv_).base.add(GPIO_IDATA),
        set: (*priv_).base.add(GPIO_OSET),
        clr: (*priv_).base.add(GPIO_OCLR),
        dirout: (*priv_).base.add(GPIO_DIR),
    };

    ret = gpio_generic_chip_init(&mut (*priv_).chip, &mut config);
    if ret != 0 {
        dev_err(dev, "unable to init generic GPIO\n");
        return ret;
    }

    girq = &mut (*priv_).chip.gc.irq;
    gpio_irq_chip_set_chip(girq, &visconti_gpio_irq_chip);
    (*girq).fwnode = dev_fwnode(dev);
    (*girq).parent_domain = parent;
    (*girq).child_to_parent_hwirq = Some(visconti_gpio_child_to_parent_hwirq);
    (*girq).populate_parent_alloc_arg = Some(visconti_gpio_populate_parent_fwspec);
    (*girq).default_type = IRQ_TYPE_NONE;
    (*girq).handler = Some(handle_level_irq);

    devm_gpiochip_add_data(dev, &mut (*priv_).chip.gc, priv_)
}

static visconti_gpio_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "toshiba,gpio-tmpv7708" },
    of_device_id { /* end of table */ },
];

static mut visconti_gpio_driver: platform_driver = platform_driver {
    probe: Some(visconti_gpio_probe),
    driver: driver {
        name: "visconti_gpio",
        of_match_table: visconti_gpio_of_match.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, visconti_gpio_of_match);
// module_platform_driver(visconti_gpio_driver);
// MODULE_AUTHOR("Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>");
// MODULE_DESCRIPTION("Toshiba Visconti GPIO Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
