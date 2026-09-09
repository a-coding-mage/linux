/* SPDX-License-Identifier: GPL-2.0 */

// Original header guard: _ASM_X86_DWARF2_H
// This header is intended to be included only in pure assembly files.

pub const CFI_STARTPROC: &str = ".cfi_startproc";
pub const CFI_ENDPROC: &str = ".cfi_endproc";
pub const CFI_DEF_CFA: &str = ".cfi_def_cfa";
pub const CFI_DEF_CFA_REGISTER: &str = ".cfi_def_cfa_register";
pub const CFI_DEF_CFA_OFFSET: &str = ".cfi_def_cfa_offset";
pub const CFI_ADJUST_CFA_OFFSET: &str = ".cfi_adjust_cfa_offset";
pub const CFI_OFFSET: &str = ".cfi_offset";
pub const CFI_REL_OFFSET: &str = ".cfi_rel_offset";
pub const CFI_REGISTER: &str = ".cfi_register";
pub const CFI_RESTORE: &str = ".cfi_restore";
pub const CFI_REMEMBER_STATE: &str = ".cfi_remember_state";
pub const CFI_RESTORE_STATE: &str = ".cfi_restore_state";
pub const CFI_UNDEFINED: &str = ".cfi_undefined";
pub const CFI_ESCAPE: &str = ".cfi_escape";
pub const CFI_SIGNAL_FRAME: &str = ".cfi_signal_frame";

// When BUILD_VDSO is not defined, emit CFI data in .debug_frame sections,
// not .eh_frame sections. The latter are currently discarded because DWARF
// unwinding is not performed at runtime, so only offline DWARF information
// is useful. This directive should not be used if runtime DWARF unwinding is
// ever enabled.
// .cfi_sections .debug_frame

// When BUILD_VDSO is defined, emit both runtime unwind information and debug
// symbols for the .dbg file.
// .cfi_sections .eh_frame, .debug_frame

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
