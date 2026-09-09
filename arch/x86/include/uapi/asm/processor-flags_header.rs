/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Various flags defined: can be included from assembler. */

/* EFLAGS bits */
pub const X86_EFLAGS_CF_BIT: u32 = 0; /* Carry Flag */
pub const X86_EFLAGS_CF: u64 = 1u64 << X86_EFLAGS_CF_BIT; 
pub const X86_EFLAGS_FIXED_BIT: u32 = 1; /* Bit 1 - always on */
pub const X86_EFLAGS_FIXED: u64 = 1u64 << X86_EFLAGS_FIXED_BIT;
pub const X86_EFLAGS_PF_BIT: u32 = 2; /* Parity Flag */
pub const X86_EFLAGS_PF: u64 = 1u64 << X86_EFLAGS_PF_BIT;
pub const X86_EFLAGS_AF_BIT: u32 = 4; /* Auxiliary carry Flag */
pub const X86_EFLAGS_AF: u64 = 1u64 << X86_EFLAGS_AF_BIT;
pub const X86_EFLAGS_ZF_BIT: u32 = 6; /* Zero Flag */
pub const X86_EFLAGS_ZF: u64 = 1u64 << X86_EFLAGS_ZF_BIT;
pub const X86_EFLAGS_SF_BIT: u32 = 7; /* Sign Flag */
pub const X86_EFLAGS_SF: u64 = 1u64 << X86_EFLAGS_SF_BIT;
pub const X86_EFLAGS_TF_BIT: u32 = 8; /* Trap Flag */
pub const X86_EFLAGS_TF: u64 = 1u64 << X86_EFLAGS_TF_BIT;
pub const X86_EFLAGS_IF_BIT: u32 = 9; /* Interrupt Flag */
pub const X86_EFLAGS_IF: u64 = 1u64 << X86_EFLAGS_IF_BIT;
pub const X86_EFLAGS_DF_BIT: u32 = 10; /* Direction Flag */
pub const X86_EFLAGS_DF: u64 = 1u64 << X86_EFLAGS_DF_BIT;
pub const X86_EFLAGS_OF_BIT: u32 = 11; /* Overflow Flag */
pub const X86_EFLAGS_OF: u64 = 1u64 << X86_EFLAGS_OF_BIT;
pub const X86_EFLAGS_IOPL_BIT: u32 = 12; /* I/O Privilege Level (2 bits) */
pub const X86_EFLAGS_IOPL: u64 = 3u64 << X86_EFLAGS_IOPL_BIT;
pub const X86_EFLAGS_NT_BIT: u32 = 14; /* Nested Task */
pub const X86_EFLAGS_NT: u64 = 1u64 << X86_EFLAGS_NT_BIT;
pub const X86_EFLAGS_RF_BIT: u32 = 16; /* Resume Flag */
pub const X86_EFLAGS_RF: u64 = 1u64 << X86_EFLAGS_RF_BIT;
pub const X86_EFLAGS_VM_BIT: u32 = 17; /* Virtual Mode */
pub const X86_EFLAGS_VM: u64 = 1u64 << X86_EFLAGS_VM_BIT;
pub const X86_EFLAGS_AC_BIT: u32 = 18; /* Alignment Check/Access Control */
pub const X86_EFLAGS_AC: u64 = 1u64 << X86_EFLAGS_AC_BIT;
pub const X86_EFLAGS_VIF_BIT: u32 = 19; /* Virtual Interrupt Flag */
pub const X86_EFLAGS_VIF: u64 = 1u64 << X86_EFLAGS_VIF_BIT;
pub const X86_EFLAGS_VIP_BIT: u32 = 20; /* Virtual Interrupt Pending */
pub const X86_EFLAGS_VIP: u64 = 1u64 << X86_EFLAGS_VIP_BIT;
pub const X86_EFLAGS_ID_BIT: u32 = 21; /* CPUID detection */
pub const X86_EFLAGS_ID: u64 = 1u64 << X86_EFLAGS_ID_BIT;

