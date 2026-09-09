// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 Spreadtrum Communications Inc.
 * Copyright (C) 2018 Linaro Ltd.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const SPRD_PMIC_EIC_DATA: u16 = 0x0;
const SPRD_PMIC_EIC_DMSK: u16 = 0x4;
const SPRD_PMIC_EIC_IEV: u16 = 0x14;
const SPRD_PMIC_EIC_IE: u16 = 0x18;
const SPRD_PMIC_EIC_RIS: u16 = 0x1c;
const SPRD_PMIC_EIC_MIS: u16 = 0x20;
const SPRD_PMIC_EIC_IC: u16 = 0x24;
const SPRD_PMIC_EIC_TRIG: u16 = 0x28;
const SPRD_PMIC_EIC_CTRL0: u32 = 0x40;

const SPRD_PMIC_EIC_PER_BANK_NR: u32 = 16;
const SPRD_PMIC_EIC_NR: u32 = SPRD_PMIC_EIC_PER_BANK_NR;
const SPRD_PMIC_EIC_DATA_MASK: u32 = 0xffff;
const SPRD_PMIC_EIC_DBNC_MASK: u32 = 0xfff;

#[repr(usize)]
enum CacheReg {
    RegIev,
    RegIe,
    RegTrig,
    CacheNrRegs,
}

#[repr(C)]
struct SprdPmicEic {
    chip: gpio_chip,
    map: *mut regmap,
    offset: u32,
    reg: [u8; CacheReg::CacheNrRegs as usize],
    buslock: mutex,
    irq: i32,
}

unsafe fn sprd_pmic_eic_update(chip: *mut gpio_chip, offset: u32, reg: u16, val: u32) {
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    let shift = offset & (SPRD_PMIC_EIC_PER_BANK_NR - 1);
    regmap_update_bits((*pmic_eic).map, (*pmic_eic).offset + reg as u32,
                       1u32.wrapping_shl(shift), val.wrapping_shl(shift));
}

unsafe fn sprd_pmic_eic_read(chip: *mut gpio_chip, offset: u32, reg: u16) -> i32 {
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    let mut value: u32 = 0;
    let ret = regmap_read((*pmic_eic).map, (*pmic_eic).offset + reg as u32, &mut value);
    if ret != 0 { return ret; }
    if value & (1u32.wrapping_shl(offset & (SPRD_PMIC_EIC_PER_BANK_NR - 1))) != 0 { 1 } else { 0 }
}

unsafe fn sprd_pmic_eic_request(chip: *mut gpio_chip, offset: u32) -> i32 {
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_DMSK, 1); 0
}

unsafe fn sprd_pmic_eic_free(chip: *mut gpio_chip, offset: u32) {
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_DMSK, 0);
}

unsafe fn sprd_pmic_eic_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    sprd_pmic_eic_read(chip, offset, SPRD_PMIC_EIC_DATA)
}

unsafe fn sprd_pmic_eic_direction_input(_chip: *mut gpio_chip, _offset: u32) -> i32 { 0 }

unsafe fn sprd_pmic_eic_set_debounce(chip: *mut gpio_chip, offset: u32, debounce: u32) -> i32 {
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    let reg = SPRD_PMIC_EIC_CTRL0 + (offset & (SPRD_PMIC_EIC_PER_BANK_NR - 1)) * 0x4;
    let mut value: u32 = 0;
    let ret = regmap_read((*pmic_eic).map, (*pmic_eic).offset + reg, &mut value);
    if ret != 0 { return ret; }
    value &= !SPRD_PMIC_EIC_DBNC_MASK;
    value |= (debounce / 1000) & SPRD_PMIC_EIC_DBNC_MASK;
    regmap_write((*pmic_eic).map, (*pmic_eic).offset + reg, value)
}

unsafe fn sprd_pmic_eic_set_config(chip: *mut gpio_chip, offset: u32, config: c_ulong) -> i32 {
    let param = pinconf_to_config_param(config);
    let arg = pinconf_to_config_argument(config) as u32;
    if param == PIN_CONFIG_INPUT_DEBOUNCE { return sprd_pmic_eic_set_debounce(chip, offset, arg); }
    -ENOTSUPP
}

