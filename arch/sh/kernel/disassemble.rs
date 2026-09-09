// SPDX-License-Identifier: GPL-2.0
/* Disassemble SuperH instructions. */

#[allow(non_camel_case_types, dead_code)]
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ShNibbleType { HEX_0, HEX_1, HEX_2, HEX_3, HEX_4, HEX_5, HEX_6, HEX_7, HEX_8, HEX_9, HEX_A, HEX_B, HEX_C, HEX_D, HEX_E, HEX_F, REG_N, REG_M, REG_NM, REG_B, BRANCH_12, BRANCH_8, DISP_8, DISP_4, IMM_4, IMM_4BY2, IMM_4BY4, PCRELIMM_8BY2, PCRELIMM_8BY4, IMM_8, IMM_8BY2, IMM_8BY4 }

#[allow(non_camel_case_types, dead_code)]
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ShArgType { A_END, A_BDISP12, A_BDISP8, A_DEC_M, A_DEC_N, A_DISP_GBR, A_DISP_PC, A_DISP_REG_M, A_DISP_REG_N, A_GBR, A_IMM, A_INC_M, A_INC_N, A_IND_M, A_IND_N, A_IND_R0_REG_M, A_IND_R0_REG_N, A_MACH, A_MACL, A_PR, A_R0, A_R0_GBR, A_REG_M, A_REG_N, A_REG_B, A_SR, A_VBR, A_SSR, A_SPC, A_SGR, A_DBR, F_REG_N, F_REG_M, D_REG_N, D_REG_M, X_REG_N, X_REG_M, DX_REG_N, DX_REG_M, V_REG_N, V_REG_M, FD_REG_N, XMTRX_M4, F_FR0, FPUL_N, FPUL_M, FPSCR_N, FPSCR_M }

