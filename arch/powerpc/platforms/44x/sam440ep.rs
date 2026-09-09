// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Sam440ep board specific routines based off bamboo.c code
 * original copyrights below
 *
 * Wade Farnsworth <wfarnsworth@mvista.com>
 * Copyright 2004 MontaVista Software Inc.
 *
 * Rewritten and ported to the merged powerpc tree:
 * Josh Boyer <jwboyer@linux.vnet.ibm.com>
 * Copyright 2007 IBM Corporation
 *
 * Modified from bamboo.c for sam440ep:
 * Copyright 2008 Giuseppe Coviello <gicoviello@gmail.com>
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_board_info {
    pub type_: [c_char; 32],
    pub addr: u16,
    pub irq: c_int,
}

extern "C" {
    fn of_platform_bus_probe(
        root: *mut c_void,
        matches: *const of_device_id,
        parent: *mut c_void,
    ) -> c_int;
    fn pci_set_flags(flags: c_int);
    fn i2c_register_board_info(busnum: c_int, info: *const i2c_board_info, len: c_int) -> c_int;

    fn udbg_progress(message: *const c_char, hex: c_uint);
    fn uic_init_tree();
    fn uic_get_irq() -> c_int;
    fn ppc4xx_reset_system();
}

type c_uint = u32;

const PCI_REASSIGN_ALL_RSRC: c_int = 0x0001;

static SAM440EP_OF_BUS: [of_device_id; 4] = [
    of_device_id {
        compatible: b"ibm,plb4\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"ibm,opb\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"ibm,ebc\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

unsafe fn sam440ep_device_probe() -> c_int {
    of_platform_bus_probe(
        core::ptr::null_mut(),
        SAM440EP_OF_BUS.as_ptr(),
        core::ptr::null_mut(),
    );

    0
}

// machine_device_initcall(sam440ep, sam440ep_device_probe);

unsafe fn sam440ep_probe() -> c_int {
    pci_set_flags(PCI_REASSIGN_ALL_RSRC);

    1
}

// define_machine(sam440ep) {
//     .name = "Sam440ep",
//     .compatible = "acube,sam440ep",
//     .probe = sam440ep_probe,
//     .progress = udbg_progress,
//     .init_IRQ = uic_init_tree,
//     .get_irq = uic_get_irq,
//     .restart = ppc4xx_reset_system,
// };

static mut SAM440EP_RTC_INFO: i2c_board_info = i2c_board_info {
    type_: [
        b'm' as c_char,
        b'4' as c_char,
        b'1' as c_char,
        b's' as c_char,
        b't' as c_char,
        b'8' as c_char,
        b'5' as c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    addr: 0x68,
    irq: -1,
};

unsafe fn sam440ep_setup_rtc() -> c_int {
    i2c_register_board_info(0, &SAM440EP_RTC_INFO, 1)
}

// machine_device_initcall(sam440ep, sam440ep_setup_rtc);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
