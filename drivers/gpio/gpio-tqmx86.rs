// SPDX-License-Identifier: GPL-2.0
/*
 * TQ-Systems TQMx86 PLD GPIO driver
 *
 * Based on vendor driver by:
 *   Vadim V.Vlasov <vvlasov@dev.rtsoft.ru>
 */

// Linux kernel dependencies are supplied by the surrounding Rust bindings.

const TQMX86_NGPIO: usize = 8;
const TQMX86_NGPO: usize = 4; // 0-3 - output
const TQMX86_NGPI: usize = 4; // 4-7 - input
const TQMX86_DIR_INPUT_MASK: u8 = 0xf0; // 0-3 - output, 4-7 - input

const TQMX86_GPIODD: u32 = 0; // GPIO Data Direction Register
const TQMX86_GPIOD: u32 = 1; // GPIO Data Register
const TQMX86_GPIIC: u32 = 3; // GPI Interrupt Configuration Register
const TQMX86_GPIIS: u32 = 4; // GPI Interrupt Status Register

/*
 * NONE, FALLING and RISING use the same bit patterns that can be programmed to
 * the GPII register (after passing them to the TQMX86_GPII_ macros to shift
 * them to the right position)
 */
const TQMX86_INT_TRIG_NONE: u8 = 0;
const TQMX86_INT_TRIG_FALLING: u8 = 1 << 0;
const TQMX86_INT_TRIG_RISING: u8 = 1 << 1;
const TQMX86_INT_TRIG_BOTH: u8 = (1 << 0) | (1 << 1);
const TQMX86_INT_TRIG_MASK: u8 = (1 << 0) | (1 << 1);
/* Stored in irq_type with GPII bits */
const TQMX86_INT_UNMASKED: u8 = 1 << 2;

#[repr(C)]
pub struct tqmx86_gpio_data {
    pub chip: gpio_chip,
    pub io_base: *mut core::ffi::c_void,
    pub irq: i32,
    /* Lock must be held for accessing output and irq_type fields */
    pub spinlock: raw_spinlock_t,
    pub output: [usize; 1],
    pub irq_type: [u8; TQMX86_NGPIO],
}

unsafe fn tqmx86_gpio_read(gd: *mut tqmx86_gpio_data, reg: u32) -> u8 {
    ioread8((*gd).io_base.add(reg as usize))
}

unsafe fn tqmx86_gpio_write(gd: *mut tqmx86_gpio_data, val: u8, reg: u32) {
    iowrite8(val, (*gd).io_base.add(reg as usize));
}

unsafe fn tqmx86_gpio_clrsetbits(gpio: *mut tqmx86_gpio_data, clr: u8, set: u8, reg: u32) {
    let mut val = tqmx86_gpio_read(gpio, reg);
    val &= !clr;
    val |= set;
    tqmx86_gpio_write(gpio, val, reg);
}

unsafe extern "C" fn tqmx86_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut tqmx86_gpio_data;
    ((tqmx86_gpio_read(gpio, TQMX86_GPIOD) & (1u8 << offset)) != 0) as i32
}

unsafe fn _tqmx86_gpio_set(gpio: *mut tqmx86_gpio_data, offset: u32, value: i32) {
    let word = &mut (*gpio).output[0];
    if value != 0 { *word |= 1usize << offset; } else { *word &= !(1usize << offset); }
    tqmx86_gpio_write(gpio, *word as u8, TQMX86_GPIOD);
}

unsafe extern "C" fn tqmx86_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut tqmx86_gpio_data;
    let _guard = raw_spinlock_irqsave_guard(&mut (*gpio).spinlock);
    _tqmx86_gpio_set(gpio, offset, value);
    0
}

unsafe extern "C" fn tqmx86_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut tqmx86_gpio_data;
    let _guard = raw_spinlock_irqsave_guard(&mut (*gpio).spinlock);
    tqmx86_gpio_clrsetbits(gpio, 1u8 << offset, 0, TQMX86_GPIODD);
    0
}

unsafe extern "C" fn tqmx86_gpio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut tqmx86_gpio_data;
    let _guard = raw_spinlock_irqsave_guard(&mut (*gpio).spinlock);
    _tqmx86_gpio_set(gpio, offset, value);
    tqmx86_gpio_clrsetbits(gpio, 0, 1u8 << offset, TQMX86_GPIODD);
    0
}

