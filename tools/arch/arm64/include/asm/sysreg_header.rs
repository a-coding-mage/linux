/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm64/include/asm/sysreg.h. */
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub const fn BIT(n: u64) -> u64 { 1u64 << n }
pub const fn BIT_ULL(n: u64) -> u64 { 1u64 << n }
pub const fn UL(x: u64) -> u64 { x }
pub const fn GENMASK(h: u64, l: u64) -> u64 { if h >= 63 { (!0u64) << l } else { ((1u64 << (h + 1)) - 1) & ((!0u64) << l) } }
pub const fn GENMASK_ULL(h: u64, l: u64) -> u64 { GENMASK(h, l) }
pub const fn FIELD_GET(mask: u64, val: u64) -> u64 { (val & mask) >> mask.trailing_zeros() }
pub const fn FIELD_PREP(mask: u64, val: u64) -> u64 { (val << mask.trailing_zeros()) & mask }

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Macros for accessing system registers with older binutils.
 *
 * Copyright (C) 2014 ARM Ltd.
 * Author: Catalin Marinas <catalin.marinas@arm.com>
 */

// C macro alias preserved: #define __ASM_SYSREG_H

// Original include: #include <linux/bits.h>
// Original include: #include <linux/stringify.h>
// Original include: #include <linux/kasan-tags.h>

// Original include: #include <asm/gpr-num.h>

/*
 * ARMv8 ARM reserves the following encoding for system registers:
 * (Ref: ARMv8 ARM, Section: "System instruction class encoding overview",
 *  C5.2, version:ARM DDI 0487A.f)
 *	[20-19] : Op0
 *	[18-16] : Op1
 *	[15-12] : CRn
 *	[11-8]  : CRm
 *	[7-5]   : Op2
 */
pub const Op0_shift: u64 = 19;
pub const Op0_mask: u64 = 0x3;
pub const Op1_shift: u64 = 16;
pub const Op1_mask: u64 = 0x7;
pub const CRn_shift: u64 = 12;
pub const CRn_mask: u64 = 0xf;
pub const CRm_shift: u64 = 8;
pub const CRm_mask: u64 = 0xf;
pub const Op2_shift: u64 = 5;
pub const Op2_mask: u64 = 0x7;

pub const fn sys_reg(op0: u64, op1: u64, crn: u64, crm: u64, op2: u64) -> u64 { (((op0) << Op0_shift) | ((op1) << Op1_shift) |  	 ((crn) << CRn_shift) | ((crm) << CRm_shift) |  	 ((op2) << Op2_shift)) }

pub const sys_insn: u64 = sys_reg;

pub const fn sys_reg_Op0(id: u64) -> u64 { (((id) >> Op0_shift) & Op0_mask) }
pub const fn sys_reg_Op1(id: u64) -> u64 { (((id) >> Op1_shift) & Op1_mask) }
pub const fn sys_reg_CRn(id: u64) -> u64 { (((id) >> CRn_shift) & CRn_mask) }
pub const fn sys_reg_CRm(id: u64) -> u64 { (((id) >> CRm_shift) & CRm_mask) }
pub const fn sys_reg_Op2(id: u64) -> u64 { (((id) >> Op2_shift) & Op2_mask) }

// Preprocessor condition preserved from C: #ifndef CONFIG_BROKEN_GAS_INST

// Preprocessor condition preserved from C: #ifdef __ASSEMBLER__
// The space separator is omitted so that __emit_inst(x) can be parsed as
// either an assembler directive or an assembler macro argument.
pub const fn __emit_inst(x: u64) -> u64 { .inst(x) }
// Preprocessor condition preserved from C: #else
// C macro requiring assembler/preprocessor context preserved: #define __emit_inst(x)			".inst " __stringify((x)) "\n\t"
// Preprocessor condition preserved from C: #endif

// Preprocessor condition preserved from C: #else  /* CONFIG_BROKEN_GAS_INST */

// Preprocessor condition preserved from C: #ifndef CONFIG_CPU_BIG_ENDIAN
pub const fn __INSTR_BSWAP(x: u64) -> u64 { (x) }
// Preprocessor condition preserved from C: #else  /* CONFIG_CPU_BIG_ENDIAN */
pub const fn __INSTR_BSWAP(x: u64) -> u64 { ((((x) << 24) & 0xff000000)	|  					 (((x) <<  8) & 0x00ff0000)	|  					 (((x) >>  8) & 0x0000ff00)	|  					 (((x) >> 24) & 0x000000ff)) }
// Preprocessor condition preserved from C: #endif	/* CONFIG_CPU_BIG_ENDIAN */

// Preprocessor condition preserved from C: #ifdef __ASSEMBLER__
pub const fn __emit_inst(x: u64) -> u64 { .long __INSTR_BSWAP(x) }
// Preprocessor condition preserved from C: #else  /* __ASSEMBLER__ */
// C macro requiring assembler/preprocessor context preserved: #define __emit_inst(x)			".long " __stringify(__INSTR_BSWAP(x)) "\n\t"
// Preprocessor condition preserved from C: #endif	/* __ASSEMBLER__ */

// Preprocessor condition preserved from C: #endif	/* CONFIG_BROKEN_GAS_INST */

/*
 * Instructions for modifying PSTATE fields.
 * As per Arm ARM for v8-A, Section "C.5.1.3 op0 == 0b00, architectural hints,
 * barriers and CLREX, and PSTATE access", ARM DDI 0487 C.a, system instructions
 * for accessing PSTATE fields have the following encoding:
 *	Op0 = 0, CRn = 4
 *	Op1, Op2 encodes the PSTATE field modified and defines the constraints.
 *	CRm = Imm4 for the instruction.
 *	Rt = 0x1f
 */
pub const fn pstate_field(op1: u64, op2: u64) -> u64 { ((op1) << Op1_shift | (op2) << Op2_shift) }
pub const PSTATE_Imm_shift: u64 = CRm_shift;
// C macro requiring assembler/preprocessor context preserved: #define SET_PSTATE(x, r)		__emit_inst(0xd500401f | PSTATE_ ## r | ((!!x) << PSTATE_Imm_shift))

pub const PSTATE_PAN: u64 = pstate_field(0, 4);
pub const PSTATE_UAO: u64 = pstate_field(0, 3);
pub const PSTATE_SSBS: u64 = pstate_field(3, 1);
pub const PSTATE_DIT: u64 = pstate_field(3, 2);
pub const PSTATE_TCO: u64 = pstate_field(3, 4);

pub const fn SET_PSTATE_PAN(x: u64) -> u64 { SET_PSTATE((x), PAN) }
pub const fn SET_PSTATE_UAO(x: u64) -> u64 { SET_PSTATE((x), UAO) }
pub const fn SET_PSTATE_SSBS(x: u64) -> u64 { SET_PSTATE((x), SSBS) }
pub const fn SET_PSTATE_DIT(x: u64) -> u64 { SET_PSTATE((x), DIT) }
pub const fn SET_PSTATE_TCO(x: u64) -> u64 { SET_PSTATE((x), TCO) }

// C macro requiring assembler/preprocessor context preserved: #define set_pstate_pan(x)		asm volatile(SET_PSTATE_PAN(x))
// C macro requiring assembler/preprocessor context preserved: #define set_pstate_uao(x)		asm volatile(SET_PSTATE_UAO(x))
// C macro requiring assembler/preprocessor context preserved: #define set_pstate_ssbs(x)		asm volatile(SET_PSTATE_SSBS(x))
// C macro requiring assembler/preprocessor context preserved: #define set_pstate_dit(x)		asm volatile(SET_PSTATE_DIT(x))

/* Register-based PAN access, for save/restore purposes */
pub const SYS_PSTATE_PAN: u64 = sys_reg(3, 0, 4, 2, 3);

// C macro requiring assembler/preprocessor context preserved: #define __SYS_BARRIER_INSN(CRm, op2, Rt)  	__emit_inst(0xd5000000 | sys_insn(0, 3, 3, (CRm), (op2)) | ((Rt) & 0x1f))

pub const SB_BARRIER_INSN: u64 = __SYS_BARRIER_INSN(0, 7, 31);

/* Data cache zero operations */
pub const SYS_DC_ISW: u64 = sys_insn(1, 0, 7, 6, 2);
pub const SYS_DC_IGSW: u64 = sys_insn(1, 0, 7, 6, 4);
pub const SYS_DC_IGDSW: u64 = sys_insn(1, 0, 7, 6, 6);
pub const SYS_DC_CSW: u64 = sys_insn(1, 0, 7, 10, 2);
pub const SYS_DC_CGSW: u64 = sys_insn(1, 0, 7, 10, 4);
pub const SYS_DC_CGDSW: u64 = sys_insn(1, 0, 7, 10, 6);
pub const SYS_DC_CISW: u64 = sys_insn(1, 0, 7, 14, 2);
pub const SYS_DC_CIGSW: u64 = sys_insn(1, 0, 7, 14, 4);
pub const SYS_DC_CIGDSW: u64 = sys_insn(1, 0, 7, 14, 6);

pub const SYS_IC_IALLUIS: u64 = sys_insn(1, 0, 7, 1, 0);
pub const SYS_IC_IALLU: u64 = sys_insn(1, 0, 7, 5, 0);
pub const SYS_IC_IVAU: u64 = sys_insn(1, 3, 7, 5, 1);

pub const SYS_DC_IVAC: u64 = sys_insn(1, 0, 7, 6, 1);
pub const SYS_DC_IGVAC: u64 = sys_insn(1, 0, 7, 6, 3);
pub const SYS_DC_IGDVAC: u64 = sys_insn(1, 0, 7, 6, 5);

pub const SYS_DC_CVAC: u64 = sys_insn(1, 3, 7, 10, 1);
pub const SYS_DC_CGVAC: u64 = sys_insn(1, 3, 7, 10, 3);
pub const SYS_DC_CGDVAC: u64 = sys_insn(1, 3, 7, 10, 5);

pub const SYS_DC_CVAU: u64 = sys_insn(1, 3, 7, 11, 1);

pub const SYS_DC_CVAP: u64 = sys_insn(1, 3, 7, 12, 1);
pub const SYS_DC_CGVAP: u64 = sys_insn(1, 3, 7, 12, 3);
pub const SYS_DC_CGDVAP: u64 = sys_insn(1, 3, 7, 12, 5);

pub const SYS_DC_CVADP: u64 = sys_insn(1, 3, 7, 13, 1);
pub const SYS_DC_CGVADP: u64 = sys_insn(1, 3, 7, 13, 3);
pub const SYS_DC_CGDVADP: u64 = sys_insn(1, 3, 7, 13, 5);

pub const SYS_DC_CIVAC: u64 = sys_insn(1, 3, 7, 14, 1);
pub const SYS_DC_CIGVAC: u64 = sys_insn(1, 3, 7, 14, 3);
pub const SYS_DC_CIGDVAC: u64 = sys_insn(1, 3, 7, 14, 5);

pub const SYS_DC_ZVA: u64 = sys_insn(1, 3, 7, 4, 1);
pub const SYS_DC_GVA: u64 = sys_insn(1, 3, 7, 4, 3);
pub const SYS_DC_GZVA: u64 = sys_insn(1, 3, 7, 4, 4);

pub const SYS_DC_CIVAPS: u64 = sys_insn(1, 0, 7, 15, 1);
pub const SYS_DC_CIGDVAPS: u64 = sys_insn(1, 0, 7, 15, 5);

/*
 * Automatically generated definitions for system registers, the
 * manual encodings below are in the process of being converted to
 * come from here. The header relies on the definition of sys_reg()
 * earlier in this file.
 */
// Original include: #include "asm/sysreg-defs.h"

/*
 * System registers, organised loosely by encoding but grouped together
 * where the architected name contains an index. e.g. ID_MMFR<n>_EL1.
 */
pub const SYS_SVCR_SMSTOP_SM_EL0: u64 = sys_reg(0, 3, 4, 2, 3);
pub const SYS_SVCR_SMSTART_SM_EL0: u64 = sys_reg(0, 3, 4, 3, 3);
pub const SYS_SVCR_SMSTOP_SMZA_EL0: u64 = sys_reg(0, 3, 4, 6, 3);

