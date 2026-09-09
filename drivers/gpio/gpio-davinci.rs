// SPDX-License-Identifier: GPL-2.0-or-later
/* TI DaVinci GPIO Support */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const MAX_REGS_BANKS: usize = 5;
const MAX_INT_PER_BANK: usize = 32;
const BINTEN: usize = 0x8; // GPIO Interrupt Per-Bank Enable Register

#[repr(C)]
pub struct davinci_gpio_regs {
    pub dir: u32, pub out_data: u32, pub set_data: u32, pub clr_data: u32,
    pub in_data: u32, pub set_rising: u32, pub clr_rising: u32,
    pub set_falling: u32, pub clr_falling: u32, pub intstat: u32,
}

pub type gpio_get_irq_chip_cb_t = unsafe extern "C" fn(irq: u32) -> *mut irq_chip;
static mut gpio_base: *mut u8 = core::ptr::null_mut();
static mut offset_array: [usize; 5] = [0x10, 0x38, 0x60, 0x88, 0xb0];

#[repr(C)]
pub struct davinci_gpio_irq_data {
    pub regs: *mut davinci_gpio_regs,
    pub chip: *mut davinci_gpio_controller,
    pub bank_num: i32,
}

#[repr(C)]
pub struct davinci_gpio_controller {
    pub chip: gpio_chip,
    pub irq_domain: *mut irq_domain,
    pub lock: spinlock_t,
    pub regs: [*mut davinci_gpio_regs; MAX_REGS_BANKS],
    pub gpio_unbanked: i32,
    pub irqs: [i32; MAX_INT_PER_BANK],
    pub context: [davinci_gpio_regs; MAX_REGS_BANKS],
    pub binten_context: u32,
}

#[inline]
unsafe fn __gpio_mask(gpio: u32) -> u32 { 1u32 << (gpio % 32) }

unsafe extern "C" { fn davinci_gpio_irq_setup(pdev: *mut platform_device) -> i32; }

unsafe fn __davinci_direction(chip: *mut gpio_chip, offset: u32, out: bool, value: i32) -> i32 {
    let d = gpiochip_get_data(chip) as *mut davinci_gpio_controller;
    let g = (*d).regs[(offset / 32) as usize];
    let mask = __gpio_mask(offset);
    let flags: ulong = 0;
    spin_lock_irqsave(&mut (*d).lock, &flags);
    let mut temp = readl_relaxed(&(*g).dir);
    if out { temp &= !mask; writel_relaxed(mask, if value != 0 { &mut (*g).set_data } else { &mut (*g).clr_data }); }
    else { temp |= mask; }
    writel_relaxed(temp, &mut (*g).dir);
    spin_unlock_irqrestore(&mut (*d).lock, flags);
    0
}
unsafe extern "C" fn davinci_direction_in(chip: *mut gpio_chip, offset: u32) -> i32 { __davinci_direction(chip, offset, false, 0) }
unsafe extern "C" fn davinci_direction_out(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 { __davinci_direction(chip, offset, true, value) }
unsafe extern "C" fn davinci_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let d = gpiochip_get_data(chip) as *mut davinci_gpio_controller;
    let g = (*d).regs[(offset / 32) as usize];
    let _guard = spinlock_irqsave_guard(&mut (*d).lock);
    if readl_relaxed(&(*g).dir) & __gpio_mask(offset) != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}
unsafe extern "C" fn davinci_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let d = gpiochip_get_data(chip) as *mut davinci_gpio_controller;
    let g = (*d).regs[(offset / 32) as usize];
    if __gpio_mask(offset) & readl_relaxed(&(*g).in_data) != 0 { 1 } else { 0 }
}
unsafe extern "C" fn davinci_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let d = gpiochip_get_data(chip) as *mut davinci_gpio_controller;
    let g = (*d).regs[(offset / 32) as usize];
    writel_relaxed(__gpio_mask(offset), if value != 0 { &mut (*g).set_data } else { &mut (*g).clr_data }); 0
}

