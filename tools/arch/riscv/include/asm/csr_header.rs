/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Regents of the University of California
 */

// Removed C header guard and include <linux/bits.h>; BIT/GENMASK are translated locally.

pub const fn bit(n: usize) -> usize {
    1usize << n
}

pub const fn genmask(h: usize, l: usize) -> usize {
    ((!0usize) >> (usize::BITS as usize - 1 - h)) & ((!0usize) << l)
}

/* Status register flags */
pub const SR_SIE: usize = 0x00000002; /* Supervisor Interrupt Enable */
pub const SR_MIE: usize = 0x00000008; /* Machine Interrupt Enable */
pub const SR_SPIE: usize = 0x00000020; /* Previous Supervisor IE */
pub const SR_MPIE: usize = 0x00000080; /* Previous Machine IE */
pub const SR_SPP: usize = 0x00000100; /* Previously Supervisor */
pub const SR_MPP: usize = 0x00001800; /* Previously Machine */
pub const SR_SUM: usize = 0x00040000; /* Supervisor User Memory Access */

pub const SR_FS: usize = 0x00006000; /* Floating-point Status */
pub const SR_FS_OFF: usize = 0x00000000;
pub const SR_FS_INITIAL: usize = 0x00002000;
pub const SR_FS_CLEAN: usize = 0x00004000;
pub const SR_FS_DIRTY: usize = 0x00006000;

pub const SR_VS: usize = 0x00000600; /* Vector Status */
pub const SR_VS_OFF: usize = 0x00000000;
pub const SR_VS_INITIAL: usize = 0x00000200;
pub const SR_VS_CLEAN: usize = 0x00000400;
pub const SR_VS_DIRTY: usize = 0x00000600;

pub const SR_XS: usize = 0x00018000; /* Extension Status */
pub const SR_XS_OFF: usize = 0x00000000;
pub const SR_XS_INITIAL: usize = 0x00008000;
pub const SR_XS_CLEAN: usize = 0x00010000;
pub const SR_XS_DIRTY: usize = 0x00018000;

pub const SR_FS_VS: usize = SR_FS | SR_VS; /* Vector and Floating-Point Unit */

#[cfg(not(CONFIG_64BIT))]
pub const SR_SD: usize = 0x80000000; /* FS/VS/XS dirty */
#[cfg(CONFIG_64BIT)]
pub const SR_SD: usize = 0x8000000000000000; /* FS/VS/XS dirty */

#[cfg(CONFIG_64BIT)]
pub const SR_UXL: usize = 0x300000000; /* XLEN mask for U-mode */
#[cfg(CONFIG_64BIT)]
pub const SR_UXL_32: usize = 0x100000000; /* XLEN = 32 for U-mode */
#[cfg(CONFIG_64BIT)]
pub const SR_UXL_64: usize = 0x200000000; /* XLEN = 64 for U-mode */

/* SATP flags */
#[cfg(not(CONFIG_64BIT))]
pub const SATP_PPN: usize = 0x003FFFFF;
#[cfg(not(CONFIG_64BIT))]
pub const SATP_MODE_32: usize = 0x80000000;
#[cfg(not(CONFIG_64BIT))]
pub const SATP_MODE_SHIFT: usize = 31;
#[cfg(not(CONFIG_64BIT))]
pub const SATP_ASID_BITS: usize = 9;
#[cfg(not(CONFIG_64BIT))]
pub const SATP_ASID_SHIFT: usize = 22;
#[cfg(not(CONFIG_64BIT))]
pub const SATP_ASID_MASK: usize = 0x1FF;
#[cfg(CONFIG_64BIT)]
pub const SATP_PPN: usize = 0x00000FFFFFFFFFFF;
#[cfg(CONFIG_64BIT)]
pub const SATP_MODE_39: usize = 0x8000000000000000;
#[cfg(CONFIG_64BIT)]
pub const SATP_MODE_48: usize = 0x9000000000000000;
#[cfg(CONFIG_64BIT)]
pub const SATP_MODE_57: usize = 0xa000000000000000;
#[cfg(CONFIG_64BIT)]
pub const SATP_MODE_SHIFT: usize = 60;
#[cfg(CONFIG_64BIT)]
pub const SATP_ASID_BITS: usize = 16;
#[cfg(CONFIG_64BIT)]
pub const SATP_ASID_SHIFT: usize = 44;
#[cfg(CONFIG_64BIT)]
pub const SATP_ASID_MASK: usize = 0xFFFF;

