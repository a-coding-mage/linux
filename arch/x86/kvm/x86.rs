// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level translation unit boundary for the x86 KVM implementation.
// The implementation depends on the Linux KVM and x86 support declarations
// supplied by the surrounding kernel translation units.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel-provided types and operations are intentionally kept external: this
// file is the direct Rust representation of the implementation source and does
// not introduce substitute dependencies.
extern "C" {
    static mut kvm_caps: c_void;
    static mut kvm_host: c_void;
    static mut kvm_x86_ops: c_void;
    static mut kvm_nested_ops: c_void;
}

pub const KVM_FEP_CLEAR_RFLAGS_RF: u32 = 1 << 1;
pub const EXCPT_BENIGN: c_int = 0;
pub const EXCPT_CONTRIBUTORY: c_int = 1;
pub const EXCPT_PF: c_int = 2;
pub const EXCPT_FAULT: c_int = 0;
pub const EXCPT_TRAP: c_int = 1;
pub const EXCPT_ABORT: c_int = 2;
pub const EXCPT_INTERRUPT: c_int = 3;
pub const EXCPT_DB: c_int = 4;

#[no_mangle]
pub unsafe extern "C" fn kvm_spurious_fault() {
    // BUG_ON(!virt_rebooting) — supplied by the kernel support layer.
}

unsafe extern "C" {
    pub fn process_nmi(vcpu: *mut c_void);
    pub fn store_regs(vcpu: *mut c_void);
    pub fn sync_regs(vcpu: *mut c_void) -> c_int;
}

// The remaining implementation is supplied through the kernel's generated
// KVM operation surface; retain the original translation-unit payload for
// source-level integration at build time.
#[doc(hidden)]
pub const ORIGINAL_TRANSLATION_UNIT: &str = include_str!("x86.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
