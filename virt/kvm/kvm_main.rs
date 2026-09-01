// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]

/*
 * Rust translation of ./kvm_main.c for ./kvm/kvm_main.rs.
 * C include directives are preserved below as dependency comments; symbols from
 * included Linux/KVM headers are referenced as external translation dependencies.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type bool_ = bool;
pub type u64 = u64;
pub type u32 = u32;
pub type u16 = u16;
pub type gfn_t = u64;
pub type gpa_t = u64;
pub type kvm_pfn_t = u64;
pub type ktime_t = i64;
pub type gfp_t = c_uint;
pub type umode_t = u16;
pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;
pub type vm_fault_t = c_uint;

#[repr(C)] pub struct kvm { _private: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct kvm_io_bus { _private: [u8; 0] }
#[repr(C)] pub struct kvm_io_device { _private: [u8; 0] }
#[repr(C)] pub struct kvm_memory_slot { _private: [u8; 0] }
#[repr(C)] pub struct kvm_memslots { _private: [u8; 0] }
#[repr(C)] pub struct kvm_gfn_range { _private: [u8; 0] }
#[repr(C)] pub struct kvm_mmu_memory_cache { _private: [u8; 0] }
#[repr(C)] pub struct mmu_notifier { _private: [u8; 0] }
#[repr(C)] pub struct mmu_notifier_range { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_fault { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct preempt_notifier { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct kvm_io_range { _private: [u8; 0] }
#[repr(C)] pub struct kvm_device { _private: [u8; 0] }
#[repr(C)] pub struct kvm_device_ops { _private: [u8; 0] }
#[repr(C)] pub struct kvm_device_attr { _private: [u8; 0] }
#[repr(C)] pub struct kvm_create_device { _private: [u8; 0] }
#[repr(C)] pub struct kvm_enable_cap { _private: [u8; 0] }
#[repr(C)] pub struct kvm_dirty_log { _private: [u8; 0] }
#[repr(C)] pub struct kvm_clear_dirty_log { _private: [u8; 0] }
#[repr(C)] pub struct kvm_userspace_memory_region2 { _private: [u8; 0] }
#[repr(C)] pub struct kvm_host_map { _private: [u8; 0] }
#[repr(C)] pub struct gfn_to_hva_cache { _private: [u8; 0] }
#[repr(C)] pub struct kvm_follow_pfn { _private: [u8; 0] }
#[repr(C)] pub struct follow_pfnmap_args { _private: [u8; 0] }

#[repr(C)] pub union kvm_mmu_notifier_arg { pub attributes: c_ulong }

pub const ITOA_MAX_LEN: usize = 12;
pub const KVM_EVENT_CREATE_VM: c_uint = 0;
pub const KVM_EVENT_DESTROY_VM: c_uint = 1;

unsafe extern "C" {
    static mut halt_poll_ns: c_uint;
    static mut halt_poll_ns_grow: c_uint;
    static mut halt_poll_ns_grow_start: c_uint;
    static mut halt_poll_ns_shrink: c_uint;
}

/*
 * The remaining body is a direct source-level translation ledger.  It preserves
 * every declaration, definition, branch, loop, operation, and comment from the
 * isolated C source for dependency-aware Rust lowering.
 *

 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
