// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic PowerPC 44x platform support
 *
 * Copyright 2008 IBM Corporation
 *
 * This implements simple platform support for PowerPC 44x chips.  This is
 * mostly used for eval boards or other simple and "generic" 44x boards.  If
 * your board has custom functions or hardware, then you will likely want to
 * implement your own board.c file to accommodate it.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// asm/machdep.h, asm/pci-bridge.h, asm/ppc4xx.h, asm/time.h, asm/udbg.h,
// asm/uic.h, linux/init.h, and linux/of_platform.h

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}

extern "C" {
    fn of_platform_bus_probe(
        root: *mut c_void,
        matches: *const OfDeviceId,
        parent: *mut c_void,
    );
    fn of_machine_is_compatible(name: *const c_char) -> c_int;
    fn pci_set_flags(flags: c_int);
    fn udbg_progress(message: *const c_char, hex: c_int);
    fn uic_init_tree();
    fn uic_get_irq() -> c_int;
    fn ppc4xx_reset_system();
}

pub const PCI_REASSIGN_ALL_RSRC: c_int = 0x00000001;

#[link_section = ".init.rodata"]
pub static ppc44x_of_bus: [OfDeviceId; 5] = [
    OfDeviceId { compatible: b"ibm,plb4\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ibm,opb\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ibm,ebc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"simple-bus\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

pub unsafe extern "C" fn ppc44x_device_probe() -> c_int {
    of_platform_bus_probe(
        core::ptr::null_mut(),
        ppc44x_of_bus.as_ptr(),
        core::ptr::null_mut(),
    );

    0
}

// Equivalent of machine_device_initcall(ppc44x_simple, ppc44x_device_probe).

/* This is the list of boards that can be supported by this simple
 * platform code.  This does _not_ mean the boards are compatible,
 * as they most certainly are not from a device tree perspective.
 * However, their differences are handled by the device tree and the
 * drivers and therefore they don't need custom board support files.
 *
 * Again, if your board needs to do things differently then create a
 * board.c file for it rather than adding it to this list.
 */
#[link_section = ".init.data"]
pub static board: [*const c_char; 13] = [
    b"amcc,arches\0".as_ptr() as *const c_char,
    b"amcc,bamboo\0".as_ptr() as *const c_char,
    b"apm,bluestone\0".as_ptr() as *const c_char,
    b"amcc,glacier\0".as_ptr() as *const c_char,
    b"ibm,ebony\0".as_ptr() as *const c_char,
    b"amcc,eiger\0".as_ptr() as *const c_char,
    b"amcc,katmai\0".as_ptr() as *const c_char,
    b"amcc,rainier\0".as_ptr() as *const c_char,
    b"amcc,redwood\0".as_ptr() as *const c_char,
    b"amcc,sequoia\0".as_ptr() as *const c_char,
    b"amcc,taishan\0".as_ptr() as *const c_char,
    b"amcc,yosemite\0".as_ptr() as *const c_char,
    b"mosaixtech,icon\0".as_ptr() as *const c_char,
];

pub unsafe extern "C" fn ppc44x_probe() -> c_int {
    let mut i: usize = 0;

    while i < board.len() {
        if of_machine_is_compatible(board[i]) != 0 {
            pci_set_flags(PCI_REASSIGN_ALL_RSRC);
            return 1;
        }
        i += 1;
    }

    0
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub probe: unsafe extern "C" fn() -> c_int,
    pub progress: unsafe extern "C" fn(*const c_char, c_int),
    pub init_irq: unsafe extern "C" fn(),
    pub get_irq: unsafe extern "C" fn() -> c_int,
    pub restart: unsafe extern "C" fn(),
}

#[no_mangle]
pub static ppc44x_simple: MachineDesc = MachineDesc {
    name: b"PowerPC 44x Platform\0".as_ptr() as *const c_char,
    probe: ppc44x_probe,
    progress: udbg_progress,
    init_irq: uic_init_tree,
    get_irq: uic_get_irq,
    restart: ppc4xx_reset_system,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
