// Direct Rust translation of powerpc/include/asm/reg.h.
// C preprocessor configuration branches are preserved as comments where Rust cfg names are external.

/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Contains the definition of registers common to all PowerPC variants.
 * If a register definition has been changed in a different PowerPC
 * variant, we will case it in #ifndef XXX ... #endif, and have the
 * number used in the Programming Environments Manual For 32-Bit
 * Implementations of the PowerPC Architecture (a.k.a. Green Book) here.
 */

// #ifndef _ASM_POWERPC_REG_H
pub const _ASM_POWERPC_REG_H: u64 = 0;
// #ifdef __KERNEL__

// #include <linux/stringify.h>
// #include <linux/const.h>
// #include <asm/cputable.h>
// #include <asm/asm-const.h>
// #include <asm/feature-fixups.h>

/* Pickup Book E specific registers. */
// #ifdef CONFIG_BOOKE
// #include <asm/reg_booke.h>
// #endif

// #ifdef CONFIG_FSL_EMB_PERFMON
// #include <asm/reg_fsl_emb.h>
// #endif

// #include <asm/reg_8xx.h>

pub const MSR_SF_LG: u64 = 63              /* Enable 64 bit mode */;
pub const MSR_HV_LG: u64 = 60              /* Hypervisor state */;
pub const MSR_TS_T_LG: u64 = 34		/* Trans Mem state: Transactional */;
pub const MSR_TS_S_LG: u64 = 33		/* Trans Mem state: Suspended */;
pub const MSR_TS_LG: u64 = 33		/* Trans Mem state (2 bits) */;
pub const MSR_TM_LG: u64 = 32		/* Trans Mem Available */;
pub const MSR_VEC_LG: u64 = 25	        /* Enable AltiVec */;
pub const MSR_VSX_LG: u64 = 23		/* Enable VSX */;
pub const MSR_S_LG: u64 = 22		/* Secure state */;
pub const MSR_POW_LG: u64 = 18		/* Enable Power Management */;
pub const MSR_WE_LG: u64 = 18		/* Wait State Enable */;
pub const MSR_TGPR_LG: u64 = 17		/* TLB Update registers in use */;
pub const MSR_CE_LG: u64 = 17		/* Critical Interrupt Enable */;
pub const MSR_ILE_LG: u64 = 16		/* Interrupt Little Endian */;
pub const MSR_EE_LG: u64 = 15		/* External Interrupt Enable */;
pub const MSR_PR_LG: u64 = 14		/* Problem State / Privilege Level */;
pub const MSR_FP_LG: u64 = 13		/* Floating Point enable */;
pub const MSR_ME_LG: u64 = 12		/* Machine Check Enable */;
pub const MSR_FE0_LG: u64 = 11		/* Floating Exception mode 0 */;
pub const MSR_SE_LG: u64 = 10		/* Single Step */;
pub const MSR_BE_LG: u64 = 9		/* Branch Trace */;
pub const MSR_DE_LG: u64 = 9 		/* Debug Exception Enable */;
pub const MSR_FE1_LG: u64 = 8		/* Floating Exception mode 1 */;
pub const MSR_IP_LG: u64 = 6		/* Exception prefix 0x000/0xFFF */;
pub const MSR_IR_LG: u64 = 5 		/* Instruction Relocate */;
pub const MSR_DR_LG: u64 = 4 		/* Data Relocate */;
pub const MSR_PE_LG: u64 = 3		/* Protection Enable */;
pub const MSR_PX_LG: u64 = 2		/* Protection Exclusive Mode */;
pub const MSR_PMM_LG: u64 = 2		/* Performance monitor */;
pub const MSR_RI_LG: u64 = 1		/* Recoverable Exception */;
pub const MSR_LE_LG: u64 = 0 		/* Little Endian */;

// #ifdef __ASSEMBLER__
// #define __MASK(X)	(1<<(X))
// #else
// #define __MASK(X)	(1UL<<(X))
// #endif

// #ifdef CONFIG_PPC64
pub const MSR_SF: u64 = (1u64 << MSR_SF_LG)	/* Enable 64 bit mode */;
pub const MSR_HV: u64 = (1u64 << MSR_HV_LG)	/* Hypervisor state */;
pub const MSR_S: u64 = (1u64 << MSR_S_LG)	/* Secure state */;
// #else
/* so tests for these bits fail on 32-bit */
pub const MSR_SF: u64 = 0;
pub const MSR_HV: u64 = 0;
pub const MSR_S: u64 = 0;
// #endif

/*
 * To be used in shared book E/book S, this avoids needing to worry about
 * book S/book E in shared code
 */
// #ifndef MSR_SPE
pub const MSR_SPE: u64 = 0;
// #endif

pub const MSR_VEC: u64 = (1u64 << MSR_VEC_LG)	/* Enable AltiVec */;
pub const MSR_VSX: u64 = (1u64 << MSR_VSX_LG)	/* Enable VSX */;
pub const MSR_POW: u64 = (1u64 << MSR_POW_LG)	/* Enable Power Management */;
pub const MSR_WE: u64 = (1u64 << MSR_WE_LG)	/* Wait State Enable */;
pub const MSR_TGPR: u64 = (1u64 << MSR_TGPR_LG)	/* TLB Update registers in use */;
pub const MSR_CE: u64 = (1u64 << MSR_CE_LG)	/* Critical Interrupt Enable */;
pub const MSR_ILE: u64 = (1u64 << MSR_ILE_LG)	/* Interrupt Little Endian */;
pub const MSR_EE: u64 = (1u64 << MSR_EE_LG)	/* External Interrupt Enable */;
pub const MSR_PR: u64 = (1u64 << MSR_PR_LG)	/* Problem State / Privilege Level */;
pub const MSR_FP: u64 = (1u64 << MSR_FP_LG)	/* Floating Point enable */;
pub const MSR_ME: u64 = (1u64 << MSR_ME_LG)	/* Machine Check Enable */;
pub const MSR_FE0: u64 = (1u64 << MSR_FE0_LG)	/* Floating Exception mode 0 */;
pub const MSR_SE: u64 = (1u64 << MSR_SE_LG)	/* Single Step */;
pub const MSR_BE: u64 = (1u64 << MSR_BE_LG)	/* Branch Trace */;
pub const MSR_DE: u64 = (1u64 << MSR_DE_LG)	/* Debug Exception Enable */;
pub const MSR_FE1: u64 = (1u64 << MSR_FE1_LG)	/* Floating Exception mode 1 */;
pub const MSR_IP: u64 = (1u64 << MSR_IP_LG)	/* Exception prefix 0x000/0xFFF */;
pub const MSR_IR: u64 = (1u64 << MSR_IR_LG)	/* Instruction Relocate */;
pub const MSR_DR: u64 = (1u64 << MSR_DR_LG)	/* Data Relocate */;
pub const MSR_PE: u64 = (1u64 << MSR_PE_LG)	/* Protection Enable */;
pub const MSR_PX: u64 = (1u64 << MSR_PX_LG)	/* Protection Exclusive Mode */;
// #ifndef MSR_PMM
pub const MSR_PMM: u64 = (1u64 << MSR_PMM_LG)	/* Performance monitor */;
// #endif
pub const MSR_RI: u64 = (1u64 << MSR_RI_LG)	/* Recoverable Exception */;
pub const MSR_LE: u64 = (1u64 << MSR_LE_LG)	/* Little Endian */;

pub const MSR_TM: u64 = (1u64 << MSR_TM_LG)	/* Transactional Mem Available */;
pub const MSR_TS_N: u64 = 0			/*  Non-transactional */;
pub const MSR_TS_S: u64 = (1u64 << MSR_TS_S_LG)	/*  Transaction Suspended */;
pub const MSR_TS_T: u64 = (1u64 << MSR_TS_T_LG)	/*  Transaction Transactional */;
pub const MSR_TS_MASK: u64 = (MSR_TS_T | MSR_TS_S)   /* Transaction State bits */;
// #define MSR_TM_RESV(x) (((x) & MSR_TS_MASK) == MSR_TS_MASK) /* Reserved */
// #define MSR_TM_TRANSACTIONAL(x)	(((x) & MSR_TS_MASK) == MSR_TS_T)
// #define MSR_TM_SUSPENDED(x)	(((x) & MSR_TS_MASK) == MSR_TS_S)

// #ifdef CONFIG_PPC_TRANSACTIONAL_MEM
// #define MSR_TM_ACTIVE(x) (((x) & MSR_TS_MASK) != 0) /* Transaction active? */
// #else
// #define MSR_TM_ACTIVE(x) ((void)(x), 0)
// #endif

// #if defined(CONFIG_PPC_BOOK3S_64)
pub const MSR_64BIT: u64 = MSR_SF;

/* Server variant */
pub const __MSR: u64 = (MSR_ME | MSR_RI | MSR_IR | MSR_DR | MSR_HV);
// #ifdef __BIG_ENDIAN__
pub const MSR_: u64 = __MSR;
pub const MSR_IDLE: u64 = (MSR_ME | MSR_SF | MSR_HV);
// #else
pub const MSR_: u64 = (__MSR | MSR_LE);
pub const MSR_IDLE: u64 = (MSR_ME | MSR_SF | MSR_HV | MSR_LE);
// #endif
pub const MSR_KERNEL: u64 = (MSR_ | MSR_64BIT);
pub const MSR_USER32: u64 = (MSR_ | MSR_PR | MSR_EE);
pub const MSR_USER64: u64 = (MSR_USER32 | MSR_64BIT);
// #elif defined(CONFIG_PPC_BOOK3S_32) || defined(CONFIG_PPC_8xx)
/* Default MSR for kernel mode. */
pub const MSR_KERNEL: u64 = (MSR_ME|MSR_RI|MSR_IR|MSR_DR);
pub const MSR_USER: u64 = (MSR_KERNEL|MSR_PR|MSR_EE);
// #endif

// #ifndef MSR_64BIT
pub const MSR_64BIT: u64 = 0;
// #endif

/* Condition Register related */
pub const CR0_SHIFT: u64 = 28;
pub const CR0_MASK: u64 = 0xF;
pub const CR0_TBEGIN_FAILURE: u64 = (0x2 << 28) /* 0b0010 */;


/* Power Management - Processor Stop Status and Control Register Fields */
pub const PSSCR_RL_MASK: u64 = 0x0000000F /* Requested Level */;
pub const PSSCR_MTL_MASK: u64 = 0x000000F0 /* Maximum Transition Level */;
pub const PSSCR_TR_MASK: u64 = 0x00000300 /* Transition State */;
pub const PSSCR_PSLL_MASK: u64 = 0x000F0000 /* Power-Saving Level Limit */;
pub const PSSCR_EC: u64 = 0x00100000 /* Exit Criterion */;
pub const PSSCR_ESL: u64 = 0x00200000 /* Enable State Loss */;
pub const PSSCR_SD: u64 = 0x00400000 /* Status Disable */;
pub const PSSCR_PLS: u64 = 0xf000000000000000 /* Power-saving Level Status */;
pub const PSSCR_PLS_SHIFT: u64 = 60;
pub const PSSCR_GUEST_VIS: u64 = 0xf0000000000003ffu64 /* Guest-visible PSSCR fields */;
pub const PSSCR_FAKE_SUSPEND: u64 = 0x00000400 /* Fake-suspend bit (P9 DD2.2) */;
pub const PSSCR_FAKE_SUSPEND_LG: u64 = 10	   /* Fake-suspend bit position */;

/* Floating Point Status and Control Register (FPSCR) Fields */
pub const FPSCR_FX: u64 = 0x80000000	/* FPU exception summary */;
pub const FPSCR_FEX: u64 = 0x40000000	/* FPU enabled exception summary */;
pub const FPSCR_VX: u64 = 0x20000000	/* Invalid operation summary */;
pub const FPSCR_OX: u64 = 0x10000000	/* Overflow exception summary */;
pub const FPSCR_UX: u64 = 0x08000000	/* Underflow exception summary */;
pub const FPSCR_ZX: u64 = 0x04000000	/* Zero-divide exception summary */;
pub const FPSCR_XX: u64 = 0x02000000	/* Inexact exception summary */;
pub const FPSCR_VXSNAN: u64 = 0x01000000	/* Invalid op for SNaN */;
pub const FPSCR_VXISI: u64 = 0x00800000	/* Invalid op for Inv - Inv */;
pub const FPSCR_VXIDI: u64 = 0x00400000	/* Invalid op for Inv / Inv */;
pub const FPSCR_VXZDZ: u64 = 0x00200000	/* Invalid op for Zero / Zero */;
pub const FPSCR_VXIMZ: u64 = 0x00100000	/* Invalid op for Inv * Zero */;
pub const FPSCR_VXVC: u64 = 0x00080000	/* Invalid op for Compare */;
pub const FPSCR_FR: u64 = 0x00040000	/* Fraction rounded */;
pub const FPSCR_FI: u64 = 0x00020000	/* Fraction inexact */;
pub const FPSCR_FPRF: u64 = 0x0001f000	/* FPU Result Flags */;
pub const FPSCR_FPCC: u64 = 0x0000f000	/* FPU Condition Codes */;
pub const FPSCR_VXSOFT: u64 = 0x00000400	/* Invalid op for software request */;
pub const FPSCR_VXSQRT: u64 = 0x00000200	/* Invalid op for square root */;
pub const FPSCR_VXCVI: u64 = 0x00000100	/* Invalid op for integer convert */;
pub const FPSCR_VE: u64 = 0x00000080	/* Invalid op exception enable */;
pub const FPSCR_OE: u64 = 0x00000040	/* IEEE overflow exception enable */;
pub const FPSCR_UE: u64 = 0x00000020	/* IEEE underflow exception enable */;
pub const FPSCR_ZE: u64 = 0x00000010	/* IEEE zero divide exception enable */;
pub const FPSCR_XE: u64 = 0x00000008	/* FP inexact exception enable */;
pub const FPSCR_NI: u64 = 0x00000004	/* FPU non IEEE-Mode */;
pub const FPSCR_RN: u64 = 0x00000003	/* FPU rounding control */;

/* Bit definitions for SPEFSCR. */
pub const SPEFSCR_SOVH: u64 = 0x80000000	/* Summary integer overflow high */;
pub const SPEFSCR_OVH: u64 = 0x40000000	/* Integer overflow high */;
pub const SPEFSCR_FGH: u64 = 0x20000000	/* Embedded FP guard bit high */;
pub const SPEFSCR_FXH: u64 = 0x10000000	/* Embedded FP sticky bit high */;
pub const SPEFSCR_FINVH: u64 = 0x08000000	/* Embedded FP invalid operation high */;
pub const SPEFSCR_FDBZH: u64 = 0x04000000	/* Embedded FP div by zero high */;
pub const SPEFSCR_FUNFH: u64 = 0x02000000	/* Embedded FP underflow high */;
pub const SPEFSCR_FOVFH: u64 = 0x01000000	/* Embedded FP overflow high */;
pub const SPEFSCR_FINXS: u64 = 0x00200000	/* Embedded FP inexact sticky */;
pub const SPEFSCR_FINVS: u64 = 0x00100000	/* Embedded FP invalid op. sticky */;
pub const SPEFSCR_FDBZS: u64 = 0x00080000	/* Embedded FP div by zero sticky */;
pub const SPEFSCR_FUNFS: u64 = 0x00040000	/* Embedded FP underflow sticky */;
pub const SPEFSCR_FOVFS: u64 = 0x00020000	/* Embedded FP overflow sticky */;
pub const SPEFSCR_MODE: u64 = 0x00010000	/* Embedded FP mode */;
pub const SPEFSCR_SOV: u64 = 0x00008000	/* Integer summary overflow */;
pub const SPEFSCR_OV: u64 = 0x00004000	/* Integer overflow */;
pub const SPEFSCR_FG: u64 = 0x00002000	/* Embedded FP guard bit */;
pub const SPEFSCR_FX: u64 = 0x00001000	/* Embedded FP sticky bit */;
pub const SPEFSCR_FINV: u64 = 0x00000800	/* Embedded FP invalid operation */;
pub const SPEFSCR_FDBZ: u64 = 0x00000400	/* Embedded FP div by zero */;
pub const SPEFSCR_FUNF: u64 = 0x00000200	/* Embedded FP underflow */;
pub const SPEFSCR_FOVF: u64 = 0x00000100	/* Embedded FP overflow */;
pub const SPEFSCR_FINXE: u64 = 0x00000040	/* Embedded FP inexact enable */;
pub const SPEFSCR_FINVE: u64 = 0x00000020	/* Embedded FP invalid op. enable */;
pub const SPEFSCR_FDBZE: u64 = 0x00000010	/* Embedded FP div by zero enable */;
pub const SPEFSCR_FUNFE: u64 = 0x00000008	/* Embedded FP underflow enable */;
pub const SPEFSCR_FOVFE: u64 = 0x00000004	/* Embedded FP overflow enable */;
pub const SPEFSCR_FRMC: u64 = 0x00000003	/* Embedded FP rounding mode control */;

