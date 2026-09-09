// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2008-2009 The GameCube Linux Team
// Copyright (C) 2008,2009 Albert Herranz
// Copyright (C) 2017-2018 Jonathan Neuschäfer
//
// Nintendo Wii (Hollywood) GPIO driver

// Dependencies supplied by the surrounding kernel Rust environment correspond
// to the C headers included by the original implementation.

const HW_GPIOB_OUT: usize = 0x00;
const HW_GPIOB_DIR: usize = 0x04;
const HW_GPIOB_IN: usize = 0x08;
const HW_GPIOB_INTLVL: usize = 0x0c;
const HW_GPIOB_INTFLAG: usize = 0x10;
const HW_GPIOB_INTMASK: usize = 0x14;
const HW_GPIOB_INMIR: usize = 0x18;
const HW_GPIO_ENABLE: usize = 0x1c;
const HW_GPIO_OUT: usize = 0x20;
const HW_GPIO_DIR: usize = 0x24;
const HW_GPIO_IN: usize = 0x28;
const HW_GPIO_INTLVL: usize = 0x2c;
const HW_GPIO_INTFLAG: usize = 0x30;
const HW_GPIO_INTMASK: usize = 0x34;
const HW_GPIO_INMIR: usize = 0x38;
const HW_GPIO_OWNER: usize = 0x3c;

#[repr(C)]
struct HlwdGpio {
    gpioc: gpio_generic_chip,
    dev: *mut device,
    regs: *mut core::ffi::c_void,
    irq: i32,
    edge_emulation: u32,
    rising_edge: u32,
    falling_edge: u32,
}

unsafe fn hlwd_gpio_irqhandler(desc: *mut irq_desc) {
    let hlwd = gpiochip_get_data(irq_desc_get_handler_data(desc)) as *mut HlwdGpio;
    let chip = irq_desc_get_chip(desc);
    let mut pending: usize;
    let mut emulated_pending: u32;

    let _guard = gpio_generic_lock_irqsave(&mut (*hlwd).gpioc);
    pending = ioread32be((*hlwd).regs.add(HW_GPIOB_INTFLAG)) as usize;
    pending &= ioread32be((*hlwd).regs.add(HW_GPIOB_INTMASK)) as usize;

    emulated_pending = (*hlwd).edge_emulation & pending as u32;
    pending &= !(emulated_pending as usize);
    if emulated_pending != 0 {
        let level = ioread32be((*hlwd).regs.add(HW_GPIOB_INTLVL));
        let mut rising = level & emulated_pending;
        let mut falling = !level & emulated_pending;
        iowrite32be(level ^ emulated_pending, (*hlwd).regs.add(HW_GPIOB_INTLVL));
        iowrite32be(emulated_pending, (*hlwd).regs.add(HW_GPIOB_INTFLAG));
        rising &= (*hlwd).rising_edge;
        falling &= (*hlwd).falling_edge;
        pending |= (rising | falling) as usize;
    }
    drop(_guard);

    chained_irq_enter(chip, desc);
    for hwirq in 0..32 {
        if (pending & (1usize << hwirq)) != 0 {
            generic_handle_domain_irq((*(*hlwd).gpioc.gc.irq.domain).irq_domain, hwirq);
        }
    }
    chained_irq_exit(chip, desc);
}

unsafe fn hlwd_gpio_irq_ack(data: *mut irq_data) {
    let hlwd = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut HlwdGpio;
    iowrite32be(1u32 << (*data).hwirq, (*hlwd).regs.add(HW_GPIOB_INTFLAG));
}

unsafe fn hlwd_gpio_irq_mask(data: *mut irq_data) {
    let hlwd = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut HlwdGpio;
    let _guard = gpio_generic_lock_irqsave(&mut (*hlwd).gpioc);
    let mut mask = ioread32be((*hlwd).regs.add(HW_GPIOB_INTMASK));
    mask &= !(1u32 << (*data).hwirq);
    iowrite32be(mask, (*hlwd).regs.add(HW_GPIOB_INTMASK));
    drop(_guard);
    gpiochip_disable_irq(&mut (*hlwd).gpioc.gc, irqd_to_hwirq(data));
}

unsafe fn hlwd_gpio_irq_unmask(data: *mut irq_data) {
    let hlwd = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut HlwdGpio;
    gpiochip_enable_irq(&mut (*hlwd).gpioc.gc, irqd_to_hwirq(data));
    let _guard = gpio_generic_lock_irqsave(&mut (*hlwd).gpioc);
    let mut mask = ioread32be((*hlwd).regs.add(HW_GPIOB_INTMASK));
    mask |= 1u32 << (*data).hwirq;
    iowrite32be(mask, (*hlwd).regs.add(HW_GPIOB_INTMASK));
}

unsafe fn hlwd_gpio_irq_enable(data: *mut irq_data) {
    hlwd_gpio_irq_ack(data);
    hlwd_gpio_irq_unmask(data);
}

unsafe fn hlwd_gpio_irq_setup_emulation(hlwd: *mut HlwdGpio, hwirq: i32, flow_type: u32) {
    let mut level = ioread32be((*hlwd).regs.add(HW_GPIOB_INTLVL));
    let state = ioread32be((*hlwd).regs.add(HW_GPIOB_IN)) & (1u32 << hwirq);
    level &= !(1u32 << hwirq);
    level |= state ^ (1u32 << hwirq);
    iowrite32be(level, (*hlwd).regs.add(HW_GPIOB_INTLVL));
    (*hlwd).edge_emulation |= 1u32 << hwirq;
    (*hlwd).rising_edge &= !(1u32 << hwirq);
    (*hlwd).falling_edge &= !(1u32 << hwirq);
    if flow_type & IRQ_TYPE_EDGE_RISING != 0 { (*hlwd).rising_edge |= 1u32 << hwirq; }
    if flow_type & IRQ_TYPE_EDGE_FALLING != 0 { (*hlwd).falling_edge |= 1u32 << hwirq; }
}

