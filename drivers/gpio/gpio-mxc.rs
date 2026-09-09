// SPDX-License-Identifier: GPL-2.0+
//
// MXC GPIO support. (c) 2008 Daniel Mack <daniel@caiaq.de>
// Copyright 2008 Juergen Beisert, kernel@pengutronix.de
//
// Based on code from Freescale Semiconductor,
// Authors: Daniel Mack, Juergen Beisert.
// Copyright (C) 2004-2010 Freescale Semiconductor, Inc. All Rights Reserved.

// Linux kernel dependencies supplied by the surrounding translation unit.

const IMX_SCU_WAKEUP_OFF: u32 = 0;
const IMX_SCU_WAKEUP_LOW_LVL: u32 = 4;
const IMX_SCU_WAKEUP_FALL_EDGE: u32 = 5;
const IMX_SCU_WAKEUP_RISE_EDGE: u32 = 6;
const IMX_SCU_WAKEUP_HIGH_LVL: u32 = 7;

#[repr(C)]
struct mxc_gpio_hwdata {
    dr_reg: u32,
    gdir_reg: u32,
    psr_reg: u32,
    icr1_reg: u32,
    icr2_reg: u32,
    imr_reg: u32,
    isr_reg: u32,
    edge_sel_reg: i32,
    low_level: u32,
    high_level: u32,
    rise_edge: u32,
    fall_edge: u32,
}

#[repr(C)]
struct mxc_gpio_reg_saved { icr1: u32, icr2: u32, imr: u32, gdir: u32, edge_sel: u32, dr: u32 }

#[repr(C)]
struct mxc_gpio_port {
    node: list_head,
    base: *mut core::ffi::c_void,
    clk: *mut clk,
    irq: i32,
    irq_high: i32,
    mx_irq_handler: Option<unsafe extern "C" fn(*mut irq_desc)>,
    domain: *mut irq_domain,
    gen_gc: gpio_generic_chip,
    dev: *mut device,
    both_edges: u32,
    gpio_saved_reg: mxc_gpio_reg_saved,
    power_off: bool,
    wakeup_pads: u32,
    is_pad_wakeup: bool,
    pad_type: [u32; 32],
    hwdata: *const mxc_gpio_hwdata,
}

static mut imx1_imx21_gpio_hwdata: mxc_gpio_hwdata = mxc_gpio_hwdata { dr_reg:0x1c,gdir_reg:0,psr_reg:0x24,icr1_reg:0x28,icr2_reg:0x2c,imr_reg:0x30,isr_reg:0x34,edge_sel_reg:-22,low_level:3,high_level:2,rise_edge:0,fall_edge:1 };
static mut imx31_gpio_hwdata: mxc_gpio_hwdata = mxc_gpio_hwdata { dr_reg:0,gdir_reg:4,psr_reg:8,icr1_reg:0xc,icr2_reg:0x10,imr_reg:0x14,isr_reg:0x18,edge_sel_reg:-22,low_level:0,high_level:1,rise_edge:2,fall_edge:3 };
static mut imx35_gpio_hwdata: mxc_gpio_hwdata = mxc_gpio_hwdata { dr_reg:0,gdir_reg:4,psr_reg:8,icr1_reg:0xc,icr2_reg:0x10,imr_reg:0x14,isr_reg:0x18,edge_sel_reg:0x1c,low_level:0,high_level:1,rise_edge:2,fall_edge:3 };

// GPIO_* macros from the C source are represented by direct field accesses below.
const GPIO_INT_BOTH_EDGES: u32 = 0x4;

extern "C" {
    static mut mxc_gpio_ports: list_head;
    fn gpio_set_irq_type(d: *mut irq_data, type_: u32) -> i32;
}

