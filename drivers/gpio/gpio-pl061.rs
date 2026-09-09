// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, 2009 Provigent Ltd.
 *
 * Author: Baruch Siach <baruch@tkos.co.il>
 *
 * Driver for the ARM PrimeCell(tm) General Purpose Input/Output (PL061)
 *
 * Data sheet: ARM DDI 0190B, September 2000
 */

const GPIODIR: usize = 0x400;
const GPIOIS: usize = 0x404;
const GPIOIBE: usize = 0x408;
const GPIOIEV: usize = 0x40c;
const GPIOIE: usize = 0x410;
const GPIORIS: usize = 0x414;
const GPIOMIS: usize = 0x418;
const GPIOIC: usize = 0x41c;

const PL061_GPIO_NR: usize = 8;

#[repr(C)]
struct pl061_context_save_regs {
    gpio_data: u8,
    gpio_dir: u8,
    gpio_is: u8,
    gpio_ibe: u8,
    gpio_iev: u8,
    gpio_ie: u8,
}

#[repr(C)]
struct pl061 {
    lock: raw_spinlock_t,
    base: *mut core::ffi::c_void,
    gc: gpio_chip,
    parent_irq: i32,
    csave_regs: pl061_context_save_regs,
}

unsafe fn pl061_get_direction(gc: *mut gpio_chip, offset: u32) -> i32 {
    let pl061 = gpiochip_get_data(gc);
    if readb((*pl061).base.add(GPIODIR) as *const u8) & BIT(offset) as u8 != 0 {
        GPIO_LINE_DIRECTION_OUT
    } else {
        GPIO_LINE_DIRECTION_IN
    }
}

unsafe fn pl061_direction_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let pl061 = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*pl061).lock, &mut flags);
    let mut gpiodir = readb((*pl061).base.add(GPIODIR) as *const u8);
    gpiodir &= !(BIT(offset) as u8);
    writeb(gpiodir, (*pl061).base.add(GPIODIR));
    raw_spin_unlock_irqrestore(&mut (*pl061).lock, flags);
    0
}

unsafe fn pl061_direction_output(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let pl061 = gpiochip_get_data(gc);
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*pl061).lock, &mut flags);
    writeb(((!(!value == 0) as u8) << offset), (*pl061).base.add(BIT(offset + 2) as usize));
    let mut gpiodir = readb((*pl061).base.add(GPIODIR) as *const u8);
    gpiodir |= BIT(offset) as u8;
    writeb(gpiodir, (*pl061).base.add(GPIODIR));
    /* gpio value is set again, because pl061 doesn't allow to set value of
     * a gpio pin before configuring it in OUT mode. */
    writeb(((!(!value == 0) as u8) << offset), (*pl061).base.add(BIT(offset + 2) as usize));
    raw_spin_unlock_irqrestore(&mut (*pl061).lock, flags);
    0
}

unsafe fn pl061_get_value(gc: *mut gpio_chip, offset: u32) -> i32 {
    let pl061 = gpiochip_get_data(gc);
    (!!readb((*pl061).base.add(BIT(offset + 2) as usize) as *const u8)) as i32
}

unsafe fn pl061_set_value(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let pl061 = gpiochip_get_data(gc);
    writeb(((!(!value == 0) as u8) << offset), (*pl061).base.add(BIT(offset + 2) as usize));
    0
}

unsafe fn pl061_irq_type(d: *mut irq_data, trigger: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let pl061 = gpiochip_get_data(gc);
    let offset = irqd_to_hwirq(d) as i32;
    if offset < 0 || offset >= PL061_GPIO_NR as i32 { return -EINVAL; }
    let mut flags: c_ulong = 0;
    let bit = BIT(offset as u32) as u8;
    raw_spin_lock_irqsave(&mut (*pl061).lock, &mut flags);
    let mut gpioiev = readb((*pl061).base.add(GPIOIEV) as *const u8);
    let mut gpiois = readb((*pl061).base.add(GPIOIS) as *const u8);
    let mut gpioibe = readb((*pl061).base.add(GPIOIBE) as *const u8);
    if trigger & IRQ_TYPE_LEVEL_MASK != 0 {
        let polarity = trigger & IRQ_TYPE_LEVEL_HIGH != 0;
        gpioibe &= !bit; gpiois |= bit;
        if polarity { gpioiev |= bit; } else { gpioiev &= !bit; }
        irq_set_handler_locked(d, handle_level_irq);
        dev_dbg((*gc).parent, "line %d: IRQ on %s level\n", offset, if polarity { "HIGH" } else { "LOW" });
    } else if trigger & IRQ_TYPE_EDGE_BOTH == IRQ_TYPE_EDGE_BOTH {
        gpiois &= !bit; gpioibe |= bit;
        irq_set_handler_locked(d, handle_edge_irq);
        dev_dbg((*gc).parent, "line %d: IRQ on both edges\n", offset);
    } else if trigger & (IRQ_TYPE_EDGE_RISING | IRQ_TYPE_EDGE_FALLING) != 0 {
        let rising = trigger & IRQ_TYPE_EDGE_RISING != 0;
        gpiois &= !bit; gpioibe &= !bit;
        if rising { gpioiev |= bit; } else { gpioiev &= !bit; }
        irq_set_handler_locked(d, handle_edge_irq);
        dev_dbg((*gc).parent, "line %d: IRQ on %s edge\n", offset, if rising { "RISING" } else { "FALLING" });
    } else {
        gpiois &= !bit; gpioibe &= !bit; gpioiev &= !bit;
        irq_set_handler_locked(d, handle_bad_irq);
        dev_warn((*gc).parent, "no trigger selected for line %d\n", offset);
    }
    writeb(gpiois, (*pl061).base.add(GPIOIS));
    writeb(gpioibe, (*pl061).base.add(GPIOIBE));
    writeb(gpioiev, (*pl061).base.add(GPIOIEV));
    raw_spin_unlock_irqrestore(&mut (*pl061).lock, flags);
    0
}

