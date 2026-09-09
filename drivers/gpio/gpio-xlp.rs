// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2003-2015 Broadcom Corporation
 * All Rights Reserved
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const GPIO_9XX_BYTESWAP: usize = 0x00;
const GPIO_9XX_CTRL: usize = 0x04;
const GPIO_9XX_OUTPUT_EN: usize = 0x14;
const GPIO_9XX_PADDRV: usize = 0x24;
const GPIO_9XX_INT_EN00: usize = 0x44;
const GPIO_9XX_INT_EN10: usize = 0x54;
const GPIO_9XX_INT_EN20: usize = 0x64;
const GPIO_9XX_INT_EN30: usize = 0x74;
const GPIO_9XX_INT_POL: usize = 0x104;
const GPIO_9XX_INT_TYPE: usize = 0x114;
const GPIO_9XX_INT_STAT: usize = 0x124;

const XLP_GPIO_IRQ_TYPE_LVL: u32 = 0x0;
const XLP_GPIO_IRQ_TYPE_EDGE: u32 = 0x1;
const XLP_GPIO_IRQ_POL_HIGH: u32 = 0x0;
const XLP_GPIO_IRQ_POL_LOW: u32 = 0x1;
const XLP_GPIO_REGSZ: usize = 32;
const XLP_GPIO_IRQ_BASE: usize = 768;
const XLP_MAX_NR_GPIO: usize = 96;

#[repr(C)]
pub struct xlp_gpio_priv {
    pub chip: gpio_chip,
    pub gpio_enabled_mask: [usize; (XLP_MAX_NR_GPIO + usize::BITS as usize - 1) / usize::BITS as usize],
    pub gpio_intr_en: *mut core::ffi::c_void,
    pub gpio_intr_stat: *mut core::ffi::c_void,
    pub gpio_intr_type: *mut core::ffi::c_void,
    pub gpio_intr_pol: *mut core::ffi::c_void,
    pub gpio_out_en: *mut core::ffi::c_void,
    pub gpio_paddrv: *mut core::ffi::c_void,
    pub lock: spinlock_t,
}

unsafe fn xlp_gpio_get_reg(addr: *mut core::ffi::c_void, gpio: u32) -> i32 {
    let pos = gpio % XLP_GPIO_REGSZ as u32;
    let regset = (gpio / XLP_GPIO_REGSZ as u32) * 4;
    ((readl((addr as *mut u8).add(regset as usize) as *mut u32) & (1u32 << pos)) != 0) as i32
}

unsafe fn xlp_gpio_set_reg(addr: *mut core::ffi::c_void, gpio: u32, state: i32) {
    let pos = gpio % XLP_GPIO_REGSZ as u32;
    let regset = (gpio / XLP_GPIO_REGSZ as u32) * 4;
    let p = (addr as *mut u8).add(regset as usize) as *mut u32;
    let mut value = readl(p);
    if state != 0 { value |= 1u32 << pos; } else { value &= !(1u32 << pos); }
    writel(value, p);
}

unsafe fn xlp_gpio_irq_enable(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    gpiochip_enable_irq(gc, irqd_to_hwirq(d));
}

unsafe fn xlp_gpio_irq_disable(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let priv_ = gpiochip_get_data(gc) as *mut xlp_gpio_priv;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*priv_).lock, &mut flags);
    xlp_gpio_set_reg((*priv_).gpio_intr_en, (*d).hwirq as u32, 0);
    __clear_bit((*d).hwirq as usize, (*priv_).gpio_enabled_mask.as_mut_ptr());
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
}

unsafe fn xlp_gpio_irq_mask_ack(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let priv_ = gpiochip_get_data(gc) as *mut xlp_gpio_priv;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*priv_).lock, &mut flags);
    xlp_gpio_set_reg((*priv_).gpio_intr_en, (*d).hwirq as u32, 0);
    xlp_gpio_set_reg((*priv_).gpio_intr_stat, (*d).hwirq as u32, 1);
    __clear_bit((*d).hwirq as usize, (*priv_).gpio_enabled_mask.as_mut_ptr());
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
}