pub const fn SYS_DBGBVRn_EL1(n: u64) -> u64 { sys_reg(2, 0, 0, n, 4) }
pub const fn SYS_DBGBCRn_EL1(n: u64) -> u64 { sys_reg(2, 0, 0, n, 5) }
pub const fn SYS_DBGWVRn_EL1(n: u64) -> u64 { sys_reg(2, 0, 0, n, 6) }
pub const fn SYS_DBGWCRn_EL1(n: u64) -> u64 { sys_reg(2, 0, 0, n, 7) }
pub const SYS_MDRAR_EL1: u64 = sys_reg(2, 0, 1, 0, 0);

pub const SYS_OSLSR_EL1: u64 = sys_reg(2, 0, 1, 1, 4);
pub const OSLSR_EL1_OSLM_MASK: u64 = (BIT(3) | BIT(0));
pub const OSLSR_EL1_OSLM_NI: u64 = 0;
pub const OSLSR_EL1_OSLM_IMPLEMENTED: u64 = BIT(3);
pub const OSLSR_EL1_OSLK: u64 = BIT(1);

pub const SYS_OSDLR_EL1: u64 = sys_reg(2, 0, 1, 3, 4);
pub const SYS_DBGPRCR_EL1: u64 = sys_reg(2, 0, 1, 4, 4);
pub const SYS_DBGCLAIMSET_EL1: u64 = sys_reg(2, 0, 7, 8, 6);
pub const SYS_DBGCLAIMCLR_EL1: u64 = sys_reg(2, 0, 7, 9, 6);
pub const SYS_DBGAUTHSTATUS_EL1: u64 = sys_reg(2, 0, 7, 14, 6);
pub const SYS_MDCCSR_EL0: u64 = sys_reg(2, 3, 0, 1, 0);
pub const SYS_DBGDTR_EL0: u64 = sys_reg(2, 3, 0, 4, 0);
pub const SYS_DBGDTRRX_EL0: u64 = sys_reg(2, 3, 0, 5, 0);
pub const SYS_DBGDTRTX_EL0: u64 = sys_reg(2, 3, 0, 5, 0);
pub const SYS_DBGVCR32_EL2: u64 = sys_reg(2, 4, 0, 7, 0);

pub const fn SYS_BRBINF_EL1(n: u64) -> u64 { sys_reg(2, 1, 8, (n & 15), (((n & 16) >> 2) | 0)) }
pub const SYS_BRBINFINJ_EL1: u64 = sys_reg(2, 1, 9, 1, 0);
pub const fn SYS_BRBSRC_EL1(n: u64) -> u64 { sys_reg(2, 1, 8, (n & 15), (((n & 16) >> 2) | 1)) }
pub const SYS_BRBSRCINJ_EL1: u64 = sys_reg(2, 1, 9, 1, 1);
pub const fn SYS_BRBTGT_EL1(n: u64) -> u64 { sys_reg(2, 1, 8, (n & 15), (((n & 16) >> 2) | 2)) }
pub const SYS_BRBTGTINJ_EL1: u64 = sys_reg(2, 1, 9, 1, 2);
pub const SYS_BRBTS_EL1: u64 = sys_reg(2, 1, 9, 0, 2);

pub const SYS_BRBCR_EL1: u64 = sys_reg(2, 1, 9, 0, 0);
pub const SYS_BRBFCR_EL1: u64 = sys_reg(2, 1, 9, 0, 1);
pub const SYS_BRBIDR0_EL1: u64 = sys_reg(2, 1, 9, 2, 0);

pub const SYS_TRCITECR_EL1: u64 = sys_reg(3, 0, 1, 2, 3);
pub const fn SYS_TRCACATR(m: u64) -> u64 { sys_reg(2, 1, 2, ((m & 7) << 1), (2 | (m >> 3))) }
pub const fn SYS_TRCACVR(m: u64) -> u64 { sys_reg(2, 1, 2, ((m & 7) << 1), (0 | (m >> 3))) }
pub const SYS_TRCAUTHSTATUS: u64 = sys_reg(2, 1, 7, 14, 6);
pub const SYS_TRCAUXCTLR: u64 = sys_reg(2, 1, 0, 6, 0);
pub const SYS_TRCBBCTLR: u64 = sys_reg(2, 1, 0, 15, 0);
pub const SYS_TRCCCCTLR: u64 = sys_reg(2, 1, 0, 14, 0);
pub const SYS_TRCCIDCCTLR0: u64 = sys_reg(2, 1, 3, 0, 2);
pub const SYS_TRCCIDCCTLR1: u64 = sys_reg(2, 1, 3, 1, 2);
pub const fn SYS_TRCCIDCVR(m: u64) -> u64 { sys_reg(2, 1, 3, ((m & 7) << 1), 0) }
pub const SYS_TRCCLAIMCLR: u64 = sys_reg(2, 1, 7, 9, 6);
pub const SYS_TRCCLAIMSET: u64 = sys_reg(2, 1, 7, 8, 6);
pub const fn SYS_TRCCNTCTLR(m: u64) -> u64 { sys_reg(2, 1, 0, (4 | (m & 3)), 5) }
pub const fn SYS_TRCCNTRLDVR(m: u64) -> u64 { sys_reg(2, 1, 0, (0 | (m & 3)), 5) }
pub const fn SYS_TRCCNTVR(m: u64) -> u64 { sys_reg(2, 1, 0, (8 | (m & 3)), 5) }
pub const SYS_TRCCONFIGR: u64 = sys_reg(2, 1, 0, 4, 0);
pub const SYS_TRCDEVARCH: u64 = sys_reg(2, 1, 7, 15, 6);
pub const SYS_TRCDEVID: u64 = sys_reg(2, 1, 7, 2, 7);
pub const SYS_TRCEVENTCTL0R: u64 = sys_reg(2, 1, 0, 8, 0);
pub const SYS_TRCEVENTCTL1R: u64 = sys_reg(2, 1, 0, 9, 0);
pub const fn SYS_TRCEXTINSELR(m: u64) -> u64 { sys_reg(2, 1, 0, (8 | (m & 3)), 4) }
pub const SYS_TRCIDR0: u64 = sys_reg(2, 1, 0, 8, 7);
pub const SYS_TRCIDR10: u64 = sys_reg(2, 1, 0, 2, 6);
pub const SYS_TRCIDR11: u64 = sys_reg(2, 1, 0, 3, 6);
pub const SYS_TRCIDR12: u64 = sys_reg(2, 1, 0, 4, 6);
pub const SYS_TRCIDR13: u64 = sys_reg(2, 1, 0, 5, 6);
pub const SYS_TRCIDR1: u64 = sys_reg(2, 1, 0, 9, 7);
pub const SYS_TRCIDR2: u64 = sys_reg(2, 1, 0, 10, 7);
pub const SYS_TRCIDR3: u64 = sys_reg(2, 1, 0, 11, 7);
pub const SYS_TRCIDR4: u64 = sys_reg(2, 1, 0, 12, 7);
pub const SYS_TRCIDR5: u64 = sys_reg(2, 1, 0, 13, 7);
pub const SYS_TRCIDR6: u64 = sys_reg(2, 1, 0, 14, 7);
pub const SYS_TRCIDR7: u64 = sys_reg(2, 1, 0, 15, 7);
pub const SYS_TRCIDR8: u64 = sys_reg(2, 1, 0, 0, 6);
pub const SYS_TRCIDR9: u64 = sys_reg(2, 1, 0, 1, 6);
pub const fn SYS_TRCIMSPEC(m: u64) -> u64 { sys_reg(2, 1, 0, (m & 7), 7) }
pub const SYS_TRCITEEDCR: u64 = sys_reg(2, 1, 0, 2, 1);
pub const SYS_TRCOSLSR: u64 = sys_reg(2, 1, 1, 1, 4);
pub const SYS_TRCPRGCTLR: u64 = sys_reg(2, 1, 0, 1, 0);
pub const SYS_TRCQCTLR: u64 = sys_reg(2, 1, 0, 1, 1);
pub const fn SYS_TRCRSCTLR(m: u64) -> u64 { sys_reg(2, 1, 1, (m & 15), (0 | (m >> 4))) }
pub const SYS_TRCRSR: u64 = sys_reg(2, 1, 0, 10, 0);
pub const fn SYS_TRCSEQEVR(m: u64) -> u64 { sys_reg(2, 1, 0, (m & 3), 4) }
pub const SYS_TRCSEQRSTEVR: u64 = sys_reg(2, 1, 0, 6, 4);
pub const SYS_TRCSEQSTR: u64 = sys_reg(2, 1, 0, 7, 4);
pub const fn SYS_TRCSSCCR(m: u64) -> u64 { sys_reg(2, 1, 1, (m & 7), 2) }
pub const fn SYS_TRCSSCSR(m: u64) -> u64 { sys_reg(2, 1, 1, (8 | (m & 7)), 2) }
pub const fn SYS_TRCSSPCICR(m: u64) -> u64 { sys_reg(2, 1, 1, (m & 7), 3) }
pub const SYS_TRCSTALLCTLR: u64 = sys_reg(2, 1, 0, 11, 0);
pub const SYS_TRCSTATR: u64 = sys_reg(2, 1, 0, 3, 0);
pub const SYS_TRCSYNCPR: u64 = sys_reg(2, 1, 0, 13, 0);
pub const SYS_TRCTRACEIDR: u64 = sys_reg(2, 1, 0, 0, 1);
pub const SYS_TRCTSCTLR: u64 = sys_reg(2, 1, 0, 12, 0);
pub const SYS_TRCVICTLR: u64 = sys_reg(2, 1, 0, 0, 2);
pub const SYS_TRCVIIECTLR: u64 = sys_reg(2, 1, 0, 1, 2);
pub const SYS_TRCVIPCSSCTLR: u64 = sys_reg(2, 1, 0, 3, 2);
pub const SYS_TRCVISSCTLR: u64 = sys_reg(2, 1, 0, 2, 2);
pub const SYS_TRCVMIDCCTLR0: u64 = sys_reg(2, 1, 3, 2, 2);
pub const SYS_TRCVMIDCCTLR1: u64 = sys_reg(2, 1, 3, 3, 2);
pub const fn SYS_TRCVMIDCVR(m: u64) -> u64 { sys_reg(2, 1, 3, ((m & 7) << 1), 1) }

/* ETM */
pub const SYS_TRCOSLAR: u64 = sys_reg(2, 1, 1, 0, 4);

pub const SYS_BRBCR_EL2: u64 = sys_reg(2, 4, 9, 0, 0);

pub const SYS_MIDR_EL1: u64 = sys_reg(3, 0, 0, 0, 0);
pub const SYS_MPIDR_EL1: u64 = sys_reg(3, 0, 0, 0, 5);
pub const SYS_REVIDR_EL1: u64 = sys_reg(3, 0, 0, 0, 6);

pub const SYS_ACTLR_EL1: u64 = sys_reg(3, 0, 1, 0, 1);
pub const SYS_RGSR_EL1: u64 = sys_reg(3, 0, 1, 0, 5);
pub const SYS_GCR_EL1: u64 = sys_reg(3, 0, 1, 0, 6);

pub const SYS_TCR_EL1: u64 = sys_reg(3, 0, 2, 0, 2);

pub const SYS_APIAKEYLO_EL1: u64 = sys_reg(3, 0, 2, 1, 0);
pub const SYS_APIAKEYHI_EL1: u64 = sys_reg(3, 0, 2, 1, 1);
pub const SYS_APIBKEYLO_EL1: u64 = sys_reg(3, 0, 2, 1, 2);
pub const SYS_APIBKEYHI_EL1: u64 = sys_reg(3, 0, 2, 1, 3);

pub const SYS_APDAKEYLO_EL1: u64 = sys_reg(3, 0, 2, 2, 0);
pub const SYS_APDAKEYHI_EL1: u64 = sys_reg(3, 0, 2, 2, 1);
pub const SYS_APDBKEYLO_EL1: u64 = sys_reg(3, 0, 2, 2, 2);
pub const SYS_APDBKEYHI_EL1: u64 = sys_reg(3, 0, 2, 2, 3);

