// SPDX-License-Identifier: GPL-2.0
/*
 * Hitachi UL SolutionEngine 7343 FPGA IRQ Support.
 *
 * Copyright (C) 2008  Yoshihiro Shimoda
 * Copyright (C) 2012  Paul Mundt
 *
 * Based on linux/arch/sh/boards/se/7343/irq.c
 * Copyright (C) 2007  Nobuhiro Iwamatsu
 */

// Dependency declarations and kernel-provided symbols are supplied by other
// translation units.

const DRV_NAME: &str = "SE7343-FPGA";

const PA_CPLD_BASE_ADDR: usize = 0x11400000;
const PA_CPLD_ST_REG: usize = 0x08; // CPLD Interrupt status register
const PA_CPLD_IMSK_REG: usize = 0x0a; // CPLD Interrupt mask register

static mut se7343_irq_regs: *mut core::ffi::c_void = core::ptr::null_mut();
static mut se7343_irq_domain: *mut irq_domain = core::ptr::null_mut();

#[repr(C)]
struct irq_desc {
    _private: [u8; 0],
}
#[repr(C)]
struct irq_data {
    _private: [u8; 0],
}
#[repr(C)]
struct irq_chip {
    irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}
#[repr(C)]
struct irq_domain {
    _private: [u8; 0],
}
#[repr(C)]
struct irq_chip_generic {
    chip_types: *mut irq_chip_type,
}
#[repr(C)]
struct irq_chip_type {
    chip: irq_chip,
    regs: irq_chip_regs,
}
#[repr(C)]
struct irq_chip_regs {
    mask: usize,
}

extern "C" {
    static irq_domain_simple_ops: core::ffi::c_void;
    fn irq_desc_get_irq_data(desc: *mut irq_desc) -> *mut irq_data;
    fn irq_data_get_irq_chip(data: *mut irq_data) -> *mut irq_chip;
    fn ioread16(addr: *mut u8) -> u16;
    fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: usize);
    fn irq_domain_create_linear(
        parent: *mut core::ffi::c_void,
        size: usize,
        ops: *const core::ffi::c_void,
        host_data: *mut core::ffi::c_void,
    ) -> *mut irq_domain;
    fn printk(fmt: *const u8, ...);
    fn irq_create_mapping(domain: *mut irq_domain, hwirq: usize) -> u32;
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: usize) -> u32;
    fn irq_alloc_generic_chip(
        name: *const u8,
        num_ct: u32,
        irq_base: u32,
        reg_base: *mut core::ffi::c_void,
        handler: unsafe extern "C" fn(),
    ) -> *mut irq_chip_generic;
    static handle_level_irq: unsafe extern "C" fn();
    static irq_gc_mask_set_bit: unsafe extern "C" fn();
    static irq_gc_mask_clr_bit: unsafe extern "C" fn();
    fn irq_setup_generic_chip(gc: *mut irq_chip_generic, mask: usize, flags: usize, exclude: usize, reserved: u32);
    fn irq_set_chained_handler(irq: u32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn irq_set_irq_type(irq: u32, irq_type: u32);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn iowrite16(value: u16, addr: *mut u8);
    fn __raw_writew(value: u16, addr: *mut core::ffi::c_void);
    fn pr_err(fmt: *const u8, ...);
}

const SE7343_FPGA_IRQ_NR: usize = 16;
const IRQ0_IRQ: u32 = 0;
const IRQ1_IRQ: u32 = 1;
const IRQ4_IRQ: u32 = 4;
const IRQ5_IRQ: u32 = 5;
const IRQ_TYPE_LEVEL_LOW: u32 = 8;
const IRQ_GC_INIT_MASK_CACHE: usize = 1;
const IRQ_NOREQUEST: usize = 1 << 0;
const IRQ_NOPROBE: usize = 1 << 1;

unsafe extern "C" fn se7343_irq_demux(desc: *mut irq_desc) {
    let data = irq_desc_get_irq_data(desc);
    let chip = irq_data_get_irq_chip(data);
    ((*chip).irq_mask_ack.unwrap())(data);

    let mask = ioread16((se7343_irq_regs as *mut u8).add(PA_CPLD_ST_REG));
    let mut bit = 0usize;
    while bit < SE7343_FPGA_IRQ_NR {
        if (mask & (1u16 << bit)) != 0 {
            generic_handle_domain_irq(se7343_irq_domain, bit);
        }
        bit += 1;
    }

    ((*chip).irq_unmask.unwrap())(data);
}

unsafe extern "C" fn se7343_domain_init() {
    se7343_irq_domain = irq_domain_create_linear(
        core::ptr::null_mut(),
        SE7343_FPGA_IRQ_NR,
        &irq_domain_simple_ops,
        core::ptr::null_mut(),
    );
    if se7343_irq_domain.is_null() {
        printk(b"Failed to get IRQ domain\0".as_ptr());
        return;
    }

    let mut i = 0usize;
    while i < SE7343_FPGA_IRQ_NR {
        let irq = irq_create_mapping(se7343_irq_domain, i);
        if irq == 0 {
            printk(b"Failed to allocate IRQ %d\n\0".as_ptr(), i);
            return;
        }
        i += 1;
    }
}

unsafe extern "C" fn se7343_gc_init() {
    let irq_base = irq_find_mapping(se7343_irq_domain, 0);
    let gc = irq_alloc_generic_chip(
        DRV_NAME.as_ptr(),
        1,
        irq_base,
        se7343_irq_regs,
        handle_level_irq,
    );
    if gc.is_null() {
        return;
    }

    let ct = (*gc).chip_types;
    (*ct).chip.irq_mask = Some(irq_gc_mask_set_bit);
    (*ct).chip.irq_unmask = Some(irq_gc_mask_clr_bit);
    (*ct).regs.mask = PA_CPLD_IMSK_REG;

    irq_setup_generic_chip(
        gc,
        (1usize << SE7343_FPGA_IRQ_NR) - 1,
        IRQ_GC_INIT_MASK_CACHE,
        IRQ_NOREQUEST | IRQ_NOPROBE,
        0,
    );

    irq_set_chained_handler(IRQ0_IRQ, se7343_irq_demux);
    irq_set_irq_type(IRQ0_IRQ, IRQ_TYPE_LEVEL_LOW);
    irq_set_chained_handler(IRQ1_IRQ, se7343_irq_demux);
    irq_set_irq_type(IRQ1_IRQ, IRQ_TYPE_LEVEL_LOW);
    irq_set_chained_handler(IRQ4_IRQ, se7343_irq_demux);
    irq_set_irq_type(IRQ4_IRQ, IRQ_TYPE_LEVEL_LOW);
    irq_set_chained_handler(IRQ5_IRQ, se7343_irq_demux);
    irq_set_irq_type(IRQ5_IRQ, IRQ_TYPE_LEVEL_LOW);
}

// Initialize IRQ setting
#[no_mangle]
pub unsafe extern "C" fn init_7343se_IRQ() {
    se7343_irq_regs = ioremap(PA_CPLD_BASE_ADDR, 16);
    if se7343_irq_regs.is_null() {
        pr_err(b"Failed to remap CPLD\0".as_ptr());
        return;
    }

    // All FPGA IRQs disabled by default
    iowrite16(0, (se7343_irq_regs as *mut u8).add(PA_CPLD_IMSK_REG));
    __raw_writew(0x2000, 0xb03fffec as *mut core::ffi::c_void); // mrshpc irq enable

    se7343_domain_init();
    se7343_gc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
