// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015-2018 Pengutronix, Uwe Kleine-König <kernel@pengutronix.de>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
struct gpio_siox_ddata {
    gchip: gpio_chip,
    lock: mutex,
    setdata: [u8; 1],
    getdata: [u8; 3],

    irqlock: raw_spinlock_t,
    irq_enable: u32,
    irq_status: u32,
    irq_type: [u32; 20],
}

/*
 * Note that this callback only sets the value that is clocked out in the next
 * cycle.
 */
unsafe fn gpio_siox_set_data(sdevice: *mut siox_device, _status: u8, buf: *mut u8) -> i32 {
    let ddata = dev_get_drvdata((*sdevice).dev);

    mutex_lock(&mut (*ddata).lock);
    *buf = (*ddata).setdata[0];
    mutex_unlock(&mut (*ddata).lock);

    0
}

unsafe fn gpio_siox_get_data(sdevice: *mut siox_device, buf: *const u8) -> i32 {
    let ddata = dev_get_drvdata((*sdevice).dev);
    let mut offset: usize;
    let trigger: u32;

    mutex_lock(&mut (*ddata).lock);
    raw_spin_lock_irq(&mut (*ddata).irqlock);

    offset = 0;
    while offset < 12 {
        let bitpos = 11 - offset;
        let gpiolevel = *buf.add(bitpos / 8) & (1 << (bitpos % 8));
        let prev_level = (*ddata).getdata[bitpos / 8] & (1 << (bitpos % 8));
        let irq_type = (*ddata).irq_type[offset];

        if gpiolevel != 0 {
            if (irq_type & IRQ_TYPE_LEVEL_HIGH) != 0
                || ((irq_type & IRQ_TYPE_EDGE_RISING) != 0 && prev_level == 0)
            {
                (*ddata).irq_status |= 1 << offset;
            }
        } else if (irq_type & IRQ_TYPE_LEVEL_LOW) != 0
            || ((irq_type & IRQ_TYPE_EDGE_FALLING) != 0 && prev_level != 0)
        {
            (*ddata).irq_status |= 1 << offset;
        }
        offset += 1;
    }

    trigger = (*ddata).irq_status & (*ddata).irq_enable;
    raw_spin_unlock_irq(&mut (*ddata).irqlock);

    (*ddata).getdata[0] = *buf.add(0);
    (*ddata).getdata[1] = *buf.add(1);
    (*ddata).getdata[2] = *buf.add(2);
    mutex_unlock(&mut (*ddata).lock);

    offset = 0;
    while offset < 12 {
        if trigger & (1 << offset) != 0 {
            let irqdomain = (*ddata).gchip.irq.domain;
            let irq = irq_find_mapping(irqdomain, offset as u32);

            /*
             * Conceptually handle_nested_irq should call the flow
             * handler of the irq chip. But it doesn't, so we have
             * to clean the irq_status here.
             */
            raw_spin_lock_irq(&mut (*ddata).irqlock);
            (*ddata).irq_status &= !(1 << offset);
            raw_spin_unlock_irq(&mut (*ddata).irqlock);
            handle_nested_irq(irq);
        }
        offset += 1;
    }
    0
}

unsafe fn gpio_siox_irq_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let ddata = gpiochip_get_data(gc);
    raw_spin_lock(&mut (*ddata).irqlock);
    (*ddata).irq_status &= !(1 << (*d).hwirq);
    raw_spin_unlock(&mut (*ddata).irqlock);
}

unsafe fn gpio_siox_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let ddata = gpiochip_get_data(gc);
    raw_spin_lock(&mut (*ddata).irqlock);
    (*ddata).irq_enable &= !(1 << (*d).hwirq);
    raw_spin_unlock(&mut (*ddata).irqlock);
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
}

unsafe fn gpio_siox_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let ddata = gpiochip_get_data(gc);
    gpiochip_enable_irq(gc, irqd_to_hwirq(d));
    raw_spin_lock(&mut (*ddata).irqlock);
    (*ddata).irq_enable |= 1 << (*d).hwirq;
    raw_spin_unlock(&mut (*ddata).irqlock);
}