/* Special Purpose Registers (SPRNs)*/

pub const SPRN_PID: u64 = 0x030	/* Process ID */;
// #ifdef CONFIG_BOOKE
pub const SPRN_PID0: u64 = SPRN_PID/* Process ID Register 0 */;
// #endif

pub const SPRN_CTR: u64 = 0x009	/* Count Register */;
pub const SPRN_DSCR: u64 = 0x11;
pub const SPRN_CFAR: u64 = 0x1c	/* Come From Address Register */;
pub const SPRN_AMR: u64 = 0x1d	/* Authority Mask Register */;
pub const SPRN_UAMOR: u64 = 0x9d	/* User Authority Mask Override Register */;
pub const SPRN_AMOR: u64 = 0x15d	/* Authority Mask Override Register */;
pub const SPRN_ACOP: u64 = 0x1F	/* Available Coprocessor Register */;
pub const SPRN_TFIAR: u64 = 0x81	/* Transaction Failure Inst Addr   */;
pub const SPRN_TEXASR: u64 = 0x82	/* Transaction EXception & Summary */;
pub const SPRN_TEXASRU: u64 = 0x83	/* ''	   ''	   ''	 Upper 32  */;

pub const TEXASR_FC_LG: u64 = (63 - 7)	/* Failure Code */;
pub const TEXASR_AB_LG: u64 = (63 - 31)	/* Abort */;
pub const TEXASR_SU_LG: u64 = (63 - 32)	/* Suspend */;
pub const TEXASR_HV_LG: u64 = (63 - 34)	/* Hypervisor state*/;
pub const TEXASR_PR_LG: u64 = (63 - 35)	/* Privilege level */;
pub const TEXASR_FS_LG: u64 = (63 - 36)	/* failure summary */;
pub const TEXASR_EX_LG: u64 = (63 - 37)	/* TFIAR exact bit */;
pub const TEXASR_ROT_LG: u64 = (63 - 38)	/* ROT bit */;

// #define   TEXASR_ABORT	__MASK(TEXASR_AB_LG) /* terminated by tabort or treclaim */
// #define   TEXASR_SUSP	__MASK(TEXASR_SU_LG) /* tx failed in suspended state */
// #define   TEXASR_HV	__MASK(TEXASR_HV_LG) /* MSR[HV] when failure occurred */
// #define   TEXASR_PR	__MASK(TEXASR_PR_LG) /* MSR[PR] when failure occurred */
// #define   TEXASR_FS	__MASK(TEXASR_FS_LG) /* TEXASR Failure Summary */
// #define   TEXASR_EXACT	__MASK(TEXASR_EX_LG) /* TFIAR value is exact */
// #define   TEXASR_ROT	__MASK(TEXASR_ROT_LG)
// #define   TEXASR_FC	(ASM_CONST(0xFF) << TEXASR_FC_LG)

pub const SPRN_TFHAR: u64 = 0x80	/* Transaction Failure Handler Addr */;

pub const SPRN_TIDR: u64 = 144	/* Thread ID register */;
pub const SPRN_CTRLF: u64 = 0x088;
pub const SPRN_CTRLT: u64 = 0x098;
// #define   CTRL_CT	0xc0000000	/* current thread */
// #define   CTRL_CT0	0x80000000	/* thread 0 */
// #define   CTRL_CT1	0x40000000	/* thread 1 */
// #define   CTRL_TE	0x00c00000	/* thread enable */
// #define   CTRL_RUNLATCH	0x1
pub const SPRN_DAWR0: u64 = 0xB4;
pub const SPRN_DAWR1: u64 = 0xB5;
pub const SPRN_RPR: u64 = 0xBA	/* Relative Priority Register */;
pub const SPRN_CIABR: u64 = 0xBB;
// #define   CIABR_PRIV		0x3
// #define   CIABR_PRIV_USER	1
// #define   CIABR_PRIV_SUPER	2
// #define   CIABR_PRIV_HYPER	3
pub const SPRN_DAWRX0: u64 = 0xBC;
pub const SPRN_DAWRX1: u64 = 0xBD;
// #define   DAWRX_USER	__MASK(0)
// #define   DAWRX_KERNEL	__MASK(1)
// #define   DAWRX_HYP	__MASK(2)
// #define   DAWRX_WTI	__MASK(3)
// #define   DAWRX_WT	__MASK(4)
// #define   DAWRX_DR	__MASK(5)
// #define   DAWRX_DW	__MASK(6)
pub const SPRN_DABR: u64 = 0x3F5	/* Data Address Breakpoint Register */;
pub const SPRN_DABR2: u64 = 0x13D	/* e300 */;
pub const SPRN_DABRX: u64 = 0x3F7	/* Data Address Breakpoint Register Extension */;
// #define   DABRX_USER	__MASK(0)
// #define   DABRX_KERNEL	__MASK(1)
// #define   DABRX_HYP	__MASK(2)
// #define   DABRX_BTI	__MASK(3)
// #define   DABRX_ALL     (DABRX_BTI | DABRX_HYP | DABRX_KERNEL | DABRX_USER)
pub const SPRN_DAR: u64 = 0x013	/* Data Address Register */;
pub const SPRN_DBCR: u64 = 0x136	/* e300 Data Breakpoint Control Reg */;
pub const SPRN_DSISR: u64 = 0x012	/* Data Storage Interrupt Status Register */;
// #define   DSISR_BAD_DIRECT_ST	0x80000000 /* Obsolete: Direct store error */
// #define   DSISR_NOHPTE		0x40000000 /* no translation found */
// #define   DSISR_ATTR_CONFLICT	0x20000000 /* P9: Process vs. Partition attr */
// #define   DSISR_NOEXEC_OR_G	0x10000000 /* Alias of SRR1 bit, see below */
// #define   DSISR_PROTFAULT	0x08000000 /* protection fault */
// #define   DSISR_BADACCESS	0x04000000 /* bad access to CI or G */
// #define   DSISR_ISSTORE		0x02000000 /* access was a store */
// #define   DSISR_DABRMATCH	0x00400000 /* hit data breakpoint */
// #define   DSISR_NOSEGMENT	0x00200000 /* STAB miss (unsupported) */
// #define   DSISR_KEYFAULT	0x00200000 /* Storage Key fault */
// #define   DSISR_BAD_EXT_CTRL	0x00100000 /* Obsolete: External ctrl error */
// #define   DSISR_UNSUPP_MMU	0x00080000 /* P9: Unsupported MMU config */
// #define   DSISR_SET_RC		0x00040000 /* P9: Failed setting of R/C bits */
// #define   DSISR_PRTABLE_FAULT   0x00020000 /* P9: Fault on process table */
// #define   DSISR_ICSWX_NO_CT     0x00004000 /* P7: icswx unavailable cp type */
// #define   DSISR_BAD_COPYPASTE   0x00000008 /* P9: Copy/Paste on wrong memtype */
// #define   DSISR_BAD_AMO		0x00000004 /* P9: Incorrect AMO opcode */
// #define   DSISR_BAD_CI_LDST	0x00000002 /* P8: Bad HV CI load/store */

/*
 * DSISR_NOEXEC_OR_G doesn't actually exist. This bit is always
 * 0 on DSIs. However, on ISIs, the corresponding bit in SRR1
 * indicates an attempt at executing from a no-execute PTE
 * or segment or from a guarded page.
 *
 * We add a definition here for completeness as we alias
 * DSISR and SRR1 in do_page_fault.
 */

/*
 * DSISR bits that are treated as a fault. Any bit set
 * here will skip hash_page, and cause do_page_fault to
 * trigger a SIGBUS or SIGSEGV:
 */
// #define   DSISR_BAD_FAULT_32S	(DSISR_BAD_DIRECT_ST	| \
				 DSISR_BADACCESS	| \
				 DSISR_BAD_EXT_CTRL)
// #define	  DSISR_BAD_FAULT_64S	(DSISR_BAD_FAULT_32S	| \
				 DSISR_ATTR_CONFLICT	| \
				 DSISR_UNSUPP_MMU	| \
				 DSISR_PRTABLE_FAULT	| \
				 DSISR_ICSWX_NO_CT	| \
				 DSISR_BAD_COPYPASTE	| \
				 DSISR_BAD_AMO		| \
				 DSISR_BAD_CI_LDST)
/*
 * These bits are equivalent in SRR1 and DSISR for 0x400
 * instruction access interrupts on Book3S
 */
// #define   DSISR_SRR1_MATCH_32S	(DSISR_NOHPTE		| \
				 DSISR_NOEXEC_OR_G	| \
				 DSISR_PROTFAULT)
// #define   DSISR_SRR1_MATCH_64S	(DSISR_SRR1_MATCH_32S	| \
				 DSISR_KEYFAULT		| \
				 DSISR_UNSUPP_MMU	| \
				 DSISR_SET_RC		| \
				 DSISR_PRTABLE_FAULT)

pub const SPRN_TBRL: u64 = 0x10C	/* Time Base Read Lower Register (user, R/O) */;
pub const SPRN_TBRU: u64 = 0x10D	/* Time Base Read Upper Register (user, R/O) */;
pub const SPRN_CIR: u64 = 0x11B	/* Chip Information Register (hyper, R/0) */;
pub const SPRN_TBWL: u64 = 0x11C	/* Time Base Lower Register (super, R/W) */;
pub const SPRN_TBWU: u64 = 0x11D	/* Time Base Upper Register (super, R/W) */;
pub const SPRN_TBU40: u64 = 0x11E	/* Timebase upper 40 bits (hyper, R/W) */;
pub const SPRN_SPURR: u64 = 0x134	/* Scaled PURR */;
pub const SPRN_HSPRG0: u64 = 0x130	/* Hypervisor Scratch 0 */;
pub const SPRN_HSPRG1: u64 = 0x131	/* Hypervisor Scratch 1 */;
pub const SPRN_HDSISR: u64 = 0x132;
pub const SPRN_HDAR: u64 = 0x133;
pub const SPRN_HDEC: u64 = 0x136	/* Hypervisor Decrementer */;
pub const SPRN_HIOR: u64 = 0x137	/* 970 Hypervisor interrupt offset */;
pub const SPRN_RMOR: u64 = 0x138	/* Real mode offset register */;
pub const SPRN_HRMOR: u64 = 0x139	/* Real mode offset register */;
pub const SPRN_HDEXCR_RO: u64 = 0x1C7	/* Hypervisor DEXCR (non-privileged, readonly) */;
pub const SPRN_HASHKEYR: u64 = 0x1D4	/* Non-privileged hashst/hashchk key register */;
pub const SPRN_HDEXCR: u64 = 0x1D7	/* Hypervisor dynamic execution control register */;
pub const SPRN_DEXCR_RO: u64 = 0x32C	/* DEXCR (non-privileged, readonly) */;
pub const SPRN_ASDR: u64 = 0x330	/* Access segment descriptor register */;
pub const SPRN_DEXCR: u64 = 0x33C	/* Dynamic execution control register */;
// #define   DEXCR_PR_SBHE	  0x80000000UL /* 0: Speculative Branch Hint Enable */
// #define   DEXCR_PR_IBRTPD 0x10000000UL /* 3: Indirect Branch Recurrent Target Prediction Disable */
// #define   DEXCR_PR_SRAPD  0x08000000UL /* 4: Subroutine Return Address Prediction Disable */
// #define   DEXCR_PR_NPHIE  0x04000000UL /* 5: Non-Privileged Hash Instruction Enable */
// #define   DEXCR_INIT	DEXCR_PR_NPHIE	/* Fixed DEXCR value to initialise all CPUs with */
pub const SPRN_IC: u64 = 0x350	/* Virtual Instruction Count */;
pub const SPRN_VTB: u64 = 0x351	/* Virtual Time Base */;
pub const SPRN_LDBAR: u64 = 0x352	/* LD Base Address Register */;
pub const SPRN_PMICR: u64 = 0x354   /* Power Management Idle Control Reg */;
pub const SPRN_PMSR: u64 = 0x355   /* Power Management Status Reg */;
pub const SPRN_PMMAR: u64 = 0x356	/* Power Management Memory Activity Register */;
pub const SPRN_PSSCR: u64 = 0x357	/* Processor Stop Status and Control Register (ISA 3.0) */;
pub const SPRN_PSSCR_PR: u64 = 0x337	/* PSSCR ISA 3.0, privileged mode access */;
pub const SPRN_TRIG2: u64 = 0x372;
pub const SPRN_PMCR: u64 = 0x374	/* Power Management Control Register */;
pub const SPRN_RWMR: u64 = 0x375	/* Region-Weighting Mode Register */;

