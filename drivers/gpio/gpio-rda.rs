// SPDX-License-Identifier: GPL-2.0-only
/*
 * RDA Micro GPIO driver
 *
 * Copyright (C) 2012 RDA Micro Inc.
 * Copyright (C) 2019 Manivannan Sadhasivam
 */

// Linux dependencies supplied by the surrounding kernel bindings.

const RDA_GPIO_OEN_VAL: u16 = 0x00;
const RDA_GPIO_OEN_SET_OUT: u16 = 0x04;
const RDA_GPIO_OEN_SET_IN: u16 = 0x08;
const RDA_GPIO_VAL: u16 = 0x0c;
const RDA_GPIO_SET: u16 = 0x10;
const RDA_GPIO_CLR: u16 = 0x14;
const RDA_GPIO_INT_CTRL_SET: u16 = 0x18;
const RDA_GPIO_INT_CTRL_CLR: u16 = 0x1c;
const RDA_GPIO_INT_CLR: u16 = 0x20;
const RDA_GPIO_INT_STATUS: u16 = 0x24;

const RDA_GPIO_IRQ_RISE_SHIFT: u32 = 0;
const RDA_GPIO_IRQ_FALL_SHIFT: u32 = 8;
const RDA_GPIO_DEBOUCE_SHIFT: u32 = 16;
const RDA_GPIO_LEVEL_SHIFT: u32 = 24;
const RDA_GPIO_IRQ_MASK: u32 = 0xff;
// Each bank consists of 32 GPIOs
const RDA_GPIO_BANK_NR: usize = 32;

#[repr(C)]
struct RdaGpio {
    chip: GpioGenericChip,
    base: *mut core::ffi::c_void,
    lock: Spinlock,
    irq: i32,
}

// External kernel types and functions are supplied by other translation units.
#[repr(C)] struct GpioGenericChip { gc: GpioChip }
#[repr(C)] struct GpioChip { label: *const core::ffi::c_char, ngpio: u32, base: i32, irq: GpioIrqChip }
#[repr(C)] struct GpioIrqChip { _private: [u8; 0] }
#[repr(C)] struct Spinlock { _private: [u8; 0] }
#[repr(C)] struct IrqData { _private: [u8; 0] }
#[repr(C)] struct IrqDesc { _private: [u8; 0] }
#[repr(C)] struct IrqChip { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { _private: [u8; 0] }
#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct GpioGenericChipConfig { dev: *mut Device, sz: u32, dat: *mut u8, set: *mut u8, clr: *mut u8, dirout: *mut u8, dirin: *mut u8, flags: u32 }

unsafe fn rda_gpio_update(chip: *mut GpioChip, offset: u32, reg: u16, val: i32) {
    let rda_gpio = gpiochip_get_data(chip);
    let base = (*rda_gpio).base;
    let flags: usize;
    let mut tmp: u32;
    spin_lock_irqsave(&mut (*rda_gpio).lock, &mut { flags = 0; flags });
    tmp = readl_relaxed((base as *mut u8).add(reg as usize) as *const u32);
    if val != 0 { tmp |= 1u32.wrapping_shl(offset); }
    else { tmp &= !(1u32.wrapping_shl(offset)); }
    writel_relaxed(tmp, (base as *mut u8).add(reg as usize) as *mut u32);
    spin_unlock_irqrestore(&mut (*rda_gpio).lock, flags);
}

unsafe fn rda_gpio_irq_mask(data: *mut IrqData) {
    let chip = irq_data_get_irq_chip_data(data);
    let rda_gpio = gpiochip_get_data(chip);
    let base = (*rda_gpio).base;
    let offset = irqd_to_hwirq(data);
    let mut value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_RISE_SHIFT);
    value |= 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_FALL_SHIFT);
    writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_CLR as usize) as *mut u32);
    gpiochip_disable_irq(chip, offset);
}

unsafe fn rda_gpio_irq_ack(data: *mut IrqData) {
    let chip = irq_data_get_irq_chip_data(data);
    let offset = irqd_to_hwirq(data);
    rda_gpio_update(chip, offset, RDA_GPIO_INT_CLR, 1);
}

unsafe fn rda_gpio_set_irq(chip: *mut GpioChip, offset: u32, flow_type: u32) -> i32 {
    let rda_gpio = gpiochip_get_data(chip);
    let base = (*rda_gpio).base;
    let mut value: u32;
    match flow_type {
        IRQ_TYPE_EDGE_RISING => { value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_RISE_SHIFT); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_SET as usize) as *mut u32); value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_LEVEL_SHIFT); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_CLR as usize) as *mut u32); }
        IRQ_TYPE_EDGE_FALLING => { value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_FALL_SHIFT); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_SET as usize) as *mut u32); value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_LEVEL_SHIFT); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_CLR as usize) as *mut u32); }
        IRQ_TYPE_EDGE_BOTH => { value = (1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_RISE_SHIFT)) | (1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_FALL_SHIFT)); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_SET as usize) as *mut u32); value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_LEVEL_SHIFT); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_CLR as usize) as *mut u32); }
        IRQ_TYPE_LEVEL_HIGH => { value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_RISE_SHIFT) | 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_LEVEL_SHIFT); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_SET as usize) as *mut u32); }
        IRQ_TYPE_LEVEL_LOW => { value = 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_IRQ_FALL_SHIFT) | 1u32.wrapping_shl(offset).wrapping_shl(RDA_GPIO_LEVEL_SHIFT); writel_relaxed(value, (base as *mut u8).add(RDA_GPIO_INT_CTRL_SET as usize) as *mut u32); }
        _ => return -22,
    }
    0
}