/* Exception cause high bit - is an interrupt if set */
pub const CAUSE_IRQ_FLAG: usize = 1usize << (usize::BITS as usize - 1);

/* Interrupt causes (minus the high bit) */
pub const IRQ_S_SOFT: usize = 1;
pub const IRQ_VS_SOFT: usize = 2;
pub const IRQ_M_SOFT: usize = 3;
pub const IRQ_S_TIMER: usize = 5;
pub const IRQ_VS_TIMER: usize = 6;
pub const IRQ_M_TIMER: usize = 7;
pub const IRQ_S_EXT: usize = 9;
pub const IRQ_VS_EXT: usize = 10;
pub const IRQ_M_EXT: usize = 11;
pub const IRQ_S_GEXT: usize = 12;
pub const IRQ_PMU_OVF: usize = 13;
pub const IRQ_LOCAL_MAX: usize = IRQ_PMU_OVF + 1;
pub const IRQ_LOCAL_MASK: usize = genmask(IRQ_LOCAL_MAX - 1, 0);

/* Exception causes */
pub const EXC_INST_MISALIGNED: usize = 0;
pub const EXC_INST_ACCESS: usize = 1;
pub const EXC_INST_ILLEGAL: usize = 2;
pub const EXC_BREAKPOINT: usize = 3;
pub const EXC_LOAD_MISALIGNED: usize = 4;
pub const EXC_LOAD_ACCESS: usize = 5;
pub const EXC_STORE_MISALIGNED: usize = 6;
pub const EXC_STORE_ACCESS: usize = 7;
pub const EXC_SYSCALL: usize = 8;
pub const EXC_HYPERVISOR_SYSCALL: usize = 9;
pub const EXC_SUPERVISOR_SYSCALL: usize = 10;
pub const EXC_INST_PAGE_FAULT: usize = 12;
pub const EXC_LOAD_PAGE_FAULT: usize = 13;
pub const EXC_STORE_PAGE_FAULT: usize = 15;
pub const EXC_INST_GUEST_PAGE_FAULT: usize = 20;
pub const EXC_LOAD_GUEST_PAGE_FAULT: usize = 21;
pub const EXC_VIRTUAL_INST_FAULT: usize = 22;
pub const EXC_STORE_GUEST_PAGE_FAULT: usize = 23;

/* PMP configuration */
pub const PMP_R: usize = 0x01;
pub const PMP_W: usize = 0x02;
pub const PMP_X: usize = 0x04;
pub const PMP_A: usize = 0x18;
pub const PMP_A_TOR: usize = 0x08;
pub const PMP_A_NA4: usize = 0x10;
pub const PMP_A_NAPOT: usize = 0x18;
pub const PMP_L: usize = 0x80;

/* HSTATUS flags */
#[cfg(CONFIG_64BIT)]
pub const HSTATUS_VSXL: usize = 0x300000000;
#[cfg(CONFIG_64BIT)]
pub const HSTATUS_VSXL_SHIFT: usize = 32;
pub const HSTATUS_VTSR: usize = 0x00400000;
pub const HSTATUS_VTW: usize = 0x00200000;
pub const HSTATUS_VTVM: usize = 0x00100000;
pub const HSTATUS_VGEIN: usize = 0x0003f000;
pub const HSTATUS_VGEIN_SHIFT: usize = 12;
pub const HSTATUS_HU: usize = 0x00000200;
pub const HSTATUS_SPVP: usize = 0x00000100;
pub const HSTATUS_SPV: usize = 0x00000080;
pub const HSTATUS_GVA: usize = 0x00000040;
pub const HSTATUS_VSBE: usize = 0x00000020;

/* HGATP flags */
pub const HGATP_MODE_OFF: usize = 0;
pub const HGATP_MODE_SV32X4: usize = 1;
pub const HGATP_MODE_SV39X4: usize = 8;
pub const HGATP_MODE_SV48X4: usize = 9;
pub const HGATP_MODE_SV57X4: usize = 10;

pub const HGATP32_MODE_SHIFT: usize = 31;
pub const HGATP32_VMID_SHIFT: usize = 22;
pub const HGATP32_VMID: usize = genmask(28, 22);
pub const HGATP32_PPN: usize = genmask(21, 0);

