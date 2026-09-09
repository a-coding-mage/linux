/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2015 Regents of the University of California */

// Translated from asm/csr.h. `CONFIG_64BIT`, `CONFIG_RISCV_M_MODE`, and
// `__riscv_xlen` are build configuration supplied by the including crate.

/* Status register flags */
pub const SR_SIE: usize = 0x00000002; pub const SR_MIE: usize = 0x00000008;
pub const SR_SPIE: usize = 0x00000020; pub const SR_MPIE: usize = 0x00000080;
pub const SR_SPP: usize = 0x00000100; pub const SR_MPP: usize = 0x00001800;
pub const SR_SUM: usize = 0x00040000;
pub const SR_SPELP: u64 = 0x00800000; pub const SR_MPELP: u64 = 0x020000000000;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const SR_ELP: u64 = SR_MPELP;
#[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const SR_ELP: u64 = SR_SPELP;
pub const SR_FS: usize = 0x6000; pub const SR_FS_OFF: usize = 0; pub const SR_FS_INITIAL: usize = 0x2000; pub const SR_FS_CLEAN: usize = 0x4000; pub const SR_FS_DIRTY: usize = 0x6000;
pub const SR_VS: usize = 0x600; pub const SR_VS_OFF: usize = 0; pub const SR_VS_INITIAL: usize = 0x200; pub const SR_VS_CLEAN: usize = 0x400; pub const SR_VS_DIRTY: usize = 0x600;
pub const SR_VS_THEAD: usize = 0x01800000; pub const SR_VS_OFF_THEAD: usize = 0; pub const SR_VS_INITIAL_THEAD: usize = 0x00800000; pub const SR_VS_CLEAN_THEAD: usize = 0x01000000; pub const SR_VS_DIRTY_THEAD: usize = 0x01800000;
pub const SR_XS: usize = 0x18000; pub const SR_XS_OFF: usize = 0; pub const SR_XS_INITIAL: usize = 0x8000; pub const SR_XS_CLEAN: usize = 0x10000; pub const SR_XS_DIRTY: usize = 0x18000;
pub const SR_FS_VS: usize = SR_FS | SR_VS;
#[cfg(not(feature = "CONFIG_64BIT"))] pub const SR_SD: u64 = 0x80000000;
#[cfg(feature = "CONFIG_64BIT")] pub const SR_SD: u64 = 0x8000000000000000;
#[cfg(feature = "CONFIG_64BIT")] pub const SR_UXL: u64 = 0x300000000; pub const SR_UXL_32: u64 = 0x100000000; pub const SR_UXL_64: u64 = 0x200000000;

/* SATP flags */
#[cfg(not(feature = "CONFIG_64BIT"))] { pub const SATP_PPN: u64 = 0x003fffff; pub const SATP_MODE_32: u64 = 0x80000000; pub const SATP_MODE_SHIFT: usize = 31; pub const SATP_ASID_BITS: usize = 9; pub const SATP_ASID_SHIFT: usize = 22; pub const SATP_ASID_MASK: u64 = 0x1ff; }
#[cfg(feature = "CONFIG_64BIT")] { pub const SATP_PPN: u64 = 0x00000fffffffffff; pub const SATP_MODE_39: u64 = 0x8000000000000000; pub const SATP_MODE_48: u64 = 0x9000000000000000; pub const SATP_MODE_57: u64 = 0xa000000000000000; pub const SATP_MODE_SHIFT: usize = 60; pub const SATP_ASID_BITS: usize = 16; pub const SATP_ASID_SHIFT: usize = 44; pub const SATP_ASID_MASK: u64 = 0xffff; }

