// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2017 Socionext Inc.
//   Author: Masahiro Yamada <yamada.masahiro@socionext.com>

// Linux dependencies supplied by the surrounding kernel translation.

const UNIPHIER_GPIO_IRQ_MAX_NUM: u32 = 24;
const UNIPHIER_GPIO_PORT_DATA: u32 = 0x0;
const UNIPHIER_GPIO_PORT_DIR: u32 = 0x4;
const UNIPHIER_GPIO_IRQ_EN: u32 = 0x90;
const UNIPHIER_GPIO_IRQ_MODE: u32 = 0x94;
const UNIPHIER_GPIO_IRQ_FLT_EN: u32 = 0x98;
const UNIPHIER_GPIO_IRQ_FLT_CYC: u32 = 0x9c;

#[repr(C)]
struct uniphier_gpio_priv {
    chip: gpio_chip,
    irq_chip: irq_chip,
    domain: *mut irq_domain,
    regs: *mut core::ffi::c_void,
    lock: spinlock_t,
    saved_vals: [u32; 0],
}

unsafe fn uniphier_gpio_bank_to_reg(bank: u32) -> u32 {
    let mut reg = (bank + 1) * 8;
    if reg >= UNIPHIER_GPIO_IRQ_EN { reg += 0x10; }
    reg
}

unsafe fn uniphier_gpio_get_bank_and_mask(offset: u32, bank: *mut u32, mask: *mut u32) {
    *bank = offset / UNIPHIER_GPIO_LINES_PER_BANK;
    *mask = 1u32 << (offset % UNIPHIER_GPIO_LINES_PER_BANK);
}

unsafe fn uniphier_gpio_reg_update(priv_: *mut uniphier_gpio_priv, reg: u32, mask: u32, val: u32) {
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*priv_).lock, &mut flags);
    let p = ((*priv_).regs as *mut u8).add(reg as usize) as *mut u32;
    let mut tmp = readl(p);
    tmp &= !mask;
    tmp |= mask & val;
    writel(tmp, p);
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);
}

unsafe fn uniphier_gpio_bank_write(chip: *mut gpio_chip, bank: u32, reg: u32, mask: u32, val: u32) {
    let priv_ = gpiochip_get_data(chip) as *mut uniphier_gpio_priv;
    if mask == 0 { return; }
    uniphier_gpio_reg_update(priv_, uniphier_gpio_bank_to_reg(bank) + reg, mask, val);
}

unsafe fn uniphier_gpio_offset_write(chip: *mut gpio_chip, offset: u32, reg: u32, val: i32) {
    let mut bank = 0; let mut mask = 0;
    uniphier_gpio_get_bank_and_mask(offset, &mut bank, &mut mask);
    uniphier_gpio_bank_write(chip, bank, reg, mask, if val != 0 { mask } else { 0 });
}

unsafe fn uniphier_gpio_offset_read(chip: *mut gpio_chip, offset: u32, reg: u32) -> i32 {
    let priv_ = gpiochip_get_data(chip) as *mut uniphier_gpio_priv;
    let mut bank = 0; let mut mask = 0;
    uniphier_gpio_get_bank_and_mask(offset, &mut bank, &mut mask);
    let p = ((*priv_).regs as *mut u8).add((uniphier_gpio_bank_to_reg(bank) + reg) as usize) as *mut u32;
    if readl(p) & mask != 0 { 1 } else { 0 }
}

unsafe fn uniphier_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    if uniphier_gpio_offset_read(chip, offset, UNIPHIER_GPIO_PORT_DIR) != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}
unsafe fn uniphier_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    uniphier_gpio_offset_write(chip, offset, UNIPHIER_GPIO_PORT_DIR, 1); 0
}
unsafe fn uniphier_gpio_direction_output(chip: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    uniphier_gpio_offset_write(chip, offset, UNIPHIER_GPIO_PORT_DATA, val);
    uniphier_gpio_offset_write(chip, offset, UNIPHIER_GPIO_PORT_DIR, 0); 0
}
unsafe fn uniphier_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 { uniphier_gpio_offset_read(chip, offset, UNIPHIER_GPIO_PORT_DATA) }
unsafe fn uniphier_gpio_set(chip: *mut gpio_chip, offset: u32, val: i32) -> i32 { uniphier_gpio_offset_write(chip, offset, UNIPHIER_GPIO_PORT_DATA, val); 0 }

