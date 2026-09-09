/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// External types supplied by other translation units.
pub enum kvm_vcpu {}
pub enum kvm_run {}
pub enum kvm_cpu_trap {}

#[repr(C)]
pub struct kvm_mmio_decode {
    pub insn: ::core::ffi::c_ulong,
    pub insn_len: ::core::ffi::c_int,
    pub len: ::core::ffi::c_int,
    pub shift: ::core::ffi::c_int,
    pub return_handled: ::core::ffi::c_int,
}

#[repr(C)]
pub struct kvm_csr_decode {
    pub insn: ::core::ffi::c_ulong,
    pub return_handled: ::core::ffi::c_int,
}

/* Return values used by function emulating a particular instruction */
#[repr(C)]
pub enum kvm_insn_return {
    KVM_INSN_EXIT_TO_USER_SPACE = 0,
    KVM_INSN_CONTINUE_NEXT_SEPC,
    KVM_INSN_CONTINUE_SAME_SEPC,
    KVM_INSN_ILLEGAL_TRAP,
    KVM_INSN_VIRTUAL_TRAP,
}

extern "C" {
    pub fn kvm_riscv_vcpu_wfi(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_csr_return(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
    ) -> ::core::ffi::c_int;
    pub fn kvm_riscv_vcpu_virtual_insn(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        trap: *mut kvm_cpu_trap,
    ) -> ::core::ffi::c_int;

    pub fn kvm_riscv_vcpu_mmio_load(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        fault_addr: gpa_t,
        htinst: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn kvm_riscv_vcpu_mmio_store(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
        fault_addr: gpa_t,
        htinst: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn kvm_riscv_vcpu_mmio_return(
        vcpu: *mut kvm_vcpu,
        run: *mut kvm_run,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