pub const SRMCFG_RCID_MASK: u64 = (1 << 12) - 1; pub const SRMCFG_MCID_MASK: u64 = ((1 << 12) - 1) << 16;
pub const HSTATUS_VTSR:u64=0x00400000; pub const HSTATUS_VTW:u64=0x00200000; pub const HSTATUS_VTVM:u64=0x00100000; pub const HSTATUS_VGEIN:u64=0x0003f000; pub const HSTATUS_VGEIN_SHIFT:usize=12; pub const HSTATUS_HU:u64=0x200; pub const HSTATUS_SPVP:u64=0x100; pub const HSTATUS_SPV:u64=0x80; pub const HSTATUS_GVA:u64=0x40; pub const HSTATUS_VSBE:u64=0x20;
pub const HGATP_MODE_OFF:usize=0; pub const HGATP_MODE_SV32X4:usize=1; pub const HGATP_MODE_SV39X4:usize=8; pub const HGATP_MODE_SV48X4:usize=9; pub const HGATP_MODE_SV57X4:usize=10; pub const HGATP32_MODE_SHIFT:usize=31; pub const HGATP32_VMID_SHIFT:usize=22; pub const HGATP32_VMID:u64=((1<<7)-1)<<22; pub const HGATP32_PPN:u64=(1<<22)-1; pub const HGATP64_MODE_SHIFT:usize=60; pub const HGATP64_VMID_SHIFT:usize=44; pub const HGATP64_VMID:u64=((1<<14)-1)<<44; pub const HGATP64_PPN:u64=(1u64<<44)-1; pub const HGATP_PAGE_SHIFT:usize=12;
pub const VSIP_TO_HVIP_SHIFT:usize=IRQ_VS_SOFT-IRQ_S_SOFT; pub const VSIP_VALID_MASK:usize=(1<<IRQ_S_SOFT)|(1<<IRQ_S_TIMER)|(1<<IRQ_S_EXT)|(1<<IRQ_PMU_OVF);
pub const TOPI_IID_SHIFT:usize=16; pub const TOPI_IID_MASK:usize=(1<<12)-1; pub const TOPI_IPRIO_MASK:usize=0xff; pub const TOPI_IPRIO_BITS:usize=8; pub const TOPEI_ID_SHIFT:usize=16; pub const TOPEI_ID_MASK:usize=(1<<11)-1; pub const TOPEI_PRIO_MASK:usize=(1<<11)-1; pub const ISELECT_IPRIO0:usize=0x30; pub const ISELECT_IPRIO15:usize=0x3f; pub const ISELECT_MASK:usize=(1<<9)-1; pub const HVICTL_VTI:u64=1<<30; pub const HVICTL_IID:u64=((1<<12)-1)<<16; pub const HVICTL_IID_SHIFT:usize=16; pub const HVICTL_DPR:usize=1<<9; pub const HVICTL_IPRIOM:usize=1<<8; pub const HVICTL_IPRIO:usize=0xff;
pub const ENVCFG_STCE:u64=1<<63; pub const ENVCFG_PBMTE:u64=1<<62; pub const ENVCFG_ADUE:u64=1<<61; pub const ENVCFG_PMM:u64=3<<32; pub const ENVCFG_PMM_PMLEN_0:u64=0; pub const ENVCFG_PMM_PMLEN_7:u64=2<<32; pub const ENVCFG_PMM_PMLEN_16:u64=3<<32; pub const ENVCFG_CBZE:usize=1<<7; pub const ENVCFG_CBCFE:usize=1<<6; pub const ENVCFG_LPE:usize=1<<2; pub const ENVCFG_SSE:usize=1<<3; pub const ENVCFG_CBIE_SHIFT:usize=4; pub const ENVCFG_CBIE:usize=3<<4; pub const ENVCFG_CBIE_ILL:usize=0; pub const ENVCFG_CBIE_FLUSH:usize=1; pub const ENVCFG_CBIE_INV:usize=3; pub const ENVCFG_FIOM:usize=1;
pub const SMSTATEEN0_AIA_IMSIC_SHIFT:usize=58; pub const SMSTATEEN0_AIA_IMSIC:u64=1<<58; pub const SMSTATEEN0_AIA_SHIFT:usize=59; pub const SMSTATEEN0_AIA:u64=1<<59; pub const SMSTATEEN0_AIA_ISEL_SHIFT:usize=60; pub const SMSTATEEN0_AIA_ISEL:u64=1<<60; pub const SMSTATEEN0_HSENVCFG_SHIFT:usize=62; pub const SMSTATEEN0_HSENVCFG:u64=1<<62; pub const SMSTATEEN0_SSTATEEN0_SHIFT:usize=63; pub const SMSTATEEN0_SSTATEEN0:u64=1<<63;
pub const HPMEVENT_OF:u64=1<<63; pub const HPMEVENT_MINH:u64=1<<62; pub const HPMEVENT_SINH:u64=1<<61; pub const HPMEVENT_UINH:u64=1<<60; pub const HPMEVENT_VSINH:u64=1<<59; pub const HPMEVENT_VUINH:u64=1<<58; pub const SISELECT_SSCCFG_BASE:usize=0x40; pub const MSECCFG_PMM:u64=ENVCFG_PMM; pub const MSECCFG_PMM_PMLEN_0:u64=ENVCFG_PMM_PMLEN_0; pub const MSECCFG_PMM_PMLEN_7:u64=ENVCFG_PMM_PMLEN_7; pub const MSECCFG_PMM_PMLEN_16:u64=ENVCFG_PMM_PMLEN_16;
pub const CAUSE_IRQ_FLAG: u64 = 1u64 << (usize::BITS - 1);
pub const IRQ_S_SOFT: usize=1; pub const IRQ_VS_SOFT: usize=2; pub const IRQ_M_SOFT: usize=3; pub const IRQ_S_TIMER: usize=5; pub const IRQ_VS_TIMER: usize=6; pub const IRQ_M_TIMER: usize=7; pub const IRQ_S_EXT: usize=9; pub const IRQ_VS_EXT: usize=10; pub const IRQ_M_EXT: usize=11; pub const IRQ_S_GEXT: usize=12; pub const IRQ_PMU_OVF: usize=13; pub const IRQ_LOCAL_MAX: usize=IRQ_PMU_OVF+1; pub const IRQ_LOCAL_MASK: usize=(1<<IRQ_LOCAL_MAX)-1;
pub const EXC_INST_MISALIGNED: usize=0; pub const EXC_INST_ACCESS: usize=1; pub const EXC_INST_ILLEGAL: usize=2; pub const EXC_BREAKPOINT: usize=3; pub const EXC_LOAD_MISALIGNED: usize=4; pub const EXC_LOAD_ACCESS: usize=5; pub const EXC_STORE_MISALIGNED: usize=6; pub const EXC_STORE_ACCESS: usize=7; pub const EXC_SYSCALL: usize=8; pub const EXC_HYPERVISOR_SYSCALL: usize=9; pub const EXC_SUPERVISOR_SYSCALL: usize=10; pub const EXC_INST_PAGE_FAULT: usize=12; pub const EXC_LOAD_PAGE_FAULT: usize=13; pub const EXC_STORE_PAGE_FAULT: usize=15; pub const EXC_SOFTWARE_CHECK: usize=18; pub const EXC_INST_GUEST_PAGE_FAULT: usize=20; pub const EXC_LOAD_GUEST_PAGE_FAULT: usize=21; pub const EXC_VIRTUAL_INST_FAULT: usize=22; pub const EXC_STORE_GUEST_PAGE_FAULT: usize=23;
pub const PMP_R:u8=1; pub const PMP_W:u8=2; pub const PMP_X:u8=4; pub const PMP_A:u8=0x18; pub const PMP_A_TOR:u8=8; pub const PMP_A_NA4:u8=0x10; pub const PMP_A_NAPOT:u8=0x18; pub const PMP_L:u8=0x80;

