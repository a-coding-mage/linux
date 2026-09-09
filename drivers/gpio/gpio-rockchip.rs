// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 MundoReader S.L.
 * Author: Heiko Stuebner <heiko@sntech.de>
 *
 * Copyright (c) 2021 Rockchip Electronics Co. Ltd.
 */

// Linux headers and pinctrl dependencies are supplied by the surrounding kernel bindings.

const GPIO_TYPE_V1: u32 = 0;
const GPIO_TYPE_V2: u32 = 0x01000C2B;
const GPIO_TYPE_V2_1: u32 = 0x0101157C;
const GPIO_TYPE_V2_2: u32 = 0x010219C8;

static GPIO_REGS_V1: rockchip_gpio_regs = rockchip_gpio_regs {
    port_dr: 0x00, port_ddr: 0x04, int_en: 0x30, int_mask: 0x34,
    int_type: 0x38, int_polarity: 0x3c, int_status: 0x40,
    int_rawstatus: 0x44, debounce: 0x48, port_eoi: 0x4c, ext_port: 0x50,
    ..rockchip_gpio_regs::default()
};

static GPIO_REGS_V2: rockchip_gpio_regs = rockchip_gpio_regs {
    port_dr: 0x00, port_ddr: 0x08, int_en: 0x10, int_mask: 0x18,
    int_type: 0x20, int_polarity: 0x28, int_bothedge: 0x30,
    int_status: 0x50, int_rawstatus: 0x58, debounce: 0x38,
    dbclk_div_en: 0x40, dbclk_div_con: 0x48, port_eoi: 0x60,
    ext_port: 0x70, version_id: 0x78, ..rockchip_gpio_regs::default()
};

unsafe fn gpio_writel_v2(val: u32, reg: *mut u8) {
    writel((val & 0xffff) | 0xffff0000, reg);
    writel((val >> 16) | 0xffff0000, reg.add(4));
}
unsafe fn gpio_readl_v2(reg: *mut u8) -> u32 { (readl(reg.add(4)) << 16) | readl(reg) }

unsafe fn rockchip_gpio_writel(bank: *mut rockchip_pin_bank, value: u32, offset: u32) {
    let reg = (*bank).reg_base.add(offset as usize);
    if (*bank).gpio_type == GPIO_TYPE_V2 { gpio_writel_v2(value, reg); } else { writel(value, reg); }
}
unsafe fn rockchip_gpio_readl(bank: *mut rockchip_pin_bank, offset: u32) -> u32 {
    let reg = (*bank).reg_base.add(offset as usize);
    if (*bank).gpio_type == GPIO_TYPE_V2 { gpio_readl_v2(reg) } else { readl(reg) }
}
unsafe fn rockchip_gpio_writel_bit(bank: *mut rockchip_pin_bank, bit: u32, value: u32, offset: u32) {
    let reg = (*bank).reg_base.add(offset as usize);
    let data;
    if (*bank).gpio_type == GPIO_TYPE_V2 {
        data = if value != 0 { (1 << (bit % 16)) | (1 << (bit % 16 + 16)) } else { 1 << (bit % 16 + 16) };
        writel(data, if bit >= 16 { reg.add(4) } else { reg });
    } else {
        data = readl(reg) & !(1 << bit) | if value != 0 { 1 << bit } else { 0 };
        writel(data, reg);
    }
}
unsafe fn rockchip_gpio_readl_bit(bank: *mut rockchip_pin_bank, bit: u32, offset: u32) -> u32 {
    let reg = (*bank).reg_base.add(offset as usize);
    let data = if (*bank).gpio_type == GPIO_TYPE_V2 { readl(if bit >= 16 { reg.add(4) } else { reg }) >> (bit % 16) } else { readl(reg) >> bit };
    data & 1
}

unsafe fn rockchip_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let bank = gpiochip_get_data(chip);
    if rockchip_gpio_readl_bit(bank, offset, (*(*bank).gpio_regs).port_ddr) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}
