/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * This file is for C prototypes of asm symbols that are EXPORTed.
 * It allows the modversions logic to see their prototype and
 * generate proper CRCs for them.
 *
 * Copyright 2016, Daniel Axtens, IBM Corporation.
 *
 * C header dependencies are supplied by other translated files.
 */

/* Ultravisor */
#[cfg(any(CONFIG_PPC_POWERNV, CONFIG_PPC_SVM))]
unsafe extern "C" {
    pub fn ucall_norets(opcode: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_long;
}

#[cfg(not(any(CONFIG_PPC_POWERNV, CONFIG_PPC_SVM)))]
#[inline]
pub unsafe fn ucall_norets(_opcode: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_long {
    U_NOT_AVAILABLE
}

/* OPAL */
unsafe extern "C" {
    pub fn __opal_call(
        a0: i64,
        a1: i64,
        a2: i64,
        a3: i64,
        a4: i64,
        a5: i64,
        a6: i64,
        a7: i64,
        opcode: i64,
        msr: u64,
    ) -> i64;
}

/* misc runtime */
unsafe extern "C" {
    pub fn enable_machine_check();
    pub fn __bswapdi2(x: u64) -> u64;
    pub fn __lshrdi3(x: i64, shift: i32) -> i64;
    pub fn __ashldi3(x: i64, shift: i32) -> i64;
    pub fn __ashrdi3(x: i64, shift: i32) -> i64;
    pub fn __cmpdi2(a: i64, b: i64) -> i32;
    pub fn __ucmpdi2(a: u64, b: u64) -> i32;
}

/* tracing */
unsafe extern "C" {
    pub fn _mcount();
}

/* Transaction memory related */
unsafe extern "C" {
    pub fn tm_enable();
    pub fn tm_disable();
    pub fn tm_abort(cause: u8);
}

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn _kvmppc_restore_tm_pr(vcpu: *mut kvm_vcpu, guest_msr: u64);
    pub fn _kvmppc_save_tm_pr(vcpu: *mut kvm_vcpu, guest_msr: u64);
}

#[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
unsafe extern "C" {
    pub fn kvmppc_save_tm_hv(vcpu: *mut kvm_vcpu, msr: u64, preserve_nv: bool);
    pub fn kvmppc_restore_tm_hv(vcpu: *mut kvm_vcpu, msr: u64, preserve_nv: bool);
}

#[cfg(not(CONFIG_PPC_TRANSACTIONAL_MEM))]
#[inline]
pub unsafe fn kvmppc_save_tm_hv(
    _vcpu: *mut kvm_vcpu,
    _msr: u64,
    _preserve_nv: bool,
) {
}

#[cfg(not(CONFIG_PPC_TRANSACTIONAL_MEM))]
#[inline]
pub unsafe fn kvmppc_restore_tm_hv(
    _vcpu: *mut kvm_vcpu,
    _msr: u64,
    _preserve_nv: bool,
) {
}

unsafe extern "C" {
    pub fn kvmppc_p9_enter_guest(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_h_set_dabr(vcpu: *mut kvm_vcpu, dabr: ::core::ffi::c_ulong) -> ::core::ffi::c_long;
    pub fn kvmppc_h_set_xdabr(
        vcpu: *mut kvm_vcpu,
        dabr: ::core::ffi::c_ulong,
        dabrx: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