unsafe fn pl061_irq_handler(desc: *mut irq_desc) {
    let gc = irq_desc_get_handler_data(desc);
    let pl061 = gpiochip_get_data(gc);
    let irqchip = irq_desc_get_chip(desc);
    chained_irq_enter(irqchip, desc);
    let pending = readb((*pl061).base.add(GPIOMIS) as *const u8);
    for offset in 0..PL061_GPIO_NR {
        if pending & BIT(offset as u32) as u8 != 0 { generic_handle_domain_irq((*gc).irq.domain, offset as u32); }
    }
    chained_irq_exit(irqchip, desc);
}

unsafe fn pl061_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let pl061 = gpiochip_get_data(gc);
    let mask = BIT((irqd_to_hwirq(d) as usize) % PL061_GPIO_NR) as u8;
    raw_spin_lock(&mut (*pl061).lock);
    writeb(readb((*pl061).base.add(GPIOIE) as *const u8) & !mask, (*pl061).base.add(GPIOIE));
    raw_spin_unlock(&mut (*pl061).lock); gpiochip_disable_irq(gc, (*d).hwirq);
}

unsafe fn pl061_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let pl061 = gpiochip_get_data(gc);
    let mask = BIT((irqd_to_hwirq(d) as usize) % PL061_GPIO_NR) as u8;
    gpiochip_enable_irq(gc, (*d).hwirq); raw_spin_lock(&mut (*pl061).lock);
    writeb(readb((*pl061).base.add(GPIOIE) as *const u8) | mask, (*pl061).base.add(GPIOIE));
    raw_spin_unlock(&mut (*pl061).lock);
}

/* ACK an edge IRQ in the GPIOIC interrupt-clear register. */
unsafe fn pl061_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let pl061 = gpiochip_get_data(gc);
    let mask = BIT((irqd_to_hwirq(d) as usize) % PL061_GPIO_NR) as u8;
    raw_spin_lock(&mut (*pl061).lock); writeb(mask, (*pl061).base.add(GPIOIC)); raw_spin_unlock(&mut (*pl061).lock);
}

unsafe fn pl061_irq_set_wake(d: *mut irq_data, state: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d); let pl061 = gpiochip_get_data(gc);
    irq_set_irq_wake((*pl061).parent_irq, state)
}

unsafe fn pl061_irq_print_chip(data: *mut irq_data, p: *mut seq_file) {
    let gc = irq_data_get_irq_chip_data(data); seq_puts(p, dev_name((*gc).parent));
}

#[repr(C)]
struct irq_chip {
    irq_ack: Option<unsafe fn(*mut irq_data)>, irq_mask: Option<unsafe fn(*mut irq_data)>,
    irq_unmask: Option<unsafe fn(*mut irq_data)>, irq_set_type: Option<unsafe fn(*mut irq_data, u32) -> i32>,
    irq_set_wake: Option<unsafe fn(*mut irq_data, u32) -> i32>, irq_print_chip: Option<unsafe fn(*mut irq_data, *mut seq_file)>,
    flags: u32,
}

static mut pl061_irq_chip: irq_chip = irq_chip { irq_ack: Some(pl061_irq_ack), irq_mask: Some(pl061_irq_mask), irq_unmask: Some(pl061_irq_unmask), irq_set_type: Some(pl061_irq_type), irq_set_wake: Some(pl061_irq_set_wake), irq_print_chip: Some(pl061_irq_print_chip), flags: IRQCHIP_IMMUTABLE };

