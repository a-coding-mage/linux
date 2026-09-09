// SPDX-License-Identifier: GPL-2.0
/*
 * r8a73a4 processor support
 *
 * Copyright (C) 2013  Renesas Solutions Corp.
 * Copyright (C) 2013  Magnus Damm
 */

// C dependencies supplied by the kernel headers and common implementation.
use core::ffi::c_char;

extern "C" {
    fn shmobile_init_late();
}

static R8A73A4_BOARDS_COMPAT_DT: [*const c_char; 2] = [
    b"renesas,r8a73a4\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Expansion of DT_MACHINE_START(R8A73A4_DT,
// "Generic R8A73A4 (Flattened Device Tree)") ... MACHINE_END.
// `MachineDesc` is supplied by the architecture dependencies.
pub static R8A73A4_DT: MachineDesc = MachineDesc {
    name: b"Generic R8A73A4 (Flattened Device Tree)\0".as_ptr() as *const c_char,
    init_late: Some(shmobile_init_late),
    dt_compat: R8A73A4_BOARDS_COMPAT_DT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
