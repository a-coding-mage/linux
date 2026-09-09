// SPDX-License-Identifier: GPL-2.0+
//
// MXS GPIO support. (c) 2008 Daniel Mack <daniel@caiaq.de>
// Copyright 2008 Juergen Beisert, kernel@pengutronix.de
//
// Based on code from Freescale,
// Copyright (C) 2004-2010 Freescale Semiconductor, Inc. All Rights Reserved.

// Linux kernel dependencies are supplied by the surrounding translation.

const MXS_SET: usize = 0x4;
const MXS_CLR: usize = 0x8;

const GPIO_INT_FALL_EDGE: u32 = 0x0;
const GPIO_INT_LOW_LEV: u32 = 0x1;
const GPIO_INT_RISE_EDGE: u32 = 0x2;
const GPIO_INT_HIGH_LEV: u32 = 0x3;
const GPIO_INT_LEV_MASK: u32 = 1 << 0;
const GPIO_INT_POL_MASK: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mxs_gpio_id {
    IMX23_GPIO,
    IMX28_GPIO,
}

#[repr(C)]
struct mxs_gpio_port {
    base: *mut core::ffi::c_void,
    id: i32,
    irq: i32,
    domain: *mut irq_domain,
    chip: gpio_generic_chip,
    dev: *mut device,
    devid: mxs_gpio_id,
    both_edges: u32,
}

#[inline]
unsafe fn is_imx23_gpio(port: *mut mxs_gpio_port) -> bool {
    (*port).devid == mxs_gpio_id::IMX23_GPIO
}

unsafe fn pinctrl_dout(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0500 } else { 0x0700 }) + (*port).id as usize * 0x10
}
unsafe fn pinctrl_din(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0600 } else { 0x0900 }) + (*port).id as usize * 0x10
}
unsafe fn pinctrl_doe(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0700 } else { 0x0b00 }) + (*port).id as usize * 0x10
}
unsafe fn pinctrl_pin2irq(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0800 } else { 0x1000 }) + (*port).id as usize * 0x10
}
unsafe fn pinctrl_irqen(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0900 } else { 0x1100 }) + (*port).id as usize * 0x10
}
unsafe fn pinctrl_irqlev(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0a00 } else { 0x1200 }) + (*port).id as usize * 0x10
}
unsafe fn pinctrl_irqpol(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0b00 } else { 0x1300 }) + (*port).id as usize * 0x10
}
unsafe fn pinctrl_irqstat(port: *mut mxs_gpio_port) -> usize {
    (if is_imx23_gpio(port) { 0x0c00 } else { 0x1400 }) + (*port).id as usize * 0x10
}

unsafe fn mxs_gpio_set_irq_type(d: *mut irq_data, irq_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d) as *mut irq_chip_generic;
    let ct = irq_data_get_chip_type(d) as *mut irq_chip_type;
    let port = (*gc).private as *mut mxs_gpio_port;
    let pin_mask = 1u32 << (*d).hwirq;
    if (*ct).type_ & irq_type == 0 && irq_setup_alt_chip(d, irq_type) != 0 { return -EINVAL; }
    (*port).both_edges &= !pin_mask;
    let edge = match irq_type {
        IRQ_TYPE_EDGE_BOTH => { let val = readl((*port).base.add(pinctrl_din(port))) & pin_mask; (*port).both_edges |= pin_mask; if val != 0 { GPIO_INT_FALL_EDGE } else { GPIO_INT_RISE_EDGE } },
        IRQ_TYPE_EDGE_RISING => GPIO_INT_RISE_EDGE,
        IRQ_TYPE_EDGE_FALLING => GPIO_INT_FALL_EDGE,
        IRQ_TYPE_LEVEL_LOW => GPIO_INT_LOW_LEV,
        IRQ_TYPE_LEVEL_HIGH => GPIO_INT_HIGH_LEV,
        _ => return -EINVAL,
    };
    let pin_addr = (*port).base.add(pinctrl_irqlev(port));
    if edge & GPIO_INT_LEV_MASK != 0 { writel(pin_mask, pin_addr.add(MXS_SET)); writel(pin_mask, (*port).base.add(pinctrl_irqen(port) + MXS_SET)); }
    else { writel(pin_mask, pin_addr.add(MXS_CLR)); writel(pin_mask, (*port).base.add(pinctrl_pin2irq(port) + MXS_SET)); }
    let pin_addr = (*port).base.add(pinctrl_irqpol(port));
    writel(pin_mask, pin_addr.add(if edge & GPIO_INT_POL_MASK != 0 { MXS_SET } else { MXS_CLR }));
    writel(pin_mask, (*port).base.add(pinctrl_irqstat(port) + MXS_CLR));
    0
}

unsafe fn mxs_flip_edge(port: *mut mxs_gpio_port, gpio: u32) {
    let bit = 1u32 << gpio;
    let pin_addr = (*port).base.add(pinctrl_irqpol(port));
    let edge = readl(pin_addr) & bit;
    writel(bit, pin_addr.add(if edge != 0 { MXS_CLR } else { MXS_SET }));
}