pub const SYS_APGAKEYLO_EL1: u64 = sys_reg(3, 0, 2, 3, 0);
pub const SYS_APGAKEYHI_EL1: u64 = sys_reg(3, 0, 2, 3, 1);

pub const SYS_SPSR_EL1: u64 = sys_reg(3, 0, 4, 0, 0);
pub const SYS_ELR_EL1: u64 = sys_reg(3, 0, 4, 0, 1);

pub const SYS_ICC_PMR_EL1: u64 = sys_reg(3, 0, 4, 6, 0);

pub const SYS_AFSR0_EL1: u64 = sys_reg(3, 0, 5, 1, 0);
pub const SYS_AFSR1_EL1: u64 = sys_reg(3, 0, 5, 1, 1);
pub const SYS_ESR_EL1: u64 = sys_reg(3, 0, 5, 2, 0);

pub const SYS_ERRIDR_EL1: u64 = sys_reg(3, 0, 5, 3, 0);
pub const SYS_ERRSELR_EL1: u64 = sys_reg(3, 0, 5, 3, 1);
pub const SYS_ERXFR_EL1: u64 = sys_reg(3, 0, 5, 4, 0);
pub const SYS_ERXCTLR_EL1: u64 = sys_reg(3, 0, 5, 4, 1);
pub const SYS_ERXSTATUS_EL1: u64 = sys_reg(3, 0, 5, 4, 2);
pub const SYS_ERXADDR_EL1: u64 = sys_reg(3, 0, 5, 4, 3);
pub const SYS_ERXPFGF_EL1: u64 = sys_reg(3, 0, 5, 4, 4);
pub const SYS_ERXPFGCTL_EL1: u64 = sys_reg(3, 0, 5, 4, 5);
pub const SYS_ERXPFGCDN_EL1: u64 = sys_reg(3, 0, 5, 4, 6);
pub const SYS_ERXMISC0_EL1: u64 = sys_reg(3, 0, 5, 5, 0);
pub const SYS_ERXMISC1_EL1: u64 = sys_reg(3, 0, 5, 5, 1);
pub const SYS_ERXMISC2_EL1: u64 = sys_reg(3, 0, 5, 5, 2);
pub const SYS_ERXMISC3_EL1: u64 = sys_reg(3, 0, 5, 5, 3);
pub const SYS_TFSR_EL1: u64 = sys_reg(3, 0, 5, 6, 0);
pub const SYS_TFSRE0_EL1: u64 = sys_reg(3, 0, 5, 6, 1);

pub const SYS_PAR_EL1: u64 = sys_reg(3, 0, 7, 4, 0);

pub const SYS_PAR_EL1_F: u64 = BIT(0);
/* When PAR_EL1.F == 1 */
pub const SYS_PAR_EL1_FST: u64 = GENMASK(6, 1);
pub const SYS_PAR_EL1_PTW: u64 = BIT(8);
pub const SYS_PAR_EL1_S: u64 = BIT(9);
pub const SYS_PAR_EL1_AssuredOnly: u64 = BIT(12);
pub const SYS_PAR_EL1_TopLevel: u64 = BIT(13);
pub const SYS_PAR_EL1_Overlay: u64 = BIT(14);
pub const SYS_PAR_EL1_DirtyBit: u64 = BIT(15);
pub const SYS_PAR_EL1_F1_IMPDEF: u64 = GENMASK_ULL(63, 48);
pub const SYS_PAR_EL1_F1_RES0: u64 = (BIT(7) | BIT(10) | GENMASK_ULL(47, 16));
pub const SYS_PAR_EL1_RES1: u64 = BIT(11);
/* When PAR_EL1.F == 0 */
pub const SYS_PAR_EL1_SH: u64 = GENMASK_ULL(8, 7);
pub const SYS_PAR_EL1_NS: u64 = BIT(9);
pub const SYS_PAR_EL1_F0_IMPDEF: u64 = BIT(10);
pub const SYS_PAR_EL1_NSE: u64 = BIT(11);
pub const SYS_PAR_EL1_PA: u64 = GENMASK_ULL(51, 12);
pub const SYS_PAR_EL1_ATTR: u64 = GENMASK_ULL(63, 56);
pub const SYS_PAR_EL1_F0_RES0: u64 = (GENMASK_ULL(6, 1) | GENMASK_ULL(55, 52));

/*** Statistical Profiling Extension ***/
pub const PMSEVFR_EL1_RES0_IMP: u64 = (GENMASK_ULL(47, 32) | GENMASK_ULL(23, 16) | GENMASK_ULL(11, 8) | 	 BIT_ULL(6) | BIT_ULL(4) | BIT_ULL(2) | BIT_ULL(0));
pub const PMSEVFR_EL1_RES0_V1P1: u64 = (PMSEVFR_EL1_RES0_IMP & ~(BIT_ULL(18) | BIT_ULL(17) | BIT_ULL(11)));
pub const PMSEVFR_EL1_RES0_V1P2: u64 = (PMSEVFR_EL1_RES0_V1P1 & ~BIT_ULL(6));

/* Buffer error reporting */
pub const PMBSR_EL1_FAULT_FSC_SHIFT: u64 = PMBSR_EL1_MSS_SHIFT;
pub const PMBSR_EL1_FAULT_FSC_MASK: u64 = PMBSR_EL1_MSS_MASK;

pub const PMBSR_EL1_BUF_BSC_SHIFT: u64 = PMBSR_EL1_MSS_SHIFT;
pub const PMBSR_EL1_BUF_BSC_MASK: u64 = PMBSR_EL1_MSS_MASK;

pub const PMBSR_EL1_BUF_BSC_FULL: u64 = u64;

/*** End of Statistical Profiling Extension ***/

pub const TRBSR_EL1_BSC_MASK: u64 = GENMASK(5, 0);
pub const TRBSR_EL1_BSC_SHIFT: u64 = 0;

pub const SYS_PMINTENSET_EL1: u64 = sys_reg(3, 0, 9, 14, 1);
pub const SYS_PMINTENCLR_EL1: u64 = sys_reg(3, 0, 9, 14, 2);

pub const SYS_PMMIR_EL1: u64 = sys_reg(3, 0, 9, 14, 6);

pub const SYS_MAIR_EL1: u64 = sys_reg(3, 0, 10, 2, 0);
pub const SYS_AMAIR_EL1: u64 = sys_reg(3, 0, 10, 3, 0);

pub const SYS_VBAR_EL1: u64 = sys_reg(3, 0, 12, 0, 0);
pub const SYS_DISR_EL1: u64 = sys_reg(3, 0, 12, 1, 1);

pub const SYS_ICC_IAR0_EL1: u64 = sys_reg(3, 0, 12, 8, 0);
pub const SYS_ICC_EOIR0_EL1: u64 = sys_reg(3, 0, 12, 8, 1);
pub const SYS_ICC_HPPIR0_EL1: u64 = sys_reg(3, 0, 12, 8, 2);
pub const SYS_ICC_BPR0_EL1: u64 = sys_reg(3, 0, 12, 8, 3);
pub const fn SYS_ICC_AP0Rn_EL1(n: u64) -> u64 { sys_reg(3, 0, 12, 8, 4 | n) }
pub const SYS_ICC_AP0R0_EL1: u64 = SYS_ICC_AP0Rn_EL1(0);
pub const SYS_ICC_AP0R1_EL1: u64 = SYS_ICC_AP0Rn_EL1(1);
pub const SYS_ICC_AP0R2_EL1: u64 = SYS_ICC_AP0Rn_EL1(2);
pub const SYS_ICC_AP0R3_EL1: u64 = SYS_ICC_AP0Rn_EL1(3);
pub const fn SYS_ICC_AP1Rn_EL1(n: u64) -> u64 { sys_reg(3, 0, 12, 9, n) }
pub const SYS_ICC_AP1R0_EL1: u64 = SYS_ICC_AP1Rn_EL1(0);
pub const SYS_ICC_AP1R1_EL1: u64 = SYS_ICC_AP1Rn_EL1(1);
pub const SYS_ICC_AP1R2_EL1: u64 = SYS_ICC_AP1Rn_EL1(2);
pub const SYS_ICC_AP1R3_EL1: u64 = SYS_ICC_AP1Rn_EL1(3);
pub const SYS_ICC_DIR_EL1: u64 = sys_reg(3, 0, 12, 11, 1);
pub const SYS_ICC_RPR_EL1: u64 = sys_reg(3, 0, 12, 11, 3);
pub const SYS_ICC_SGI1R_EL1: u64 = sys_reg(3, 0, 12, 11, 5);
pub const SYS_ICC_ASGI1R_EL1: u64 = sys_reg(3, 0, 12, 11, 6);
pub const SYS_ICC_SGI0R_EL1: u64 = sys_reg(3, 0, 12, 11, 7);
pub const SYS_ICC_IAR1_EL1: u64 = sys_reg(3, 0, 12, 12, 0);
pub const SYS_ICC_EOIR1_EL1: u64 = sys_reg(3, 0, 12, 12, 1);
pub const SYS_ICC_HPPIR1_EL1: u64 = sys_reg(3, 0, 12, 12, 2);
pub const SYS_ICC_BPR1_EL1: u64 = sys_reg(3, 0, 12, 12, 3);
pub const SYS_ICC_CTLR_EL1: u64 = sys_reg(3, 0, 12, 12, 4);
pub const SYS_ICC_SRE_EL1: u64 = sys_reg(3, 0, 12, 12, 5);
pub const SYS_ICC_IGRPEN0_EL1: u64 = sys_reg(3, 0, 12, 12, 6);
pub const SYS_ICC_IGRPEN1_EL1: u64 = sys_reg(3, 0, 12, 12, 7);

pub const SYS_ACCDATA_EL1: u64 = sys_reg(3, 0, 13, 0, 5);

pub const SYS_CNTKCTL_EL1: u64 = sys_reg(3, 0, 14, 1, 0);

pub const SYS_AIDR_EL1: u64 = sys_reg(3, 1, 0, 0, 7);

pub const SYS_RNDR_EL0: u64 = sys_reg(3, 3, 2, 4, 0);
pub const SYS_RNDRRS_EL0: u64 = sys_reg(3, 3, 2, 4, 1);

pub const SYS_PMCR_EL0: u64 = sys_reg(3, 3, 9, 12, 0);
pub const SYS_PMCNTENSET_EL0: u64 = sys_reg(3, 3, 9, 12, 1);
pub const SYS_PMCNTENCLR_EL0: u64 = sys_reg(3, 3, 9, 12, 2);
pub const SYS_PMOVSCLR_EL0: u64 = sys_reg(3, 3, 9, 12, 3);
pub const SYS_PMSWINC_EL0: u64 = sys_reg(3, 3, 9, 12, 4);
pub const SYS_PMCEID0_EL0: u64 = sys_reg(3, 3, 9, 12, 6);
pub const SYS_PMCEID1_EL0: u64 = sys_reg(3, 3, 9, 12, 7);
pub const SYS_PMCCNTR_EL0: u64 = sys_reg(3, 3, 9, 13, 0);
pub const SYS_PMXEVTYPER_EL0: u64 = sys_reg(3, 3, 9, 13, 1);
pub const SYS_PMXEVCNTR_EL0: u64 = sys_reg(3, 3, 9, 13, 2);
pub const SYS_PMUSERENR_EL0: u64 = sys_reg(3, 3, 9, 14, 0);
pub const SYS_PMOVSSET_EL0: u64 = sys_reg(3, 3, 9, 14, 3);

pub const SYS_TPIDR_EL0: u64 = sys_reg(3, 3, 13, 0, 2);
pub const SYS_TPIDRRO_EL0: u64 = sys_reg(3, 3, 13, 0, 3);
pub const SYS_TPIDR2_EL0: u64 = sys_reg(3, 3, 13, 0, 5);

pub const SYS_SCXTNUM_EL0: u64 = sys_reg(3, 3, 13, 0, 7);