unsafe extern "C" fn davinci_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut ngpio = 0u32; let mut gpio_unbanked = 0u32;
    let mut ret = device_property_read_u32(dev, c"ti,ngpio".as_ptr(), &mut ngpio);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to get the number of GPIOs\n".as_ptr()); }
    if ngpio == 0 { return dev_err_probe(dev, -EINVAL, c"How many GPIOs?\n".as_ptr()); }
    ret = device_property_read_u32(dev, c"ti,davinci-gpio-unbanked".as_ptr(), &mut gpio_unbanked);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to get the unbanked GPIOs property\n".as_ptr()); }
    let nirq = if gpio_unbanked != 0 { gpio_unbanked } else { (ngpio + 15) / 16 };
    if nirq > MAX_INT_PER_BANK as u32 { dev_err(dev, c"Too many IRQs!\n".as_ptr()); return -EINVAL; }
    let chips = devm_kzalloc(dev, core::mem::size_of::<davinci_gpio_controller>(), GFP_KERNEL) as *mut davinci_gpio_controller;
    if chips.is_null() { return -ENOMEM; }
    gpio_base = devm_platform_ioremap_resource(pdev, 0) as *mut u8;
    if IS_ERR(gpio_base as *mut core::ffi::c_void) { return PTR_ERR(gpio_base as *mut core::ffi::c_void); }
    for i in 0..nirq as usize { (*chips).irqs[i] = platform_get_irq(pdev, i as u32); if (*chips).irqs[i] < 0 { return (*chips).irqs[i]; } }
    (*chips).chip.label = dev_name(dev); (*chips).chip.direction_input = Some(davinci_direction_in); (*chips).chip.get = Some(davinci_gpio_get);
    (*chips).chip.direction_output = Some(davinci_direction_out); (*chips).chip.set = Some(davinci_gpio_set); (*chips).chip.get_direction = Some(davinci_get_direction);
    (*chips).chip.ngpio = ngpio; (*chips).chip.base = -1; spin_lock_init(&mut (*chips).lock); (*chips).gpio_unbanked = gpio_unbanked as i32;
    let nbank = (ngpio + 31) / 32;
    for bank in 0..nbank as usize { (*chips).regs[bank] = gpio_base.add(offset_array[bank]) as *mut davinci_gpio_regs; }
    ret = devm_gpiochip_add_data(dev, &mut (*chips).chip, chips as *mut core::ffi::c_void); if ret != 0 { return ret; }
    platform_set_drvdata(pdev, chips as *mut core::ffi::c_void); ret = davinci_gpio_irq_setup(pdev); if ret != 0 { return ret; } 0
}

unsafe extern "C" fn gpio_irq_mask(d: *mut irq_data) {
    let chips = irq_data_get_irq_chip_data(d) as *mut davinci_gpio_controller;
    let hwirq = irqd_to_hwirq(d); let g = (*chips).regs[(hwirq / 32) as usize];
    let mask = irq_data_get_irq_handler_data(d) as usize as u32;
    writel_relaxed(mask, &mut (*g).clr_falling); writel_relaxed(mask, &mut (*g).clr_rising);
    gpiochip_disable_irq(&mut (*chips).chip, hwirq);
}
unsafe extern "C" fn gpio_irq_unmask(d: *mut irq_data) {
    let chips = irq_data_get_irq_chip_data(d) as *mut davinci_gpio_controller;
    let hwirq = irqd_to_hwirq(d); let g = (*chips).regs[(hwirq / 32) as usize];
    let mask = irq_data_get_irq_handler_data(d) as usize as u32;
    gpiochip_enable_irq(&mut (*chips).chip, hwirq);
    let mut status = irqd_get_trigger_type(d) & IRQ_TYPE_EDGE_BOTH;
    if status == 0 { status = IRQ_TYPE_EDGE_BOTH; }
    if status & IRQ_TYPE_EDGE_FALLING != 0 { writel_relaxed(mask, &mut (*g).set_falling); }
    if status & IRQ_TYPE_EDGE_RISING != 0 { writel_relaxed(mask, &mut (*g).set_rising); }
}
unsafe extern "C" fn gpio_irq_type(_d: *mut irq_data, trigger: u32) -> i32 { if trigger & !IRQ_TYPE_EDGE_BOTH != 0 { -EINVAL } else { 0 } }

#[repr(C)]
static mut gpio_irqchip: irq_chip = irq_chip { name: c"GPIO".as_ptr(), irq_unmask: Some(gpio_irq_unmask), irq_mask: Some(gpio_irq_mask), irq_set_type: Some(gpio_irq_type), flags: IRQCHIP_IMMUTABLE | IRQCHIP_SET_TYPE_MASKED | IRQCHIP_SKIP_SET_WAKE };

