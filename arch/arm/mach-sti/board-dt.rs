// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 STMicroelectronics (R&D) Limited.
 * Author(s): Srinivas Kandagatla <srinivas.kandagatla@st.com>
 */

// C dependencies:
// #include <asm/hardware/cache-l2x0.h>
// #include <asm/mach/arch.h>
// #include "smp.h"

use core::ffi::c_char;

// Constants and symbols supplied by the architecture and SMP dependencies.
extern "C" {
    static sti_smp_ops: SmpOperations;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct SmpOperations {
    _private: [u8; 0],
}

const L2C_AUX_CTRL_SHARED_OVERRIDE: u32 = 0;
const L310_AUX_CTRL_DATA_PREFETCH: u32 = 0;
const L310_AUX_CTRL_INSTR_PREFETCH: u32 = 0;

// Direct Rust equivalent of L2C_AUX_CTRL_WAY_SIZE(4).
const L2C_AUX_CTRL_WAY_SIZE_4: u32 = 4;

#[repr(C)]
pub struct MachineDesc {
    pub dt_compat: *const *const c_char,
    pub l2c_aux_val: u32,
    pub l2c_aux_mask: u32,
    pub smp: *const SmpOperations,
}

static STIH41X_DT_MATCH: [*const c_char; 4] = [
    b"st,stih407\0".as_ptr() as *const c_char,
    b"st,stih410\0".as_ptr() as *const c_char,
    b"st,stih418\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[no_mangle]
pub static STM: MachineDesc = MachineDesc {
    dt_compat: STIH41X_DT_MATCH.as_ptr(),
    l2c_aux_val: L2C_AUX_CTRL_SHARED_OVERRIDE
        | L310_AUX_CTRL_DATA_PREFETCH
        | L310_AUX_CTRL_INSTR_PREFETCH
        | L2C_AUX_CTRL_WAY_SIZE_4,
    l2c_aux_mask: 0xc0000fff,
    smp: unsafe { &sti_smp_ops as *const SmpOperations },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
