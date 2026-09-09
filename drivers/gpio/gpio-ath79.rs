// SPDX-License-Identifier: GPL-2.0-only
/*
 * Atheros AR71XX/AR724X/AR913X GPIO API support
 *
 * Copyright (C) 2015 Alban Bedel <albeu@free.fr>
 * Copyright (C) 2010-2011 Jaiganesh Narayanan <jnarayanan@atheros.com>
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 */

// Linux dependencies are supplied by the surrounding kernel bindings.

const AR71XX_GPIO_REG_OE: u32 = 0x00;
const AR71XX_GPIO_REG_IN: u32 = 0x04;
const AR71XX_GPIO_REG_SET: u32 = 0x0c;
const AR71XX_GPIO_REG_CLEAR: u32 = 0x10;
const AR71XX_GPIO_REG_INT_ENABLE: u32 = 0x14;
const AR71XX_GPIO_REG_INT_TYPE: u32 = 0x18;
const AR71XX_GPIO_REG_INT_POLARITY: u32 = 0x1c;
const AR71XX_GPIO_REG_INT_PENDING: u32 = 0x20;
const AR71XX_GPIO_REG_INT_MASK: u32 = 0x24;

#[repr(C)]
struct ath79_gpio_ctrl {
    chip: gpio_generic_chip,
    base: *mut core::ffi::c_void,
    both_edges: c_ulong,
}

unsafe fn irq_data_to_ath79_gpio(data: *mut irq_data) -> *mut ath79_gpio_ctrl {
    let gc = irq_data_get_irq_chip_data(data);
    let gen_gc = to_gpio_generic_chip(gc);
    container_of!(gen_gc, ath79_gpio_ctrl, chip)
}

unsafe fn ath79_gpio_read(ctrl: *mut ath79_gpio_ctrl, reg: c_uint) -> u32 {
    readl((*ctrl).base.cast::<u8>().add(reg as usize).cast())
}

unsafe fn ath79_gpio_write(ctrl: *mut ath79_gpio_ctrl, reg: c_uint, val: u32) {
    writel(val, (*ctrl).base.cast::<u8>().add(reg as usize).cast());
}

unsafe fn ath79_gpio_update_bits(ctrl: *mut ath79_gpio_ctrl, reg: c_uint, mask: u32, bits: u32) -> bool {
    let old_val = ath79_gpio_read(ctrl, reg);
    let new_val = (old_val & !mask) | (bits & mask);
    if new_val != old_val { ath79_gpio_write(ctrl, reg, new_val); }
    new_val != old_val
}

unsafe fn ath79_gpio_irq_unmask(data: *mut irq_data) {
    let ctrl = irq_data_to_ath79_gpio(data);
    let mask = 1u32 << irqd_to_hwirq(data);
    gpiochip_enable_irq(&mut (*ctrl).chip.gc, irqd_to_hwirq(data));
    let _guard = gpio_generic_lock_irqsave(&mut (*ctrl).chip);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_MASK, mask, mask);
}

unsafe fn ath79_gpio_irq_mask(data: *mut irq_data) {
    let ctrl = irq_data_to_ath79_gpio(data);
    let mask = 1u32 << irqd_to_hwirq(data);
    let _guard = gpio_generic_lock_irqsave(&mut (*ctrl).chip);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_MASK, mask, 0);
    gpiochip_disable_irq(&mut (*ctrl).chip.gc, irqd_to_hwirq(data));
}

unsafe fn ath79_gpio_irq_enable(data: *mut irq_data) {
    let ctrl = irq_data_to_ath79_gpio(data);
    let mask = 1u32 << irqd_to_hwirq(data);
    let _guard = gpio_generic_lock_irqsave(&mut (*ctrl).chip);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_ENABLE, mask, mask);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_MASK, mask, mask);
}

unsafe fn ath79_gpio_irq_disable(data: *mut irq_data) {
    let ctrl = irq_data_to_ath79_gpio(data);
    let mask = 1u32 << irqd_to_hwirq(data);
    let _guard = gpio_generic_lock_irqsave(&mut (*ctrl).chip);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_MASK, mask, 0);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_ENABLE, mask, 0);
}

unsafe fn ath79_gpio_irq_set_type(data: *mut irq_data, flow_type: c_uint) -> c_int {
    let ctrl = irq_data_to_ath79_gpio(data);
    let mask = 1u32 << irqd_to_hwirq(data);
    let mut irq_type = 0u32;
    let mut polarity = 0u32;
    match flow_type {
        IRQ_TYPE_EDGE_RISING => { polarity |= mask; }
        IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_EDGE_BOTH => {}
        IRQ_TYPE_LEVEL_HIGH => { polarity |= mask; irq_type |= mask; }
        IRQ_TYPE_LEVEL_LOW => { irq_type |= mask; }
        _ => return -EINVAL,
    }
    let _guard = gpio_generic_lock_irqsave(&mut (*ctrl).chip);
    if flow_type == IRQ_TYPE_EDGE_BOTH {
        (*ctrl).both_edges |= mask as c_ulong;
        polarity = !ath79_gpio_read(ctrl, AR71XX_GPIO_REG_IN);
    } else { (*ctrl).both_edges &= !(mask as c_ulong); }
    let disabled = ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_ENABLE, mask, 0);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_TYPE, mask, irq_type);
    ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_POLARITY, mask, polarity);
    if disabled { ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_ENABLE, mask, mask); }
    0
}