unsafe fn rockchip_gpio_set_direction(chip: *mut gpio_chip, offset: u32, input: bool) -> i32 {
    let bank = gpiochip_get_data(chip); let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*bank).slock, &mut flags);
    rockchip_gpio_writel_bit(bank, offset, if input { 0 } else { 1 }, (*(*bank).gpio_regs).port_ddr);
    raw_spin_unlock_irqrestore(&mut (*bank).slock, flags); 0
}
unsafe fn rockchip_gpio_set(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let bank = gpiochip_get_data(gc); let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*bank).slock, &mut flags);
    rockchip_gpio_writel_bit(bank, offset, value as u32, (*(*bank).gpio_regs).port_dr);
    raw_spin_unlock_irqrestore(&mut (*bank).slock, flags); 0
}
unsafe fn rockchip_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let bank = gpiochip_get_data(gc); (readl((*bank).reg_base.add((*(*bank).gpio_regs).ext_port as usize)) >> offset & 1) as i32
}

unsafe fn rockchip_gpio_set_debounce(gc: *mut gpio_chip, offset: u32, debounce: u32) -> i32 {
    let bank = gpiochip_get_data(gc); let reg = (*bank).gpio_regs; let mut flags = 0; let mut div_reg = 0; let mut div_support = false;
    if (*bank).gpio_type == GPIO_TYPE_V2 && !IS_ERR((*bank).db_clk) {
        div_support = true; let freq = clk_get_rate((*bank).db_clk); let max = ((1u64 << 24) * 2 * 1_000_000 / freq as u64) as u32;
        if debounce > max { return -EINVAL; }
        div_reg = ((debounce as u64 * freq as u64 + 1_000_000) / 2_000_000 - 1) as u32;
    }
    raw_spin_lock_irqsave(&mut (*bank).slock, &mut flags);
    if debounce != 0 {
        if div_support { let cur = readl((*bank).reg_base.add((*reg).dbclk_div_con as usize)); if cur < div_reg { writel(div_reg, (*bank).reg_base.add((*reg).dbclk_div_con as usize)); } rockchip_gpio_writel_bit(bank, offset, 1, (*reg).dbclk_div_en); }
        rockchip_gpio_writel_bit(bank, offset, 1, (*reg).debounce);
    } else { if div_support { rockchip_gpio_writel_bit(bank, offset, 0, (*reg).dbclk_div_en); } rockchip_gpio_writel_bit(bank, offset, 0, (*reg).debounce); }
    raw_spin_unlock_irqrestore(&mut (*bank).slock, flags);
    if div_support { if debounce != 0 { clk_prepare_enable((*bank).db_clk); } else { clk_disable_unprepare((*bank).db_clk); } } 0
}
unsafe fn rockchip_gpio_direction_input(gc: *mut gpio_chip, offset: u32) -> i32 { rockchip_gpio_set_direction(gc, offset, true) }
unsafe fn rockchip_gpio_direction_output(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 { rockchip_gpio_set(gc, offset, value); rockchip_gpio_set_direction(gc, offset, false) }

/* gpiolib set_config callback; GPIO muxing is handled by pinctrl. */
unsafe fn rockchip_gpio_set_config(gc: *mut gpio_chip, offset: u32, config: u64) -> i32 {
    match pinconf_to_config_param(config) { PIN_CONFIG_INPUT_DEBOUNCE => { rockchip_gpio_set_debounce(gc, offset, 1); -ENOTSUPP }, _ => gpiochip_generic_config(gc, offset, config) }
}
unsafe fn rockchip_gpio_to_irq(gc: *mut gpio_chip, offset: u32) -> i32 { let bank = gpiochip_get_data(gc); if (*bank).domain.is_null() { return -ENXIO; } let virq = irq_create_mapping((*bank).domain, offset); if virq != 0 { virq as i32 } else { -ENXIO } }

static ROCKCHIP_GPIOLIB_CHIP: gpio_chip = gpio_chip { request: Some(gpiochip_generic_request), free: Some(gpiochip_generic_free), set: Some(rockchip_gpio_set), get: Some(rockchip_gpio_get), get_direction: Some(rockchip_gpio_get_direction), direction_input: Some(rockchip_gpio_direction_input), direction_output: Some(rockchip_gpio_direction_output), set_config: Some(rockchip_gpio_set_config), to_irq: Some(rockchip_gpio_to_irq), owner: THIS_MODULE, ..gpio_chip::default() };

unsafe fn rockchip_irq_demux(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc); let bank = irq_desc_get_handler_data(desc); let mut pending = readl_relaxed((*bank).reg_base.add((*(*bank).gpio_regs).int_status as usize));
    chained_irq_enter(chip, desc);
    for irq in 0..32 { if pending & (1 << irq) != 0 { if (*bank).toggle_edge_mode & (1 << irq) != 0 { let mut data = readl_relaxed((*bank).reg_base.add((*(*bank).gpio_regs).ext_port as usize)); loop { let mut flags = 0; raw_spin_lock_irqsave(&mut (*bank).slock, &mut flags); let mut polarity = readl_relaxed((*bank).reg_base.add((*(*bank).gpio_regs).int_polarity as usize)); if data & (1 << irq) != 0 { polarity &= !(1 << irq); } else { polarity |= 1 << irq; } writel(polarity, (*bank).reg_base.add((*(*bank).gpio_regs).int_polarity as usize)); raw_spin_unlock_irqrestore(&mut (*bank).slock, flags); let old = data; data = readl_relaxed((*bank).reg_base.add((*(*bank).gpio_regs).ext_port as usize)); if (data & (1 << irq)) == (old & (1 << irq)) { break; } } } generic_handle_domain_irq((*bank).domain, irq); } }
    chained_irq_exit(chip, desc);
}
unsafe fn rockchip_irq_set_type(d: *mut irq_data, typ: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d); let bank = (*gc).private; let mask = 1u32 << (*d).hwirq; let mut flags = 0; raw_spin_lock_irqsave(&mut (*bank).slock, &mut flags); rockchip_gpio_writel_bit(bank, (*d).hwirq, 0, (*(*bank).gpio_regs).port_ddr); raw_spin_unlock_irqrestore(&mut (*bank).slock, flags); if typ & IRQ_TYPE_EDGE_BOTH != 0 { irq_set_handler_locked(d, handle_edge_irq); } else { irq_set_handler_locked(d, handle_level_irq); } raw_spin_lock_irqsave(&mut (*bank).slock, &mut flags); let mut level = rockchip_gpio_readl(bank, (*(*bank).gpio_regs).int_type); let mut polarity = rockchip_gpio_readl(bank, (*(*bank).gpio_regs).int_polarity); let mut ret = 0;
    if typ == IRQ_TYPE_EDGE_BOTH { if (*bank).gpio_type == GPIO_TYPE_V2 { rockchip_gpio_writel_bit(bank, (*d).hwirq, 1, (*(*bank).gpio_regs).int_bothedge); } else { (*bank).toggle_edge_mode |= mask; level &= !mask; let data = readl((*bank).reg_base.add((*(*bank).gpio_regs).ext_port as usize)); if data & mask != 0 { polarity &= !mask; } else { polarity |= mask; } } } else { if (*bank).gpio_type == GPIO_TYPE_V2 { rockchip_gpio_writel_bit(bank, (*d).hwirq, 0, (*(*bank).gpio_regs).int_bothedge); } else { (*bank).toggle_edge_mode &= !mask; } match typ { IRQ_TYPE_EDGE_RISING => { level |= mask; polarity |= mask; }, IRQ_TYPE_EDGE_FALLING => { level |= mask; polarity &= !mask; }, IRQ_TYPE_LEVEL_HIGH => { level &= !mask; polarity |= mask; }, IRQ_TYPE_LEVEL_LOW => { level &= !mask; polarity &= !mask; }, _ => ret = -EINVAL } } if ret == 0 { rockchip_gpio_writel(bank, level, (*(*bank).gpio_regs).int_type); rockchip_gpio_writel(bank, polarity, (*(*bank).gpio_regs).int_polarity); } raw_spin_unlock_irqrestore(&mut (*bank).slock, flags); ret
}
unsafe fn rockchip_irq_reqres(d: *mut irq_data) -> i32 { let gc = irq_data_get_irq_chip_data(d); gpiochip_reqres_irq(&mut (*(*gc).private).gpio_chip, (*d).hwirq) }
unsafe fn rockchip_irq_relres(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); gpiochip_relres_irq(&mut (*(*gc).private).gpio_chip, (*d).hwirq); }
unsafe fn rockchip_irq_suspend(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); let bank = (*gc).private; (*bank).saved_masks = irq_reg_readl(gc, (*(*bank).gpio_regs).int_mask); irq_reg_writel(gc, !(*gc).wake_active, (*(*bank).gpio_regs).int_mask); }
unsafe fn rockchip_irq_resume(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); let bank = (*gc).private; irq_reg_writel(gc, (*bank).saved_masks, (*(*bank).gpio_regs).int_mask); }
unsafe fn rockchip_irq_enable(d: *mut irq_data) { irq_gc_mask_clr_bit(d); }
unsafe fn rockchip_irq_disable(d: *mut irq_data) { irq_gc_mask_set_bit(d); }

