// SPDX-License-Identifier: GPL-2.0-only
/* Abilis Systems MODULE DESCRIPTION
 *
 * Copyright (C) Abilis Systems 2013
 *
 * Authors: Sascha Leuenberger <sascha.leuenberger@abilis.com>
 *          Christian Ruppert <christian.ruppert@abilis.com>
 */

// Linux kernel dependencies are supplied by the surrounding build.

const TB10X_GPIO_DIR_IN: u32 = 0x00000000;
const TB10X_GPIO_DIR_OUT: u32 = 0x00000001;
const OFFSET_TO_REG_DDR: u32 = 0x00;
const OFFSET_TO_REG_DATA: u32 = 0x04;
const OFFSET_TO_REG_INT_EN: u32 = 0x08;
const OFFSET_TO_REG_CHANGE: u32 = 0x0C;
const OFFSET_TO_REG_WRMASK: u32 = 0x10;
const OFFSET_TO_REG_INT_TYPE: u32 = 0x14;

/**
 * struct tb10x_gpio - TB10x GPIO controller structure
 * @base: register base address
 * @domain: IRQ domain of GPIO generated interrupts managed by this controller
 * @irq: Interrupt line of parent interrupt controller
 * @chip: Generic GPIO chip structure associated with this GPIO controller
 */
#[repr(C)]
struct tb10x_gpio {
    base: *mut core::ffi::c_void,
    domain: *mut irq_domain,
    irq: i32,
    chip: gpio_generic_chip,
}

unsafe fn tb10x_reg_read(gpio: *mut tb10x_gpio, offs: u32) -> u32 {
    ioread32((*gpio).base.cast::<u8>().add(offs as usize).cast())
}

unsafe fn tb10x_gpio_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 {
    let tb10x_gpio = gpiochip_get_data(chip);
    irq_create_mapping((*tb10x_gpio.cast::<tb10x_gpio>()).domain, offset)
}

unsafe fn tb10x_gpio_irq_set_type(data: *mut irq_data, irq_type: u32) -> i32 {
    if (irq_type & IRQF_TRIGGER_MASK) != IRQ_TYPE_EDGE_BOTH {
        pr_err("Only (both) edge triggered interrupts supported.\n");
        return -EINVAL;
    }

    irqd_set_trigger_type(data, irq_type);
    IRQ_SET_MASK_OK
}

unsafe extern "C" fn tb10x_gpio_irq_cascade(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let tb10x_gpio = data.cast::<tb10x_gpio>();
    let r = tb10x_reg_read(tb10x_gpio, OFFSET_TO_REG_CHANGE);
    let m = tb10x_reg_read(tb10x_gpio, OFFSET_TO_REG_INT_EN);
    let bits: usize = (r & m) as usize;

    for i in 0..32 {
        if (bits & (1usize << i)) != 0 {
            generic_handle_domain_irq((*tb10x_gpio).domain, i as u32);
        }
    }

    IRQ_HANDLED
}

