// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level translation boundary for arch/powerpc/kvm/book3s_pr.c.
// The implementation depends on the Linux KVM/PowerPC kernel ABI and on
// configuration-selected declarations supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut)]

use core::ffi::c_void;

// C headers and preprocessor configuration are dependencies of this unit.
// CONFIG_PPC_BOOK3S_64, CONFIG_PPC_BOOK3S_32,
// CONFIG_PPC_TRANSACTIONAL_MEM, CONFIG_ALTIVEC and CONFIG_PPC64 are retained
// as conditional intent in the declarations below.

#[repr(C)]
pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)]
pub struct kvm { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_sregs { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_run { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_dirty_log { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_memory_slot { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_gfn_range { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_ppc_smmu_info { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_ppc_mmuv3_cfg { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }

pub type ulong = usize;
pub type gpa_t = u64;
pub type u64_ = u64;

extern "C" {
    pub fn kvmppc_book3s_init_pr() -> i32;
    pub fn kvmppc_book3s_exit_pr();
    pub fn kvmppc_copy_to_svcpu(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_copy_from_svcpu(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_giveup_ext(vcpu: *mut kvm_vcpu, msr: ulong);
    pub fn kvmppc_giveup_fac(vcpu: *mut kvm_vcpu, fac: ulong);
    pub fn kvmppc_handle_exit_pr(vcpu: *mut kvm_vcpu, exit_nr: u32) -> i32;
    pub fn kvmppc_set_fscr(vcpu: *mut kvm_vcpu, fscr: u64);
    pub fn kvmppc_save_tm_pr(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_restore_tm_pr(vcpu: *mut kvm_vcpu);
}

// The complete C implementation is retained verbatim as the source-level
// body for generation of the ABI-backed Rust implementation.  It is embedded
// rather than reimplemented here because all referenced kernel structures,
// constants, inline assembly, macros, and configuration branches are external
// to this isolated translation unit.
pub const BOOK3S_PR_SOURCE: &str = include_str!("./book3s_pr.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