pub const HGATP64_MODE_SHIFT: usize = 60;
pub const HGATP64_VMID_SHIFT: usize = 44;
pub const HGATP64_VMID: usize = genmask(57, 44);
pub const HGATP64_PPN: usize = genmask(43, 0);

pub const HGATP_PAGE_SHIFT: usize = 12;

#[cfg(CONFIG_64BIT)]
pub const HGATP_PPN: usize = HGATP64_PPN;
#[cfg(CONFIG_64BIT)]
pub const HGATP_VMID_SHIFT: usize = HGATP64_VMID_SHIFT;
#[cfg(CONFIG_64BIT)]
pub const HGATP_VMID: usize = HGATP64_VMID;
#[cfg(CONFIG_64BIT)]
pub const HGATP_MODE_SHIFT: usize = HGATP64_MODE_SHIFT;
#[cfg(not(CONFIG_64BIT))]
pub const HGATP_PPN: usize = HGATP32_PPN;
#[cfg(not(CONFIG_64BIT))]
pub const HGATP_VMID_SHIFT: usize = HGATP32_VMID_SHIFT;
#[cfg(not(CONFIG_64BIT))]
pub const HGATP_VMID: usize = HGATP32_VMID;
#[cfg(not(CONFIG_64BIT))]
pub const HGATP_MODE_SHIFT: usize = HGATP32_MODE_SHIFT;

/* VSIP & HVIP relation */
pub const VSIP_TO_HVIP_SHIFT: usize = IRQ_VS_SOFT - IRQ_S_SOFT;
pub const VSIP_VALID_MASK: usize = (1usize << IRQ_S_SOFT)
    | (1usize << IRQ_S_TIMER)
    | (1usize << IRQ_S_EXT)
    | (1usize << IRQ_PMU_OVF);

/* AIA CSR bits */
pub const TOPI_IID_SHIFT: usize = 16;
pub const TOPI_IID_MASK: usize = genmask(11, 0);
pub const TOPI_IPRIO_MASK: usize = genmask(7, 0);
pub const TOPI_IPRIO_BITS: usize = 8;

pub const TOPEI_ID_SHIFT: usize = 16;
pub const TOPEI_ID_MASK: usize = genmask(10, 0);
pub const TOPEI_PRIO_MASK: usize = genmask(10, 0);

pub const ISELECT_IPRIO0: usize = 0x30;
pub const ISELECT_IPRIO15: usize = 0x3f;
pub const ISELECT_MASK: usize = genmask(8, 0);

pub const HVICTL_VTI: usize = bit(30);
pub const HVICTL_IID: usize = genmask(27, 16);
pub const HVICTL_IID_SHIFT: usize = 16;
pub const HVICTL_DPR: usize = bit(9);
pub const HVICTL_IPRIOM: usize = bit(8);
pub const HVICTL_IPRIO: usize = genmask(7, 0);

/* xENVCFG flags */
pub const ENVCFG_STCE: u64 = 1u64 << 63;
pub const ENVCFG_PBMTE: u64 = 1u64 << 62;
pub const ENVCFG_CBZE: usize = 1usize << 7;
pub const ENVCFG_CBCFE: usize = 1usize << 6;
pub const ENVCFG_CBIE_SHIFT: usize = 4;
pub const ENVCFG_CBIE: usize = 0x3usize << ENVCFG_CBIE_SHIFT;
pub const ENVCFG_CBIE_ILL: usize = 0x0;
pub const ENVCFG_CBIE_FLUSH: usize = 0x1;
pub const ENVCFG_CBIE_INV: usize = 0x3;
pub const ENVCFG_FIOM: usize = 0x1;

