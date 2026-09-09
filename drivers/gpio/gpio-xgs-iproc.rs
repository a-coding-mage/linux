// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Broadcom
 */

// Linux kernel dependencies are supplied by the surrounding Rust environment.

const IPROC_CCA_INT_F_GPIOINT: u32 = 1 << 0;
const IPROC_CCA_INT_STS: usize = 0x20;
const IPROC_CCA_INT_MASK: usize = 0x24;

const IPROC_GPIO_CCA_DIN: usize = 0x0;
const IPROC_GPIO_CCA_DOUT: usize = 0x4;
const IPROC_GPIO_CCA_OUT_EN: usize = 0x8;
const IPROC_GPIO_CCA_INT_LEVEL: usize = 0x10;
const IPROC_GPIO_CCA_INT_LEVEL_MASK: usize = 0x14;
const IPROC_GPIO_CCA_INT_EVENT: usize = 0x18;
const IPROC_GPIO_CCA_INT_EVENT_MASK: usize = 0x1c;
const IPROC_GPIO_CCA_INT_EDGE: usize = 0x24;

#[repr(C)]
struct IprocGpioChip {
    gen_gc: gpio_generic_chip,
    lock: spinlock_t,
    dev: *mut device,
    base: *mut core::ffi::c_void,
    intr: *mut core::ffi::c_void,
}

unsafe fn to_iproc_gpio(gc: *mut gpio_chip) -> *mut IprocGpioChip {
    container_of(to_gpio_generic_chip(gc), IprocGpioChip, gen_gc)
}

unsafe fn iproc_gpio_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = to_iproc_gpio(gc);
    let pin = (*d).hwirq;
    let mut flags: c_ulong = 0;
    let irq = (*d).irq;
    let irq_type: u32;
    let mut event_status: u32 = 0;

    spin_lock_irqsave(&mut (*chip).lock, &mut flags);
    irq_type = irq_get_trigger_type(irq);
    if irq_type & IRQ_TYPE_EDGE_BOTH != 0 {
        event_status |= 1u32 << pin;
        writel_relaxed(event_status, (*chip).base.add(IPROC_GPIO_CCA_INT_EVENT));
    }
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
}

unsafe fn iproc_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = to_iproc_gpio(gc);
    let pin = (*d).hwirq;
    let mut flags: c_ulong = 0;
    let irq = (*d).irq;
    let irq_type: u32;
    let mut int_mask;
    let mut event_mask;

    gpiochip_enable_irq(gc, pin);
    spin_lock_irqsave(&mut (*chip).lock, &mut flags);
    irq_type = irq_get_trigger_type(irq);
    event_mask = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_EVENT_MASK));
    int_mask = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL_MASK));
    if irq_type & IRQ_TYPE_EDGE_BOTH != 0 {
        event_mask |= 1u32 << pin;
        writel_relaxed(event_mask, (*chip).base.add(IPROC_GPIO_CCA_INT_EVENT_MASK));
    } else {
        int_mask |= 1u32 << pin;
        writel_relaxed(int_mask, (*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL_MASK));
    }
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
}

unsafe fn iproc_gpio_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = to_iproc_gpio(gc);
    let pin = (*d).hwirq;
    let mut flags: c_ulong = 0;
    let irq = (*d).irq;
    spin_lock_irqsave(&mut (*chip).lock, &mut flags);
    let irq_type = irq_get_trigger_type(irq);
    let mut event_mask = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_EVENT_MASK));
    let mut int_mask = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL_MASK));
    if irq_type & IRQ_TYPE_EDGE_BOTH != 0 {
        event_mask &= !(1u32 << pin);
        writel_relaxed(event_mask, (*chip).base.add(IPROC_GPIO_CCA_INT_EVENT_MASK));
    } else {
        int_mask &= !(1u32 << pin);
        writel_relaxed(int_mask, (*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL_MASK));
    }
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
    gpiochip_disable_irq(gc, pin);
}

unsafe fn iproc_gpio_irq_set_type(d: *mut irq_data, type_: u32) -> c_int {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = to_iproc_gpio(gc);
    let pin = (*d).hwirq;
    let mut flags: c_ulong = 0;
    let irq = (*d).irq;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&mut (*chip).lock, &mut flags);
    match type_ & IRQ_TYPE_SENSE_MASK {
        IRQ_TYPE_EDGE_RISING => {
            let mut event_pol = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_EDGE));
            event_pol &= !(1u32 << pin);
            writel_relaxed(event_pol, (*chip).base.add(IPROC_GPIO_CCA_INT_EDGE));
        }
        IRQ_TYPE_EDGE_FALLING => {
            let mut event_pol = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_EDGE));
            event_pol |= 1u32 << pin;
            writel_relaxed(event_pol, (*chip).base.add(IPROC_GPIO_CCA_INT_EDGE));
        }
        IRQ_TYPE_LEVEL_HIGH => {
            let mut int_pol = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL));
            int_pol &= !(1u32 << pin);
            writel_relaxed(int_pol, (*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL));
        }
        IRQ_TYPE_LEVEL_LOW => {
            let mut int_pol = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL));
            int_pol |= 1u32 << pin;
            writel_relaxed(int_pol, (*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL));
        }
        _ => {
            // should not come here
            ret = -EINVAL;
            spin_unlock_irqrestore(&mut (*chip).lock, flags);
            return ret;
        }
    }
    if type_ & IRQ_TYPE_LEVEL_MASK != 0 {
        irq_set_handler_locked(irq_get_irq_data(irq), handle_level_irq);
    } else if type_ & IRQ_TYPE_EDGE_BOTH != 0 {
        irq_set_handler_locked(irq_get_irq_data(irq), handle_edge_irq);
    }
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
    ret
}