/* HFSCR and FSCR bit numbers are the same */
pub const FSCR_PREFIX_LG: u64 = 13	/* Enable Prefix Instructions */;
pub const FSCR_SCV_LG: u64 = 12	/* Enable System Call Vectored */;
pub const FSCR_MSGP_LG: u64 = 10	/* Enable MSGP */;
pub const FSCR_TAR_LG: u64 = 8	/* Enable Target Address Register */;
pub const FSCR_EBB_LG: u64 = 7	/* Enable Event Based Branching */;
pub const FSCR_TM_LG: u64 = 5	/* Enable Transactional Memory */;
pub const FSCR_BHRB_LG: u64 = 4	/* Enable Branch History Rolling Buffer*/;
pub const FSCR_PM_LG: u64 = 3	/* Enable prob/priv access to PMU SPRs */;
pub const FSCR_DSCR_LG: u64 = 2	/* Enable Data Stream Control Register */;
pub const FSCR_VECVSX_LG: u64 = 1	/* Enable VMX/VSX  */;
pub const FSCR_FP_LG: u64 = 0	/* Enable Floating Point */;
pub const SPRN_FSCR: u64 = 0x099	/* Facility Status & Control Register */;
// #define   FSCR_PREFIX	__MASK(FSCR_PREFIX_LG)
// #define   FSCR_SCV	__MASK(FSCR_SCV_LG)
// #define   FSCR_TAR	__MASK(FSCR_TAR_LG)
// #define   FSCR_EBB	__MASK(FSCR_EBB_LG)
// #define   FSCR_DSCR	__MASK(FSCR_DSCR_LG)
// #define   FSCR_INTR_CAUSE (ASM_CONST(0xFF) << 56)	/* interrupt cause */
pub const SPRN_HFSCR: u64 = 0xbe	/* HV=1 Facility Status & Control Register */;
// #define   HFSCR_PREFIX	__MASK(FSCR_PREFIX_LG)
// #define   HFSCR_MSGP	__MASK(FSCR_MSGP_LG)
// #define   HFSCR_TAR	__MASK(FSCR_TAR_LG)
// #define   HFSCR_EBB	__MASK(FSCR_EBB_LG)
// #define   HFSCR_TM	__MASK(FSCR_TM_LG)
// #define   HFSCR_PM	__MASK(FSCR_PM_LG)
// #define   HFSCR_BHRB	__MASK(FSCR_BHRB_LG)
// #define   HFSCR_DSCR	__MASK(FSCR_DSCR_LG)
// #define   HFSCR_VECVSX	__MASK(FSCR_VECVSX_LG)
// #define   HFSCR_FP	__MASK(FSCR_FP_LG)
// #define   HFSCR_INTR_CAUSE FSCR_INTR_CAUSE
pub const SPRN_TAR: u64 = 0x32f	/* Target Address Register */;
pub const SPRN_LPCR: u64 = 0x13E	/* LPAR Control Register */;
// #define   LPCR_VPM0		ASM_CONST(0x8000000000000000)
// #define   LPCR_VPM1		ASM_CONST(0x4000000000000000)
// #define   LPCR_ISL		ASM_CONST(0x2000000000000000)
// #define   LPCR_VC_SH		61
// #define   LPCR_DPFD_SH		52
// #define   LPCR_DPFD		(ASM_CONST(7) << LPCR_DPFD_SH)
// #define   LPCR_VRMASD_SH	47
// #define   LPCR_VRMASD		(ASM_CONST(0x1f) << LPCR_VRMASD_SH)
// #define   LPCR_VRMA_L		ASM_CONST(0x0008000000000000)
// #define   LPCR_VRMA_LP0		ASM_CONST(0x0001000000000000)
// #define   LPCR_VRMA_LP1		ASM_CONST(0x0000800000000000)
// #define   LPCR_RMLS		0x1C000000	/* Implementation dependent RMO limit sel */
// #define   LPCR_RMLS_SH		26
// #define   LPCR_HAIL		ASM_CONST(0x0000000004000000)   /* HV AIL (ISAv3.1) */
// #define   LPCR_ILE		ASM_CONST(0x0000000002000000)   /* !HV irqs set MSR:LE */
// #define   LPCR_AIL		ASM_CONST(0x0000000001800000)	/* Alternate interrupt location */
// #define   LPCR_AIL_0		ASM_CONST(0x0000000000000000)	/* MMU off exception offset 0x0 */
// #define   LPCR_AIL_3		ASM_CONST(0x0000000001800000)   /* MMU on exception offset 0xc00...4xxx */
// #define   LPCR_ONL		ASM_CONST(0x0000000000040000)	/* online - PURR/SPURR count */
// #define   LPCR_LD		ASM_CONST(0x0000000000020000)	/* large decremeter */
// #define   LPCR_PECE		ASM_CONST(0x000000000001f000)	/* powersave exit cause enable */
// #define     LPCR_PECEDP	ASM_CONST(0x0000000000010000)	/* directed priv dbells cause exit */
// #define     LPCR_PECEDH	ASM_CONST(0x0000000000008000)	/* directed hyp dbells cause exit */
// #define     LPCR_PECE0		ASM_CONST(0x0000000000004000)	/* ext. exceptions can cause exit */
// #define     LPCR_PECE1		ASM_CONST(0x0000000000002000)	/* decrementer can cause exit */
// #define     LPCR_PECE2		ASM_CONST(0x0000000000001000)	/* machine check etc can cause exit */
// #define     LPCR_PECE_HVEE	ASM_CONST(0x0000400000000000)	/* P9 Wakeup on HV interrupts */
// #define   LPCR_MER		ASM_CONST(0x0000000000000800)	/* Mediated External Exception */
// #define   LPCR_MER_SH		11
// #define	  LPCR_GTSE		ASM_CONST(0x0000000000000400)  	/* Guest Translation Shootdown Enable */
// #define   LPCR_TC		ASM_CONST(0x0000000000000200)	/* Translation control */
// #define   LPCR_HEIC		ASM_CONST(0x0000000000000010)   /* Hypervisor External Interrupt Control */
// #define   LPCR_LPES		0x0000000c
// #define   LPCR_LPES0		ASM_CONST(0x0000000000000008)      /* LPAR Env selector 0 */
// #define   LPCR_LPES1		ASM_CONST(0x0000000000000004)      /* LPAR Env selector 1 */
// #define   LPCR_LPES_SH		2
// #define   LPCR_RMI		ASM_CONST(0x0000000000000002)      /* real mode is cache inhibit */
// #define   LPCR_HVICE		ASM_CONST(0x0000000000000002)      /* P9: HV interrupt enable */
// #define   LPCR_HDICE		ASM_CONST(0x0000000000000001)      /* Hyp Decr enable (HV,PR,EE) */
// #define   LPCR_UPRT		ASM_CONST(0x0000000000400000)      /* Use Process Table (ISA 3) */
// #define   LPCR_HR		ASM_CONST(0x0000000000100000)
// #ifndef SPRN_LPID
pub const SPRN_LPID: u64 = 0x13F	/* Logical Partition Identifier */;
// #endif
// #define	SPRN_HMER	0x150	/* Hypervisor maintenance exception reg */
// #define   HMER_DEBUG_TRIG	(1ul << (63 - 17)) /* Debug trigger */
// #define	SPRN_HMEER	0x151	/* Hyp maintenance exception enable reg */
pub const SPRN_PCR: u64 = 0x152	/* Processor compatibility register */;
// #define   PCR_VEC_DIS	(__MASK(63-0))	/* Vec. disable (bit NA since POWER8) */
// #define   PCR_VSX_DIS	(__MASK(63-1))	/* VSX disable (bit NA since POWER8) */
// #define   PCR_TM_DIS	(__MASK(63-2))	/* Trans. memory disable (POWER8) */
// #define   PCR_MMA_DIS	(__MASK(63-3)) /* Matrix-Multiply Accelerator */
// #define   PCR_HIGH_BITS	(PCR_MMA_DIS | PCR_VEC_DIS | PCR_VSX_DIS | PCR_TM_DIS)
/*
 * These bits are used in the function kvmppc_set_arch_compat() to specify and
 * determine both the compatibility level which we want to emulate and the
 * compatibility level which the host is capable of emulating.
 */
// #define   PCR_ARCH_31   0x20		/* Architecture 3.1 */
// #define   PCR_ARCH_300	0x10		/* Architecture 3.00 */
// #define   PCR_ARCH_207	0x8		/* Architecture 2.07 */
// #define   PCR_ARCH_206	0x4		/* Architecture 2.06 */
// #define   PCR_ARCH_205	0x2		/* Architecture 2.05 */
// #define   PCR_LOW_BITS	(PCR_ARCH_207 | PCR_ARCH_206 | PCR_ARCH_205 | PCR_ARCH_300 | PCR_ARCH_31)
// #define   PCR_MASK	~(PCR_HIGH_BITS | PCR_LOW_BITS)	/* PCR Reserved Bits */
// #define	SPRN_HEIR	0x153	/* Hypervisor Emulated Instruction Register */
pub const SPRN_TLBINDEXR: u64 = 0x154	/* P7 TLB control register */;
pub const SPRN_TLBVPNR: u64 = 0x155	/* P7 TLB control register */;
pub const SPRN_TLBRPNR: u64 = 0x156	/* P7 TLB control register */;
pub const SPRN_TLBLPIDR: u64 = 0x157	/* P7 TLB control register */;
pub const SPRN_DBAT0L: u64 = 0x219	/* Data BAT 0 Lower Register */;
pub const SPRN_DBAT0U: u64 = 0x218	/* Data BAT 0 Upper Register */;
pub const SPRN_DBAT1L: u64 = 0x21B	/* Data BAT 1 Lower Register */;
pub const SPRN_DBAT1U: u64 = 0x21A	/* Data BAT 1 Upper Register */;
pub const SPRN_DBAT2L: u64 = 0x21D	/* Data BAT 2 Lower Register */;
pub const SPRN_DBAT2U: u64 = 0x21C	/* Data BAT 2 Upper Register */;
pub const SPRN_DBAT3L: u64 = 0x21F	/* Data BAT 3 Lower Register */;
pub const SPRN_DBAT3U: u64 = 0x21E	/* Data BAT 3 Upper Register */;
pub const SPRN_DBAT4L: u64 = 0x239	/* Data BAT 4 Lower Register */;
pub const SPRN_DBAT4U: u64 = 0x238	/* Data BAT 4 Upper Register */;
pub const SPRN_DBAT5L: u64 = 0x23B	/* Data BAT 5 Lower Register */;
pub const SPRN_DBAT5U: u64 = 0x23A	/* Data BAT 5 Upper Register */;
pub const SPRN_DBAT6L: u64 = 0x23D	/* Data BAT 6 Lower Register */;
pub const SPRN_DBAT6U: u64 = 0x23C	/* Data BAT 6 Upper Register */;
pub const SPRN_DBAT7L: u64 = 0x23F	/* Data BAT 7 Lower Register */;
pub const SPRN_DBAT7U: u64 = 0x23E	/* Data BAT 7 Upper Register */;
pub const SPRN_PPR: u64 = 0x380	/* SMT Thread status Register */;
pub const SPRN_TSCR: u64 = 0x399	/* Thread Switch Control Register */;

pub const SPRN_DEC: u64 = 0x016		/* Decrement Register */;
pub const SPRN_PIT: u64 = 0x3DB		/* Programmable Interval Timer (BOOKE) */;

pub const SPRN_DER: u64 = 0x095		/* Debug Enable Register */;
pub const DER_RSTE: u64 = 0x40000000	/* Reset Interrupt */;
pub const DER_CHSTPE: u64 = 0x20000000	/* Check Stop */;
pub const DER_MCIE: u64 = 0x10000000	/* Machine Check Interrupt */;
pub const DER_EXTIE: u64 = 0x02000000	/* External Interrupt */;
pub const DER_ALIE: u64 = 0x01000000	/* Alignment Interrupt */;
pub const DER_PRIE: u64 = 0x00800000	/* Program Interrupt */;
pub const DER_FPUVIE: u64 = 0x00400000	/* FP Unavailable Interrupt */;
pub const DER_DECIE: u64 = 0x00200000	/* Decrementer Interrupt */;
pub const DER_SYSIE: u64 = 0x00040000	/* System Call Interrupt */;
pub const DER_TRE: u64 = 0x00020000	/* Trace Interrupt */;
pub const DER_SEIE: u64 = 0x00004000	/* FP SW Emulation Interrupt */;
pub const DER_ITLBMSE: u64 = 0x00002000	/* Imp. Spec. Instruction TLB Miss */;
pub const DER_ITLBERE: u64 = 0x00001000	/* Imp. Spec. Instruction TLB Error */;
pub const DER_DTLBMSE: u64 = 0x00000800	/* Imp. Spec. Data TLB Miss */;
pub const DER_DTLBERE: u64 = 0x00000400	/* Imp. Spec. Data TLB Error */;
pub const DER_LBRKE: u64 = 0x00000008	/* Load/Store Breakpoint Interrupt */;
pub const DER_IBRKE: u64 = 0x00000004	/* Instruction Breakpoint Interrupt */;
pub const DER_EBRKE: u64 = 0x00000002	/* External Breakpoint Interrupt */;
pub const DER_DPIE: u64 = 0x00000001	/* Dev. Port Nonmaskable Request */;
pub const SPRN_DMISS: u64 = 0x3D0		/* Data TLB Miss Register */;
pub const SPRN_DHDES: u64 = 0x0B1		/* Directed Hyp. Doorbell Exc. State */;
pub const SPRN_DPDES: u64 = 0x0B0		/* Directed Priv. Doorbell Exc. State */;
pub const SPRN_EAR: u64 = 0x11A		/* External Address Register */;
pub const SPRN_HASH1: u64 = 0x3D2		/* Primary Hash Address Register */;
pub const SPRN_HASH2: u64 = 0x3D3		/* Secondary Hash Address Register */;
pub const SPRN_HID0: u64 = 0x3F0		/* Hardware Implementation Register 0 */;
pub const HID0_HDICE_SH: u64 = (63 - 23)	/* 970 HDEC interrupt enable */;
pub const HID0_EMCP: u64 = (1<<31)		/* Enable Machine Check pin */;
pub const HID0_EBA: u64 = (1<<29)		/* Enable Bus Address Parity */;
pub const HID0_EBD: u64 = (1<<28)		/* Enable Bus Data Parity */;
pub const HID0_SBCLK: u64 = (1<<27);
pub const HID0_EICE: u64 = (1<<26);
pub const HID0_TBEN: u64 = (1<<26)		/* Timebase enable - 745x */;
pub const HID0_ECLK: u64 = (1<<25);
pub const HID0_PAR: u64 = (1<<24);
pub const HID0_STEN: u64 = (1<<24)		/* Software table search enable - 745x */;
pub const HID0_HIGH_BAT: u64 = (1<<23)		/* Enable high BATs - 7455 */;
pub const HID0_DOZE: u64 = (1<<23);
pub const HID0_NAP: u64 = (1<<22);
pub const HID0_SLEEP: u64 = (1<<21);
pub const HID0_DPM: u64 = (1<<20);
pub const HID0_BHTCLR: u64 = (1<<18)		/* Clear branch history table - 7450 */;
pub const HID0_XAEN: u64 = (1<<17)		/* Extended addressing enable - 7450 */;
pub const HID0_NHR: u64 = (1<<16)		/* Not hard reset (software bit-7450)*/;
pub const HID0_ICE: u64 = (1<<15)		/* Instruction Cache Enable */;
pub const HID0_DCE: u64 = (1<<14)		/* Data Cache Enable */;
pub const HID0_ILOCK: u64 = (1<<13)		/* Instruction Cache Lock */;
pub const HID0_DLOCK: u64 = (1<<12)		/* Data Cache Lock */;
pub const HID0_ICFI: u64 = (1<<11)		/* Instr. Cache Flash Invalidate */;
pub const HID0_DCI: u64 = (1<<10)		/* Data Cache Invalidate */;
pub const HID0_SPD: u64 = (1<<9)		/* Speculative disable */;
pub const HID0_DAPUEN: u64 = (1<<8)		/* Debug APU enable */;
pub const HID0_SGE: u64 = (1<<7)		/* Store Gathering Enable */;
pub const HID0_SIED: u64 = (1<<7)		/* Serial Instr. Execution [Disable] */;
pub const HID0_DCFA: u64 = (1<<6)		/* Data Cache Flush Assist */;
pub const HID0_LRSTK: u64 = (1<<4)		/* Link register stack - 745x */;
pub const HID0_BTIC: u64 = (1<<5)		/* Branch Target Instr Cache Enable */;
pub const HID0_ABE: u64 = (1<<3)		/* Address Broadcast Enable */;
pub const HID0_FOLD: u64 = (1<<3)		/* Branch Folding enable - 745x */;
pub const HID0_BHTE: u64 = (1<<2)		/* Branch History Table Enable */;
pub const HID0_BTCD: u64 = (1<<1)		/* Branch target cache disable */;
pub const HID0_NOPDST: u64 = (1<<1)		/* No-op dst, dstt, etc. instr. */;
pub const HID0_NOPTI: u64 = (1<<0)		/* No-op dcbt and dcbst instr. */;
/* POWER8 HID0 bits */
pub const HID0_POWER8_4LPARMODE: u64 = (1u64 << 61);
pub const HID0_POWER8_2LPARMODE: u64 = (1u64 << 57);
pub const HID0_POWER8_1TO2LPAR: u64 = (1u64 << 52);
pub const HID0_POWER8_1TO4LPAR: u64 = (1u64 << 51);
pub const HID0_POWER8_DYNLPARDIS: u64 = (1u64 << 48);

/* POWER9 HID0 bits */
pub const HID0_POWER9_RADIX: u64 = (1u64 << 63 - 8);

