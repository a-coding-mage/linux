// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Nuvoton Technology corporation.
// Copyright 2018 Google, Inc.

// Translated from the Linux kernel machine-description declarations.

static NPCM7XX_DT_MATCH: [*const u8; 2] = [
    b"nuvoton,npcm750\0".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(NPCM7XX_DT, "NPCM7XX Chip family")
// MACHINE_END
// The machine-description type and its registration are supplied by the
// architecture dependencies represented by the original includes.
#[allow(non_upper_case_globals)]
pub static NPCM7XX_DT: MachineDesc = MachineDesc {
    name: b"NPCM7XX Chip family\0".as_ptr(),
    atag_offset: 0x100,
    dt_compat: NPCM7XX_DT_MATCH.as_ptr(),
    l2c_aux_val: 0x0,
    l2c_aux_mask: !0u32,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
