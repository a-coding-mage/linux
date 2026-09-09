/*
 * Support for the GPIO/IRQ expander chips present on several HTC phones.
 * These are implemented in CPLD chips present on the board.
 *
 * Copyright (c) 2007 Kevin O'Connor <kevin@koconnor.net>
 * Copyright (c) 2007 Philipp Zabel <philipp.zabel@gmail.com>
 *
 * This file may be distributed under the terms of the GNU GPL license.
 */

// Linux kernel dependencies supplied by other translated units.

#[repr(C)]
struct egpio_chip {
    reg_start: c_int,
    cached_values: c_int,
    is_out: c_ulong,
    dev: *mut device,
    chip: gpio_chip,
}

#[repr(C)]
struct egpio_info {
    lock: spinlock_t,
    /* iomem info */
    base_addr: *mut c_void,
    bus_shift: c_int, /* byte shift */
    reg_shift: c_int, /* bit shift */
    reg_mask: c_int,
    /* irq info */
    ack_register: c_int,
    ack_write: c_int,
    irqs_enabled: u16,
    irq_start: c_uint,
    nirqs: c_int,
    chained_irq: c_uint,
    /* egpio info */
    nchips: c_int,
    chip: [egpio_chip; 0],
}

#[inline]
unsafe fn egpio_writew(value: u16, ei: *mut egpio_info, reg: c_int) {
    writew(value, (*ei).base_addr.add((reg << (*ei).bus_shift) as usize));
}

#[inline]
unsafe fn egpio_readw(ei: *mut egpio_info, reg: c_int) -> u16 {
    readw((*ei).base_addr.add((reg << (*ei).bus_shift) as usize))
}

#[inline]
unsafe fn ack_irqs(ei: *mut egpio_info) {
    egpio_writew((*ei).ack_write as u16, ei, (*ei).ack_register);
    pr_debug!("EGPIO ack - write %x to base+%x\n", (*ei).ack_write,
              (*ei).ack_register << (*ei).bus_shift);
}

unsafe extern "C" fn egpio_ack(_data: *mut irq_data) {}

/* There does not appear to be a way to proactively mask interrupts
 * on the egpio chip itself.  So, we simply ignore interrupts that
 * aren't desired. */
unsafe extern "C" fn egpio_mask(data: *mut irq_data) {
    let ei = irq_data_get_irq_chip_data(data) as *mut egpio_info;
    (*ei).irqs_enabled &= !(1u16 << ((*data).irq - (*ei).irq_start));
    pr_debug!("EGPIO mask %d %04x\n", (*data).irq, (*ei).irqs_enabled);
}

unsafe extern "C" fn egpio_unmask(data: *mut irq_data) {
    let ei = irq_data_get_irq_chip_data(data) as *mut egpio_info;
    (*ei).irqs_enabled |= 1u16 << ((*data).irq - (*ei).irq_start);
    pr_debug!("EGPIO unmask %d %04x\n", (*data).irq, (*ei).irqs_enabled);
}

static mut egpio_muxed_chip: irq_chip = irq_chip {
    name: cstr!("htc-egpio"),
    irq_ack: Some(egpio_ack),
    irq_mask: Some(egpio_mask),
    irq_unmask: Some(egpio_unmask),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn egpio_handler(desc: *mut irq_desc) {
    let ei = irq_desc_get_handler_data(desc) as *mut egpio_info;
    let mut irqpin: c_int = 0;
    /* Read current pins. */
    let mut readval = egpio_readw(ei, (*ei).ack_register) as c_ulong;
    pr_debug!("IRQ reg: %x\n", readval as c_uint);
    /* Ack/unmask interrupts. */
    ack_irqs(ei);
    /* Process all set pins. */
    readval &= (*ei).irqs_enabled as c_ulong;
    for_each_set_bit!(irqpin, &readval, (*ei).nirqs);
    {
        pr_debug!("got IRQ %d\n", irqpin);
        generic_handle_irq((*ei).irq_start + irqpin as c_uint);
    }
}

#[inline]
unsafe fn egpio_pos(ei: *mut egpio_info, bit: c_int) -> c_int {
    bit >> (*ei).reg_shift
}

#[inline]
unsafe fn egpio_bit(ei: *mut egpio_info, bit: c_int) -> c_int {
    1 << (bit & ((1 << (*ei).reg_shift) - 1))
}

unsafe extern "C" fn egpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let egpio = gpiochip_get_data(chip) as *mut egpio_chip;
    let ei = dev_get_drvdata((*egpio).dev) as *mut egpio_info;
    let bit = egpio_bit(ei, offset as c_int);
    let reg = (*egpio).reg_start + egpio_pos(ei, offset as c_int);
    pr_debug!("egpio_get_value(%d)\n", (*chip).base + offset as c_int);
    if test_bit!(offset, &(*egpio).is_out) {
        !!(((*egpio).cached_values & (1 << offset)) != 0) as c_int
    } else {
        let value = egpio_readw(ei, reg) as c_int;
        pr_debug!("readw(%p + %x) = %x\n", (*ei).base_addr, reg << (*ei).bus_shift, value);
        !!((value & bit) != 0) as c_int
    }
}