unsafe fn mxs_gpio_irq_handler(desc: *mut irq_desc) {
    let port = irq_desc_get_handler_data(desc) as *mut mxs_gpio_port;
    ((*(*desc).irq_data.chip).irq_ack.unwrap())(&mut (*desc).irq_data);
    let mut irq_stat = readl((*port).base.add(pinctrl_irqstat(port))) & readl((*port).base.add(pinctrl_irqen(port)));
    while irq_stat != 0 {
        let irqoffset = (31 - irq_stat.leading_zeros()) as u32;
        if (*port).both_edges & (1 << irqoffset) != 0 { mxs_flip_edge(port, irqoffset); }
        generic_handle_domain_irq((*port).domain, irqoffset);
        irq_stat &= !(1 << irqoffset);
    }
}

unsafe fn mxs_gpio_set_wake_irq(d: *mut irq_data, enable: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d) as *mut irq_chip_generic;
    let port = (*gc).private as *mut mxs_gpio_port;
    if enable != 0 { enable_irq_wake((*port).irq); } else { disable_irq_wake((*port).irq); }
    0
}

unsafe fn mxs_gpio_to_irq(gc: *mut gpio_chip, offset: u32) -> i32 {
    let port = gpiochip_get_data(gc) as *mut mxs_gpio_port;
    irq_find_mapping((*port).domain, offset)
}

unsafe fn mxs_gpio_get_direction(gc: *mut gpio_chip, offset: u32) -> i32 {
    let port = gpiochip_get_data(gc) as *mut mxs_gpio_port;
    if readl((*port).base.add(pinctrl_doe(port))) & (1 << offset) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char, data: *const core::ffi::c_void }

static MXS_GPIO_DT_IDS: [of_device_id; 3] = [
    of_device_id { compatible: b"fsl,imx23-gpio\0".as_ptr() as *const _, data: IMX23_GPIO as usize as *const _ },
    of_device_id { compatible: b"fsl,imx28-gpio\0".as_ptr() as *const _, data: IMX28_GPIO as usize as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn mxs_gpio_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let port = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mxs_gpio_port>(), GFP_KERNEL) as *mut mxs_gpio_port;
    if port.is_null() { return -ENOMEM; }
    (*port).id = of_alias_get_id(np, b"gpio\0".as_ptr() as *const _);
    if (*port).id < 0 { return (*port).id; }
    (*port).devid = *(of_device_get_match_data(&mut (*pdev).dev) as *const mxs_gpio_id);
    (*port).dev = &mut (*pdev).dev;
    (*port).irq = platform_get_irq(pdev, 0);
    if (*port).irq < 0 { return (*port).irq; }
    let parent = of_get_parent(np);
    (*port).base = of_iomap(parent, 0);
    of_node_put(parent);
    if (*port).base.is_null() { return -EADDRNOTAVAIL; }
    writel(0, (*port).base.add(pinctrl_pin2irq(port)));
    writel(0, (*port).base.add(pinctrl_irqen(port)));
    writel(!0u32, (*port).base.add(pinctrl_irqstat(port) + MXS_CLR));
    let irq_base = devm_irq_alloc_descs(&mut (*pdev).dev, -1, 0, 32, numa_node_id());
    if irq_base < 0 { iounmap((*port).base); return irq_base; }
    (*port).domain = irq_domain_create_legacy(dev_fwnode(&mut (*pdev).dev), 32, irq_base, 0, &irq_domain_simple_ops, core::ptr::null_mut());
    if (*port).domain.is_null() { iounmap((*port).base); return -ENODEV; }
    let err = mxs_gpio_init_gc(port, irq_base);
    if err < 0 { irq_domain_remove((*port).domain); iounmap((*port).base); return err; }
    irq_set_chained_handler_and_data((*port).irq, Some(mxs_gpio_irq_handler), port);
    let config = gpio_generic_chip_config { dev: &mut (*pdev).dev, sz: 4, dat: (*port).base.add(pinctrl_din(port)), set: (*port).base.add(pinctrl_dout(port)+MXS_SET), clr: (*port).base.add(pinctrl_dout(port)+MXS_CLR), dirout: (*port).base.add(pinctrl_doe(port)) };
    let err = gpio_generic_chip_init(&mut (*port).chip, &config);
    if err != 0 { irq_domain_remove((*port).domain); iounmap((*port).base); return err; }
    (*port).chip.gc.to_irq = Some(mxs_gpio_to_irq); (*port).chip.gc.get_direction = Some(mxs_gpio_get_direction); (*port).chip.gc.base = (*port).id * 32;
    let err = gpiochip_add_data(&mut (*port).chip.gc, port);
    if err != 0 { irq_domain_remove((*port).domain); iounmap((*port).base); }
    err
}

unsafe fn mxs_gpio_init() -> i32 { platform_driver_register(&mut MXS_GPIO_DRIVER) }

static mut MXS_GPIO_DRIVER: platform_driver = platform_driver {
    driver: driver { name: b"gpio-mxs\0".as_ptr() as *const _, of_match_table: MXS_GPIO_DT_IDS.as_ptr(), suppress_bind_attrs: true },
    probe: Some(mxs_gpio_probe),
};

// postcore_initcall(mxs_gpio_init);
// MODULE_AUTHOR("Freescale Semiconductor, Daniel Mack, Juergen Beisert");
// MODULE_DESCRIPTION("Freescale MXS GPIO");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