macro_rules! csr_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: usize = $v;)* }; }
csr_consts! {
CSR_CYCLE=0xc00, CSR_TIME=0xc01, CSR_INSTRET=0xc02, CSR_SCOUNTOVF=0xda0,
CSR_SSTATUS=0x100, CSR_SIE=0x104, CSR_STVEC=0x105, CSR_SCOUNTEREN=0x106, CSR_SENVCFG=0x10a, CSR_SSTATEEN0=0x10c, CSR_SCOUNTINHIBIT=0x120, CSR_SSCRATCH=0x140, CSR_SEPC=0x141, CSR_SCAUSE=0x142, CSR_STVAL=0x143, CSR_SIP=0x144, CSR_SATP=0x180, CSR_SRMCFG=0x181, CSR_STIMECMP=0x14d, CSR_STIMECMPH=0x15d, CSR_SSP=0x011,
CSR_VXSAT=9, CSR_VXRM=0xa, CSR_SISELECT=0x150, CSR_SIREG=0x151, CSR_SIREG2=0x152, CSR_SIREG3=0x153, CSR_SIREG4=0x155, CSR_SIREG5=0x156, CSR_SIREG6=0x157, CSR_STOPEI=0x15c, CSR_STOPI=0xdb0, CSR_SIEH=0x114, CSR_SIPH=0x154,
CSR_VSSTATUS=0x200, CSR_VSIE=0x204, CSR_VSTVEC=0x205, CSR_VSSCRATCH=0x240, CSR_VSEPC=0x241, CSR_VSCAUSE=0x242, CSR_VSTVAL=0x243, CSR_VSIP=0x244, CSR_VSATP=0x280, CSR_VSTIMECMP=0x24d, CSR_VSTIMECMPH=0x25d,
CSR_HSTATUS=0x600, CSR_HEDELEG=0x602, CSR_HIDELEG=0x603, CSR_HIE=0x604, CSR_HTIMEDELTA=0x605, CSR_HCOUNTEREN=0x606, CSR_HGEIE=0x607, CSR_HENVCFG=0x60a, CSR_HTIMEDELTAH=0x615, CSR_HENVCFGH=0x61a, CSR_HTVAL=0x643, CSR_HIP=0x644, CSR_HVIP=0x645, CSR_HTINST=0x64a, CSR_HGATP=0x680, CSR_HGEIP=0xe12,
CSR_HVIEN=0x608, CSR_HVICTL=0x609, CSR_HVIPRIO1=0x646, CSR_HVIPRIO2=0x647, CSR_VSISELECT=0x250, CSR_VSIREG=0x251, CSR_VSIREG2=0x252, CSR_VSIREG3=0x253, CSR_VSIREG4=0x255, CSR_VSIREG5=0x256, CSR_VSIREG6=0x257, CSR_VSTOPEI=0x25c, CSR_VSTOPI=0xeb0, CSR_HIDELEGH=0x613, CSR_HVIENH=0x618, CSR_HVIPH=0x655, CSR_HVIPRIO1H=0x656, CSR_HVIPRIO2H=0x657, CSR_VSIEH=0x214, CSR_VSIPH=0x254, CSR_HSTATEEN0=0x60c, CSR_HSTATEEN0H=0x61c,
CSR_MSTATUS=0x300, CSR_MISA=0x301, CSR_MIDELEG=0x303, CSR_MIE=0x304, CSR_MTVEC=0x305, CSR_MENVCFG=0x30a, CSR_MENVCFGH=0x31a, CSR_MSCRATCH=0x340, CSR_MEPC=0x341, CSR_MCAUSE=0x342, CSR_MTVAL=0x343, CSR_MIP=0x344, CSR_PMPCFG0=0x3a0, CSR_PMPADDR0=0x3b0, CSR_MSECCFG=0x747, CSR_MSECCFGH=0x757, CSR_MVENDORID=0xf11, CSR_MARCHID=0xf12, CSR_MIMPID=0xf13, CSR_MHARTID=0xf14, CSR_MISELECT=0x350, CSR_MIREG=0x351, CSR_MIREG2=0x352, CSR_MIREG3=0x353, CSR_MIREG4=0x355, CSR_MIREG5=0x356, CSR_MIREG6=0x357, CSR_MTOPEI=0x35c, CSR_MTOPI=0xfb0, CSR_MVIEN=0x308, CSR_MVIP=0x309, CSR_MIDELEGH=0x313, CSR_MIEH=0x314, CSR_MVIENH=0x318, CSR_MVIPH=0x319, CSR_MIPH=0x354,
CSR_VSTART=8, CSR_VCSR=0xf, CSR_VL=0xc20, CSR_VTYPE=0xc21, CSR_VLENB=0xc22, CSR_SEED=0x015
}