unsafe fn rockchip_gpio_find_bank(pctldev: *mut pinctrl_dev, id: i32) -> *mut rockchip_pin_bank { let info = pinctrl_dev_get_drvdata(pctldev); let mut bank = (*(*info).ctrl).pin_banks; for _ in 0..(*(*info).ctrl).nr_banks { if (*bank).bank_num == id { return bank; } bank = bank.add(1); } core::ptr::null_mut() }

unsafe fn rockchip_gpio_probe(pdev: *mut platform_device) -> i32 { let dev = &mut (*pdev).dev; let np = (*dev).of_node; let pctlnp = of_get_parent(np); if np.is_null() || pctlnp.is_null() { return -ENODEV; } let pctldev = of_pinctrl_get(pctlnp); of_node_put(pctlnp); if pctldev.is_null() { return -EPROBE_DEFER; } let mut id = of_alias_get_id(np, "gpio"); static mut GPIO: i32 = 0; if id < 0 { id = GPIO; GPIO += 1; } let bank = rockchip_gpio_find_bank(pctldev, id); if bank.is_null() { return -EINVAL; } (*bank).dev = dev; (*bank).of_node = np; raw_spin_lock_init(&mut (*bank).slock); let ret = rockchip_get_bank_data(bank); if ret != 0 { return ret; } mutex_lock(&mut (*bank).deferred_lock); let ret = rockchip_gpiolib_register(bank); mutex_unlock(&mut (*bank).deferred_lock); if ret != 0 { return ret; } platform_set_drvdata(pdev, bank); 0 }
unsafe fn rockchip_gpio_remove(pdev: *mut platform_device) { let bank = platform_get_drvdata(pdev); irq_set_chained_handler_and_data((*bank).irq, None, core::ptr::null_mut()); if !(*bank).domain.is_null() { irq_domain_remove_generic_chips((*bank).domain); irq_domain_remove((*bank).domain); } gpiochip_remove(&mut (*bank).gpio_chip); }

