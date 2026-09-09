/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Translated from asm/linkage.h. The original assembler-only macros are
// preserved below as Rust-side documentation because their expansions are
// target assembler directives and instructions, not Rust expressions.

/// Use '`' to mark a new line in an assembler macro.
pub const ASM_NL: &str = "`";

/// Original assembler alignment directive: `.align 4`.
pub const ALIGN: &str = ".align 4";

/// String form of `ALIGN`.
pub const ALIGN_STR: &str = ".align 4";

/*
 * Original assembler macro:
 *
 * .macro ST2 e, o, off
 * #ifdef CONFIG_ARC_HAS_LL64
 *     std \\e, [sp, \\off]
 * #else
 *     st \\e, [sp, \\off]
 *     st \\o, [sp, \\off+4]
 * #endif
 * .endm
 */

/*
 * Original assembler macro:
 *
 * .macro LD2 e, o, off
 * #ifdef CONFIG_ARC_HAS_LL64
 *     ldd \\e, [sp, \\off]
 * #else
 *     ld \\e, [sp, \\off]
 *     ld \\o, [sp, \\off+4]
 * #endif
 * .endm
 */

/*
 * ARCFP_DATA selects `.data.arcfp` when CONFIG_ARC_HAS_DCCM is enabled and
 * otherwise selects `.data`, then makes the supplied symbol global.
 */

/*
 * ARCFP_CODE selects `.text.arcfp` with executable/progbits attributes when
 * CONFIG_ARC_HAS_ICCM is enabled and otherwise selects `.text` with the same
 * attributes.
 */

/*
 * ENTRY_CFI(name) emits:
 *   .globl name
 *   .align 4
 *   name:
 *   CFI_STARTPROC
 *
 * END_CFI(name) emits CFI_ENDPROC followed by `.size name, .-name`.
 */

// C-side section annotations represented by the original conditional macros:
//   __arcfp_code = __section(".text.arcfp") if CONFIG_ARC_HAS_ICCM,
//                  otherwise __section(".text").
//   __arcfp_data = __section(".data.arcfp") if CONFIG_ARC_HAS_DCCM,
//                  otherwise __section(".data").


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