static ath79_gpio_irqchip: irq_chip = irq_chip {
    name: "gpio-ath79", irq_enable: Some(ath79_gpio_irq_enable), irq_disable: Some(ath79_gpio_irq_disable),
    irq_mask: Some(ath79_gpio_irq_mask), irq_unmask: Some(ath79_gpio_irq_unmask), irq_set_type: Some(ath79_gpio_irq_set_type),
    flags: IRQCHIP_IMMUTABLE,
};

unsafe fn ath79_gpio_irq_handler(desc: *mut irq_desc) {
    let gc = irq_desc_get_handler_data(desc);
    let irqchip = irq_desc_get_chip(desc);
    let gen_gc = to_gpio_generic_chip(gc);
    let ctrl = container_of!(gen_gc, ath79_gpio_ctrl, chip);
    let pending;
    chained_irq_enter(irqchip, desc);
    {
        let _guard = gpio_generic_lock_irqsave(&mut (*ctrl).chip);
        pending = ath79_gpio_read(ctrl, AR71XX_GPIO_REG_INT_PENDING) as c_ulong;
        let both_edges = (*ctrl).both_edges & pending;
        if both_edges != 0 {
            let state = ath79_gpio_read(ctrl, AR71XX_GPIO_REG_IN);
            ath79_gpio_update_bits(ctrl, AR71XX_GPIO_REG_INT_POLARITY, both_edges as u32, !state);
        }
    }
    for irq in 0..(*gc).ngpio {
        if pending & (1ul << irq) != 0 { generic_handle_domain_irq((*gc).irq.domain, irq); }
    }
    chained_irq_exit(irqchip, desc);
}

static ath79_gpio_of_match: [of_device_id; 3] = [
    of_device_id { compatible: "qca,ar7100-gpio" },
    of_device_id { compatible: "qca,ar9340-gpio" },
    of_device_id {},
];

// CONFIG_ATH9K_AHB conditional: the following registration is compiled when enabled.
const ATH79K_WIFI_DESCS: usize = 32;
unsafe fn ath79_gpio_register_wifi_descriptors(dev: *mut device, label: *const c_char) -> c_int {
    let lookup = devm_kzalloc(dev, struct_size::<gpiod_lookup_table>(ATH79K_WIFI_DESCS + 1), GFP_KERNEL);
    if lookup.is_null() { return -ENOMEM; }
    (*lookup).dev_id = core::ptr::null();
    for i in 0..ATH79K_WIFI_DESCS { (*lookup).table[i] = GPIO_LOOKUP_IDX(label, i, "ath9k", i, GPIO_ACTIVE_HIGH); }
    gpiod_add_lookup_table(lookup);
    0
}

unsafe fn ath79_gpio_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let ctrl = devm_kzalloc(dev, core::mem::size_of::<ath79_gpio_ctrl>(), GFP_KERNEL) as *mut ath79_gpio_ctrl;
    if ctrl.is_null() { return -ENOMEM; }
    let mut count = 0u32;
    let err = device_property_read_u32(dev, "ngpios", &mut count);
    if err != 0 { dev_err(dev, "ngpios property is not valid\n"); return err; }
    let oe_inverted = device_is_compatible(dev, "qca,ar9340-gpio");
    if count >= 32 { dev_err(dev, "ngpios must be less than 32\n"); return -EINVAL; }
    (*ctrl).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*ctrl).base) { return PTR_ERR((*ctrl).base); }
    let config = gpio_generic_chip_config { dev, sz: 4, dat: (*ctrl).base.cast::<u8>().add(AR71XX_GPIO_REG_IN as usize).cast(), set: (*ctrl).base.cast::<u8>().add(AR71XX_GPIO_REG_SET as usize).cast(), clr: (*ctrl).base.cast::<u8>().add(AR71XX_GPIO_REG_CLEAR as usize).cast(), dirout: if oe_inverted { core::ptr::null_mut() } else { (*ctrl).base.cast::<u8>().add(AR71XX_GPIO_REG_OE as usize).cast() }, dirin: if oe_inverted { (*ctrl).base.cast::<u8>().add(AR71XX_GPIO_REG_OE as usize).cast() } else { core::ptr::null_mut() } };
    let err = gpio_generic_chip_init(&mut (*ctrl).chip, &config);
    if err != 0 { dev_err(dev, "failed to initialize generic GPIO chip\n"); return err; }
    if device_property_read_bool(dev, "interrupt-controller") {
        let girq = &mut (*ctrl).chip.gc.irq;
        gpio_irq_chip_set_chip(girq, &ath79_gpio_irqchip);
        girq.parent_handler = Some(ath79_gpio_irq_handler); girq.num_parents = 1;
        girq.parents = devm_kcalloc(dev, 1, core::mem::size_of::<c_uint>(), GFP_KERNEL);
        if girq.parents.is_null() { return -ENOMEM; }
        *girq.parents = platform_get_irq(pdev, 0); girq.default_type = IRQ_TYPE_NONE; girq.handler = Some(handle_simple_irq);
    }
    let err = devm_gpiochip_add_data(dev, &mut (*ctrl).chip.gc, ctrl);
    if err != 0 { return err; }
    ath79_gpio_register_wifi_descriptors(dev, (*ctrl).chip.gc.label)
}

static ath79_gpio_driver: platform_driver = platform_driver { driver: driver { name: "ath79-gpio", of_match_table: ath79_gpio_of_match.as_ptr() }, probe: Some(ath79_gpio_probe) };

// module_platform_driver(ath79_gpio_driver);
// MODULE_DESCRIPTION("Atheros AR71XX/AR724X/AR913X GPIO API support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