unsafe extern "C" fn tqmx86_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut tqmx86_gpio_data;
    if tqmx86_gpio_read(gpio, TQMX86_GPIODD) & (1u8 << offset) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn tqmx86_gpio_irq_config(gpio: *mut tqmx86_gpio_data, hwirq: i32) {
    let mut kind = TQMX86_INT_TRIG_NONE;
    let gpiic_irq = hwirq - TQMX86_NGPO as i32;
    if (*gpio).irq_type[hwirq as usize] & TQMX86_INT_UNMASKED != 0 {
        kind = (*gpio).irq_type[hwirq as usize] & TQMX86_INT_TRIG_MASK;
        if kind == TQMX86_INT_TRIG_BOTH { kind = if tqmx86_gpio_get(&mut (*gpio).chip, hwirq as u32) != 0 { TQMX86_INT_TRIG_FALLING } else { TQMX86_INT_TRIG_RISING }; }
    }
    tqmx86_gpio_clrsetbits(gpio, TQMX86_INT_TRIG_MASK << (2 * gpiic_irq), kind << (2 * gpiic_irq), TQMX86_GPIIC);
}

unsafe extern "C" fn tqmx86_gpio_irq_mask(data: *mut irq_data) {
    let gpio = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut tqmx86_gpio_data;
    let _guard = raw_spinlock_irqsave_guard(&mut (*gpio).spinlock);
    (*gpio).irq_type[(*data).hwirq as usize] &= !TQMX86_INT_UNMASKED;
    tqmx86_gpio_irq_config(gpio, (*data).hwirq as i32);
    gpiochip_disable_irq(&mut (*gpio).chip, irqd_to_hwirq(data));
}

unsafe extern "C" fn tqmx86_gpio_irq_unmask(data: *mut irq_data) {
    let gpio = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut tqmx86_gpio_data;
    gpiochip_enable_irq(&mut (*gpio).chip, irqd_to_hwirq(data));
    let _guard = raw_spinlock_irqsave_guard(&mut (*gpio).spinlock);
    (*gpio).irq_type[(*data).hwirq as usize] |= TQMX86_INT_UNMASKED;
    tqmx86_gpio_irq_config(gpio, (*data).hwirq as i32);
}

unsafe extern "C" fn tqmx86_gpio_irq_set_type(data: *mut irq_data, ty: u32) -> i32 {
    let gpio = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut tqmx86_gpio_data;
    let mut new_type;
    match ty & IRQF_TRIGGER_MASK { IRQ_TYPE_EDGE_RISING => new_type = TQMX86_INT_TRIG_RISING, IRQ_TYPE_EDGE_FALLING => new_type = TQMX86_INT_TRIG_FALLING, IRQ_TYPE_EDGE_BOTH => new_type = TQMX86_INT_TRIG_BOTH, _ => return -EINVAL }
    let _guard = raw_spinlock_irqsave_guard(&mut (*gpio).spinlock);
    (*gpio).irq_type[(*data).hwirq as usize] &= !TQMX86_INT_TRIG_MASK;
    (*gpio).irq_type[(*data).hwirq as usize] |= new_type;
    tqmx86_gpio_irq_config(gpio, (*data).hwirq as i32);
    0
}

// Edge-both triggers are implemented by flipping the edge trigger after each interrupt.
unsafe extern "C" fn tqmx86_gpio_irq_handler(desc: *mut irq_desc) {
    let chip = irq_desc_get_handler_data(desc) as *mut gpio_chip;
    let gpio = gpiochip_get_data(chip) as *mut tqmx86_gpio_data;
    let irq_chip = irq_desc_get_chip(desc);
    chained_irq_enter(irq_chip, desc);
    let irq_status = tqmx86_gpio_read(gpio, TQMX86_GPIIS);
    tqmx86_gpio_write(gpio, irq_status, TQMX86_GPIIS);
    let _guard = raw_spinlock_irqsave_guard(&mut (*gpio).spinlock);
    for i in 0..TQMX86_NGPI { if irq_status & (1 << i) != 0 { let hwirq = i + TQMX86_NGPO; if (*gpio).irq_type[hwirq] & TQMX86_INT_TRIG_MASK == TQMX86_INT_TRIG_BOTH { tqmx86_gpio_irq_config(gpio, hwirq as i32); } } }
    drop(_guard);
    for i in 0..TQMX86_NGPI { if irq_status & (1 << i) != 0 { generic_handle_domain_irq((*chip).irq.domain, (i + TQMX86_NGPO) as u32); } }
    chained_irq_exit(irq_chip, desc);
}

