/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <asm/page.h>

#[repr(C)]
pub struct mmu_psize_def {
    pub shift: ::core::ffi::c_uint,
    pub penc: [::core::ffi::c_int; MMU_PAGE_COUNT],
    pub tlbiel: ::core::ffi::c_uint,
    pub avpnm: ::core::ffi::c_ulong,
    pub h_rpt_pgsize: ::core::ffi::c_ulong,
    pub sllp_ap: mmu_psize_def__sllp_ap,
}

#[repr(C)]
pub union mmu_psize_def__sllp_ap {
    pub sllp: ::core::ffi::c_ulong,
    pub ap: ::core::ffi::c_ulong,
}

extern "C" {
    pub static mut mmu_psize_defs: [mmu_psize_def; MMU_PAGE_COUNT];
}

// 64-bit classic hash table MMU; C dependency: <asm/book3s/64/mmu-hash.h>

#[repr(C)]
pub struct prtb_entry { pub prtb0: __be64, pub prtb1: __be64 }
extern "C" { pub static mut process_tb: *mut prtb_entry; }

#[repr(C)]
pub struct patb_entry { pub patb0: __be64, pub patb1: __be64 }
extern "C" { pub static mut partition_tb: *mut patb_entry; }

pub const PATB_HR: ::core::ffi::c_ulong = 1u64 << 63;
pub const RPDB_MASK: ::core::ffi::c_ulong = 0x0fffffffffffff00;
pub const RPDB_SHIFT: ::core::ffi::c_ulong = 1u64 << 8;
pub const RTS1_SHIFT: u32 = 61;
pub const RTS1_MASK: ::core::ffi::c_ulong = 3u64 << RTS1_SHIFT;
pub const RTS2_SHIFT: u32 = 5;
pub const RTS2_MASK: ::core::ffi::c_ulong = 7u64 << RTS2_SHIFT;
pub const RPDS_MASK: ::core::ffi::c_ulong = 0x1f;
pub const PATB_GR: ::core::ffi::c_ulong = 1u64 << 63;
pub const PRTS_MASK: ::core::ffi::c_ulong = 0x1f;
pub const PRTB_MASK: ::core::ffi::c_ulong = 0x0ffffffffffff000;

extern "C" {
    pub static mut mmu_lpid_bits: ::core::ffi::c_uint;
    pub static mut mmu_pid_bits: ::core::ffi::c_uint;
    pub static mut mmu_base_pid: ::core::ffi::c_uint;
    pub static mut memory_block_size: ::core::ffi::c_ulong;
}

// These are C macros depending on runtime globals.
#[inline] pub unsafe fn PRTB_SIZE_SHIFT() -> ::core::ffi::c_uint { mmu_pid_bits + 4 }
#[inline] pub unsafe fn PRTB_ENTRIES() -> ::core::ffi::c_ulong { 1u64 << mmu_pid_bits }
#[inline] pub unsafe fn PATB_SIZE_SHIFT() -> ::core::ffi::c_uint { mmu_lpid_bits + 4 }
#[inline] pub unsafe fn PATB_ENTRIES() -> ::core::ffi::c_ulong { 1u64 << mmu_lpid_bits }

pub type mm_context_id_t = ::core::ffi::c_ulong;
pub struct spinlock;
pub const NV_MAX_NPUS: usize = 8;

#[repr(C)]
pub struct mm_context_t {
    pub id_or_extended_id: mm_context_t__id_or_extended_id,
    pub active_cpus: atomic_t,
    pub copros: atomic_t,
    pub vas_windows: atomic_t,
    pub hash_context: *mut hash_mm_context,
    pub vdso: *mut ::core::ffi::c_void,
    pub pte_frag: *mut ::core::ffi::c_void,
    pub pmd_frag: *mut ::core::ffi::c_void,
    pub iommu_group_mem_list: list_head,
    pub pkey_allocation_map: u32,
    pub execute_only_pkey: i16,
}

#[repr(C)]
pub union mm_context_t__id_or_extended_id {
    pub id: mm_context_id_t,
    pub extended_id: [mm_context_id_t; TASK_SIZE_USER64 / TASK_CONTEXT_SIZE],
}

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn mm_ctx_user_psize(ctx: *mut mm_context_t) -> u16 { (*(*ctx).hash_context).user_psize }
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn mm_ctx_set_user_psize(ctx: *mut mm_context_t, v: u16) { (*(*ctx).hash_context).user_psize = v; }
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn mm_ctx_low_slices(ctx: *mut mm_context_t) -> *mut u8 { (*(*ctx).hash_context).low_slices_psize }
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn mm_ctx_high_slices(ctx: *mut mm_context_t) -> *mut u8 { (*(*ctx).hash_context).high_slices_psize }
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn mm_ctx_slb_addr_limit(ctx: *mut mm_context_t) -> ::core::ffi::c_ulong { (*(*ctx).hash_context).slb_addr_limit }
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn mm_ctx_set_slb_addr_limit(ctx: *mut mm_context_t, v: ::core::ffi::c_ulong) { (*(*ctx).hash_context).slb_addr_limit = v; }

extern "C" {
    pub fn mmu_early_init_devtree();
    pub fn hash__early_init_devtree();
    pub fn radix__early_init_devtree();
    pub fn hash__early_init_mmu();
    pub fn radix__early_init_mmu();
    pub fn hash__early_init_mmu_secondary();
    pub fn radix__early_init_mmu_secondary();
    pub fn hash__setup_initial_memory_limit(first_memblock_base: phys_addr_t, first_memblock_size: phys_addr_t);
    pub fn early_radix_enabled() -> bool;
    pub fn radix_enabled() -> bool;
}

#[inline] pub unsafe fn early_init_mmu() { if radix_enabled() { radix__early_init_mmu() } else { hash__early_init_mmu() } }
#[inline] pub unsafe fn early_init_mmu_secondary() { if radix_enabled() { radix__early_init_mmu_secondary() } else { hash__early_init_mmu_secondary() } }
#[inline] pub unsafe fn setup_initial_memory_limit(a: phys_addr_t, s: phys_addr_t) { if !early_radix_enabled() { hash__setup_initial_memory_limit(a, s); } }

extern "C" {
    pub static mut mmu_linear_psize: ::core::ffi::c_int;
    pub static mut mmu_vmemmap_psize: ::core::ffi::c_int;
}

#[cfg(CONFIG_PPC_PSERIES)] extern "C" { pub fn radix_init_pseries(); }

#[cfg(CONFIG_HOTPLUG_CPU)] extern "C" { pub fn cleanup_cpu_mmu_context(); }

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn get_user_context(ctx: *mut mm_context_t, ea: ::core::ffi::c_ulong) -> mm_context_id_t {
    let index = ea >> MAX_EA_BITS_PER_CONTEXT;
    if index < (TASK_SIZE_USER64 / TASK_CONTEXT_SIZE) { return (*ctx).id_or_extended_id.extended_id[index]; }
    0
}
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
#[inline] pub unsafe fn get_user_vsid(ctx: *mut mm_context_t, ea: ::core::ffi::c_ulong, ssize: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    get_vsid(get_user_context(ctx, ea), ea, ssize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