/* Definitions for system register interface to AMU for ARMv8.4 onwards */
pub const fn SYS_AM_EL0(crm: u64, op2: u64) -> u64 { sys_reg(3, 3, 13, (crm), (op2)) }
pub const SYS_AMCR_EL0: u64 = SYS_AM_EL0(2, 0);
pub const SYS_AMCFGR_EL0: u64 = SYS_AM_EL0(2, 1);
pub const SYS_AMCGCR_EL0: u64 = SYS_AM_EL0(2, 2);
pub const SYS_AMUSERENR_EL0: u64 = SYS_AM_EL0(2, 3);
pub const SYS_AMCNTENCLR0_EL0: u64 = SYS_AM_EL0(2, 4);
pub const SYS_AMCNTENSET0_EL0: u64 = SYS_AM_EL0(2, 5);
pub const SYS_AMCNTENCLR1_EL0: u64 = SYS_AM_EL0(3, 0);
pub const SYS_AMCNTENSET1_EL0: u64 = SYS_AM_EL0(3, 1);

/*
 * Group 0 of activity monitors (architected):
 *                op0  op1  CRn   CRm       op2
 * Counter:       11   011  1101  010:n<3>  n<2:0>
 * Type:          11   011  1101  011:n<3>  n<2:0>
 * n: 0-15
 *
 * Group 1 of activity monitors (auxiliary):
 *                op0  op1  CRn   CRm       op2
 * Counter:       11   011  1101  110:n<3>  n<2:0>
 * Type:          11   011  1101  111:n<3>  n<2:0>
 * n: 0-15
 */

pub const fn SYS_AMEVCNTR0_EL0(n: u64) -> u64 { SYS_AM_EL0(4 + ((n) >> 3), (n) & 7) }
pub const fn SYS_AMEVTYPER0_EL0(n: u64) -> u64 { SYS_AM_EL0(6 + ((n) >> 3), (n) & 7) }
pub const fn SYS_AMEVCNTR1_EL0(n: u64) -> u64 { SYS_AM_EL0(12 + ((n) >> 3), (n) & 7) }
pub const fn SYS_AMEVTYPER1_EL0(n: u64) -> u64 { SYS_AM_EL0(14 + ((n) >> 3), (n) & 7) }

/* AMU v1: Fixed (architecturally defined) activity monitors */
pub const SYS_AMEVCNTR0_CORE_EL0: u64 = SYS_AMEVCNTR0_EL0(0);
pub const SYS_AMEVCNTR0_CONST_EL0: u64 = SYS_AMEVCNTR0_EL0(1);
pub const SYS_AMEVCNTR0_INST_RET_EL0: u64 = SYS_AMEVCNTR0_EL0(2);
pub const SYS_AMEVCNTR0_MEM_STALL: u64 = SYS_AMEVCNTR0_EL0(3);

pub const SYS_CNTFRQ_EL0: u64 = sys_reg(3, 3, 14, 0, 0);

pub const SYS_CNTPCT_EL0: u64 = sys_reg(3, 3, 14, 0, 1);
pub const SYS_CNTVCT_EL0: u64 = sys_reg(3, 3, 14, 0, 2);
pub const SYS_CNTPCTSS_EL0: u64 = sys_reg(3, 3, 14, 0, 5);
pub const SYS_CNTVCTSS_EL0: u64 = sys_reg(3, 3, 14, 0, 6);

pub const SYS_CNTP_TVAL_EL0: u64 = sys_reg(3, 3, 14, 2, 0);
pub const SYS_CNTP_CTL_EL0: u64 = sys_reg(3, 3, 14, 2, 1);
pub const SYS_CNTP_CVAL_EL0: u64 = sys_reg(3, 3, 14, 2, 2);

pub const SYS_CNTV_TVAL_EL0: u64 = sys_reg(3, 3, 14, 3, 0);
pub const SYS_CNTV_CTL_EL0: u64 = sys_reg(3, 3, 14, 3, 1);
pub const SYS_CNTV_CVAL_EL0: u64 = sys_reg(3, 3, 14, 3, 2);

pub const SYS_AARCH32_CNTP_TVAL: u64 = sys_reg(0, 0, 14, 2, 0);
pub const SYS_AARCH32_CNTP_CTL: u64 = sys_reg(0, 0, 14, 2, 1);
pub const SYS_AARCH32_CNTPCT: u64 = sys_reg(0, 0, 0, 14, 0);
pub const SYS_AARCH32_CNTVCT: u64 = sys_reg(0, 1, 0, 14, 0);
pub const SYS_AARCH32_CNTP_CVAL: u64 = sys_reg(0, 2, 0, 14, 0);
pub const SYS_AARCH32_CNTPCTSS: u64 = sys_reg(0, 8, 0, 14, 0);
pub const SYS_AARCH32_CNTVCTSS: u64 = sys_reg(0, 9, 0, 14, 0);

pub const fn __PMEV_op2(n: u64) -> u64 { ((n) & 0x7) }
pub const fn __CNTR_CRm(n: u64) -> u64 { (0x8 | (((n) >> 3) & 0x3)) }
pub const fn SYS_PMEVCNTSVRn_EL1(n: u64) -> u64 { sys_reg(2, 0, 14, __CNTR_CRm(n), __PMEV_op2(n)) }
pub const fn SYS_PMEVCNTRn_EL0(n: u64) -> u64 { sys_reg(3, 3, 14, __CNTR_CRm(n), __PMEV_op2(n)) }
pub const fn __TYPER_CRm(n: u64) -> u64 { (0xc | (((n) >> 3) & 0x3)) }
pub const fn SYS_PMEVTYPERn_EL0(n: u64) -> u64 { sys_reg(3, 3, 14, __TYPER_CRm(n), __PMEV_op2(n)) }

pub const SYS_PMCCFILTR_EL0: u64 = sys_reg(3, 3, 14, 15, 7);

pub const fn SYS_SPMCGCRn_EL1(n: u64) -> u64 { sys_reg(2, 0, 9, 13, ((n) & 1)) }

pub const fn __SPMEV_op2(n: u64) -> u64 { ((n) & 0x7) }
pub const fn __SPMEV_crm(p: u64, n: u64) -> u64 { ((((p) & 7) << 1) | (((n) >> 3) & 1)) }
pub const fn SYS_SPMEVCNTRn_EL0(n: u64) -> u64 { sys_reg(2, 3, 14, __SPMEV_crm(0b000, n), __SPMEV_op2(n)) }
pub const fn SYS_SPMEVFILT2Rn_EL0(n: u64) -> u64 { sys_reg(2, 3, 14, __SPMEV_crm(0b011, n), __SPMEV_op2(n)) }
pub const fn SYS_SPMEVFILTRn_EL0(n: u64) -> u64 { sys_reg(2, 3, 14, __SPMEV_crm(0b010, n), __SPMEV_op2(n)) }
pub const fn SYS_SPMEVTYPERn_EL0(n: u64) -> u64 { sys_reg(2, 3, 14, __SPMEV_crm(0b001, n), __SPMEV_op2(n)) }

pub const SYS_VPIDR_EL2: u64 = sys_reg(3, 4, 0, 0, 0);
pub const SYS_VMPIDR_EL2: u64 = sys_reg(3, 4, 0, 0, 5);

pub const SYS_SCTLR_EL2: u64 = sys_reg(3, 4, 1, 0, 0);
pub const SYS_ACTLR_EL2: u64 = sys_reg(3, 4, 1, 0, 1);
pub const SYS_SCTLR2_EL2: u64 = sys_reg(3, 4, 1, 0, 3);
pub const SYS_HCR_EL2: u64 = sys_reg(3, 4, 1, 1, 0);
pub const SYS_MDCR_EL2: u64 = sys_reg(3, 4, 1, 1, 1);
pub const SYS_CPTR_EL2: u64 = sys_reg(3, 4, 1, 1, 2);
pub const SYS_HSTR_EL2: u64 = sys_reg(3, 4, 1, 1, 3);
pub const SYS_HACR_EL2: u64 = sys_reg(3, 4, 1, 1, 7);

pub const SYS_TTBR0_EL2: u64 = sys_reg(3, 4, 2, 0, 0);
pub const SYS_TTBR1_EL2: u64 = sys_reg(3, 4, 2, 0, 1);
pub const SYS_TCR_EL2: u64 = sys_reg(3, 4, 2, 0, 2);
pub const SYS_VTTBR_EL2: u64 = sys_reg(3, 4, 2, 1, 0);
pub const SYS_VTCR_EL2: u64 = sys_reg(3, 4, 2, 1, 2);

pub const SYS_VNCR_EL2: u64 = sys_reg(3, 4, 2, 2, 0);
pub const SYS_SPSR_EL2: u64 = sys_reg(3, 4, 4, 0, 0);
pub const SYS_ELR_EL2: u64 = sys_reg(3, 4, 4, 0, 1);
pub const SYS_SP_EL1: u64 = sys_reg(3, 4, 4, 1, 0);
pub const SYS_SPSR_irq: u64 = sys_reg(3, 4, 4, 3, 0);
pub const SYS_SPSR_abt: u64 = sys_reg(3, 4, 4, 3, 1);
pub const SYS_SPSR_und: u64 = sys_reg(3, 4, 4, 3, 2);
pub const SYS_SPSR_fiq: u64 = sys_reg(3, 4, 4, 3, 3);
pub const SYS_IFSR32_EL2: u64 = sys_reg(3, 4, 5, 0, 1);
pub const SYS_AFSR0_EL2: u64 = sys_reg(3, 4, 5, 1, 0);
pub const SYS_AFSR1_EL2: u64 = sys_reg(3, 4, 5, 1, 1);
pub const SYS_ESR_EL2: u64 = sys_reg(3, 4, 5, 2, 0);
pub const SYS_VSESR_EL2: u64 = sys_reg(3, 4, 5, 2, 3);
pub const SYS_FPEXC32_EL2: u64 = sys_reg(3, 4, 5, 3, 0);
pub const SYS_TFSR_EL2: u64 = sys_reg(3, 4, 5, 6, 0);

pub const SYS_FAR_EL2: u64 = sys_reg(3, 4, 6, 0, 0);
pub const SYS_HPFAR_EL2: u64 = sys_reg(3, 4, 6, 0, 4);

pub const SYS_MAIR_EL2: u64 = sys_reg(3, 4, 10, 2, 0);
pub const SYS_AMAIR_EL2: u64 = sys_reg(3, 4, 10, 3, 0);

pub const SYS_VBAR_EL2: u64 = sys_reg(3, 4, 12, 0, 0);
pub const SYS_RVBAR_EL2: u64 = sys_reg(3, 4, 12, 0, 1);
pub const SYS_RMR_EL2: u64 = sys_reg(3, 4, 12, 0, 2);
pub const SYS_VDISR_EL2: u64 = sys_reg(3, 4, 12, 1, 1);
pub const fn __SYS__AP0Rx_EL2(x: u64) -> u64 { sys_reg(3, 4, 12, 8, x) }
pub const SYS_ICH_AP0R0_EL2: u64 = __SYS__AP0Rx_EL2(0);
pub const SYS_ICH_AP0R1_EL2: u64 = __SYS__AP0Rx_EL2(1);
pub const SYS_ICH_AP0R2_EL2: u64 = __SYS__AP0Rx_EL2(2);
pub const SYS_ICH_AP0R3_EL2: u64 = __SYS__AP0Rx_EL2(3);

pub const fn __SYS__AP1Rx_EL2(x: u64) -> u64 { sys_reg(3, 4, 12, 9, x) }
pub const SYS_ICH_AP1R0_EL2: u64 = __SYS__AP1Rx_EL2(0);
pub const SYS_ICH_AP1R1_EL2: u64 = __SYS__AP1Rx_EL2(1);
pub const SYS_ICH_AP1R2_EL2: u64 = __SYS__AP1Rx_EL2(2);
pub const SYS_ICH_AP1R3_EL2: u64 = __SYS__AP1Rx_EL2(3);

pub const SYS_ICH_VSEIR_EL2: u64 = sys_reg(3, 4, 12, 9, 4);
pub const SYS_ICC_SRE_EL2: u64 = sys_reg(3, 4, 12, 9, 5);
pub const SYS_ICH_EISR_EL2: u64 = sys_reg(3, 4, 12, 11, 3);
pub const SYS_ICH_ELRSR_EL2: u64 = sys_reg(3, 4, 12, 11, 5);
pub const SYS_ICH_VMCR_EL2: u64 = sys_reg(3, 4, 12, 11, 7);