/* Smstateen bits */
pub const SMSTATEEN0_AIA_IMSIC_SHIFT: usize = 58;
pub const SMSTATEEN0_AIA_IMSIC: u64 = 1u64 << SMSTATEEN0_AIA_IMSIC_SHIFT;
pub const SMSTATEEN0_AIA_SHIFT: usize = 59;
pub const SMSTATEEN0_AIA: u64 = 1u64 << SMSTATEEN0_AIA_SHIFT;
pub const SMSTATEEN0_AIA_ISEL_SHIFT: usize = 60;
pub const SMSTATEEN0_AIA_ISEL: u64 = 1u64 << SMSTATEEN0_AIA_ISEL_SHIFT;
pub const SMSTATEEN0_HSENVCFG_SHIFT: usize = 62;
pub const SMSTATEEN0_HSENVCFG: u64 = 1u64 << SMSTATEEN0_HSENVCFG_SHIFT;
pub const SMSTATEEN0_SSTATEEN0_SHIFT: usize = 63;
pub const SMSTATEEN0_SSTATEEN0: u64 = 1u64 << SMSTATEEN0_SSTATEEN0_SHIFT;

/* symbolic CSR names: */
pub const CSR_CYCLE: usize = 0xc00;
pub const CSR_TIME: usize = 0xc01;
pub const CSR_INSTRET: usize = 0xc02;
pub const CSR_HPMCOUNTER3: usize = 0xc03;
pub const CSR_HPMCOUNTER4: usize = 0xc04;
pub const CSR_HPMCOUNTER5: usize = 0xc05;
pub const CSR_HPMCOUNTER6: usize = 0xc06;
pub const CSR_HPMCOUNTER7: usize = 0xc07;
pub const CSR_HPMCOUNTER8: usize = 0xc08;
pub const CSR_HPMCOUNTER9: usize = 0xc09;
pub const CSR_HPMCOUNTER10: usize = 0xc0a;
pub const CSR_HPMCOUNTER11: usize = 0xc0b;
pub const CSR_HPMCOUNTER12: usize = 0xc0c;
pub const CSR_HPMCOUNTER13: usize = 0xc0d;
pub const CSR_HPMCOUNTER14: usize = 0xc0e;
pub const CSR_HPMCOUNTER15: usize = 0xc0f;
pub const CSR_HPMCOUNTER16: usize = 0xc10;
pub const CSR_HPMCOUNTER17: usize = 0xc11;
pub const CSR_HPMCOUNTER18: usize = 0xc12;
pub const CSR_HPMCOUNTER19: usize = 0xc13;
pub const CSR_HPMCOUNTER20: usize = 0xc14;
pub const CSR_HPMCOUNTER21: usize = 0xc15;
pub const CSR_HPMCOUNTER22: usize = 0xc16;
pub const CSR_HPMCOUNTER23: usize = 0xc17;
pub const CSR_HPMCOUNTER24: usize = 0xc18;
pub const CSR_HPMCOUNTER25: usize = 0xc19;
pub const CSR_HPMCOUNTER26: usize = 0xc1a;
pub const CSR_HPMCOUNTER27: usize = 0xc1b;
pub const CSR_HPMCOUNTER28: usize = 0xc1c;
pub const CSR_HPMCOUNTER29: usize = 0xc1d;
pub const CSR_HPMCOUNTER30: usize = 0xc1e;
pub const CSR_HPMCOUNTER31: usize = 0xc1f;
pub const CSR_CYCLEH: usize = 0xc80;
pub const CSR_TIMEH: usize = 0xc81;
pub const CSR_INSTRETH: usize = 0xc82;
pub const CSR_HPMCOUNTER3H: usize = 0xc83;
pub const CSR_HPMCOUNTER4H: usize = 0xc84;
pub const CSR_HPMCOUNTER5H: usize = 0xc85;
pub const CSR_HPMCOUNTER6H: usize = 0xc86;
pub const CSR_HPMCOUNTER7H: usize = 0xc87;
pub const CSR_HPMCOUNTER8H: usize = 0xc88;
pub const CSR_HPMCOUNTER9H: usize = 0xc89;
pub const CSR_HPMCOUNTER10H: usize = 0xc8a;
pub const CSR_HPMCOUNTER11H: usize = 0xc8b;
pub const CSR_HPMCOUNTER12H: usize = 0xc8c;
pub const CSR_HPMCOUNTER13H: usize = 0xc8d;
pub const CSR_HPMCOUNTER14H: usize = 0xc8e;
pub const CSR_HPMCOUNTER15H: usize = 0xc8f;
pub const CSR_HPMCOUNTER16H: usize = 0xc90;
pub const CSR_HPMCOUNTER17H: usize = 0xc91;
pub const CSR_HPMCOUNTER18H: usize = 0xc92;
pub const CSR_HPMCOUNTER19H: usize = 0xc93;
pub const CSR_HPMCOUNTER20H: usize = 0xc94;
pub const CSR_HPMCOUNTER21H: usize = 0xc95;
pub const CSR_HPMCOUNTER22H: usize = 0xc96;
pub const CSR_HPMCOUNTER23H: usize = 0xc97;
pub const CSR_HPMCOUNTER24H: usize = 0xc98;
pub const CSR_HPMCOUNTER25H: usize = 0xc99;
pub const CSR_HPMCOUNTER26H: usize = 0xc9a;
pub const CSR_HPMCOUNTER27H: usize = 0xc9b;
pub const CSR_HPMCOUNTER28H: usize = 0xc9c;
pub const CSR_HPMCOUNTER29H: usize = 0xc9d;
pub const CSR_HPMCOUNTER30H: usize = 0xc9e;
pub const CSR_HPMCOUNTER31H: usize = 0xc9f;