pub const SPRN_HID1: u64 = 0x3F1		/* Hardware Implementation Register 1 */;
// #ifdef CONFIG_PPC_BOOK3S_32
pub const HID1_EMCP: u64 = (1<<31)		/* 7450 Machine Check Pin Enable */;
pub const HID1_DFS: u64 = (1<<22)		/* 7447A Dynamic Frequency Scaling */;
pub const HID1_PC0: u64 = (1<<16)		/* 7450 PLL_CFG[0] */;
pub const HID1_PC1: u64 = (1<<15)		/* 7450 PLL_CFG[1] */;
pub const HID1_PC2: u64 = (1<<14)		/* 7450 PLL_CFG[2] */;
pub const HID1_PC3: u64 = (1<<13)		/* 7450 PLL_CFG[3] */;
pub const HID1_SYNCBE: u64 = (1<<11)		/* 7450 ABE for sync, eieio */;
pub const HID1_ABE: u64 = (1<<10)		/* 7450 Address Broadcast Enable */;
pub const HID1_PS: u64 = (1<<16)		/* 750FX PLL selection */;
// #endif
pub const SPRN_HID2_750FX: u64 = 0x3F8		/* IBM 750FX HID2 Register */;
pub const SPRN_HID2_GEKKO: u64 = 0x398		/* Gekko HID2 Register */;
pub const SPRN_HID2_G2_LE: u64 = 0x3F3		/* G2_LE HID2 Register */;
// #define  HID2_G2_LE_HBE	(1<<18)		/* High BAT Enable (G2_LE) */
pub const SPRN_IABR: u64 = 0x3F2	/* Instruction Address Breakpoint Register */;
pub const SPRN_IABR2: u64 = 0x3FA		/* 83xx */;
pub const SPRN_IBCR: u64 = 0x135		/* 83xx Insn Breakpoint Control Reg */;
pub const SPRN_IAMR: u64 = 0x03D		/* Instr. Authority Mask Reg */;
pub const SPRN_HID4: u64 = 0x3F4		/* 970 HID4 */;
// #define  HID4_LPES0	 (1ul << (63-0)) /* LPAR env. sel. bit 0 */
// #define	 HID4_RMLS2_SH	 (63 - 2)	/* Real mode limit bottom 2 bits */
// #define	 HID4_LPID5_SH	 (63 - 6)	/* partition ID bottom 4 bits */
// #define	 HID4_RMOR_SH	 (63 - 22)	/* real mode offset (16 bits) */
// #define  HID4_RMOR	 (0xFFFFul << HID4_RMOR_SH)
// #define  HID4_LPES1	 (1 << (63-57))	/* LPAR env. sel. bit 1 */
// #define  HID4_RMLS0_SH	 (63 - 58)	/* Real mode limit top bit */
// #define	 HID4_LPID1_SH	 0		/* partition ID top 2 bits */
pub const SPRN_HID4_GEKKO: u64 = 0x3F3		/* Gekko HID4 */;
pub const SPRN_HID5: u64 = 0x3F6		/* 970 HID5 */;
pub const SPRN_HID6: u64 = 0x3F9	/* BE HID 6 */;
// #define   HID6_LB	(0x0F<<12) /* Concurrent Large Page Modes */
// #define   HID6_DLP	(1<<20)	/* Disable all large page modes (4K only) */
pub const SPRN_TSC_CELL: u64 = 0x399	/* Thread switch control on Cell */;
// #define   TSC_CELL_DEC_ENABLE_0	0x400000 /* Decrementer Interrupt */
// #define   TSC_CELL_DEC_ENABLE_1	0x200000 /* Decrementer Interrupt */
// #define   TSC_CELL_EE_ENABLE	0x100000 /* External Interrupt */
// #define   TSC_CELL_EE_BOOST	0x080000 /* External Interrupt Boost */
pub const SPRN_TSC: u64 = 0x3FD	/* Thread switch control on others */;
pub const SPRN_TST: u64 = 0x3FC	/* Thread switch timeout on others */;
// #if !defined(SPRN_IAC1) && !defined(SPRN_IAC2)
pub const SPRN_IAC1: u64 = 0x3F4		/* Instruction Address Compare 1 */;
pub const SPRN_IAC2: u64 = 0x3F5		/* Instruction Address Compare 2 */;
// #endif
pub const SPRN_IBAT0L: u64 = 0x211		/* Instruction BAT 0 Lower Register */;
pub const SPRN_IBAT0U: u64 = 0x210		/* Instruction BAT 0 Upper Register */;
pub const SPRN_IBAT1L: u64 = 0x213		/* Instruction BAT 1 Lower Register */;
pub const SPRN_IBAT1U: u64 = 0x212		/* Instruction BAT 1 Upper Register */;
pub const SPRN_IBAT2L: u64 = 0x215		/* Instruction BAT 2 Lower Register */;
pub const SPRN_IBAT2U: u64 = 0x214		/* Instruction BAT 2 Upper Register */;
pub const SPRN_IBAT3L: u64 = 0x217		/* Instruction BAT 3 Lower Register */;
pub const SPRN_IBAT3U: u64 = 0x216		/* Instruction BAT 3 Upper Register */;
pub const SPRN_IBAT4L: u64 = 0x231		/* Instruction BAT 4 Lower Register */;
pub const SPRN_IBAT4U: u64 = 0x230		/* Instruction BAT 4 Upper Register */;
pub const SPRN_IBAT5L: u64 = 0x233		/* Instruction BAT 5 Lower Register */;
pub const SPRN_IBAT5U: u64 = 0x232		/* Instruction BAT 5 Upper Register */;
pub const SPRN_IBAT6L: u64 = 0x235		/* Instruction BAT 6 Lower Register */;
pub const SPRN_IBAT6U: u64 = 0x234		/* Instruction BAT 6 Upper Register */;
pub const SPRN_IBAT7L: u64 = 0x237		/* Instruction BAT 7 Lower Register */;
pub const SPRN_IBAT7U: u64 = 0x236		/* Instruction BAT 7 Upper Register */;
pub const SPRN_ICMP: u64 = 0x3D5		/* Instruction TLB Compare Register */;
pub const SPRN_ICTC: u64 = 0x3FB	/* Instruction Cache Throttling Control Reg */;
// #ifndef SPRN_ICTRL
pub const SPRN_ICTRL: u64 = 0x3F3	/* 1011 7450 icache and interrupt ctrl */;
// #endif
pub const ICTRL_EICE: u64 = 0x08000000	/* enable icache parity errs */;
pub const ICTRL_EDC: u64 = 0x04000000	/* enable dcache parity errs */;
pub const ICTRL_EICP: u64 = 0x00000100	/* enable icache par. check */;
pub const SPRN_IMISS: u64 = 0x3D4		/* Instruction TLB Miss Register */;
pub const SPRN_IMMR: u64 = 0x27E		/* Internal Memory Map Register */;
pub const SPRN_L2CR: u64 = 0x3F9		/* Level 2 Cache Control Register */;
pub const SPRN_L2CR2: u64 = 0x3f8;
pub const L2CR_L2E: u64 = 0x80000000	/* L2 enable */;
pub const L2CR_L2PE: u64 = 0x40000000	/* L2 parity enable */;
pub const L2CR_L2SIZ_MASK: u64 = 0x30000000	/* L2 size mask */;
pub const L2CR_L2SIZ_256KB: u64 = 0x10000000	/* L2 size 256KB */;
pub const L2CR_L2SIZ_512KB: u64 = 0x20000000	/* L2 size 512KB */;
pub const L2CR_L2SIZ_1MB: u64 = 0x30000000	/* L2 size 1MB */;
pub const L2CR_L2CLK_MASK: u64 = 0x0e000000	/* L2 clock mask */;
pub const L2CR_L2CLK_DISABLED: u64 = 0x00000000	/* L2 clock disabled */;
pub const L2CR_L2CLK_DIV1: u64 = 0x02000000	/* L2 clock / 1 */;
pub const L2CR_L2CLK_DIV1_5: u64 = 0x04000000	/* L2 clock / 1.5 */;
pub const L2CR_L2CLK_DIV2: u64 = 0x08000000	/* L2 clock / 2 */;
pub const L2CR_L2CLK_DIV2_5: u64 = 0x0a000000	/* L2 clock / 2.5 */;
pub const L2CR_L2CLK_DIV3: u64 = 0x0c000000	/* L2 clock / 3 */;
pub const L2CR_L2RAM_MASK: u64 = 0x01800000	/* L2 RAM type mask */;
pub const L2CR_L2RAM_FLOW: u64 = 0x00000000	/* L2 RAM flow through */;
pub const L2CR_L2RAM_PIPE: u64 = 0x01000000	/* L2 RAM pipelined */;
pub const L2CR_L2RAM_PIPE_LW: u64 = 0x01800000	/* L2 RAM pipelined latewr */;
pub const L2CR_L2DO: u64 = 0x00400000	/* L2 data only */;
pub const L2CR_L2I: u64 = 0x00200000	/* L2 global invalidate */;
pub const L2CR_L2CTL: u64 = 0x00100000	/* L2 RAM control */;
pub const L2CR_L2WT: u64 = 0x00080000	/* L2 write-through */;
pub const L2CR_L2TS: u64 = 0x00040000	/* L2 test support */;
pub const L2CR_L2OH_MASK: u64 = 0x00030000	/* L2 output hold mask */;
pub const L2CR_L2OH_0_5: u64 = 0x00000000	/* L2 output hold 0.5 ns */;
pub const L2CR_L2OH_1_0: u64 = 0x00010000	/* L2 output hold 1.0 ns */;
pub const L2CR_L2SL: u64 = 0x00008000	/* L2 DLL slow */;
pub const L2CR_L2DF: u64 = 0x00004000	/* L2 differential clock */;
pub const L2CR_L2BYP: u64 = 0x00002000	/* L2 DLL bypass */;
pub const L2CR_L2IP: u64 = 0x00000001	/* L2 GI in progress */;
pub const L2CR_L2IO_745x: u64 = 0x00100000	/* L2 instr. only (745x) */;
pub const L2CR_L2DO_745x: u64 = 0x00010000	/* L2 data only (745x) */;
pub const L2CR_L2REP_745x: u64 = 0x00001000	/* L2 repl. algorithm (745x) */;
pub const L2CR_L2HWF_745x: u64 = 0x00000800	/* L2 hardware flush (745x) */;
pub const SPRN_L3CR: u64 = 0x3FA	/* Level 3 Cache Control Register */;
pub const L3CR_L3E: u64 = 0x80000000	/* L3 enable */;
pub const L3CR_L3PE: u64 = 0x40000000	/* L3 data parity enable */;
pub const L3CR_L3APE: u64 = 0x20000000	/* L3 addr parity enable */;
pub const L3CR_L3SIZ: u64 = 0x10000000	/* L3 size */;
pub const L3CR_L3CLKEN: u64 = 0x08000000	/* L3 clock enable */;
pub const L3CR_L3RES: u64 = 0x04000000	/* L3 special reserved bit */;
pub const L3CR_L3CLKDIV: u64 = 0x03800000	/* L3 clock divisor */;
pub const L3CR_L3IO: u64 = 0x00400000	/* L3 instruction only */;
pub const L3CR_L3SPO: u64 = 0x00040000	/* L3 sample point override */;
pub const L3CR_L3CKSP: u64 = 0x00030000	/* L3 clock sample point */;
pub const L3CR_L3PSP: u64 = 0x0000e000	/* L3 P-clock sample point */;
pub const L3CR_L3REP: u64 = 0x00001000	/* L3 replacement algorithm */;
pub const L3CR_L3HWF: u64 = 0x00000800	/* L3 hardware flush */;
pub const L3CR_L3I: u64 = 0x00000400	/* L3 global invalidate */;
pub const L3CR_L3RT: u64 = 0x00000300	/* L3 SRAM type */;
pub const L3CR_L3NIRCA: u64 = 0x00000080	/* L3 non-integer ratio clock adj. */;
pub const L3CR_L3DO: u64 = 0x00000040	/* L3 data only mode */;
pub const L3CR_PMEN: u64 = 0x00000004	/* L3 private memory enable */;
pub const L3CR_PMSIZ: u64 = 0x00000001	/* L3 private memory size */;

pub const SPRN_MSSCR0: u64 = 0x3f6	/* Memory Subsystem Control Register 0 */;
pub const SPRN_MSSSR0: u64 = 0x3f7	/* Memory Subsystem Status Register 1 */;
pub const SPRN_LDSTCR: u64 = 0x3f8	/* Load/Store control register */;
pub const SPRN_LDSTDB: u64 = 0x3f4	/* */;
pub const SPRN_LR: u64 = 0x008	/* Link Register */;
// #ifndef SPRN_PIR
pub const SPRN_PIR: u64 = 0x3FF	/* Processor Identification Register */;
// #endif
pub const SPRN_TIR: u64 = 0x1BE	/* Thread Identification Register */;
pub const SPRN_PTCR: u64 = 0x1D0	/* Partition table control Register */;
pub const SPRN_PSPB: u64 = 0x09F	/* Problem State Priority Boost reg */;
pub const SPRN_PTEHI: u64 = 0x3D5	/* 981 7450 PTE HI word (S/W TLB load) */;
pub const SPRN_PTELO: u64 = 0x3D6	/* 982 7450 PTE LO word (S/W TLB load) */;
pub const SPRN_PURR: u64 = 0x135	/* Processor Utilization of Resources Reg */;
pub const SPRN_PVR: u64 = 0x11F	/* Processor Version Register */;
pub const SPRN_RPA: u64 = 0x3D6	/* Required Physical Address Register */;
pub const SPRN_SDA: u64 = 0x3BF	/* Sampled Data Address Register */;
pub const SPRN_SDR1: u64 = 0x019	/* MMU Hash Base Register */;
pub const SPRN_ASR: u64 = 0x118   /* Address Space Register */;
pub const SPRN_SIA: u64 = 0x3BB	/* Sampled Instruction Address Register */;
pub const SPRN_SPRG0: u64 = 0x110	/* Special Purpose Register General 0 */;
pub const SPRN_SPRG1: u64 = 0x111	/* Special Purpose Register General 1 */;
pub const SPRN_SPRG2: u64 = 0x112	/* Special Purpose Register General 2 */;
pub const SPRN_SPRG3: u64 = 0x113	/* Special Purpose Register General 3 */;
pub const SPRN_USPRG3: u64 = 0x103	/* SPRG3 userspace read */;
pub const SPRN_SPRG4: u64 = 0x114	/* Special Purpose Register General 4 */;
pub const SPRN_USPRG4: u64 = 0x104	/* SPRG4 userspace read */;
pub const SPRN_SPRG5: u64 = 0x115	/* Special Purpose Register General 5 */;
pub const SPRN_USPRG5: u64 = 0x105	/* SPRG5 userspace read */;
pub const SPRN_SPRG6: u64 = 0x116	/* Special Purpose Register General 6 */;
pub const SPRN_USPRG6: u64 = 0x106	/* SPRG6 userspace read */;
pub const SPRN_SPRG7: u64 = 0x117	/* Special Purpose Register General 7 */;
pub const SPRN_USPRG7: u64 = 0x107	/* SPRG7 userspace read */;
pub const SPRN_SRR0: u64 = 0x01A	/* Save/Restore Register 0 */;
pub const SPRN_SRR1: u64 = 0x01B	/* Save/Restore Register 1 */;

// #ifdef CONFIG_PPC_BOOK3S
/*
 * Bits loaded from MSR upon interrupt.
 * PPC (64-bit) bits 33-36,42-47 are interrupt dependent, the others are
 * loaded from MSR. The exception is that SRESET and MCE do not always load
 * bit 62 (RI) from MSR. Don't use PPC_BITMASK for this because 32-bit uses
 * it.
 */
// #define   SRR1_MSR_BITS		(~0x783f0000UL)
// #endif

// #define   SRR1_ISI_NOPT		0x40000000 /* ISI: Not found in hash */
// #define   SRR1_ISI_N_G_OR_CIP	0x10000000 /* ISI: Access is no-exec or G or CI for a prefixed instruction */
// #define   SRR1_ISI_PROT		0x08000000 /* ISI: Other protection fault */
// #define   SRR1_WAKEMASK		0x00380000 /* reason for wakeup */
// #define   SRR1_WAKEMASK_P8	0x003c0000 /* reason for wakeup on POWER8 and 9 */
// #define   SRR1_WAKEMCE_RESVD	0x003c0000 /* Unused/reserved value used by MCE wakeup to indicate cause to idle wakeup handler */
// #define   SRR1_WAKESYSERR	0x00300000 /* System error */
// #define   SRR1_WAKEEE		0x00200000 /* External interrupt */
// #define   SRR1_WAKEHVI		0x00240000 /* Hypervisor Virtualization Interrupt (P9) */
// #define   SRR1_WAKEMT		0x00280000 /* mtctrl */
// #define	  SRR1_WAKEHMI		0x00280000 /* Hypervisor maintenance */
// #define   SRR1_WAKEDEC		0x00180000 /* Decrementer interrupt */
// #define   SRR1_WAKEDBELL	0x00140000 /* Privileged doorbell on P8 */
// #define   SRR1_WAKETHERM	0x00100000 /* Thermal management interrupt */
// #define	  SRR1_WAKERESET	0x00100000 /* System reset */
// #define   SRR1_WAKEHDBELL	0x000c0000 /* Hypervisor doorbell on P8 */
// #define	  SRR1_WAKESTATE	0x00030000 /* Powersave exit mask [46:47] */
// #define	  SRR1_WS_HVLOSS	0x00030000 /* HV resources not maintained */
// #define	  SRR1_WS_GPRLOSS	0x00020000 /* GPRs not maintained */
// #define	  SRR1_WS_NOLOSS	0x00010000 /* All resources maintained */
// #define   SRR1_PROGTM		0x00200000 /* TM Bad Thing */
// #define   SRR1_PROGFPE		0x00100000 /* Floating Point Enabled */
// #define   SRR1_PROGILL		0x00080000 /* Illegal instruction */
// #define   SRR1_PROGPRIV		0x00040000 /* Privileged instruction */
// #define   SRR1_PROGTRAP		0x00020000 /* Trap */
// #define   SRR1_PROGADDR		0x00010000 /* SRR0 contains subsequent addr */