unsafe fn gpio_siox_irq_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let ddata = gpiochip_get_data(gc);
    raw_spin_lock(&mut (*ddata).irqlock);
    (*ddata).irq_type[(*d).hwirq as usize] = type_;
    raw_spin_unlock(&mut (*ddata).irqlock);
    0
}

unsafe fn gpio_siox_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let ddata = gpiochip_get_data(chip);
    let ret: i32;
    mutex_lock(&mut (*ddata).lock);
    if offset >= 12 {
        let bitpos = 19 - offset;
        ret = ((*ddata).setdata[0] & (1 << bitpos)) as i32;
    } else {
        let bitpos = 11 - offset;
        ret = ((*ddata).getdata[(bitpos / 8) as usize] & (1 << (bitpos % 8))) as i32;
    }
    mutex_unlock(&mut (*ddata).lock);
    ret
}

unsafe fn gpio_siox_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let ddata = gpiochip_get_data(chip);
    let mask: u8 = 1 << (19 - offset);
    mutex_lock(&mut (*ddata).lock);
    if value != 0 { (*ddata).setdata[0] |= mask; } else { (*ddata).setdata[0] &= !mask; }
    mutex_unlock(&mut (*ddata).lock);
    0
}

unsafe fn gpio_siox_direction_input(_chip: *mut gpio_chip, offset: u32) -> i32 {
    if offset >= 12 { return -EINVAL; }
    0
}

unsafe fn gpio_siox_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    if offset < 12 { return -EINVAL; }
    gpio_siox_set(chip, offset, value)
}

unsafe fn gpio_siox_get_direction(_chip: *mut gpio_chip, offset: u32) -> i32 {
    if offset < 12 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

static mut gpio_siox_irq_chip: irq_chip = irq_chip {
    name: "siox-gpio", irq_ack: Some(gpio_siox_irq_ack), irq_mask: Some(gpio_siox_irq_mask),
    irq_unmask: Some(gpio_siox_irq_unmask), irq_set_type: Some(gpio_siox_irq_set_type),
    flags: IRQCHIP_IMMUTABLE,
};

unsafe fn gpio_siox_probe(sdevice: *mut siox_device) -> i32 {
    let dev = &mut (*sdevice).dev;
    let ddata = devm_kzalloc(dev, core::mem::size_of::<gpio_siox_ddata>(), GFP_KERNEL);
    if ddata.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, ddata);
    mutex_init(&mut (*ddata).lock);
    raw_spin_lock_init(&mut (*ddata).irqlock);

    let gc = &mut (*ddata).gchip;
    gc.base = -1;
    gc.can_sleep = 1;
    gc.parent = dev;
    gc.owner = THIS_MODULE;
    gc.get = Some(gpio_siox_get);
    gc.set = Some(gpio_siox_set);
    gc.direction_input = Some(gpio_siox_direction_input);
    gc.direction_output = Some(gpio_siox_direction_output);
    gc.get_direction = Some(gpio_siox_get_direction);
    gc.ngpio = 20;

    let girq = &mut gc.irq;
    gpio_irq_chip_set_chip(girq, &raw mut gpio_siox_irq_chip);
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = Some(handle_level_irq);
    girq.threaded = true;
    let ret = devm_gpiochip_add_data(dev, gc, ddata);
    if ret != 0 { dev_err(dev, "Failed to register gpio chip (%d)\n", ret); }
    ret
}

static mut gpio_siox_driver: siox_driver = siox_driver {
    probe: Some(gpio_siox_probe),
    set_data: Some(gpio_siox_set_data),
    get_data: Some(gpio_siox_get_data),
    driver: device_driver { name: "gpio-siox" },
};

module_siox_driver!(gpio_siox_driver);

module_author!("Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>");
module_description!("SIOX gpio driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
