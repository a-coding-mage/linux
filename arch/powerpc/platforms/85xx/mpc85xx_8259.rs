// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPC85xx 8259 functions for DS Board Setup
 *
 * Author Xianghua Xiao (x.xiao@freescale.com)
 * Roy Zang <tie-fei.zang@freescale.com>
 *      - Add PCI/PCI Express support
 * Copyright 2007 Freescale Semiconductor Inc.
 */

// C includes translated as dependencies on symbols supplied by other files.

#[repr(C)]
pub struct irq_desc {
    pub irq_data: irq_data,
}

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_chip {
    pub irq_eoi: Option<unsafe extern "C" fn(data: *mut irq_data)>,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
    fn i8259_irq() -> u32;
    fn generic_handle_irq(irq: u32);
    fn irq_of_parse_and_map(node: *mut device_node, index: u32) -> i32;
    fn i8259_init(node: *mut device_node, offset: u32);
    fn of_node_put(node: *mut device_node);
    fn irq_set_chained_handler(irq: i32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn of_device_is_compatible(node: *mut device_node, compatible: *const u8) -> bool;
    fn of_first_node_by_type(node_type: *const u8) -> *mut device_node;
    fn of_next_node_by_type(node: *mut device_node, node_type: *const u8) -> *mut device_node;
}

unsafe extern "C" fn mpc85xx_8259_cascade(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    let cascade_irq = i8259_irq();

    if cascade_irq != 0 {
        generic_handle_irq(cascade_irq);
    }

    if let Some(irq_eoi) = (*chip).irq_eoi {
        irq_eoi(&mut (*desc).irq_data);
    }
}

pub unsafe extern "C" fn mpc85xx_8259_init() {
    let mut np: *mut device_node;
    let mut cascade_node: *mut device_node = core::ptr::null_mut();
    let mut cascade_irq: i32;

    /* Initialize the i8259 controller */
    np = of_first_node_by_type(b"interrupt-controller\0".as_ptr());
    while !np.is_null() {
        if of_device_is_compatible(np, b"chrp,iic\0".as_ptr()) {
            cascade_node = np;
            break;
        }
        np = of_next_node_by_type(np, b"interrupt-controller\0".as_ptr());
    }

    if cascade_node.is_null() {
        // pr_debug("i8259: Could not find i8259 PIC\n");
        return;
    }

    cascade_irq = irq_of_parse_and_map(cascade_node, 0);
    if cascade_irq == 0 {
        // pr_err("i8259: Failed to map cascade interrupt\n");
        return;
    }

    // pr_debug("i8259: cascade mapped to irq %d\n", cascade_irq);

    i8259_init(cascade_node, 0);
    of_node_put(cascade_node);

    irq_set_chained_handler(cascade_irq, mpc85xx_8259_cascade);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