unsafe fn iproc_gpio_irq_handler(irq: c_int, data: *mut core::ffi::c_void) -> irqreturn_t {
    let gc = data as *mut gpio_chip;
    let chip = to_iproc_gpio(gc);
    let mut int_bits: c_ulong = 0;
    let int_status = readl_relaxed((*chip).intr.add(IPROC_CCA_INT_STS));
    if int_status & IPROC_CCA_INT_F_GPIOINT != 0 {
        let mut event = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_EVENT_MASK));
        event &= readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_EVENT));
        let mut level = readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_DIN));
        level ^= readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL));
        level &= readl_relaxed((*chip).base.add(IPROC_GPIO_CCA_INT_LEVEL_MASK));
        int_bits = (level | event) as c_ulong;
        for_each_set_bit(|bit| { generic_handle_domain_irq((*gc).irq.domain, bit); }, &int_bits, (*gc).ngpio);
    }
    if int_bits != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn iproc_gpio_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    let gc = irq_data_get_irq_chip_data(d);
    let chip = to_iproc_gpio(gc);
    seq_puts(p, dev_name((*chip).dev));
}

static IPROC_GPIO_IRQ_CHIP: irq_chip = irq_chip {
    irq_ack: Some(iproc_gpio_irq_ack), irq_mask: Some(iproc_gpio_irq_mask),
    irq_unmask: Some(iproc_gpio_irq_unmask), irq_set_type: Some(iproc_gpio_irq_set_type),
    irq_print_chip: Some(iproc_gpio_irq_print_chip), flags: IRQCHIP_IMMUTABLE,
};

unsafe fn iproc_gpio_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let dn = (*pdev).dev.of_node;
    let chip = devm_kzalloc(dev, core::mem::size_of::<IprocGpioChip>(), GFP_KERNEL)
        as *mut IprocGpioChip;
    if chip.is_null() { return -ENOMEM; }
    (*chip).dev = dev;
    platform_set_drvdata(pdev, chip as *mut core::ffi::c_void);
    spin_lock_init(&mut (*chip).lock);
    (*chip).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*chip).base) { return PTR_ERR((*chip).base); }
    let config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: (*chip).base.add(IPROC_GPIO_CCA_DIN),
        set: (*chip).base.add(IPROC_GPIO_CCA_DOUT),
        dirout: (*chip).base.add(IPROC_GPIO_CCA_OUT_EN),
    };
    let mut ret = gpio_generic_chip_init(&mut (*chip).gen_gc, &config);
    if ret != 0 { dev_err(dev, "unable to init GPIO chip\n"); return ret; }
    (*chip).gen_gc.gc.label = dev_name(dev);
    let mut num_gpios = 0;
    if of_property_read_u32(dn, b"ngpios\0".as_ptr() as *const c_char, &mut num_gpios) == 0 {
        (*chip).gen_gc.gc.ngpio = num_gpios;
    }
    let irq = platform_get_irq(pdev, 0);
    if irq > 0 {
        (*chip).intr = devm_platform_ioremap_resource(pdev, 1);
        if IS_ERR((*chip).intr) { return PTR_ERR((*chip).intr); }
        let mut val = readl_relaxed((*chip).intr.add(IPROC_CCA_INT_MASK));
        val |= IPROC_CCA_INT_F_GPIOINT;
        writel_relaxed(val, (*chip).intr.add(IPROC_CCA_INT_MASK));
        ret = devm_request_irq(dev, irq, Some(iproc_gpio_irq_handler), IRQF_SHARED,
                               (*chip).gen_gc.gc.label, &mut (*chip).gen_gc.gc as *mut _ as *mut _);
        if ret != 0 { dev_err(dev, "Fail to request IRQ%d: %d\n", irq, ret); return ret; }
        let girq = &mut (*chip).gen_gc.gc.irq;
        gpio_irq_chip_set_chip(girq, &IPROC_GPIO_IRQ_CHIP);
        girq.parent_handler = None;
        girq.num_parents = 0;
        girq.parents = core::ptr::null_mut();
        girq.default_type = IRQ_TYPE_NONE;
        girq.handler = Some(handle_simple_irq);
    }
    ret = devm_gpiochip_add_data(dev, &mut (*chip).gen_gc.gc, chip as *mut _);
    if ret != 0 { dev_err(dev, "unable to add GPIO chip\n"); return ret; }
    0
}

unsafe fn iproc_gpio_remove(pdev: *mut platform_device) {
    let chip = platform_get_drvdata(pdev) as *mut IprocGpioChip;
    if !(*chip).intr.is_null() {
        let mut val = readl_relaxed((*chip).intr.add(IPROC_CCA_INT_MASK));
        val &= !IPROC_CCA_INT_F_GPIOINT;
        writel_relaxed(val, (*chip).intr.add(IPROC_CCA_INT_MASK));
    }
}

static BCM_IPROC_GPIO_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"brcm,iproc-gpio-cca\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static BCM_IPROC_GPIO_DRIVER: platform_driver = platform_driver {
    driver: device_driver { name: b"iproc-xgs-gpio\0".as_ptr() as *const c_char, of_match_table: BCM_IPROC_GPIO_OF_MATCH.as_ptr() },
    probe: Some(iproc_gpio_probe), remove: Some(iproc_gpio_remove),
};

// module_platform_driver!(BCM_IPROC_GPIO_DRIVER);
// MODULE_DESCRIPTION!("XGS IPROC GPIO driver");
// MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