/* Basic CPU control in CR0 */
pub const X86_CR0_PE_BIT: u32 = 0; pub const X86_CR0_PE: u64 = 1u64 << X86_CR0_PE_BIT;
pub const X86_CR0_MP_BIT: u32 = 1; pub const X86_CR0_MP: u64 = 1u64 << X86_CR0_MP_BIT;
pub const X86_CR0_EM_BIT: u32 = 2; pub const X86_CR0_EM: u64 = 1u64 << X86_CR0_EM_BIT;
pub const X86_CR0_TS_BIT: u32 = 3; pub const X86_CR0_TS: u64 = 1u64 << X86_CR0_TS_BIT;
pub const X86_CR0_ET_BIT: u32 = 4; pub const X86_CR0_ET: u64 = 1u64 << X86_CR0_ET_BIT;
pub const X86_CR0_NE_BIT: u32 = 5; pub const X86_CR0_NE: u64 = 1u64 << X86_CR0_NE_BIT;
pub const X86_CR0_WP_BIT: u32 = 16; pub const X86_CR0_WP: u64 = 1u64 << X86_CR0_WP_BIT;
pub const X86_CR0_AM_BIT: u32 = 18; pub const X86_CR0_AM: u64 = 1u64 << X86_CR0_AM_BIT;
pub const X86_CR0_NW_BIT: u32 = 29; pub const X86_CR0_NW: u64 = 1u64 << X86_CR0_NW_BIT;
pub const X86_CR0_CD_BIT: u32 = 30; pub const X86_CR0_CD: u64 = 1u64 << X86_CR0_CD_BIT;
pub const X86_CR0_PG_BIT: u32 = 31; pub const X86_CR0_PG: u64 = 1u64 << X86_CR0_PG_BIT;

/* Paging options in CR3 */
pub const X86_CR3_PWT_BIT: u32 = 3; pub const X86_CR3_PWT: u64 = 1u64 << X86_CR3_PWT_BIT;
pub const X86_CR3_PCD_BIT: u32 = 4; pub const X86_CR3_PCD: u64 = 1u64 << X86_CR3_PCD_BIT;
pub const X86_CR3_PCID_BITS: u32 = 12;
pub const X86_CR3_PCID_MASK: u64 = (1u64 << X86_CR3_PCID_BITS) - 1;
pub const X86_CR3_LAM_U57_BIT: u32 = 61; /* Activate LAM for userspace, 62:57 bits masked */
pub const X86_CR3_LAM_U57: u64 = 1u64 << X86_CR3_LAM_U57_BIT;
pub const X86_CR3_LAM_U48_BIT: u32 = 62; /* Activate LAM for userspace, 62:48 bits masked */
pub const X86_CR3_LAM_U48: u64 = 1u64 << X86_CR3_LAM_U48_BIT;
pub const X86_CR3_PCID_NOFLUSH_BIT: u32 = 63; /* Preserve old PCID */
pub const X86_CR3_PCID_NOFLUSH: u64 = 1u64 << X86_CR3_PCID_NOFLUSH_BIT;

