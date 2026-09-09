// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ebony board specific routines
 *
 * Matt Porter <mporter@kernel.crashing.org>
 * Copyright 2002-2005 MontaVista Software Inc.
 *
 * Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 * Copyright (c) 2003-2005 Zultys Technologies
 *
 * Rewritten and ported to the merged powerpc tree:
 * Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/of_platform.h, linux/rtc.h,
// asm/machdep.h, asm/prom.h, asm/udbg.h, asm/time.h, asm/uic.h,
// asm/pci-bridge.h, and asm/ppc4xx.h.

extern "C" {
    fn of_platform_bus_probe(
        bus: *mut core::ffi::c_void,
        matches: *const of_device_id,
        parent: *mut core::ffi::c_void,
    );
    fn of_instantiate_rtc();
    fn pci_set_flags(flags: u32);
    fn udbg_progress(message: *const core::ffi::c_char, hex: u32);
    fn uic_init_tree();
    fn uic_get_irq() -> u32;
    fn ppc4xx_reset_system() -> !;
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

static EBONY_OF_BUS: [of_device_id; 4] = [
    of_device_id { compatible: b"ibm,plb4\0".as_ptr() as *const core::ffi::c_char },
    of_device_id { compatible: b"ibm,opb\0".as_ptr() as *const core::ffi::c_char },
    of_device_id { compatible: b"ibm,ebc\0".as_ptr() as *const core::ffi::c_char },
    of_device_id { compatible: core::ptr::null() },
];

// __init
unsafe fn ebony_device_probe() -> i32 {
    of_platform_bus_probe(
        core::ptr::null_mut(),
        EBONY_OF_BUS.as_ptr(),
        core::ptr::null_mut(),
    );
    of_instantiate_rtc();

    0
}

// machine_device_initcall(ebony, ebony_device_probe)

/*
 * Called very early, MMU is off, device-tree isn't unflattened
 */
// __init
unsafe fn ebony_probe() -> i32 {
    pci_set_flags(PCI_REASSIGN_ALL_RSRC);

    1
}

// define_machine(ebony)
#[repr(C)]
pub struct machdep_calls {
    pub name: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub probe: unsafe fn() -> i32,
    pub progress: Option<unsafe extern "C" fn(*const core::ffi::c_char, u32)>,
    pub init_IRQ: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> u32>,
    pub restart: Option<unsafe extern "C" fn() -> !>,
}

static EBONY_MACHINE: machdep_calls = machdep_calls {
    name: b"Ebony\0".as_ptr() as *const core::ffi::c_char,
    compatible: b"ibm,ebony\0".as_ptr() as *const core::ffi::c_char,
    probe: ebony_probe,
    progress: Some(udbg_progress),
    init_IRQ: Some(uic_init_tree),
    get_irq: Some(uic_get_irq),
    restart: Some(ppc4xx_reset_system),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