pub const CSR_SCOUNTOVF: usize = 0xda0;

pub const CSR_SSTATUS: usize = 0x100;
pub const CSR_SIE: usize = 0x104;
pub const CSR_STVEC: usize = 0x105;
pub const CSR_SCOUNTEREN: usize = 0x106;
pub const CSR_SENVCFG: usize = 0x10a;
pub const CSR_SSTATEEN0: usize = 0x10c;
pub const CSR_SSCRATCH: usize = 0x140;
pub const CSR_SEPC: usize = 0x141;
pub const CSR_SCAUSE: usize = 0x142;
pub const CSR_STVAL: usize = 0x143;
pub const CSR_SIP: usize = 0x144;
pub const CSR_SATP: usize = 0x180;

pub const CSR_STIMECMP: usize = 0x14D;
pub const CSR_STIMECMPH: usize = 0x15D;

/* Supervisor-Level Window to Indirectly Accessed Registers (AIA) */
pub const CSR_SISELECT: usize = 0x150;
pub const CSR_SIREG: usize = 0x151;

/* Supervisor-Level Interrupts (AIA) */
pub const CSR_STOPEI: usize = 0x15c;
pub const CSR_STOPI: usize = 0xdb0;

/* Supervisor-Level High-Half CSRs (AIA) */
pub const CSR_SIEH: usize = 0x114;
pub const CSR_SIPH: usize = 0x154;

pub const CSR_VSSTATUS: usize = 0x200;
pub const CSR_VSIE: usize = 0x204;
pub const CSR_VSTVEC: usize = 0x205;
pub const CSR_VSSCRATCH: usize = 0x240;
pub const CSR_VSEPC: usize = 0x241;
pub const CSR_VSCAUSE: usize = 0x242;
pub const CSR_VSTVAL: usize = 0x243;
pub const CSR_VSIP: usize = 0x244;
pub const CSR_VSATP: usize = 0x280;
pub const CSR_VSTIMECMP: usize = 0x24D;
pub const CSR_VSTIMECMPH: usize = 0x25D;

pub const CSR_HSTATUS: usize = 0x600;
pub const CSR_HEDELEG: usize = 0x602;
pub const CSR_HIDELEG: usize = 0x603;
pub const CSR_HIE: usize = 0x604;
pub const CSR_HTIMEDELTA: usize = 0x605;
pub const CSR_HCOUNTEREN: usize = 0x606;
pub const CSR_HGEIE: usize = 0x607;
pub const CSR_HENVCFG: usize = 0x60a;
pub const CSR_HTIMEDELTAH: usize = 0x615;
pub const CSR_HENVCFGH: usize = 0x61a;
pub const CSR_HTVAL: usize = 0x643;
pub const CSR_HIP: usize = 0x644;
pub const CSR_HVIP: usize = 0x645;
pub const CSR_HTINST: usize = 0x64a;
pub const CSR_HGATP: usize = 0x680;
pub const CSR_HGEIP: usize = 0xe12;

/* Virtual Interrupts and Interrupt Priorities (H-extension with AIA) */
pub const CSR_HVIEN: usize = 0x608;
pub const CSR_HVICTL: usize = 0x609;
pub const CSR_HVIPRIO1: usize = 0x646;
pub const CSR_HVIPRIO2: usize = 0x647;

/* VS-Level Window to Indirectly Accessed Registers (H-extension with AIA) */
pub const CSR_VSISELECT: usize = 0x250;
pub const CSR_VSIREG: usize = 0x251;

