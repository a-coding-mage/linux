// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2008 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_PPC_I8259")]
unsafe extern "C" {
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
    fn i8259_irq() -> u32;
    fn generic_handle_irq(irq: u32);
    fn irq_of_parse_and_map(node: *mut device_node, index: u32) -> u32;
    fn i8259_init(node: *mut device_node, index: u32);
    fn of_node_put(node: *mut device_node);
    fn irq_set_chained_handler(irq: u32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn printk(format: *const core::ffi::c_char, ...);
    fn of_device_is_compatible(
        node: *mut device_node,
        compatible: *const core::ffi::c_char,
    ) -> bool;
}

#[cfg(feature = "CONFIG_PPC_I8259")]
#[repr(C)]
pub struct irq_desc {
    pub irq_data: irq_data,
}

#[cfg(feature = "CONFIG_PPC_I8259")]
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_PPC_I8259")]
#[repr(C)]
pub struct irq_chip {
    pub irq_eoi: Option<unsafe extern "C" fn(data: *mut irq_data)>,
}

#[cfg(feature = "CONFIG_PPC_I8259")]
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn mpic_alloc(
        node: *mut core::ffi::c_void,
        flags: u32,
        op_flags: u32,
        irq_offset: u32,
        irq_count: u32,
        name: *const core::ffi::c_char,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn bug_on(condition: bool);
}

#[repr(C)]
pub struct mpic {
    _private: [u8; 0],
}

const MPIC_BIG_ENDIAN: u32 = 1 << 0;
const MPIC_SINGLE_DEST_CPU: u32 = 1 << 1;

#[cfg(feature = "CONFIG_PPC_I8259")]
unsafe extern "C" fn mpc86xx_8259_cascade(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    let cascade_irq = i8259_irq();

    if cascade_irq != 0 {
        generic_handle_irq(cascade_irq);
    }

    if let Some(irq_eoi) = (*chip).irq_eoi {
        irq_eoi(&mut (*desc).irq_data);
    }
}

pub unsafe extern "C" fn mpc86xx_init_irq() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        256,
        b" MPIC     \0".as_ptr() as *const core::ffi::c_char,
    );
    bug_on(mpic.is_null());

    mpic_init(mpic);

    // CONFIG_PPC_I8259 is a build-time condition in the original source.
    #[cfg(feature = "CONFIG_PPC_I8259")]
    {
        let mut cascade_node: *mut device_node = core::ptr::null_mut();
        let mut cascade_irq: i32;

        // Original: for_each_node_by_type(np, "interrupt-controller")
        // and select the first node compatible with "chrp,iic".
        let mut np: *mut device_node = core::ptr::null_mut();
        while !np.is_null() {
            if of_device_is_compatible(np, b"chrp,iic\0".as_ptr() as *const core::ffi::c_char) {
                cascade_node = np;
                break;
            }
            break;
        }

        if cascade_node.is_null() {
            printk(b"Could not find i8259 PIC\0".as_ptr() as *const core::ffi::c_char);
            return;
        }

        cascade_irq = irq_of_parse_and_map(cascade_node, 0) as i32;
        if cascade_irq == 0 {
            printk(b"Failed to map cascade interrupt\0".as_ptr() as *const core::ffi::c_char);
            return;
        }

        i8259_init(cascade_node, 0);
        of_node_put(cascade_node);

        irq_set_chained_handler(cascade_irq as u32, mpc86xx_8259_cascade);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