unsafe fn tb10x_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config: gpio_generic_chip_config = core::mem::zeroed();
    let mut tb10x_gpio: *mut tb10x_gpio;
    let dev = &mut (*pdev).dev as *mut device;
    let np = (*dev).of_node;
    let mut ret: i32 = -EBUSY;
    let mut ngpio: u32 = 0;

    if np.is_null() {
        return -EINVAL;
    }
    if of_property_read_u32(np, c"abilis,ngpio".as_ptr(), &mut ngpio) != 0 {
        return -EINVAL;
    }

    tb10x_gpio = devm_kzalloc(dev, core::mem::size_of::<tb10x_gpio>(), GFP_KERNEL).cast();
    if tb10x_gpio.is_null() {
        return -ENOMEM;
    }

    (*tb10x_gpio).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*tb10x_gpio).base) {
        return PTR_ERR((*tb10x_gpio).base);
    }

    (*tb10x_gpio).chip.gc.label = devm_kasprintf(dev, GFP_KERNEL, c"%pOF".as_ptr(), (*pdev).dev.of_node);
    if (*tb10x_gpio).chip.gc.label.is_null() {
        return -ENOMEM;
    }

    /* Initialize generic GPIO with one register for reading and setting lines,
     * no special set or clear registers, and a data direction register where
     * 1 means output. */
    config.dev = dev;
    config.sz = 4;
    config.dat = (*tb10x_gpio).base.cast::<u8>().add(OFFSET_TO_REG_DATA as usize).cast();
    config.dirout = (*tb10x_gpio).base.cast::<u8>().add(OFFSET_TO_REG_DDR as usize).cast();

    ret = gpio_generic_chip_init(&mut (*tb10x_gpio).chip, &mut config);
    if ret != 0 {
        dev_err(dev, c"unable to init generic GPIO\n".as_ptr());
        return ret;
    }
    (*tb10x_gpio).chip.gc.base = -1;
    (*tb10x_gpio).chip.gc.parent = dev;
    (*tb10x_gpio).chip.gc.owner = THIS_MODULE;
    (*tb10x_gpio).chip.gc.ngpio = ngpio;
    (*tb10x_gpio).chip.gc.request = Some(gpiochip_generic_request);
    (*tb10x_gpio).chip.gc.free = Some(gpiochip_generic_free);

    ret = devm_gpiochip_add_data(dev, &mut (*tb10x_gpio).chip.gc, tb10x_gpio.cast());
    if ret < 0 {
        dev_err(dev, c"Could not add gpiochip.\n".as_ptr());
        return ret;
    }

    platform_set_drvdata(pdev, tb10x_gpio.cast());

    if of_property_read_bool(np, c"interrupt-controller".as_ptr()) {
        let mut gc: *mut irq_chip_generic;
        ret = platform_get_irq(pdev, 0);
        if ret < 0 { return ret; }
        (*tb10x_gpio).chip.gc.to_irq = Some(tb10x_gpio_to_irq);
        (*tb10x_gpio).irq = ret;
        ret = devm_request_irq(dev, ret, Some(tb10x_gpio_irq_cascade), IRQF_TRIGGER_NONE | IRQF_SHARED, dev_name(dev), tb10x_gpio.cast());
        if ret != 0 { return ret; }
        (*tb10x_gpio).domain = irq_domain_create_linear(dev_fwnode(dev), (*tb10x_gpio).chip.gc.ngpio, &irq_generic_chip_ops, core::ptr::null_mut());
        if (*tb10x_gpio).domain.is_null() { return -ENOMEM; }
        ret = irq_alloc_domain_generic_chips((*tb10x_gpio).domain, (*tb10x_gpio).chip.gc.ngpio, 1, (*tb10x_gpio).chip.gc.label, Some(handle_edge_irq), IRQ_NOREQUEST, IRQ_NOPROBE, IRQ_GC_INIT_MASK_CACHE);
        if ret != 0 { irq_domain_remove((*tb10x_gpio).domain); return ret; }
        gc = (*tb10x_gpio).domain.gc.gc[0];
        (*gc).reg_base = (*tb10x_gpio).base;
        (*gc).chip_types[0].type_ = IRQ_TYPE_EDGE_BOTH;
        (*gc).chip_types[0].chip.irq_ack = Some(irq_gc_ack_set_bit);
        (*gc).chip_types[0].chip.irq_mask = Some(irq_gc_mask_clr_bit);
        (*gc).chip_types[0].chip.irq_unmask = Some(irq_gc_mask_set_bit);
        (*gc).chip_types[0].chip.irq_set_type = Some(tb10x_gpio_irq_set_type);
        (*gc).chip_types[0].regs.ack = OFFSET_TO_REG_CHANGE;
        (*gc).chip_types[0].regs.mask = OFFSET_TO_REG_INT_EN;
    }
    0
}

unsafe fn tb10x_gpio_remove(pdev: *mut platform_device) {
    let tb10x_gpio = platform_get_drvdata(pdev).cast::<tb10x_gpio>();
    if (*tb10x_gpio).chip.gc.to_irq.is_some() {
        irq_remove_generic_chip((*tb10x_gpio).domain.gc.gc[0], (1u32 << (*tb10x_gpio).chip.gc.ngpio) - 1, 0, 0);
        kfree((*tb10x_gpio).domain.gc);
        irq_domain_remove((*tb10x_gpio).domain);
    }
}

static mut tb10x_gpio_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"abilis,tb10x-gpio".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut tb10x_gpio_driver: platform_driver = platform_driver {
    probe: Some(tb10x_gpio_probe),
    remove: Some(tb10x_gpio_remove),
    driver: device_driver {
        name: c"tb10x-gpio".as_ptr(),
        of_match_table: tb10x_gpio_dt_ids.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
};

// module_platform_driver!(tb10x_gpio_driver);
// MODULE_DEVICE_TABLE(of, tb10x_gpio_dt_ids);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("tb10x gpio.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