unsafe fn xlp_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let priv_ = gpiochip_get_data(gc) as *mut xlp_gpio_priv;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*priv_).lock, &mut flags);
    xlp_gpio_set_reg((*priv_).gpio_intr_en, (*d).hwirq as u32, 1);
    __set_bit((*d).hwirq as usize, (*priv_).gpio_enabled_mask.as_mut_ptr());
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
}

unsafe fn xlp_gpio_set_irq_type(d: *mut irq_data, ty: c_uint) -> c_int {
    let gc = irq_data_get_irq_chip_data(d);
    let priv_ = gpiochip_get_data(gc) as *mut xlp_gpio_priv;
    let (irq_type, pol) = match ty {
        IRQ_TYPE_EDGE_RISING => (XLP_GPIO_IRQ_TYPE_EDGE, XLP_GPIO_IRQ_POL_HIGH),
        IRQ_TYPE_EDGE_FALLING => (XLP_GPIO_IRQ_TYPE_EDGE, XLP_GPIO_IRQ_POL_LOW),
        IRQ_TYPE_LEVEL_HIGH => (XLP_GPIO_IRQ_TYPE_LVL, XLP_GPIO_IRQ_POL_HIGH),
        IRQ_TYPE_LEVEL_LOW => (XLP_GPIO_IRQ_TYPE_LVL, XLP_GPIO_IRQ_POL_LOW),
        _ => return -EINVAL,
    };
    xlp_gpio_set_reg((*priv_).gpio_intr_type, (*d).hwirq as u32, irq_type as i32);
    xlp_gpio_set_reg((*priv_).gpio_intr_pol, (*d).hwirq as u32, pol as i32);
    0
}

static mut xlp_gpio_irq_chip: irq_chip = irq_chip {
    name: b"XLP-GPIO\0".as_ptr() as *const c_char,
    irq_mask_ack: Some(xlp_gpio_irq_mask_ack), irq_enable: Some(xlp_gpio_irq_enable),
    irq_disable: Some(xlp_gpio_irq_disable), irq_set_type: Some(xlp_gpio_set_irq_type),
    irq_unmask: Some(xlp_gpio_irq_unmask), ..irq_chip::ZERO
};

unsafe fn xlp_gpio_generic_handler(desc: *mut irq_desc) {
    let priv_ = irq_desc_get_handler_data(desc) as *mut xlp_gpio_priv;
    let irqchip = irq_desc_get_chip(desc);
    let mut regoff: i32 = -1;
    let mut gpio_stat: u32 = 0;
    chained_irq_enter(irqchip, desc);
    for gpio in 0..XLP_MAX_NR_GPIO {
        if test_bit(gpio, (*priv_).gpio_enabled_mask.as_ptr()) != 0 {
            if regoff != (gpio / XLP_GPIO_REGSZ) as i32 {
                regoff = (gpio / XLP_GPIO_REGSZ) as i32;
                gpio_stat = readl(((*priv_).gpio_intr_stat as *mut u8).add((regoff * 4) as usize) as *mut u32);
            }
            if gpio_stat & (1u32 << (gpio % XLP_GPIO_REGSZ)) != 0 { generic_handle_domain_irq((*priv_).chip.irq.domain, gpio as u32); }
        }
    }
    chained_irq_exit(irqchip, desc);
}

unsafe fn xlp_gpio_dir_output(gc: *mut gpio_chip, gpio: u32, _state: i32) -> c_int {
    xlp_gpio_set_reg((*(gpiochip_get_data(gc) as *mut xlp_gpio_priv)).gpio_out_en, gpio, 1); 0
}
unsafe fn xlp_gpio_dir_input(gc: *mut gpio_chip, gpio: u32) -> c_int {
    xlp_gpio_set_reg((*(gpiochip_get_data(gc) as *mut xlp_gpio_priv)).gpio_out_en, gpio, 0); 0
}
unsafe fn xlp_gpio_get(gc: *mut gpio_chip, gpio: u32) -> c_int { xlp_gpio_get_reg((*(gpiochip_get_data(gc) as *mut xlp_gpio_priv)).gpio_paddrv, gpio) }
unsafe fn xlp_gpio_set(gc: *mut gpio_chip, gpio: u32, state: i32) -> c_int {
    xlp_gpio_set_reg((*(gpiochip_get_data(gc) as *mut xlp_gpio_priv)).gpio_paddrv, gpio, state); 0
}