unsafe fn sprd_pmic_eic_irq_mask(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data) as *mut gpio_chip;
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    let offset = irqd_to_hwirq(data) as u32;
    (*pmic_eic).reg[CacheReg::RegIe as usize] &= !(1u8.wrapping_shl(offset));
    (*pmic_eic).reg[CacheReg::RegTrig as usize] &= !(1u8.wrapping_shl(offset));
    gpiochip_disable_irq(chip, offset);
}

unsafe fn sprd_pmic_eic_irq_unmask(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data) as *mut gpio_chip;
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    let offset = irqd_to_hwirq(data) as u32;
    gpiochip_enable_irq(chip, offset);
    (*pmic_eic).reg[CacheReg::RegIe as usize] |= 1u8.wrapping_shl(offset);
    (*pmic_eic).reg[CacheReg::RegTrig as usize] |= 1u8.wrapping_shl(offset);
}

unsafe fn sprd_pmic_eic_irq_set_type(data: *mut irq_data, flow_type: u32) -> i32 {
    let chip = irq_data_get_irq_chip_data(data) as *mut gpio_chip;
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    let offset = irqd_to_hwirq(data) as u32;
    match flow_type {
        IRQ_TYPE_LEVEL_HIGH => (*pmic_eic).reg[CacheReg::RegIev as usize] |= 1u8.wrapping_shl(offset),
        IRQ_TYPE_LEVEL_LOW => (*pmic_eic).reg[CacheReg::RegIev as usize] &= !(1u8.wrapping_shl(offset)),
        IRQ_TYPE_EDGE_RISING | IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_EDGE_BOTH => (),
        _ => return -ENOTSUPP,
    }
    0
}

unsafe fn sprd_pmic_eic_bus_lock(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data) as *mut gpio_chip;
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    mutex_lock(&mut (*pmic_eic).buslock);
}

unsafe fn sprd_pmic_eic_bus_sync_unlock(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data) as *mut gpio_chip;
    let pmic_eic = gpiochip_get_data(chip) as *mut SprdPmicEic;
    let trigger = irqd_get_trigger_type(data);
    let offset = irqd_to_hwirq(data) as u32;
    if trigger & IRQ_TYPE_EDGE_BOTH != 0 {
        if sprd_pmic_eic_get(chip, offset) != 0 { sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 0); }
        else { sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 1); }
    } else {
        sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV,
            ((*pmic_eic).reg[CacheReg::RegIev as usize] & (1u8.wrapping_shl(offset)) != 0) as u32);
    }
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IE,
        ((*pmic_eic).reg[CacheReg::RegIe as usize] & (1u8.wrapping_shl(offset)) != 0) as u32);
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_TRIG,
        ((*pmic_eic).reg[CacheReg::RegTrig as usize] & (1u8.wrapping_shl(offset)) != 0) as u32);
    mutex_unlock(&mut (*pmic_eic).buslock);
}

unsafe fn sprd_pmic_eic_toggle_trigger(chip: *mut gpio_chip, irq: u32, offset: u32) {
    let trigger = irq_get_trigger_type(irq);
    if trigger & IRQ_TYPE_EDGE_BOTH == 0 { return; }
    let mut state = sprd_pmic_eic_get(chip, offset);
    loop {
        if state != 0 { sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 0); }
        else { sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IEV, 1); }
        let post_state = sprd_pmic_eic_get(chip, offset);
        if state == post_state { break; }
        dev_warn((*chip).parent, "PMIC EIC level was changed.\n");
        state = post_state;
    }
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_IE, 1);
    sprd_pmic_eic_update(chip, offset, SPRD_PMIC_EIC_TRIG, 1);
}

unsafe extern "C" fn sprd_pmic_eic_irq_handler(_irq: i32, data: *mut c_void) -> irqreturn_t {
    let pmic_eic = data as *mut SprdPmicEic;
    let chip = &mut (*pmic_eic).chip as *mut gpio_chip;
    let mut val: u32 = 0;
    let ret = regmap_read((*pmic_eic).map, (*pmic_eic).offset + SPRD_PMIC_EIC_MIS as u32, &mut val);
    if ret != 0 { return IRQ_RETVAL(ret); }
    let mut status = val & SPRD_PMIC_EIC_DATA_MASK;
    while status != 0 {
        let n = status.trailing_zeros();
        status &= !(1u32 << n);
        sprd_pmic_eic_update(chip, n, SPRD_PMIC_EIC_IC, 1);
        let girq = irq_find_mapping((*chip).irq.domain, n);
        handle_nested_irq(girq);
        sprd_pmic_eic_toggle_trigger(chip, girq, n);
    }
    IRQ_HANDLED
}