pub const fn __SYS__LR0_EL2(x: u64) -> u64 { sys_reg(3, 4, 12, 12, x) }
pub const SYS_ICH_LR0_EL2: u64 = __SYS__LR0_EL2(0);
pub const SYS_ICH_LR1_EL2: u64 = __SYS__LR0_EL2(1);
pub const SYS_ICH_LR2_EL2: u64 = __SYS__LR0_EL2(2);
pub const SYS_ICH_LR3_EL2: u64 = __SYS__LR0_EL2(3);
pub const SYS_ICH_LR4_EL2: u64 = __SYS__LR0_EL2(4);
pub const SYS_ICH_LR5_EL2: u64 = __SYS__LR0_EL2(5);
pub const SYS_ICH_LR6_EL2: u64 = __SYS__LR0_EL2(6);
pub const SYS_ICH_LR7_EL2: u64 = __SYS__LR0_EL2(7);

pub const fn __SYS__LR8_EL2(x: u64) -> u64 { sys_reg(3, 4, 12, 13, x) }
pub const SYS_ICH_LR8_EL2: u64 = __SYS__LR8_EL2(0);
pub const SYS_ICH_LR9_EL2: u64 = __SYS__LR8_EL2(1);
pub const SYS_ICH_LR10_EL2: u64 = __SYS__LR8_EL2(2);
pub const SYS_ICH_LR11_EL2: u64 = __SYS__LR8_EL2(3);
pub const SYS_ICH_LR12_EL2: u64 = __SYS__LR8_EL2(4);
pub const SYS_ICH_LR13_EL2: u64 = __SYS__LR8_EL2(5);
pub const SYS_ICH_LR14_EL2: u64 = __SYS__LR8_EL2(6);
pub const SYS_ICH_LR15_EL2: u64 = __SYS__LR8_EL2(7);

pub const SYS_CONTEXTIDR_EL2: u64 = sys_reg(3, 4, 13, 0, 1);
pub const SYS_TPIDR_EL2: u64 = sys_reg(3, 4, 13, 0, 2);
pub const SYS_SCXTNUM_EL2: u64 = sys_reg(3, 4, 13, 0, 7);

pub const fn __AMEV_op2(m: u64) -> u64 { (m & 0x7) }
pub const fn __AMEV_CRm(n: u64, m: u64) -> u64 { (n | ((m & 0x8) >> 3)) }
pub const fn __SYS__AMEVCNTVOFF0n_EL2(m: u64) -> u64 { sys_reg(3, 4, 13, __AMEV_CRm(0x8, m), __AMEV_op2(m)) }
pub const fn SYS_AMEVCNTVOFF0n_EL2(m: u64) -> u64 { __SYS__AMEVCNTVOFF0n_EL2(m) }
pub const fn __SYS__AMEVCNTVOFF1n_EL2(m: u64) -> u64 { sys_reg(3, 4, 13, __AMEV_CRm(0xA, m), __AMEV_op2(m)) }
pub const fn SYS_AMEVCNTVOFF1n_EL2(m: u64) -> u64 { __SYS__AMEVCNTVOFF1n_EL2(m) }

pub const SYS_CNTVOFF_EL2: u64 = sys_reg(3, 4, 14, 0, 3);
pub const SYS_CNTHCTL_EL2: u64 = sys_reg(3, 4, 14, 1, 0);
pub const SYS_CNTHP_TVAL_EL2: u64 = sys_reg(3, 4, 14, 2, 0);
pub const SYS_CNTHP_CTL_EL2: u64 = sys_reg(3, 4, 14, 2, 1);
pub const SYS_CNTHP_CVAL_EL2: u64 = sys_reg(3, 4, 14, 2, 2);
pub const SYS_CNTHV_TVAL_EL2: u64 = sys_reg(3, 4, 14, 3, 0);
pub const SYS_CNTHV_CTL_EL2: u64 = sys_reg(3, 4, 14, 3, 1);
pub const SYS_CNTHV_CVAL_EL2: u64 = sys_reg(3, 4, 14, 3, 2);

/* VHE encodings for architectural EL0/1 system registers */
pub const SYS_BRBCR_EL12: u64 = sys_reg(2, 5, 9, 0, 0);
pub const SYS_TTBR0_EL12: u64 = sys_reg(3, 5, 2, 0, 0);
pub const SYS_TTBR1_EL12: u64 = sys_reg(3, 5, 2, 0, 1);
pub const SYS_SPSR_EL12: u64 = sys_reg(3, 5, 4, 0, 0);
pub const SYS_ELR_EL12: u64 = sys_reg(3, 5, 4, 0, 1);
pub const SYS_AFSR0_EL12: u64 = sys_reg(3, 5, 5, 1, 0);
pub const SYS_AFSR1_EL12: u64 = sys_reg(3, 5, 5, 1, 1);
pub const SYS_ESR_EL12: u64 = sys_reg(3, 5, 5, 2, 0);
pub const SYS_TFSR_EL12: u64 = sys_reg(3, 5, 5, 6, 0);
pub const SYS_PMSCR_EL12: u64 = sys_reg(3, 5, 9, 9, 0);
pub const SYS_MAIR_EL12: u64 = sys_reg(3, 5, 10, 2, 0);
pub const SYS_AMAIR_EL12: u64 = sys_reg(3, 5, 10, 3, 0);
pub const SYS_VBAR_EL12: u64 = sys_reg(3, 5, 12, 0, 0);
pub const SYS_SCXTNUM_EL12: u64 = sys_reg(3, 5, 13, 0, 7);
pub const SYS_CNTKCTL_EL12: u64 = sys_reg(3, 5, 14, 1, 0);
pub const SYS_CNTP_TVAL_EL02: u64 = sys_reg(3, 5, 14, 2, 0);
pub const SYS_CNTP_CTL_EL02: u64 = sys_reg(3, 5, 14, 2, 1);
pub const SYS_CNTP_CVAL_EL02: u64 = sys_reg(3, 5, 14, 2, 2);
pub const SYS_CNTV_TVAL_EL02: u64 = sys_reg(3, 5, 14, 3, 0);
pub const SYS_CNTV_CTL_EL02: u64 = sys_reg(3, 5, 14, 3, 1);
pub const SYS_CNTV_CVAL_EL02: u64 = sys_reg(3, 5, 14, 3, 2);

pub const SYS_SP_EL2: u64 = sys_reg(3, 6,  4, 1, 0);

/* AT instructions */
pub const AT_Op0: u64 = 1;
pub const AT_CRn: u64 = 7;

pub const OP_AT_S1E1R: u64 = sys_insn(AT_Op0, 0, AT_CRn, 8, 0);
pub const OP_AT_S1E1W: u64 = sys_insn(AT_Op0, 0, AT_CRn, 8, 1);
pub const OP_AT_S1E0R: u64 = sys_insn(AT_Op0, 0, AT_CRn, 8, 2);
pub const OP_AT_S1E0W: u64 = sys_insn(AT_Op0, 0, AT_CRn, 8, 3);
pub const OP_AT_S1E1RP: u64 = sys_insn(AT_Op0, 0, AT_CRn, 9, 0);
pub const OP_AT_S1E1WP: u64 = sys_insn(AT_Op0, 0, AT_CRn, 9, 1);
pub const OP_AT_S1E1A: u64 = sys_insn(AT_Op0, 0, AT_CRn, 9, 2);
pub const OP_AT_S1E2R: u64 = sys_insn(AT_Op0, 4, AT_CRn, 8, 0);
pub const OP_AT_S1E2W: u64 = sys_insn(AT_Op0, 4, AT_CRn, 8, 1);
pub const OP_AT_S12E1R: u64 = sys_insn(AT_Op0, 4, AT_CRn, 8, 4);
pub const OP_AT_S12E1W: u64 = sys_insn(AT_Op0, 4, AT_CRn, 8, 5);
pub const OP_AT_S12E0R: u64 = sys_insn(AT_Op0, 4, AT_CRn, 8, 6);
pub const OP_AT_S12E0W: u64 = sys_insn(AT_Op0, 4, AT_CRn, 8, 7);
pub const OP_AT_S1E2A: u64 = sys_insn(AT_Op0, 4, AT_CRn, 9, 2);

/* TLBI instructions */
pub const TLBI_Op0: u64 = 1;

pub const TLBI_Op1_EL1: u64 = 0	/* Accessible from EL1 or higher */;
pub const TLBI_Op1_EL2: u64 = 4	/* Accessible from EL2 or higher */;

pub const TLBI_CRn_XS: u64 = 8	/* Extra Slow (the common one) */;
pub const TLBI_CRn_nXS: u64 = 9	/* not Extra Slow (which nobody uses)*/;

pub const TLBI_CRm_IPAIS: u64 = 0	/* S2 Inner-Shareable */;
pub const TLBI_CRm_nROS: u64 = 1	/* non-Range, Outer-Sharable */;
pub const TLBI_CRm_RIS: u64 = 2	/* Range, Inner-Sharable */;
pub const TLBI_CRm_nRIS: u64 = 3	/* non-Range, Inner-Sharable */;
pub const TLBI_CRm_IPAONS: u64 = 4	/* S2 Outer and Non-Shareable */;
pub const TLBI_CRm_ROS: u64 = 5	/* Range, Outer-Sharable */;
pub const TLBI_CRm_RNS: u64 = 6	/* Range, Non-Sharable */;
pub const TLBI_CRm_nRNS: u64 = 7	/* non-Range, Non-Sharable */;

