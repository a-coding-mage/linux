/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright IBM Corp. 2007
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

/* Depends on Linux UAPI integer aliases from <linux/types.h>. */
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

/* Select powerpc specific features in <linux/kvm.h> */
pub const __KVM_HAVE_SPAPR_TCE: bool = true;
pub const __KVM_HAVE_PPC_SMT: bool = true;
pub const __KVM_HAVE_IRQCHIP: bool = true;
pub const __KVM_HAVE_IRQ_LINE: bool = true;

/* Not always available, but if it is, this is the correct offset.  */
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
    pub pc: __u64,
    pub cr: __u64,
    pub ctr: __u64,
    pub lr: __u64,
    pub xer: __u64,
    pub msr: __u64,
    pub srr0: __u64,
    pub srr1: __u64,
    pub pid: __u64,
    pub sprg0: __u64,
    pub sprg1: __u64,
    pub sprg2: __u64,
    pub sprg3: __u64,
    pub sprg4: __u64,
    pub sprg5: __u64,
    pub sprg6: __u64,
    pub sprg7: __u64,
    pub gpr: [__u64; 32],
}

pub const KVM_SREGS_E_IMPL_NONE: u32 = 0;
pub const KVM_SREGS_E_IMPL_FSL: u32 = 1;

pub const KVM_SREGS_E_FSL_PIDn: u32 = 1 << 0; /* PID1/PID2 */

/* flags for kvm_run.flags */
pub const KVM_RUN_PPC_NMI_DISP_MASK: u32 = 3 << 0;
pub const KVM_RUN_PPC_NMI_DISP_FULLY_RECOV: u32 = 1 << 0;
pub const KVM_RUN_PPC_NMI_DISP_LIMITED_RECOV: u32 = 2 << 0;
pub const KVM_RUN_PPC_NMI_DISP_NOT_RECOV: u32 = 3 << 0;

/*
 * Feature bits indicate which sections of the sregs struct are valid,
 * both in KVM_GET_SREGS and KVM_SET_SREGS.  On KVM_SET_SREGS, registers
 * corresponding to unset feature bits will not be modified.  This allows
 * restoring a checkpoint made without that feature, while keeping the
 * default values of the new registers.
 *
 * KVM_SREGS_E_BASE contains:
 * CSRR0/1 (refers to SRR2/3 on 40x)
 * ESR
 * DEAR
 * MCSR
 * TSR
 * TCR
 * DEC
 * TB
 * VRSAVE (USPRG0)
 */
pub const KVM_SREGS_E_BASE: u32 = 1 << 0;

/*
 * KVM_SREGS_E_ARCH206 contains:
 *
 * PIR
 * MCSRR0/1
 * DECAR
 * IVPR
 */
pub const KVM_SREGS_E_ARCH206: u32 = 1 << 1;

/*
 * Contains EPCR, plus the upper half of 64-bit registers
 * that are 32-bit on 32-bit implementations.
 */
pub const KVM_SREGS_E_64: u32 = 1 << 2;

pub const KVM_SREGS_E_SPRG8: u32 = 1 << 3;
pub const KVM_SREGS_E_MCIVPR: u32 = 1 << 4;

/*
 * IVORs are used -- contains IVOR0-15, plus additional IVORs
 * in combination with an appropriate feature bit.
 */
pub const KVM_SREGS_E_IVOR: u32 = 1 << 5;

/*
 * Contains MAS0-4, MAS6-7, TLBnCFG, MMUCFG.
 * Also TLBnPS if MMUCFG[MAVN] = 1.
 */
pub const KVM_SREGS_E_ARCH206_MMU: u32 = 1 << 6;

/* DBSR, DBCR, IAC, DAC, DVC */
pub const KVM_SREGS_E_DEBUG: u32 = 1 << 7;

/* Enhanced debug -- DSRR0/1, SPRG9 */
pub const KVM_SREGS_E_ED: u32 = 1 << 8;

/* Embedded Floating Point (SPE) -- IVOR32-34 if KVM_SREGS_E_IVOR */
pub const KVM_SREGS_E_SPE: u32 = 1 << 9;

/*
 * DEPRECATED! USE ONE_REG FOR THIS ONE!
 * External Proxy (EXP) -- EPR
 */
pub const KVM_SREGS_EXP: u32 = 1 << 10;