/* Intel CPU features in CR4 */
pub const X86_CR4_VME_BIT: u32 = 0; pub const X86_CR4_VME: u64 = 1u64 << X86_CR4_VME_BIT;
pub const X86_CR4_PVI_BIT: u32 = 1; pub const X86_CR4_PVI: u64 = 1u64 << X86_CR4_PVI_BIT;
pub const X86_CR4_TSD_BIT: u32 = 2; pub const X86_CR4_TSD: u64 = 1u64 << X86_CR4_TSD_BIT;
pub const X86_CR4_DE_BIT: u32 = 3; pub const X86_CR4_DE: u64 = 1u64 << X86_CR4_DE_BIT;
pub const X86_CR4_PSE_BIT: u32 = 4; pub const X86_CR4_PSE: u64 = 1u64 << X86_CR4_PSE_BIT;
pub const X86_CR4_PAE_BIT: u32 = 5; pub const X86_CR4_PAE: u64 = 1u64 << X86_CR4_PAE_BIT;
pub const X86_CR4_MCE_BIT: u32 = 6; pub const X86_CR4_MCE: u64 = 1u64 << X86_CR4_MCE_BIT;
pub const X86_CR4_PGE_BIT: u32 = 7; pub const X86_CR4_PGE: u64 = 1u64 << X86_CR4_PGE_BIT;
pub const X86_CR4_PCE_BIT: u32 = 8; pub const X86_CR4_PCE: u64 = 1u64 << X86_CR4_PCE_BIT;
pub const X86_CR4_OSFXSR_BIT: u32 = 9; pub const X86_CR4_OSFXSR: u64 = 1u64 << X86_CR4_OSFXSR_BIT;
pub const X86_CR4_OSXMMEXCPT_BIT: u32 = 10; pub const X86_CR4_OSXMMEXCPT: u64 = 1u64 << X86_CR4_OSXMMEXCPT_BIT;
pub const X86_CR4_UMIP_BIT: u32 = 11; pub const X86_CR4_UMIP: u64 = 1u64 << X86_CR4_UMIP_BIT;
pub const X86_CR4_LA57_BIT: u32 = 12; pub const X86_CR4_LA57: u64 = 1u64 << X86_CR4_LA57_BIT;
pub const X86_CR4_VMXE_BIT: u32 = 13; pub const X86_CR4_VMXE: u64 = 1u64 << X86_CR4_VMXE_BIT;
pub const X86_CR4_SMXE_BIT: u32 = 14; pub const X86_CR4_SMXE: u64 = 1u64 << X86_CR4_SMXE_BIT;
pub const X86_CR4_FSGSBASE_BIT: u32 = 16; pub const X86_CR4_FSGSBASE: u64 = 1u64 << X86_CR4_FSGSBASE_BIT;
pub const X86_CR4_PCIDE_BIT: u32 = 17; pub const X86_CR4_PCIDE: u64 = 1u64 << X86_CR4_PCIDE_BIT;
pub const X86_CR4_OSXSAVE_BIT: u32 = 18; pub const X86_CR4_OSXSAVE: u64 = 1u64 << X86_CR4_OSXSAVE_BIT;
pub const X86_CR4_SMEP_BIT: u32 = 20; pub const X86_CR4_SMEP: u64 = 1u64 << X86_CR4_SMEP_BIT;
pub const X86_CR4_SMAP_BIT: u32 = 21; pub const X86_CR4_SMAP: u64 = 1u64 << X86_CR4_SMAP_BIT;
pub const X86_CR4_PKE_BIT: u32 = 22; pub const X86_CR4_PKE: u64 = 1u64 << X86_CR4_PKE_BIT;
pub const X86_CR4_CET_BIT: u32 = 23; pub const X86_CR4_CET: u64 = 1u64 << X86_CR4_CET_BIT;
pub const X86_CR4_LASS_BIT: u32 = 27; pub const X86_CR4_LASS: u64 = 1u64 << X86_CR4_LASS_BIT;
pub const X86_CR4_LAM_SUP_BIT: u32 = 28; pub const X86_CR4_LAM_SUP: u64 = 1u64 << X86_CR4_LAM_SUP_BIT;
/* __x86_64__: */
pub const X86_CR4_FRED_BIT: u32 = 32; pub const X86_CR4_FRED: u64 = 1u64 << X86_CR4_FRED_BIT;

/* x86-64 Task Priority Register, CR8 */
pub const X86_CR8_TPR: u64 = 0x0000000f;

/* AMD and Transmeta use MSRs for configuration; see <asm/msr-index.h> */
/* NSC/Cyrix CPU configuration register indexes */
pub const CX86_PCR0: u32 = 0x20;
pub const CX86_GCR: u32 = 0xb8;
pub const CX86_CCR0: u32 = 0xc0;
pub const CX86_CCR1: u32 = 0xc1;
pub const CX86_CCR2: u32 = 0xc2;
pub const CX86_CCR3: u32 = 0xc3;
pub const CX86_CCR4: u32 = 0xe8;
pub const CX86_CCR5: u32 = 0xe9;
pub const CX86_CCR6: u32 = 0xea;
pub const CX86_CCR7: u32 = 0xeb;
pub const CX86_PCR1: u32 = 0xf0;
pub const CX86_DIR0: u32 = 0xfe;
pub const CX86_DIR1: u32 = 0xff;
pub const CX86_ARR_BASE: u32 = 0xc4;
pub const CX86_RCR_BASE: u32 = 0xdc;

pub const CR0_STATE: u64 = X86_CR0_PE | X86_CR0_MP | X86_CR0_ET |
    X86_CR0_NE | X86_CR0_WP | X86_CR0_AM | X86_CR0_PG;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
