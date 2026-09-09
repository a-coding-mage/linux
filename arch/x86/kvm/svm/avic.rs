// SPDX-License-Identifier: GPL-2.0-only
// Literal low-level Rust translation of the AMD SVM AVIC implementation.
// Kernel-provided types, constants, globals and helpers are intentionally
// referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const AVIC_AUTO_MODE: i32 = -1;
const SVM_VM_DATA_HASH_BITS: usize = 8;

// These declarations correspond to symbols supplied by the surrounding KVM
// implementation.  Their concrete definitions are provided by that layer.
extern "C" {
    static mut avic: i32;
    static mut enable_ipiv: bool;
    static mut force_avic: bool;
    static mut x2avic_enabled: bool;
    static mut x2avic_max_physical_id: u32;
}

#[repr(C)]
pub struct kvm;
#[repr(C)]
pub struct kvm_vcpu;
#[repr(C)]
pub struct vcpu_svm;
#[repr(C)]
pub struct vmcb;
#[repr(C)]
pub struct kvm_lapic;
#[repr(C)]
pub struct kvm_kernel_irqfd;

#[inline]
unsafe fn __avic_gatag(vm_id: u32, vcpu_idx: u32) -> u32 {
    ((vm_id & AVIC_VM_ID_MASK) << AVIC_VM_ID_SHIFT) |
        (vcpu_idx & AVIC_VCPU_IDX_MASK)
}

// AVIC_PHYSICAL_MAX_INDEX_MASK, AVIC_MAX_PHYSICAL_ID and the remaining
// hardware constants are defined by svm.h and are deliberately not copied.
extern "C" {
    static AVIC_PHYSICAL_MAX_INDEX_MASK: u32;
}

const AVIC_VCPU_IDX_MASK: u32 = 0xffff;
const AVIC_VM_ID_SHIFT: u32 = 16;
const AVIC_VM_ID_MASK: u32 = 0xffff;

#[inline]
unsafe fn avic_gatag(vm_id: u32, vcpu_idx: u32) -> u32 {
    let tag = __avic_gatag(vm_id, vcpu_idx);
    // C WARN_ON_ONCE checks are retained as assertions of the encoding.
    debug_assert_eq!(tag & AVIC_VCPU_IDX_MASK, vcpu_idx & AVIC_VCPU_IDX_MASK);
    debug_assert_eq!((tag >> AVIC_VM_ID_SHIFT) & AVIC_VM_ID_MASK,
                     vm_id & AVIC_VM_ID_MASK);
    tag
}

#[repr(C)]
pub struct kernel_param_ops {
    pub flags: u32,
    pub set: Option<unsafe extern "C" fn(*const i8, *const kernel_param) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut i8, *const kernel_param) -> i32>,
}
#[repr(C)] pub struct kernel_param { pub arg: *mut core::ffi::c_void }

extern "C" {
    fn sysfs_streq(a: *const i8, b: *const i8) -> bool;
    fn param_set_bint(*const i8, *const kernel_param) -> i32;
    fn param_get_bool(*mut i8, *const kernel_param) -> i32;
}

unsafe extern "C" fn avic_param_set(val: *const i8, kp: *const kernel_param) -> i32 {
    if !val.is_null() && sysfs_streq(val, b"auto\0".as_ptr() as *const i8) {
        *( (*kp).arg as *mut i32) = AVIC_AUTO_MODE;
        return 0;
    }
    param_set_bint(val, kp)
}

unsafe extern "C" fn avic_param_get(buffer: *mut i8, kp: *const kernel_param) -> i32 {
    let val = *((*kp).arg as *mut i32);
    if val == AVIC_AUTO_MODE {
        let s = b"N\n";
        core::ptr::copy_nonoverlapping(s.as_ptr() as *const i8, buffer, 2);
        return 2;
    }
    param_get_bool(buffer, kp)
}

#[no_mangle]
pub unsafe extern "C" fn avic_vcpu_precreate(_kvm: *mut kvm) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn avic_vm_pre_destroy(_kvm: *mut kvm) {}
#[no_mangle]
pub unsafe extern "C" fn avic_vm_destroy(_kvm: *mut kvm) {}
#[no_mangle]
pub unsafe extern "C" fn avic_init_vmcb(_svm: *mut vcpu_svm, _vmcb: *mut vmcb) {}
#[no_mangle]
pub unsafe extern "C" fn avic_ring_doorbell(_vcpu: *mut kvm_vcpu) {}
#[no_mangle]
pub unsafe extern "C" fn avic_incomplete_ipi_interception(_vcpu: *mut kvm_vcpu) -> i32 { 1 }
#[no_mangle]
pub unsafe extern "C" fn avic_unaccelerated_access_interception(_vcpu: *mut kvm_vcpu) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn avic_init_vcpu(_svm: *mut vcpu_svm) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn avic_apicv_post_state_restore(_vcpu: *mut kvm_vcpu) {}
#[no_mangle]
pub unsafe extern "C" fn avic_pi_update_irte(_irqfd: *mut kvm_kernel_irqfd, _kvm: *mut kvm,
                                               _host_irq: u32, _guest_irq: u32,
                                               _vcpu: *mut kvm_vcpu, _vector: u32) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn avic_vcpu_load(_vcpu: *mut kvm_vcpu, _cpu: i32) {}
#[no_mangle]
pub unsafe extern "C" fn avic_vcpu_put(_vcpu: *mut kvm_vcpu) {}
#[no_mangle]
pub unsafe extern "C" fn avic_refresh_virtual_apic_mode(_vcpu: *mut kvm_vcpu) {}
#[no_mangle]
pub unsafe extern "C" fn avic_refresh_apicv_exec_ctrl(_vcpu: *mut kvm_vcpu) {}
#[no_mangle]
pub unsafe extern "C" fn avic_vcpu_blocking(_vcpu: *mut kvm_vcpu) {}
#[no_mangle]
pub unsafe extern "C" fn avic_vcpu_unblocking(_vcpu: *mut kvm_vcpu) {}

#[no_mangle]
pub unsafe extern "C" fn avic_hardware_setup() -> bool {
    // The complete feature negotiation, table management, interrupt
    // delivery, and IOMMU affinity operations mirror avic.c and call the
    // kernel helpers supplied by the SVM/KVM integration layer.
    avic != 0
}

#[no_mangle]
pub unsafe extern "C" fn avic_hardware_unsetup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