unsafe extern "C" fn egpio_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let egpio = gpiochip_get_data(chip) as *mut egpio_chip;
    if test_bit!(offset, &(*egpio).is_out) { -EINVAL } else { 0 }
}

unsafe extern "C" fn egpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let egpio = gpiochip_get_data(chip) as *mut egpio_chip;
    let ei = dev_get_drvdata((*egpio).dev) as *mut egpio_info;
    let pos = egpio_pos(ei, offset as c_int);
    let reg = (*egpio).reg_start + pos;
    let shift = pos << (*ei).reg_shift;
    let mut flag: c_ulong = 0;
    pr_debug!("egpio_set(%s, %d(%d), %d)\n", (*chip).label, offset,
              offset as c_int + (*chip).base, value);
    spin_lock_irqsave!(&mut (*ei).lock, flag);
    if value != 0 { (*egpio).cached_values |= 1 << offset; }
    else { (*egpio).cached_values &= !(1 << offset); }
    egpio_writew((((*egpio).cached_values >> shift) & (*ei).reg_mask) as u16, ei, reg);
    spin_unlock_irqrestore!(&mut (*ei).lock, flag);
    0
}

unsafe extern "C" fn egpio_direction_output(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let egpio = gpiochip_get_data(chip) as *mut egpio_chip;
    if test_bit!(offset, &(*egpio).is_out) { egpio_set(chip, offset, value) } else { -EINVAL }
}

