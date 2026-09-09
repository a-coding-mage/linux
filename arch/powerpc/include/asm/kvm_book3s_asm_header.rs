/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright SUSE Linux Products GmbH 2009
 *
 * Authors: Alexander Graf <agraf@suse.de>
 */

use core::ffi::c_void;

/* XICS ICP register offsets */
pub const XICS_XIRR: i32 = 4;
pub const XICS_MFRR: i32 = 0xc;
pub const XICS_IPI: i32 = 2; /* interrupt source # for IPIs */

/* Maximum number of threads per physical core */
pub const MAX_SMT_THREADS: usize = 8;

/* Maximum number of subcores per physical core */
pub const MAX_SUBCORES: usize = 4;

/*
 * In the assembler source, DO_KVM branches to kvmppc_trampoline_<intno>
 * for the listed interrupt numbers when CONFIG_KVM_BOOK3S_HANDLER is set,
 * and defines an empty macro otherwise.  The assembler macro is intentionally
 * retained here as source-level intent rather than executable Rust.
 */

pub type ulong = usize;

pub struct kvmppc_vcore;
pub struct kvm_vcpu;

/* Struct used for coordinating micro-threading (split-core) mode changes */
#[repr(C)]
pub struct kvm_split_mode {
    pub rpr: ulong,
    pub pmmar: ulong,
    pub ldbar: ulong,
    pub subcore_size: u8,
    pub do_nap: u8,
    pub napped: [u8; MAX_SMT_THREADS],
    pub vc: [*mut kvmppc_vcore; MAX_SUBCORES],
}

/*
 * This struct goes in the PACA on 64-bit processors.  It is used
 * to store host state that needs to be saved when we enter a guest
 * and restored when we exit, but isn't specific to any particular
 * guest or vcpu.  It also has some scratch fields used by the guest
 * exit code.
 */
#[repr(C)]
pub struct kvmppc_host_state {
    pub host_r1: ulong,
    pub host_r2: ulong,
    pub host_msr: ulong,
    pub vmhandler: ulong,
    pub scratch0: ulong,
    pub scratch1: ulong,
    pub scratch2: ulong,
    pub in_guest: u8,
    pub restore_hid5: u8,
    pub napping: u8,

    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub hwthread_req: u8,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub hwthread_state: u8,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub host_ipi: u8,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub ptid: u8, /* thread number within subcore when split */
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub fake_suspend: u8,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub kvm_vcpu: *mut kvm_vcpu,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub kvm_vcore: *mut kvmppc_vcore,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub xics_phys: *mut c_void,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub xive_tima_phys: *mut c_void,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub xive_tima_virt: *mut c_void,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub saved_xirr: u32,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub dabr: u64,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub host_mmcr: [u64; 7], /* MMCR 0,1,A, SIAR, SDAR, MMCR2, SIER */
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub host_pmc: [u32; 8],
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub host_purr: u64,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub host_spurr: u64,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub host_dscr: u64,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub dec_expires: u64,
    #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
    pub kvm_split_mode: *mut kvm_split_mode,

    #[cfg(CONFIG_PPC_BOOK3S_64)]
    pub cfar: u64,
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    pub ppr: u64,
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    pub host_fscr: u64,
}

#[repr(C)]
pub struct kvmppc_book3s_shadow_vcpu {
    pub in_use: bool,
    pub gpr: [ulong; 14],
    pub cr: u32,
    pub xer: ulong,
    pub ctr: ulong,
    pub lr: ulong,
    pub pc: ulong,
    pub shadow_srr1: ulong,
    pub fault_dar: ulong,
    pub fault_dsisr: u32,
    pub last_inst: u32,

    #[cfg(CONFIG_PPC_BOOK3S_32)]
    pub sr: [u32; 16], /* Guest SRs */
    #[cfg(CONFIG_PPC_BOOK3S_32)]
    pub hstate: kvmppc_host_state,

    #[cfg(CONFIG_PPC_BOOK3S_64)]
    pub slb_max: u8, /* highest used guest slb entry */
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    pub slb: [kvmppc_shadow_slb_entry; 64], /* guest SLB */
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    pub shadow_fscr: u64,
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
#[repr(C)]
pub struct kvmppc_shadow_slb_entry {
    pub esid: u64,
    pub vsid: u64,
}

/* Values for kvm_state */
pub const KVM_HWTHREAD_IN_KERNEL: i32 = 0;
pub const KVM_HWTHREAD_IN_IDLE: i32 = 1;
pub const KVM_HWTHREAD_IN_KVM: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