pub const CSR_VXRM_MASK: usize=3; pub const CSR_VXRM_SHIFT: usize=1; pub const CSR_VXSAT_MASK: usize=1;
pub const VTYPE_VLMUL: usize=7; pub const VTYPE_VLMUL_FRAC: usize=4; pub const VTYPE_VSEW_SHIFT: usize=3; pub const VTYPE_VSEW: usize=7<<3; pub const VTYPE_VTA_SHIFT: usize=6; pub const VTYPE_VTA: usize=1<<6; pub const VTYPE_VMA_SHIFT: usize=7; pub const VTYPE_VMA: usize=1<<7; pub const VTYPE_VILL_SHIFT: usize=usize::BITS as usize-1; pub const VTYPE_VILL: usize=1usize<<(usize::BITS-1);
pub const VTYPE_VLMUL_THEAD: usize=3; pub const VTYPE_VSEW_THEAD_SHIFT: usize=2; pub const VTYPE_VSEW_THEAD: usize=7<<2; pub const VTYPE_VEDIV_THEAD_SHIFT: usize=5; pub const VTYPE_VEDIV_THEAD: usize=3<<5;
pub const SEED_OPST_MASK:u64=0xc0000000; pub const SEED_OPST_BIST:u64=0; pub const SEED_OPST_WAIT:u64=0x40000000; pub const SEED_OPST_ES16:u64=0x80000000; pub const SEED_OPST_DEAD:u64=0xc0000000; pub const SEED_ENTROPY_MASK:u64=0xffff;

