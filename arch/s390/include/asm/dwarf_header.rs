/* SPDX-License-Identifier: GPL-2.0 */

// This header's definitions are assembler-only CFI directives.  The Rust
// equivalents below preserve those directive names and values for consumers
// that need to emit the corresponding assembly text.

pub const CFI_STARTPROC: &str = ".cfi_startproc";
pub const CFI_ENDPROC: &str = ".cfi_endproc";
pub const CFI_DEF_CFA_OFFSET: &str = ".cfi_def_cfa_offset";
pub const CFI_ADJUST_CFA_OFFSET: &str = ".cfi_adjust_cfa_offset";
pub const CFI_RESTORE: &str = ".cfi_restore";
pub const CFI_REL_OFFSET: &str = ".cfi_rel_offset";

// CONFIG_AS_CFI_VAL_OFFSET selects the assembler directive; when it is not
// enabled, the original macro expands to an assembler comment marker.
#[cfg(CONFIG_AS_CFI_VAL_OFFSET)]
pub const CFI_VAL_OFFSET: &str = ".cfi_val_offset";

#[cfg(not(CONFIG_AS_CFI_VAL_OFFSET))]
pub const CFI_VAL_OFFSET: &str = "#";

// For non-BUILD_VDSO builds, CFI is emitted only in .debug_frame sections;
// vDSO builds emit it in both .eh_frame and .debug_frame sections.
#[cfg(not(BUILD_VDSO))]
pub const CFI_SECTIONS: &str = ".debug_frame";

#[cfg(BUILD_VDSO)]
pub const CFI_SECTIONS: &str = ".eh_frame, .debug_frame";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
