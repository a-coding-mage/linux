/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2009 Freescale Semiconductor, Inc.
 *
 * provides masks and opcode images for use by code generation, emulation
 * and for instructions that older assemblers might not know about
 */


// #define	__REG_R0	0
// #define	__REG_R1	1
// #define	__REG_R2	2
// #define	__REG_R3	3
// #define	__REG_R4	4
// #define	__REG_R5	5
// #define	__REG_R6	6
// #define	__REG_R7	7
// #define	__REG_R8	8
// #define	__REG_R9	9
// #define	__REG_R10	10
// #define	__REG_R11	11
// #define	__REG_R12	12
// #define	__REG_R13	13
// #define	__REG_R14	14
// #define	__REG_R15	15
// #define	__REG_R16	16
// #define	__REG_R17	17
// #define	__REG_R18	18
// #define	__REG_R19	19
// #define	__REG_R20	20
// #define	__REG_R21	21
// #define	__REG_R22	22
// #define	__REG_R23	23
// #define	__REG_R24	24
// #define	__REG_R25	25
// #define	__REG_R26	26
// #define	__REG_R27	27
// #define	__REG_R28	28
// #define	__REG_R29	29
// #define	__REG_R30	30
// #define	__REG_R31	31

// #define	__REGA0_0	0
// #define	__REGA0_R1	1
// #define	__REGA0_R2	2
// #define	__REGA0_R3	3
// #define	__REGA0_R4	4
// #define	__REGA0_R5	5
// #define	__REGA0_R6	6
// #define	__REGA0_R7	7
// #define	__REGA0_R8	8
// #define	__REGA0_R9	9
// #define	__REGA0_R10	10
// #define	__REGA0_R11	11
// #define	__REGA0_R12	12
// #define	__REGA0_R13	13
// #define	__REGA0_R14	14
// #define	__REGA0_R15	15
// #define	__REGA0_R16	16
// #define	__REGA0_R17	17
// #define	__REGA0_R18	18
// #define	__REGA0_R19	19
// #define	__REGA0_R20	20
// #define	__REGA0_R21	21
// #define	__REGA0_R22	22
// #define	__REGA0_R23	23
// #define	__REGA0_R24	24
// #define	__REGA0_R25	25
// #define	__REGA0_R26	26
// #define	__REGA0_R27	27
// #define	__REGA0_R28	28
// #define	__REGA0_R29	29
// #define	__REGA0_R30	30
// #define	__REGA0_R31	31

/* For use with PPC_RAW_() macros */
// #define	_R0	0
// #define	_R1	1
// #define	_R2	2
// #define	_R3	3
// #define	_R4	4
// #define	_R5	5
// #define	_R6	6
// #define	_R7	7
// #define	_R8	8
// #define	_R9	9
// #define	_R10	10
// #define	_R11	11
// #define	_R12	12
// #define	_R13	13
// #define	_R14	14
// #define	_R15	15
// #define	_R16	16
// #define	_R17	17
// #define	_R18	18
// #define	_R19	19
// #define	_R20	20
// #define	_R21	21
// #define	_R22	22
// #define	_R23	23
// #define	_R24	24
// #define	_R25	25
// #define	_R26	26
// #define	_R27	27
// #define	_R28	28
// #define	_R29	29
// #define	_R30	30
// #define	_R31	31

macro_rules! IMM_L {
    ($i:expr) => { ((usize)(i) & 0xffff) };
}
macro_rules! IMM_DS {
    ($i:expr) => { ((usize)(i) & 0xfffc) };
}
macro_rules! IMM_DQ {
    ($i:expr) => { ((usize)(i) & 0xfff0) };
}
macro_rules! IMM_D0 {
    ($i:expr) => { (((usize)(i) >> 16) & 0x3ffff) };
}
macro_rules! IMM_D1 {
    ($i:expr) => { IMM_L(i) };
}

/*
 * 16-bit immediate helper macros: HA() is for use with sign-extending instrs
 * (e.g. LD, ADDI).  If the bottom 16 bits is "-ve", add another bit into the
 * top half to negate the effect (i.e. 0xffff + 1 = 0x(1)0000).
 *
 * XXX: should these mask out possible sign bits?
 */
macro_rules! IMM_H {
    ($i:expr) => { ((usize)(i)>>16) };
}
// #define IMM_HA(i)               (((uintptr_t)(i)>>16) +                       \
					(((uintptr_t)(i) & 0x8000) >> 15))

/*
 * 18-bit immediate helper for prefix 18-bit upper immediate si0 field.
 */
macro_rules! IMM_H18 {
    ($i:expr) => { (((usize)(i)>>16) & 0x3ffff) };
}


/* opcode and xopcode for instructions */
macro_rules! OP_PREFIX { () => { 1 }; }
macro_rules! OP_TRAP_64 { () => { 2 }; }
macro_rules! OP_TRAP { () => { 3 }; }
macro_rules! OP_SC { () => { 17 }; }
macro_rules! OP_19 { () => { 19 }; }
macro_rules! OP_31 { () => { 31 }; }
macro_rules! OP_LWZ { () => { 32 }; }
macro_rules! OP_LWZU { () => { 33 }; }
macro_rules! OP_LBZ { () => { 34 }; }
macro_rules! OP_LBZU { () => { 35 }; }
macro_rules! OP_STW { () => { 36 }; }
macro_rules! OP_STWU { () => { 37 }; }
macro_rules! OP_STB { () => { 38 }; }
macro_rules! OP_STBU { () => { 39 }; }
macro_rules! OP_LHZ { () => { 40 }; }
macro_rules! OP_LHZU { () => { 41 }; }
macro_rules! OP_LHA { () => { 42 }; }
macro_rules! OP_LHAU { () => { 43 }; }
macro_rules! OP_STH { () => { 44 }; }
macro_rules! OP_STHU { () => { 45 }; }
macro_rules! OP_LMW { () => { 46 }; }
macro_rules! OP_STMW { () => { 47 }; }
macro_rules! OP_LFS { () => { 48 }; }
macro_rules! OP_LFSU { () => { 49 }; }
macro_rules! OP_LFD { () => { 50 }; }
macro_rules! OP_LFDU { () => { 51 }; }
macro_rules! OP_STFS { () => { 52 }; }
macro_rules! OP_STFSU { () => { 53 }; }
macro_rules! OP_STFD { () => { 54 }; }
macro_rules! OP_STFDU { () => { 55 }; }
macro_rules! OP_LQ { () => { 56 }; }
macro_rules! OP_LD { () => { 58 }; }
macro_rules! OP_STD { () => { 62 }; }

macro_rules! OP_19_XOP_RFID { () => { 18 }; }
macro_rules! OP_19_XOP_RFMCI { () => { 38 }; }
macro_rules! OP_19_XOP_RFDI { () => { 39 }; }
macro_rules! OP_19_XOP_RFI { () => { 50 }; }
macro_rules! OP_19_XOP_RFCI { () => { 51 }; }
macro_rules! OP_19_XOP_RFSCV { () => { 82 }; }
macro_rules! OP_19_XOP_HRFID { () => { 274 }; }
macro_rules! OP_19_XOP_URFID { () => { 306 }; }
macro_rules! OP_19_XOP_STOP { () => { 370 }; }
macro_rules! OP_19_XOP_DOZE { () => { 402 }; }
macro_rules! OP_19_XOP_NAP { () => { 434 }; }
macro_rules! OP_19_XOP_SLEEP { () => { 466 }; }
macro_rules! OP_19_XOP_RVWINKLE { () => { 498 }; }

