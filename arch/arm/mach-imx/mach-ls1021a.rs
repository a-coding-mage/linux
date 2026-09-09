// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013-2014 Freescale Semiconductor, Inc.
 */

// C dependencies supplied by the surrounding kernel sources:
// #include <asm/mach/arch.h>
// #include "common.h"

use core::ffi::c_char;

/// Device-tree compatibility strings for the LS1021A machine.
#[used]
static LS1021A_DT_COMPAT: [*const c_char; 2] = [
    b"fsl,ls1021a\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// The following machine descriptor is emitted by the kernel's
// DT_MACHINE_START/ MACHINE_END registration macros in the C source.
// `smp_ops(ls1021a_smp_ops)` supplies the SMP operations and
// `LS1021A_DT_COMPAT` supplies the device-tree compatibility table.
unsafe extern "C" {
    static ls1021a_smp_ops: core::ffi::c_void;
}

// Equivalent registration intent:
// DT_MACHINE_START(LS1021A, "Freescale LS1021A")
//     .smp        = smp_ops(ls1021a_smp_ops),
//     .dt_compat  = ls1021a_dt_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