pub const OP_TLBI_VMALLE1OS: u64 = sys_insn(1, 0, 8, 1, 0);
pub const OP_TLBI_VAE1OS: u64 = sys_insn(1, 0, 8, 1, 1);
pub const OP_TLBI_ASIDE1OS: u64 = sys_insn(1, 0, 8, 1, 2);
pub const OP_TLBI_VAAE1OS: u64 = sys_insn(1, 0, 8, 1, 3);
pub const OP_TLBI_VALE1OS: u64 = sys_insn(1, 0, 8, 1, 5);
pub const OP_TLBI_VAALE1OS: u64 = sys_insn(1, 0, 8, 1, 7);
pub const OP_TLBI_RVAE1IS: u64 = sys_insn(1, 0, 8, 2, 1);
pub const OP_TLBI_RVAAE1IS: u64 = sys_insn(1, 0, 8, 2, 3);
pub const OP_TLBI_RVALE1IS: u64 = sys_insn(1, 0, 8, 2, 5);
pub const OP_TLBI_RVAALE1IS: u64 = sys_insn(1, 0, 8, 2, 7);
pub const OP_TLBI_VMALLE1IS: u64 = sys_insn(1, 0, 8, 3, 0);
pub const OP_TLBI_VAE1IS: u64 = sys_insn(1, 0, 8, 3, 1);
pub const OP_TLBI_ASIDE1IS: u64 = sys_insn(1, 0, 8, 3, 2);
pub const OP_TLBI_VAAE1IS: u64 = sys_insn(1, 0, 8, 3, 3);
pub const OP_TLBI_VALE1IS: u64 = sys_insn(1, 0, 8, 3, 5);
pub const OP_TLBI_VAALE1IS: u64 = sys_insn(1, 0, 8, 3, 7);
pub const OP_TLBI_RVAE1OS: u64 = sys_insn(1, 0, 8, 5, 1);
pub const OP_TLBI_RVAAE1OS: u64 = sys_insn(1, 0, 8, 5, 3);
pub const OP_TLBI_RVALE1OS: u64 = sys_insn(1, 0, 8, 5, 5);
pub const OP_TLBI_RVAALE1OS: u64 = sys_insn(1, 0, 8, 5, 7);
pub const OP_TLBI_RVAE1: u64 = sys_insn(1, 0, 8, 6, 1);
pub const OP_TLBI_RVAAE1: u64 = sys_insn(1, 0, 8, 6, 3);
pub const OP_TLBI_RVALE1: u64 = sys_insn(1, 0, 8, 6, 5);
pub const OP_TLBI_RVAALE1: u64 = sys_insn(1, 0, 8, 6, 7);
pub const OP_TLBI_VMALLE1: u64 = sys_insn(1, 0, 8, 7, 0);
pub const OP_TLBI_VAE1: u64 = sys_insn(1, 0, 8, 7, 1);
pub const OP_TLBI_ASIDE1: u64 = sys_insn(1, 0, 8, 7, 2);
pub const OP_TLBI_VAAE1: u64 = sys_insn(1, 0, 8, 7, 3);
pub const OP_TLBI_VALE1: u64 = sys_insn(1, 0, 8, 7, 5);
pub const OP_TLBI_VAALE1: u64 = sys_insn(1, 0, 8, 7, 7);
pub const OP_TLBI_VMALLE1OSNXS: u64 = sys_insn(1, 0, 9, 1, 0);
pub const OP_TLBI_VAE1OSNXS: u64 = sys_insn(1, 0, 9, 1, 1);
pub const OP_TLBI_ASIDE1OSNXS: u64 = sys_insn(1, 0, 9, 1, 2);
pub const OP_TLBI_VAAE1OSNXS: u64 = sys_insn(1, 0, 9, 1, 3);
pub const OP_TLBI_VALE1OSNXS: u64 = sys_insn(1, 0, 9, 1, 5);
pub const OP_TLBI_VAALE1OSNXS: u64 = sys_insn(1, 0, 9, 1, 7);
pub const OP_TLBI_RVAE1ISNXS: u64 = sys_insn(1, 0, 9, 2, 1);
pub const OP_TLBI_RVAAE1ISNXS: u64 = sys_insn(1, 0, 9, 2, 3);
pub const OP_TLBI_RVALE1ISNXS: u64 = sys_insn(1, 0, 9, 2, 5);
pub const OP_TLBI_RVAALE1ISNXS: u64 = sys_insn(1, 0, 9, 2, 7);
pub const OP_TLBI_VMALLE1ISNXS: u64 = sys_insn(1, 0, 9, 3, 0);
pub const OP_TLBI_VAE1ISNXS: u64 = sys_insn(1, 0, 9, 3, 1);
pub const OP_TLBI_ASIDE1ISNXS: u64 = sys_insn(1, 0, 9, 3, 2);
pub const OP_TLBI_VAAE1ISNXS: u64 = sys_insn(1, 0, 9, 3, 3);
pub const OP_TLBI_VALE1ISNXS: u64 = sys_insn(1, 0, 9, 3, 5);
pub const OP_TLBI_VAALE1ISNXS: u64 = sys_insn(1, 0, 9, 3, 7);
pub const OP_TLBI_RVAE1OSNXS: u64 = sys_insn(1, 0, 9, 5, 1);
pub const OP_TLBI_RVAAE1OSNXS: u64 = sys_insn(1, 0, 9, 5, 3);
pub const OP_TLBI_RVALE1OSNXS: u64 = sys_insn(1, 0, 9, 5, 5);
pub const OP_TLBI_RVAALE1OSNXS: u64 = sys_insn(1, 0, 9, 5, 7);
pub const OP_TLBI_RVAE1NXS: u64 = sys_insn(1, 0, 9, 6, 1);
pub const OP_TLBI_RVAAE1NXS: u64 = sys_insn(1, 0, 9, 6, 3);
pub const OP_TLBI_RVALE1NXS: u64 = sys_insn(1, 0, 9, 6, 5);
pub const OP_TLBI_RVAALE1NXS: u64 = sys_insn(1, 0, 9, 6, 7);
pub const OP_TLBI_VMALLE1NXS: u64 = sys_insn(1, 0, 9, 7, 0);
pub const OP_TLBI_VAE1NXS: u64 = sys_insn(1, 0, 9, 7, 1);
pub const OP_TLBI_ASIDE1NXS: u64 = sys_insn(1, 0, 9, 7, 2);
pub const OP_TLBI_VAAE1NXS: u64 = sys_insn(1, 0, 9, 7, 3);
pub const OP_TLBI_VALE1NXS: u64 = sys_insn(1, 0, 9, 7, 5);
pub const OP_TLBI_VAALE1NXS: u64 = sys_insn(1, 0, 9, 7, 7);
pub const OP_TLBI_IPAS2E1IS: u64 = sys_insn(1, 4, 8, 0, 1);
pub const OP_TLBI_RIPAS2E1IS: u64 = sys_insn(1, 4, 8, 0, 2);
pub const OP_TLBI_IPAS2LE1IS: u64 = sys_insn(1, 4, 8, 0, 5);
pub const OP_TLBI_RIPAS2LE1IS: u64 = sys_insn(1, 4, 8, 0, 6);
pub const OP_TLBI_ALLE2OS: u64 = sys_insn(1, 4, 8, 1, 0);
pub const OP_TLBI_VAE2OS: u64 = sys_insn(1, 4, 8, 1, 1);
pub const OP_TLBI_ALLE1OS: u64 = sys_insn(1, 4, 8, 1, 4);
pub const OP_TLBI_VALE2OS: u64 = sys_insn(1, 4, 8, 1, 5);
pub const OP_TLBI_VMALLS12E1OS: u64 = sys_insn(1, 4, 8, 1, 6);
pub const OP_TLBI_RVAE2IS: u64 = sys_insn(1, 4, 8, 2, 1);
pub const OP_TLBI_RVALE2IS: u64 = sys_insn(1, 4, 8, 2, 5);
pub const OP_TLBI_ALLE2IS: u64 = sys_insn(1, 4, 8, 3, 0);
pub const OP_TLBI_VAE2IS: u64 = sys_insn(1, 4, 8, 3, 1);
pub const OP_TLBI_ALLE1IS: u64 = sys_insn(1, 4, 8, 3, 4);
pub const OP_TLBI_VALE2IS: u64 = sys_insn(1, 4, 8, 3, 5);
pub const OP_TLBI_VMALLS12E1IS: u64 = sys_insn(1, 4, 8, 3, 6);
pub const OP_TLBI_IPAS2E1OS: u64 = sys_insn(1, 4, 8, 4, 0);
pub const OP_TLBI_IPAS2E1: u64 = sys_insn(1, 4, 8, 4, 1);
pub const OP_TLBI_RIPAS2E1: u64 = sys_insn(1, 4, 8, 4, 2);
pub const OP_TLBI_RIPAS2E1OS: u64 = sys_insn(1, 4, 8, 4, 3);
pub const OP_TLBI_IPAS2LE1OS: u64 = sys_insn(1, 4, 8, 4, 4);
pub const OP_TLBI_IPAS2LE1: u64 = sys_insn(1, 4, 8, 4, 5);
pub const OP_TLBI_RIPAS2LE1: u64 = sys_insn(1, 4, 8, 4, 6);
pub const OP_TLBI_RIPAS2LE1OS: u64 = sys_insn(1, 4, 8, 4, 7);
pub const OP_TLBI_RVAE2OS: u64 = sys_insn(1, 4, 8, 5, 1);
pub const OP_TLBI_RVALE2OS: u64 = sys_insn(1, 4, 8, 5, 5);
pub const OP_TLBI_RVAE2: u64 = sys_insn(1, 4, 8, 6, 1);
pub const OP_TLBI_RVALE2: u64 = sys_insn(1, 4, 8, 6, 5);
pub const OP_TLBI_ALLE2: u64 = sys_insn(1, 4, 8, 7, 0);
pub const OP_TLBI_VAE2: u64 = sys_insn(1, 4, 8, 7, 1);
pub const OP_TLBI_ALLE1: u64 = sys_insn(1, 4, 8, 7, 4);
pub const OP_TLBI_VALE2: u64 = sys_insn(1, 4, 8, 7, 5);
pub const OP_TLBI_VMALLS12E1: u64 = sys_insn(1, 4, 8, 7, 6);
pub const OP_TLBI_IPAS2E1ISNXS: u64 = sys_insn(1, 4, 9, 0, 1);
pub const OP_TLBI_RIPAS2E1ISNXS: u64 = sys_insn(1, 4, 9, 0, 2);
pub const OP_TLBI_IPAS2LE1ISNXS: u64 = sys_insn(1, 4, 9, 0, 5);
pub const OP_TLBI_RIPAS2LE1ISNXS: u64 = sys_insn(1, 4, 9, 0, 6);
pub const OP_TLBI_ALLE2OSNXS: u64 = sys_insn(1, 4, 9, 1, 0);
pub const OP_TLBI_VAE2OSNXS: u64 = sys_insn(1, 4, 9, 1, 1);
pub const OP_TLBI_ALLE1OSNXS: u64 = sys_insn(1, 4, 9, 1, 4);
pub const OP_TLBI_VALE2OSNXS: u64 = sys_insn(1, 4, 9, 1, 5);
pub const OP_TLBI_VMALLS12E1OSNXS: u64 = sys_insn(1, 4, 9, 1, 6);
pub const OP_TLBI_RVAE2ISNXS: u64 = sys_insn(1, 4, 9, 2, 1);
pub const OP_TLBI_RVALE2ISNXS: u64 = sys_insn(1, 4, 9, 2, 5);
pub const OP_TLBI_ALLE2ISNXS: u64 = sys_insn(1, 4, 9, 3, 0);
pub const OP_TLBI_VAE2ISNXS: u64 = sys_insn(1, 4, 9, 3, 1);
pub const OP_TLBI_ALLE1ISNXS: u64 = sys_insn(1, 4, 9, 3, 4);
pub const OP_TLBI_VALE2ISNXS: u64 = sys_insn(1, 4, 9, 3, 5);
pub const OP_TLBI_VMALLS12E1ISNXS: u64 = sys_insn(1, 4, 9, 3, 6);
pub const OP_TLBI_IPAS2E1OSNXS: u64 = sys_insn(1, 4, 9, 4, 0);
pub const OP_TLBI_IPAS2E1NXS: u64 = sys_insn(1, 4, 9, 4, 1);
pub const OP_TLBI_RIPAS2E1NXS: u64 = sys_insn(1, 4, 9, 4, 2);
pub const OP_TLBI_RIPAS2E1OSNXS: u64 = sys_insn(1, 4, 9, 4, 3);
pub const OP_TLBI_IPAS2LE1OSNXS: u64 = sys_insn(1, 4, 9, 4, 4);
pub const OP_TLBI_IPAS2LE1NXS: u64 = sys_insn(1, 4, 9, 4, 5);
pub const OP_TLBI_RIPAS2LE1NXS: u64 = sys_insn(1, 4, 9, 4, 6);
pub const OP_TLBI_RIPAS2LE1OSNXS: u64 = sys_insn(1, 4, 9, 4, 7);
pub const OP_TLBI_RVAE2OSNXS: u64 = sys_insn(1, 4, 9, 5, 1);
pub const OP_TLBI_RVALE2OSNXS: u64 = sys_insn(1, 4, 9, 5, 5);
pub const OP_TLBI_RVAE2NXS: u64 = sys_insn(1, 4, 9, 6, 1);
pub const OP_TLBI_RVALE2NXS: u64 = sys_insn(1, 4, 9, 6, 5);
pub const OP_TLBI_ALLE2NXS: u64 = sys_insn(1, 4, 9, 7, 0);
pub const OP_TLBI_VAE2NXS: u64 = sys_insn(1, 4, 9, 7, 1);
pub const OP_TLBI_ALLE1NXS: u64 = sys_insn(1, 4, 9, 7, 4);
pub const OP_TLBI_VALE2NXS: u64 = sys_insn(1, 4, 9, 7, 5);
pub const OP_TLBI_VMALLS12E1NXS: u64 = sys_insn(1, 4, 9, 7, 6);

/* Misc instructions */
pub const OP_GCSPUSHX: u64 = sys_insn(1, 0, 7, 7, 4);
pub const OP_GCSPOPCX: u64 = sys_insn(1, 0, 7, 7, 5);
pub const OP_GCSPOPX: u64 = sys_insn(1, 0, 7, 7, 6);
pub const OP_GCSPUSHM: u64 = sys_insn(1, 3, 7, 7, 0);

pub const OP_BRB_IALL: u64 = sys_insn(1, 1, 7, 2, 4);
pub const OP_BRB_INJ: u64 = sys_insn(1, 1, 7, 2, 5);
pub const OP_CFP_RCTX: u64 = sys_insn(1, 3, 7, 3, 4);
pub const OP_DVP_RCTX: u64 = sys_insn(1, 3, 7, 3, 5);
pub const OP_COSP_RCTX: u64 = sys_insn(1, 3, 7, 3, 6);
pub const OP_CPP_RCTX: u64 = sys_insn(1, 3, 7, 3, 7);