unsafe extern "C" fn gpio_irq_handler(desc: *mut irq_desc) {
    let irqdata = irq_desc_get_handler_data(desc) as *mut davinci_gpio_irq_data;
    let bank_num = (*irqdata).bank_num; let g = (*irqdata).regs; let d = (*irqdata).chip;
    let mut mask = 0xffffu32; if bank_num % 2 == 1 { mask <<= 16; }
    chained_irq_enter(irq_desc_get_chip(desc), desc);
    loop {
        let mut status = readl_relaxed(&(*g).intstat) & mask; if status == 0 { break; }
        writel_relaxed(status, &mut (*g).intstat);
        while status != 0 { let bit = status.trailing_zeros(); status &= !(1 << bit); let hw_irq = (bank_num as u32 / 2) * 32 + bit; generic_handle_domain_irq((*d).irq_domain, hw_irq); }
    }
    chained_irq_exit(irq_desc_get_chip(desc), desc);
}
unsafe extern "C" fn gpio_to_irq_banked(chip: *mut gpio_chip, offset: u32) -> i32 { let d = gpiochip_get_data(chip) as *mut davinci_gpio_controller; if !(*d).irq_domain.is_null() { irq_create_mapping((*d).irq_domain, offset) } else { -ENXIO } }
unsafe extern "C" fn gpio_to_irq_unbanked(chip: *mut gpio_chip, offset: u32) -> i32 { let d = gpiochip_get_data(chip) as *mut davinci_gpio_controller; if offset < (*d).gpio_unbanked as u32 { (*d).irqs[offset as usize] } else { -ENODEV } }
unsafe extern "C" fn gpio_irq_type_unbanked(data: *mut irq_data, trigger: u32) -> i32 {
    let d = irq_data_get_irq_handler_data(data) as *mut davinci_gpio_controller; let g = (*d).regs[0];
    let mut i = 0; while i < MAX_INT_PER_BANK && (*data).irq != (*d).irqs[i] { i += 1; }
    if i == MAX_INT_PER_BANK || trigger & !IRQ_TYPE_EDGE_BOTH != 0 { return -EINVAL; }
    let mask = __gpio_mask(i as u32); writel_relaxed(mask, if trigger & IRQ_TYPE_EDGE_FALLING != 0 { &mut (*g).set_falling } else { &mut (*g).clr_falling }); writel_relaxed(mask, if trigger & IRQ_TYPE_EDGE_RISING != 0 { &mut (*g).set_rising } else { &mut (*g).clr_rising }); 0
}
unsafe extern "C" fn davinci_gpio_irq_map(domain: *mut irq_domain, irq: u32, hw: u64) -> i32 {
    let chips = (*domain).host_data as *mut davinci_gpio_controller; irq_set_chip_and_handler_name(irq, &mut gpio_irqchip, handle_simple_irq, c"davinci_gpio".as_ptr()); irq_set_irq_type(irq, IRQ_TYPE_NONE); irq_set_chip_data(irq, chips as *mut _); irq_set_handler_data(irq, __gpio_mask(hw as u32) as usize as *mut _); 0
}

unsafe fn davinci_gpio_save_context(chips: *mut davinci_gpio_controller, nbank: u32) {
    let base = (*chips).regs[0] as *mut u8; (*chips).binten_context = readl_relaxed(base.add(BINTEN) as *const u32);
    for bank in 0..nbank as usize { let g = (*chips).regs[bank]; let c = &mut (*chips).context[bank]; c.dir = readl_relaxed(&(*g).dir); c.set_data = readl_relaxed(&(*g).set_data); c.set_rising = readl_relaxed(&(*g).set_rising); c.set_falling = readl_relaxed(&(*g).set_falling); }
}
unsafe fn davinci_gpio_restore_context(chips: *mut davinci_gpio_controller, nbank: u32) {
    let base = (*chips).regs[0] as *mut u8; if readl_relaxed(base.add(BINTEN) as *const u32) != (*chips).binten_context { writel_relaxed((*chips).binten_context, base.add(BINTEN) as *mut u32); }
    for bank in 0..nbank as usize { let g = (*chips).regs[bank]; let c = &(*chips).context[bank]; if readl_relaxed(&(*g).dir) != c.dir { writel_relaxed(c.dir, &mut (*g).dir); } if readl_relaxed(&(*g).set_data) != c.set_data { writel_relaxed(c.set_data, &mut (*g).set_data); } if readl_relaxed(&(*g).set_rising) != c.set_rising { writel_relaxed(c.set_rising, &mut (*g).set_rising); } if readl_relaxed(&(*g).set_falling) != c.set_falling { writel_relaxed(c.set_falling, &mut (*g).set_falling); } }
}
unsafe extern "C" fn davinci_gpio_suspend(dev: *mut device) -> i32 { let c = dev_get_drvdata(dev) as *mut davinci_gpio_controller; davinci_gpio_save_context(c, ((*c).chip.ngpio + 31) / 32); 0 }
unsafe extern "C" fn davinci_gpio_resume(dev: *mut device) -> i32 { let c = dev_get_drvdata(dev) as *mut davinci_gpio_controller; davinci_gpio_restore_context(c, ((*c).chip.ngpio + 31) / 32); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
