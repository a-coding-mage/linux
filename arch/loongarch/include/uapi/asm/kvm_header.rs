/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// #include <linux/types.h>

pub const __KVM_HAVE_IRQ_LINE: bool = true;

/*
 * KVM LoongArch specific structures and definitions.
 *
 * Some parts derived from the x86 version of this file.
 */

pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u64 = 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: u64 = 64;

pub const KVM_GUESTDBG_USE_SW_BP: u64 = 0x0001_0000;

/*
 * for KVM_GET_REGS and KVM_SET_REGS
 */
#[repr(C)]
pub struct kvm_regs {
    /* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
    pub gpr: [u64; 32],
    pub pc: u64,
}

/*
 * for KVM_GET_FPU and KVM_SET_FPU
 */
#[repr(C)]
pub struct kvm_fpureg {
    pub val64: [u64; 4],
}

#[repr(C)]
pub struct kvm_fpu {
    pub fcsr: u32,
    pub fcc: u64, /* 8x8 */
    pub fpr: [kvm_fpureg; 32],
}

/*
 * For LoongArch, we use KVM_SET_ONE_REG and KVM_GET_ONE_REG to access various
 * registers.  The id field is broken down as follows:
 *
 *  bits[63..52] - As per linux/kvm.h
 *  bits[51..32] - Must be zero.
 *  bits[31..16] - Register set.
 *
 * Register set = 0: GP registers from kvm_regs (see definitions below).
 *
 * Register set = 1: CSR registers.
 *
 * Register set = 2: KVM specific registers (see definitions below).
 *
 * Register set = 3: FPU / SIMD registers (see definitions below).
 *
 * Other sets registers may be added in the future.  Each set would
 * have its own identifier in bits[31..16].
 */

pub const KVM_REG_LOONGARCH_GPR: u64 = KVM_REG_LOONGARCH | 0x00000u64;
pub const KVM_REG_LOONGARCH_CSR: u64 = KVM_REG_LOONGARCH | 0x10000u64;
pub const KVM_REG_LOONGARCH_KVM: u64 = KVM_REG_LOONGARCH | 0x20000u64;
pub const KVM_REG_LOONGARCH_FPSIMD: u64 = KVM_REG_LOONGARCH | 0x30000u64;
pub const KVM_REG_LOONGARCH_CPUCFG: u64 = KVM_REG_LOONGARCH | 0x40000u64;
pub const KVM_REG_LOONGARCH_LBT: u64 = KVM_REG_LOONGARCH | 0x50000u64;
pub const KVM_REG_LOONGARCH_MASK: u64 = KVM_REG_LOONGARCH | 0x70000u64;
pub const KVM_CSR_IDX_MASK: u64 = 0x7fff;
pub const KVM_CPUCFG_IDX_MASK: u64 = 0x7fff;

/* KVM_REG_LOONGARCH_KVM - KVM specific control registers. */
pub const KVM_REG_LOONGARCH_COUNTER: u64 = KVM_REG_LOONGARCH_KVM | KVM_REG_SIZE_U64 | 1;
pub const KVM_REG_LOONGARCH_VCPU_RESET: u64 = KVM_REG_LOONGARCH_KVM | KVM_REG_SIZE_U64 | 2;
/* Debugging: Special instruction for software breakpoint */
pub const KVM_REG_LOONGARCH_DEBUG_INST: u64 = KVM_REG_LOONGARCH_KVM | KVM_REG_SIZE_U64 | 3;

/* LBT registers */
pub const KVM_REG_LOONGARCH_LBT_SCR0: u64 = KVM_REG_LOONGARCH_LBT | KVM_REG_SIZE_U64 | 1;
pub const KVM_REG_LOONGARCH_LBT_SCR1: u64 = KVM_REG_LOONGARCH_LBT | KVM_REG_SIZE_U64 | 2;
pub const KVM_REG_LOONGARCH_LBT_SCR2: u64 = KVM_REG_LOONGARCH_LBT | KVM_REG_SIZE_U64 | 3;
pub const KVM_REG_LOONGARCH_LBT_SCR3: u64 = KVM_REG_LOONGARCH_LBT | KVM_REG_SIZE_U64 | 4;
pub const KVM_REG_LOONGARCH_LBT_EFLAGS: u64 = KVM_REG_LOONGARCH_LBT | KVM_REG_SIZE_U64 | 5;
pub const KVM_REG_LOONGARCH_LBT_FTOP: u64 = KVM_REG_LOONGARCH_LBT | KVM_REG_SIZE_U64 | 6;

pub const LOONGARCH_REG_SHIFT: u64 = 3;
pub const fn LOONGARCH_REG_64(typ: u64, reg: u64) -> u64 {
    typ | KVM_REG_SIZE_U64 | (reg << LOONGARCH_REG_SHIFT)
}
pub const fn KVM_IOC_CSRID(reg: u64) -> u64 {
    LOONGARCH_REG_64(KVM_REG_LOONGARCH_CSR, reg)
}
pub const fn KVM_IOC_CPUCFG(reg: u64) -> u64 {
    LOONGARCH_REG_64(KVM_REG_LOONGARCH_CPUCFG, reg)
}

/* Device Control API on vm fd */
pub const KVM_LOONGARCH_VM_FEAT_CTRL: u64 = 0;
pub const KVM_LOONGARCH_VM_FEAT_LSX: u64 = 0;
pub const KVM_LOONGARCH_VM_FEAT_LASX: u64 = 1;
pub const KVM_LOONGARCH_VM_FEAT_X86BT: u64 = 2;
pub const KVM_LOONGARCH_VM_FEAT_ARMBT: u64 = 3;
pub const KVM_LOONGARCH_VM_FEAT_MIPSBT: u64 = 4;
pub const KVM_LOONGARCH_VM_FEAT_PMU: u64 = 5;
pub const KVM_LOONGARCH_VM_FEAT_PV_IPI: u64 = 6;
pub const KVM_LOONGARCH_VM_FEAT_PV_STEALTIME: u64 = 7;
pub const KVM_LOONGARCH_VM_FEAT_PTW: u64 = 8;
pub const KVM_LOONGARCH_VM_FEAT_MSGINT: u64 = 9;
pub const KVM_LOONGARCH_VM_FEAT_PV_PREEMPT: u64 = 10;

/* Device Control API on vcpu fd */
pub const KVM_LOONGARCH_VCPU_CPUCFG: u64 = 0;
pub const KVM_LOONGARCH_VCPU_PVTIME_CTRL: u64 = 1;
pub const KVM_LOONGARCH_VCPU_PVTIME_GPA: u64 = 0;

#[repr(C)]
pub struct kvm_debug_exit_arch {}

/* for KVM_SET_GUEST_DEBUG */
#[repr(C)]
pub struct kvm_guest_debug_arch {}

/* definition of registers in kvm_run */
#[repr(C)]
pub struct kvm_sync_regs {}

/* dummy definition */
#[repr(C)]
pub struct kvm_sregs {}

#[repr(C)]
pub struct kvm_iocsr_entry {
    pub addr: u32,
    pub pad: u32,
    pub data: u64,
}

pub const KVM_NR_IRQCHIPS: u64 = 1;
pub const KVM_IRQCHIP_NUM_PINS: u64 = 64;
pub const KVM_MAX_CORES: u64 = 256;

pub const KVM_DEV_LOONGARCH_IPI_GRP_REGS: u64 = 0x40000001;
pub const KVM_DEV_LOONGARCH_EXTIOI_GRP_REGS: u64 = 0x40000002;
pub const KVM_DEV_LOONGARCH_EXTIOI_GRP_SW_STATUS: u64 = 0x40000003;
pub const KVM_DEV_LOONGARCH_EXTIOI_SW_STATUS_NUM_CPU: u64 = 0x0;
pub const KVM_DEV_LOONGARCH_EXTIOI_SW_STATUS_FEATURE: u64 = 0x1;
pub const KVM_DEV_LOONGARCH_EXTIOI_SW_STATUS_STATE: u64 = 0x2;
pub const KVM_DEV_LOONGARCH_EXTIOI_GRP_CTRL: u64 = 0x40000004;
pub const KVM_DEV_LOONGARCH_EXTIOI_CTRL_INIT_NUM_CPU: u64 = 0x0;
pub const KVM_DEV_LOONGARCH_EXTIOI_CTRL_INIT_FEATURE: u64 = 0x1;
pub const KVM_DEV_LOONGARCH_EXTIOI_CTRL_LOAD_FINISHED: u64 = 0x3;
pub const KVM_DEV_LOONGARCH_PCH_PIC_GRP_REGS: u64 = 0x40000005;
pub const KVM_DEV_LOONGARCH_PCH_PIC_GRP_CTRL: u64 = 0x40000006;
pub const KVM_DEV_LOONGARCH_PCH_PIC_CTRL_INIT: u64 = 0;
pub const KVM_DEV_LOONGARCH_DMSINTC_GRP_CTRL: u64 = 0x40000007;
pub const KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_BASE: u64 = 0x0;
pub const KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_SIZE: u64 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
