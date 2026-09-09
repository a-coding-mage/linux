// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011-2012 Avionic Design GmbH
 */

// External Linux kernel types, constants, macros, and functions are supplied by
// the surrounding kernel translation environment.

#[repr(C)]
pub struct adnp {
    pub client: *mut i2c_client,
    pub gpio: gpio_chip,
    pub reg_shift: c_uint,
    pub i2c_lock: mutex,
    pub irq_lock: mutex,
    pub irq_enable: *mut u8,
    pub irq_level: *mut u8,
    pub irq_rise: *mut u8,
    pub irq_fall: *mut u8,
    pub irq_high: *mut u8,
    pub irq_low: *mut u8,
}

#[inline]
unsafe fn GPIO_DDR(gpio: *const adnp) -> c_uint { 0x00 << (*gpio).reg_shift }
#[inline]
unsafe fn GPIO_PLR(gpio: *const adnp) -> c_uint { 0x01 << (*gpio).reg_shift }
#[inline]
unsafe fn GPIO_IER(gpio: *const adnp) -> c_uint { 0x02 << (*gpio).reg_shift }
#[inline]
unsafe fn GPIO_ISR(gpio: *const adnp) -> c_uint { 0x03 << (*gpio).reg_shift }
#[inline]
unsafe fn GPIO_PTR(gpio: *const adnp) -> c_uint { 0x04 << (*gpio).reg_shift }

unsafe fn adnp_read(adnp: *mut adnp, offset: c_uint, value: *mut u8) -> c_int {
    let err = i2c_smbus_read_byte_data((*adnp).client, offset);
    if err < 0 {
        dev_err((*adnp).gpio.parent, "%s failed: %d\n", "i2c_smbus_read_byte_data()", err);
        return err;
    }
    *value = err as u8;
    0
}

unsafe fn adnp_write(adnp: *mut adnp, offset: c_uint, value: u8) -> c_int {
    let err = i2c_smbus_write_byte_data((*adnp).client, offset, value);
    if err < 0 {
        dev_err((*adnp).gpio.parent, "%s failed: %d\n", "i2c_smbus_write_byte_data()", err);
        return err;
    }
    0
}

unsafe fn adnp_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let adnp = gpiochip_get_data(chip);
    let reg = offset >> (*adnp).reg_shift;
    let pos = offset & 7;
    let mut value = 0u8;
    let err = adnp_read(adnp, GPIO_PLR(adnp) + reg, &mut value);
    if err < 0 { return err; }
    if (value & BIT(pos)) != 0 { 1 } else { 0 }
}

unsafe fn __adnp_gpio_set(adnp: *mut adnp, offset: c_uint, value: c_int) -> c_int {
    let reg = offset >> (*adnp).reg_shift;
    let pos = offset & 7;
    let mut val = 0u8;
    let err = adnp_read(adnp, GPIO_PLR(adnp) + reg, &mut val);
    if err < 0 { return err; }
    if value != 0 { val |= BIT(pos); } else { val &= !BIT(pos); }
    adnp_write(adnp, GPIO_PLR(adnp) + reg, val)
}

unsafe fn adnp_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let adnp = gpiochip_get_data(chip);
    let _guard = mutex_guard(&mut (*adnp).i2c_lock);
    __adnp_gpio_set(adnp, offset, value)
}

unsafe fn adnp_gpio_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let adnp = gpiochip_get_data(chip);
    let reg = offset >> (*adnp).reg_shift;
    let pos = offset & 7;
    let _guard = mutex_guard(&mut (*adnp).i2c_lock);
    let mut value = 0u8;
    let mut err = adnp_read(adnp, GPIO_DDR(adnp) + reg, &mut value);
    if err < 0 { return err; }
    value &= !BIT(pos);
    err = adnp_write(adnp, GPIO_DDR(adnp) + reg, value);
    if err < 0 { return err; }
    err = adnp_read(adnp, GPIO_DDR(adnp) + reg, &mut value);
    if err < 0 { return err; }
    if value & BIT(pos) != 0 { -EPERM } else { 0 }
}

unsafe fn adnp_gpio_direction_output(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let adnp = gpiochip_get_data(chip);
    let reg = offset >> (*adnp).reg_shift;
    let pos = offset & 7;
    let _guard = mutex_guard(&mut (*adnp).i2c_lock);
    let mut val = 0u8;
    let mut err = adnp_read(adnp, GPIO_DDR(adnp) + reg, &mut val);
    if err < 0 { return err; }
    val |= BIT(pos);
    err = adnp_write(adnp, GPIO_DDR(adnp) + reg, val);
    if err < 0 { return err; }
    err = adnp_read(adnp, GPIO_DDR(adnp) + reg, &mut val);
    if err < 0 { return err; }
    if val & BIT(pos) == 0 { return -EPERM; }
    __adnp_gpio_set(adnp, offset, value);
    0
}