unsafe extern "C" fn tqmx86_gpio_runtime_suspend(_dev: *mut device) -> i32 { 0 }
unsafe extern "C" fn tqmx86_gpio_runtime_resume(_dev: *mut device) -> i32 { 0 }

unsafe extern "C" fn tqmx86_init_irq_valid_mask(_chip: *mut gpio_chip, valid_mask: *mut usize, _ngpios: u32) {
    *valid_mask &= !0x0f;
}

unsafe extern "C" fn tqmx86_gpio_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    seq_puts(p, (*gc).label);
}

#[repr(C)]
static mut tqmx86_gpio_irq_chip: irq_chip = irq_chip {
    irq_mask: Some(tqmx86_gpio_irq_mask), irq_unmask: Some(tqmx86_gpio_irq_unmask),
    irq_set_type: Some(tqmx86_gpio_irq_set_type), irq_print_chip: Some(tqmx86_gpio_irq_print_chip),
};

unsafe extern "C" fn tqmx86_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let gpio = devm_kzalloc(dev, core::mem::size_of::<tqmx86_gpio_data>(), GFP_KERNEL) as *mut tqmx86_gpio_data;
    if gpio.is_null() { return -ENOMEM; }
    raw_spin_lock_init(&mut (*gpio).spinlock);
    let res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if res.is_null() { dev_err(dev, "Cannot get I/O\n"); return -ENODEV; }
    (*gpio).io_base = devm_ioport_map(dev, (*res).start, resource_size(res));
    if (*gpio).io_base.is_null() { return -ENOMEM; }
    tqmx86_gpio_write(gpio, !TQMX86_DIR_INPUT_MASK, TQMX86_GPIODD);
    tqmx86_gpio_write(gpio, 0, TQMX86_GPIOD);
    let chip = &mut (*gpio).chip;
    chip.label = "gpio-tqmx86";
    chip.owner = THIS_MODULE;
    chip.can_sleep = false;
    chip.base = -1;
    chip.direction_input = Some(tqmx86_gpio_direction_input);
    chip.direction_output = Some(tqmx86_gpio_direction_output);
    chip.get_direction = Some(tqmx86_gpio_get_direction);
    chip.get = Some(tqmx86_gpio_get);
    chip.set = Some(tqmx86_gpio_set);
    chip.ngpio = TQMX86_NGPIO as u32;
    chip.parent = (*pdev).dev.parent;
    pm_runtime_enable(dev);
    let irq = platform_get_irq_optional(pdev, 0);
    if irq > 0 {
        tqmx86_gpio_write(gpio, 0, TQMX86_GPIIC);
        let status = tqmx86_gpio_read(gpio, TQMX86_GPIIS);
        tqmx86_gpio_write(gpio, status, TQMX86_GPIIS);
        let girq = &mut chip.irq;
        gpio_irq_chip_set_chip(girq, &raw mut tqmx86_gpio_irq_chip);
        girq.parent_handler = Some(tqmx86_gpio_irq_handler);
        girq.num_parents = 1;
        girq.default_type = IRQ_TYPE_NONE;
        girq.handler = Some(handle_simple_irq);
        girq.init_valid_mask = Some(tqmx86_init_irq_valid_mask);
        irq_domain_set_pm_device(girq.domain, dev);
    }
    let ret = devm_gpiochip_add_data(dev, chip, gpio as *mut core::ffi::c_void);
    if ret != 0 { dev_err(dev, "Could not register GPIO chip\n"); pm_runtime_disable(dev); return ret; }
    dev_info(dev, "GPIO functionality initialized with %d pins\n", chip.ngpio);
    0
}

static mut tqmx86_gpio_driver: platform_driver = platform_driver { probe: Some(tqmx86_gpio_probe) };

// module_platform_driver(tqmx86_gpio_driver);
// MODULE_DESCRIPTION("TQMx86 PLD GPIO Driver");
// MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:tqmx86-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