unsafe fn hlwd_gpio_irq_set_type(data: *mut irq_data, flow_type: u32) -> i32 {
    let hlwd = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut HlwdGpio;
    let _guard = gpio_generic_lock_irqsave(&mut (*hlwd).gpioc);
    (*hlwd).edge_emulation &= !(1u32 << (*data).hwirq);
    match flow_type {
        IRQ_TYPE_LEVEL_HIGH => { let level = ioread32be((*hlwd).regs.add(HW_GPIOB_INTLVL)) | (1u32 << (*data).hwirq); iowrite32be(level, (*hlwd).regs.add(HW_GPIOB_INTLVL)); }
        IRQ_TYPE_LEVEL_LOW => { let level = ioread32be((*hlwd).regs.add(HW_GPIOB_INTLVL)) & !(1u32 << (*data).hwirq); iowrite32be(level, (*hlwd).regs.add(HW_GPIOB_INTLVL)); }
        IRQ_TYPE_EDGE_RISING | IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_EDGE_BOTH => hlwd_gpio_irq_setup_emulation(hlwd, (*data).hwirq, flow_type),
        _ => return -EINVAL,
    }
    0
}

unsafe fn hlwd_gpio_irq_print_chip(data: *mut irq_data, p: *mut seq_file) {
    let hlwd = gpiochip_get_data(irq_data_get_irq_chip_data(data)) as *mut HlwdGpio;
    seq_puts(p, dev_name((*hlwd).dev));
}

static hlwd_gpio_irq_chip: irq_chip = irq_chip {
    irq_mask: Some(hlwd_gpio_irq_mask), irq_unmask: Some(hlwd_gpio_irq_unmask),
    irq_enable: Some(hlwd_gpio_irq_enable), irq_set_type: Some(hlwd_gpio_irq_set_type),
    irq_print_chip: Some(hlwd_gpio_irq_print_chip), flags: IRQCHIP_IMMUTABLE,
    ..IRQCHIP_IRQ_RESOURCE_HELPERS
};

unsafe fn hlwd_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut hlwd = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<HlwdGpio>(), GFP_KERNEL) as *mut HlwdGpio;
    if hlwd.is_null() { return -ENOMEM; }
    (*hlwd).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*hlwd).regs) { return PTR_ERR((*hlwd).regs); }
    (*hlwd).dev = &mut (*pdev).dev;
    iowrite32be(0xffff_ffff, (*hlwd).regs.add(HW_GPIO_OWNER));
    let config = gpio_generic_chip_config { dev: &mut (*pdev).dev, sz: 4, dat: (*hlwd).regs.add(HW_GPIOB_IN), set: (*hlwd).regs.add(HW_GPIOB_OUT), dirout: (*hlwd).regs.add(HW_GPIOB_DIR), flags: GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER };
    let mut res = gpio_generic_chip_init(&mut (*hlwd).gpioc, &config);
    if res < 0 { dev_warn(&mut (*pdev).dev, "failed to initialize generic GPIO chip: %d\n", res); return res; }
    let mut ngpios = 0u32;
    if of_property_read_u32((*pdev).dev.of_node, "ngpios", &mut ngpios) != 0 { ngpios = 32; }
    (*hlwd).gpioc.gc.ngpio = ngpios;
    iowrite32be(0, (*hlwd).regs.add(HW_GPIOB_INTMASK));
    iowrite32be(0xffff_ffff, (*hlwd).regs.add(HW_GPIOB_INTFLAG));
    if of_property_read_bool((*pdev).dev.of_node, "interrupt-controller") {
        (*hlwd).irq = platform_get_irq(pdev, 0);
        if (*hlwd).irq < 0 { dev_info(&mut (*pdev).dev, "platform_get_irq returned %d\n", (*hlwd).irq); return (*hlwd).irq; }
        let girq = &mut (*hlwd).gpioc.gc.irq;
        gpio_irq_chip_set_chip(girq, &hlwd_gpio_irq_chip);
        girq.parent_handler = Some(hlwd_gpio_irqhandler); girq.num_parents = 1;
        girq.parents = devm_kcalloc(&mut (*pdev).dev, 1, core::mem::size_of::<i32>(), GFP_KERNEL);
        if girq.parents.is_null() { return -ENOMEM; }
        *girq.parents = (*hlwd).irq; girq.default_type = IRQ_TYPE_NONE; girq.handler = Some(handle_level_irq);
    }
    res = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*hlwd).gpioc.gc, hlwd as *mut _);
    res
}

static hlwd_gpio_match: [of_device_id; 2] = [of_device_id { compatible: "nintendo,hollywood-gpio" }, of_device_id { compatible: "" }];
static hlwd_gpio_driver: platform_driver = platform_driver { driver: driver { name: "gpio-hlwd", of_match_table: &hlwd_gpio_match }, probe: Some(hlwd_gpio_probe) };
module_platform_driver!(hlwd_gpio_driver);

MODULE_DEVICE_TABLE!(of, hlwd_gpio_match);
MODULE_AUTHOR!("Jonathan Neuschäfer <j.neuschaefer@gmx.net>");
MODULE_DESCRIPTION!("Nintendo Wii GPIO driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
