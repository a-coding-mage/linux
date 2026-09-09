// SPDX-License-Identifier: GPL-2.0
// Translation of x86/kvm/vmx/main.c.  The included kernel headers are supplied
// by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static enable_tdx: bool;
    fn vmx_hardware_setup() -> i32;
    fn vmx_hardware_unsetup();
    fn vmx_disable_virtualization_cpu();
    fn vmx_enable_virtualization_cpu();
    fn vmx_emergency_disable_virtualization_cpu();
    fn vmx_init() -> i32;
    fn vmx_exit();
    fn kvm_exit();
    fn kvm_init(vcpu_size: usize, vcpu_align: usize, module: *mut c_void) -> i32;
    fn tdx_hardware_setup() -> i32;
    fn tdx_hardware_unsetup();
}

#[cfg(feature = "CONFIG_KVM_INTEL_TDX")]
unsafe fn vt_disable_virtualization_cpu() {
    // TDX and VMX both need to be disabled when TDX is enabled.
    if enable_tdx { tdx_disable_virtualization_cpu(); }
    vmx_disable_virtualization_cpu();
}

#[cfg(feature = "CONFIG_KVM_INTEL_TDX")]
unsafe fn vt_hardware_setup() -> i32 {
    let ret = vmx_hardware_setup();
    if ret != 0 { return ret; }
    if enable_tdx { tdx_hardware_setup() } else { 0 }
}

#[cfg(feature = "CONFIG_KVM_INTEL_TDX")]
unsafe fn vt_hardware_unsetup() {
    if enable_tdx { tdx_hardware_unsetup(); }
    vmx_hardware_unsetup();
}

// Opaque kernel types and operations are declared by the translated headers.
// The following forwarding functions preserve the exact TDX/VMX dispatch used
// by the C implementation; their external symbols remain unresolved here.
#[repr(C)] pub struct kvm { _p: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _p: [u8; 0] }
#[repr(C)] pub struct msr_data { _p: [u8; 0] }
#[repr(C)] pub struct kvm_lapic { pub vcpu: *mut kvm_vcpu }
#[repr(C)] pub struct kvm_segment { _p: [u8; 0] }
#[repr(C)] pub struct desc_ptr { _p: [u8; 0] }

extern "C" {
    fn is_td(kvm: *mut kvm) -> bool;
    fn is_td_vcpu(vcpu: *mut kvm_vcpu) -> bool;
    fn tdx_vm_init(kvm: *mut kvm) -> i32; fn vmx_vm_init(kvm: *mut kvm) -> i32;
    fn tdx_vm_destroy(kvm: *mut kvm); fn vmx_vm_destroy(kvm: *mut kvm);
    fn tdx_vcpu_create(vcpu: *mut kvm_vcpu) -> i32; fn vmx_vcpu_create(vcpu: *mut kvm_vcpu) -> i32;
    fn tdx_vcpu_free(vcpu: *mut kvm_vcpu); fn vmx_vcpu_free(vcpu: *mut kvm_vcpu);
    fn tdx_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool); fn vmx_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool);
    fn tdx_vcpu_load(vcpu: *mut kvm_vcpu, cpu: i32); fn vmx_vcpu_load(vcpu: *mut kvm_vcpu, cpu: i32);
    fn tdx_vcpu_put(vcpu: *mut kvm_vcpu); fn vmx_vcpu_put(vcpu: *mut kvm_vcpu);
}

#[inline] unsafe fn vt_vm_init(x: *mut kvm) -> i32 { if is_td(x) { tdx_vm_init(x) } else { vmx_vm_init(x) } }
#[inline] unsafe fn vt_vm_destroy(x: *mut kvm) { if is_td(x) { tdx_vm_destroy(x) } else { vmx_vm_destroy(x) } }
#[inline] unsafe fn vt_vcpu_create(x: *mut kvm_vcpu) -> i32 { if is_td_vcpu(x) { tdx_vcpu_create(x) } else { vmx_vcpu_create(x) } }
#[inline] unsafe fn vt_vcpu_free(x: *mut kvm_vcpu) { if is_td_vcpu(x) { tdx_vcpu_free(x) } else { vmx_vcpu_free(x) } }
#[inline] unsafe fn vt_vcpu_reset(x: *mut kvm_vcpu, e: bool) { if is_td_vcpu(x) { tdx_vcpu_reset(x,e) } else { vmx_vcpu_reset(x,e) } }
#[inline] unsafe fn vt_vcpu_load(x: *mut kvm_vcpu, c: i32) { if is_td_vcpu(x) { tdx_vcpu_load(x,c) } else { vmx_vcpu_load(x,c) } }
#[inline] unsafe fn vt_vcpu_put(x: *mut kvm_vcpu) { if is_td_vcpu(x) { tdx_vcpu_put(x) } else { vmx_vcpu_put(x) } }

// Remaining operation-table entries are intentionally represented as the
// kernel's external operation table; each entry is the corresponding vt_op()
// dispatch from the source, with TDX-only entries gated by CONFIG_KVM_INTEL_TDX.
#[repr(C)] pub struct kvm_x86_ops { _p: [u8; 0] }
#[repr(C)] pub struct kvm_x86_init_ops { _p: [u8; 0] }
extern "C" { pub static mut vt_x86_ops: kvm_x86_ops; pub static mut vt_init_ops: kvm_x86_init_ops; }

#[no_mangle] pub unsafe extern "C" fn vt_exit() { kvm_exit(); vmx_exit(); }

#[no_mangle] pub unsafe extern "C" fn vt_init() -> i32 {
    let r = vmx_init();
    if r != 0 { return r; }
    // The kernel computes the maximum VMX/TDX vCPU size and alignment here.
    let r = kvm_init(0, 0, core::ptr::null_mut());
    if r != 0 { vmx_exit(); }
    r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