/* Common SCTLR_ELx flags. */
pub const SCTLR_ELx_ENTP2: u64 = (BIT(60));
pub const SCTLR_ELx_DSSBS: u64 = (BIT(44));
pub const SCTLR_ELx_ATA: u64 = (BIT(43));

pub const SCTLR_ELx_EE_SHIFT: u64 = 25;
pub const SCTLR_ELx_ENIA_SHIFT: u64 = 31;

pub const SCTLR_ELx_ITFSB: u64 = (BIT(37));
pub const SCTLR_ELx_ENIA: u64 = (BIT(SCTLR_ELx_ENIA_SHIFT));
pub const SCTLR_ELx_ENIB: u64 = (BIT(30));
pub const SCTLR_ELx_LSMAOE: u64 = (BIT(29));
pub const SCTLR_ELx_nTLSMD: u64 = (BIT(28));
pub const SCTLR_ELx_ENDA: u64 = (BIT(27));
pub const SCTLR_ELx_EE: u64 = (BIT(SCTLR_ELx_EE_SHIFT));
pub const SCTLR_ELx_EIS: u64 = (BIT(22));
pub const SCTLR_ELx_IESB: u64 = (BIT(21));
pub const SCTLR_ELx_TSCXT: u64 = (BIT(20));
pub const SCTLR_ELx_WXN: u64 = (BIT(19));
pub const SCTLR_ELx_ENDB: u64 = (BIT(13));
pub const SCTLR_ELx_I: u64 = (BIT(12));
pub const SCTLR_ELx_EOS: u64 = (BIT(11));
pub const SCTLR_ELx_SA: u64 = (BIT(3));
pub const SCTLR_ELx_C: u64 = (BIT(2));
pub const SCTLR_ELx_A: u64 = (BIT(1));
pub const SCTLR_ELx_M: u64 = (BIT(0));

// Preprocessor condition preserved from C: #ifdef CONFIG_CPU_BIG_ENDIAN
pub const ENDIAN_SET_EL2: u64 = SCTLR_ELx_EE;
// Preprocessor condition preserved from C: #else
pub const ENDIAN_SET_EL2: u64 = 0;
// Preprocessor condition preserved from C: #endif

pub const INIT_SCTLR_EL2_MMU_ON: u64 = (SCTLR_ELx_M  | SCTLR_ELx_C | SCTLR_ELx_SA | SCTLR_ELx_I |	 	 SCTLR_ELx_IESB | SCTLR_ELx_WXN | ENDIAN_SET_EL2 |		 	 SCTLR_ELx_ITFSB | SCTLR_EL2_RES1);

pub const INIT_SCTLR_EL2_MMU_OFF: u64 = (SCTLR_EL2_RES1 | ENDIAN_SET_EL2);

/* SCTLR_EL1 specific flags. */
// Preprocessor condition preserved from C: #ifdef CONFIG_CPU_BIG_ENDIAN
pub const ENDIAN_SET_EL1: u64 = (SCTLR_EL1_E0E | SCTLR_ELx_EE);
// Preprocessor condition preserved from C: #else
pub const ENDIAN_SET_EL1: u64 = 0;
// Preprocessor condition preserved from C: #endif

pub const INIT_SCTLR_EL1_MMU_OFF: u64 = (ENDIAN_SET_EL1 | SCTLR_EL1_LSMAOE | SCTLR_EL1_nTLSMD |  	 SCTLR_EL1_EIS  | SCTLR_EL1_TSCXT  | SCTLR_EL1_EOS);

pub const INIT_SCTLR_EL1_MMU_ON: u64 = (SCTLR_ELx_M      | SCTLR_ELx_C      | SCTLR_ELx_SA    |  	 SCTLR_EL1_SA0    | SCTLR_EL1_SED    | SCTLR_ELx_I     |  	 SCTLR_EL1_DZE    | SCTLR_EL1_UCT    | SCTLR_EL1_nTWE  |  	 SCTLR_ELx_IESB   | SCTLR_EL1_SPAN   | SCTLR_ELx_ITFSB |  	 ENDIAN_SET_EL1   | SCTLR_EL1_UCI    | SCTLR_EL1_EPAN  |  	 SCTLR_EL1_LSMAOE | SCTLR_EL1_nTLSMD | SCTLR_EL1_EIS   |  	 SCTLR_EL1_TSCXT  | SCTLR_EL1_EOS);

/* MAIR_ELx memory attributes (used by Linux) */
pub const MAIR_ATTR_DEVICE_nGnRnE: u64 = UL(0x00);
pub const MAIR_ATTR_DEVICE_nGnRE: u64 = UL(0x04);
pub const MAIR_ATTR_NORMAL_NC: u64 = UL(0x44);
pub const MAIR_ATTR_NORMAL_TAGGED: u64 = UL(0xf0);
pub const MAIR_ATTR_NORMAL: u64 = UL(0xff);
pub const MAIR_ATTR_MASK: u64 = UL(0xff);

/* Position the attr at the correct index */
pub const fn MAIR_ATTRIDX(attr: u64, idx: u64) -> u64 { ((attr) << ((idx) * 8)) }

/* id_aa64mmfr0 */
pub const ID_AA64MMFR0_EL1_TGRAN4_SUPPORTED_MIN: u64 = 0x0;
pub const ID_AA64MMFR0_EL1_TGRAN4_LPA2: u64 = ID_AA64MMFR0_EL1_TGRAN4_52_BIT;
pub const ID_AA64MMFR0_EL1_TGRAN4_SUPPORTED_MAX: u64 = 0x7;
pub const ID_AA64MMFR0_EL1_TGRAN64_SUPPORTED_MIN: u64 = 0x0;
pub const ID_AA64MMFR0_EL1_TGRAN64_SUPPORTED_MAX: u64 = 0x7;
pub const ID_AA64MMFR0_EL1_TGRAN16_SUPPORTED_MIN: u64 = 0x1;
pub const ID_AA64MMFR0_EL1_TGRAN16_LPA2: u64 = ID_AA64MMFR0_EL1_TGRAN16_52_BIT;
pub const ID_AA64MMFR0_EL1_TGRAN16_SUPPORTED_MAX: u64 = 0xf;

pub const ARM64_MIN_PARANGE_BITS: u64 = 32;

pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_DEFAULT: u64 = 0x0;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_NONE: u64 = 0x1;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_MIN: u64 = 0x2;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_LPA2: u64 = 0x3;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_MAX: u64 = 0x7;

// Preprocessor condition preserved from C: #ifdef CONFIG_ARM64_PA_BITS_52
pub const ID_AA64MMFR0_EL1_PARANGE_MAX: u64 = ID_AA64MMFR0_EL1_PARANGE_52;
// Preprocessor condition preserved from C: #else
pub const ID_AA64MMFR0_EL1_PARANGE_MAX: u64 = ID_AA64MMFR0_EL1_PARANGE_48;
// Preprocessor condition preserved from C: #endif

// Preprocessor condition preserved from C: #if defined(CONFIG_ARM64_4K_PAGES)
pub const ID_AA64MMFR0_EL1_TGRAN_SHIFT: u64 = ID_AA64MMFR0_EL1_TGRAN4_SHIFT;
pub const ID_AA64MMFR0_EL1_TGRAN_LPA2: u64 = ID_AA64MMFR0_EL1_TGRAN4_52_BIT;
pub const ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MIN: u64 = ID_AA64MMFR0_EL1_TGRAN4_SUPPORTED_MIN;
pub const ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MAX: u64 = ID_AA64MMFR0_EL1_TGRAN4_SUPPORTED_MAX;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SHIFT: u64 = ID_AA64MMFR0_EL1_TGRAN4_2_SHIFT;
// Preprocessor condition preserved from C: #elif defined(CONFIG_ARM64_16K_PAGES)
pub const ID_AA64MMFR0_EL1_TGRAN_SHIFT: u64 = ID_AA64MMFR0_EL1_TGRAN16_SHIFT;
pub const ID_AA64MMFR0_EL1_TGRAN_LPA2: u64 = ID_AA64MMFR0_EL1_TGRAN16_52_BIT;
pub const ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MIN: u64 = ID_AA64MMFR0_EL1_TGRAN16_SUPPORTED_MIN;
pub const ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MAX: u64 = ID_AA64MMFR0_EL1_TGRAN16_SUPPORTED_MAX;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SHIFT: u64 = ID_AA64MMFR0_EL1_TGRAN16_2_SHIFT;
// Preprocessor condition preserved from C: #elif defined(CONFIG_ARM64_64K_PAGES)
pub const ID_AA64MMFR0_EL1_TGRAN_SHIFT: u64 = ID_AA64MMFR0_EL1_TGRAN64_SHIFT;
pub const ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MIN: u64 = ID_AA64MMFR0_EL1_TGRAN64_SUPPORTED_MIN;
pub const ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MAX: u64 = ID_AA64MMFR0_EL1_TGRAN64_SUPPORTED_MAX;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SHIFT: u64 = ID_AA64MMFR0_EL1_TGRAN64_2_SHIFT;
// Preprocessor condition preserved from C: #endif

pub const CPACR_EL1_FPEN_EL1EN: u64 = (BIT(20)) /* enable EL1 access */;
pub const CPACR_EL1_FPEN_EL0EN: u64 = (BIT(21)) /* enable EL0 access, if EL1EN set */;

pub const CPACR_EL1_SMEN_EL1EN: u64 = (BIT(24)) /* enable EL1 access */;
pub const CPACR_EL1_SMEN_EL0EN: u64 = (BIT(25)) /* enable EL0 access, if EL1EN set */;

pub const CPACR_EL1_ZEN_EL1EN: u64 = (BIT(16)) /* enable EL1 access */;
pub const CPACR_EL1_ZEN_EL0EN: u64 = (BIT(17)) /* enable EL0 access, if EL1EN set */;

/* GCR_EL1 Definitions */
pub const SYS_GCR_EL1_RRND: u64 = (BIT(16));
pub const SYS_GCR_EL1_EXCL_MASK: u64 = u64;

// Preprocessor condition preserved from C: #ifdef CONFIG_KASAN_HW_TAGS
/*
 * KASAN always uses a whole byte for its tags. With CONFIG_KASAN_HW_TAGS it
 * only uses tags in the range 0xF0-0xFF, which we map to MTE tags 0x0-0xF.
 */
pub const __MTE_TAG_MIN: u64 = (KASAN_TAG_MIN & 0xf);
pub const __MTE_TAG_MAX: u64 = (KASAN_TAG_MAX & 0xf);
pub const __MTE_TAG_INCL: u64 = GENMASK(__MTE_TAG_MAX, __MTE_TAG_MIN);
pub const KERNEL_GCR_EL1_EXCL: u64 = (SYS_GCR_EL1_EXCL_MASK & ~__MTE_TAG_INCL);
// Preprocessor condition preserved from C: #else
pub const KERNEL_GCR_EL1_EXCL: u64 = SYS_GCR_EL1_EXCL_MASK;
// Preprocessor condition preserved from C: #endif

pub const KERNEL_GCR_EL1: u64 = (SYS_GCR_EL1_RRND | KERNEL_GCR_EL1_EXCL);

/* RGSR_EL1 Definitions */
pub const SYS_RGSR_EL1_TAG_MASK: u64 = u64;
pub const SYS_RGSR_EL1_SEED_SHIFT: u64 = 8;
pub const SYS_RGSR_EL1_SEED_MASK: u64 = u64;

/* TFSR{,E0}_EL1 bit definitions */
pub const SYS_TFSR_EL1_TF0_SHIFT: u64 = 0;
pub const SYS_TFSR_EL1_TF1_SHIFT: u64 = 1;
pub const SYS_TFSR_EL1_TF0: u64 = (UL(1) << SYS_TFSR_EL1_TF0_SHIFT);
pub const SYS_TFSR_EL1_TF1: u64 = (UL(1) << SYS_TFSR_EL1_TF1_SHIFT);