/* VS-Level Interrupts (H-extension with AIA) */
pub const CSR_VSTOPEI: usize = 0x25c;
pub const CSR_VSTOPI: usize = 0xeb0;

/* Hypervisor and VS-Level High-Half CSRs (H-extension with AIA) */
pub const CSR_HIDELEGH: usize = 0x613;
pub const CSR_HVIENH: usize = 0x618;
pub const CSR_HVIPH: usize = 0x655;
pub const CSR_HVIPRIO1H: usize = 0x656;
pub const CSR_HVIPRIO2H: usize = 0x657;
pub const CSR_VSIEH: usize = 0x214;
pub const CSR_VSIPH: usize = 0x254;

/* Hypervisor stateen CSRs */
pub const CSR_HSTATEEN0: usize = 0x60c;
pub const CSR_HSTATEEN0H: usize = 0x61c;

pub const CSR_MSTATUS: usize = 0x300;
pub const CSR_MISA: usize = 0x301;
pub const CSR_MIDELEG: usize = 0x303;
pub const CSR_MIE: usize = 0x304;
pub const CSR_MTVEC: usize = 0x305;
pub const CSR_MENVCFG: usize = 0x30a;
pub const CSR_MENVCFGH: usize = 0x31a;
pub const CSR_MSCRATCH: usize = 0x340;
pub const CSR_MEPC: usize = 0x341;
pub const CSR_MCAUSE: usize = 0x342;
pub const CSR_MTVAL: usize = 0x343;
pub const CSR_MIP: usize = 0x344;
pub const CSR_PMPCFG0: usize = 0x3a0;
pub const CSR_PMPADDR0: usize = 0x3b0;
pub const CSR_MVENDORID: usize = 0xf11;
pub const CSR_MARCHID: usize = 0xf12;
pub const CSR_MIMPID: usize = 0xf13;
pub const CSR_MHARTID: usize = 0xf14;

/* Machine-Level Window to Indirectly Accessed Registers (AIA) */
pub const CSR_MISELECT: usize = 0x350;
pub const CSR_MIREG: usize = 0x351;

/* Machine-Level Interrupts (AIA) */
pub const CSR_MTOPEI: usize = 0x35c;
pub const CSR_MTOPI: usize = 0xfb0;

/* Virtual Interrupts for Supervisor Level (AIA) */
pub const CSR_MVIEN: usize = 0x308;
pub const CSR_MVIP: usize = 0x309;

/* Machine-Level High-Half CSRs (AIA) */
pub const CSR_MIDELEGH: usize = 0x313;
pub const CSR_MIEH: usize = 0x314;
pub const CSR_MVIENH: usize = 0x318;
pub const CSR_MVIPH: usize = 0x319;
pub const CSR_MIPH: usize = 0x354;

pub const CSR_VSTART: usize = 0x8;
pub const CSR_VCSR: usize = 0xf;
pub const CSR_VL: usize = 0xc20;
pub const CSR_VTYPE: usize = 0xc21;
pub const CSR_VLENB: usize = 0xc22;

#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_STATUS: usize = CSR_MSTATUS;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_IE: usize = CSR_MIE;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_TVEC: usize = CSR_MTVEC;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_SCRATCH: usize = CSR_MSCRATCH;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_EPC: usize = CSR_MEPC;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_CAUSE: usize = CSR_MCAUSE;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_TVAL: usize = CSR_MTVAL;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_IP: usize = CSR_MIP;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_IEH: usize = CSR_MIEH;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_ISELECT: usize = CSR_MISELECT;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_IREG: usize = CSR_MIREG;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_IPH: usize = CSR_MIPH;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_TOPEI: usize = CSR_MTOPEI;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const CSR_TOPI: usize = CSR_MTOPI;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const SR_IE: usize = SR_MIE;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const SR_PIE: usize = SR_MPIE;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const SR_PP: usize = SR_MPP;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const RV_IRQ_SOFT: usize = IRQ_M_SOFT;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const RV_IRQ_TIMER: usize = IRQ_M_TIMER;
#[cfg(CONFIG_RISCV_M_MODE)]
pub const RV_IRQ_EXT: usize = IRQ_M_EXT;

