/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Helge Deller <deller@gmx.de>
 */

// The following definitions correspond to the C header's __ASSEMBLER__
// conditional. They retain the assembler directive text represented by the
// original macros.
pub const CFI_STARTPROC: &str = ".cfi_startproc";
pub const CFI_ENDPROC: &str = ".cfi_endproc";
pub const CFI_DEF_CFA: &str = ".cfi_def_cfa";
pub const CFI_REGISTER: &str = ".cfi_register";
pub const CFI_REL_OFFSET: &str = ".cfi_rel_offset";
pub const CFI_UNDEFINED: &str = ".cfi_undefined";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