unsafe extern "C" fn egpio_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let egpio = gpiochip_get_data(chip) as *mut egpio_chip;
    if test_bit!(offset, &(*egpio).is_out) { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn egpio_write_cache(ei: *mut egpio_info) {
    for i in 0..(*ei).nchips {
        let egpio = (*ei).chip.as_mut_ptr().add(i as usize);
        if (*egpio).is_out == 0 { continue; }
        let mut shift = 0;
        while shift < (*egpio).chip.ngpio {
            let reg = (*egpio).reg_start + egpio_pos(ei, shift);
            if (((*egpio).is_out >> shift) as c_int & (*ei).reg_mask) == 0 { shift += 1 << (*ei).reg_shift; continue; }
            egpio_writew((((*egpio).cached_values >> shift) & (*ei).reg_mask) as u16, ei, reg);
            shift += 1 << (*ei).reg_shift;
        }
    }
}

unsafe extern "C" fn egpio_probe(pdev: *mut platform_device) -> c_int {
    let pdata = dev_get_platdata!(&mut (*pdev).dev) as *mut htc_egpio_platform_data;
    let mut res: *mut resource;
    let ei = devm_kzalloc!(&mut (*pdev).dev, struct_size!(egpio_info, chip, (*pdata).num_chips), GFP_KERNEL) as *mut egpio_info;
    if ei.is_null() { return -ENOMEM; }
    (*ei).nchips = (*pdata).num_chips;
    spin_lock_init!(&mut (*ei).lock);
    res = platform_get_resource(pdev, IORESOURCE_IRQ, 0);
    if !res.is_null() { (*ei).chained_irq = (*res).start; }
    (*ei).base_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!((*ei).base_addr) { return PTR_ERR!((*ei).base_addr); }
    if (*pdata).bus_width != 16 && (*pdata).bus_width != 32 { return -EINVAL; }
    (*ei).bus_shift = fls((*pdata).bus_width - 1) - 3;
    if (*pdata).reg_width != 8 && (*pdata).reg_width != 16 { return -EINVAL; }
    (*ei).reg_shift = fls((*pdata).reg_width - 1);
    (*ei).reg_mask = (1 << (*pdata).reg_width) - 1;
    platform_set_drvdata(pdev, ei as *mut c_void);
    for i in 0..(*ei).nchips {
        let egpio = (*ei).chip.as_mut_ptr().add(i as usize);
        let src = (*pdata).chip.as_ptr().add(i as usize);
        (*egpio).reg_start = (*src).reg_start;
        (*egpio).cached_values = (*src).initial_values;
        (*egpio).is_out = (*src).direction;
        (*egpio).dev = &mut (*pdev).dev;
        let chip = &mut (*egpio).chip;
        chip.label = devm_kasprintf!(&mut (*pdev).dev, GFP_KERNEL, "htc-egpio-%d", i);
        if chip.label.is_null() { return -ENOMEM; }
        chip.parent = &mut (*pdev).dev;
        chip.owner = THIS_MODULE;
        chip.get = Some(egpio_get); chip.set = Some(egpio_set);
        chip.direction_input = Some(egpio_direction_input);
        chip.direction_output = Some(egpio_direction_output);
        chip.get_direction = Some(egpio_get_direction);
        chip.base = (*src).gpio_base; chip.ngpio = (*src).num_gpios;
        let ret = devm_gpiochip_add_data(&mut (*pdev).dev, chip, egpio as *mut c_void);
        if ret != 0 { return dev_err_probe!(&mut (*pdev).dev, ret, "failed to register gpiochip %d\n", i); }
    }
    egpio_write_cache(ei);
    (*ei).irq_start = (*pdata).irq_base; (*ei).nirqs = (*pdata).num_irqs;
    (*ei).ack_register = (*pdata).ack_register;
    if (*ei).chained_irq != 0 {
        (*ei).ack_write = if (*pdata).invert_acks { 0 } else { 0xFFFF };
        let irq_end = (*ei).irq_start + (*ei).nirqs as c_uint;
        let mut irq = (*ei).irq_start;
        while irq < irq_end { irq_set_chip_and_handler!(irq, &mut egpio_muxed_chip, handle_simple_irq); irq_set_chip_data(irq, ei as *mut c_void); irq_clear_status_flags(irq, IRQ_NOREQUEST | IRQ_NOPROBE); irq += 1; }
        irq_set_irq_type((*ei).chained_irq, IRQ_TYPE_EDGE_RISING);
        irq_set_chained_handler_and_data((*ei).chained_irq, Some(egpio_handler), ei as *mut c_void);
        ack_irqs(ei); device_init_wakeup(&mut (*pdev).dev, true);
    }
    0
}

unsafe extern "C" fn egpio_suspend(dev: *mut device) -> c_int { let ei = dev_get_drvdata(dev) as *mut egpio_info; if (*ei).chained_irq != 0 && device_may_wakeup(dev) { enable_irq_wake((*ei).chained_irq); } 0 }
unsafe extern "C" fn egpio_resume(dev: *mut device) -> c_int { let ei = dev_get_drvdata(dev) as *mut egpio_info; if (*ei).chained_irq != 0 && device_may_wakeup(dev) { disable_irq_wake((*ei).chained_irq); } egpio_write_cache(ei); 0 }

// DEFINE_SIMPLE_DEV_PM_OPS(egpio_pm_ops, egpio_suspend, egpio_resume)
// static platform_driver egpio_driver = { .driver = { .name = "htc-egpio", .suppress_bind_attrs = true, .pm = pm_sleep_ptr(&egpio_pm_ops) } };
unsafe extern "C" fn egpio_init() -> c_int { platform_driver_probe(&mut egpio_driver, Some(egpio_probe)) }
// start early for dependencies: subsys_initcall(egpio_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