// #define   SRR1_MCE_MCP		0x00080000 /* Machine check signal caused interrupt */
// #define   SRR1_BOUNDARY		0x10000000 /* Prefixed instruction crosses 64-byte boundary */
// #define   SRR1_PREFIXED		0x20000000 /* Exception caused by prefixed instruction */

pub const SPRN_HSRR0: u64 = 0x13A	/* Save/Restore Register 0 */;
pub const SPRN_HSRR1: u64 = 0x13B	/* Save/Restore Register 1 */;
// #define   HSRR1_DENORM		0x00100000 /* Denorm exception */
// #define   HSRR1_HISI_WRITE	0x00010000 /* HISI bcs couldn't update mem */

pub const SPRN_TBCTL: u64 = 0x35f	/* PA6T Timebase control register */;
// #define   TBCTL_FREEZE		0x0000000000000000ull /* Freeze all tbs */
// #define   TBCTL_RESTART		0x0000000100000000ull /* Restart all tbs */
// #define   TBCTL_UPDATE_UPPER	0x0000000200000000ull /* Set upper 32 bits */
// #define   TBCTL_UPDATE_LOWER	0x0000000300000000ull /* Set lower 32 bits */

// #ifndef SPRN_SVR
pub const SPRN_SVR: u64 = 0x11E	/* System Version Register */;
// #endif
pub const SPRN_THRM1: u64 = 0x3FC		/* Thermal Management Register 1 */;
/* these bits were defined in inverted endian sense originally, ugh, confusing */
pub const THRM1_TIN: u64 = (1 << 31);
pub const THRM1_TIV: u64 = (1 << 30);
// #define THRM1_THRES(x)	((x&0x7f)<<23)
// #define THRM3_SITV(x)	((x & 0x1fff) << 1)
pub const THRM1_TID: u64 = (1<<2);
pub const THRM1_TIE: u64 = (1<<1);
pub const THRM1_V: u64 = (1<<0);
pub const SPRN_THRM2: u64 = 0x3FD		/* Thermal Management Register 2 */;
pub const SPRN_THRM3: u64 = 0x3FE		/* Thermal Management Register 3 */;
pub const THRM3_E: u64 = (1<<0);
pub const SPRN_TLBMISS: u64 = 0x3D4		/* 980 7450 TLB Miss Register */;
pub const SPRN_UMMCR0: u64 = 0x3A8	/* User Monitor Mode Control Register 0 */;
pub const SPRN_UMMCR1: u64 = 0x3AC	/* User Monitor Mode Control Register 0 */;
pub const SPRN_UPMC1: u64 = 0x3A9	/* User Performance Counter Register 1 */;
pub const SPRN_UPMC2: u64 = 0x3AA	/* User Performance Counter Register 2 */;
pub const SPRN_UPMC3: u64 = 0x3AD	/* User Performance Counter Register 3 */;
pub const SPRN_UPMC4: u64 = 0x3AE	/* User Performance Counter Register 4 */;
pub const SPRN_USIA: u64 = 0x3AB	/* User Sampled Instruction Address Register */;
pub const SPRN_VRSAVE: u64 = 0x100	/* Vector Register Save Register */;
pub const SPRN_XER: u64 = 0x001	/* Fixed Point Exception Register */;

pub const SPRN_MMCR0_GEKKO: u64 = 0x3B8 /* Gekko Monitor Mode Control Register 0 */;
pub const SPRN_MMCR1_GEKKO: u64 = 0x3BC /* Gekko Monitor Mode Control Register 1 */;
pub const SPRN_PMC1_GEKKO: u64 = 0x3B9 /* Gekko Performance Monitor Control 1 */;
pub const SPRN_PMC2_GEKKO: u64 = 0x3BA /* Gekko Performance Monitor Control 2 */;
pub const SPRN_PMC3_GEKKO: u64 = 0x3BD /* Gekko Performance Monitor Control 3 */;
pub const SPRN_PMC4_GEKKO: u64 = 0x3BE /* Gekko Performance Monitor Control 4 */;
pub const SPRN_WPAR_GEKKO: u64 = 0x399 /* Gekko Write Pipe Address Register */;

pub const SPRN_SCOMC: u64 = 0x114	/* SCOM Access Control */;
pub const SPRN_SCOMD: u64 = 0x115	/* SCOM Access DATA */;

/* Performance monitor SPRs */
// #ifdef CONFIG_PPC64
pub const SPRN_MMCR0: u64 = 795;
// #define   MMCR0_FC	0x80000000UL /* freeze counters */
// #define   MMCR0_FCS	0x40000000UL /* freeze in supervisor state */
// #define   MMCR0_KERNEL_DISABLE MMCR0_FCS
// #define   MMCR0_FCP	0x20000000UL /* freeze in problem state */
// #define   MMCR0_PROBLEM_DISABLE MMCR0_FCP
// #define   MMCR0_FCM1	0x10000000UL /* freeze counters while MSR mark = 1 */
// #define   MMCR0_FCM0	0x08000000UL /* freeze counters while MSR mark = 0 */
// #define   MMCR0_PMXE	ASM_CONST(0x04000000) /* perf mon exception enable */
// #define   MMCR0_FCECE	ASM_CONST(0x02000000) /* freeze ctrs on enabled cond or event */
// #define   MMCR0_TBEE	0x00400000UL /* time base exception enable */
// #define   MMCR0_BHRBA	0x00200000UL /* BHRB Access allowed in userspace */
// #define   MMCR0_EBE	0x00100000UL /* Event based branch enable */
// #define   MMCR0_PMCC	0x000c0000UL /* PMC control */
// #define   MMCR0_PMCCEXT	ASM_CONST(0x00000200) /* PMCCEXT control */
// #define   MMCR0_PMCC_U6	0x00080000UL /* PMC1-6 are R/W by user (PR) */
// #define   MMCR0_PMC1CE	0x00008000UL /* PMC1 count enable*/
// #define   MMCR0_PMCjCE	ASM_CONST(0x00004000) /* PMCj count enable*/
// #define   MMCR0_TRIGGER	0x00002000UL /* TRIGGER enable */
// #define   MMCR0_PMAO_SYNC ASM_CONST(0x00000800) /* PMU intr is synchronous */
// #define   MMCR0_C56RUN	ASM_CONST(0x00000100) /* PMC5/6 count when RUN=0 */
/* performance monitor alert has occurred, set to 0 after handling exception */
// #define   MMCR0_PMAO	ASM_CONST(0x00000080)
// #define   MMCR0_SHRFC	0x00000040UL /* SHRre freeze conditions between threads */
// #define   MMCR0_FC56	0x00000010UL /* freeze counters 5 and 6 */
// #define   MMCR0_FCTI	0x00000008UL /* freeze counters in tags inactive mode */
// #define   MMCR0_FCTA	0x00000004UL /* freeze counters in tags active mode */
// #define   MMCR0_FCWAIT	0x00000002UL /* freeze counter in WAIT state */
// #define   MMCR0_FCHV	0x00000001UL /* freeze conditions in hypervisor mode */
pub const SPRN_MMCR1: u64 = 798;
pub const SPRN_MMCR2: u64 = 785;
pub const SPRN_MMCR3: u64 = 754;
pub const SPRN_UMMCR2: u64 = 769;
pub const SPRN_UMMCR3: u64 = 738;
pub const SPRN_MMCRA: u64 = 0x312;
// #define   MMCRA_SDSYNC	0x80000000UL /* SDAR synced with SIAR */
// #define   MMCRA_SDAR_DCACHE_MISS 0x40000000UL
// #define   MMCRA_SDAR_ERAT_MISS   0x20000000UL
// #define   MMCRA_SIHV	0x10000000UL /* state of MSR HV when SIAR set */
// #define   MMCRA_SIPR	0x08000000UL /* state of MSR PR when SIAR set */
// #define   MMCRA_SLOT	0x07000000UL /* SLOT bits (37-39) */
// #define   MMCRA_SLOT_SHIFT	24
// #define   MMCRA_SAMPLE_ENABLE 0x00000001UL /* enable sampling */
// #define   MMCRA_BHRB_DISABLE  _UL(0x2000000000) // BHRB disable bit for ISA v3.1
// #define   POWER6_MMCRA_SDSYNC 0x0000080000000000ULL	/* SDAR/SIAR synced */
// #define   POWER6_MMCRA_SIHV   0x0000040000000000ULL
// #define   POWER6_MMCRA_SIPR   0x0000020000000000ULL
// #define   POWER6_MMCRA_THRM	0x00000020UL
// #define   POWER6_MMCRA_OTHER	0x0000000EUL

// #define   POWER7P_MMCRA_SIAR_VALID 0x10000000	/* P7+ SIAR contents valid */
// #define   POWER7P_MMCRA_SDAR_VALID 0x08000000	/* P7+ SDAR contents valid */

pub const SPRN_MMCRH: u64 = 316	/* Hypervisor monitor mode control register */;
pub const SPRN_MMCRS: u64 = 894	/* Supervisor monitor mode control register */;
pub const SPRN_MMCRC: u64 = 851	/* Core monitor mode control register */;
pub const SPRN_EBBHR: u64 = 804	/* Event based branch handler register */;
pub const SPRN_EBBRR: u64 = 805	/* Event based branch return register */;
pub const SPRN_BESCR: u64 = 806	/* Branch event status and control register */;
// #define   BESCR_GE	0x8000000000000000ULL /* Global Enable */
pub const SPRN_WORT: u64 = 895	/* Workload optimization register - thread */;
pub const SPRN_WORC: u64 = 863	/* Workload optimization register - core */;

pub const SPRN_PMC1: u64 = 787;
pub const SPRN_PMC2: u64 = 788;
pub const SPRN_PMC3: u64 = 789;
pub const SPRN_PMC4: u64 = 790;
pub const SPRN_PMC5: u64 = 791;
pub const SPRN_PMC6: u64 = 792;
pub const SPRN_PMC7: u64 = 793;
pub const SPRN_PMC8: u64 = 794;
pub const SPRN_SIER: u64 = 784;
// #define   SIER_SIPR		0x2000000	/* Sampled MSR_PR */
// #define   SIER_SIHV		0x1000000	/* Sampled MSR_HV */
// #define   SIER_SIAR_VALID	0x0400000	/* SIAR contents valid */
// #define   SIER_SDAR_VALID	0x0200000	/* SDAR contents valid */
pub const SPRN_SIER2: u64 = 752;
pub const SPRN_SIER3: u64 = 753;
pub const SPRN_USIER2: u64 = 736;
pub const SPRN_USIER3: u64 = 737;
pub const SPRN_SIAR: u64 = 796;
pub const SPRN_SDAR: u64 = 797;
pub const SPRN_TACR: u64 = 888;
pub const SPRN_TCSCR: u64 = 889;
pub const SPRN_CSIGR: u64 = 890;
pub const SPRN_SPMC1: u64 = 892;
pub const SPRN_SPMC2: u64 = 893;

/* When EBB is enabled, some of MMCR0/MMCR2/SIER are user accessible */
pub const MMCR0_USER_MASK: u64 = (MMCR0_FC | MMCR0_PMXE | MMCR0_PMAO);
pub const MMCR2_USER_MASK: u64 = 0x4020100804020000u64 /* (FC1P|FC2P|FC3P|FC4P|FC5P|FC6P) */;
pub const SIER_USER_MASK: u64 = 0x7fffffu64;

pub const SPRN_PA6T_MMCR0: u64 = 795;
// #define   PA6T_MMCR0_EN0	0x0000000000000001UL
// #define   PA6T_MMCR0_EN1	0x0000000000000002UL
// #define   PA6T_MMCR0_EN2	0x0000000000000004UL
// #define   PA6T_MMCR0_EN3	0x0000000000000008UL
// #define   PA6T_MMCR0_EN4	0x0000000000000010UL
// #define   PA6T_MMCR0_EN5	0x0000000000000020UL
// #define   PA6T_MMCR0_SUPEN	0x0000000000000040UL
// #define   PA6T_MMCR0_PREN	0x0000000000000080UL
// #define   PA6T_MMCR0_HYPEN	0x0000000000000100UL
// #define   PA6T_MMCR0_FCM0	0x0000000000000200UL
// #define   PA6T_MMCR0_FCM1	0x0000000000000400UL
// #define   PA6T_MMCR0_INTGEN	0x0000000000000800UL
// #define   PA6T_MMCR0_INTEN0	0x0000000000001000UL
// #define   PA6T_MMCR0_INTEN1	0x0000000000002000UL
// #define   PA6T_MMCR0_INTEN2	0x0000000000004000UL
// #define   PA6T_MMCR0_INTEN3	0x0000000000008000UL
// #define   PA6T_MMCR0_INTEN4	0x0000000000010000UL
// #define   PA6T_MMCR0_INTEN5	0x0000000000020000UL
// #define   PA6T_MMCR0_DISCNT	0x0000000000040000UL
// #define   PA6T_MMCR0_UOP	0x0000000000080000UL
// #define   PA6T_MMCR0_TRG	0x0000000000100000UL
// #define   PA6T_MMCR0_TRGEN	0x0000000000200000UL
// #define   PA6T_MMCR0_TRGREG	0x0000000001600000UL
// #define   PA6T_MMCR0_SIARLOG	0x0000000002000000UL
// #define   PA6T_MMCR0_SDARLOG	0x0000000004000000UL
// #define   PA6T_MMCR0_PROEN	0x0000000008000000UL
// #define   PA6T_MMCR0_PROLOG	0x0000000010000000UL
// #define   PA6T_MMCR0_DAMEN2	0x0000000020000000UL
// #define   PA6T_MMCR0_DAMEN3	0x0000000040000000UL
// #define   PA6T_MMCR0_DAMEN4	0x0000000080000000UL
// #define   PA6T_MMCR0_DAMEN5	0x0000000100000000UL
// #define   PA6T_MMCR0_DAMSEL2	0x0000000200000000UL
// #define   PA6T_MMCR0_DAMSEL3	0x0000000400000000UL
// #define   PA6T_MMCR0_DAMSEL4	0x0000000800000000UL
// #define   PA6T_MMCR0_DAMSEL5	0x0000001000000000UL
// #define   PA6T_MMCR0_HANDDIS	0x0000002000000000UL
// #define   PA6T_MMCR0_PCTEN	0x0000004000000000UL
// #define   PA6T_MMCR0_SOCEN	0x0000008000000000UL
// #define   PA6T_MMCR0_SOCMOD	0x0000010000000000UL

pub const SPRN_PA6T_MMCR1: u64 = 798;
// #define   PA6T_MMCR1_ES2	0x00000000000000ffUL
// #define   PA6T_MMCR1_ES3	0x000000000000ff00UL
// #define   PA6T_MMCR1_ES4	0x0000000000ff0000UL
// #define   PA6T_MMCR1_ES5	0x00000000ff000000UL

pub const SPRN_PA6T_UPMC0: u64 = 771	/* User PerfMon Counter 0 */;
pub const SPRN_PA6T_UPMC1: u64 = 772	/* ... */;
pub const SPRN_PA6T_UPMC2: u64 = 773;
pub const SPRN_PA6T_UPMC3: u64 = 774;
pub const SPRN_PA6T_UPMC4: u64 = 775;
pub const SPRN_PA6T_UPMC5: u64 = 776;
pub const SPRN_PA6T_UMMCR0: u64 = 779	/* User Monitor Mode Control Register 0 */;
pub const SPRN_PA6T_SIAR: u64 = 780	/* Sampled Instruction Address */;
pub const SPRN_PA6T_UMMCR1: u64 = 782	/* User Monitor Mode Control Register 1 */;
pub const SPRN_PA6T_SIER: u64 = 785	/* Sampled Instruction Event Register */;
pub const SPRN_PA6T_PMC0: u64 = 787;
pub const SPRN_PA6T_PMC1: u64 = 788;
pub const SPRN_PA6T_PMC2: u64 = 789;
pub const SPRN_PA6T_PMC3: u64 = 790;
pub const SPRN_PA6T_PMC4: u64 = 791;
pub const SPRN_PA6T_PMC5: u64 = 792;
pub const SPRN_PA6T_TSR0: u64 = 793	/* Timestamp Register 0 */;
pub const SPRN_PA6T_TSR1: u64 = 794	/* Timestamp Register 1 */;
pub const SPRN_PA6T_TSR2: u64 = 799	/* Timestamp Register 2 */;
pub const SPRN_PA6T_TSR3: u64 = 784	/* Timestamp Register 3 */;