unsafe fn adnp_gpio_dbg_show(s: *mut seq_file, chip: *mut gpio_chip) {
    let adnp = gpiochip_get_data(chip);
    let num_regs = 1u32 << (*adnp).reg_shift;
    for i in 0..num_regs {
        let (mut ddr, mut plr, mut ier, mut isr) = (0u8, 0u8, 0u8, 0u8);
        let _guard = mutex_guard(&mut (*adnp).i2c_lock);
        if adnp_read(adnp, GPIO_DDR(adnp) + i, &mut ddr) < 0 { return; }
        if adnp_read(adnp, GPIO_PLR(adnp) + i, &mut plr) < 0 { return; }
        if adnp_read(adnp, GPIO_IER(adnp) + i, &mut ier) < 0 { return; }
        if adnp_read(adnp, GPIO_ISR(adnp) + i, &mut isr) < 0 { return; }
        drop(_guard);
        for j in 0..8 {
            let bit = (i << (*adnp).reg_shift) + j;
            let direction = if ddr & BIT(j) != 0 { "output" } else { "input " };
            let level = if plr & BIT(j) != 0 { "high" } else { "low " };
            let interrupt = if ier & BIT(j) != 0 { "enabled " } else { "disabled" };
            let pending = if isr & BIT(j) != 0 { "pending" } else { "" };
            seq_printf(s, "%2u: %s %s IRQ %s %s\n", bit, direction, level, interrupt, pending);
        }
    }
}

unsafe fn adnp_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let adnp = data as *mut adnp;
    let num_regs = 1u32 << (*adnp).reg_shift;
    for i in 0..num_regs {
        let base = i << (*adnp).reg_shift;
        let (mut level, mut isr, mut ier) = (0u8, 0u8, 0u8);
        let _guard = mutex_guard(&mut (*adnp).i2c_lock);
        if adnp_read(adnp, GPIO_PLR(adnp) + i, &mut level) < 0 { continue; }
        if adnp_read(adnp, GPIO_ISR(adnp) + i, &mut isr) < 0 { continue; }
        if adnp_read(adnp, GPIO_IER(adnp) + i, &mut ier) < 0 { continue; }
        let changed = level ^ *(*adnp).irq_level.add(i as usize);
        let mut pending = changed & ((*(*adnp).irq_fall.add(i as usize) & !level) |
            (*(*adnp).irq_rise.add(i as usize) & level));
        pending |= (*(*adnp).irq_high.add(i as usize) & level) |
            (*(*adnp).irq_low.add(i as usize) & !level);
        pending &= isr & ier;
        for bit in 0..8 {
            if pending & BIT(bit) != 0 {
                let child_irq = irq_find_mapping((*adnp).gpio.irq.domain, base + bit);
                handle_nested_irq(child_irq);
            }
        }
    }
    IRQ_HANDLED
}

static mut adnp_irq_chip: irq_chip = irq_chip {
    name: "gpio-adnp", irq_mask: Some(adnp_irq_mask), irq_unmask: Some(adnp_irq_unmask),
    irq_set_type: Some(adnp_irq_set_type), irq_bus_lock: Some(adnp_irq_bus_lock),
    irq_bus_sync_unlock: Some(adnp_irq_bus_unlock), flags: IRQCHIP_IMMUTABLE,
};

unsafe fn adnp_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let adnp = gpiochip_get_data(gc);
    let reg = (*d).hwirq >> (*adnp).reg_shift; let pos = (*d).hwirq & 7;
    *(*adnp).irq_enable.add(reg as usize) &= !BIT(pos);
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
}
unsafe fn adnp_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d); let adnp = gpiochip_get_data(gc);
    let reg = (*d).hwirq >> (*adnp).reg_shift; let pos = (*d).hwirq & 7;
    gpiochip_enable_irq(gc, irqd_to_hwirq(d)); *(*adnp).irq_enable.add(reg as usize) |= BIT(pos);
}
unsafe fn adnp_irq_set_type(d: *mut irq_data, typ: c_uint) -> c_int {
    let gc = irq_data_get_irq_chip_data(d); let adnp = gpiochip_get_data(gc);
    let reg = (*d).hwirq >> (*adnp).reg_shift; let pos = (*d).hwirq & 7;
    for (p, mask) in [((*adnp).irq_rise, IRQ_TYPE_EDGE_RISING), ((*adnp).irq_fall, IRQ_TYPE_EDGE_FALLING),
                      ((*adnp).irq_high, IRQ_TYPE_LEVEL_HIGH), ((*adnp).irq_low, IRQ_TYPE_LEVEL_LOW)] {
        if typ & mask != 0 { *p.add(reg as usize) |= BIT(pos); } else { *p.add(reg as usize) &= !BIT(pos); }
    } 0
}
unsafe fn adnp_irq_bus_lock(d: *mut irq_data) { let gc=irq_data_get_irq_chip_data(d); let a=gpiochip_get_data(gc); mutex_lock(&mut (*a).irq_lock); }
unsafe fn adnp_irq_bus_unlock(d: *mut irq_data) {
    let gc=irq_data_get_irq_chip_data(d); let a=gpiochip_get_data(gc); let n=1u32<<(*a).reg_shift;
    let _g=mutex_guard(&mut (*a).i2c_lock); for i in 0..n { adnp_write(a, GPIO_IER(a)+i, *(*a).irq_enable.add(i as usize)); }
    mutex_unlock(&mut (*a).irq_lock);
}