macro_rules! OP_31_XOP_TRAP { () => { 4 }; }
macro_rules! OP_31_XOP_LDX { () => { 21 }; }
macro_rules! OP_31_XOP_LWZX { () => { 23 }; }
macro_rules! OP_31_XOP_LDUX { () => { 53 }; }
macro_rules! OP_31_XOP_DCBST { () => { 54 }; }
macro_rules! OP_31_XOP_LWZUX { () => { 55 }; }
macro_rules! OP_31_XOP_TRAP_64 { () => { 68 }; }
macro_rules! OP_31_XOP_DCBF { () => { 86 }; }
macro_rules! OP_31_XOP_LBZX { () => { 87 }; }
macro_rules! OP_31_XOP_STDX { () => { 149 }; }
macro_rules! OP_31_XOP_STWX { () => { 151 }; }
macro_rules! OP_31_XOP_STDUX { () => { 181 }; }
macro_rules! OP_31_XOP_STWUX { () => { 183 }; }
macro_rules! OP_31_XOP_STBX { () => { 215 }; }
macro_rules! OP_31_XOP_LBZUX { () => { 119 }; }
macro_rules! OP_31_XOP_STBUX { () => { 247 }; }
macro_rules! OP_31_XOP_LHZX { () => { 279 }; }
macro_rules! OP_31_XOP_LHZUX { () => { 311 }; }
macro_rules! OP_31_XOP_MSGSNDP { () => { 142 }; }
macro_rules! OP_31_XOP_MSGCLRP { () => { 174 }; }
macro_rules! OP_31_XOP_MTMSR { () => { 146 }; }
macro_rules! OP_31_XOP_MTMSRD { () => { 178 }; }
macro_rules! OP_31_XOP_TLBIE { () => { 306 }; }
macro_rules! OP_31_XOP_MFSPR { () => { 339 }; }
macro_rules! OP_31_XOP_LWAX { () => { 341 }; }
macro_rules! OP_31_XOP_LHAX { () => { 343 }; }
macro_rules! OP_31_XOP_LWAUX { () => { 373 }; }
macro_rules! OP_31_XOP_LHAUX { () => { 375 }; }
macro_rules! OP_31_XOP_STHX { () => { 407 }; }
macro_rules! OP_31_XOP_STHUX { () => { 439 }; }
macro_rules! OP_31_XOP_MTSPR { () => { 467 }; }
macro_rules! OP_31_XOP_DCBI { () => { 470 }; }
macro_rules! OP_31_XOP_LDBRX { () => { 532 }; }
macro_rules! OP_31_XOP_LWBRX { () => { 534 }; }
macro_rules! OP_31_XOP_TLBSYNC { () => { 566 }; }
macro_rules! OP_31_XOP_STDBRX { () => { 660 }; }
macro_rules! OP_31_XOP_STWBRX { () => { 662 }; }
macro_rules! OP_31_XOP_STFSX { () => { 663 }; }
macro_rules! OP_31_XOP_STFSUX { () => { 695 }; }
macro_rules! OP_31_XOP_STFDX { () => { 727 }; }
macro_rules! OP_31_XOP_HASHCHK { () => { 754 }; }
macro_rules! OP_31_XOP_STFDUX { () => { 759 }; }
macro_rules! OP_31_XOP_LHBRX { () => { 790 }; }
macro_rules! OP_31_XOP_LFIWAX { () => { 855 }; }
macro_rules! OP_31_XOP_LFIWZX { () => { 887 }; }
macro_rules! OP_31_XOP_STHBRX { () => { 918 }; }
macro_rules! OP_31_XOP_STFIWX { () => { 983 }; }

/* VSX Scalar Load Instructions */
macro_rules! OP_31_XOP_LXSDX { () => { 588 }; }
macro_rules! OP_31_XOP_LXSSPX { () => { 524 }; }
macro_rules! OP_31_XOP_LXSIWAX { () => { 76 }; }
macro_rules! OP_31_XOP_LXSIWZX { () => { 12 }; }

/* VSX Scalar Store Instructions */
macro_rules! OP_31_XOP_STXSDX { () => { 716 }; }
macro_rules! OP_31_XOP_STXSSPX { () => { 652 }; }
macro_rules! OP_31_XOP_STXSIWX { () => { 140 }; }

/* VSX Vector Load Instructions */
macro_rules! OP_31_XOP_LXVD2X { () => { 844 }; }
macro_rules! OP_31_XOP_LXVW4X { () => { 780 }; }

/* VSX Vector Load and Splat Instruction */
macro_rules! OP_31_XOP_LXVDSX { () => { 332 }; }

/* VSX Vector Store Instructions */
macro_rules! OP_31_XOP_STXVD2X { () => { 972 }; }
macro_rules! OP_31_XOP_STXVW4X { () => { 908 }; }

macro_rules! OP_31_XOP_LFSX { () => { 535 }; }
macro_rules! OP_31_XOP_LFSUX { () => { 567 }; }
macro_rules! OP_31_XOP_LFDX { () => { 599 }; }
macro_rules! OP_31_XOP_LFDUX { () => { 631 }; }

/* VMX Vector Load Instructions */
macro_rules! OP_31_XOP_LVX { () => { 103 }; }

/* VMX Vector Store Instructions */
macro_rules! OP_31_XOP_STVX { () => { 231 }; }

/* sorted alphabetically */
macro_rules! PPC_INST_BCCTR_FLUSH { () => { 0x4c400420 }; }
macro_rules! PPC_INST_COPY { () => { 0x7c20060c }; }
macro_rules! PPC_INST_DCBA { () => { 0x7c0005ec }; }
macro_rules! PPC_INST_DCBA_MASK { () => { 0xfc0007fe }; }
macro_rules! PPC_INST_DSSALL { () => { 0x7e00066c }; }
macro_rules! PPC_INST_ISEL { () => { 0x7c00001e }; }
macro_rules! PPC_INST_ISEL_MASK { () => { 0xfc00003e }; }
macro_rules! PPC_INST_LSWI { () => { 0x7c0004aa }; }
macro_rules! PPC_INST_LSWX { () => { 0x7c00042a }; }
macro_rules! PPC_INST_LWSYNC { () => { 0x7c2004ac }; }
macro_rules! PPC_INST_SYNC { () => { 0x7c0004ac }; }
macro_rules! PPC_INST_SYNC_MASK { () => { 0xfc0007fe }; }
macro_rules! PPC_INST_MCRXR { () => { 0x7c000400 }; }
macro_rules! PPC_INST_MCRXR_MASK { () => { 0xfc0007fe }; }
macro_rules! PPC_INST_MFSPR_PVR { () => { 0x7c1f42a6 }; }
macro_rules! PPC_INST_MFSPR_PVR_MASK { () => { 0xfc1ffffe }; }
macro_rules! PPC_INST_MTMSRD { () => { 0x7c000164 }; }
macro_rules! PPC_INST_PASTE { () => { 0x7c20070d }; }
macro_rules! PPC_INST_PASTE_MASK { () => { 0xfc2007ff }; }
macro_rules! PPC_INST_POPCNTB { () => { 0x7c0000f4 }; }
macro_rules! PPC_INST_POPCNTB_MASK { () => { 0xfc0007fe }; }
macro_rules! PPC_INST_RFEBB { () => { 0x4c000124 }; }
macro_rules! PPC_INST_RFID { () => { 0x4c000024 }; }
macro_rules! PPC_INST_MFSPR_DSCR { () => { 0x7c1102a6 }; }
macro_rules! PPC_INST_MFSPR_DSCR_MASK { () => { 0xfc1ffffe }; }
macro_rules! PPC_INST_MTSPR_DSCR { () => { 0x7c1103a6 }; }
macro_rules! PPC_INST_MTSPR_DSCR_MASK { () => { 0xfc1ffffe }; }
macro_rules! PPC_INST_MFSPR_DSCR_USER { () => { 0x7c0302a6 }; }
macro_rules! PPC_INST_MFSPR_DSCR_USER_MASK { () => { 0xfc1ffffe }; }
macro_rules! PPC_INST_MTSPR_DSCR_USER { () => { 0x7c0303a6 }; }
macro_rules! PPC_INST_MTSPR_DSCR_USER_MASK { () => { 0xfc1ffffe }; }
macro_rules! PPC_INST_STRING { () => { 0x7c00042a }; }
macro_rules! PPC_INST_STRING_MASK { () => { 0xfc0007fe }; }
macro_rules! PPC_INST_STRING_GEN_MASK { () => { 0xfc00067e }; }
macro_rules! PPC_INST_STSWI { () => { 0x7c0005aa }; }
macro_rules! PPC_INST_STSWX { () => { 0x7c00052a }; }
macro_rules! PPC_INST_TRECHKPT { () => { 0x7c0007dd }; }
macro_rules! PPC_INST_TRECLAIM { () => { 0x7c00075d }; }
macro_rules! PPC_INST_TSR { () => { 0x7c0005dd }; }
macro_rules! PPC_INST_BRANCH_COND { () => { 0x40800000 }; }

