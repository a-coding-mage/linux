/* SPDX-License-Identifier: GPL-2.0 */

// LSU Control Register
pub const LSU_CONTROL_PM: u64 = 0x000001fe00000000u64; // Phys-watchpoint byte mask
pub const LSU_CONTROL_VM: u64 = 0x00000001fe000000u64; // Virt-watchpoint byte mask
pub const LSU_CONTROL_PR: u64 = 0x0000000001000000u64; // Phys-rd watchpoint enable
pub const LSU_CONTROL_PW: u64 = 0x0000000000800000u64; // Phys-wr watchpoint enable
pub const LSU_CONTROL_VR: u64 = 0x0000000000400000u64; // Virt-rd watchpoint enable
pub const LSU_CONTROL_VW: u64 = 0x0000000000200000u64; // Virt-wr watchpoint enable
pub const LSU_CONTROL_FM: u64 = 0x00000000000ffff0u64; // Parity mask enables.
pub const LSU_CONTROL_DM: u64 = 0x0000000000000008u64; // Data MMU enable.
pub const LSU_CONTROL_IM: u64 = 0x0000000000000004u64; // Instruction MMU enable.
pub const LSU_CONTROL_DC: u64 = 0x0000000000000002u64; // Data cache enable.
pub const LSU_CONTROL_IC: u64 = 0x0000000000000001u64; // Instruction cache enable.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
