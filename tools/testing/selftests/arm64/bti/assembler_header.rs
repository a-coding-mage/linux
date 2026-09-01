/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019  Arm Limited
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

pub const NT_GNU_PROPERTY_TYPE_0: u32 = 5;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_AND: u32 = 0xc0000000;

/* Bits for GNU_PROPERTY_AARCH64_FEATURE_1_BTI */
pub const GNU_PROPERTY_AARCH64_FEATURE_1_BTI: u32 = 1u32 << 0;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_PAC: u32 = 1u32 << 1;

/*
 * GNU assembler macro translated as assembly source text. The original C
 * header defines this for assembler inclusion:
 *
 * .macro startfn name:req
 *      .globl \name
 * \name:
 *      .macro endfn
 *              .size \name, . - \name
 *              .type \name, @function
 *              .purgem endfn
 *      .endm
 * .endm
 */
pub const STARTFN_ASM: &str = r#"
.macro startfn name:req
	.globl \name
\name:
	.macro endfn
		.size \name, . - \name
		.type \name, @function
		.purgem endfn
	.endm
.endm
"#;

/*
 * GNU assembler macro translated as assembly source text.
 *
 * The original #if BTI chooses either:
 *   GNU_PROPERTY_AARCH64_FEATURE_1_PAC | GNU_PROPERTY_AARCH64_FEATURE_1_BTI
 * or:
 *   0
 * Rust cannot determine that preprocessor symbol from this isolated file.
 */
pub const EMIT_AARCH64_FEATURE_1_AND_ASM: &str = r#"
.macro emit_aarch64_feature_1_and
	.pushsection .note.gnu.property, "a"
	.align	3
	.long	2f - 1f
	.long	6f - 3f
	.long	NT_GNU_PROPERTY_TYPE_0
1:	.string	"GNU"
2:
	.align	3
3:	.long	GNU_PROPERTY_AARCH64_FEATURE_1_AND
	.long	5f - 4f
4:
#if BTI
	.long	GNU_PROPERTY_AARCH64_FEATURE_1_PAC | \
		GNU_PROPERTY_AARCH64_FEATURE_1_BTI
#else
	.long	0
#endif
5:
	.align	3
6:
	.popsection
.endm
"#;

pub const PACIASP_ASM: &str = r#"
.macro paciasp
	hint	0x19
.endm
"#;

pub const AUTIASP_ASM: &str = r#"
.macro autiasp
	hint	0x1d
.endm
"#;

pub const __BTI__ASM: &str = r#"
.macro __bti_
	hint	0x20
.endm
"#;

pub const __BTI_C_ASM: &str = r#"
.macro __bti_c
	hint	0x22
.endm
"#;

pub const __BTI_J_ASM: &str = r#"
.macro __bti_j
	hint	0x24
.endm
"#;

pub const __BTI_JC_ASM: &str = r#"
.macro __bti_jc
	hint	0x26
.endm
"#;

pub const BTI_ASM: &str = r#"
.macro bti what=
	__bti_\what
.endm
"#;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