unsafe fn rda_gpio_irq_unmask(data: *mut IrqData) {
    let chip = irq_data_get_irq_chip_data(data);
    let offset = irqd_to_hwirq(data);
    let trigger = irqd_get_trigger_type(data);
    gpiochip_enable_irq(chip, offset);
    rda_gpio_set_irq(chip, offset, trigger);
}

unsafe fn rda_gpio_irq_set_type(data: *mut IrqData, flow_type: u32) -> i32 {
    let chip = irq_data_get_irq_chip_data(data);
    let offset = irqd_to_hwirq(data);
    let ret = rda_gpio_set_irq(chip, offset, flow_type);
    if ret != 0 { return ret; }
    if flow_type & IRQ_TYPE_LEVEL_MASK != 0 { irq_set_handler_locked(data, handle_level_irq); }
    else if flow_type & IRQ_TYPE_EDGE_BOTH != 0 { irq_set_handler_locked(data, handle_edge_irq); }
    0
}

unsafe fn rda_gpio_irq_handler(desc: *mut IrqDesc) {
    let chip = irq_desc_get_handler_data(desc);
    let ic = irq_desc_get_chip(desc);
    let rda_gpio = gpiochip_get_data(chip);
    chained_irq_enter(ic, desc);
    let mut status = readl_relaxed(((*rda_gpio).base as *mut u8).add(RDA_GPIO_INT_STATUS as usize) as *const u32) as usize;
    status &= RDA_GPIO_IRQ_MASK as usize;
    for n in 0..RDA_GPIO_BANK_NR { if status & (1usize << n) != 0 { generic_handle_domain_irq((*chip).irq.domain, n as u32); } }
    chained_irq_exit(ic, desc);
}

// The remaining driver registration data and probe entry point retain the C
// kernel interfaces; their external types and helpers are supplied elsewhere.
unsafe fn rda_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = platform_device_dev(pdev);
    let rda_gpio = devm_kzalloc(dev, core::mem::size_of::<RdaGpio>(), GFP_KERNEL) as *mut RdaGpio;
    if rda_gpio.is_null() { return -12; }
    let mut ngpios = 0u32;
    let ret = device_property_read_u32(dev, "ngpios\0".as_ptr() as *const _, &mut ngpios);
    if ret < 0 { return ret; }
    (*rda_gpio).irq = platform_get_irq(pdev, 0);
    (*rda_gpio).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*rda_gpio).base) { return ptr_err((*rda_gpio).base); }
    spin_lock_init(&mut (*rda_gpio).lock);
    let config = GpioGenericChipConfig { dev, sz: 4, dat: ((*rda_gpio).base as *mut u8).add(RDA_GPIO_VAL as usize), set: ((*rda_gpio).base as *mut u8).add(RDA_GPIO_SET as usize), clr: ((*rda_gpio).base as *mut u8).add(RDA_GPIO_CLR as usize), dirout: ((*rda_gpio).base as *mut u8).add(RDA_GPIO_OEN_SET_OUT as usize), dirin: ((*rda_gpio).base as *mut u8).add(RDA_GPIO_OEN_SET_IN as usize), flags: GPIO_GENERIC_READ_OUTPUT_REG_SET };
    let ret = gpio_generic_chip_init(&mut (*rda_gpio).chip, &config);
    if ret != 0 { dev_err(dev, "failed to initialize the generic GPIO chip\0".as_ptr() as *const _); return ret; }
    (*rda_gpio).chip.gc.label = dev_name(dev);
    (*rda_gpio).chip.gc.ngpio = ngpios;
    (*rda_gpio).chip.gc.base = -1;
    if (*rda_gpio).irq >= 0 {
        // gpio_irq_chip_set_chip(girq, &rda_gpio_irq_chip);
        // girq->handler = handle_bad_irq;
        // girq->default_type = IRQ_TYPE_NONE;
        // girq->parent_handler = rda_gpio_irq_handler;
        // girq->parent_handler_data = rda_gpio;
        // girq->num_parents = 1;
        // girq->parents = devm_kcalloc(dev, 1, sizeof(*girq->parents), GFP_KERNEL);
        // if (!girq->parents) return -ENOMEM;
        // girq->parents[0] = rda_gpio->irq;
    }
    platform_set_drvdata(pdev, rda_gpio as *mut _);
    devm_gpiochip_add_data(dev, &mut (*rda_gpio).chip.gc, rda_gpio as *mut _)
}

// Device matching, module registration, description, and author metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