unsafe fn adnp_irq_setup(a: *mut adnp) -> c_int {
    let n=1u32<<(*a).reg_shift; mutex_init(&mut (*a).irq_lock);
    (*a).irq_enable=devm_kcalloc((*a).gpio.parent,n,6,GFP_KERNEL); if (*a).irq_enable.is_null(){return -ENOMEM;}
    (*a).irq_level=(*a).irq_enable.add(n as usize); (*a).irq_rise=(*a).irq_enable.add((n*2) as usize);
    (*a).irq_fall=(*a).irq_enable.add((n*3) as usize); (*a).irq_high=(*a).irq_enable.add((n*4) as usize); (*a).irq_low=(*a).irq_enable.add((n*5) as usize);
    for i in 0..n { let e=adnp_read(a,GPIO_PLR(a)+i,(*a).irq_level.add(i as usize)); if e<0{return e;} let e=adnp_write(a,GPIO_IER(a)+i,0); if e<0{return e;} *(*a).irq_enable.add(i as usize)=0; }
    devm_request_threaded_irq((*a).gpio.parent,(*a).client_irq(),None,Some(adnp_irq),IRQF_TRIGGER_RISING|IRQF_ONESHOT,dev_name((*a).gpio.parent),a)
}

unsafe fn adnp_gpio_setup(a:*mut adnp,n:c_uint,is_irq:bool)->c_int { (*a).reg_shift=get_count_order(n)-3; let c=&mut (*a).gpio; c.direction_input=Some(adnp_gpio_direction_input); c.direction_output=Some(adnp_gpio_direction_output); c.get=Some(adnp_gpio_get); c.set=Some(adnp_gpio_set); c.can_sleep=true; c.base=-1; c.ngpio=n; c.label=(*a).client_name(); c.parent=(*a).client_dev(); c.owner=THIS_MODULE; if is_irq { let e=adnp_irq_setup(a); if e!=0{return e;} c.irq.set_chip(&adnp_irq_chip); c.irq.default_type=IRQ_TYPE_NONE; c.irq.handler=Some(handle_simple_irq); c.irq.threaded=true; } devm_gpiochip_add_data(c.parent,c,a) }

unsafe fn adnp_i2c_probe(client:*mut i2c_client)->c_int { let dev=client_dev(client); let mut n=0u32; let e=device_property_read_u32(dev,"nr-gpios",&mut n); if e<0{return e;} let a=devm_kzalloc(dev,size_of::<adnp>(),GFP_KERNEL) as *mut adnp; if a.is_null(){return -ENOMEM;} let e=devm_mutex_init(dev,&mut (*a).i2c_lock); if e!=0{return e;} (*a).client=client; let e=adnp_gpio_setup(a,n,device_property_read_bool(dev,"interrupt-controller")); if e!=0{return e;} i2c_set_clientdata(client,a); 0 }

static adnp_i2c_id: [i2c_device_id; 2] = [i2c_device_id{name:"gpio-adnp"}, i2c_device_id{name:""}];
static adnp_of_match: [of_device_id; 2] = [of_device_id{compatible:"ad,gpio-adnp"}, of_device_id{compatible:""}];
static mut adnp_i2c_driver: i2c_driver = i2c_driver { name:"gpio-adnp", of_match_table:&adnp_of_match, probe:Some(adnp_i2c_probe), id_table:&adnp_i2c_id };
module_i2c_driver!(adnp_i2c_driver);
MODULE_DESCRIPTION!("Avionic Design N-bit GPIO expander"); MODULE_AUTHOR!("Thierry Reding <thierry.reding@avionic-design.de>"); MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