/* Prefixes */
macro_rules! PPC_INST_LFS { () => { 0xc0000000 }; }
macro_rules! PPC_INST_STFS { () => { 0xd0000000 }; }
macro_rules! PPC_INST_LFD { () => { 0xc8000000 }; }
macro_rules! PPC_INST_STFD { () => { 0xd8000000 }; }
macro_rules! PPC_PREFIX_MLS { () => { 0x06000000 }; }
macro_rules! PPC_PREFIX_8LS { () => { 0x04000000 }; }

/* Prefixed instructions */
macro_rules! PPC_INST_PADDI { () => { 0x38000000 }; }
macro_rules! PPC_INST_PLD { () => { 0xe4000000 }; }
macro_rules! PPC_INST_PSTD { () => { 0xf4000000 }; }

/* macros to insert fields into opcodes */
macro_rules! ___PPC_RA {
    ($a:expr) => { (((a) & 0x1f) << 16) };
}
macro_rules! ___PPC_RB {
    ($b:expr) => { (((b) & 0x1f) << 11) };
}
macro_rules! ___PPC_RC {
    ($c:expr) => { (((c) & 0x1f) << 6) };
}
macro_rules! ___PPC_RS {
    ($s:expr) => { (((s) & 0x1f) << 21) };
}
macro_rules! ___PPC_RT {
    ($t:expr) => { ___PPC_RS(t) };
}
macro_rules! ___PPC_R {
    ($r:expr) => { (((r) & 0x1) << 16) };
}
macro_rules! ___PPC_PRS {
    ($prs:expr) => { (((prs) & 0x1) << 17) };
}
macro_rules! ___PPC_RIC {
    ($ric:expr) => { (((ric) & 0x3) << 18) };
}
macro_rules! __PPC_RA {
    ($a:expr) => { ___PPC_RA(__REG_##a) };
}
macro_rules! __PPC_RA0 {
    ($a:expr) => { ___PPC_RA(__REGA0_##a) };
}
macro_rules! __PPC_RB {
    ($b:expr) => { ___PPC_RB(__REG_##b) };
}
macro_rules! __PPC_RS {
    ($s:expr) => { ___PPC_RS(__REG_##s) };
}
macro_rules! __PPC_RT {
    ($t:expr) => { ___PPC_RT(__REG_##t) };
}
macro_rules! __PPC_XA {
    ($a:expr) => { ((((a) & 0x1f) << 16) | (((a) & 0x20) >> 3)) };
}
macro_rules! __PPC_XB {
    ($b:expr) => { ((((b) & 0x1f) << 11) | (((b) & 0x20) >> 4)) };
}
macro_rules! __PPC_XS {
    ($s:expr) => { ((((s) & 0x1f) << 21) | (((s) & 0x20) >> 5)) };
}
macro_rules! __PPC_XT {
    ($s:expr) => { __PPC_XS(s) };
}
macro_rules! __PPC_XSP {
    ($s:expr) => { ((((s) & 0x1e) | (((s) >> 5) & 0x1)) << 21) };
}
macro_rules! __PPC_XTP {
    ($s:expr) => { __PPC_XSP(s) };
}
macro_rules! __PPC_T_TLB {
    ($t:expr) => { (((t) & 0x3) << 21) };
}
macro_rules! __PPC_PL {
    ($p:expr) => { (((p) & 0x3) << 16) };
}
macro_rules! __PPC_WC {
    ($w:expr) => { (((w) & 0x3) << 21) };
}
macro_rules! __PPC_WS {
    ($w:expr) => { (((w) & 0x1f) << 11) };
}
macro_rules! __PPC_SH {
    ($s:expr) => { __PPC_WS(s) };
}
macro_rules! __PPC_SH64 {
    ($s:expr) => { (__PPC_SH(s) | (((s) & 0x20) >> 4)) };
}
macro_rules! __PPC_MB {
    ($s:expr) => { ___PPC_RC(s) };
}
macro_rules! __PPC_ME {
    ($s:expr) => { (((s) & 0x1f) << 1) };
}
macro_rules! __PPC_MB64 {
    ($s:expr) => { (__PPC_MB(s) | ((s) & 0x20)) };
}
macro_rules! __PPC_ME64 {
    ($s:expr) => { __PPC_MB64(s) };
}
macro_rules! __PPC_BI {
    ($s:expr) => { (((s) & 0x1f) << 16) };
}
macro_rules! __PPC_CT {
    ($t:expr) => { (((t) & 0x0f) << 21) };
}
macro_rules! __PPC_SPR {
    ($r:expr) => { ((((r) & 0x1f) << 16) | ((((r) >> 5) & 0x1f) << 11)) };
}
macro_rules! __PPC_RC21 { () => { (0x1 << 10) }; }
macro_rules! __PPC_PRFX_R {
    ($r:expr) => { (((r) & 0x1) << 20) };
}
macro_rules! __PPC_EH {
    ($eh:expr) => { (((eh) & 0x1) << 0) };
}

/*
 * Both low and high 16 bits are added as SIGNED additions, so if low 16 bits
 * has high bit set, high 16 bits must be adjusted. These macros do that (stolen
 * from binutils).
 */
macro_rules! PPC_LO {
    ($v:expr) => { ((v) & 0xffff) };
}
macro_rules! PPC_HI {
    ($v:expr) => { (((v) >> 16) & 0xffff) };
}
macro_rules! PPC_HA {
    ($v:expr) => { PPC_HI((v) + 0x8000) };
}
macro_rules! PPC_HIGHER {
    ($v:expr) => { (((v) >> 32) & 0xffff) };
}
macro_rules! PPC_HIGHEST {
    ($v:expr) => { (((v) >> 48) & 0xffff) };
}

/* LI Field */
macro_rules! PPC_LI_MASK { () => { 0x03fffffc }; }
macro_rules! PPC_LI {
    ($v:expr) => { ((v) & PPC_LI_MASK) };
}

/* Base instruction encoding */
macro_rules! PPC_RAW_CP_ABORT { () => { (0x7c00068c) }; }
macro_rules! PPC_RAW_COPY {
    ($a:expr, $:expr) => { b)		(PPC_INST_COPY | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_DARN {
    ($t:expr, $:expr) => { l)		(0x7c0005e6 | ___PPC_RT(t) | (((l) & 0x3) << 16)) };
}
macro_rules! PPC_RAW_DCBAL {
    ($a:expr, $:expr) => { b)		(0x7c2005ec | __PPC_RA(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_DCBZL {
    ($a:expr, $:expr) => { b)		(0x7c2007ec | __PPC_RA(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_LQARX {
    ($t:expr, $:expr) => { a, b, eh)	(0x7c000228 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | __PPC_EH(eh)) };
}
macro_rules! PPC_RAW_LDARX {
    ($t:expr, $:expr) => { a, b, eh)	(0x7c0000a8 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | __PPC_EH(eh)) };
}
macro_rules! PPC_RAW_LWARX {
    ($t:expr, $:expr) => { a, b, eh)	(0x7c000028 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | __PPC_EH(eh)) };
}
macro_rules! PPC_RAW_PHWSYNC { () => { (0x7c8004ac) }; }
macro_rules! PPC_RAW_PLWSYNC { () => { (0x7ca004ac) }; }
macro_rules! PPC_RAW_STQCX {
    ($t:expr, $:expr) => { a, b)		(0x7c00016d | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MADDHD {
    ($t:expr, $:expr) => { a, b, c)	(0x10000030 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | ___PPC_RC(c)) };
}
macro_rules! PPC_RAW_MADDHDU {
    ($t:expr, $:expr) => { a, b, c)	(0x10000031 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | ___PPC_RC(c)) };
}
macro_rules! PPC_RAW_MADDLD {
    ($t:expr, $:expr) => { a, b, c)	(0x10000033 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | ___PPC_RC(c)) };
}
macro_rules! PPC_RAW_MSGSND {
    ($b:expr) => { (0x7c00019c | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MSGSYNC { () => { (0x7c0006ec) }; }
macro_rules! PPC_RAW_MSGCLR {
    ($b:expr) => { (0x7c0001dc | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MSGSNDP {
    ($b:expr) => { (0x7c00011c | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MSGCLRP {
    ($b:expr) => { (0x7c00015c | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_PASTE {
    ($a:expr, $:expr) => { b)		(0x7c20070d | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_POPCNTB {
    ($a:expr, $:expr) => { s)		(PPC_INST_POPCNTB | __PPC_RA(a) | __PPC_RS(s)) };
}
macro_rules! PPC_RAW_POPCNTD {
    ($a:expr, $:expr) => { s)		(0x7c0003f4 | __PPC_RA(a) | __PPC_RS(s)) };
}
macro_rules! PPC_RAW_POPCNTW {
    ($a:expr, $:expr) => { s)		(0x7c0002f4 | __PPC_RA(a) | __PPC_RS(s)) };
}
macro_rules! PPC_RAW_RFCI { () => { (0x4c000066) }; }
macro_rules! PPC_RAW_RFDI { () => { (0x4c00004e) }; }
macro_rules! PPC_RAW_RFMCI { () => { (0x4c00004c) }; }
macro_rules! PPC_RAW_TLBILX_LPID { () => { (0x7c000024) }; }
macro_rules! PPC_RAW_TLBILX {
    ($t:expr, $:expr) => { a, b)		(0x7c000024 | __PPC_T_TLB(t) | 	__PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_WAIT_v203 { () => { (0x7c00007c) }; }
macro_rules! PPC_RAW_WAIT {
    ($w:expr, $:expr) => { p)		(0x7c00003c | __PPC_WC(w) | __PPC_PL(p)) };
}
macro_rules! PPC_RAW_TLBIE {
    ($lp:expr, $:expr) => { a)		(0x7c000264 | ___PPC_RB(a) | ___PPC_RS(lp)) };
}
// #define PPC_RAW_TLBIE_5(rb, rs, ric, prs, r) \
	(0x7c000264 | ___PPC_RB(rb) | ___PPC_RS(rs) | ___PPC_RIC(ric) | ___PPC_PRS(prs) | ___PPC_R(r))
// #define PPC_RAW_TLBIEL(rb, rs, ric, prs, r) \
	(0x7c000224 | ___PPC_RB(rb) | ___PPC_RS(rs) | ___PPC_RIC(ric) | ___PPC_PRS(prs) | ___PPC_R(r))
macro_rules! PPC_RAW_TLBIEL_v205 {
    ($rb:expr, $:expr) => { l)	(0x7c000224 | ___PPC_RB(rb) | (l << 21)) };
}
macro_rules! PPC_RAW_TLBSRX_DOT {
    ($a:expr, $:expr) => { b)	(0x7c0006a5 | __PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_TLBIVAX {
    ($a:expr, $:expr) => { b)		(0x7c000624 | __PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_ERATWE {
    ($s:expr, $:expr) => { a, w)		(0x7c0001a6 | __PPC_RS(s) | __PPC_RA(a) | __PPC_WS(w)) };
}
macro_rules! PPC_RAW_ERATRE {
    ($s:expr, $:expr) => { a, w)		(0x7c000166 | __PPC_RS(s) | __PPC_RA(a) | __PPC_WS(w)) };
}
macro_rules! PPC_RAW_ERATILX {
    ($t:expr, $:expr) => { a, b)	(0x7c000066 | __PPC_T_TLB(t) | __PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_ERATIVAX {
    ($s:expr, $:expr) => { a, b)	(0x7c000666 | __PPC_RS(s) | __PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_ERATSX {
    ($t:expr, $:expr) => { a, w)		(0x7c000126 | __PPC_RS(t) | __PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_ERATSX_DOT {
    ($t:expr, $:expr) => { a, w)	(0x7c000127 | __PPC_RS(t) | __PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_SLBFEE_DOT {
    ($t:expr, $:expr) => { b)	(0x7c0007a7 | __PPC_RT(t) | __PPC_RB(b)) };
}
macro_rules! __PPC_RAW_SLBFEE_DOT {
    ($t:expr, $:expr) => { b)	(0x7c0007a7 | ___PPC_RT(t) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_ICBT {
    ($c:expr, $:expr) => { a, b)		(0x7c00002c | __PPC_CT(c) | __PPC_RA0(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_LBZCIX {
    ($t:expr, $:expr) => { a, b)		(0x7c0006aa | __PPC_RT(t) | __PPC_RA(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_STBCIX {
    ($s:expr, $:expr) => { a, b)		(0x7c0007aa | __PPC_RS(s) | __PPC_RA(a) | __PPC_RB(b)) };
}
macro_rules! PPC_RAW_DCBFPS {
    ($a:expr, $:expr) => { b)		(0x7c0000ac | ___PPC_RA(a) | ___PPC_RB(b) | (4 << 21)) };
}
macro_rules! PPC_RAW_DCBSTPS {
    ($a:expr, $:expr) => { b)		(0x7c0000ac | ___PPC_RA(a) | ___PPC_RB(b) | (6 << 21)) };
}
macro_rules! PPC_RAW_SC {
    () => { (0x44000002) };
}
macro_rules! PPC_RAW_SYNC {
    () => { (0x7c0004ac) };
}
macro_rules! PPC_RAW_ISYNC {
    () => { (0x4c00012c) };
}
macro_rules! PPC_RAW_LWSYNC {
    () => { (0x7c2004ac) };
}

/*
 * Define what the VSX XX1 form instructions will look like, then add
 * the 128 bit load store instructions based on that.
 */
macro_rules! VSX_XX1 {
    ($s:expr, $:expr) => { a, b)		(__PPC_XS(s) | __PPC_RA(a) | __PPC_RB(b)) };
}
macro_rules! VSX_XX3 {
    ($t:expr, $:expr) => { a, b)		(__PPC_XT(t) | __PPC_XA(a) | __PPC_XB(b)) };
}
macro_rules! PPC_RAW_STXVD2X {
    ($s:expr, $:expr) => { a, b)	(0x7c000798 | VSX_XX1((s), a, b)) };
}
macro_rules! PPC_RAW_LXVD2X {
    ($s:expr, $:expr) => { a, b)		(0x7c000698 | VSX_XX1((s), a, b)) };
}
macro_rules! PPC_RAW_MFVRD {
    ($a:expr, $:expr) => { t)		(0x7c000066 | VSX_XX1((t) + 32, a, R0)) };
}
macro_rules! PPC_RAW_MTVRD {
    ($t:expr, $:expr) => { a)		(0x7c000166 | VSX_XX1((t) + 32, a, R0)) };
}
macro_rules! PPC_RAW_VPMSUMW {
    ($t:expr, $:expr) => { a, b)	(0x10000488 | VSX_XX3((t), a, b)) };
}
macro_rules! PPC_RAW_VPMSUMD {
    ($t:expr, $:expr) => { a, b)	(0x100004c8 | VSX_XX3((t), a, b)) };
}
macro_rules! PPC_RAW_XXLOR {
    ($t:expr, $:expr) => { a, b)		(0xf0000490 | VSX_XX3((t), a, b)) };
}
macro_rules! PPC_RAW_XXSWAPD {
    ($t:expr, $:expr) => { a)		(0xf0000250 | VSX_XX3((t), a, a)) };
}
macro_rules! PPC_RAW_XVCPSGNDP {
    ($t:expr, $:expr) => { a, b)	((0xf0000780 | VSX_XX3((t), (a), (b)))) };
}
// #define PPC_RAW_VPERMXOR(vrt, vra, vrb, vrc) \
	((0x1000002d | ___PPC_RT(vrt) | ___PPC_RA(vra) | ___PPC_RB(vrb) | (((vrc) & 0x1f) << 6)))
macro_rules! PPC_RAW_LXVP {
    ($xtp:expr, $:expr) => { a, i)		(0x18000000 | __PPC_XTP(xtp) | ___PPC_RA(a) | IMM_DQ(i)) };
}
macro_rules! PPC_RAW_STXVP {
    ($xsp:expr, $:expr) => { a, i)	(0x18000001 | __PPC_XSP(xsp) | ___PPC_RA(a) | IMM_DQ(i)) };
}
macro_rules! PPC_RAW_LXVPX {
    ($xtp:expr, $:expr) => { a, b)	(0x7c00029a | __PPC_XTP(xtp) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_STXVPX {
    ($xsp:expr, $:expr) => { a, b)	(0x7c00039a | __PPC_XSP(xsp) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_PLXVP_P {
    ($xtp:expr, $:expr) => { i, a, pr)	(PPC_PREFIX_8LS | __PPC_PRFX_R(pr) | IMM_D0(i)) };
}
macro_rules! PPC_RAW_PLXVP_S {
    ($xtp:expr, $:expr) => { i, a, pr)	(0xe8000000 | __PPC_XTP(xtp) | ___PPC_RA(a) | IMM_D1(i)) };
}
macro_rules! PPC_RAW_PSTXVP_P {
    ($xsp:expr, $:expr) => { i, a, pr)	(PPC_PREFIX_8LS | __PPC_PRFX_R(pr) | IMM_D0(i)) };
}
macro_rules! PPC_RAW_PSTXVP_S {
    ($xsp:expr, $:expr) => { i, a, pr)	(0xf8000000 | __PPC_XSP(xsp) | ___PPC_RA(a) | IMM_D1(i)) };
}
macro_rules! PPC_RAW_NAP { () => { (0x4c000364) }; }
macro_rules! PPC_RAW_SLEEP { () => { (0x4c0003a4) }; }
macro_rules! PPC_RAW_WINKLE { () => { (0x4c0003e4) }; }
macro_rules! PPC_RAW_STOP { () => { (0x4c0002e4) }; }
macro_rules! PPC_RAW_CLRBHRB { () => { (0x7c00035c) }; }
macro_rules! PPC_RAW_MFBHRBE {
    ($r:expr, $:expr) => { n)		(0x7c00025c | __PPC_RT(r) | (((n) & 0x3ff) << 11)) };
}
macro_rules! PPC_RAW_TRECHKPT { () => { (PPC_INST_TRECHKPT) }; }
macro_rules! PPC_RAW_TRECLAIM {
    ($r:expr) => { (PPC_INST_TRECLAIM | __PPC_RA(r)) };
}
macro_rules! PPC_RAW_TABORT {
    ($r:expr) => { (0x7c00071d | __PPC_RA(r)) };
}
macro_rules! TMRN {
    ($x:expr) => { ((((x) & 0x1f) << 16) | (((x) & 0x3e0) << 6)) };
}
macro_rules! PPC_RAW_MTTMR {
    ($tmr:expr, $:expr) => { r)		(0x7c0003dc | TMRN(tmr) | ___PPC_RS(r)) };
}
macro_rules! PPC_RAW_MFTMR {
    ($tmr:expr, $:expr) => { r)		(0x7c0002dc | TMRN(tmr) | ___PPC_RT(r)) };
}
macro_rules! PPC_RAW_ICSWX {
    ($s:expr, $:expr) => { a, b)		(0x7c00032d | ___PPC_RS(s) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_ICSWEPX {
    ($s:expr, $:expr) => { a, b)	(0x7c00076d | ___PPC_RS(s) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_SLBIA {
    ($IH:expr) => { (0x7c0003e4 | (((IH) & 0x7) << 21)) };
}
// #define PPC_RAW_VCMPEQUD_RC(vrt, vra, vrb) \
	(0x100000c7 | ___PPC_RT(vrt) | ___PPC_RA(vra) | ___PPC_RB(vrb) | __PPC_RC21)
// #define PPC_RAW_VCMPEQUB_RC(vrt, vra, vrb) \
	(0x10000006 | ___PPC_RT(vrt) | ___PPC_RA(vra) | ___PPC_RB(vrb) | __PPC_RC21)
macro_rules! PPC_RAW_LD {
    ($r:expr, $:expr) => { base, i)		(0xe8000000 | ___PPC_RT(r) | ___PPC_RA(base) | IMM_DS(i)) };
}
macro_rules! PPC_RAW_LWA {
    ($r:expr, $:expr) => { base, i)		(0xe8000002 | ___PPC_RT(r) | ___PPC_RA(base) | IMM_DS(i)) };
}
macro_rules! PPC_RAW_LWZ {
    ($r:expr, $:expr) => { base, i)		(0x80000000 | ___PPC_RT(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_LWZX {
    ($t:expr, $:expr) => { a, b)		(0x7c00002e | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_STD {
    ($r:expr, $:expr) => { base, i)		(0xf8000000 | ___PPC_RS(r) | ___PPC_RA(base) | IMM_DS(i)) };
}
macro_rules! PPC_RAW_STDCX {
    ($s:expr, $:expr) => { a, b)		(0x7c0001ad | ___PPC_RS(s) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_LFSX {
    ($t:expr, $:expr) => { a, b)		(0x7c00042e | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_STFSX {
    ($s:expr, $:expr) => { a, b)		(0x7c00052e | ___PPC_RS(s) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_LFDX {
    ($t:expr, $:expr) => { a, b)		(0x7c0004ae | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_STFDX {
    ($s:expr, $:expr) => { a, b)		(0x7c0005ae | ___PPC_RS(s) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_LVX {
    ($t:expr, $:expr) => { a, b)		(0x7c0000ce | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_STVX {
    ($s:expr, $:expr) => { a, b)		(0x7c0001ce | ___PPC_RS(s) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_ADDE {
    ($t:expr, $:expr) => { a, b)		(0x7c000114 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_ADDZE {
    ($t:expr, $:expr) => { a)		(0x7c000194 | ___PPC_RT(t) | ___PPC_RA(a)) };
}
macro_rules! PPC_RAW_ADDME {
    ($t:expr, $:expr) => { a)		(0x7c0001d4 | ___PPC_RT(t) | ___PPC_RA(a)) };
}
macro_rules! PPC_RAW_ADD {
    ($t:expr, $:expr) => { a, b)		(0x7c000214 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_ADD_DOT {
    ($t:expr, $:expr) => { a, b)	(0x7c000214 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | 0x1) };
}
macro_rules! PPC_RAW_ADDC {
    ($t:expr, $:expr) => { a, b)		(0x7c000014 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_ADDC_DOT {
    ($t:expr, $:expr) => { a, b)	(0x7c000014 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | 0x1) };
}
macro_rules! PPC_RAW_NOP {
    () => { PPC_RAW_ORI(0, 0, 0) };
}
macro_rules! PPC_RAW_BLR {
    () => { (0x4e800020) };
}
macro_rules! PPC_RAW_BLRL {
    () => { (0x4e800021) };
}
macro_rules! PPC_RAW_MTLR {
    ($r:expr) => { (0x7c0803a6 | ___PPC_RT(r)) };
}
macro_rules! PPC_RAW_MFLR {
    ($t:expr) => { (0x7c0802a6 | ___PPC_RT(t)) };
}
macro_rules! PPC_RAW_BCTR {
    () => { (0x4e800420) };
}
macro_rules! PPC_RAW_BCTRL {
    () => { (0x4e800421) };
}
macro_rules! PPC_RAW_MTCTR {
    ($r:expr) => { (0x7c0903a6 | ___PPC_RT(r)) };
}
macro_rules! PPC_RAW_ADDI {
    ($d:expr, $:expr) => { a, i)		(0x38000000 | ___PPC_RT(d) | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_LI {
    ($r:expr, $:expr) => { i)		PPC_RAW_ADDI(r, 0, i) };
}
macro_rules! PPC_RAW_ADDIS {
    ($d:expr, $:expr) => { a, i)		(0x3c000000 | ___PPC_RT(d) | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_ADDIC {
    ($d:expr, $:expr) => { a, i)		(0x30000000 | ___PPC_RT(d) | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_ADDIC_DOT {
    ($d:expr, $:expr) => { a, i)	(0x34000000 | ___PPC_RT(d) | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_LIS {
    ($r:expr, $:expr) => { i)		PPC_RAW_ADDIS(r, 0, i) };
}
macro_rules! PPC_RAW_STDX {
    ($r:expr, $:expr) => { base, b)	(0x7c00012a | ___PPC_RS(r) | ___PPC_RA(base) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_STDU {
    ($r:expr, $:expr) => { base, i)	(0xf8000001 | ___PPC_RS(r) | ___PPC_RA(base) | ((i) & 0xfffc)) };
}
macro_rules! PPC_RAW_STW {
    ($r:expr, $:expr) => { base, i)		(0x90000000 | ___PPC_RS(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_STWU {
    ($r:expr, $:expr) => { base, i)	(0x94000000 | ___PPC_RS(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_STH {
    ($r:expr, $:expr) => { base, i)		(0xb0000000 | ___PPC_RS(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_STB {
    ($r:expr, $:expr) => { base, i)		(0x98000000 | ___PPC_RS(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_LBZ {
    ($r:expr, $:expr) => { base, i)		(0x88000000 | ___PPC_RT(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_LDX {
    ($r:expr, $:expr) => { base, b)		(0x7c00002a | ___PPC_RT(r) | ___PPC_RA(base) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_LHA {
    ($r:expr, $:expr) => { base, i)		(0xa8000000 | ___PPC_RT(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_LHZ {
    ($r:expr, $:expr) => { base, i)		(0xa0000000 | ___PPC_RT(r) | ___PPC_RA(base) | IMM_L(i)) };
}
macro_rules! PPC_RAW_LHBRX {
    ($r:expr, $:expr) => { base, b)	(0x7c00062c | ___PPC_RT(r) | ___PPC_RA(base) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_LWBRX {
    ($r:expr, $:expr) => { base, b)	(0x7c00042c | ___PPC_RT(r) | ___PPC_RA(base) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_LDBRX {
    ($r:expr, $:expr) => { base, b)	(0x7c000428 | ___PPC_RT(r) | ___PPC_RA(base) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_STWCX {
    ($s:expr, $:expr) => { a, b)		(0x7c00012d | ___PPC_RS(s) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_CMPWI {
    ($a:expr, $:expr) => { i)		(0x2c000000 | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_CMPDI {
    ($a:expr, $:expr) => { i)		(0x2c200000 | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_CMPW {
    ($a:expr, $:expr) => { b)		(0x7c000000 | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_CMPD {
    ($a:expr, $:expr) => { b)		(0x7c200000 | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_CMPLWI {
    ($a:expr, $:expr) => { i)		(0x28000000 | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_CMPLDI {
    ($a:expr, $:expr) => { i)		(0x28200000 | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_CMPLW {
    ($a:expr, $:expr) => { b)		(0x7c000040 | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_CMPLD {
    ($a:expr, $:expr) => { b)		(0x7c200040 | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_SUB {
    ($d:expr, $:expr) => { a, b)		(0x7c000050 | ___PPC_RT(d) | ___PPC_RB(a) | ___PPC_RA(b)) };
}
macro_rules! PPC_RAW_SUBFC {
    ($d:expr, $:expr) => { a, b)		(0x7c000010 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_SUBFE {
    ($d:expr, $:expr) => { a, b)		(0x7c000110 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_SUBFIC {
    ($d:expr, $:expr) => { a, i)		(0x20000000 | ___PPC_RT(d) | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_SUBFZE {
    ($d:expr, $:expr) => { a)		(0x7c000190 | ___PPC_RT(d) | ___PPC_RA(a)) };
}
macro_rules! PPC_RAW_MULD {
    ($d:expr, $:expr) => { a, b)		(0x7c0001d2 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MULW {
    ($d:expr, $:expr) => { a, b)		(0x7c0001d6 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MULHWU {
    ($d:expr, $:expr) => { a, b)		(0x7c000016 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MULI {
    ($d:expr, $:expr) => { a, i)		(0x1c000000 | ___PPC_RT(d) | ___PPC_RA(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_DIVW {
    ($d:expr, $:expr) => { a, b)		(0x7c0003d6 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_DIVWU {
    ($d:expr, $:expr) => { a, b)		(0x7c000396 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_DIVD {
    ($d:expr, $:expr) => { a, b)		(0x7c0003d2 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_DIVDU {
    ($d:expr, $:expr) => { a, b)		(0x7c000392 | ___PPC_RT(d) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_DIVDE {
    ($t:expr, $:expr) => { a, b)		(0x7c000352 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_DIVDE_DOT {
    ($t:expr, $:expr) => { a, b)	(0x7c000352 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | 0x1) };
}
macro_rules! PPC_RAW_DIVDEU {
    ($t:expr, $:expr) => { a, b)		(0x7c000312 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_DIVDEU_DOT {
    ($t:expr, $:expr) => { a, b)	(0x7c000312 | ___PPC_RT(t) | ___PPC_RA(a) | ___PPC_RB(b) | 0x1) };
}
macro_rules! PPC_RAW_AND {
    ($d:expr, $:expr) => { a, b)		(0x7c000038 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_ANDI {
    ($d:expr, $:expr) => { a, i)		(0x70000000 | ___PPC_RA(d) | ___PPC_RS(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_ANDIS {
    ($d:expr, $:expr) => { a, i)		(0x74000000 | ___PPC_RA(d) | ___PPC_RS(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_AND_DOT {
    ($d:expr, $:expr) => { a, b)	(0x7c000039 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_OR {
    ($d:expr, $:expr) => { a, b)		(0x7c000378 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_MR {
    ($d:expr, $:expr) => { a)		PPC_RAW_OR(d, a, a) };
}
macro_rules! PPC_RAW_ORI {
    ($d:expr, $:expr) => { a, i)		(0x60000000 | ___PPC_RA(d) | ___PPC_RS(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_ORIS {
    ($d:expr, $:expr) => { a, i)		(0x64000000 | ___PPC_RA(d) | ___PPC_RS(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_NOR {
    ($d:expr, $:expr) => { a, b)		(0x7c0000f8 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_XOR {
    ($d:expr, $:expr) => { a, b)		(0x7c000278 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_XORI {
    ($d:expr, $:expr) => { a, i)		(0x68000000 | ___PPC_RA(d) | ___PPC_RS(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_XORIS {
    ($d:expr, $:expr) => { a, i)		(0x6c000000 | ___PPC_RA(d) | ___PPC_RS(a) | IMM_L(i)) };
}
macro_rules! PPC_RAW_EXTSB {
    ($d:expr, $:expr) => { a)		(0x7c000774 | ___PPC_RA(d) | ___PPC_RS(a)) };
}
macro_rules! PPC_RAW_EXTSH {
    ($d:expr, $:expr) => { a)		(0x7c000734 | ___PPC_RA(d) | ___PPC_RS(a)) };
}
macro_rules! PPC_RAW_EXTSW {
    ($d:expr, $:expr) => { a)		(0x7c0007b4 | ___PPC_RA(d) | ___PPC_RS(a)) };
}
macro_rules! PPC_RAW_SLW {
    ($d:expr, $:expr) => { a, s)		(0x7c000030 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(s)) };
}
macro_rules! PPC_RAW_SLD {
    ($d:expr, $:expr) => { a, s)		(0x7c000036 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(s)) };
}
macro_rules! PPC_RAW_SRW {
    ($d:expr, $:expr) => { a, s)		(0x7c000430 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(s)) };
}
macro_rules! PPC_RAW_SRAW {
    ($d:expr, $:expr) => { a, s)		(0x7c000630 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(s)) };
}
macro_rules! PPC_RAW_SRAWI {
    ($d:expr, $:expr) => { a, i)		(0x7c000670 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH(i)) };
}
macro_rules! PPC_RAW_SRD {
    ($d:expr, $:expr) => { a, s)		(0x7c000436 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(s)) };
}
macro_rules! PPC_RAW_SRAD {
    ($d:expr, $:expr) => { a, s)		(0x7c000634 | ___PPC_RA(d) | ___PPC_RS(a) | ___PPC_RB(s)) };
}
macro_rules! PPC_RAW_SRADI {
    ($d:expr, $:expr) => { a, i)		(0x7c000674 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH64(i)) };
}
macro_rules! PPC_RAW_RLWINM {
    ($d:expr, $:expr) => { a, i, mb, me)	(0x54000000 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH(i) | __PPC_MB(mb) | __PPC_ME(me)) };
}
// #define PPC_RAW_RLWINM_DOT(d, a, i, mb, me) \
					(0x54000001 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH(i) | __PPC_MB(mb) | __PPC_ME(me))
macro_rules! PPC_RAW_RLWIMI {
    ($d:expr, $:expr) => { a, i, mb, me) (0x50000000 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH(i) | __PPC_MB(mb) | __PPC_ME(me)) };
}
macro_rules! PPC_RAW_RLDICL {
    ($d:expr, $:expr) => { a, i, mb)     (0x78000000 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH64(i) | __PPC_MB64(mb)) };
}
macro_rules! PPC_RAW_RLDICL_DOT {
    ($d:expr, $:expr) => { a, i, mb) (0x78000000 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH64(i) | __PPC_MB64(mb) | 0x1) };
}
macro_rules! PPC_RAW_RLDICR {
    ($d:expr, $:expr) => { a, i, me)     (0x78000004 | ___PPC_RA(d) | ___PPC_RS(a) | __PPC_SH64(i) | __PPC_ME64(me)) };
}

/* slwi = rlwinm Rx, Ry, n, 0, 31-n */
macro_rules! PPC_RAW_SLWI {
    ($d:expr, $:expr) => { a, i)		PPC_RAW_RLWINM(d, a, i, 0, 31-(i)) };
}
/* srwi = rlwinm Rx, Ry, 32-n, n, 31 */
macro_rules! PPC_RAW_SRWI {
    ($d:expr, $:expr) => { a, i)		PPC_RAW_RLWINM(d, a, 32-(i), i, 31) };
}
/* sldi = rldicr Rx, Ry, n, 63-n */
macro_rules! PPC_RAW_SLDI {
    ($d:expr, $:expr) => { a, i)		PPC_RAW_RLDICR(d, a, i, 63-(i)) };
}
/* sldi = rldicl Rx, Ry, 64-n, n */
macro_rules! PPC_RAW_SRDI {
    ($d:expr, $:expr) => { a, i)		PPC_RAW_RLDICL(d, a, 64-(i), i) };
}

macro_rules! PPC_RAW_NEG {
    ($d:expr, $:expr) => { a)		(0x7c0000d0 | ___PPC_RT(d) | ___PPC_RA(a)) };
}

macro_rules! PPC_RAW_MFSPR {
    ($d:expr, $:expr) => { spr)		(0x7c0002a6 | ___PPC_RT(d) | __PPC_SPR(spr)) };
}
macro_rules! PPC_RAW_MTSPR {
    ($spr:expr, $:expr) => { d)		(0x7c0003a6 | ___PPC_RS(d) | __PPC_SPR(spr)) };
}
macro_rules! PPC_RAW_EIEIO {
    () => { (0x7c0006ac) };
}

/* bcl 20,31,$+4 */
macro_rules! PPC_RAW_BCL4 {
    () => { (0x429f0005) };
}
macro_rules! PPC_RAW_BRANCH {
    ($offset:expr) => { (0x48000000 | PPC_LI(offset)) };
}
macro_rules! PPC_RAW_BL {
    ($offset:expr) => { (0x48000001 | PPC_LI(offset)) };
}
macro_rules! PPC_RAW_TW {
    ($t0:expr, $:expr) => { a, b)		(0x7c000008 | ___PPC_RS(t0) | ___PPC_RA(a) | ___PPC_RB(b)) };
}
macro_rules! PPC_RAW_TRAP {
    () => { PPC_RAW_TW(31, 0, 0) };
}
macro_rules! PPC_RAW_SETB {
    ($t:expr, $:expr) => { bfa)		(0x7c000100 | ___PPC_RT(t) | ___PPC_RA((bfa) << 2)) };
}

// Build-time CONFIG_PPC32 conditional follows in the source.
#[cfg(CONFIG_PPC32)]
macro_rules! PPC_RAW_STL { () => { PPC_RAW_STW }; }
macro_rules! PPC_RAW_STLU { () => { PPC_RAW_STWU }; }
macro_rules! PPC_RAW_LL { () => { PPC_RAW_LWZ }; }
macro_rules! PPC_RAW_CMPLI { () => { PPC_RAW_CMPWI }; }
#[cfg(not(CONFIG_PPC32))]
macro_rules! PPC_RAW_STL { () => { PPC_RAW_STD }; }
macro_rules! PPC_RAW_STLU { () => { PPC_RAW_STDU }; }
macro_rules! PPC_RAW_LL { () => { PPC_RAW_LD }; }
macro_rules! PPC_RAW_CMPLI { () => { PPC_RAW_CMPDI }; }

/* Deal with instructions that older assemblers aren't aware of */
// #define	PPC_BCCTR_FLUSH		stringify_in_c(.long PPC_INST_BCCTR_FLUSH)
// #define	PPC_CP_ABORT		stringify_in_c(.long PPC_RAW_CP_ABORT)
// #define	PPC_COPY(a, b)		stringify_in_c(.long PPC_RAW_COPY(a, b))
macro_rules! PPC_DARN {
    ($t:expr, $:expr) => { l)		stringify_in_c(.long PPC_RAW_DARN(t, l)) };
}
// #define	PPC_DCBAL(a, b)		stringify_in_c(.long PPC_RAW_DCBAL(a, b))
// #define	PPC_DCBZL(a, b)		stringify_in_c(.long PPC_RAW_DCBZL(a, b))
// #define	PPC_DIVDE(t, a, b)	stringify_in_c(.long PPC_RAW_DIVDE(t, a, b))
// #define	PPC_DIVDEU(t, a, b)	stringify_in_c(.long PPC_RAW_DIVDEU(t, a, b))
macro_rules! PPC_DSSALL { () => { stringify_in_c(.long PPC_INST_DSSALL) }; }
macro_rules! PPC_LQARX {
    ($t:expr, $:expr) => { a, b, eh)	stringify_in_c(.long PPC_RAW_LQARX(t, a, b, eh)) };
}
macro_rules! PPC_STQCX {
    ($t:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_STQCX(t, a, b)) };
}
macro_rules! PPC_MADDHD {
    ($t:expr, $:expr) => { a, b, c)	stringify_in_c(.long PPC_RAW_MADDHD(t, a, b, c)) };
}
macro_rules! PPC_MADDHDU {
    ($t:expr, $:expr) => { a, b, c)	stringify_in_c(.long PPC_RAW_MADDHDU(t, a, b, c)) };
}
macro_rules! PPC_MADDLD {
    ($t:expr, $:expr) => { a, b, c)	stringify_in_c(.long PPC_RAW_MADDLD(t, a, b, c)) };
}
macro_rules! PPC_MSGSND {
    ($b:expr) => { stringify_in_c(.long PPC_RAW_MSGSND(b)) };
}
macro_rules! PPC_MSGSYNC { () => { stringify_in_c(.long PPC_RAW_MSGSYNC) }; }
macro_rules! PPC_MSGCLR {
    ($b:expr) => { stringify_in_c(.long PPC_RAW_MSGCLR(b)) };
}
macro_rules! PPC_MSGSNDP {
    ($b:expr) => { stringify_in_c(.long PPC_RAW_MSGSNDP(b)) };
}
macro_rules! PPC_MSGCLRP {
    ($b:expr) => { stringify_in_c(.long PPC_RAW_MSGCLRP(b)) };
}
macro_rules! PPC_PASTE {
    ($a:expr, $:expr) => { b)		stringify_in_c(.long PPC_RAW_PASTE(a, b)) };
}
macro_rules! PPC_POPCNTB {
    ($a:expr, $:expr) => { s)	stringify_in_c(.long PPC_RAW_POPCNTB(a, s)) };
}
macro_rules! PPC_POPCNTD {
    ($a:expr, $:expr) => { s)	stringify_in_c(.long PPC_RAW_POPCNTD(a, s)) };
}
macro_rules! PPC_POPCNTW {
    ($a:expr, $:expr) => { s)	stringify_in_c(.long PPC_RAW_POPCNTW(a, s)) };
}
macro_rules! PPC_RFCI { () => { stringify_in_c(.long PPC_RAW_RFCI) }; }
macro_rules! PPC_RFDI { () => { stringify_in_c(.long PPC_RAW_RFDI) }; }
macro_rules! PPC_RFMCI { () => { stringify_in_c(.long PPC_RAW_RFMCI) }; }
macro_rules! PPC_TLBILX {
    ($t:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_TLBILX(t, a, b)) };
}
macro_rules! PPC_TLBILX_ALL {
    ($a:expr, $:expr) => { b)	PPC_TLBILX(0, a, b) };
}
macro_rules! PPC_TLBILX_PID {
    ($a:expr, $:expr) => { b)	PPC_TLBILX(1, a, b) };
}
macro_rules! PPC_TLBILX_LPID { () => { stringify_in_c(.long PPC_RAW_TLBILX_LPID) }; }
macro_rules! PPC_TLBILX_VA {
    ($a:expr, $:expr) => { b)	PPC_TLBILX(3, a, b) };
}
macro_rules! PPC_WAIT_v203 { () => { stringify_in_c(.long PPC_RAW_WAIT_v203) }; }
macro_rules! PPC_WAIT {
    ($w:expr, $:expr) => { p)		stringify_in_c(.long PPC_RAW_WAIT(w, p)) };
}
macro_rules! PPC_TLBIE {
    ($lp:expr, $:expr) => { a) 	stringify_in_c(.long PPC_RAW_TLBIE(lp, a)) };
}
// #define	PPC_TLBIE_5(rb, rs, ric, prs, r) \
				stringify_in_c(.long PPC_RAW_TLBIE_5(rb, rs, ric, prs, r))
// #define	PPC_TLBIEL(rb,rs,ric,prs,r) \
				stringify_in_c(.long PPC_RAW_TLBIEL(rb, rs, ric, prs, r))
macro_rules! PPC_TLBIEL_v205 {
    ($rb:expr, $:expr) => { l)	stringify_in_c(.long PPC_RAW_TLBIEL_v205(rb, l)) };
}
macro_rules! PPC_TLBSRX_DOT {
    ($a:expr, $:expr) => { b)	stringify_in_c(.long PPC_RAW_TLBSRX_DOT(a, b)) };
}
macro_rules! PPC_TLBIVAX {
    ($a:expr, $:expr) => { b)	stringify_in_c(.long PPC_RAW_TLBIVAX(a, b)) };
}

macro_rules! PPC_ERATWE {
    ($s:expr, $:expr) => { a, w)	stringify_in_c(.long PPC_RAW_ERATWE(s, a, w)) };
}
macro_rules! PPC_ERATRE {
    ($s:expr, $:expr) => { a, w)	stringify_in_c(.long PPC_RAW_ERATRE(a, a, w)) };
}
macro_rules! PPC_ERATILX {
    ($t:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_ERATILX(t, a, b)) };
}
macro_rules! PPC_ERATIVAX {
    ($s:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_ERATIVAX(s, a, b)) };
}
macro_rules! PPC_ERATSX {
    ($t:expr, $:expr) => { a, w)	stringify_in_c(.long PPC_RAW_ERATSX(t, a, w)) };
}
macro_rules! PPC_ERATSX_DOT {
    ($t:expr, $:expr) => { a, w)	stringify_in_c(.long PPC_RAW_ERATSX_DOT(t, a, w)) };
}
macro_rules! PPC_SLBFEE_DOT {
    ($t:expr, $:expr) => { b)	stringify_in_c(.long PPC_RAW_SLBFEE_DOT(t, b)) };
}
macro_rules! __PPC_SLBFEE_DOT {
    ($t:expr, $:expr) => { b)	stringify_in_c(.long __PPC_RAW_SLBFEE_DOT(t, b)) };
}
macro_rules! PPC_ICBT {
    ($c:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_ICBT(c, a, b)) };
}
/* PASemi instructions */
macro_rules! LBZCIX {
    ($t:expr, $:expr) => { a, b)		stringify_in_c(.long PPC_RAW_LBZCIX(t, a, b)) };
}
macro_rules! STBCIX {
    ($s:expr, $:expr) => { a, b)		stringify_in_c(.long PPC_RAW_STBCIX(s, a, b)) };
}
macro_rules! PPC_DCBFPS {
    ($a:expr, $:expr) => { b)	stringify_in_c(.long PPC_RAW_DCBFPS(a, b)) };
}
macro_rules! PPC_DCBSTPS {
    ($a:expr, $:expr) => { b)	stringify_in_c(.long PPC_RAW_DCBSTPS(a, b)) };
}
macro_rules! PPC_PHWSYNC { () => { stringify_in_c(.long PPC_RAW_PHWSYNC) }; }
macro_rules! PPC_PLWSYNC { () => { stringify_in_c(.long PPC_RAW_PLWSYNC) }; }
macro_rules! STXVD2X {
    ($s:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_STXVD2X(s, a, b)) };
}
macro_rules! LXVD2X {
    ($s:expr, $:expr) => { a, b)		stringify_in_c(.long PPC_RAW_LXVD2X(s, a, b)) };
}
macro_rules! MFVRD {
    ($a:expr, $:expr) => { t)		stringify_in_c(.long PPC_RAW_MFVRD(a, t)) };
}
macro_rules! MTVRD {
    ($t:expr, $:expr) => { a)		stringify_in_c(.long PPC_RAW_MTVRD(t, a)) };
}
macro_rules! VPMSUMW {
    ($t:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_VPMSUMW(t, a, b)) };
}
macro_rules! VPMSUMD {
    ($t:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_VPMSUMD(t, a, b)) };
}
macro_rules! XXLOR {
    ($t:expr, $:expr) => { a, b)		stringify_in_c(.long PPC_RAW_XXLOR(t, a, b)) };
}
macro_rules! XXSWAPD {
    ($t:expr, $:expr) => { a)		stringify_in_c(.long PPC_RAW_XXSWAPD(t, a)) };
}
macro_rules! XVCPSGNDP {
    ($t:expr, $:expr) => { a, b)	stringify_in_c(.long (PPC_RAW_XVCPSGNDP(t, a, b))) };
}

// #define VPERMXOR(vrt, vra, vrb, vrc)				\
	stringify_in_c(.long (PPC_RAW_VPERMXOR(vrt, vra, vrb, vrc)))

macro_rules! PPC_NAP { () => { stringify_in_c(.long PPC_RAW_NAP) }; }
macro_rules! PPC_SLEEP { () => { stringify_in_c(.long PPC_RAW_SLEEP) }; }
macro_rules! PPC_WINKLE { () => { stringify_in_c(.long PPC_RAW_WINKLE) }; }

macro_rules! PPC_STOP { () => { stringify_in_c(.long PPC_RAW_STOP) }; }

/* BHRB instructions */
macro_rules! PPC_CLRBHRB { () => { stringify_in_c(.long PPC_RAW_CLRBHRB) }; }
macro_rules! PPC_MFBHRBE {
    ($r:expr, $:expr) => { n)	stringify_in_c(.long PPC_RAW_MFBHRBE(r, n)) };
}

/* Transactional memory instructions */
macro_rules! TRECHKPT { () => { stringify_in_c(.long PPC_RAW_TRECHKPT) }; }
macro_rules! TRECLAIM {
    ($r:expr) => { stringify_in_c(.long PPC_RAW_TRECLAIM(r)) };
}
macro_rules! TABORT {
    ($r:expr) => { stringify_in_c(.long PPC_RAW_TABORT(r)) };
}

/* book3e thread control instructions */
macro_rules! MTTMR {
    ($tmr:expr, $:expr) => { r)		stringify_in_c(.long PPC_RAW_MTTMR(tmr, r)) };
}
macro_rules! MFTMR {
    ($tmr:expr, $:expr) => { r)		stringify_in_c(.long PPC_RAW_MFTMR(tmr, r)) };
}

/* Coprocessor instructions */
macro_rules! PPC_ICSWX {
    ($s:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_ICSWX(s, a, b)) };
}
macro_rules! PPC_ICSWEPX {
    ($s:expr, $:expr) => { a, b)	stringify_in_c(.long PPC_RAW_ICSWEPX(s, a, b)) };
}

macro_rules! PPC_SLBIA {
    ($IH:expr) => { stringify_in_c(.long PPC_RAW_SLBIA(IH)) };
}

/*
 * These may only be used on ISA v3.0 or later (aka. CPU_FTR_ARCH_300, radix
 * implies CPU_FTR_ARCH_300). USER/GUEST invalidates may only be used by radix
 * mode (on HPT these would also invalidate various SLBEs which may not be
 * desired).
 */
macro_rules! PPC_ISA_3_0_INVALIDATE_ERAT { () => { PPC_SLBIA(7) }; }
macro_rules! PPC_RADIX_INVALIDATE_ERAT_USER { () => { PPC_SLBIA(3) }; }
macro_rules! PPC_RADIX_INVALIDATE_ERAT_GUEST { () => { PPC_SLBIA(6) }; }

macro_rules! VCMPEQUD_RC {
    ($vrt:expr, $:expr) => { vra, vrb)	stringify_in_c(.long PPC_RAW_VCMPEQUD_RC(vrt, vra, vrb)) };
}

macro_rules! VCMPEQUB_RC {
    ($vrt:expr, $:expr) => { vra, vrb)	stringify_in_c(.long PPC_RAW_VCMPEQUB_RC(vrt, vra, vrb)) };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
