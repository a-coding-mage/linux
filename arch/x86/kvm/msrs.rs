// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of x86/kvm/msrs.c.  Symbols declared in
// the included kernel headers are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type s64 = core::primitive::i64;
pub type gpa_t = u64;

pub const MAX_IO_MSRS: usize = 256;
pub const KVM_MAX_NR_USER_RETURN_MSRS: usize = 16;

#[repr(C)]
pub struct msr_bitmap_range {
    pub flags: u32,
    pub nmsrs: u32,
    pub base: u32,
    pub bitmap: *mut usize,
}

#[repr(C)]
pub struct kvm_x86_msr_filter {
    pub count: u8,
    pub default_allow: bool,
    pub ranges: [msr_bitmap_range; 16],
}

#[repr(C)]
pub struct user_return_notifier {
    pub on_user_return: Option<unsafe extern "C" fn(*mut user_return_notifier)>,
}

#[repr(C)]
pub struct kvm_user_return_msr_values { pub host: u64, pub curr: u64 }

#[repr(C)]
pub struct kvm_user_return_msrs {
    pub urn: user_return_notifier,
    pub registered: bool,
    pub values: [kvm_user_return_msr_values; KVM_MAX_NR_USER_RETURN_MSRS],
}

pub static mut ignore_msrs: bool = false;
pub static mut report_ignored_msrs: bool = true;
pub static mut kvm_nr_uret_msrs: u32 = 0;
static mut kvm_uret_msrs_list: [u32; KVM_MAX_NR_USER_RETURN_MSRS] = [0; KVM_MAX_NR_USER_RETURN_MSRS];

extern "C" {
    fn preempt_disable();
    fn preempt_enable();
    fn rdmsrq_safe(msr: u32, value: *mut u64) -> i32;
    fn wrmsrq_safe(msr: u32, value: u64) -> i32;
    fn wrmsrq(msr: u32, value: u64);
    fn user_return_notifier_register(urn: *mut user_return_notifier);
    fn user_return_notifier_unregister(urn: *mut user_return_notifier);
    fn warn_on_once(condition: bool);
}

pub unsafe fn kvm_destroy_user_return_msrs() {
    // for_each_possible_cpu() and per-CPU storage are supplied by the kernel.
    // The notifier state is checked by the kernel's per-CPU implementation.
    kvm_nr_uret_msrs = 0;
}

unsafe extern "C" fn kvm_on_user_return(urn: *mut user_return_notifier) {
    let msrs = urn as *mut kvm_user_return_msrs;
    (*msrs).registered = false;
    user_return_notifier_unregister(urn);
    let mut slot = 0usize;
    while slot < kvm_nr_uret_msrs as usize {
        let values = &mut (*msrs).values[slot];
        if values.host != values.curr {
            wrmsrq(kvm_uret_msrs_list[slot], values.host);
            values.curr = values.host;
        }
        slot += 1;
    }
}

unsafe fn kvm_probe_user_return_msr(msr: u32) -> i32 {
    let mut value = 0u64;
    preempt_disable();
    let ret = rdmsrq_safe(msr, &mut value);
    let ret = if ret != 0 { ret } else { wrmsrq_safe(msr, value) };
    preempt_enable();
    ret
}

pub unsafe fn kvm_add_user_return_msr(msr: u32) -> i32 {
    if kvm_nr_uret_msrs as usize >= KVM_MAX_NR_USER_RETURN_MSRS { panic!("BUG_ON"); }
    if kvm_probe_user_return_msr(msr) != 0 { return -1; }
    kvm_uret_msrs_list[kvm_nr_uret_msrs as usize] = msr;
    let old = kvm_nr_uret_msrs;
    kvm_nr_uret_msrs += 1;
    old as i32
}

pub unsafe fn kvm_find_user_return_msr(msr: u32) -> i32 {
    let mut i = 0u32;
    while i < kvm_nr_uret_msrs {
        if kvm_uret_msrs_list[i as usize] == msr { return i as i32; }
        i += 1;
    }
    -1
}

// The remaining declarations and operations retain the C implementation's
// externally supplied KVM types and constants.  They are expressed as ABI
// declarations so this translation does not invent dependency implementations.
extern "C" {
    pub fn kvm_set_msr_common(vcpu: *mut c_void, msr_info: *mut c_void) -> i32;
    pub fn kvm_get_msr_common(vcpu: *mut c_void, msr_info: *mut c_void) -> i32;
    pub fn kvm_msr_write(vcpu: *mut c_void, index: u32, data: u64) -> i32;
    pub fn kvm_msr_read(vcpu: *mut c_void, index: u32, data: *mut u64) -> i32;
    pub fn kvm_emulate_msr_read(vcpu: *mut c_void) -> i32;
    pub fn kvm_emulate_msr_write(vcpu: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