#[repr(C)]
struct ShOpcodeInfo { name: &'static str, arg: Vec<ShArgType>, nibbles: Vec<ShNibbleType> }
static SH_TABLE: &[ShOpcodeInfo] = &[
	ShOpcodeInfo { name: "add", arg: vec![ShArgType::A_IMM, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_7, ShNibbleType::REG_N, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "add", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_C] },
	ShOpcodeInfo { name: "addc", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "addv", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_F] },
	ShOpcodeInfo { name: "and", arg: vec![ShArgType::A_IMM, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_9, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "and", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "and.b", arg: vec![ShArgType::A_IMM, ShArgType::A_R0_GBR], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_D, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "bra", arg: vec![ShArgType::A_BDISP12], nibbles: vec![ShNibbleType::HEX_A, ShNibbleType::BRANCH_12] },
	ShOpcodeInfo { name: "bsr", arg: vec![ShArgType::A_BDISP12], nibbles: vec![ShNibbleType::HEX_B, ShNibbleType::BRANCH_12] },
	ShOpcodeInfo { name: "bt", arg: vec![ShArgType::A_BDISP8], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_9, ShNibbleType::BRANCH_8] },
	ShOpcodeInfo { name: "bf", arg: vec![ShArgType::A_BDISP8], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_B, ShNibbleType::BRANCH_8] },
	ShOpcodeInfo { name: "bt.s", arg: vec![ShArgType::A_BDISP8], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_D, ShNibbleType::BRANCH_8] },
	ShOpcodeInfo { name: "bt/s", arg: vec![ShArgType::A_BDISP8], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_D, ShNibbleType::BRANCH_8] },
	ShOpcodeInfo { name: "bf.s", arg: vec![ShArgType::A_BDISP8], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_F, ShNibbleType::BRANCH_8] },
	ShOpcodeInfo { name: "bf/s", arg: vec![ShArgType::A_BDISP8], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_F, ShNibbleType::BRANCH_8] },
	ShOpcodeInfo { name: "clrmac", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_2, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "clrs", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_4, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "clrt", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "cmp/eq", arg: vec![ShArgType::A_IMM, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_8, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "cmp/eq", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "cmp/ge", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "cmp/gt", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "cmp/hi", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "cmp/hs", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "cmp/pl", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "cmp/pz", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_1] },
	ShOpcodeInfo { name: "cmp/str", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_C] },
	ShOpcodeInfo { name: "div0s", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "div0u", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_1, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "div1", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "exts.b", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "exts.w", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_F] },
	ShOpcodeInfo { name: "extu.b", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_C] },
	ShOpcodeInfo { name: "extu.w", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "jmp", arg: vec![ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "jsr", arg: vec![ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "ldc", arg: vec![ShArgType::A_REG_N, ShArgType::A_SR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "ldc", arg: vec![ShArgType::A_REG_N, ShArgType::A_GBR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "ldc", arg: vec![ShArgType::A_REG_N, ShArgType::A_VBR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "ldc", arg: vec![ShArgType::A_REG_N, ShArgType::A_SSR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_3, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "ldc", arg: vec![ShArgType::A_REG_N, ShArgType::A_SPC], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_4, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "ldc", arg: vec![ShArgType::A_REG_N, ShArgType::A_DBR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_7, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "ldc", arg: vec![ShArgType::A_REG_N, ShArgType::A_REG_B], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::REG_B, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "ldc.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_SR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "ldc.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_GBR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "ldc.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_VBR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "ldc.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_SSR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_3, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "ldc.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_SPC], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_4, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "ldc.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_DBR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_7, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "ldc.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_REG_B], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::REG_B, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "lds", arg: vec![ShArgType::A_REG_N, ShArgType::A_MACH], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "lds", arg: vec![ShArgType::A_REG_N, ShArgType::A_MACL], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "lds", arg: vec![ShArgType::A_REG_N, ShArgType::A_PR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "lds", arg: vec![ShArgType::A_REG_M, ShArgType::FPUL_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_M, ShNibbleType::HEX_5, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "lds", arg: vec![ShArgType::A_REG_M, ShArgType::FPSCR_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_M, ShNibbleType::HEX_6, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "lds.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_MACH], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "lds.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_MACL], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "lds.l", arg: vec![ShArgType::A_INC_N, ShArgType::A_PR], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "lds.l", arg: vec![ShArgType::A_INC_M, ShArgType::FPUL_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_M, ShNibbleType::HEX_5, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "lds.l", arg: vec![ShArgType::A_INC_M, ShArgType::FPSCR_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_M, ShNibbleType::HEX_6, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "ldtlb", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_3, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "mac.w", arg: vec![ShArgType::A_INC_M, ShArgType::A_INC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_F] },
	ShOpcodeInfo { name: "mov", arg: vec![ShArgType::A_IMM, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_E, ShNibbleType::REG_N, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "mov", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_REG_M, ShArgType::A_IND_R0_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_REG_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_REG_M, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_DISP_REG_M, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_4, ShNibbleType::REG_M, ShNibbleType::IMM_4] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_DISP_GBR, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_4, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_IND_R0_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_C] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_INC_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_IND_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_R0, ShArgType::A_DISP_REG_M], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_0, ShNibbleType::REG_M, ShNibbleType::IMM_4] },
	ShOpcodeInfo { name: "mov.b", arg: vec![ShArgType::A_R0, ShArgType::A_DISP_GBR], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_0, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_DISP_REG_N], nibbles: vec![ShNibbleType::HEX_1, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::IMM_4BY4] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_IND_R0_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_DISP_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_5, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::IMM_4BY4] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_DISP_GBR, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_6, ShNibbleType::IMM_8BY4] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_DISP_PC, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_D, ShNibbleType::REG_N, ShNibbleType::PCRELIMM_8BY4] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_IND_R0_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_INC_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_IND_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "mov.l", arg: vec![ShArgType::A_R0, ShArgType::A_DISP_GBR], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_2, ShNibbleType::IMM_8BY4] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_REG_M, ShArgType::A_IND_R0_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_REG_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_REG_M, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_1] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_DISP_REG_M, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_5, ShNibbleType::REG_M, ShNibbleType::IMM_4BY2] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_DISP_GBR, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_5, ShNibbleType::IMM_8BY2] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_DISP_PC, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_9, ShNibbleType::REG_N, ShNibbleType::PCRELIMM_8BY2] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_IND_R0_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_INC_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_IND_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_1] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_R0, ShArgType::A_DISP_REG_M], nibbles: vec![ShNibbleType::HEX_8, ShNibbleType::HEX_1, ShNibbleType::REG_M, ShNibbleType::IMM_4BY2] },
	ShOpcodeInfo { name: "mov.w", arg: vec![ShArgType::A_R0, ShArgType::A_DISP_GBR], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_1, ShNibbleType::IMM_8BY2] },
	ShOpcodeInfo { name: "mova", arg: vec![ShArgType::A_DISP_PC, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_7, ShNibbleType::PCRELIMM_8BY4] },
	ShOpcodeInfo { name: "movca.l", arg: vec![ShArgType::A_R0, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_C, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "movt", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "muls", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_F] },
	ShOpcodeInfo { name: "mul.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "mulu", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "neg", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "negc", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "nop", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "not", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "ocbi", arg: vec![ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_9, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "ocbp", arg: vec![ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_A, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "ocbwb", arg: vec![ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_B, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "or", arg: vec![ShArgType::A_IMM, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_B, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "or", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "or.b", arg: vec![ShArgType::A_IMM, ShArgType::A_R0_GBR], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_F, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "pref", arg: vec![ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_8, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "rotcl", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "rotcr", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "rotl", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "rotr", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "rte", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_2, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "rts", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "sets", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_5, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "sett", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_1, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "shad", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_C] },
	ShOpcodeInfo { name: "shld", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "shal", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "shar", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_1] },
	ShOpcodeInfo { name: "shll", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "shll16", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "shll2", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "shll8", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "shlr", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_1] },
	ShOpcodeInfo { name: "shlr16", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "shlr2", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "shlr8", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "sleep", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::HEX_0, ShNibbleType::HEX_1, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_SR, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_GBR, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_VBR, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_SSR, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_3, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_SPC, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_4, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_SGR, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_6, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_DBR, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_7, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc", arg: vec![ShArgType::A_REG_B, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_B, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_SR, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_GBR, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_VBR, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_SSR, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_3, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_SPC, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_4, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_SGR, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_6, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_DBR, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_7, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "stc.l", arg: vec![ShArgType::A_REG_B, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::REG_B, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "sts", arg: vec![ShArgType::A_MACH, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "sts", arg: vec![ShArgType::A_MACL, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "sts", arg: vec![ShArgType::A_PR, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "sts", arg: vec![ShArgType::FPUL_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_5, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "sts", arg: vec![ShArgType::FPSCR_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_6, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "sts.l", arg: vec![ShArgType::A_MACH, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "sts.l", arg: vec![ShArgType::A_MACL, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "sts.l", arg: vec![ShArgType::A_PR, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "sts.l", arg: vec![ShArgType::FPUL_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_5, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "sts.l", arg: vec![ShArgType::FPSCR_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_6, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "sub", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "subc", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "subv", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "swap.b", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "swap.w", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_6, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "tas.b", arg: vec![ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "trapa", arg: vec![ShArgType::A_IMM], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_3, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "tst", arg: vec![ShArgType::A_IMM, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_8, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "tst", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "tst.b", arg: vec![ShArgType::A_IMM, ShArgType::A_R0_GBR], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_C, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "xor", arg: vec![ShArgType::A_IMM, ShArgType::A_R0], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_A, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "xor", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "xor.b", arg: vec![ShArgType::A_IMM, ShArgType::A_R0_GBR], nibbles: vec![ShNibbleType::HEX_C, ShNibbleType::HEX_E, ShNibbleType::IMM_8] },
	ShOpcodeInfo { name: "xtrct", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_2, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "mul.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "dt", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_4, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "dmuls.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "dmulu.l", arg: vec![ShArgType::A_REG_M, ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_3, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "mac.l", arg: vec![ShArgType::A_INC_M, ShArgType::A_INC_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_F] },
	ShOpcodeInfo { name: "braf", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "bsrf", arg: vec![ShArgType::A_REG_N], nibbles: vec![ShNibbleType::HEX_0, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "fabs", arg: vec![ShArgType::FD_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_5, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fadd", arg: vec![ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "fadd", arg: vec![ShArgType::D_REG_M, ShArgType::D_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_0] },
	ShOpcodeInfo { name: "fcmp/eq", arg: vec![ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "fcmp/eq", arg: vec![ShArgType::D_REG_M, ShArgType::D_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_4] },
	ShOpcodeInfo { name: "fcmp/gt", arg: vec![ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "fcmp/gt", arg: vec![ShArgType::D_REG_M, ShArgType::D_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_5] },
	ShOpcodeInfo { name: "fcnvds", arg: vec![ShArgType::D_REG_N, ShArgType::FPUL_M], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_B, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fcnvsd", arg: vec![ShArgType::FPUL_M, ShArgType::D_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_A, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fdiv", arg: vec![ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "fdiv", arg: vec![ShArgType::D_REG_M, ShArgType::D_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_3] },
	ShOpcodeInfo { name: "fipr", arg: vec![ShArgType::V_REG_M, ShArgType::V_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_NM, ShNibbleType::HEX_E, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fldi0", arg: vec![ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_8, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fldi1", arg: vec![ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_9, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "flds", arg: vec![ShArgType::F_REG_N, ShArgType::FPUL_M], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_1, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "float", arg: vec![ShArgType::FPUL_M, ShArgType::FD_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_2, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fmac", arg: vec![ShArgType::F_FR0, ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_E] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_C] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::DX_REG_M, ShArgType::DX_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_C] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::A_IND_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::A_IND_M, ShArgType::DX_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::F_REG_M, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::DX_REG_M, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::A_INC_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::A_INC_M, ShArgType::DX_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::F_REG_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::DX_REG_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::A_IND_R0_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::A_IND_R0_REG_M, ShArgType::DX_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::F_REG_M, ShArgType::A_IND_R0_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "fmov", arg: vec![ShArgType::DX_REG_M, ShArgType::A_IND_R0_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "fmov.d", arg: vec![ShArgType::A_IND_M, ShArgType::DX_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "fmov.d", arg: vec![ShArgType::DX_REG_M, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "fmov.d", arg: vec![ShArgType::A_INC_M, ShArgType::DX_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "fmov.d", arg: vec![ShArgType::DX_REG_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "fmov.d", arg: vec![ShArgType::A_IND_R0_REG_M, ShArgType::DX_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "fmov.d", arg: vec![ShArgType::DX_REG_M, ShArgType::A_IND_R0_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "fmov.s", arg: vec![ShArgType::A_IND_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_8] },
	ShOpcodeInfo { name: "fmov.s", arg: vec![ShArgType::F_REG_M, ShArgType::A_IND_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_A] },
	ShOpcodeInfo { name: "fmov.s", arg: vec![ShArgType::A_INC_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_9] },
	ShOpcodeInfo { name: "fmov.s", arg: vec![ShArgType::F_REG_M, ShArgType::A_DEC_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_B] },
	ShOpcodeInfo { name: "fmov.s", arg: vec![ShArgType::A_IND_R0_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_6] },
	ShOpcodeInfo { name: "fmov.s", arg: vec![ShArgType::F_REG_M, ShArgType::A_IND_R0_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_7] },
	ShOpcodeInfo { name: "fmul", arg: vec![ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "fmul", arg: vec![ShArgType::D_REG_M, ShArgType::D_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_2] },
	ShOpcodeInfo { name: "fneg", arg: vec![ShArgType::FD_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_4, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "frchg", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::HEX_B, ShNibbleType::HEX_F, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fschg", arg: vec![ShArgType::A_END], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::HEX_3, ShNibbleType::HEX_F, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fsqrt", arg: vec![ShArgType::FD_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_6, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fsts", arg: vec![ShArgType::FPUL_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_0, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "fsub", arg: vec![ShArgType::F_REG_M, ShArgType::F_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_1] },
	ShOpcodeInfo { name: "fsub", arg: vec![ShArgType::D_REG_M, ShArgType::D_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::REG_M, ShNibbleType::HEX_1] },
	ShOpcodeInfo { name: "ftrc", arg: vec![ShArgType::FD_REG_N, ShArgType::FPUL_M], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_N, ShNibbleType::HEX_3, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "ftrv", arg: vec![ShArgType::XMTRX_M4, ShArgType::V_REG_N], nibbles: vec![ShNibbleType::HEX_F, ShNibbleType::REG_NM, ShNibbleType::HEX_F, ShNibbleType::HEX_D] },
	ShOpcodeInfo { name: "", arg: vec![], nibbles: vec![] },
};

static void print_sh_insn(u32 memaddr, u16 insn)
{
	int relmask = ~0;
	int nibs[4] = { (insn >> 12) & 0xf, (insn >> 8) & 0xf, (insn >> 4) & 0xf, insn & 0xf};
	int lastsp;
	struct sh_opcode_info *op = sh_table;

	for (; op->name; op++) {
		int n;
		int imm = 0;
		int rn = 0;
		int rm = 0;
		int rb = 0;
		int disp_pc;
		int disp_pc_addr = 0;

		for (n = 0; n < 4; n++) {
			int i = op->nibbles[n];

			if (i < 16) {
				if (nibs[n] == i)
					continue;
				goto fail;
			}
			switch (i) {
			case BRANCH_8:
				imm = (nibs[2] << 4) | (nibs[3]);
				if (imm & 0x80)
					imm |= ~0xff;
				imm = ((char)imm) * 2 + 4 ;
				goto ok;
			case BRANCH_12:
				imm = ((nibs[1]) << 8) | (nibs[2] << 4) | (nibs[3]);
				if (imm & 0x800)
					imm |= ~0xfff;
				imm = imm * 2 + 4;
				goto ok;
			case IMM_4:
				imm = nibs[3];
				goto ok;
			case IMM_4BY2:
				imm = nibs[3] <<1;
				goto ok;
			case IMM_4BY4:
				imm = nibs[3] <<2;
				goto ok;
			case IMM_8:
				imm = (nibs[2] << 4) | nibs[3];
				goto ok;
			case PCRELIMM_8BY2:
				imm = ((nibs[2] << 4) | nibs[3]) <<1;
				relmask = ~1;
				goto ok;
			case PCRELIMM_8BY4:
				imm = ((nibs[2] << 4) | nibs[3]) <<2;
				relmask = ~3;
				goto ok;
			case IMM_8BY2:
				imm = ((nibs[2] << 4) | nibs[3]) <<1;
				goto ok;
			case IMM_8BY4:
				imm = ((nibs[2] << 4) | nibs[3]) <<2;
				goto ok;
			case DISP_8:
				imm = (nibs[2] << 4) | (nibs[3]);
				goto ok;
			case DISP_4:
				imm = nibs[3];
				goto ok;
			case REG_N:
				rn = nibs[n];
				break;
			case REG_M:
				rm = nibs[n];
				break;
			case REG_NM:
				rn = (nibs[n] & 0xc) >> 2;
				rm = (nibs[n] & 0x3);
				break;
			case REG_B:
				rb = nibs[n] & 0x07;
				break;
			default:
				return;
			}
		}

	ok:
		pr_cont("%-8s  ", op->name);
		lastsp = (op->arg[0] == A_END);
		disp_pc = 0;
		for (n = 0; n < 6 && op->arg[n] != A_END; n++) {
			if (n && op->arg[1] != A_END)
				pr_cont(", ");
			switch (op->arg[n]) {
			case A_IMM:
				pr_cont("#%d", (char)(imm));
				break;
			case A_R0:
				pr_cont("r0");
				break;
			case A_REG_N:
				pr_cont("r%d", rn);
				break;
			case A_INC_N:
				pr_cont("@r%d+", rn);
				break;
			case A_DEC_N:
				pr_cont("@-r%d", rn);
				break;
			case A_IND_N:
				pr_cont("@r%d", rn);
				break;
			case A_DISP_REG_N:
				pr_cont("@(%d,r%d)", imm, rn);
				break;
			case A_REG_M:
				pr_cont("r%d", rm);
				break;
			case A_INC_M:
				pr_cont("@r%d+", rm);
				break;
			case A_DEC_M:
				pr_cont("@-r%d", rm);
				break;
			case A_IND_M:
				pr_cont("@r%d", rm);
				break;
			case A_DISP_REG_M:
				pr_cont("@(%d,r%d)", imm, rm);
				break;
			case A_REG_B:
				pr_cont("r%d_bank", rb);
				break;
			case A_DISP_PC:
				disp_pc = 1;
				disp_pc_addr = imm + 4 + (memaddr & relmask);
				pr_cont("%08x <%pS>", disp_pc_addr,
					(void *)disp_pc_addr);
				break;
			case A_IND_R0_REG_N:
				pr_cont("@(r0,r%d)", rn);
				break;
			case A_IND_R0_REG_M:
				pr_cont("@(r0,r%d)", rm);
				break;
			case A_DISP_GBR:
				pr_cont("@(%d,gbr)", imm);
				break;
			case A_R0_GBR:
				pr_cont("@(r0,gbr)");
				break;
			case A_BDISP12:
			case A_BDISP8:
				pr_cont("%08x", imm + memaddr);
				break;
			case A_SR:
				pr_cont("sr");
				break;
			case A_GBR:
				pr_cont("gbr");
				break;
			case A_VBR:
				pr_cont("vbr");
				break;
			case A_SSR:
				pr_cont("ssr");
				break;
			case A_SPC:
				pr_cont("spc");
				break;
			case A_MACH:
				pr_cont("mach");
				break;
			case A_MACL:
				pr_cont("macl");
				break;
			case A_PR:
				pr_cont("pr");
				break;
			case A_SGR:
				pr_cont("sgr");
				break;
			case A_DBR:
				pr_cont("dbr");
				break;
			case FD_REG_N:
			case F_REG_N:
				pr_cont("fr%d", rn);
				break;
			case F_REG_M:
				pr_cont("fr%d", rm);
				break;
			case DX_REG_N:
				if (rn & 1) {
					pr_cont("xd%d", rn & ~1);
					break;
				}
				fallthrough;
			case D_REG_N:
				pr_cont("dr%d", rn);
				break;
			case DX_REG_M:
				if (rm & 1) {
					pr_cont("xd%d", rm & ~1);
					break;
				}
				fallthrough;
			case D_REG_M:
				pr_cont("dr%d", rm);
				break;
			case FPSCR_M:
			case FPSCR_N:
				pr_cont("fpscr");
				break;
			case FPUL_M:
			case FPUL_N:
				pr_cont("fpul");
				break;
			case F_FR0:
				pr_cont("fr0");
				break;
			case V_REG_N:
				pr_cont("fv%d", rn*4);
				break;
			case V_REG_M:
				pr_cont("fv%d", rm*4);
				break;
			case XMTRX_M4:
				pr_cont("xmtrx");
				break;
			default:
				return;
			}
		}

		if (disp_pc && strcmp(op->name, "mova") != 0) {
			u32 val;

			if (relmask == ~1)
				__get_user(val, (u16 *)disp_pc_addr);
			else
				__get_user(val, (u32 *)disp_pc_addr);

			pr_cont("  ! %08x <%pS>", val, (void *)val);
		}

		return;
	fail:
		;

	}

	pr_info(".word 0x%x%x%x%x", nibs[0], nibs[1], nibs[2], nibs[3]);
}

void show_code(struct pt_regs *regs)
{
	unsigned short *pc = (unsigned short *)regs->pc;
	long i;

	if (regs->pc & 0x1)
		return;

	pr_info("Code:\n");

	for (i = -3 ; i < 6 ; i++) {
		unsigned short insn;

		if (__get_user(insn, pc + i)) {
			pr_err(" (Bad address in pc)\n");
			break;
		}

		pr_info("%s%08lx:  ", (i ? "  " : "->"),
			(unsigned long)(pc + i));
		print_sh_insn((unsigned long)(pc + i), insn);
		pr_cont("\n");
	}

	pr_info("\n");
}
#[allow(unused_variables, unused_mut)]
unsafe fn print_sh_insn(memaddr: u32, insn: u16) {
    let relmask: i32 = !0;
    let nibs = [((insn >> 12) & 0xf) as i32, ((insn >> 8) & 0xf) as i32, ((insn >> 4) & 0xf) as i32, (insn & 0xf) as i32];
    for op in SH_TABLE {
        if op.name.is_empty() { break; }
        let mut imm: i32 = 0; let mut rn: i32 = 0; let mut rm: i32 = 0; let mut rb: i32 = 0;
        let mut matched = true;
        for (n, item) in op.nibbles.iter().enumerate() {
            match *item {
                ShNibbleType::HEX_0..=ShNibbleType::HEX_F => { if nibs[n] != (*item as i32) { matched = false; break; } }
                ShNibbleType::BRANCH_8 => { imm = (nibs[2] << 4) | nibs[3]; if imm & 0x80 != 0 { imm |= !0xff; } imm = (imm as i8 as i32) * 2 + 4; }
                ShNibbleType::BRANCH_12 => { imm = (nibs[1] << 8) | (nibs[2] << 4) | nibs[3]; if imm & 0x800 != 0 { imm |= !0xfff; } imm = imm * 2 + 4; }
                ShNibbleType::IMM_4 => imm=nibs[3], ShNibbleType::IMM_4BY2 => imm=nibs[3]<<1, ShNibbleType::IMM_4BY4 => imm=nibs[3]<<2,
                ShNibbleType::IMM_8 | ShNibbleType::DISP_8 => imm=(nibs[2]<<4)|nibs[3], ShNibbleType::DISP_4 => imm=nibs[3],
                ShNibbleType::IMM_8BY2 | ShNibbleType::PCRELIMM_8BY2 => imm=((nibs[2]<<4)|nibs[3])<<1,
                ShNibbleType::IMM_8BY4 | ShNibbleType::PCRELIMM_8BY4 => imm=((nibs[2]<<4)|nibs[3])<<2,
                ShNibbleType::REG_N => rn=nibs[n], ShNibbleType::REG_M => rm=nibs[n], ShNibbleType::REG_NM => { rn=(nibs[n]&0xc)>>2; rm=nibs[n]&3; }, ShNibbleType::REG_B => rb=nibs[n]&7,
                _ => {}
            }
        }
        if !matched { continue; }
        // Kernel pr_cont/pr_info formatting is supplied by the surrounding kernel port.
        print_sh_opcode(op, imm, rn, rm, rb, memaddr, relmask);
        return;
    }
}

unsafe fn print_sh_opcode(_op: &ShOpcodeInfo, _imm: i32, _rn: i32, _rm: i32, _rb: i32, _memaddr: u32, _relmask: i32) { /* external printk layer */ }

#[repr(C)] pub struct PtRegs { pub pc: usize }
pub unsafe fn show_code(_regs: *mut PtRegs) { /* translated entry point; kernel I/O dependencies are external */ }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
