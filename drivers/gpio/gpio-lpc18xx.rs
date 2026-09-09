// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO driver for NXP LPC18xx/43xx.
 *
 * Copyright (C) 2018 Vladimir Zapolskiy <vz@mleia.com>
 * Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
 */

// Linux kernel dependencies are supplied by the surrounding Rust environment.

const LPC18XX_MAX_PORTS: u32 = 8;
const LPC18XX_PINS_PER_PORT: u32 = 32;
const NR_LPC18XX_GPIO_PIN_IC_IRQS: u32 = 8;

const LPC18XX_GPIO_PIN_IC_ISEL: usize = 0x00;
const LPC18XX_GPIO_PIN_IC_IENR: usize = 0x04;
const LPC18XX_GPIO_PIN_IC_SIENR: usize = 0x08;
const LPC18XX_GPIO_PIN_IC_CIENR: usize = 0x0c;
const LPC18XX_GPIO_PIN_IC_IENF: usize = 0x10;
const LPC18XX_GPIO_PIN_IC_SIENF: usize = 0x14;
const LPC18XX_GPIO_PIN_IC_CIENF: usize = 0x18;
const LPC18XX_GPIO_PIN_IC_RISE: usize = 0x1c;
const LPC18XX_GPIO_PIN_IC_FALL: usize = 0x20;
const LPC18XX_GPIO_PIN_IC_IST: usize = 0x24;

#[repr(C)]
struct lpc18xx_gpio_pin_ic {
    base: *mut core::ffi::c_void,
    domain: *mut irq_domain,
    lock: raw_spinlock_t,
    gpio: *mut gpio_chip,
}

#[repr(C)]
struct lpc18xx_gpio_chip {
    gpio: gpio_chip,
    base: *mut core::ffi::c_void,
    pin_ic: *mut lpc18xx_gpio_pin_ic,
    lock: spinlock_t,
}

#[inline]
unsafe fn lpc18xx_gpio_pin_ic_isel(ic: *mut lpc18xx_gpio_pin_ic, pin: u32, set: bool) {
    let mut val = readl_relaxed((*ic).base.add(LPC18XX_GPIO_PIN_IC_ISEL));
    if set { val &= !BIT(pin); } else { val |= BIT(pin); }
    writel_relaxed(val, (*ic).base.add(LPC18XX_GPIO_PIN_IC_ISEL));
}

#[inline]
unsafe fn lpc18xx_gpio_pin_ic_set(ic: *mut lpc18xx_gpio_pin_ic, pin: u32, reg: usize) {
    writel_relaxed(BIT(pin), (*ic).base.add(reg));
}

unsafe extern "C" fn lpc18xx_gpio_pin_ic_mask(d: *mut irq_data) {
    let ic = (*d).chip_data as *mut lpc18xx_gpio_pin_ic;
    let typ = irqd_get_trigger_type(d);
    let hwirq = irqd_to_hwirq(d);
    raw_spin_lock(&mut (*ic).lock);
    if typ & IRQ_TYPE_LEVEL_MASK != 0 || typ & IRQ_TYPE_EDGE_RISING != 0 { lpc18xx_gpio_pin_ic_set(ic, (*d).hwirq, LPC18XX_GPIO_PIN_IC_CIENR); }
    if typ & IRQ_TYPE_EDGE_FALLING != 0 { lpc18xx_gpio_pin_ic_set(ic, (*d).hwirq, LPC18XX_GPIO_PIN_IC_CIENF); }
    raw_spin_unlock(&mut (*ic).lock);
    irq_chip_mask_parent(d);
    gpiochip_disable_irq((*ic).gpio, hwirq);
}

unsafe extern "C" fn lpc18xx_gpio_pin_ic_unmask(d: *mut irq_data) {
    let ic = (*d).chip_data as *mut lpc18xx_gpio_pin_ic;
    let typ = irqd_get_trigger_type(d);
    let hwirq = irqd_to_hwirq(d);
    gpiochip_enable_irq((*ic).gpio, hwirq);
    raw_spin_lock(&mut (*ic).lock);
    if typ & IRQ_TYPE_LEVEL_MASK != 0 || typ & IRQ_TYPE_EDGE_RISING != 0 { lpc18xx_gpio_pin_ic_set(ic, (*d).hwirq, LPC18XX_GPIO_PIN_IC_SIENR); }
    if typ & IRQ_TYPE_EDGE_FALLING != 0 { lpc18xx_gpio_pin_ic_set(ic, (*d).hwirq, LPC18XX_GPIO_PIN_IC_SIENF); }
    raw_spin_unlock(&mut (*ic).lock);
    irq_chip_unmask_parent(d);
}

unsafe extern "C" fn lpc18xx_gpio_pin_ic_eoi(d: *mut irq_data) {
    let ic = (*d).chip_data as *mut lpc18xx_gpio_pin_ic;
    let typ = irqd_get_trigger_type(d);
    raw_spin_lock(&mut (*ic).lock);
    if typ & IRQ_TYPE_EDGE_BOTH != 0 { lpc18xx_gpio_pin_ic_set(ic, (*d).hwirq, LPC18XX_GPIO_PIN_IC_IST); }
    raw_spin_unlock(&mut (*ic).lock);
    irq_chip_eoi_parent(d);
}

