// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Marvell MMP3 aka PXA2128 aka 88AP2128 support
 *
 *  Copyright (C) 2019 Lubomir Rintel <lkundrak@v3.sk>
 */

use core::ffi::{c_char, c_uint};

// Supplied by asm/mach/arch.h, asm/hardware/cache-l2x0.h, and common.h.
unsafe extern "C" {
    fn mmp2_map_io();
}

// The terminating null pointer is part of the C compatibility table.
#[used]
static MMP3_DT_BOARD_COMPAT: [*const c_char; 2] = [
    c"marvell,mmp3".as_ptr(),
    core::ptr::null(),
];

// Equivalent representation of the DT_MACHINE_START(MMP2_DT, ...) descriptor.
#[repr(C)]
struct MachineDesc {
    map_io: unsafe extern "C" fn(),
    dt_compat: *const *const c_char,
    l2c_aux_val: c_uint,
    l2c_aux_mask: c_uint,
}

const L310_AUX_CTRL_FWA_SHIFT: u32 = 0;
const L310_AUX_CTRL_DATA_PREFETCH: c_uint = 1 << 28;
const L310_AUX_CTRL_INSTR_PREFETCH: c_uint = 1 << 29;

#[used]
static MMP2_DT: MachineDesc = MachineDesc {
    map_io: mmp2_map_io,
    dt_compat: MMP3_DT_BOARD_COMPAT.as_ptr(),
    l2c_aux_val: (1 << L310_AUX_CTRL_FWA_SHIFT)
        | L310_AUX_CTRL_DATA_PREFETCH
        | L310_AUX_CTRL_INSTR_PREFETCH,
    l2c_aux_mask: 0xc20fffff,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