pub const SPRN_PA6T_IER: u64 = 981	/* Icache Error Register */;
pub const SPRN_PA6T_DER: u64 = 982	/* Dcache Error Register */;
pub const SPRN_PA6T_BER: u64 = 862	/* BIU Error Address Register */;
pub const SPRN_PA6T_MER: u64 = 849	/* MMU Error Register */;

pub const SPRN_PA6T_IMA0: u64 = 880	/* Instruction Match Array 0 */;
pub const SPRN_PA6T_IMA1: u64 = 881	/* ... */;
pub const SPRN_PA6T_IMA2: u64 = 882;
pub const SPRN_PA6T_IMA3: u64 = 883;
pub const SPRN_PA6T_IMA4: u64 = 884;
pub const SPRN_PA6T_IMA5: u64 = 885;
pub const SPRN_PA6T_IMA6: u64 = 886;
pub const SPRN_PA6T_IMA7: u64 = 887;
pub const SPRN_PA6T_IMA8: u64 = 888;
pub const SPRN_PA6T_IMA9: u64 = 889;
pub const SPRN_PA6T_BTCR: u64 = 978	/* Breakpoint and Tagging Control Register */;
pub const SPRN_PA6T_IMAAT: u64 = 979	/* Instruction Match Array Action Table */;
pub const SPRN_PA6T_PCCR: u64 = 1019	/* Power Counter Control Register */;
pub const SPRN_BKMK: u64 = 1020	/* Cell Bookmark Register */;
pub const SPRN_PA6T_RPCCR: u64 = 1021	/* Retire PC Trace Control Register */;


// #else /* 32-bit */
pub const SPRN_MMCR0: u64 = 952	/* Monitor Mode Control Register 0 */;
// #define   MMCR0_FC	0x80000000UL /* freeze counters */
// #define   MMCR0_FCS	0x40000000UL /* freeze in supervisor state */
// #define   MMCR0_FCP	0x20000000UL /* freeze in problem state */
// #define   MMCR0_FCM1	0x10000000UL /* freeze counters while MSR mark = 1 */
// #define   MMCR0_FCM0	0x08000000UL /* freeze counters while MSR mark = 0 */
// #define   MMCR0_PMXE	0x04000000UL /* performance monitor exception enable */
// #define   MMCR0_FCECE	0x02000000UL /* freeze ctrs on enabled cond or event */
// #define   MMCR0_TBEE	0x00400000UL /* time base exception enable */
// #define   MMCR0_PMC1CE	0x00008000UL /* PMC1 count enable*/
// #define   MMCR0_PMCnCE	0x00004000UL /* count enable for all but PMC 1*/
// #define   MMCR0_TRIGGER	0x00002000UL /* TRIGGER enable */
// #define   MMCR0_PMC1SEL	0x00001fc0UL /* PMC 1 Event */
// #define   MMCR0_PMC2SEL	0x0000003fUL /* PMC 2 Event */

pub const SPRN_MMCR1: u64 = 956;
// #define   MMCR1_PMC3SEL	0xf8000000UL /* PMC 3 Event */
// #define   MMCR1_PMC4SEL	0x07c00000UL /* PMC 4 Event */
// #define   MMCR1_PMC5SEL	0x003e0000UL /* PMC 5 Event */
// #define   MMCR1_PMC6SEL 0x0001f800UL /* PMC 6 Event */
pub const SPRN_MMCR2: u64 = 944;
pub const SPRN_PMC1: u64 = 953	/* Performance Counter Register 1 */;
pub const SPRN_PMC2: u64 = 954	/* Performance Counter Register 2 */;
pub const SPRN_PMC3: u64 = 957	/* Performance Counter Register 3 */;
pub const SPRN_PMC4: u64 = 958	/* Performance Counter Register 4 */;
pub const SPRN_PMC5: u64 = 945	/* Performance Counter Register 5 */;
pub const SPRN_PMC6: u64 = 946	/* Performance Counter Register 6 */;

pub const SPRN_SIAR: u64 = 955	/* Sampled Instruction Address Register */;

/* Bit definitions for MMCR0 and PMC1 / PMC2. */
pub const MMCR0_PMC1_CYCLES: u64 = (1 << 7);
pub const MMCR0_PMC1_ICACHEMISS: u64 = (5 << 7);
pub const MMCR0_PMC1_DTLB: u64 = (6 << 7);
pub const MMCR0_PMC2_DCACHEMISS: u64 = 0x6;
pub const MMCR0_PMC2_CYCLES: u64 = 0x1;
pub const MMCR0_PMC2_ITLB: u64 = 0x7;
pub const MMCR0_PMC2_LOADMISSTIME: u64 = 0x5;
// #endif

/*
 * SPRG usage:
 *
 * All 64-bit:
 *	- SPRG1 stores PACA pointer except 64-bit server in
 *        HV mode in which case it is HSPRG0
 *
 * 64-bit server:
 *	- SPRG0 scratch for TM recheckpoint/reclaim (reserved for HV on Power4)
 *	- SPRG2 scratch for exception vectors
 *	- SPRG3 CPU and NUMA node for VDSO getcpu (user visible)
 *      - HSPRG0 stores PACA in HV mode
 *      - HSPRG1 scratch for "HV" exceptions
 *
 * 64-bit embedded
 *	- SPRG0 generic exception scratch
 *	- SPRG2 TLB exception stack
 *	- SPRG3 critical exception scratch (user visible, sorry!)
 *	- SPRG4 unused (user visible)
 *	- SPRG6 TLB miss scratch (user visible, sorry !)
 *	- SPRG7 CPU and NUMA node for VDSO getcpu (user visible)
 *	- SPRG8 machine check exception scratch
 *	- SPRG9 debug exception scratch
 *
 * All 32-bit:
 *	- SPRG3 current thread_struct physical addr pointer
 *        (virtual on BookE, physical on others)
 *
 * 32-bit classic:
 *	- SPRG0 scratch for exception vectors
 *	- SPRG1 scratch for exception vectors
 *	- SPRG2 indicator that we are in RTAS
 *	- SPRG4 (603 only) pseudo TLB LRU data
 *
 * 32-bit 440 and FSL BookE:
 *	- SPRG0 scratch for exception vectors
 *	- SPRG1 scratch for exception vectors (*)
 *	- SPRG2 scratch for crit interrupts handler
 *	- SPRG4 scratch for exception vectors
 *	- SPRG5 scratch for exception vectors
 *	- SPRG6 scratch for machine check handler
 *	- SPRG7 scratch for exception vectors
 *	- SPRG9 scratch for debug vectors (e500 only)
 *
 *      Additionally, BookE separates "read" and "write"
 *      of those registers. That allows to use the userspace
 *      readable variant for reads, which can avoid a fault
 *      with KVM type virtualization.
 *
 * 32-bit 8xx:
 *	- SPRG0 scratch for exception vectors
 *	- SPRG1 scratch for exception vectors
 *	- SPRG2 scratch for exception vectors
 *
 */
// #ifdef CONFIG_PPC64
pub const SPRN_SPRG_PACA: u64 = SPRN_SPRG1;
// #else
pub const SPRN_SPRG_THREAD: u64 = SPRN_SPRG3;
// #endif

// #ifdef CONFIG_PPC_BOOK3S_64
pub const SPRN_SPRG_SCRATCH0: u64 = SPRN_SPRG2;
pub const SPRN_SPRG_HPACA: u64 = SPRN_HSPRG0;
pub const SPRN_SPRG_HSCRATCH0: u64 = SPRN_HSPRG1;
pub const SPRN_SPRG_VDSO_READ: u64 = SPRN_USPRG3;
pub const SPRN_SPRG_VDSO_WRITE: u64 = SPRN_SPRG3;

// #define GET_PACA(rX)					\
	BEGIN_FTR_SECTION_NESTED(66);			\
	mfspr	rX,SPRN_SPRG_PACA;			\
	FTR_SECTION_ELSE_NESTED(66);			\
	mfspr	rX,SPRN_SPRG_HPACA;			\
	ALT_FTR_SECTION_END_NESTED_IFCLR(CPU_FTR_HVMODE, 66)

// #define SET_PACA(rX)					\
	BEGIN_FTR_SECTION_NESTED(66);			\
	mtspr	SPRN_SPRG_PACA,rX;			\
	FTR_SECTION_ELSE_NESTED(66);			\
	mtspr	SPRN_SPRG_HPACA,rX;			\
	ALT_FTR_SECTION_END_NESTED_IFCLR(CPU_FTR_HVMODE, 66)

// #define GET_SCRATCH0(rX)				\
	BEGIN_FTR_SECTION_NESTED(66);			\
	mfspr	rX,SPRN_SPRG_SCRATCH0;			\
	FTR_SECTION_ELSE_NESTED(66);			\
	mfspr	rX,SPRN_SPRG_HSCRATCH0;			\
	ALT_FTR_SECTION_END_NESTED_IFCLR(CPU_FTR_HVMODE, 66)

// #define SET_SCRATCH0(rX)				\
	BEGIN_FTR_SECTION_NESTED(66);			\
	mtspr	SPRN_SPRG_SCRATCH0,rX;			\
	FTR_SECTION_ELSE_NESTED(66);			\
	mtspr	SPRN_SPRG_HSCRATCH0,rX;			\
	ALT_FTR_SECTION_END_NESTED_IFCLR(CPU_FTR_HVMODE, 66)

// #else /* CONFIG_PPC_BOOK3S_64 */
// #define GET_SCRATCH0(rX)	mfspr	rX,SPRN_SPRG_SCRATCH0
// #define SET_SCRATCH0(rX)	mtspr	SPRN_SPRG_SCRATCH0,rX

// #endif

// #ifdef CONFIG_PPC_BOOK3E_64
pub const SPRN_SPRG_MC_SCRATCH: u64 = SPRN_SPRG8;
pub const SPRN_SPRG_CRIT_SCRATCH: u64 = SPRN_SPRG3;
pub const SPRN_SPRG_DBG_SCRATCH: u64 = SPRN_SPRG9;
pub const SPRN_SPRG_TLB_EXFRAME: u64 = SPRN_SPRG2;
pub const SPRN_SPRG_TLB_SCRATCH: u64 = SPRN_SPRG6;
pub const SPRN_SPRG_GEN_SCRATCH: u64 = SPRN_SPRG0;
pub const SPRN_SPRG_GDBELL_SCRATCH: u64 = SPRN_SPRG_GEN_SCRATCH;
pub const SPRN_SPRG_VDSO_READ: u64 = SPRN_USPRG7;
pub const SPRN_SPRG_VDSO_WRITE: u64 = SPRN_SPRG7;

// #define SET_PACA(rX)	mtspr	SPRN_SPRG_PACA,rX
// #define GET_PACA(rX)	mfspr	rX,SPRN_SPRG_PACA

// #endif

// #ifdef CONFIG_PPC_BOOK3S_32
pub const SPRN_SPRG_SCRATCH0: u64 = SPRN_SPRG0;
pub const SPRN_SPRG_SCRATCH1: u64 = SPRN_SPRG1;
pub const SPRN_SPRG_SCRATCH2: u64 = SPRN_SPRG2;
pub const SPRN_SPRG_603_LRU: u64 = SPRN_SPRG4;
// #endif

// #ifdef CONFIG_BOOKE
pub const SPRN_SPRG_RSCRATCH0: u64 = SPRN_SPRG0;
pub const SPRN_SPRG_WSCRATCH0: u64 = SPRN_SPRG0;
pub const SPRN_SPRG_RSCRATCH1: u64 = SPRN_SPRG1;
pub const SPRN_SPRG_WSCRATCH1: u64 = SPRN_SPRG1;
pub const SPRN_SPRG_RSCRATCH_CRIT: u64 = SPRN_SPRG2;
pub const SPRN_SPRG_WSCRATCH_CRIT: u64 = SPRN_SPRG2;
pub const SPRN_SPRG_RSCRATCH2: u64 = SPRN_SPRG4R;
pub const SPRN_SPRG_WSCRATCH2: u64 = SPRN_SPRG4W;
pub const SPRN_SPRG_RSCRATCH3: u64 = SPRN_SPRG5R;
pub const SPRN_SPRG_WSCRATCH3: u64 = SPRN_SPRG5W;
pub const SPRN_SPRG_RSCRATCH_MC: u64 = SPRN_SPRG1;
pub const SPRN_SPRG_WSCRATCH_MC: u64 = SPRN_SPRG1;
pub const SPRN_SPRG_RSCRATCH4: u64 = SPRN_SPRG7R;
pub const SPRN_SPRG_WSCRATCH4: u64 = SPRN_SPRG7W;
pub const SPRN_SPRG_RSCRATCH_DBG: u64 = SPRN_SPRG9;
pub const SPRN_SPRG_WSCRATCH_DBG: u64 = SPRN_SPRG9;
// #endif

// #ifdef CONFIG_PPC_8xx
pub const SPRN_SPRG_SCRATCH0: u64 = SPRN_SPRG0;
pub const SPRN_SPRG_SCRATCH1: u64 = SPRN_SPRG1;
pub const SPRN_SPRG_SCRATCH2: u64 = SPRN_SPRG2;
// #endif



/*
 * An mtfsf instruction with the L bit set. On CPUs that support this a
 * full 64bits of FPSCR is restored and on other CPUs the L bit is ignored.
 *
 * Until binutils gets the new form of mtfsf, hardwire the instruction.
 */
// #ifdef CONFIG_PPC64
// #define MTFSF_L(REG) \
	.long (0xfc00058e | ((0xff) << 17) | ((REG) << 11) | (1 << 25))
// #else
// #define MTFSF_L(REG)	mtfsf	0xff, (REG)
// #endif

/* Processor Version Register (PVR) field extraction */

// #define PVR_VER(pvr)	(((pvr) >>  16) & 0xFFFF)	/* Version field */
// #define PVR_REV(pvr)	(((pvr) >>   0) & 0xFFFF)	/* Revison field */

// #define pvr_version_is(pvr)	(PVR_VER(mfspr(SPRN_PVR)) == (pvr))

/*
 * IBM has further subdivided the standard PowerPC 16-bit version and
 * revision subfields of the PVR for the PowerPC 403s into the following:
 */

// #define PVR_FAM(pvr)	(((pvr) >> 20) & 0xFFF)	/* Family field */
// #define PVR_MEM(pvr)	(((pvr) >> 16) & 0xF)	/* Member field */
// #define PVR_CORE(pvr)	(((pvr) >> 12) & 0xF)	/* Core field */
// #define PVR_CFG(pvr)	(((pvr) >>  8) & 0xF)	/* Configuration field */
// #define PVR_MAJ(pvr)	(((pvr) >>  4) & 0xF)	/* Major revision field */
// #define PVR_MIN(pvr)	(((pvr) >>  0) & 0xF)	/* Minor revision field */

/* Processor Version Numbers */

pub const PVR_403GA: u64 = 0x00200000;
pub const PVR_403GB: u64 = 0x00200100;
pub const PVR_403GC: u64 = 0x00200200;
pub const PVR_403GCX: u64 = 0x00201400;
pub const PVR_405GP: u64 = 0x40110000;
pub const PVR_476: u64 = 0x11a52000;
pub const PVR_476FPE: u64 = 0x7ff50000;
pub const PVR_STB03XXX: u64 = 0x40310000;
pub const PVR_NP405H: u64 = 0x41410000;
pub const PVR_NP405L: u64 = 0x41610000;
pub const PVR_601: u64 = 0x00010000;
pub const PVR_602: u64 = 0x00050000;
pub const PVR_603: u64 = 0x00030000;
pub const PVR_603e: u64 = 0x00060000;
pub const PVR_603ev: u64 = 0x00070000;
pub const PVR_603r: u64 = 0x00071000;
pub const PVR_604: u64 = 0x00040000;
pub const PVR_604e: u64 = 0x00090000;
pub const PVR_604r: u64 = 0x000A0000;
pub const PVR_620: u64 = 0x00140000;
pub const PVR_740: u64 = 0x00080000;
pub const PVR_750: u64 = PVR_740;
pub const PVR_740P: u64 = 0x10080000;
pub const PVR_750P: u64 = PVR_740P;
pub const PVR_7400: u64 = 0x000C0000;
pub const PVR_7410: u64 = 0x800C0000;
pub const PVR_7450: u64 = 0x80000000;
pub const PVR_8540: u64 = 0x80200000;
pub const PVR_8560: u64 = 0x80200000;
pub const PVR_VER_E500V1: u64 = 0x8020;
pub const PVR_VER_E500V2: u64 = 0x8021;
pub const PVR_VER_E500MC: u64 = 0x8023;
pub const PVR_VER_E5500: u64 = 0x8024;
pub const PVR_VER_E6500: u64 = 0x8040;
pub const PVR_VER_7450: u64 = 0x8000;
pub const PVR_VER_7455: u64 = 0x8001;
pub const PVR_VER_7447: u64 = 0x8002;
pub const PVR_VER_7447A: u64 = 0x8003;
pub const PVR_VER_7448: u64 = 0x8004;