unsafe fn gpio_set_irq_type_impl(d: *mut irq_data, type_: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d) as *mut irq_chip_generic;
    let port = (*gc).private as *mut mxc_gpio_port;
    let gpio_idx = (*d).hwirq as u32;
    let hw = &*(*port).hwdata;
    (*port).both_edges &= !(1u32 << gpio_idx);
    let mut edge: u32;
    match type_ {
        IRQ_TYPE_EDGE_RISING => edge = hw.rise_edge,
        IRQ_TYPE_EDGE_FALLING => edge = hw.fall_edge,
        IRQ_TYPE_EDGE_BOTH => {
            if hw.edge_sel_reg >= 0 { edge = GPIO_INT_BOTH_EDGES; }
            else {
                let val = ((*port).gen_gc.gc.get.unwrap())(&mut (*port).gen_gc.gc, gpio_idx);
                if val != 0 { edge = hw.low_level; } else { edge = hw.high_level; }
                (*port).both_edges |= 1u32 << gpio_idx;
            }
        },
        IRQ_TYPE_LEVEL_LOW => edge = hw.low_level,
        IRQ_TYPE_LEVEL_HIGH => edge = hw.high_level,
        _ => return -22,
    }
    let reg = (*port).base as *mut u8;
    if hw.edge_sel_reg >= 0 {
        let v = readl(reg.add(hw.edge_sel_reg as usize) as *const u32);
        if edge == GPIO_INT_BOTH_EDGES { writel(v | (1 << gpio_idx), reg.add(hw.edge_sel_reg as usize) as *mut u32); }
        else { writel(v & !(1 << gpio_idx), reg.add(hw.edge_sel_reg as usize) as *mut u32); }
    }
    if edge != GPIO_INT_BOTH_EDGES {
        let r = reg.add((hw.icr1_reg + ((gpio_idx & 0x10) >> 2)) as usize) as *mut u32;
        let bit = gpio_idx & 0xf;
        let v = readl(r) & !(0x3 << (bit << 1));
        writel(v | (edge << (bit << 1)), r);
    }
    writel(1 << gpio_idx, reg.add(hw.isr_reg as usize) as *mut u32);
    (*port).pad_type[gpio_idx as usize] = type_;
    ((*port).gen_gc.gc.direction_input.unwrap())(&mut (*port).gen_gc.gc, gpio_idx)
}

unsafe fn mxc_flip_edge(port: *mut mxc_gpio_port, gpio: u32) {
    let hw = &*(*port).hwdata;
    let r = ((*port).base as *mut u8).add((hw.icr1_reg + ((gpio & 0x10) >> 2)) as usize) as *mut u32;
    let bit = gpio & 0xf;
    let mut val = readl(r);
    let mut edge = (val >> (bit << 1)) & 3;
    val &= !(0x3 << (bit << 1));
    if edge == hw.high_level { edge = hw.low_level; }
    else if edge == hw.low_level { edge = hw.high_level; }
    else { return; }
    writel(val | (edge << (bit << 1)), r);
}

unsafe fn mxc_gpio_irq_handler(port: *mut mxc_gpio_port, mut irq_stat: u32) {
    while irq_stat != 0 {
        let irqoffset = 31 - irq_stat.leading_zeros();
        if (*port).both_edges & (1 << irqoffset) != 0 { mxc_flip_edge(port, irqoffset); }
        generic_handle_domain_irq((*port).domain, irqoffset);
        irq_stat &= !(1 << irqoffset);
    }
}

unsafe extern "C" fn mx3_gpio_irq_handler(desc: *mut irq_desc) {
    let port = irq_desc_get_handler_data(desc) as *mut mxc_gpio_port;
    if (*port).is_pad_wakeup { return; }
    let chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    let b = (*port).base as *mut u8;
    let hw = &*(*port).hwdata;
    let stat = readl(b.add(hw.isr_reg as usize) as *const u32) & readl(b.add(hw.imr_reg as usize) as *const u32);
    mxc_gpio_irq_handler(port, stat);
    chained_irq_exit(chip, desc);
}

unsafe extern "C" fn mx2_gpio_irq_handler(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    list_for_each_entry!(port, &mut mxc_gpio_ports, node, mxc_gpio_port, {
        let b = (*port).base as *mut u8; let hw = &*(*port).hwdata;
        let mask = readl(b.add(hw.imr_reg as usize) as *const u32);
        if mask != 0 { let stat = readl(b.add(hw.isr_reg as usize) as *const u32) & mask; if stat != 0 { mxc_gpio_irq_handler(port, stat); } }
    });
    chained_irq_exit(chip, desc);
}

// Remaining declarations and registration retain the C driver's external kernel interfaces.
extern "C" {
    fn gpio_set_wake_irq(d: *mut irq_data, enable: u32) -> i32;
    fn mxc_gpio_probe(pdev: *mut platform_device) -> i32;
    fn mxc_gpio_runtime_suspend(dev: *mut device) -> i32;
    fn mxc_gpio_runtime_resume(dev: *mut device) -> i32;
    fn mxc_gpio_noirq_suspend(dev: *mut device) -> i32;
    fn mxc_gpio_noirq_resume(dev: *mut device) -> i32;
    fn gpio_mxc_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
