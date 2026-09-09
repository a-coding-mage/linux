/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020-2023 Loongson Technology Corporation Limited */

//! Rust translation of `loongarch/kvm/trace.h`.
//!
//! The C tracepoint declarations are represented by their event payloads and
//! symbolic tables.  Registration and formatted emission remain provided by
//! the tracepoint subsystem.

use core::ffi::c_void;

pub const KVM_TRACE_EXIT_IDLE: u32 = 64;
pub const KVM_TRACE_EXIT_CACHE: u32 = 65;
pub const KVM_TRACE_EXIT_CPUCFG: u32 = 66;
pub const KVM_TRACE_EXIT_CSR: u32 = 67;

pub const KVM_TRACE_AUX_SAVE: u32 = 0;
pub const KVM_TRACE_AUX_RESTORE: u32 = 1;
pub const KVM_TRACE_AUX_ENABLE: u32 = 2;
pub const KVM_TRACE_AUX_DISABLE: u32 = 3;
pub const KVM_TRACE_AUX_DISCARD: u32 = 4;

pub const KVM_TRACE_AUX_FPU: u32 = 1;
pub const KVM_TRACE_AUX_LSX: u32 = 2;
pub const KVM_TRACE_AUX_LASX: u32 = 3;

pub const KVM_TRACE_IOCSR_READ_UNSATISFIED: u32 = 0;
pub const KVM_TRACE_IOCSR_READ: u32 = 1;
pub const KVM_TRACE_IOCSR_WRITE: u32 = 2;

pub static KVM_TRACE_SYMBOL_EXIT_TYPES: &[(u32, &str)] = &[
    (KVM_TRACE_EXIT_IDLE, "IDLE"),
    (KVM_TRACE_EXIT_CACHE, "CACHE"),
    (KVM_TRACE_EXIT_CPUCFG, "CPUCFG"),
    (KVM_TRACE_EXIT_CSR, "CSR"),
];

pub static KVM_TRACE_SYMBOL_AUX_OP: &[(u32, &str)] = &[
    (KVM_TRACE_AUX_SAVE, "save"),
    (KVM_TRACE_AUX_RESTORE, "restore"),
    (KVM_TRACE_AUX_ENABLE, "enable"),
    (KVM_TRACE_AUX_DISABLE, "disable"),
    (KVM_TRACE_AUX_DISCARD, "discard"),
];

pub static KVM_TRACE_SYMBOL_AUX_STATE: &[(u32, &str)] = &[
    (KVM_TRACE_AUX_FPU, "FPU"),
    (KVM_TRACE_AUX_LSX, "LSX"),
    (KVM_TRACE_AUX_LASX, "LASX"),
];

pub static KVM_TRACE_SYMBOL_IOCSR: &[(u32, &str)] = &[
    (KVM_TRACE_IOCSR_READ_UNSATISFIED, "unsatisfied-read"),
    (KVM_TRACE_IOCSR_READ, "read"),
    (KVM_TRACE_IOCSR_WRITE, "write"),
];

#[repr(C)]
pub struct KvmTransitionEntry {
    pub vcpu_id: u32,
    pub pc: usize,
}

#[repr(C)]
pub struct KvmExitEntry {
    pub vcpu_id: u32,
    pub pc: usize,
    pub reason: u32,
}

#[repr(C)]
pub struct KvmExitGsprEntry {
    pub vcpu_id: u32,
    pub inst_word: u32,
}

#[repr(C)]
pub struct KvmAuxEntry {
    pub pc: usize,
    pub op: u8,
    pub state: u8,
}

#[repr(C)]
pub struct KvmIocsrEntry {
    pub type_: u32,
    pub len: u32,
    pub gpa: u64,
    pub val: u64,
}

#[repr(C)]
pub struct KvmVpidChangeEntry {
    pub vpid: usize,
}

#[repr(C)]
pub struct KvmVcpu;

extern "C" {
    pub fn kvm_enter(vcpu: *mut KvmVcpu);
    pub fn kvm_reenter(vcpu: *mut KvmVcpu);
    pub fn kvm_out(vcpu: *mut KvmVcpu);
    pub fn kvm_exit_idle(vcpu: *mut KvmVcpu, reason: u32);
    pub fn kvm_exit_cache(vcpu: *mut KvmVcpu, reason: u32);
    pub fn kvm_exit_cpucfg(vcpu: *mut KvmVcpu, reason: u32);
    pub fn kvm_exit_csr(vcpu: *mut KvmVcpu, reason: u32);
    pub fn kvm_exit(vcpu: *mut KvmVcpu, reason: u32);
    pub fn kvm_exit_gspr(vcpu: *mut KvmVcpu, inst_word: u32);
    pub fn kvm_aux(vcpu: *mut KvmVcpu, op: u32, state: u32);
    pub fn kvm_iocsr(type_: i32, len: i32, gpa: u64, val: *mut c_void);
    pub fn kvm_vpid_change(vcpu: *mut KvmVcpu, vpid: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
