/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020-2022 Loongson Technology Corporation Limited */

// Linker-script translation: the sections below are aligned to 4 bytes and
// each begins at address 0 with one zero byte.
#[repr(C)]
pub struct ModuleLdsSections {
    pub got: [u8; 1],
    pub plt: [u8; 1],
    pub plt_idx: [u8; 1],
    pub ftrace_trampoline: [u8; 1],
}

pub const MODULE_LDS_SECTIONS: ModuleLdsSections = ModuleLdsSections {
    got: [0],
    plt: [0],
    plt_idx: [0],
    ftrace_trampoline: [0],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
