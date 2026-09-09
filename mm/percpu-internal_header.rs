/* SPDX-License-Identifier: GPL-2.0 */
// Translated from percpu-internal.h. Configuration symbols and external
// kernel definitions are supplied by the surrounding translation unit.

#[repr(C)]
pub struct pcpu_block_md {
    pub scan_hint: ::core::ffi::c_int,
    pub scan_hint_start: ::core::ffi::c_int,
    pub contig_hint: ::core::ffi::c_int,
    pub contig_hint_start: ::core::ffi::c_int,
    pub left_free: ::core::ffi::c_int,
    pub right_free: ::core::ffi::c_int,
    pub first_free: ::core::ffi::c_int,
    pub nr_bits: ::core::ffi::c_int,
}

#[repr(C)]
pub struct pcpuobj_ext {
    // Present when CONFIG_MEMCG is enabled.
    #[cfg(CONFIG_MEMCG)]
    pub cgroup: *mut obj_cgroup,
    // Present when CONFIG_MEM_ALLOC_PROFILING is enabled.
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)]
    pub tag: codetag_ref,
}

#[repr(C)]
pub struct pcpu_chunk {
    #[cfg(CONFIG_PERCPU_STATS)]
    pub nr_alloc: ::core::ffi::c_int,
    #[cfg(CONFIG_PERCPU_STATS)]
    pub max_alloc_size: usize,
    pub list: list_head,
    pub free_bytes: ::core::ffi::c_int,
    pub chunk_md: pcpu_block_md,
    pub bound_map: *mut ::core::ffi::c_ulong,
    pub base_addr: *mut ::core::ffi::c_void,
    pub alloc_map: *mut ::core::ffi::c_ulong,
    pub md_blocks: *mut pcpu_block_md,
    pub data: *mut ::core::ffi::c_void,
    pub immutable: bool,
    pub isolated: bool,
    pub start_offset: ::core::ffi::c_int,
    pub end_offset: ::core::ffi::c_int,
    pub nr_pages: ::core::ffi::c_int,
    pub nr_populated: ::core::ffi::c_int,
    pub nr_empty_pop_pages: ::core::ffi::c_int,
    #[cfg(any(CONFIG_MEMCG, CONFIG_MEM_ALLOC_PROFILING))]
    pub obj_exts: *mut pcpuobj_ext,
    pub populated: [::core::ffi::c_ulong; 0],
}

#[inline]
pub unsafe fn need_pcpuobj_ext() -> bool {
    if cfg!(CONFIG_MEM_ALLOC_PROFILING) {
        return true;
    }
    if !mem_cgroup_kmem_disabled() {
        return true;
    }
    false
}

extern "C" {
    pub static mut pcpu_lock: spinlock_t;
    pub static mut pcpu_chunk_lists: *mut list_head;
    pub static mut pcpu_nr_slots: ::core::ffi::c_int;
    pub static mut pcpu_sidelined_slot: ::core::ffi::c_int;
    pub static mut pcpu_to_depopulate_slot: ::core::ffi::c_int;
    pub static mut pcpu_nr_empty_pop_pages: ::core::ffi::c_int;
    pub static mut pcpu_first_chunk: *mut pcpu_chunk;
    pub static mut pcpu_reserved_chunk: *mut pcpu_chunk;
}

#[inline]
pub unsafe fn pcpu_chunk_nr_blocks(chunk: *mut pcpu_chunk) -> ::core::ffi::c_int {
    (*chunk).nr_pages * PAGE_SIZE / PCPU_BITMAP_BLOCK_SIZE
}

#[inline]
pub fn pcpu_nr_pages_to_map_bits(pages: ::core::ffi::c_int) -> ::core::ffi::c_int {
    pages * PAGE_SIZE / PCPU_MIN_ALLOC_SIZE
}

#[inline]
pub unsafe fn pcpu_chunk_map_bits(chunk: *mut pcpu_chunk) -> ::core::ffi::c_int {
    pcpu_nr_pages_to_map_bits((*chunk).nr_pages)
}

#[inline]
pub unsafe fn pcpu_obj_full_size(size: usize) -> usize {
    let mut extra_size: usize = 0;
    #[cfg(CONFIG_MEMCG)]
    if !mem_cgroup_kmem_disabled() {
        extra_size += size / PCPU_MIN_ALLOC_SIZE as usize * core::mem::size_of::<*mut obj_cgroup>();
    }
    size * num_possible_cpus() + extra_size
}

