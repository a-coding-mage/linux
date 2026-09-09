/* SPDX-License-Identifier: GPL-2.0+ */

// External declarations corresponding to the C header's unsigned long globals.
extern "C" {
    pub static mut kprobes_target_odd_offs: core::ffi::c_ulong;
    pub static mut kprobes_target_in_insn4_offs: core::ffi::c_ulong;
    pub static mut kprobes_target_in_insn6_lo_offs: core::ffi::c_ulong;
    pub static mut kprobes_target_in_insn6_hi_offs: core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