/*
 * For the 8xx processors, all of them report the same PVR family for
 * the PowerPC core. The various versions of these processors must be
 * differentiated by the version number in the Communication Processor
 * Module (CPM).
 */
pub const PVR_8xx: u64 = 0x00500000;

pub const PVR_8240: u64 = 0x00810100;
pub const PVR_8245: u64 = 0x80811014;
pub const PVR_8260: u64 = PVR_8240;

/* 476 Simulator seems to currently have the PVR of the 602... */
pub const PVR_476_ISS: u64 = 0x00052000;

/* 64-bit processors */
pub const PVR_NORTHSTAR: u64 = 0x0033;
pub const PVR_PULSAR: u64 = 0x0034;
pub const PVR_POWER4: u64 = 0x0035;
pub const PVR_ICESTAR: u64 = 0x0036;
pub const PVR_SSTAR: u64 = 0x0037;
pub const PVR_POWER4p: u64 = 0x0038;
pub const PVR_970: u64 = 0x0039;
pub const PVR_POWER5: u64 = 0x003A;
pub const PVR_POWER5p: u64 = 0x003B;
pub const PVR_970FX: u64 = 0x003C;
pub const PVR_POWER6: u64 = 0x003E;
pub const PVR_POWER7: u64 = 0x003F;
pub const PVR_630: u64 = 0x0040;
pub const PVR_630p: u64 = 0x0041;
pub const PVR_970MP: u64 = 0x0044;
pub const PVR_970GX: u64 = 0x0045;
pub const PVR_POWER7p: u64 = 0x004A;
pub const PVR_POWER8E: u64 = 0x004B;
pub const PVR_POWER8NVL: u64 = 0x004C;
pub const PVR_POWER8: u64 = 0x004D;
pub const PVR_HX_C2000: u64 = 0x0066;
pub const PVR_POWER9: u64 = 0x004E;
pub const PVR_POWER10: u64 = 0x0080;
pub const PVR_POWER11: u64 = 0x0082;
pub const PVR_POWER12: u64 = 0x0083;
pub const PVR_BE: u64 = 0x0070;
pub const PVR_PA6T: u64 = 0x0090;

/* "Logical" PVR values defined in PAPR, representing architecture levels */
pub const PVR_ARCH_204: u64 = 0x0f000001;
pub const PVR_ARCH_205: u64 = 0x0f000002;
pub const PVR_ARCH_206: u64 = 0x0f000003;
pub const PVR_ARCH_206p: u64 = 0x0f100003;
pub const PVR_ARCH_207: u64 = 0x0f000004;
pub const PVR_ARCH_300: u64 = 0x0f000005;
pub const PVR_ARCH_31: u64 = 0x0f000006;
pub const PVR_ARCH_31_P11: u64 = 0x0f000007;
pub const PVR_ARCH_32: u64 = 0x0f000008;

/*
 * Kernel-internal sentinel for invalid processor compatibility modes.
 * PAPR specifies that the first byte of a valid logical PVR value is
 * 0x0f. So 0xffffffff lies permanently outside the PAPR-defined range
 * and is safe to repurpose. KVM stores it in vcpu->arch.arch_compat
 * when userspace requests an unsupported compatibility mode (e.g.,
 * Power11 PVR on a Power11 host booted in Power10 compat).
 * kvmppc_sanity_check() detects this and prevents the vCPU from
 * running with an unsupported arch_compat.
 */
pub const PVR_ARCH_INVALID: u64 = 0xffffffff;

/* Macros for setting and retrieving special purpose registers */
// #ifndef __ASSEMBLER__

// #if defined(CONFIG_PPC64) || defined(__CHECKER__)
// typedef struct {
	u32 val;
// #ifdef CONFIG_PPC64
	u32 suffix;
// #endif
} __packed ppc_inst_t;
// #else
// typedef u32 ppc_inst_t;
// #endif

// #define mfmsr()		({unsigned long rval; \
			asm volatile("mfmsr %0" : "=r" (rval) : \
						: "memory"); rval;})
// #ifdef CONFIG_PPC_BOOK3S_64
// #define __mtmsrd(v, l)	asm volatile("mtmsrd %0," __stringify(l) \
				     : : "r" (v) : "memory")
// #define mtmsr(v)	__mtmsrd((v), 0)
pub const __MTMSR: u64 = "mtmsrd";
// #else
// #define mtmsr(v)	asm volatile("mtmsr %0" : \
				     : "r" ((unsigned long)(v)) \
				     : "memory")
// #define __mtmsrd(v, l)	BUILD_BUG()
pub const __MTMSR: u64 = "mtmsr";
// #endif

// static inline void mtmsr_isync(unsigned long val)
{
	asm volatile(__MTMSR " %0; " ASM_FTR_IFCLR("isync", "nop", %1) : :
			"r" (val), "i" (CPU_FTR_ARCH_206) : "memory");
}

// #define mfspr(rn)	({unsigned long rval; \
			asm volatile("mfspr %0," __stringify(rn) \
				: "=r" (rval)); rval;})
// #define mtspr(rn, v)	asm volatile("mtspr " __stringify(rn) ",%0" : \
				     : "r" ((unsigned long)(v)) \
				     : "memory")
// #define wrtspr(rn)	asm volatile("mtspr " __stringify(rn) ",2" : : : "memory")
// #define wrtspr_sync(rn)	asm volatile("mtspr " __stringify(rn) ",2; sync" : : : "memory")

// static inline void wrtee(unsigned long val)
{
	if (__builtin_constant_p(val))
		asm volatile("wrteei %0" : : "i" ((val & MSR_EE) ? 1 : 0) : "memory");
	else
		asm volatile("wrtee %0" : : "r" (val) : "memory");
}

// extern unsigned long msr_check_and_set(unsigned long bits);
// extern bool strict_msr_control;
// extern void __msr_check_and_clear(unsigned long bits);
// static inline void msr_check_and_clear(unsigned long bits)
{
	if (strict_msr_control)
		__msr_check_and_clear(bits);
}

// #ifdef CONFIG_PPC32
// static inline u32 mfsr(u32 idx)
{
	u32 val;

	if (__builtin_constant_p(idx))
		asm volatile("mfsr %0, %1" : "=r" (val): "i" (idx >> 28));
	else
		asm volatile("mfsrin %0, %1" : "=r" (val): "r" (idx));

	return val;
}

// static inline void mtsr(u32 val, u32 idx)
{
	if (__builtin_constant_p(idx))
		asm volatile("mtsr %1, %0" : : "r" (val), "i" (idx >> 28));
	else
		asm volatile("mtsrin %0, %1" : : "r" (val), "r" (idx));
}
// #endif

// extern unsigned long current_stack_frame(void);

register unsigned long current_stack_pointer asm("r1");

// extern unsigned long scom970_read(unsigned int address);
// extern void scom970_write(unsigned int address, unsigned long value);

// struct pt_regs;

// extern void ppc_save_regs(struct pt_regs *regs);
// #endif /* __ASSEMBLER__ */
// #endif /* __KERNEL__ */
// #endif /* _ASM_POWERPC_REG_H */