#[cfg(CONFIG_PERCPU_STATS)]
#[repr(C)]
pub struct percpu_stats {
    pub nr_alloc: u64,
    pub nr_dealloc: u64,
    pub nr_cur_alloc: u64,
    pub nr_max_alloc: u64,
    pub nr_chunks: u32,
    pub nr_max_chunks: u32,
    pub min_alloc_size: usize,
    pub max_alloc_size: usize,
}

#[cfg(CONFIG_PERCPU_STATS)]
extern "C" {
    pub static mut pcpu_stats: percpu_stats;
    pub static mut pcpu_stats_ai: pcpu_alloc_info;
}

#[cfg(CONFIG_PERCPU_STATS)]
#[inline]
pub unsafe fn pcpu_stats_save_ai(ai: *const pcpu_alloc_info) {
    ::core::ptr::copy_nonoverlapping(ai, &mut pcpu_stats_ai, 1);
    pcpu_stats.min_alloc_size = pcpu_stats_ai.unit_size;
}

#[cfg(CONFIG_PERCPU_STATS)]
#[inline]
pub unsafe fn pcpu_stats_area_alloc(chunk: *mut pcpu_chunk, size: usize) {
    lockdep_assert_held(&pcpu_lock);
    pcpu_stats.nr_alloc += 1;
    pcpu_stats.nr_cur_alloc += 1;
    pcpu_stats.nr_max_alloc = pcpu_stats.nr_max_alloc.max(pcpu_stats.nr_cur_alloc);
    pcpu_stats.min_alloc_size = pcpu_stats.min_alloc_size.min(size);
    pcpu_stats.max_alloc_size = pcpu_stats.max_alloc_size.max(size);
    (*chunk).nr_alloc += 1;
    (*chunk).max_alloc_size = (*chunk).max_alloc_size.max(size);
}

#[cfg(CONFIG_PERCPU_STATS)]
#[inline]
pub unsafe fn pcpu_stats_area_dealloc(chunk: *mut pcpu_chunk) {
    lockdep_assert_held(&pcpu_lock);
    pcpu_stats.nr_dealloc += 1;
    pcpu_stats.nr_cur_alloc -= 1;
    (*chunk).nr_alloc -= 1;
}

#[cfg(CONFIG_PERCPU_STATS)]
#[inline]
pub unsafe fn pcpu_stats_chunk_alloc() {
    let mut flags: ::core::ffi::c_ulong = 0;
    spin_lock_irqsave(&mut pcpu_lock, &mut flags);
    pcpu_stats.nr_chunks += 1;
    pcpu_stats.nr_max_chunks = pcpu_stats.nr_max_chunks.max(pcpu_stats.nr_chunks);
    spin_unlock_irqrestore(&mut pcpu_lock, flags);
}

#[cfg(CONFIG_PERCPU_STATS)]
#[inline]
pub unsafe fn pcpu_stats_chunk_dealloc() {
    let mut flags: ::core::ffi::c_ulong = 0;
    spin_lock_irqsave(&mut pcpu_lock, &mut flags);
    pcpu_stats.nr_chunks -= 1;
    spin_unlock_irqrestore(&mut pcpu_lock, flags);
}

#[cfg(not(CONFIG_PERCPU_STATS))]
#[inline]
pub unsafe fn pcpu_stats_save_ai(_ai: *const pcpu_alloc_info) {}
#[cfg(not(CONFIG_PERCPU_STATS))]
#[inline]
pub unsafe fn pcpu_stats_area_alloc(_chunk: *mut pcpu_chunk, _size: usize) {}
#[cfg(not(CONFIG_PERCPU_STATS))]
#[inline]
pub unsafe fn pcpu_stats_area_dealloc(_chunk: *mut pcpu_chunk) {}
#[cfg(not(CONFIG_PERCPU_STATS))]
#[inline]
pub unsafe fn pcpu_stats_chunk_alloc() {}
#[cfg(not(CONFIG_PERCPU_STATS))]
#[inline]
pub unsafe fn pcpu_stats_chunk_dealloc() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