/* Safe value for MPIDR_EL1: Bit31:RES1, Bit30:U:0, Bit24:MT:0 */
pub const SYS_MPIDR_SAFE_VAL: u64 = (BIT(31));

/* GIC Hypervisor interface registers */
/* ICH_LR*_EL2 bit definitions */
pub const ICH_LR_VIRTUAL_ID_MASK: u64 = ((u64 << 32) - 1);

pub const ICH_LR_EOI: u64 = (u64 << 41);
pub const ICH_LR_GROUP: u64 = (u64 << 60);
pub const ICH_LR_HW: u64 = (u64 << 61);
pub const ICH_LR_STATE: u64 = (u64 << 62);
pub const ICH_LR_PENDING_BIT: u64 = (u64 << 62);
pub const ICH_LR_ACTIVE_BIT: u64 = (u64 << 63);
pub const ICH_LR_PHYS_ID_SHIFT: u64 = 32;
pub const ICH_LR_PHYS_ID_MASK: u64 = (u64 << ICH_LR_PHYS_ID_SHIFT);
pub const ICH_LR_PRIORITY_SHIFT: u64 = 48;
pub const ICH_LR_PRIORITY_MASK: u64 = (u64 << ICH_LR_PRIORITY_SHIFT);

/* ICH_VMCR_EL2 bit definitions */
pub const ICH_VMCR_ACK_CTL_SHIFT: u64 = 2;
pub const ICH_VMCR_ACK_CTL_MASK: u64 = (1 << ICH_VMCR_ACK_CTL_SHIFT);
pub const ICH_VMCR_FIQ_EN_SHIFT: u64 = 3;
pub const ICH_VMCR_FIQ_EN_MASK: u64 = (1 << ICH_VMCR_FIQ_EN_SHIFT);
pub const ICH_VMCR_CBPR_SHIFT: u64 = 4;
pub const ICH_VMCR_CBPR_MASK: u64 = (1 << ICH_VMCR_CBPR_SHIFT);
pub const ICH_VMCR_EOIM_SHIFT: u64 = 9;
pub const ICH_VMCR_EOIM_MASK: u64 = (1 << ICH_VMCR_EOIM_SHIFT);
pub const ICH_VMCR_BPR1_SHIFT: u64 = 18;
pub const ICH_VMCR_BPR1_MASK: u64 = (7 << ICH_VMCR_BPR1_SHIFT);
pub const ICH_VMCR_BPR0_SHIFT: u64 = 21;
pub const ICH_VMCR_BPR0_MASK: u64 = (7 << ICH_VMCR_BPR0_SHIFT);
pub const ICH_VMCR_PMR_SHIFT: u64 = 24;
pub const ICH_VMCR_PMR_MASK: u64 = (u64 << ICH_VMCR_PMR_SHIFT);
pub const ICH_VMCR_ENG0_SHIFT: u64 = 0;
pub const ICH_VMCR_ENG0_MASK: u64 = (1 << ICH_VMCR_ENG0_SHIFT);
pub const ICH_VMCR_ENG1_SHIFT: u64 = 1;
pub const ICH_VMCR_ENG1_MASK: u64 = (1 << ICH_VMCR_ENG1_SHIFT);

/*
 * Permission Indirection Extension (PIE) permission encodings.
 * Encodings with the _O suffix, have overlays applied (Permission Overlay Extension).
 */
pub const PIE_NONE_O: u64 = UL(0x0);
pub const PIE_R_O: u64 = UL(0x1);
pub const PIE_X_O: u64 = UL(0x2);
pub const PIE_RX_O: u64 = UL(0x3);
pub const PIE_RW_O: u64 = UL(0x5);
pub const PIE_RWnX_O: u64 = UL(0x6);
pub const PIE_RWX_O: u64 = UL(0x7);
pub const PIE_R: u64 = UL(0x8);
pub const PIE_GCS: u64 = UL(0x9);
pub const PIE_RX: u64 = UL(0xa);
pub const PIE_RW: u64 = UL(0xc);
pub const PIE_RWX: u64 = UL(0xe);
pub const PIE_MASK: u64 = UL(0xf);

pub const PIRx_ELx_BITS_PER_IDX: u64 = 4;
pub const fn PIRx_ELx_PERM_SHIFT(idx: u64) -> u64 { ((idx) * PIRx_ELx_BITS_PER_IDX) }
pub const fn PIRx_ELx_PERM_PREP(idx: u64, perm: u64) -> u64 { (((perm) & PIE_MASK) << PIRx_ELx_PERM_SHIFT(idx)) }

/*
 * Permission Overlay Extension (POE) permission encodings.
 */
pub const POE_NONE: u64 = UL(0x0);
pub const POE_R: u64 = UL(0x1);
pub const POE_X: u64 = UL(0x2);
pub const POE_RX: u64 = UL(0x3);
pub const POE_W: u64 = UL(0x4);
pub const POE_RW: u64 = UL(0x5);
pub const POE_WX: u64 = UL(0x6);
pub const POE_RWX: u64 = UL(0x7);
pub const POE_MASK: u64 = UL(0xf);

pub const POR_ELx_BITS_PER_IDX: u64 = 4;
pub const fn POR_ELx_PERM_SHIFT(idx: u64) -> u64 { ((idx) * POR_ELx_BITS_PER_IDX) }
pub const fn POR_ELx_PERM_GET(idx: u64, reg: u64) -> u64 { (((reg) >> POR_ELx_PERM_SHIFT(idx)) & POE_MASK) }
pub const fn POR_ELx_PERM_PREP(idx: u64, perm: u64) -> u64 { (((perm) & POE_MASK) << POR_ELx_PERM_SHIFT(idx)) }

/*
 * Definitions for Guarded Control Stack
 */

pub const GCS_CAP_ADDR_MASK: u64 = GENMASK(63, 12);
pub const GCS_CAP_ADDR_SHIFT: u64 = 12;
pub const GCS_CAP_ADDR_WIDTH: u64 = 52;
pub const fn GCS_CAP_ADDR(x: u64) -> u64 { FIELD_GET(GCS_CAP_ADDR_MASK, x) }

pub const GCS_CAP_TOKEN_MASK: u64 = GENMASK(11, 0);
pub const GCS_CAP_TOKEN_SHIFT: u64 = 0;
pub const GCS_CAP_TOKEN_WIDTH: u64 = 12;
pub const fn GCS_CAP_TOKEN(x: u64) -> u64 { FIELD_GET(GCS_CAP_TOKEN_MASK, x) }

pub const GCS_CAP_VALID_TOKEN: u64 = 0x1;
pub const GCS_CAP_IN_PROGRESS_TOKEN: u64 = 0x5;

pub const fn GCS_CAP(x: u64) -> u64 { (((x) & GCS_CAP_ADDR_MASK) |  					       GCS_CAP_VALID_TOKEN) }

// Preprocessor condition preserved from C: #ifdef __ASSEMBLER__

	.macro	mrs_s, rt, sreg
	 __emit_inst(0xd5200000|(\sreg)|(.L__gpr_num_\rt))
	.endm

	.macro	msr_s, sreg, rt
	__emit_inst(0xd5000000|(\sreg)|(.L__gpr_num_\rt))
	.endm

// Preprocessor condition preserved from C: #else

// Original include: #include <linux/bitfield.h>
// Original include: #include <linux/build_bug.h>
// Original include: #include <linux/types.h>
// Original include: #include <asm/alternative.h>

// C macro requiring assembler/preprocessor context preserved: #define DEFINE_MRS_S						 	__DEFINE_ASM_GPR_NUMS					 "	.macro	mrs_s, rt, sreg\n"				 	__emit_inst(0xd5200000|(\\sreg)|(.L__gpr_num_\\rt))	 "	.endm\n"

// C macro requiring assembler/preprocessor context preserved: #define DEFINE_MSR_S						 	__DEFINE_ASM_GPR_NUMS					 "	.macro	msr_s, sreg, rt\n"				 	__emit_inst(0xd5000000|(\\sreg)|(.L__gpr_num_\\rt))	 "	.endm\n"

// C macro requiring assembler/preprocessor context preserved: #define UNDEFINE_MRS_S						 "	.purgem	mrs_s\n"

// C macro requiring assembler/preprocessor context preserved: #define UNDEFINE_MSR_S						 "	.purgem	msr_s\n"

// C macro requiring assembler/preprocessor context preserved: #define __mrs_s(v, r)						 	DEFINE_MRS_S						 "	mrs_s " v ", " __stringify(r) "\n"			 	UNDEFINE_MRS_S

// C macro requiring assembler/preprocessor context preserved: #define __msr_s(r, v)						 	DEFINE_MSR_S						 "	msr_s " __stringify(r) ", " v "\n"			 	UNDEFINE_MSR_S

/*
 * Unlike read_cpuid, calls to read_sysreg are never expected to be
 * optimized away or replaced with synthetic values.
 */
// C macro requiring assembler/preprocessor context preserved: #define read_sysreg(r) ({					 	u64 __val;						 	asm volatile("mrs %0, " __stringify(r) : "=r" (__val));	 	__val;							 })

/*
 * The "Z" constraint normally means a zero immediate, but when combined with
 * the "%x0" template means XZR.
 */
// C macro requiring assembler/preprocessor context preserved: #define write_sysreg(v, r) do {					 	u64 __val = (u64)(v);					 	asm volatile("msr " __stringify(r) ", %x0"		 		     : : "rZ" (__val));				 } while (0)

/*
 * For registers without architectural names, or simply unsupported by
 * GAS.
 *
 * __check_r forces warnings to be generated by the compiler when
 * evaluating r which wouldn't normally happen due to being passed to
 * the assembler via __stringify(r).
 */
// C macro requiring assembler/preprocessor context preserved: #define read_sysreg_s(r) ({						 	u64 __val;							 	u32 __maybe_unused __check_r = (u32)(r);			 	asm volatile(__mrs_s("%0", r) : "=r" (__val));			 	__val;								 })

// C macro requiring assembler/preprocessor context preserved: #define write_sysreg_s(v, r) do {					 	u64 __val = (u64)(v);						 	u32 __maybe_unused __check_r = (u32)(r);			 	asm volatile(__msr_s(r, "%x0") : : "rZ" (__val));		 } while (0)

/*
 * Modify bits in a sysreg. Bits in the clear mask are zeroed, then bits in the
 * set mask are set. Other bits are left as-is.
 */
// C macro requiring assembler/preprocessor context preserved: #define sysreg_clear_set(sysreg, clear, set) do {			 	u64 __scs_val = read_sysreg(sysreg);				 	u64 __scs_new = (__scs_val & ~(u64)(clear)) | (set);		 	if (__scs_new != __scs_val)					 		write_sysreg(__scs_new, sysreg);			 } while (0)

// C macro requiring assembler/preprocessor context preserved: #define sysreg_clear_set_s(sysreg, clear, set) do {			 	u64 __scs_val = read_sysreg_s(sysreg);				 	u64 __scs_new = (__scs_val & ~(u64)(clear)) | (set);		 	if (__scs_new != __scs_val)					 		write_sysreg_s(__scs_new, sysreg);			 } while (0)

// C macro requiring assembler/preprocessor context preserved: #define read_sysreg_par() ({						 	u64 par;							 	asm(ALTERNATIVE("nop", "dmb sy", ARM64_WORKAROUND_1508412));	 	par = read_sysreg(par_el1);					 	asm(ALTERNATIVE("nop", "dmb sy", ARM64_WORKAROUND_1508412));	 	par;								 })

// C macro requiring assembler/preprocessor context preserved: #define SYS_FIELD_VALUE(reg, field, val)	reg##_##field##_##val

// C macro requiring assembler/preprocessor context preserved: #define SYS_FIELD_GET(reg, field, val)		 		 FIELD_GET(reg##_##field##_MASK, val)

// C macro requiring assembler/preprocessor context preserved: #define SYS_FIELD_PREP(reg, field, val)		 		 FIELD_PREP(reg##_##field##_MASK, val)

// C macro requiring assembler/preprocessor context preserved: #define SYS_FIELD_PREP_ENUM(reg, field, val)		 		 FIELD_PREP(reg##_##field##_MASK,	 			    SYS_FIELD_VALUE(reg, field, val))

// Preprocessor condition preserved from C: #endif


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
