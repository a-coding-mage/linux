/* SPDX-License-Identifier: GPL-2.0-only */

// Original header guard: _ASM_ARC_KDEBUG_H

#[repr(C)]
pub enum die_val {
    DIE_UNUSED = 0,
    DIE_TRAP,
    DIE_OOPS,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
