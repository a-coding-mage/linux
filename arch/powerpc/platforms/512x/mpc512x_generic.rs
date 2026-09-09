// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007,2008 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: John Rigby, <jrigby@freescale.com>
 *
 * Description:
 * MPC512x SoC setup
 */

use core::ffi::{c_char, c_int};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn mpc512x_init_early();
    fn mpc512x_init();
    fn mpc512x_setup_arch();
    fn mpc512x_init_IRQ();
    fn ipic_get_irq() -> c_int;
    fn mpc512x_restart();
}

/*
 * list of supported boards
 */
static BOARD: [*const c_char; 4] = [
    b"prt,prtlvt\0".as_ptr() as *const c_char,
    b"fsl,mpc5125ads\0".as_ptr() as *const c_char,
    b"ifm,ac14xx\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

/*
 * Called very early, MMU is off, device-tree isn't unflattened
 */
unsafe extern "C" fn mpc512x_generic_probe() -> c_int {
    mpc512x_init_early();

    1
}

// Equivalent to the architecture's define_machine(mpc512x_generic) declaration.
#[repr(C)]
struct MachineDesc {
    name: *const c_char,
    compatibles: *const *const c_char,
    probe: Option<unsafe extern "C" fn() -> c_int>,
    init: Option<unsafe extern "C" fn()>,
    setup_arch: Option<unsafe extern "C" fn()>,
    init_irq: Option<unsafe extern "C" fn()>,
    get_irq: Option<unsafe extern "C" fn() -> c_int>,
    restart: Option<unsafe extern "C" fn()>,
}

#[used]
static MPC512X_GENERIC: MachineDesc = MachineDesc {
    name: b"MPC512x generic\0".as_ptr() as *const c_char,
    compatibles: BOARD.as_ptr(),
    probe: Some(mpc512x_generic_probe),
    init: Some(mpc512x_init),
    setup_arch: Some(mpc512x_setup_arch),
    init_irq: Some(mpc512x_init_IRQ),
    get_irq: Some(ipic_get_irq),
    restart: Some(mpc512x_restart),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