#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const CSR_STATUS:usize=CSR_MSTATUS; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const CSR_STATUS:usize=CSR_SSTATUS;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const CSR_IE:usize=CSR_MIE; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const CSR_IE:usize=CSR_SIE;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const CSR_TVEC:usize=CSR_MTVEC; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const CSR_TVEC:usize=CSR_STVEC;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const CSR_EPC:usize=CSR_MEPC; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const CSR_EPC:usize=CSR_SEPC;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const CSR_CAUSE:usize=CSR_MCAUSE; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const CSR_CAUSE:usize=CSR_SCAUSE;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const CSR_TVAL:usize=CSR_MTVAL; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const CSR_TVAL:usize=CSR_STVAL;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const CSR_IP:usize=CSR_MIP; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const CSR_IP:usize=CSR_SIP;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const SR_IE:usize=SR_MIE; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const SR_IE:usize=SR_SIE;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const SR_PIE:usize=SR_MPIE; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const SR_PIE:usize=SR_SPIE;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const SR_PP:usize=SR_MPP; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const SR_PP:usize=SR_SPP;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const RV_IRQ_SOFT:usize=IRQ_M_SOFT; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const RV_IRQ_SOFT:usize=IRQ_S_SOFT;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const RV_IRQ_TIMER:usize=IRQ_M_TIMER; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const RV_IRQ_TIMER:usize=IRQ_S_TIMER;
#[cfg(feature = "CONFIG_RISCV_M_MODE")] pub const RV_IRQ_EXT:usize=IRQ_M_EXT; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const RV_IRQ_EXT:usize=IRQ_S_EXT;
#[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const RV_IRQ_PMU:usize=IRQ_PMU_OVF; #[cfg(not(feature = "CONFIG_RISCV_M_MODE"))] pub const SIP_LCOFIP:usize=1<<IRQ_PMU_OVF;
pub const IE_SIE:usize=1<<RV_IRQ_SOFT; pub const IE_TIE:usize=1<<RV_IRQ_TIMER; pub const IE_EIE:usize=1<<RV_IRQ_EXT;

// The C header's CSR inline-assembly macros are represented as exported Rust
// macros; the CSR operand is intentionally kept as an assembly token.
#[macro_export] macro_rules! csr_read { ($csr:tt) => {{ let __v: usize; unsafe { core::arch::asm!(concat!("csrr {0}, ", stringify!($csr)), out(reg) __v, options(nostack, preserves_flags)); } __v }}; }
#[macro_export] macro_rules! csr_write { ($csr:tt, $val:expr) => {{ let __v: usize = $val as usize; unsafe { core::arch::asm!(concat!("csrw ", stringify!($csr), ", {0}"), in(reg) __v, options(nostack, preserves_flags)); } }}; }
#[macro_export] macro_rules! csr_swap { ($csr:tt, $val:expr) => {{ let mut __v: usize = $val as usize; unsafe { core::arch::asm!(concat!("csrrw {0}, ", stringify!($csr), ", {0}"), inout(reg) __v, options(nostack, preserves_flags)); } __v }}; }
#[macro_export] macro_rules! csr_read_set { ($csr:tt, $val:expr) => {{ let mut __v: usize = $val as usize; unsafe { core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr), ", {0}"), inout(reg) __v, options(nostack, preserves_flags)); } __v }}; }
#[macro_export] macro_rules! csr_set { ($csr:tt, $val:expr) => {{ let __v: usize = $val as usize; unsafe { core::arch::asm!(concat!("csrs ", stringify!($csr), ", {0}"), in(reg) __v, options(nostack, preserves_flags)); } }}; }
#[macro_export] macro_rules! csr_read_clear { ($csr:tt, $val:expr) => {{ let mut __v: usize = $val as usize; unsafe { core::arch::asm!(concat!("csrrc {0}, ", stringify!($csr), ", {0}"), inout(reg) __v, options(nostack, preserves_flags)); } __v }}; }
#[macro_export] macro_rules! csr_clear { ($csr:tt, $val:expr) => {{ let __v: usize = $val as usize; unsafe { core::arch::asm!(concat!("csrc ", stringify!($csr), ", {0}"), in(reg) __v, options(nostack, preserves_flags)); } }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