unsafe fn xlp_gpio_probe(pdev: *mut platform_device) -> c_int {
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<xlp_gpio_priv>(), GFP_KERNEL) as *mut xlp_gpio_priv;
    if priv_.is_null() { return -ENOMEM; }
    let gpio_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(gpio_base) { return PTR_ERR(gpio_base); }
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    (*priv_).gpio_out_en = (gpio_base as *mut u8).add(GPIO_9XX_OUTPUT_EN) as *mut _;
    (*priv_).gpio_paddrv = (gpio_base as *mut u8).add(GPIO_9XX_PADDRV) as *mut _;
    (*priv_).gpio_intr_stat = (gpio_base as *mut u8).add(GPIO_9XX_INT_STAT) as *mut _;
    (*priv_).gpio_intr_type = (gpio_base as *mut u8).add(GPIO_9XX_INT_TYPE) as *mut _;
    (*priv_).gpio_intr_pol = (gpio_base as *mut u8).add(GPIO_9XX_INT_POL) as *mut _;
    (*priv_).gpio_intr_en = (gpio_base as *mut u8).add(GPIO_9XX_INT_EN00) as *mut _;
    bitmap_zero((*priv_).gpio_enabled_mask.as_mut_ptr(), XLP_MAX_NR_GPIO);
    let gc = &mut (*priv_).chip;
    gc.owner = THIS_MODULE; gc.label = dev_name(&mut (*pdev).dev); gc.base = 0;
    gc.parent = &mut (*pdev).dev; gc.ngpio = 70;
    gc.direction_output = Some(xlp_gpio_dir_output); gc.direction_input = Some(xlp_gpio_dir_input);
    gc.set = Some(xlp_gpio_set); gc.get = Some(xlp_gpio_get);
    spin_lock_init(&mut (*priv_).lock);
    let girq = &mut gc.irq;
    gpio_irq_chip_set_chip(girq, &mut xlp_gpio_irq_chip);
    girq.parent_handler = Some(xlp_gpio_generic_handler); girq.num_parents = 1;
    girq.parents = devm_kcalloc(&mut (*pdev).dev, 1, core::mem::size_of::<c_uint>(), GFP_KERNEL);
    if girq.parents.is_null() { return -ENOMEM; }
    *girq.parents = irq as c_uint; girq.first = 0; girq.default_type = IRQ_TYPE_NONE;
    girq.handler = Some(handle_level_irq);
    let err = gpiochip_add_data(gc, priv_);
    if err < 0 { return err; }
    dev_info(&mut (*pdev).dev, b"registered %d GPIOs\n\0".as_ptr(), gc.ngpio);
    0
}

#[cfg(CONFIG_ACPI)]
static xlp_gpio_acpi_match: [acpi_device_id; 3] = [acpi_device_id { name: *b"BRCM9006\0" }, acpi_device_id { name: *b"CAV9006\0" }, acpi_device_id::ZERO];

static mut xlp_gpio_driver: platform_driver = platform_driver { driver: driver { name: b"xlp-gpio\0".as_ptr() as *const c_char, ..driver::ZERO }, probe: Some(xlp_gpio_probe), ..platform_driver::ZERO };

// MODULE_DEVICE_TABLE(acpi, xlp_gpio_acpi_match)
// module_platform_driver(xlp_gpio_driver)
// MODULE_AUTHOR("Kamlakant Patel <kamlakant.patel@broadcom.com>")
// MODULE_AUTHOR("Ganesan Ramalingam <ganesanr@broadcom.com>")
// MODULE_DESCRIPTION("Netlogic XLP GPIO Driver")
// MODULE_LICENSE("GPL v2")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
