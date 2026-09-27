/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * C dependencies supplied by the surrounding translation unit:
 * asm/kvm_pgtable.h, asm/spectre.h, linux/memblock.h, linux/types.h,
 * nvhe/memory.h, and nvhe/spinlock.h.
 */

#[repr(C)]
pub struct kvm_pgtable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hyp_spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub enum arm64_hyp_spectre_vector {}

#[repr(C)]
pub enum kvm_pgtable_prot {}

pub type phys_addr_t = u64;

extern "C" {
    pub static mut pkvm_pgtable: kvm_pgtable;
    pub static mut pkvm_pgd_lock: hyp_spinlock_t;

    pub fn hyp_create_fixmap() -> ::kernel::ffi::c_int;
    pub fn hyp_fixmap_map(phys: phys_addr_t) -> *mut ::kernel::ffi::c_void;
    pub fn hyp_fixmap_unmap();
    pub fn hyp_fixblock_map(
        phys: phys_addr_t,
        size: *mut usize,
    ) -> *mut ::kernel::ffi::c_void;
    pub fn hyp_fixblock_unmap();

    pub fn hyp_create_idmap(hyp_va_bits: u32) -> ::kernel::ffi::c_int;
    pub fn hyp_map_vectors() -> ::kernel::ffi::c_int;
    pub fn hyp_back_vmemmap(back: phys_addr_t) -> ::kernel::ffi::c_int;
    pub fn pkvm_cpu_set_vector(
        slot: arm64_hyp_spectre_vector,
    ) -> ::kernel::ffi::c_int;
    pub fn pkvm_create_mappings(
        from: *mut ::kernel::ffi::c_void,
        to: *mut ::kernel::ffi::c_void,
        prot: kvm_pgtable_prot,
    ) -> ::kernel::ffi::c_int;
    pub fn pkvm_create_mappings_locked(
        from: *mut ::kernel::ffi::c_void,
        to: *mut ::kernel::ffi::c_void,
        prot: kvm_pgtable_prot,
    ) -> ::kernel::ffi::c_int;
    pub fn __pkvm_create_private_mapping(
        phys: phys_addr_t,
        size: usize,
        prot: kvm_pgtable_prot,
        haddr: *mut ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
    pub fn pkvm_create_stack(
        phys: phys_addr_t,
        haddr: *mut ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
    pub fn pkvm_alloc_private_va_range(
        size: usize,
        haddr: *mut ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