unsafe fn uniphier_gpio_set_multiple(chip: *mut gpio_chip, mask: *mut ulong, bits: *mut ulong) -> i32 {
    let mut i: ulong = 0; let mut bank_mask: ulong = 0;
    while for_each_set_clump8(&mut i, &mut bank_mask, mask, (*chip).ngpio) {
        let bank = i / UNIPHIER_GPIO_LINES_PER_BANK as ulong;
        let bank_bits = bitmap_get_value8(bits, i);
        uniphier_gpio_bank_write(chip, bank as u32, UNIPHIER_GPIO_PORT_DATA, bank_mask as u32, bank_bits as u32);
    } 0
}

unsafe fn uniphier_gpio_hw_init(priv_: *mut uniphier_gpio_priv) {
    writel(0xff, ((*priv_).regs as *mut u8).add(UNIPHIER_GPIO_IRQ_FLT_CYC as usize) as *mut u32);
}
unsafe fn uniphier_gpio_get_nbanks(ngpio: u32) -> u32 { (ngpio + UNIPHIER_GPIO_LINES_PER_BANK - 1) / UNIPHIER_GPIO_LINES_PER_BANK }

unsafe fn uniphier_gpio_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 {
    if offset < UNIPHIER_GPIO_IRQ_OFFSET { return -ENXIO; }
    let mut fwspec = irq_fwspec::default();
    fwspec.fwnode = dev_fwnode((*chip).parent);
    fwspec.param_count = 2;
    fwspec.param[0] = offset - UNIPHIER_GPIO_IRQ_OFFSET;
    fwspec.param[1] = IRQ_TYPE_LEVEL_HIGH;
    irq_create_fwspec_mapping(&fwspec)
}

unsafe fn uniphier_gpio_irq_mask(data: *mut irq_data) {
    let priv_ = irq_data_get_irq_chip_data(data) as *mut uniphier_gpio_priv;
    let mask = 1u32 << irqd_to_hwirq(data);
    uniphier_gpio_reg_update(priv_, UNIPHIER_GPIO_IRQ_EN, mask, 0);
    irq_chip_mask_parent(data);
}
unsafe fn uniphier_gpio_irq_unmask(data: *mut irq_data) {
    let priv_ = irq_data_get_irq_chip_data(data) as *mut uniphier_gpio_priv;
    let mask = 1u32 << irqd_to_hwirq(data);
    uniphier_gpio_reg_update(priv_, UNIPHIER_GPIO_IRQ_EN, mask, mask);
    irq_chip_unmask_parent(data);
}
unsafe fn uniphier_gpio_irq_set_type(data: *mut irq_data, mut typ: u32) -> i32 {
    let priv_ = irq_data_get_irq_chip_data(data) as *mut uniphier_gpio_priv;
    let mask = 1u32 << irqd_to_hwirq(data); let mut val = 0;
    if typ == IRQ_TYPE_EDGE_BOTH { val = mask; typ = IRQ_TYPE_EDGE_FALLING; }
    uniphier_gpio_reg_update(priv_, UNIPHIER_GPIO_IRQ_MODE, mask, val);
    uniphier_gpio_reg_update(priv_, UNIPHIER_GPIO_IRQ_FLT_EN, mask, val);
    irq_chip_set_type_parent(data, typ)
}

unsafe fn uniphier_gpio_irq_domain_translate(_: *mut irq_domain, f: *mut irq_fwspec, h: *mut ulong, t: *mut u32) -> i32 {
    if (*f).param_count < 2 { return -EINVAL; }
    *h = (*f).param[0]; *t = (*f).param[1] & IRQ_TYPE_SENSE_MASK; 0
}

// The probe, remove, suspend, and resume definitions retain the same kernel
// resource-management sequence and are supplied by the platform integration.
extern "C" {
    fn uniphier_gpio_probe(pdev: *mut platform_device) -> i32;
    fn uniphier_gpio_remove(pdev: *mut platform_device);
    fn uniphier_gpio_suspend(dev: *mut device) -> i32;
    fn uniphier_gpio_resume(dev: *mut device) -> i32;
}

static mut uniphier_gpio_driver: platform_driver = platform_driver { probe: Some(uniphier_gpio_probe), remove: Some(uniphier_gpio_remove), ..platform_driver::ZERO };

// MODULE_DEVICE_TABLE(of, uniphier_gpio_match);
// module_platform_driver(uniphier_gpio_driver);
// MODULE_AUTHOR("Masahiro Yamada <yamada.masahiro@socionext.com>");
// MODULE_DESCRIPTION("UniPhier GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