unsafe fn sprd_pmic_eic_probe(pdev: *mut platform_device) -> i32 {
    let pmic_eic = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<SprdPmicEic>(), GFP_KERNEL) as *mut SprdPmicEic;
    if pmic_eic.is_null() { return -ENOMEM; }
    mutex_init(&mut (*pmic_eic).buslock);
    (*pmic_eic).irq = platform_get_irq(pdev, 0);
    if (*pmic_eic).irq < 0 { return (*pmic_eic).irq; }
    (*pmic_eic).map = dev_get_regmap((*pdev).dev.parent, core::ptr::null());
    if (*pmic_eic).map.is_null() { return -ENODEV; }
    let ret = of_property_read_u32((*pdev).dev.of_node, "reg", &mut (*pmic_eic).offset);
    if ret != 0 { dev_err(&mut (*pdev).dev, "Failed to get PMIC EIC base address.\n"); return ret; }
    let ret = devm_request_threaded_irq(&mut (*pdev).dev, (*pmic_eic).irq, None,
        Some(sprd_pmic_eic_irq_handler), IRQF_ONESHOT | IRQF_NO_SUSPEND,
        dev_name(&mut (*pdev).dev), pmic_eic as *mut c_void);
    if ret != 0 { dev_err(&mut (*pdev).dev, "Failed to request PMIC EIC IRQ.\n"); return ret; }
    (*pmic_eic).chip.label = dev_name(&mut (*pdev).dev);
    (*pmic_eic).chip.ngpio = SPRD_PMIC_EIC_NR;
    (*pmic_eic).chip.base = -1;
    (*pmic_eic).chip.parent = &mut (*pdev).dev;
    (*pmic_eic).chip.direction_input = Some(sprd_pmic_eic_direction_input);
    (*pmic_eic).chip.request = Some(sprd_pmic_eic_request);
    (*pmic_eic).chip.free = Some(sprd_pmic_eic_free);
    (*pmic_eic).chip.set_config = Some(sprd_pmic_eic_set_config);
    (*pmic_eic).chip.get = Some(sprd_pmic_eic_get);
    (*pmic_eic).chip.can_sleep = true;
    let irq = &mut (*pmic_eic).chip.irq;
    gpio_irq_chip_set_chip(irq, &mut pmic_eic_irq_chip);
    irq.threaded = true;
    let ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*pmic_eic).chip, pmic_eic as *mut c_void);
    if ret < 0 { dev_err(&mut (*pdev).dev, "Could not register gpiochip %d.\n", ret); }
    ret
}

static mut pmic_eic_irq_chip: irq_chip = irq_chip {
    name: "sprd-pmic-eic",
    irq_mask: Some(sprd_pmic_eic_irq_mask), irq_unmask: Some(sprd_pmic_eic_irq_unmask),
    irq_set_type: Some(sprd_pmic_eic_irq_set_type), irq_bus_lock: Some(sprd_pmic_eic_bus_lock),
    irq_bus_sync_unlock: Some(sprd_pmic_eic_bus_sync_unlock),
    flags: IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
};

static mut sprd_pmic_eic_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "sprd,sc2731-eic" }, of_device_id { compatible: core::ptr::null() },
];
static mut sprd_pmic_eic_driver: platform_driver = platform_driver {
    probe: Some(sprd_pmic_eic_probe),
    driver: device_driver { name: "sprd-pmic-eic", of_match_table: sprd_pmic_eic_of_match.as_ptr() },
};

// MODULE_DEVICE_TABLE(of, sprd_pmic_eic_of_match);
// module_platform_driver(sprd_pmic_eic_driver);
// MODULE_DESCRIPTION("Spreadtrum PMIC EIC driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
