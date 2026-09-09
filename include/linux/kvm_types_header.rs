/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * The C header includes and preprocessor export macros are represented by
 * references to the corresponding external Rust types and by comments below.
 */

/* KVM_SUB_MODULES and CONFIG_KVM are build-time configuration conditions. */

/* The C declarations below are excluded for assembler translation units. */
/* Forward declarations from the C header. */
pub enum kvm {}
pub enum kvm_async_pf {}
pub enum kvm_device_ops {}
pub enum kvm_gfn_range {}
pub enum kvm_interrupt {}
pub enum kvm_irq_routing_table {}
pub enum kvm_memory_slot {}
pub enum kvm_one_reg {}
pub enum kvm_run {}
pub enum kvm_userspace_memory_region {}
pub enum kvm_vcpu {}
pub enum kvm_vcpu_init {}
pub enum kvm_memslots {}

pub enum kvm_mr_change {}

    /*
     * Address types:
     *
     *  gva - guest virtual address
     *  gpa - guest physical address
     *  gfn - guest frame number
     *  hva - host virtual address
     *  hpa - host physical address
     *  hfn - host frame number
     */
pub type gva_t = ::std::os::raw::c_ulong;
pub type gpa_t = u64;
pub type gfn_t = u64;

pub const INVALID_GPA: gpa_t = !0 as gpa_t;

pub type hva_t = ::std::os::raw::c_ulong;
pub type hpa_t = u64;
pub type hfn_t = u64;

pub type kvm_pfn_t = hfn_t;

#[repr(C)]
pub struct gfn_to_hva_cache {
        pub generation: u64,
        pub gpa: gpa_t,
        pub hva: ::std::os::raw::c_ulong,
        pub len: ::std::os::raw::c_ulong,
        pub memslot: *mut kvm_memory_slot,
}

#[repr(C)]
pub struct gfn_to_pfn_cache {
        pub generation: u64,
        pub gpa: gpa_t,
        pub uhva: ::std::os::raw::c_ulong,
        pub memslot: *mut kvm_memory_slot,
        pub kvm: *mut kvm,
        pub list: list_head,
        pub lock: rwlock_t,
        pub refresh_lock: mutex,
        pub khva: *mut ::std::ffi::c_void,
        pub pfn: kvm_pfn_t,
        pub active: bool,
        pub valid: bool,
}

    /* Present only when KVM_ARCH_NR_OBJS_PER_MEMORY_CACHE is defined. */
#[cfg(KVM_ARCH_NR_OBJS_PER_MEMORY_CACHE)]
#[repr(C)]
pub struct kvm_mmu_memory_cache {
        pub gfp_zero: gfp_t,
        pub gfp_custom: gfp_t,
        pub init_value: u64,
        pub kmem_cache: *mut kmem_cache,
        pub capacity: ::std::os::raw::c_int,
        pub nobjs: ::std::os::raw::c_int,
        pub objects: *mut *mut ::std::ffi::c_void,
}

pub const HALT_POLL_HIST_COUNT: usize = 32;

#[repr(C)]
pub struct kvm_vm_stat_generic {
        pub remote_tlb_flush: u64,
        pub remote_tlb_flush_requests: u64,
}

#[repr(C)]
pub struct kvm_vcpu_stat_generic {
        pub halt_successful_poll: u64,
        pub halt_attempted_poll: u64,
        pub halt_poll_invalid: u64,
        pub halt_wakeup: u64,
        pub halt_poll_success_ns: u64,
        pub halt_poll_fail_ns: u64,
        pub halt_wait_ns: u64,
        pub halt_poll_success_hist: [u64; HALT_POLL_HIST_COUNT],
        pub halt_poll_fail_hist: [u64; HALT_POLL_HIST_COUNT],
        pub halt_wait_hist: [u64; HALT_POLL_HIST_COUNT],
        pub blocking: u64,
}

pub const KVM_STATS_NAME_SIZE: usize = 48;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
