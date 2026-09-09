// SPDX-License-Identifier: GPL-2.0-only
/*
 * Routines common to most mpc85xx-based boards.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct fsl_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_chip {
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[repr(C)]
pub struct irq_desc {
    pub irq_data: irq_data,
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

pub static qoriq_pm_ops: *const fsl_pm_ops = core::ptr::null();

extern "C" {
    fn of_platform_bus_probe(
        root: *mut device_node,
        matches: *const of_device_id,
        parent: *mut device_node,
    ) -> c_int;
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
    fn cpm2_get_irq() -> c_int;
    fn generic_handle_irq(irq: c_int);
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn printk(fmt: *const c_char, ...);
    fn irq_of_parse_and_map(np: *mut device_node, index: c_int) -> c_int;
    fn of_node_put(np: *mut device_node);
    fn cpm2_pic_init(np: *mut device_node);
    fn irq_set_chained_handler(irq: c_int, handler: unsafe extern "C" fn(*mut irq_desc));
    fn of_find_node_by_name(from: *mut device_node, name: *const c_char) -> *mut device_node;
    fn par_io_init(np: *mut device_node);
    fn par_io_of_config(ucc: *mut device_node);
}

static MPC85XX_COMMON_IDS: &[of_device_id] = &[
    of_device_id { name: core::ptr::null(), type_: c"soc".as_ptr(), compatible: core::ptr::null(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"soc".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"simple-bus".as_ptr(), data: core::ptr::null() },
    of_device_id { name: c"cpm".as_ptr(), type_: core::ptr::null(), compatible: core::ptr::null(), data: core::ptr::null() },
    of_device_id { name: c"localbus".as_ptr(), type_: core::ptr::null(), compatible: core::ptr::null(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"gianfar".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,qe".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,cpm2".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,srio".as_ptr(), data: core::ptr::null() },
    // So that the DMA channel nodes can be probed individually:
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,eloplus-dma".as_ptr(), data: core::ptr::null() },
    // For the PMC driver
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,mpc8548-guts".as_ptr(), data: core::ptr::null() },
    // Probably unnecessary?
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"gpio-leds".as_ptr(), data: core::ptr::null() },
    // For all PCI controllers
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,mpc8540-pci".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,mpc8548-pcie".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,p1022-pcie".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,p1010-pcie".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,p1023-pcie".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,p4080-pcie".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,qoriq-pcie-v2.4".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,qoriq-pcie-v2.3".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,qoriq-pcie-v2.2".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,fman".as_ptr(), data: core::ptr::null() },
    // IFC NAND and NOR controllers
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: c"fsl,ifc".as_ptr(), data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe extern "C" fn mpc85xx_common_publish_devices() -> c_int {
    of_platform_bus_probe(core::ptr::null_mut(), MPC85XX_COMMON_IDS.as_ptr(), core::ptr::null_mut())
}

#[cfg(CONFIG_CPM2)]
unsafe extern "C" fn cpm2_cascade(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    let mut cascade_irq;
    while {
        cascade_irq = cpm2_get_irq();
        cascade_irq >= 0
    } {
        generic_handle_irq(cascade_irq);
    }
    if let Some(irq_eoi) = (*chip).irq_eoi {
        irq_eoi(&mut (*desc).irq_data);
    }
}

#[cfg(CONFIG_CPM2)]
pub unsafe extern "C" fn mpc85xx_cpm2_pic_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,cpm2-pic".as_ptr());
    if np.is_null() {
        printk(c"PIC init: can not find fsl,cpm2-pic node\\n".as_ptr());
        return;
    }
    let irq = irq_of_parse_and_map(np, 0);
    if irq == 0 {
        of_node_put(np);
        printk(c"PIC init: got no IRQ for cpm cascade\\n".as_ptr());
        return;
    }
    cpm2_pic_init(np);
    of_node_put(np);
    irq_set_chained_handler(irq, cpm2_cascade);
}

#[cfg(CONFIG_QUICC_ENGINE)]
pub unsafe extern "C" fn mpc85xx_qe_par_io_init() {
    let np = of_find_node_by_name(core::ptr::null_mut(), c"par_io".as_ptr());
    if !np.is_null() {
        par_io_init(np);
        of_node_put(np);
        let mut ucc = of_find_node_by_name(core::ptr::null_mut(), c"ucc".as_ptr());
        while !ucc.is_null() {
            par_io_of_config(ucc);
            ucc = of_find_node_by_name(ucc, c"ucc".as_ptr());
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