unsafe fn pl061_probe(adev: *mut amba_device, _id: *const amba_id) -> i32 {
    let dev = &mut (*adev).dev;
    let pl061 = devm_kzalloc(dev, core::mem::size_of::<pl061>(), GFP_KERNEL);
    if pl061.is_null() { return -ENOMEM; }
    (*pl061).base = devm_ioremap_resource(dev, &mut (*adev).res);
    if IS_ERR((*pl061).base) { return PTR_ERR((*pl061).base); }
    raw_spin_lock_init(&mut (*pl061).lock);
    (*pl061).gc.request = Some(gpiochip_generic_request); (*pl061).gc.free = Some(gpiochip_generic_free);
    (*pl061).gc.base = -1; (*pl061).gc.get_direction = Some(pl061_get_direction);
    (*pl061).gc.direction_input = Some(pl061_direction_input); (*pl061).gc.direction_output = Some(pl061_direction_output);
    (*pl061).gc.get = Some(pl061_get_value); (*pl061).gc.set = Some(pl061_set_value);
    (*pl061).gc.ngpio = PL061_GPIO_NR as u32; (*pl061).gc.label = dev_name(dev);
    (*pl061).gc.parent = dev; (*pl061).gc.owner = THIS_MODULE;
    writeb(0, (*pl061).base.add(GPIOIE));
    let irq = (*adev).irq[0]; if irq == 0 { dev_warn(dev, "IRQ support disabled\n"); }
    (*pl061).parent_irq = irq;
    let girq = &mut (*pl061).gc.irq;
    gpio_irq_chip_set_chip(girq, &mut pl061_irq_chip);
    (*girq).parent_handler = Some(pl061_irq_handler); (*girq).num_parents = 1;
    (*girq).parents = devm_kcalloc(dev, 1, core::mem::size_of::<i32>(), GFP_KERNEL);
    if (*girq).parents.is_null() { return -ENOMEM; }
    *(*girq).parents = irq; (*girq).default_type = IRQ_TYPE_NONE; (*girq).handler = Some(handle_bad_irq);
    let ret = devm_gpiochip_add_data(dev, &mut (*pl061).gc, pl061);
    if ret != 0 { return ret; }
    amba_set_drvdata(adev, pl061); dev_info(dev, "PL061 GPIO chip registered\n"); 0
}

unsafe fn pl061_suspend(dev: *mut device) -> i32 {
    let pl061 = dev_get_drvdata(dev); let r = &mut (*pl061).csave_regs;
    r.gpio_data = 0; r.gpio_dir = readb((*pl061).base.add(GPIODIR) as *const u8);
    r.gpio_is = readb((*pl061).base.add(GPIOIS) as *const u8); r.gpio_ibe = readb((*pl061).base.add(GPIOIBE) as *const u8);
    r.gpio_iev = readb((*pl061).base.add(GPIOIEV) as *const u8); r.gpio_ie = readb((*pl061).base.add(GPIOIE) as *const u8);
    for offset in 0..PL061_GPIO_NR as u32 { if r.gpio_dir & BIT(offset) as u8 != 0 { r.gpio_data |= (pl061_get_value(&mut (*pl061).gc, offset) as u8) << offset; } }
    0
}

unsafe fn pl061_resume(dev: *mut device) -> i32 {
    let pl061 = dev_get_drvdata(dev); let r = &(*pl061).csave_regs;
    for offset in 0..PL061_GPIO_NR as u32 {
        if r.gpio_dir & BIT(offset) as u8 != 0 { pl061_direction_output(&mut (*pl061).gc, offset, (r.gpio_data & BIT(offset) as u8) as i32); }
        else { pl061_direction_input(&mut (*pl061).gc, offset); }
    }
    writeb(r.gpio_is, (*pl061).base.add(GPIOIS)); writeb(r.gpio_ibe, (*pl061).base.add(GPIOIBE));
    writeb(r.gpio_iev, (*pl061).base.add(GPIOIEV)); writeb(r.gpio_ie, (*pl061).base.add(GPIOIE)); 0
}

static pl061_ids: [amba_id; 2] = [
    amba_id { id: 0x00041061, mask: 0x000fffff },
    amba_id { id: 0, mask: 0 },
];

static mut pl061_gpio_driver: amba_driver = amba_driver {
    drv: driver { name: "pl061_gpio\0", pm: Some(pl061_suspend), resume: Some(pl061_resume) },
    id_table: pl061_ids.as_ptr(), probe: Some(pl061_probe),
};

/* MODULE_DEVICE_TABLE(amba, pl061_ids); module_amba_driver(pl061_gpio_driver); */
/* MODULE_DESCRIPTION("Driver for the ARM PrimeCell(tm) General Purpose Input/Output (PL061)"); */
/* MODULE_LICENSE("GPL v2"); */

/* External kernel types, constants, and helper functions are supplied by the surrounding kernel translation. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
