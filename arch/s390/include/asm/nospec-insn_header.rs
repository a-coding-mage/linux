/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the s390 assembler-only nospec instruction header.
// The original file contains assembler preprocessor macros; their textual
// definitions are retained below because they have no direct Rust item form.

// Build-time condition: these definitions are active only for assembler
// sources (__ASSEMBLER__) and when CC_USING_EXPOLINE is enabled.
//
// .macro __THUNK_PROLOG_NAME name
// #ifdef CONFIG_EXPOLINE_EXTERN
// SYM_CODE_START(\name)
// #else
// .pushsection .text..\name,"axG",@progbits,\name,comdat
// .globl \name
// .hidden \name
// .type \name,@function
// \name:
// CFI_STARTPROC
// #endif
// .endm
//
// .macro __THUNK_EPILOG_NAME name
// #ifdef CONFIG_EXPOLINE_EXTERN
// SYM_CODE_END(\name)
// EXPORT_SYMBOL(\name)
// #else
// CFI_ENDPROC
// .popsection
// #endif
// .endm
//
// .macro __THUNK_PROLOG_BR r1
// __THUNK_PROLOG_NAME __s390_indirect_jump_r\r1
// .endm
// .macro __THUNK_EPILOG_BR r1
// __THUNK_EPILOG_NAME __s390_indirect_jump_r\r1
// .endm
// .macro __THUNK_BR r1
// jg __s390_indirect_jump_r\r1
// .endm
// .macro __THUNK_BRASL r1,r2
// brasl \r1,__s390_indirect_jump_r\r2
// .endm
//
// .macro __DECODE_R expand,reg
// .set .L__decode_fail,1
// .irp r1,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
// .ifc \reg,%r\r1
// \expand \r1
// .set .L__decode_fail,0
// .endif
// .endr
// .if .L__decode_fail == 1
// .error "__DECODE_R failed"
// .endif
// .endm
//
// .macro __DECODE_RR expand,rsave,rtarget
// .set .L__decode_fail,1
// .irp r1,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
// .ifc \rsave,%r\r1
// .irp r2,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
// .ifc \rtarget,%r\r2
// \expand \r1,\r2
// .set .L__decode_fail,0
// .endif
// .endr
// .endif
// .endr
// .if .L__decode_fail == 1
// .error "__DECODE_RR failed"
// .endif
// .endm
//
// .macro __THUNK_EX_BR reg
// exrl 0,555f
// j .
// 555: br \reg
// .endm
//
// With CC_USING_EXPOLINE, GEN_BR_THUNK and GEN_BR_THUNK_EXTERN expand to the
// decoded thunk prologue, indirect branch, and epilogue; GEN_BR_THUNK is empty
// under CONFIG_EXPOLINE_EXTERN. BR_EX and BASR_EX emit the corresponding
// indirect branch sequence and record its local address in
// .s390_indirect_branches.
//
// Without CC_USING_EXPOLINE:
// .macro GEN_BR_THUNK reg
// .endm
// .macro BR_EX reg
// br \reg
// .endm
// .macro BASR_EX rsave,rtarget
// basr \rsave,\rtarget
// .endm


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