/* External PID (E.PD) -- EPSC/EPLC */
pub const KVM_SREGS_E_PD: u32 = 1 << 11;

/* Processor Control (E.PC) -- IVOR36-37 if KVM_SREGS_E_IVOR */
pub const KVM_SREGS_E_PC: u32 = 1 << 12;

/* Page table (E.PT) -- EPTCFG */
pub const KVM_SREGS_E_PT: u32 = 1 << 13;

/* Embedded Performance Monitor (E.PM) -- IVOR35 if KVM_SREGS_E_IVOR */
pub const KVM_SREGS_E_PM: u32 = 1 << 14;

/*
 * Special updates:
 *
 * Some registers may change even while a vcpu is not running.
 * To avoid losing these changes, by default these registers are
 * not updated by KVM_SET_SREGS.  To force an update, set the bit
 * in u.e.update_special corresponding to the register to be updated.
 *
 * The update_special field is zero on return from KVM_GET_SREGS.
 *
 * When restoring a checkpoint, the caller can set update_special
 * to 0xffffffff to ensure that everything is restored, even new features
 * that the caller doesn't know about.
 */
pub const KVM_SREGS_E_UPDATE_MCSR: u32 = 1 << 0;
pub const KVM_SREGS_E_UPDATE_TSR: u32 = 1 << 1;
pub const KVM_SREGS_E_UPDATE_DEC: u32 = 1 << 2;
pub const KVM_SREGS_E_UPDATE_DBSR: u32 = 1 << 3;

