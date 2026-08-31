// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2015-2019 ARM Limited.
// Original author: Dave Martin <Dave.Martin@arm.com>

// Original header guard: ASSEMBLER_H

// GNU assembler macro, preserved for source-level intent:
//
// .macro __for from:req, to:req
// 	.if (\from) == (\to)
// 		_for__body %\from
// 	.else
// 		__for \from, %(\from) + ((\to) - (\from)) / 2
// 		__for %(\from) + ((\to) - (\from)) / 2 + 1, \to
// 	.endif
// .endm
//
// Rust has no direct file-local equivalent for recursive GNU assembler macros
// with altmacro argument substitution.

// GNU assembler macro, preserved for source-level intent:
//
// .macro _for var:req, from:req, to:req, insn:vararg
// 	.macro _for__body \var:req
// 		.noaltmacro
// 		\insn
// 		.altmacro
// 	.endm
//
// 	.altmacro
// 	__for \from, \to
// 	.noaltmacro
//
// 	.purgem _for__body
// .endm
//
// Rust has no direct file-local equivalent for this GNU assembler macro.

// GNU assembler macro, preserved for source-level intent:
//
// .macro function name
// 	.macro endfunction
// 		.type \name, @function
// 		.purgem endfunction
// 	.endm
// \name:
// .endm
//
// Rust has no direct file-local equivalent for defining assembler labels and
// temporary assembler macros in this form.

// GNU assembler macro, preserved for source-level intent:
//
// .macro define_accessor name, num, insn
// 	.macro \name\()_entry n
// 		\insn \n, 1
// 		ret
// 	.endm
//
// function \name
// 	adr	x2, .L__accessor_tbl\@
// 	add	x2, x2, x0, lsl #3
// 	br	x2
//
// .L__accessor_tbl\@:
// 	_for x, 0, (\num) - 1, \name\()_entry \x
// endfunction
//
// 	.purgem \name\()_entry
// .endm
//
// Rust has no direct file-local equivalent for this GNU assembler code
// generator. It emits an AArch64 branch table accessor function whose entries
// execute the supplied instruction and return.

// Utility macro to print a literal string
// Clobbers x0-x4,x8
//
// GNU assembler macro, preserved for source-level intent:
//
// .macro puts string
// 	.pushsection .rodata.str1.1, "aMS", @progbits, 1
// .L__puts_literal\@: .string "\string"
// 	.popsection
//
// 	ldr	x0, =.L__puts_literal\@
// 	bl	puts
// .endm
//
// Rust has no direct file-local equivalent for this GNU assembler macro. It
// creates a string literal in .rodata.str1.1, loads its address into x0, and
// branches to the external puts symbol.

pub const PR_SET_SHADOW_STACK_STATUS: u32 = 75;
pub const PR_SHADOW_STACK_ENABLE: u64 = 1u64 << 0;

// GNU assembler macro, preserved for source-level intent:
//
// .macro enable_gcs
// 	// Run with GCS
// 	mov	x0, PR_SET_SHADOW_STACK_STATUS
// 	mov	x1, PR_SHADOW_STACK_ENABLE
// 	mov	x2, xzr
// 	mov	x3, xzr
// 	mov	x4, xzr
// 	mov	x5, xzr
// 	mov	x8, #__NR_prctl
// 	svc	#0
// .endm
//
// Rust equivalent depends on the external __NR_prctl syscall number and the
// ability to emit AArch64 system-call assembly at the macro expansion site.
