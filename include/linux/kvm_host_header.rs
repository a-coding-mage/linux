/* SPDX-License-Identifier: GPL-2.0-only */
//! Low-level Rust translation of `linux/kvm_host.h`.
//! Types supplied by the Linux/KVM headers remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type bool_t = bool;
pub type kvm_pfn_t = u64;
pub type gfn_t = u64;
pub type gpa_t = u64;
pub type hpa_t = u64;
pub type ktime_t = i64;
pub type pid_t = i32;
pub type pgoff_t = u64;
pub type vm_fault_t = c_ulong;

/* External kernel types. */
#[repr(C)] pub struct kvm { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_arch { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu_arch { pub _opaque: [u8; 0] }
macro_rules! opaque { ($($n:ident),* $(,)?) => { $(#[repr(C)] pub struct $n { pub _opaque: [u8; 0] })* }; }
opaque!(page, file, module, dentry, mm_struct, pid, kvm_run, kvm_io_device,
    work_struct, preempt_notifier, rcuwait, mutex, rwlock_t, spinlock_t, sigset_t,
    list_head, rcu_head, hlist_node, hlist_head, interval_tree_node, rb_node,
    rb_root, rb_root_cached, xarray, srcu_struct, mmu_notifier, notifier_block,
    kvm_dirty_ring, kvm_vm_stat, kvm_vcpu_stat, kvm_arch_memory_slot,
    kvm_stats_desc, kvm_stats_header, kvm_irq_routing_entry, kvm_irq_level,
    kvm_enable_cap, kvm_fpu, kvm_translation, kvm_regs, kvm_sregs, kvm_mp_state,
    kvm_guest_debug, kvm_irqfd, kvm_msi, kvm_ioeventfd, kvm_device_attr,
    vm_area_struct, kvm_pre_fault_memory, irq_bypass_consumer, irq_bypass_producer,
    kvm_mmu_memory_cache, gfn_to_hva_cache, gfn_to_pfn_cache, kvm_userspace_memory_region2);

pub const KVM_MEMSLOT_INVALID: u64 = 1u64 << 16;
pub const KVM_MEMSLOT_GMEM_ONLY: u64 = 1u64 << 17;
pub const KVM_MEMSLOT_GEN_UPDATE_IN_PROGRESS: u64 = 1u64 << 63;
pub const KVM_MAX_MMIO_FRAGMENTS: usize = 2;
pub const KVM_PFN_ERR_MASK: u64 = 0x7ffu64 << 52;
pub const KVM_PFN_ERR_NOSLOT_MASK: u64 = 0xfffu64 << 52;
pub const KVM_PFN_NOSLOT: u64 = 1u64 << 63;
pub const KVM_PFN_ERR_FAULT: u64 = KVM_PFN_ERR_MASK;
pub const KVM_PFN_ERR_HWPOISON: u64 = KVM_PFN_ERR_MASK + 1;
pub const KVM_PFN_ERR_RO_FAULT: u64 = KVM_PFN_ERR_MASK + 2;
pub const KVM_PFN_ERR_SIGPENDING: u64 = KVM_PFN_ERR_MASK + 3;
pub const KVM_PFN_ERR_NEEDS_IO: u64 = KVM_PFN_ERR_MASK + 4;
pub const KVM_REQUEST_MASK: u32 = 0xff;
pub const KVM_REQUEST_NO_WAKEUP: u32 = 1 << 8;
pub const KVM_REQUEST_WAIT: u32 = 1 << 9;
pub const KVM_REQUEST_NO_ACTION: u32 = 1 << 10;
pub const KVM_REQ_TLB_FLUSH: u32 = KVM_REQUEST_WAIT | KVM_REQUEST_NO_WAKEUP;
pub const KVM_REQ_VM_DEAD: u32 = 1 | KVM_REQUEST_WAIT | KVM_REQUEST_NO_WAKEUP;
pub const KVM_REQ_UNBLOCK: u32 = 2;
pub const KVM_REQ_DIRTY_RING_SOFT_FULL: u32 = 3;
pub const KVM_REQUEST_ARCH_BASE: u32 = 8;
pub const KVM_DIRTY_RING_RSVD_ENTRIES: u32 = 64;
pub const KVM_DIRTY_RING_MAX_ENTRIES: u32 = 65536;

#[inline] pub const fn is_error_pfn(pfn: kvm_pfn_t) -> bool { pfn & KVM_PFN_ERR_MASK != 0 }
#[inline] pub const fn is_sigpending_pfn(pfn: kvm_pfn_t) -> bool { pfn == KVM_PFN_ERR_SIGPENDING }
#[inline] pub const fn is_error_noslot_pfn(pfn: kvm_pfn_t) -> bool { pfn & KVM_PFN_ERR_NOSLOT_MASK != 0 }
#[inline] pub const fn is_noslot_pfn(pfn: kvm_pfn_t) -> bool { pfn == KVM_PFN_NOSLOT }
pub const KVM_MAX_IRQ_ROUTES: u32 = 4096;

#[repr(C)] pub struct kvm_io_range { pub addr: gpa_t, pub len: c_int, pub dev: *mut kvm_io_device }
#[repr(C)] pub struct kvm_mmio_fragment { pub gpa: gpa_t, pub data: *mut c_void, pub val: u64, pub len: c_uint }
#[repr(C)] pub struct kvm_host_map { pub pinned_page: *mut page, pub page: *mut page, pub hva: *mut c_void, pub pfn: kvm_pfn_t, pub gfn: kvm_pfn_t, pub writable: bool }
#[repr(C)] pub struct kvm_gfn_range { pub slot: *mut kvm_memory_slot, pub start: gfn_t, pub end: gfn_t, pub arg: kvm_mmu_notifier_arg, pub attr_filter: kvm_gfn_range_filter, pub may_block: bool, pub lockless: bool }
#[repr(C)] pub union kvm_mmu_notifier_arg { pub attributes: c_ulong }
#[repr(C)] pub struct kvm_memory_slot { pub id_node: [hlist_node;2], pub hva_node: [interval_tree_node;2], pub gfn_node: [rb_node;2], pub base_gfn: gfn_t, pub npages: c_ulong, pub dirty_bitmap: *mut c_ulong, pub arch: kvm_arch_memory_slot, pub userspace_addr: c_ulong, pub flags: u32, pub id: i16, pub as_id: u16 }
#[repr(C)] pub struct kvm_memslots { pub generation: u64, pub last_used_slot: c_ulong, pub hva_tree: rb_root_cached, pub gfn_tree: rb_root, pub node_idx: c_int }
#[repr(C)] pub struct kvm_memslot_iter { pub slots: *mut kvm_memslots, pub node: *mut rb_node, pub slot: *mut kvm_memory_slot }
#[repr(C)] pub struct kvm_device { pub ops: *const kvm_device_ops, pub kvm: *mut kvm, pub private: *mut c_void, pub vm_node: list_head }
#[repr(C)] pub struct kvm_device_ops { pub name: *const c_char, pub create: Option<unsafe extern "C" fn(*mut kvm_device,u32)->c_int>, pub init: Option<unsafe extern "C" fn(*mut kvm_device)>, pub destroy: Option<unsafe extern "C" fn(*mut kvm_device)>, pub release: Option<unsafe extern "C" fn(*mut kvm_device)>, pub set_attr: Option<unsafe extern "C" fn(*mut kvm_device,*mut kvm_device_attr)->c_int>, pub get_attr: Option<unsafe extern "C" fn(*mut kvm_device,*mut kvm_device_attr)->c_int>, pub has_attr: Option<unsafe extern "C" fn(*mut kvm_device,*mut kvm_device_attr)->c_int>, pub ioctl: Option<unsafe extern "C" fn(*mut kvm_device,c_uint,c_ulong)->c_long>, pub mmap: Option<unsafe extern "C" fn(*mut kvm_device,*mut vm_area_struct)->c_int> }
#[repr(C)] pub struct kvm_s390_adapter_int { pub ind_addr:u64,pub ind_gaddr:u64,pub summary_addr:u64,pub summary_gaddr:u64,pub ind_offset:u64,pub summary_offset:u32,pub adapter_id:u32 }
#[repr(C)] pub struct kvm_hv_sint { pub vcpu:u32,pub sint:u32 }
#[repr(C)] pub struct kvm_xen_evtchn { pub port:u32,pub vcpu_id:u32,pub vcpu_idx:c_int,pub priority:u32 }
#[repr(C)] pub enum kvm_bus { KVM_MMIO_BUS, KVM_PIO_BUS, KVM_VIRTIO_CCW_NOTIFY_BUS, KVM_FAST_MMIO_BUS, KVM_IOCSR_BUS, KVM_NR_BUSES }
#[repr(C)] pub enum kvm_gfn_range_filter { KVM_FILTER_SHARED=1, KVM_FILTER_PRIVATE=2 }
#[repr(C)] pub enum kvm_mr_change { KVM_MR_CREATE, KVM_MR_DELETE, KVM_MR_MOVE, KVM_MR_FLAGS_ONLY }
#[repr(C)] pub enum kvm_stat_kind { KVM_STAT_VM, KVM_STAT_VCPU }
pub const OUTSIDE_GUEST_MODE: c_int=0; pub const IN_GUEST_MODE:c_int=1; pub const EXITING_GUEST_MODE:c_int=2; pub const READING_SHADOW_PAGE_TABLES:c_int=3;

#[inline] pub fn kvm_vcpu_mapped(map: *const kvm_host_map) -> bool { unsafe { !(*map).hva.is_null() } }
#[inline] pub fn gfn_to_gpa(gfn:gfn_t)->gpa_t { gfn << 12 }
#[inline] pub fn gpa_to_gfn(gpa:gpa_t)->gfn_t { gpa >> 12 }
#[inline] pub fn pfn_to_hpa(pfn:kvm_pfn_t)->hpa_t { pfn << 12 }

extern "C" {
    pub fn kvm_make_vcpus_request_mask(kvm:*mut kvm, req:c_uint, bitmap:*mut c_ulong)->bool;
    pub fn kvm_make_all_cpus_request(kvm:*mut kvm, req:c_uint)->bool;
    pub fn kvm_io_bus_write(vcpu:*mut kvm_vcpu,bus:kvm_bus,addr:gpa_t,len:c_int,val:*const c_void)->c_int;
    pub fn kvm_io_bus_read(vcpu:*mut kvm_vcpu,bus:kvm_bus,addr:gpa_t,len:c_int,val:*mut c_void)->c_int;
    pub fn kvm_destroy_vcpus(kvm:*mut kvm); pub fn kvm_init(vcpu_size:c_uint,vcpu_align:c_uint,module:*mut module)->c_int; pub fn kvm_exit();
    pub fn kvm_get_vcpu(kvm:*mut kvm,i:c_int)->*mut kvm_vcpu;
    pub fn gfn_to_memslot(kvm:*mut kvm,gfn:gfn_t)->*mut kvm_memory_slot;
    pub fn kvm_read_guest(kvm:*mut kvm,gpa:gpa_t,data:*mut c_void,len:c_ulong)->c_int;
    pub fn kvm_write_guest(kvm:*mut kvm,gpa:gpa_t,data:*const c_void,len:c_ulong)->c_int;
    pub fn kvm_vcpu_map(vcpu:*mut kvm_vcpu,gfn:gfn_t,map:*mut kvm_host_map)->c_int;
    pub fn kvm_vcpu_unmap(vcpu:*mut kvm_vcpu,map:*mut kvm_host_map);
    pub fn kvm_arch_init_vm(kvm:*mut kvm,ty:c_ulong)->c_int; pub fn kvm_arch_destroy_vm(kvm:*mut kvm);
    pub fn kvm_irqfd(kvm:*mut kvm,args:*mut kvm_irqfd)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