unsafe fn rockchip_get_bank_data(bank: *mut rockchip_pin_bank) -> i32 { let pdev = to_platform_device((*bank).dev); (*bank).reg_base = devm_platform_ioremap_resource(pdev, 0); if IS_ERR((*bank).reg_base) { return PTR_ERR((*bank).reg_base); } let irq = platform_get_irq(pdev, 0); if irq < 0 { return irq; } (*bank).irq = irq; (*bank).clk = devm_clk_get_enabled((*bank).dev, core::ptr::null()); if IS_ERR((*bank).clk) { return PTR_ERR((*bank).clk); } let id = readl((*bank).reg_base.add(GPIO_REGS_V2.version_id as usize)); match id { GPIO_TYPE_V2 | GPIO_TYPE_V2_1 | GPIO_TYPE_V2_2 => { (*bank).gpio_regs = &GPIO_REGS_V2; (*bank).gpio_type = GPIO_TYPE_V2; }, GPIO_TYPE_V1 => { (*bank).gpio_regs = &GPIO_REGS_V1; (*bank).gpio_type = GPIO_TYPE_V1; }, _ => return -ENODEV } 0 }

unsafe fn rockchip_gpio_init() -> i32 { platform_driver_register(&mut rockchip_gpio_driver) }
unsafe fn rockchip_gpio_exit() { platform_driver_unregister(&mut rockchip_gpio_driver); }

static mut ROCKCHIP_GPIO_DRIVER: platform_driver = platform_driver { probe: Some(rockchip_gpio_probe), remove: Some(rockchip_gpio_remove), driver: driver { name: "rockchip-gpio", of_match_table: rockchip_gpio_match, ..driver::default() }, ..platform_driver::default() };

// postcore_initcall(rockchip_gpio_init); module_exit(rockchip_gpio_exit);
// MODULE_DESCRIPTION("Rockchip gpio driver"); MODULE_ALIAS("platform:rockchip-gpio");
// MODULE_LICENSE("GPL v2"); MODULE_DEVICE_TABLE(of, rockchip_gpio_match);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
