// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2016 Broadcom

use core::ffi::c_char;

// C dependency: <asm/mach/arch.h>

static BCM23550_DT_COMPAT_0: &[u8] = b"brcm,bcm23550\0";

static bcm23550_dt_compat: [*const c_char; 2] = [
    BCM23550_DT_COMPAT_0.as_ptr() as *const c_char,
    core::ptr::null(),
];

// C macro expansion preserved from:
// DT_MACHINE_START(BCM23550_DT, "BCM23550 Broadcom Application Processor")
//     .dt_compat = bcm23550_dt_compat,
// MACHINE_END
// The machine descriptor and registration are supplied by the architecture
// dependency represented by <asm/mach/arch.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