/* Untranslated source-level forms retained verbatim for external integration:

 * #define MSR_TS_LG	33		/* Trans Mem state (2 bits) */
 * #define __MASK(X)	(1<<(X))
 * #define __MASK(X)	(1UL<<(X))
 * #define MSR_SF		__MASK(MSR_SF_LG)	/* Enable 64 bit mode */
 * #define MSR_HV 		__MASK(MSR_HV_LG)	/* Hypervisor state */
 * #define MSR_S		__MASK(MSR_S_LG)	/* Secure state */
 * #define MSR_VEC		__MASK(MSR_VEC_LG)	/* Enable AltiVec */
 * #define MSR_VSX		__MASK(MSR_VSX_LG)	/* Enable VSX */
 * #define MSR_POW		__MASK(MSR_POW_LG)	/* Enable Power Management */
 * #define MSR_WE		__MASK(MSR_WE_LG)	/* Wait State Enable */
 * #define MSR_TGPR	__MASK(MSR_TGPR_LG)	/* TLB Update registers in use */
 * #define MSR_CE		__MASK(MSR_CE_LG)	/* Critical Interrupt Enable */
 * #define MSR_ILE		__MASK(MSR_ILE_LG)	/* Interrupt Little Endian */
 * #define MSR_EE		__MASK(MSR_EE_LG)	/* External Interrupt Enable */
 * #define MSR_PR		__MASK(MSR_PR_LG)	/* Problem State / Privilege Level */
 * #define MSR_FP		__MASK(MSR_FP_LG)	/* Floating Point enable */
 * #define MSR_ME		__MASK(MSR_ME_LG)	/* Machine Check Enable */
 * #define MSR_FE0		__MASK(MSR_FE0_LG)	/* Floating Exception mode 0 */
 * #define MSR_SE		__MASK(MSR_SE_LG)	/* Single Step */
 * #define MSR_BE		__MASK(MSR_BE_LG)	/* Branch Trace */
 * #define MSR_DE		__MASK(MSR_DE_LG)	/* Debug Exception Enable */
 * #define MSR_FE1		__MASK(MSR_FE1_LG)	/* Floating Exception mode 1 */
 * #define MSR_IP		__MASK(MSR_IP_LG)	/* Exception prefix 0x000/0xFFF */
 * #define MSR_IR		__MASK(MSR_IR_LG)	/* Instruction Relocate */
 * #define MSR_DR		__MASK(MSR_DR_LG)	/* Data Relocate */
 * #define MSR_PE		__MASK(MSR_PE_LG)	/* Protection Enable */
 * #define MSR_PX		__MASK(MSR_PX_LG)	/* Protection Exclusive Mode */
 * #define MSR_PMM		__MASK(MSR_PMM_LG)	/* Performance monitor */
 * #define MSR_RI		__MASK(MSR_RI_LG)	/* Recoverable Exception */
 * #define MSR_LE		__MASK(MSR_LE_LG)	/* Little Endian */
 * #define MSR_TM		__MASK(MSR_TM_LG)	/* Transactional Mem Available */
 * #define MSR_TS_S	__MASK(MSR_TS_S_LG)	/*  Transaction Suspended */
 * #define MSR_TS_T	__MASK(MSR_TS_T_LG)	/*  Transaction Transactional */
 * #define MSR_TS_MASK	(MSR_TS_T | MSR_TS_S)   /* Transaction State bits */
 * #define MSR_TM_RESV(x) (((x) & MSR_TS_MASK) == MSR_TS_MASK) /* Reserved */
 * #define MSR_TM_TRANSACTIONAL(x)	(((x) & MSR_TS_MASK) == MSR_TS_T)
 * #define MSR_TM_SUSPENDED(x)	(((x) & MSR_TS_MASK) == MSR_TS_S)
 * #define MSR_TM_ACTIVE(x) (((x) & MSR_TS_MASK) != 0) /* Transaction active? */
 * #define MSR_TM_ACTIVE(x) ((void)(x), 0)
 * #define __MSR		(MSR_ME | MSR_RI | MSR_IR | MSR_DR | MSR_HV)
 * #define MSR_IDLE	(MSR_ME | MSR_SF | MSR_HV)
 * #define MSR_		(__MSR | MSR_LE)
 * #define MSR_IDLE	(MSR_ME | MSR_SF | MSR_HV | MSR_LE)
 * #define MSR_KERNEL	(MSR_ | MSR_64BIT)
 * #define MSR_USER32	(MSR_ | MSR_PR | MSR_EE)
 * #define MSR_USER64	(MSR_USER32 | MSR_64BIT)
 * #define MSR_KERNEL	(MSR_ME|MSR_RI|MSR_IR|MSR_DR)
 * #define MSR_USER	(MSR_KERNEL|MSR_PR|MSR_EE)
 * #define CR0_TBEGIN_FAILURE	(0x2 << 28) /* 0b0010 */
 * #define PSSCR_FAKE_SUSPEND	0x00000400 /* Fake-suspend bit (P9 DD2.2) */
 * #define TEXASR_FC_LG	(63 - 7)	/* Failure Code */
 * #define TEXASR_AB_LG	(63 - 31)	/* Abort */
 * #define TEXASR_SU_LG	(63 - 32)	/* Suspend */
 * #define TEXASR_HV_LG	(63 - 34)	/* Hypervisor state*/
 * #define TEXASR_PR_LG	(63 - 35)	/* Privilege level */
 * #define TEXASR_FS_LG	(63 - 36)	/* failure summary */
 * #define TEXASR_EX_LG	(63 - 37)	/* TFIAR exact bit */
 * #define TEXASR_ROT_LG	(63 - 38)	/* ROT bit */
 * #define   TEXASR_ABORT	__MASK(TEXASR_AB_LG) /* terminated by tabort or treclaim */
 * #define   TEXASR_SUSP	__MASK(TEXASR_SU_LG) /* tx failed in suspended state */
 * #define   TEXASR_HV	__MASK(TEXASR_HV_LG) /* MSR[HV] when failure occurred */
 * #define   TEXASR_PR	__MASK(TEXASR_PR_LG) /* MSR[PR] when failure occurred */
 * #define   TEXASR_FS	__MASK(TEXASR_FS_LG) /* TEXASR Failure Summary */
 * #define   TEXASR_EXACT	__MASK(TEXASR_EX_LG) /* TFIAR value is exact */
 * #define   TEXASR_ROT	__MASK(TEXASR_ROT_LG)
 * #define   TEXASR_FC	(ASM_CONST(0xFF) << TEXASR_FC_LG)
 * #define   DAWRX_USER	__MASK(0)
 * #define   DAWRX_KERNEL	__MASK(1)
 * #define   DAWRX_HYP	__MASK(2)
 * #define   DAWRX_WTI	__MASK(3)
 * #define   DAWRX_WT	__MASK(4)
 * #define   DAWRX_DR	__MASK(5)
 * #define   DAWRX_DW	__MASK(6)
 * #define   DABRX_USER	__MASK(0)
 * #define   DABRX_KERNEL	__MASK(1)
 * #define   DABRX_HYP	__MASK(2)
 * #define   DABRX_BTI	__MASK(3)
 * #define   DABRX_ALL     (DABRX_BTI | DABRX_HYP | DABRX_KERNEL | DABRX_USER)
 * #define   DSISR_NOSEGMENT	0x00200000 /* STAB miss (unsupported) */
 * #define   DSISR_BAD_FAULT_32S	(DSISR_BAD_DIRECT_ST	| \
 * #define	  DSISR_BAD_FAULT_64S	(DSISR_BAD_FAULT_32S	| \
 * #define   DSISR_SRR1_MATCH_32S	(DSISR_NOHPTE		| \
 * #define   DSISR_SRR1_MATCH_64S	(DSISR_SRR1_MATCH_32S	| \
 * #define SPRN_TBRL	0x10C	/* Time Base Read Lower Register (user, R/O) */
 * #define SPRN_TBRU	0x10D	/* Time Base Read Upper Register (user, R/O) */
 * #define SPRN_CIR	0x11B	/* Chip Information Register (hyper, R/0) */
 * #define SPRN_TBWL	0x11C	/* Time Base Lower Register (super, R/W) */
 * #define SPRN_TBWU	0x11D	/* Time Base Upper Register (super, R/W) */
 * #define SPRN_TBU40	0x11E	/* Timebase upper 40 bits (hyper, R/W) */
 * #define SPRN_HDEXCR_RO	0x1C7	/* Hypervisor DEXCR (non-privileged, readonly) */
 * #define SPRN_DEXCR_RO	0x32C	/* DEXCR (non-privileged, readonly) */
 * #define SPRN_PSSCR	0x357	/* Processor Stop Status and Control Register (ISA 3.0) */
 * #define   FSCR_PREFIX	__MASK(FSCR_PREFIX_LG)
 * #define   FSCR_SCV	__MASK(FSCR_SCV_LG)
 * #define   FSCR_TAR	__MASK(FSCR_TAR_LG)
 * #define   FSCR_EBB	__MASK(FSCR_EBB_LG)
 * #define   FSCR_DSCR	__MASK(FSCR_DSCR_LG)
 * #define   FSCR_INTR_CAUSE (ASM_CONST(0xFF) << 56)	/* interrupt cause */
 * #define   HFSCR_PREFIX	__MASK(FSCR_PREFIX_LG)
 * #define   HFSCR_MSGP	__MASK(FSCR_MSGP_LG)
 * #define   HFSCR_TAR	__MASK(FSCR_TAR_LG)
 * #define   HFSCR_EBB	__MASK(FSCR_EBB_LG)
 * #define   HFSCR_TM	__MASK(FSCR_TM_LG)
 * #define   HFSCR_PM	__MASK(FSCR_PM_LG)
 * #define   HFSCR_BHRB	__MASK(FSCR_BHRB_LG)
 * #define   HFSCR_DSCR	__MASK(FSCR_DSCR_LG)
 * #define   HFSCR_VECVSX	__MASK(FSCR_VECVSX_LG)
 * #define   HFSCR_FP	__MASK(FSCR_FP_LG)
 * #define   LPCR_VPM0		ASM_CONST(0x8000000000000000)
 * #define   LPCR_VPM1		ASM_CONST(0x4000000000000000)
 * #define   LPCR_ISL		ASM_CONST(0x2000000000000000)
 * #define   LPCR_DPFD		(ASM_CONST(7) << LPCR_DPFD_SH)
 * #define   LPCR_VRMASD		(ASM_CONST(0x1f) << LPCR_VRMASD_SH)
 * #define   LPCR_VRMA_L		ASM_CONST(0x0008000000000000)
 * #define   LPCR_VRMA_LP0		ASM_CONST(0x0001000000000000)
 * #define   LPCR_VRMA_LP1		ASM_CONST(0x0000800000000000)
 * #define   LPCR_HAIL		ASM_CONST(0x0000000004000000)   /* HV AIL (ISAv3.1) */
 * #define   LPCR_ILE		ASM_CONST(0x0000000002000000)   /* !HV irqs set MSR:LE */
 * #define   LPCR_AIL		ASM_CONST(0x0000000001800000)	/* Alternate interrupt location */
 * #define   LPCR_AIL_0		ASM_CONST(0x0000000000000000)	/* MMU off exception offset 0x0 */
 * #define   LPCR_AIL_3		ASM_CONST(0x0000000001800000)   /* MMU on exception offset 0xc00...4xxx */
 * #define   LPCR_ONL		ASM_CONST(0x0000000000040000)	/* online - PURR/SPURR count */
 * #define   LPCR_LD		ASM_CONST(0x0000000000020000)	/* large decremeter */
 * #define   LPCR_PECE		ASM_CONST(0x000000000001f000)	/* powersave exit cause enable */
 * #define     LPCR_PECEDP	ASM_CONST(0x0000000000010000)	/* directed priv dbells cause exit */
 * #define     LPCR_PECEDH	ASM_CONST(0x0000000000008000)	/* directed hyp dbells cause exit */
 * #define     LPCR_PECE0		ASM_CONST(0x0000000000004000)	/* ext. exceptions can cause exit */
 * #define     LPCR_PECE1		ASM_CONST(0x0000000000002000)	/* decrementer can cause exit */
 * #define     LPCR_PECE2		ASM_CONST(0x0000000000001000)	/* machine check etc can cause exit */
 * #define     LPCR_PECE_HVEE	ASM_CONST(0x0000400000000000)	/* P9 Wakeup on HV interrupts */
 * #define   LPCR_MER		ASM_CONST(0x0000000000000800)	/* Mediated External Exception */
 * #define	  LPCR_GTSE		ASM_CONST(0x0000000000000400)  	/* Guest Translation Shootdown Enable */
 * #define   LPCR_TC		ASM_CONST(0x0000000000000200)	/* Translation control */
 * #define   LPCR_HEIC		ASM_CONST(0x0000000000000010)   /* Hypervisor External Interrupt Control */
 * #define   LPCR_LPES0		ASM_CONST(0x0000000000000008)      /* LPAR Env selector 0 */
 * #define   LPCR_LPES1		ASM_CONST(0x0000000000000004)      /* LPAR Env selector 1 */
 * #define   LPCR_RMI		ASM_CONST(0x0000000000000002)      /* real mode is cache inhibit */
 * #define   LPCR_HVICE		ASM_CONST(0x0000000000000002)      /* P9: HV interrupt enable */
 * #define   LPCR_HDICE		ASM_CONST(0x0000000000000001)      /* Hyp Decr enable (HV,PR,EE) */
 * #define   LPCR_UPRT		ASM_CONST(0x0000000000400000)      /* Use Process Table (ISA 3) */
 * #define   LPCR_HR		ASM_CONST(0x0000000000100000)
 * #define   HMER_DEBUG_TRIG	(1ul << (63 - 17)) /* Debug trigger */
 * #define   PCR_VEC_DIS	(__MASK(63-0))	/* Vec. disable (bit NA since POWER8) */
 * #define   PCR_VSX_DIS	(__MASK(63-1))	/* VSX disable (bit NA since POWER8) */
 * #define   PCR_TM_DIS	(__MASK(63-2))	/* Trans. memory disable (POWER8) */
 * #define   PCR_MMA_DIS	(__MASK(63-3)) /* Matrix-Multiply Accelerator */
 * #define   PCR_HIGH_BITS	(PCR_MMA_DIS | PCR_VEC_DIS | PCR_VSX_DIS | PCR_TM_DIS)
 * #define   PCR_LOW_BITS	(PCR_ARCH_207 | PCR_ARCH_206 | PCR_ARCH_205 | PCR_ARCH_300 | PCR_ARCH_31)
 * #define   PCR_MASK	~(PCR_HIGH_BITS | PCR_LOW_BITS)	/* PCR Reserved Bits */
 * #define SPRN_PIT	0x3DB		/* Programmable Interval Timer (BOOKE) */
 * #define HID0_HDICE_SH	(63 - 23)	/* 970 HDEC interrupt enable */
 * #define HID0_EMCP	(1<<31)		/* Enable Machine Check pin */
 * #define HID0_EBA	(1<<29)		/* Enable Bus Address Parity */
 * #define HID0_EBD	(1<<28)		/* Enable Bus Data Parity */
 * #define HID0_SBCLK	(1<<27)
 * #define HID0_EICE	(1<<26)
 * #define HID0_TBEN	(1<<26)		/* Timebase enable - 745x */
 * #define HID0_ECLK	(1<<25)
 * #define HID0_PAR	(1<<24)
 * #define HID0_STEN	(1<<24)		/* Software table search enable - 745x */
 * #define HID0_HIGH_BAT	(1<<23)		/* Enable high BATs - 7455 */
 * #define HID0_DOZE	(1<<23)
 * #define HID0_NAP	(1<<22)
 * #define HID0_SLEEP	(1<<21)
 * #define HID0_DPM	(1<<20)
 * #define HID0_BHTCLR	(1<<18)		/* Clear branch history table - 7450 */
 * #define HID0_XAEN	(1<<17)		/* Extended addressing enable - 7450 */
 * #define HID0_NHR	(1<<16)		/* Not hard reset (software bit-7450)*/
 * #define HID0_ICE	(1<<15)		/* Instruction Cache Enable */
 * #define HID0_DCE	(1<<14)		/* Data Cache Enable */
 * #define HID0_ILOCK	(1<<13)		/* Instruction Cache Lock */
 * #define HID0_DLOCK	(1<<12)		/* Data Cache Lock */
 * #define HID0_ICFI	(1<<11)		/* Instr. Cache Flash Invalidate */
 * #define HID0_DCI	(1<<10)		/* Data Cache Invalidate */
 * #define HID0_SPD	(1<<9)		/* Speculative disable */
 * #define HID0_DAPUEN	(1<<8)		/* Debug APU enable */
 * #define HID0_SGE	(1<<7)		/* Store Gathering Enable */
 * #define HID0_SIED	(1<<7)		/* Serial Instr. Execution [Disable] */
 * #define HID0_DCFA	(1<<6)		/* Data Cache Flush Assist */
 * #define HID0_LRSTK	(1<<4)		/* Link register stack - 745x */
 * #define HID0_BTIC	(1<<5)		/* Branch Target Instr Cache Enable */
 * #define HID0_ABE	(1<<3)		/* Address Broadcast Enable */
 * #define HID0_FOLD	(1<<3)		/* Branch Folding enable - 745x */
 * #define HID0_BHTE	(1<<2)		/* Branch History Table Enable */
 * #define HID0_BTCD	(1<<1)		/* Branch target cache disable */
 * #define HID0_NOPDST	(1<<1)		/* No-op dst, dstt, etc. instr. */
 * #define HID0_NOPTI	(1<<0)		/* No-op dcbt and dcbst instr. */
 * #define HID0_POWER8_4LPARMODE	__MASK(61)
 * #define HID0_POWER8_2LPARMODE	__MASK(57)
 * #define HID0_POWER8_1TO2LPAR	__MASK(52)
 * #define HID0_POWER8_1TO4LPAR	__MASK(51)
 * #define HID0_POWER8_DYNLPARDIS	__MASK(48)
 * #define HID0_POWER9_RADIX	__MASK(63 - 8)
 * #define HID1_EMCP	(1<<31)		/* 7450 Machine Check Pin Enable */
 * #define HID1_DFS	(1<<22)		/* 7447A Dynamic Frequency Scaling */
 * #define HID1_PC0	(1<<16)		/* 7450 PLL_CFG[0] */
 * #define HID1_PC1	(1<<15)		/* 7450 PLL_CFG[1] */
 * #define HID1_PC2	(1<<14)		/* 7450 PLL_CFG[2] */
 * #define HID1_PC3	(1<<13)		/* 7450 PLL_CFG[3] */
 * #define HID1_SYNCBE	(1<<11)		/* 7450 ABE for sync, eieio */
 * #define HID1_ABE	(1<<10)		/* 7450 Address Broadcast Enable */
 * #define HID1_PS		(1<<16)		/* 750FX PLL selection */
 * #define  HID2_G2_LE_HBE	(1<<18)		/* High BAT Enable (G2_LE) */
 * #define  HID4_LPES0	 (1ul << (63-0)) /* LPAR env. sel. bit 0 */
 * #define	 HID4_RMLS2_SH	 (63 - 2)	/* Real mode limit bottom 2 bits */
 * #define	 HID4_LPID5_SH	 (63 - 6)	/* partition ID bottom 4 bits */
 * #define	 HID4_RMOR_SH	 (63 - 22)	/* real mode offset (16 bits) */
 * #define  HID4_RMOR	 (0xFFFFul << HID4_RMOR_SH)
 * #define  HID4_LPES1	 (1 << (63-57))	/* LPAR env. sel. bit 1 */
 * #define  HID4_RMLS0_SH	 (63 - 58)	/* Real mode limit top bit */
 * #define   HID6_LB	(0x0F<<12) /* Concurrent Large Page Modes */
 * #define   HID6_DLP	(1<<20)	/* Disable all large page modes (4K only) */
 * #define L2CR_L2IO_745x		0x00100000	/* L2 instr. only (745x) */
 * #define L2CR_L2DO_745x		0x00010000	/* L2 data only (745x) */
 * #define L2CR_L2REP_745x		0x00001000	/* L2 repl. algorithm (745x) */
 * #define L2CR_L2HWF_745x		0x00000800	/* L2 hardware flush (745x) */
 * #define SPRN_PTEHI	0x3D5	/* 981 7450 PTE HI word (S/W TLB load) */
 * #define SPRN_PTELO	0x3D6	/* 982 7450 PTE LO word (S/W TLB load) */
 * #define   SRR1_MSR_BITS		(~0x783f0000UL)
 * #define   SRR1_WAKEHVI		0x00240000 /* Hypervisor Virtualization Interrupt (P9) */
 * #define THRM1_TIN	(1 << 31)
 * #define THRM1_TIV	(1 << 30)
 * #define THRM1_THRES(x)	((x&0x7f)<<23)
 * #define THRM3_SITV(x)	((x & 0x1fff) << 1)
 * #define THRM1_TID	(1<<2)
 * #define THRM1_TIE	(1<<1)
 * #define THRM1_V		(1<<0)
 * #define THRM3_E		(1<<0)
 * #define   MMCR0_PMXE	ASM_CONST(0x04000000) /* perf mon exception enable */
 * #define   MMCR0_FCECE	ASM_CONST(0x02000000) /* freeze ctrs on enabled cond or event */
 * #define   MMCR0_PMCCEXT	ASM_CONST(0x00000200) /* PMCCEXT control */
 * #define   MMCR0_PMCC_U6	0x00080000UL /* PMC1-6 are R/W by user (PR) */
 * #define   MMCR0_PMCjCE	ASM_CONST(0x00004000) /* PMCj count enable*/
 * #define   MMCR0_PMAO_SYNC ASM_CONST(0x00000800) /* PMU intr is synchronous */
 * #define   MMCR0_C56RUN	ASM_CONST(0x00000100) /* PMC5/6 count when RUN=0 */
 * #define   MMCR0_PMAO	ASM_CONST(0x00000080)
 * #define   MMCRA_SLOT	0x07000000UL /* SLOT bits (37-39) */
 * #define   MMCRA_BHRB_DISABLE  _UL(0x2000000000) // BHRB disable bit for ISA v3.1
 * #define MMCR0_USER_MASK	(MMCR0_FC | MMCR0_PMXE | MMCR0_PMAO)
 * #define MMCR2_USER_MASK	0x4020100804020000UL /* (FC1P|FC2P|FC3P|FC4P|FC5P|FC6P) */
 * #define MMCR0_PMC1_CYCLES	(1 << 7)
 * #define MMCR0_PMC1_ICACHEMISS	(5 << 7)
 * #define MMCR0_PMC1_DTLB		(6 << 7)
 * #define GET_PACA(rX)					\
 * #define SET_PACA(rX)					\
 * #define GET_SCRATCH0(rX)				\
 * #define SET_SCRATCH0(rX)				\
 * #define GET_SCRATCH0(rX)	mfspr	rX,SPRN_SPRG_SCRATCH0
 * #define SET_SCRATCH0(rX)	mtspr	SPRN_SPRG_SCRATCH0,rX
 * #define SET_PACA(rX)	mtspr	SPRN_SPRG_PACA,rX
 * #define GET_PACA(rX)	mfspr	rX,SPRN_SPRG_PACA
 * #define MTFSF_L(REG) \
 * #define MTFSF_L(REG)	mtfsf	0xff, (REG)
 * #define PVR_VER(pvr)	(((pvr) >>  16) & 0xFFFF)	/* Version field */
 * #define PVR_REV(pvr)	(((pvr) >>   0) & 0xFFFF)	/* Revison field */
 * #define pvr_version_is(pvr)	(PVR_VER(mfspr(SPRN_PVR)) == (pvr))
 * #define PVR_FAM(pvr)	(((pvr) >> 20) & 0xFFF)	/* Family field */
 * #define PVR_MEM(pvr)	(((pvr) >> 16) & 0xF)	/* Member field */
 * #define PVR_CORE(pvr)	(((pvr) >> 12) & 0xF)	/* Core field */
 * #define PVR_CFG(pvr)	(((pvr) >>  8) & 0xF)	/* Configuration field */
 * #define PVR_MAJ(pvr)	(((pvr) >>  4) & 0xF)	/* Major revision field */
 * #define PVR_MIN(pvr)	(((pvr) >>  0) & 0xF)	/* Minor revision field */
 * #define mfmsr()		({unsigned long rval; \
 * #define __mtmsrd(v, l)	asm volatile("mtmsrd %0," __stringify(l) \
 * #define mtmsr(v)	__mtmsrd((v), 0)
 * #define mtmsr(v)	asm volatile("mtmsr %0" : \
 * #define __mtmsrd(v, l)	BUILD_BUG()
 * #define mfspr(rn)	({unsigned long rval; \
 * #define mtspr(rn, v)	asm volatile("mtspr " __stringify(rn) ",%0" : \
 * #define wrtspr(rn)	asm volatile("mtspr " __stringify(rn) ",2" : : : "memory")
 * #define wrtspr_sync(rn)	asm volatile("mtspr " __stringify(rn) ",2; sync" : : : "memory")
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