/*
 * In KVM_SET_SREGS, reserved/pad fields must be left untouched from a
 * previous KVM_GET_REGS.
 *
 * Unless otherwise indicated, setting any register with KVM_SET_SREGS
 * directly sets its value.  It does not trigger any special semantics such
 * as write-one-to-clear.  Calling KVM_SET_SREGS on an unmodified struct
 * just received from KVM_GET_SREGS is always a no-op.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
    pub pvr: __u32,
    pub u: kvm_sregs_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_sregs_u {
    pub s: kvm_sregs_s,
    pub e: kvm_sregs_e,
    pub pad: [__u8; 1020],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs_s {
    pub sdr1: __u64,
    pub ppc64: kvm_sregs_ppc64,
    pub ppc32: kvm_sregs_ppc32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs_ppc64 {
    pub slb: [kvm_sregs_slb; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs_slb {
    pub slbe: __u64,
    pub slbv: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs_ppc32 {
    pub sr: [__u32; 16],
    pub ibat: [__u64; 8],
    pub dbat: [__u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs_e {
    pub impl_: kvm_sregs_impl,
    pub features: __u32,
    pub impl_id: __u32,
    pub update_special: __u32,
    pub pir: __u32,
    pub sprg8: __u64,
    pub sprg9: __u64,
    pub csrr0: __u64,
    pub dsrr0: __u64,
    pub mcsrr0: __u64,
    pub csrr1: __u32,
    pub dsrr1: __u32,
    pub mcsrr1: __u32,
    pub esr: __u32,
    pub dear: __u64,
    pub ivpr: __u64,
    pub mcivpr: __u64,
    pub mcsr: __u64,
    pub tsr: __u32,
    pub tcr: __u32,
    pub decar: __u32,
    pub dec: __u32,
    /*
     * Userspace can read TB directly, but the
     * value reported here is consistent with "dec".
     *
     * Read-only.
     */
    pub tb: __u64,
    pub dbsr: __u32,
    pub dbcr: [__u32; 3],
    /*
     * iac/dac registers are 64bit wide, while this API
     * interface provides only lower 32 bits on 64 bit
     * processors. ONE_REG interface is added for 64bit
     * iac/dac registers.
     */
    pub iac: [__u32; 4],
    pub dac: [__u32; 2],
    pub dvc: [__u32; 2],
    pub num_iac: __u8,
    pub num_dac: __u8,
    pub num_dvc: __u8,
    pub pad: __u8,
    pub epr: __u32,
    pub vrsave: __u32,
    pub epcr: __u32,
    pub mas0: __u32,
    pub mas1: __u32,
    pub mas2: __u64,
    pub mas7_3: __u64,
    pub mas4: __u32,
    pub mas6: __u32,
    pub ivor_low: [__u32; 16],
    pub ivor_high: [__u32; 18],
    pub mmucfg: __u32,
    pub eptcfg: __u32,
    pub tlbcfg: [__u32; 4],
    pub tlbps: [__u32; 4],
    pub eplc: __u32,
    pub epsc: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_sregs_impl {
    pub fsl: kvm_sregs_impl_fsl,
    pub pad: [__u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs_impl_fsl {
    pub features: __u32,
    pub svr: __u32,
    pub mcar: __u64,
    pub hid0: __u32,
    pub pid1: __u32,
    pub pid2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
    pub fpr: [__u64; 32],
}

/*
 * Defines for h/w breakpoint, watchpoint (read, write or both) and
 * software breakpoint.
 * These are used as "type" in KVM_SET_GUEST_DEBUG ioctl and "status"
 * for KVM_DEBUG_EXIT.
 */
pub const KVMPPC_DEBUG_NONE: u64 = 0x0;
pub const KVMPPC_DEBUG_BREAKPOINT: u64 = 1u64 << 1;
pub const KVMPPC_DEBUG_WATCH_WRITE: u64 = 1u64 << 2;
pub const KVMPPC_DEBUG_WATCH_READ: u64 = 1u64 << 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
    pub address: __u64,
    /*
     * exiting to userspace because of h/w breakpoint, watchpoint
     * (read, write or both) and software breakpoint.
     */
    pub status: __u32,
    pub reserved: __u32,
}

/* for KVM_SET_GUEST_DEBUG */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
    pub bp: [kvm_guest_debug_arch_bp; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch_bp {
    /* H/W breakpoint/watchpoint address */
    pub addr: __u64,
    /*
     * Type denotes h/w breakpoint, read watchpoint, write
     * watchpoint or watchpoint (both read and write).
     */
    pub type_: __u32,
    pub reserved: __u32,
}

/* Debug related defines */
/*
 * kvm_guest_debug->control is a 32 bit field. The lower 16 bits are generic
 * and upper 16 bits are architecture specific. Architecture specific defines
 * that ioctl is for setting hardware breakpoint or software breakpoint.
 */
pub const KVM_GUESTDBG_USE_SW_BP: u32 = 0x00010000;
pub const KVM_GUESTDBG_USE_HW_BP: u32 = 0x00020000;

/* definition of registers in kvm_run */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {}

pub const KVM_INTERRUPT_SET: u32 = -1i32 as u32;
pub const KVM_INTERRUPT_UNSET: u32 = -2i32 as u32;
pub const KVM_INTERRUPT_SET_LEVEL: u32 = -3i32 as u32;

pub const KVM_CPU_440: u32 = 1;
pub const KVM_CPU_E500V2: u32 = 2;
pub const KVM_CPU_3S_32: u32 = 3;
pub const KVM_CPU_3S_64: u32 = 4;
pub const KVM_CPU_E500MC: u32 = 5;

/* for KVM_CAP_SPAPR_TCE */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_create_spapr_tce {
    pub liobn: __u64,
    pub window_size: __u32,
}

/* for KVM_CAP_SPAPR_TCE_64 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_create_spapr_tce_64 {
    pub liobn: __u64,
    pub page_shift: __u32,
    pub flags: __u32,
    pub offset: __u64,
    pub size: __u64,
}

/* for KVM_ALLOCATE_RMA */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_allocate_rma {
    pub rma_size: __u64,
}

/* for KVM_CAP_PPC_RTAS */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_rtas_token_args {
    pub name: [::std::os::raw::c_char; 120],
    pub token: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_book3e_206_tlb_entry {
    pub mas8: __u32,
    pub mas1: __u32,
    pub mas2: __u64,
    pub mas7_3: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_book3e_206_tlb_params {
    /*
     * For mmu types KVM_MMU_FSL_BOOKE_NOHV and KVM_MMU_FSL_BOOKE_HV:
     *
     * - The number of ways of TLB0 must be a power of two between 2 and
     *   16.
     * - TLB1 must be fully associative.
     * - The size of TLB0 must be a multiple of the number of ways, and
     *   the number of sets must be a power of two.
     * - The size of TLB1 may not exceed 64 entries.
     * - TLB0 supports 4 KiB pages.
     * - The page sizes supported by TLB1 are as indicated by
     *   TLB1CFG (if MMUCFG[MAVN] = 0) or TLB1PS (if MMUCFG[MAVN] = 1)
     *   as returned by KVM_GET_SREGS.
     * - TLB2 and TLB3 are reserved, and their entries in tlb_sizes[]
     *   and tlb_ways[] must be zero.
     *
     * tlb_ways[n] = tlb_sizes[n] means the array is fully associative.
     *
     * KVM will adjust TLBnCFG based on the sizes configured here,
     * though arrays greater than 2048 entries will have TLBnCFG[NENTRY]
     * set to zero.
     */
    pub tlb_sizes: [__u32; 4],
    pub tlb_ways: [__u32; 4],
    pub reserved: [__u32; 8],
}

/* For KVM_PPC_GET_HTAB_FD */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_get_htab_fd {
    pub flags: __u64,
    pub start_index: __u64,
    pub reserved: [__u64; 2],
}

/* Values for kvm_get_htab_fd.flags */
pub const KVM_GET_HTAB_BOLTED_ONLY: __u64 = 0x1;
pub const KVM_GET_HTAB_WRITE: __u64 = 0x2;

/*
 * Data read on the file descriptor is formatted as a series of
 * records, each consisting of a header followed by a series of
 * `n_valid' HPTEs (16 bytes each), which are all valid.  Following
 * those valid HPTEs there are `n_invalid' invalid HPTEs, which
 * are not represented explicitly in the stream.  The same format
 * is used for writing.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_get_htab_header {
    pub index: __u32,
    pub n_valid: __u16,
    pub n_invalid: __u16,
}

/* For KVM_PPC_CONFIGURE_V3_MMU */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_mmuv3_cfg {
    pub flags: __u64,
    pub process_table: __u64,
}

/* Flag values for KVM_PPC_CONFIGURE_V3_MMU */
pub const KVM_PPC_MMUV3_RADIX: u32 = 1;
pub const KVM_PPC_MMUV3_GTSE: u32 = 2;

/* For KVM_PPC_GET_RMMU_INFO */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_rmmu_info {
    pub geometries: [kvm_ppc_radix_geom; 8],
    pub ap_encodings: [__u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_radix_geom {
    pub page_shift: __u8,
    pub level_bits: [__u8; 4],
    pub pad: [__u8; 3],
}

/* For KVM_PPC_GET_CPU_CHAR */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_cpu_char {
    pub character: __u64,
    pub behaviour: __u64,
    pub character_mask: __u64,
    pub behaviour_mask: __u64,
}

/*
 * Values for character and character_mask.
 * These are identical to the values used by H_GET_CPU_CHARACTERISTICS.
 */
pub const KVM_PPC_CPU_CHAR_SPEC_BAR_ORI31: u64 = 1u64 << 63;
pub const KVM_PPC_CPU_CHAR_BCCTRL_SERIALISED: u64 = 1u64 << 62;
pub const KVM_PPC_CPU_CHAR_L1D_FLUSH_ORI30: u64 = 1u64 << 61;
pub const KVM_PPC_CPU_CHAR_L1D_FLUSH_TRIG2: u64 = 1u64 << 60;
pub const KVM_PPC_CPU_CHAR_L1D_THREAD_PRIV: u64 = 1u64 << 59;
pub const KVM_PPC_CPU_CHAR_BR_HINT_HONOURED: u64 = 1u64 << 58;
pub const KVM_PPC_CPU_CHAR_MTTRIG_THR_RECONF: u64 = 1u64 << 57;
pub const KVM_PPC_CPU_CHAR_COUNT_CACHE_DIS: u64 = 1u64 << 56;
pub const KVM_PPC_CPU_CHAR_BCCTR_FLUSH_ASSIST: u64 = 1u64 << 54;

pub const KVM_PPC_CPU_BEHAV_FAVOUR_SECURITY: u64 = 1u64 << 63;
pub const KVM_PPC_CPU_BEHAV_L1D_FLUSH_PR: u64 = 1u64 << 62;
pub const KVM_PPC_CPU_BEHAV_BNDS_CHK_SPEC_BAR: u64 = 1u64 << 61;
pub const KVM_PPC_CPU_BEHAV_FLUSH_COUNT_CACHE: u64 = 1u64 << 58;

/* The following one-reg constants depend on KVM_REG_PPC and KVM_REG_SIZE_* from <linux/kvm.h>. */
pub const KVM_REG_PPC_ICP_STATE: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x8c;

pub const KVM_REG_PPC_ICP_CPPR_SHIFT: u32 = 56;
pub const KVM_REG_PPC_ICP_CPPR_MASK: u32 = 0xff;
pub const KVM_REG_PPC_ICP_XISR_SHIFT: u32 = 32;
pub const KVM_REG_PPC_ICP_XISR_MASK: u32 = 0xffffff;
pub const KVM_REG_PPC_ICP_MFRR_SHIFT: u32 = 24;
pub const KVM_REG_PPC_ICP_MFRR_MASK: u32 = 0xff;
pub const KVM_REG_PPC_ICP_PPRI_SHIFT: u32 = 16;
pub const KVM_REG_PPC_ICP_PPRI_MASK: u32 = 0xff;

pub const KVM_REG_PPC_VP_STATE: u64 = KVM_REG_PPC | KVM_REG_SIZE_U128 | 0x8d;

/* Device control API: PPC-specific devices */
pub const KVM_DEV_MPIC_GRP_MISC: u32 = 1;
pub const KVM_DEV_MPIC_BASE_ADDR: u32 = 0;

pub const KVM_DEV_MPIC_GRP_REGISTER: u32 = 2;
pub const KVM_DEV_MPIC_GRP_IRQ_ACTIVE: u32 = 3;

/* One-Reg API: PPC-specific registers */
pub const KVM_REG_PPC_HIOR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x1;
pub const KVM_REG_PPC_IAC1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x2;
pub const KVM_REG_PPC_IAC2: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x3;
pub const KVM_REG_PPC_IAC3: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x4;
pub const KVM_REG_PPC_IAC4: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x5;
pub const KVM_REG_PPC_DAC1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x6;
pub const KVM_REG_PPC_DAC2: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x7;
pub const KVM_REG_PPC_DABR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x8;
pub const KVM_REG_PPC_DSCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x9;
pub const KVM_REG_PPC_PURR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa;
pub const KVM_REG_PPC_SPURR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb;
pub const KVM_REG_PPC_DAR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc;
pub const KVM_REG_PPC_DSISR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xd;
pub const KVM_REG_PPC_AMR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xe;
pub const KVM_REG_PPC_UAMOR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xf;

pub const KVM_REG_PPC_MMCR0: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x10;
pub const KVM_REG_PPC_MMCR1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x11;
pub const KVM_REG_PPC_MMCRA: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x12;
pub const KVM_REG_PPC_MMCR2: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x13;
pub const KVM_REG_PPC_MMCRS: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x14;
pub const KVM_REG_PPC_SIAR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x15;
pub const KVM_REG_PPC_SDAR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x16;
pub const KVM_REG_PPC_SIER: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x17;

pub const KVM_REG_PPC_PMC1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x18;
pub const KVM_REG_PPC_PMC2: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x19;
pub const KVM_REG_PPC_PMC3: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x1a;
pub const KVM_REG_PPC_PMC4: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x1b;
pub const KVM_REG_PPC_PMC5: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x1c;
pub const KVM_REG_PPC_PMC6: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x1d;
pub const KVM_REG_PPC_PMC7: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x1e;
pub const KVM_REG_PPC_PMC8: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x1f;

/* 32 floating-point registers */
pub const KVM_REG_PPC_FPR0: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x20;
pub const fn KVM_REG_PPC_FPR(n: u64) -> u64 { KVM_REG_PPC_FPR0 + n }
pub const KVM_REG_PPC_FPR31: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x3f;

/* 32 VMX/Altivec vector registers */
pub const KVM_REG_PPC_VR0: u64 = KVM_REG_PPC | KVM_REG_SIZE_U128 | 0x40;
pub const fn KVM_REG_PPC_VR(n: u64) -> u64 { KVM_REG_PPC_VR0 + n }
pub const KVM_REG_PPC_VR31: u64 = KVM_REG_PPC | KVM_REG_SIZE_U128 | 0x5f;

/* 32 double-width FP registers for VSX */
/* High-order halves overlap with FP regs */
pub const KVM_REG_PPC_VSR0: u64 = KVM_REG_PPC | KVM_REG_SIZE_U128 | 0x60;
pub const fn KVM_REG_PPC_VSR(n: u64) -> u64 { KVM_REG_PPC_VSR0 + n }
pub const KVM_REG_PPC_VSR31: u64 = KVM_REG_PPC | KVM_REG_SIZE_U128 | 0x7f;

/* FP and vector status/control registers */
pub const KVM_REG_PPC_FPSCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x80;
/*
 * VSCR register is documented as a 32-bit register in the ISA, but it can
 * only be accesses via a vector register. Expose VSCR as a 32-bit register
 * even though the kernel represents it as a 128-bit vector.
 */
pub const KVM_REG_PPC_VSCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x81;

/* Virtual processor areas */
/* For SLB & DTL, address in high (first) half, length in low half */
pub const KVM_REG_PPC_VPA_ADDR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x82;
pub const KVM_REG_PPC_VPA_SLB: u64 = KVM_REG_PPC | KVM_REG_SIZE_U128 | 0x83;
pub const KVM_REG_PPC_VPA_DTL: u64 = KVM_REG_PPC | KVM_REG_SIZE_U128 | 0x84;

pub const KVM_REG_PPC_EPCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x85;
pub const KVM_REG_PPC_EPR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x86;

/* Timer Status Register OR/CLEAR interface */
pub const KVM_REG_PPC_OR_TSR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x87;
pub const KVM_REG_PPC_CLEAR_TSR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x88;
pub const KVM_REG_PPC_TCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x89;
pub const KVM_REG_PPC_TSR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x8a;

/* Debugging: Special instruction for software breakpoint */
pub const KVM_REG_PPC_DEBUG_INST: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x8b;

/* MMU registers */
pub const KVM_REG_PPC_MAS0: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x8c;
pub const KVM_REG_PPC_MAS1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x8d;
pub const KVM_REG_PPC_MAS2: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x8e;
pub const KVM_REG_PPC_MAS7_3: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x8f;
pub const KVM_REG_PPC_MAS4: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x90;
pub const KVM_REG_PPC_MAS6: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x91;
pub const KVM_REG_PPC_MMUCFG: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x92;
/*
 * TLBnCFG fields TLBnCFG_N_ENTRY and TLBnCFG_ASSOC can be changed only using
 * KVM_CAP_SW_TLB ioctl
 */
pub const KVM_REG_PPC_TLB0CFG: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x93;
pub const KVM_REG_PPC_TLB1CFG: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x94;
pub const KVM_REG_PPC_TLB2CFG: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x95;
pub const KVM_REG_PPC_TLB3CFG: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x96;
pub const KVM_REG_PPC_TLB0PS: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x97;
pub const KVM_REG_PPC_TLB1PS: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x98;
pub const KVM_REG_PPC_TLB2PS: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x99;
pub const KVM_REG_PPC_TLB3PS: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x9a;
pub const KVM_REG_PPC_EPTCFG: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x9b;

/* Timebase offset */
pub const KVM_REG_PPC_TB_OFFSET: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x9c;

/* POWER8 registers */
pub const KVM_REG_PPC_SPMC1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x9d;
pub const KVM_REG_PPC_SPMC2: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0x9e;
pub const KVM_REG_PPC_IAMR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0x9f;
pub const KVM_REG_PPC_TFHAR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa0;
pub const KVM_REG_PPC_TFIAR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa1;
pub const KVM_REG_PPC_TEXASR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa2;
pub const KVM_REG_PPC_FSCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa3;
pub const KVM_REG_PPC_PSPB: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xa4;
pub const KVM_REG_PPC_EBBHR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa5;
pub const KVM_REG_PPC_EBBRR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa6;
pub const KVM_REG_PPC_BESCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa7;
pub const KVM_REG_PPC_TAR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa8;
pub const KVM_REG_PPC_DPDES: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xa9;
pub const KVM_REG_PPC_DAWR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xaa;
pub const KVM_REG_PPC_DAWRX: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xab;
pub const KVM_REG_PPC_CIABR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xac;
pub const KVM_REG_PPC_IC: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xad;
pub const KVM_REG_PPC_VTB: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xae;
pub const KVM_REG_PPC_CSIGR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xaf;
pub const KVM_REG_PPC_TACR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb0;
pub const KVM_REG_PPC_TCSCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb1;
pub const KVM_REG_PPC_PID: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb2;
pub const KVM_REG_PPC_ACOP: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb3;

pub const KVM_REG_PPC_VRSAVE: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xb4;
pub const KVM_REG_PPC_LPCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xb5;
pub const KVM_REG_PPC_LPCR_64: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb5;
pub const KVM_REG_PPC_PPR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb6;

/* Architecture compatibility level */
pub const KVM_REG_PPC_ARCH_COMPAT: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xb7;

pub const KVM_REG_PPC_DABRX: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xb8;
pub const KVM_REG_PPC_WORT: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xb9;
pub const KVM_REG_PPC_SPRG9: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xba;
pub const KVM_REG_PPC_DBSR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xbb;

/* POWER9 registers */
pub const KVM_REG_PPC_TIDR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xbc;
pub const KVM_REG_PPC_PSSCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xbd;

pub const KVM_REG_PPC_DEC_EXPIRY: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xbe;
pub const KVM_REG_PPC_ONLINE: u64 = KVM_REG_PPC | KVM_REG_SIZE_U32 | 0xbf;
pub const KVM_REG_PPC_PTCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc0;

/* POWER10 registers */
pub const KVM_REG_PPC_MMCR3: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc1;
pub const KVM_REG_PPC_SIER2: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc2;
pub const KVM_REG_PPC_SIER3: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc3;
pub const KVM_REG_PPC_DAWR1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc4;
pub const KVM_REG_PPC_DAWRX1: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc5;
pub const KVM_REG_PPC_DEXCR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc6;
pub const KVM_REG_PPC_HASHKEYR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc7;
pub const KVM_REG_PPC_HASHPKEYR: u64 = KVM_REG_PPC | KVM_REG_SIZE_U64 | 0xc8;

/* Transactional Memory checkpointed state:
 * This is all GPRs, all VSX regs and a subset of SPRs
 */
pub const KVM_REG_PPC_TM: u64 = KVM_REG_PPC | 0x80000000;
/* TM GPRs */
pub const KVM_REG_PPC_TM_GPR0: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0;
pub const fn KVM_REG_PPC_TM_GPR(n: u64) -> u64 { KVM_REG_PPC_TM_GPR0 + n }
pub const KVM_REG_PPC_TM_GPR31: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x1f;
/* TM VSX */
pub const KVM_REG_PPC_TM_VSR0: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U128 | 0x20;
pub const fn KVM_REG_PPC_TM_VSR(n: u64) -> u64 { KVM_REG_PPC_TM_VSR0 + n }
pub const KVM_REG_PPC_TM_VSR63: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U128 | 0x5f;
/* TM SPRS */
pub const KVM_REG_PPC_TM_CR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x60;
pub const KVM_REG_PPC_TM_LR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x61;
pub const KVM_REG_PPC_TM_CTR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x62;
pub const KVM_REG_PPC_TM_FPSCR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x63;
pub const KVM_REG_PPC_TM_AMR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x64;
pub const KVM_REG_PPC_TM_PPR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x65;
pub const KVM_REG_PPC_TM_VRSAVE: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x66;
pub const KVM_REG_PPC_TM_VSCR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U32 | 0x67;
pub const KVM_REG_PPC_TM_DSCR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x68;
pub const KVM_REG_PPC_TM_TAR: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x69;
pub const KVM_REG_PPC_TM_XER: u64 = KVM_REG_PPC_TM | KVM_REG_SIZE_U64 | 0x6a;

/* PPC64 eXternal Interrupt Controller Specification */
pub const KVM_DEV_XICS_GRP_SOURCES: u32 = 1;
pub const KVM_DEV_XICS_GRP_CTRL: u32 = 2;
pub const KVM_DEV_XICS_NR_SERVERS: u32 = 1;

/* Layout of 64-bit source attribute values */
pub const KVM_XICS_DESTINATION_SHIFT: u32 = 0;
pub const KVM_XICS_DESTINATION_MASK: u64 = 0xffffffff;
pub const KVM_XICS_PRIORITY_SHIFT: u32 = 32;
pub const KVM_XICS_PRIORITY_MASK: u32 = 0xff;
pub const KVM_XICS_LEVEL_SENSITIVE: u64 = 1u64 << 40;
pub const KVM_XICS_MASKED: u64 = 1u64 << 41;
pub const KVM_XICS_PENDING: u64 = 1u64 << 42;
pub const KVM_XICS_PRESENTED: u64 = 1u64 << 43;
pub const KVM_XICS_QUEUED: u64 = 1u64 << 44;

/* POWER9 XIVE Native Interrupt Controller */
pub const KVM_DEV_XIVE_GRP_CTRL: u32 = 1;
pub const KVM_DEV_XIVE_RESET: u32 = 1;
pub const KVM_DEV_XIVE_EQ_SYNC: u32 = 2;
pub const KVM_DEV_XIVE_NR_SERVERS: u32 = 3;
pub const KVM_DEV_XIVE_GRP_SOURCE: u32 = 2;
pub const KVM_DEV_XIVE_GRP_SOURCE_CONFIG: u32 = 3;
pub const KVM_DEV_XIVE_GRP_EQ_CONFIG: u32 = 4;
pub const KVM_DEV_XIVE_GRP_SOURCE_SYNC: u32 = 5;

/* Layout of 64-bit XIVE source attribute values */
pub const KVM_XIVE_LEVEL_SENSITIVE: u64 = 1u64 << 0;
pub const KVM_XIVE_LEVEL_ASSERTED: u64 = 1u64 << 1;

/* Layout of 64-bit XIVE source configuration attribute values */
pub const KVM_XIVE_SOURCE_PRIORITY_SHIFT: u32 = 0;
pub const KVM_XIVE_SOURCE_PRIORITY_MASK: u32 = 0x7;
pub const KVM_XIVE_SOURCE_SERVER_SHIFT: u32 = 3;
pub const KVM_XIVE_SOURCE_SERVER_MASK: u64 = 0xfffffff8;
pub const KVM_XIVE_SOURCE_MASKED_SHIFT: u32 = 32;
pub const KVM_XIVE_SOURCE_MASKED_MASK: u64 = 0x100000000;
pub const KVM_XIVE_SOURCE_EISN_SHIFT: u32 = 33;
pub const KVM_XIVE_SOURCE_EISN_MASK: u64 = 0xfffffffe00000000;

/* Layout of 64-bit EQ identifier */
pub const KVM_XIVE_EQ_PRIORITY_SHIFT: u32 = 0;
pub const KVM_XIVE_EQ_PRIORITY_MASK: u32 = 0x7;
pub const KVM_XIVE_EQ_SERVER_SHIFT: u32 = 3;
pub const KVM_XIVE_EQ_SERVER_MASK: u64 = 0xfffffff8;

/* Layout of EQ configuration values (64 bytes) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_xive_eq {
    pub flags: __u32,
    pub qshift: __u32,
    pub qaddr: __u64,
    pub qtoggle: __u32,
    pub qindex: __u32,
    pub pad: [__u8; 40],
}

pub const KVM_XIVE_EQ_ALWAYS_NOTIFY: u32 = 0x00000001;

pub const KVM_XIVE_TIMA_PAGE_OFFSET: u32 = 0;
pub const KVM_XIVE_ESB_PAGE_OFFSET: u32 = 4;

/* for KVM_PPC_GET_PVINFO */

pub const KVM_PPC_PVINFO_FLAGS_EV_IDLE: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_pvinfo {
    /* out */
    pub flags: __u32,
    pub hcall: [__u32; 4],
    pub pad: [__u8; 108],
}

/* for KVM_PPC_GET_SMMU_INFO */
pub const KVM_PPC_PAGE_SIZES_MAX_SZ: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_one_page_size {
    pub page_shift: __u32,
    pub pte_enc: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_one_seg_page_size {
    pub page_shift: __u32,
    pub slb_enc: __u32,
    pub enc: [kvm_ppc_one_page_size; KVM_PPC_PAGE_SIZES_MAX_SZ],
}

pub const KVM_PPC_PAGE_SIZES_REAL: u32 = 0x00000001;
pub const KVM_PPC_1T_SEGMENTS: u32 = 0x00000002;
pub const KVM_PPC_NO_HASH: u32 = 0x00000004;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_smmu_info {
    pub flags: __u64,
    pub slb_size: __u32,
    pub data_keys: __u16,
    pub instr_keys: __u16,
    pub sps: [kvm_ppc_one_seg_page_size; KVM_PPC_PAGE_SIZES_MAX_SZ],
}

/* for KVM_PPC_RESIZE_HPT_{PREPARE,COMMIT} */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_resize_hpt {
    pub flags: __u64,
    pub shift: __u32,
    pub pad: __u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
