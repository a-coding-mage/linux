// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2008 Thomas Chou <thomas@wytron.com.tw>
 *
 * based on irq.c from m68k which is:
 *
 * Copyright (C) 2007 Greg Ungerer <gerg@snapgear.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Linux kernel dependencies supplied by other translation units.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_data {
    pub hwirq: c_uint,
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

pub type irq_hw_number_t = c_uint;

#[repr(C)]
pub struct irq_chip {
    pub name: *const c_char,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[repr(C)]
pub struct irq_domain_ops {
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, c_uint, irq_hw_number_t) -> c_int>,
    pub xlate: Option<*const c_void>,
}

unsafe extern "C" {
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    fn irq_enter();
    fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: c_int);
    fn irq_exit();
    fn irq_set_chip_and_handler(virq: c_uint, chip: *mut irq_chip, handler: *const c_void);
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_fwnode_handle(node: *mut device_node) -> *mut c_void;
    fn irq_domain_create_linear(
        fwnode: *mut c_void,
        size: c_uint,
        ops: *const irq_domain_ops,
        host_data: *mut c_void,
    ) -> *mut irq_domain;
    fn irq_set_default_domain(domain: *mut irq_domain);
    fn of_node_put(node: *mut device_node);
    fn irq_domain_xlate_onecell();
    fn handle_level_irq();
    fn BUG_ON(condition: bool);
}

// These names are supplied by the platform headers/preprocessor environment.
extern "C" {
    static NIOS2_CPU_NR_IRQS: c_uint;
    static CTL_IENABLE: c_uint;
    fn WRCTL(control: c_uint, value: u32);
    fn RDCTL(control: c_uint) -> u32;
}

static mut ienable: u32 = 0;

pub unsafe extern "C" fn do_IRQ(hwirq: c_int, regs: *mut pt_regs) {
    let oldregs = set_irq_regs(regs);

    irq_enter();
    generic_handle_domain_irq(core::ptr::null_mut(), hwirq);
    irq_exit();

    set_irq_regs(oldregs);
}

unsafe extern "C" fn chip_unmask(d: *mut irq_data) {
    ienable |= 1u32.wrapping_shl((*d).hwirq);
    WRCTL(CTL_IENABLE, ienable);
}

unsafe extern "C" fn chip_mask(d: *mut irq_data) {
    ienable &= !(1u32.wrapping_shl((*d).hwirq));
    WRCTL(CTL_IENABLE, ienable);
}

static mut m_irq_chip: irq_chip = irq_chip {
    name: b"NIOS2-INTC\0".as_ptr() as *const c_char,
    irq_unmask: Some(chip_unmask),
    irq_mask: Some(chip_mask),
};

unsafe extern "C" fn irq_map(
    _h: *mut irq_domain,
    virq: c_uint,
    _hw_irq_num: irq_hw_number_t,
) -> c_int {
    irq_set_chip_and_handler(
        virq,
        &raw mut m_irq_chip,
        handle_level_irq as *const c_void,
    );

    0
}

static irq_ops: irq_domain_ops = irq_domain_ops {
    map: Some(irq_map),
    xlate: Some(irq_domain_xlate_onecell as *const c_void),
};

pub unsafe extern "C" fn init_IRQ() {
    let mut domain: *mut irq_domain;
    let mut node: *mut device_node;

    node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"altr,nios2-1.0\0".as_ptr() as *const c_char,
    );
    if node.is_null() {
        node = of_find_compatible_node(
            core::ptr::null_mut(),
            core::ptr::null(),
            b"altr,nios2-1.1\0".as_ptr() as *const c_char,
        );
    }

    BUG_ON(node.is_null());

    domain = irq_domain_create_linear(
        of_fwnode_handle(node),
        NIOS2_CPU_NR_IRQS,
        &irq_ops,
        core::ptr::null_mut(),
    );
    BUG_ON(domain.is_null());

    irq_set_default_domain(domain);
    of_node_put(node);
    // Load the initial ienable value
    ienable = RDCTL(CTL_IENABLE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