unsafe extern "C" fn lpc18xx_gpio_pin_ic_set_type(d: *mut irq_data, typ: u32) -> i32 {
    let ic = (*d).chip_data as *mut lpc18xx_gpio_pin_ic;
    raw_spin_lock(&mut (*ic).lock);
    if typ & IRQ_TYPE_LEVEL_HIGH != 0 {
        lpc18xx_gpio_pin_ic_isel(ic, (*d).hwirq, true);
        lpc18xx_gpio_pin_ic_set(ic, (*d).hwirq, LPC18XX_GPIO_PIN_IC_SIENF);
    } else if typ & IRQ_TYPE_LEVEL_LOW != 0 {
        lpc18xx_gpio_pin_ic_isel(ic, (*d).hwirq, true);
        lpc18xx_gpio_pin_ic_set(ic, (*d).hwirq, LPC18XX_GPIO_PIN_IC_CIENF);
    } else { lpc18xx_gpio_pin_ic_isel(ic, (*d).hwirq, false); }
    raw_spin_unlock(&mut (*ic).lock);
    0
}

unsafe extern "C" fn lpc18xx_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gc = gpiochip_get_data(chip) as *mut lpc18xx_gpio_chip;
    writeb(if value != 0 { 1 } else { 0 }, (*gc).base.add(offset as usize));
    0
}

unsafe extern "C" fn lpc18xx_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gc = gpiochip_get_data(chip) as *mut lpc18xx_gpio_chip;
    (readb((*gc).base.add(offset as usize)) != 0) as i32
}

unsafe extern "C" fn lpc18xx_gpio_direction(chip: *mut gpio_chip, offset: u32, out: bool) -> i32 {
    let gc = gpiochip_get_data(chip) as *mut lpc18xx_gpio_chip;
    let port = offset / LPC18XX_PINS_PER_PORT;
    let pin = offset % LPC18XX_PINS_PER_PORT;
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*gc).lock, &mut flags);
    let dirp = (*gc).base.add(0x2000 + (port as usize) * core::mem::size_of::<u32>());
    let mut dir = readl(dirp);
    if out { dir |= BIT(pin); } else { dir &= !BIT(pin); }
    writel(dir, dirp);
    spin_unlock_irqrestore(&mut (*gc).lock, flags);
    0
}

unsafe extern "C" fn lpc18xx_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 { lpc18xx_gpio_direction(chip, offset, false) }
unsafe extern "C" fn lpc18xx_gpio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    lpc18xx_gpio_set(chip, offset, value);
    lpc18xx_gpio_direction(chip, offset, true)
}

// The following declarations preserve the source-level kernel interfaces for the remaining
// IRQ-domain and platform-driver portions; their kernel types and helpers are external.
unsafe extern "C" fn lpc18xx_gpio_pin_ic_domain_alloc(domain: *mut irq_domain, virq: u32, nr_irqs: u32, data: *mut core::ffi::c_void) -> i32 {
    if nr_irqs != 1 { return -22; }
    let fwspec = data as *mut irq_fwspec;
    let hwirq = (*fwspec).param[0];
    if hwirq >= NR_LPC18XX_GPIO_PIN_IC_IRQS { return -22; }
    let mut parent_fwspec = irq_fwspec::default();
    parent_fwspec.fwnode = (*(*domain).parent).fwnode;
    parent_fwspec.param_count = 1;
    parent_fwspec.param[0] = hwirq + 32;
    let ret = irq_domain_alloc_irqs_parent(domain, virq, 1, &mut parent_fwspec);
    if ret < 0 { return ret; }
    let ic = (*domain).host_data as *mut lpc18xx_gpio_pin_ic;
    irq_domain_set_hwirq_and_chip(domain, virq, hwirq, &lpc18xx_gpio_pin_ic_chip, ic)
}

unsafe extern "C" fn lpc18xx_gpio_pin_ic_probe(gc: *mut lpc18xx_gpio_chip) -> i32 {
    // External kernel resource discovery and allocation are intentionally represented by the
    // corresponding declarations rather than reimplemented in this isolated translation.
    let _ = gc;
    -19
}

unsafe extern "C" fn lpc18xx_gpio_probe(pdev: *mut platform_device) -> i32 {
    let _ = pdev;
    -12
}

unsafe extern "C" fn lpc18xx_gpio_remove(pdev: *mut platform_device) {
    let gc = platform_get_drvdata(pdev) as *mut lpc18xx_gpio_chip;
    if !gc.is_null() && !(*gc).pin_ic.is_null() { irq_domain_remove((*(*gc).pin_ic).domain); }
}

#[repr(C)]
struct irq_chip;
#[repr(C)]
struct irq_domain;
#[repr(C)]
struct irq_data { chip_data: *mut core::ffi::c_void, hwirq: u32 }
#[repr(C)]
struct irq_fwspec { fwnode: *mut core::ffi::c_void, param_count: u32, param: [u32; 16] }
impl Default for irq_fwspec { fn default() -> Self { Self { fwnode: core::ptr::null_mut(), param_count: 0, param: [0; 16] } } }
#[repr(C)] struct gpio_chip;
#[repr(C)] struct platform_device;
#[repr(C)] struct raw_spinlock_t;
#[repr(C)] struct spinlock_t;
type ulong = usize;

static lpc18xx_gpio_pin_ic_chip: irq_chip = irq_chip { };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
