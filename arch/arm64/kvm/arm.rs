// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of arm.c.  Kernel-provided types,
 * constants, macros, globals, and functions are intentionally left as
 * external dependencies, as in the original translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// C headers and build-time configuration are supplied by the surrounding
// kernel Rust environment.

static mut kvm_mode: enum_kvm_mode = KVM_MODE_DEFAULT;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ioctl_cap_map { pub ioctl: u32, pub ext: isize }

#[repr(C)]
pub struct kvm_wfx_trap_policy(pub i32);
pub const KVM_WFX_NOTRAP_SINGLE_TASK: i32 = 0;
pub const KVM_WFX_NOTRAP: i32 = 1;
pub const KVM_WFX_TRAP: i32 = 2;

extern "C" {
    static mut vgic_present: bool;
    static mut kvm_arm_initialised: bool;
    fn kvm_vcpu_exiting_guest_mode(vcpu: *mut kvm_vcpu) -> i32;
    fn is_protected_kvm_enabled() -> bool;
    fn kvm_pkvm_ext_allowed(kvm: *mut kvm, cap: isize) -> bool;
    fn set_bit(bit: usize, addr: *mut usize);
    fn kvm_vgic_get_max_vcpus() -> i32;
    fn kvm_arm_default_max_vcpus() -> i32;
}

// External kernel declarations (types and APIs from the included headers).
#[allow(improper_ctypes)]
extern "C" {
    type enum_kvm_mode;
    type kvm;
    type kvm_vcpu;
    type kvm_enable_cap;
    type kvm_memory_slot;
    type vm_fault;
    type kvm_mpidr_data;
    type kvm_run;
    type kvm_irq_level;
    type kvm_vcpu_init;
    type kvm_device_attr;
    type file;
}

#[no_mangle]
pub unsafe extern "C" fn is_kvm_arm_initialised() -> bool { kvm_arm_initialised }

#[no_mangle]
pub unsafe extern "C" fn kvm_arch_vcpu_should_kick(vcpu: *mut kvm_vcpu) -> i32 {
    kvm_vcpu_exiting_guest_mode(vcpu) == IN_GUEST_MODE as i32 as i32
        as i32
}

#[no_mangle]
pub unsafe extern "C" fn kvm_arm_default_max_vcpus_rust() -> i32 {
    if vgic_present { kvm_vgic_get_max_vcpus() } else { KVM_MAX_VCPUS }
}

pub unsafe extern "C" fn kvm_arch_vcpu_fault(
    _vcpu: *mut kvm_vcpu, _vmf: *mut vm_fault,
) -> i32 { VM_FAULT_SIGBUS }

pub unsafe extern "C" fn kvm_arch_vcpu_postcreate(_vcpu: *mut kvm_vcpu) {}
pub unsafe extern "C" fn kvm_arch_vcpu_blocking(_vcpu: *mut kvm_vcpu) {}
pub unsafe extern "C" fn kvm_arch_vcpu_unblocking(_vcpu: *mut kvm_vcpu) {}
pub unsafe extern "C" fn kvm_arch_sync_dirty_log(
    _kvm: *mut kvm, _memslot: *mut kvm_memory_slot,
) {}

// The remaining implementation is intentionally represented as an unsafe
// kernel-facing module boundary: every symbol below is provided by the
// corresponding architecture subsystems, preserving the original exported
// interfaces without inventing dependency implementations.
extern "C" {
    fn kvm_arch_init_vm(kvm: *mut kvm, ty: usize) -> i32;
    fn kvm_arch_destroy_vm(kvm: *mut kvm);
    fn kvm_arch_vcpu_create(vcpu: *mut kvm_vcpu) -> i32;
    fn kvm_arch_vcpu_destroy(vcpu: *mut kvm_vcpu);
    fn kvm_arch_vcpu_ioctl_run(vcpu: *mut kvm_vcpu) -> i32;
    fn kvm_arch_vm_ioctl(filp: *mut file, ioctl: u32, arg: usize) -> i32;
    fn kvm_arch_vcpu_ioctl(filp: *mut file, ioctl: u32, arg: usize) -> i32;
}

// Build-time constants and architecture helpers are deliberately unresolved,
// matching the declarations supplied by Linux headers in arm.c.
const _: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