#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_STATUS: usize = CSR_SSTATUS;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_IE: usize = CSR_SIE;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_TVEC: usize = CSR_STVEC;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_SCRATCH: usize = CSR_SSCRATCH;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_EPC: usize = CSR_SEPC;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_CAUSE: usize = CSR_SCAUSE;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_TVAL: usize = CSR_STVAL;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_IP: usize = CSR_SIP;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_IEH: usize = CSR_SIEH;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_ISELECT: usize = CSR_SISELECT;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_IREG: usize = CSR_SIREG;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_IPH: usize = CSR_SIPH;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_TOPEI: usize = CSR_STOPEI;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const CSR_TOPI: usize = CSR_STOPI;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const SR_IE: usize = SR_SIE;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const SR_PIE: usize = SR_SPIE;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const SR_PP: usize = SR_SPP;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const RV_IRQ_SOFT: usize = IRQ_S_SOFT;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const RV_IRQ_TIMER: usize = IRQ_S_TIMER;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const RV_IRQ_EXT: usize = IRQ_S_EXT;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const RV_IRQ_PMU: usize = IRQ_PMU_OVF;
#[cfg(not(CONFIG_RISCV_M_MODE))]
pub const SIP_LCOFIP: usize = 0x1usize << IRQ_PMU_OVF;

/* IE/IP (Supervisor/Machine Interrupt Enable/Pending) flags */
pub const IE_SIE: usize = 0x1usize << RV_IRQ_SOFT;
pub const IE_TIE: usize = 0x1usize << RV_IRQ_TIMER;
pub const IE_EIE: usize = 0x1usize << RV_IRQ_EXT;

// __ASM_STR is a C preprocessor-only helper; Rust inline assembly uses const
// operands for the translated CSR helpers below.

#[cfg(not(__ASSEMBLER__))]
#[inline]
pub unsafe fn csr_swap<const CSR: usize>(val: usize) -> usize {
    let mut __v = val;
    unsafe {
        core::arch::asm!(
            "csrrw {0}, {csr}, {0}",
            inout(reg) __v,
            csr = const CSR,
            options(nostack, preserves_flags),
        );
    }
    __v
}

#[cfg(not(__ASSEMBLER__))]
#[inline]
pub unsafe fn csr_read<const CSR: usize>() -> usize {
    let __v: usize;
    unsafe {
        core::arch::asm!(
            "csrr {0}, {csr}",
            out(reg) __v,
            csr = const CSR,
            options(nostack, preserves_flags),
        );
    }
    __v
}

#[cfg(not(__ASSEMBLER__))]
#[inline]
pub unsafe fn csr_write<const CSR: usize>(val: usize) {
    let __v = val;
    unsafe {
        core::arch::asm!(
            "csrw {csr}, {0}",
            in(reg) __v,
            csr = const CSR,
            options(nostack, preserves_flags),
        );
    }
}

#[cfg(not(__ASSEMBLER__))]
#[inline]
pub unsafe fn csr_read_set<const CSR: usize>(val: usize) -> usize {
    let mut __v = val;
    unsafe {
        core::arch::asm!(
            "csrrs {0}, {csr}, {0}",
            inout(reg) __v,
            csr = const CSR,
            options(nostack, preserves_flags),
        );
    }
    __v
}

#[cfg(not(__ASSEMBLER__))]
#[inline]
pub unsafe fn csr_set<const CSR: usize>(val: usize) {
    let __v = val;
    unsafe {
        core::arch::asm!(
            "csrs {csr}, {0}",
            in(reg) __v,
            csr = const CSR,
            options(nostack, preserves_flags),
        );
    }
}

#[cfg(not(__ASSEMBLER__))]
#[inline]
pub unsafe fn csr_read_clear<const CSR: usize>(val: usize) -> usize {
    let mut __v = val;
    unsafe {
        core::arch::asm!(
            "csrrc {0}, {csr}, {0}",
            inout(reg) __v,
            csr = const CSR,
            options(nostack, preserves_flags),
        );
    }
    __v
}

#[cfg(not(__ASSEMBLER__))]
#[inline]
pub unsafe fn csr_clear<const CSR: usize>(val: usize) {
    let __v = val;
    unsafe {
        core::arch::asm!(
            "csrc {csr}, {0}",
            in(reg) __v,
            csr = const CSR,
            options(nostack, preserves_flags),
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
