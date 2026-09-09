/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/rcupdate.h, linux/atomic.h, and swap.h.

#[repr(C)]
pub struct swap_table {
    pub entries: [atomic_long_t; SWAPFILE_CLUSTER],
}

#[repr(C)]
pub struct swap_memcg_table {
    pub id: [u16; SWAPFILE_CLUSTER],
}

pub const SWP_TABLE_USE_PAGE: bool = core::mem::size_of::<swap_table>() == PAGE_SIZE;

pub const SWP_TB_NULL: usize = 0;
pub const SWP_TB_SHADOW_MARK: usize = 0b1;
pub const SWP_TB_PFN_BITS: usize = SWAP_CACHE_PFN_BITS + SWAP_CACHE_PFN_MARK_BITS;
pub const SWP_TB_PFN_MARK: usize = 0b10;
pub const SWP_TB_PFN_MARK_MASK: usize = (1usize << SWAP_CACHE_PFN_MARK_BITS) - 1;
pub const SWP_TB_FLAGS_BITS: usize = if 5 < BITS_PER_LONG - SWP_TB_PFN_BITS { 5 } else { BITS_PER_LONG - SWP_TB_PFN_BITS };
pub const SWP_TB_COUNT_BITS: usize = SWP_TB_FLAGS_BITS - SWAP_TABLE_HAS_ZEROFLAG;
pub const SWP_TB_FLAGS_MASK: usize = !((!0usize) >> SWP_TB_FLAGS_BITS);
pub const SWP_TB_COUNT_MASK: usize = !((!0usize) >> SWP_TB_COUNT_BITS);
pub const SWP_TB_FLAGS_SHIFT: usize = BITS_PER_LONG - SWP_TB_FLAGS_BITS;
pub const SWP_TB_COUNT_SHIFT: usize = BITS_PER_LONG - SWP_TB_COUNT_BITS;
pub const SWP_TB_COUNT_MAX: usize = (1usize << SWP_TB_COUNT_BITS) - 1;
pub const SWP_TB_ZERO_FLAG: usize = 1usize << (BITS_PER_LONG - SWP_TB_FLAGS_BITS);
pub const SWP_TB_BAD: usize = (!0usize) << 3;
pub const SWAP_COUNT_SHIFT: usize = SWP_TB_FLAGS_BITS;

#[inline]
pub fn null_to_swp_tb() -> usize { 0 }

#[inline]
pub fn __count_to_swp_tb(count: u8) -> usize {
    VM_WARN_ON!(count as usize > SWP_TB_COUNT_MAX);
    (count as usize) << SWP_TB_COUNT_SHIFT
}

#[inline]
pub fn __flags_to_swp_tb(flags: u8) -> usize {
    VM_WARN_ON!((flags >> SWP_TB_FLAGS_BITS) != 0);
    (flags as usize) << SWP_TB_FLAGS_SHIFT
}

#[inline]
pub unsafe fn pfn_to_swp_tb(pfn: usize, flags: u8) -> usize {
    let swp_tb = (pfn << SWAP_CACHE_PFN_MARK_BITS) | SWP_TB_PFN_MARK;
    VM_WARN_ON_ONCE!(swp_tb & SWP_TB_FLAGS_MASK != 0);
    swp_tb | __flags_to_swp_tb(flags)
}

#[inline]
pub unsafe fn folio_to_swp_tb(folio: *mut folio, flags: u8) -> usize {
    pfn_to_swp_tb(folio_pfn(folio), flags)
}

#[inline]
pub unsafe fn shadow_to_swp_tb(shadow: *mut core::ffi::c_void, flags: u8) -> usize {
    VM_WARN_ON_ONCE!(!shadow.is_null() && !xa_is_value(shadow));
    VM_WARN_ON_ONCE!(!shadow.is_null() && (shadow as usize & SWP_TB_FLAGS_MASK) != 0);
    shadow as usize | SWP_TB_SHADOW_MARK | __flags_to_swp_tb(flags)
}

#[inline]
pub fn swp_tb_is_null(swp_tb: usize) -> bool { swp_tb == 0 }
#[inline]
pub fn swp_tb_is_folio(swp_tb: usize) -> bool { (swp_tb & SWP_TB_PFN_MARK_MASK) == SWP_TB_PFN_MARK }
#[inline]
pub unsafe fn swp_tb_is_shadow(swp_tb: usize) -> bool { xa_is_value(swp_tb as *mut core::ffi::c_void) }
#[inline]
pub fn swp_tb_is_bad(swp_tb: usize) -> bool { swp_tb == SWP_TB_BAD }
#[inline]
pub unsafe fn swp_tb_is_countable(swp_tb: usize) -> bool {
    swp_tb_is_shadow(swp_tb) || swp_tb_is_folio(swp_tb) || swp_tb_is_null(swp_tb)
}

#[inline]
pub unsafe fn swp_tb_to_folio(swp_tb: usize) -> *mut folio {
    pfn_folio((swp_tb & !SWP_TB_FLAGS_MASK) >> SWAP_CACHE_PFN_MARK_BITS)
}

#[inline]
pub unsafe fn swp_tb_to_shadow(swp_tb: usize) -> *mut core::ffi::c_void {
    (swp_tb & !SWP_TB_FLAGS_MASK) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn __swp_tb_get_count(swp_tb: usize) -> u8 {
    ((swp_tb & SWP_TB_COUNT_MASK) >> SWP_TB_COUNT_SHIFT) as u8
}

#[inline]
pub unsafe fn __swp_tb_get_flags(swp_tb: usize) -> u8 {
    ((swp_tb & SWP_TB_FLAGS_MASK) >> SWP_TB_FLAGS_SHIFT) as u8
}

#[inline]
pub unsafe fn swp_tb_get_count(swp_tb: usize) -> i32 {
    if swp_tb_is_countable(swp_tb) { __swp_tb_get_count(swp_tb) as i32 } else { -EINVAL }
}

#[inline]
pub fn __swp_tb_mk_count(swp_tb: usize, count: i32) -> usize {
    (swp_tb & !SWP_TB_COUNT_MASK) | __count_to_swp_tb(count as u8)
}

#[inline]
pub unsafe fn __swap_table_set(ci: *mut swap_cluster_info, off: u32, swp_tb: usize) {
    let table = rcu_dereference_protected((*ci).table, true);
    atomic_long_set(table.add(off as usize), swp_tb);
}

#[inline]
pub unsafe fn __swap_table_xchg(ci: *mut swap_cluster_info, off: u32, swp_tb: usize) -> usize {
    let table = rcu_dereference_protected((*ci).table, true);
    atomic_long_xchg_relaxed(table.add(off as usize), swp_tb)
}

#[inline]
pub unsafe fn __swap_table_get(ci: *mut swap_cluster_info, off: u32) -> usize {
    let table = rcu_dereference_check((*ci).table, lockdep_is_held(&(*ci).lock));
    atomic_long_read(table.add(off as usize))
}

#[inline]
pub unsafe fn swap_table_get(ci: *mut swap_cluster_info, off: u32) -> usize {
    rcu_read_lock();
    let table = rcu_dereference((*ci).table);
    let swp_tb = if !table.is_null() { atomic_long_read(table.add(off as usize)) } else { null_to_swp_tb() };
    rcu_read_unlock();
    swp_tb
}

#[cfg(SWAP_TABLE_HAS_ZEROFLAG)]
#[inline]
pub unsafe fn __swap_table_set_zero(ci: *mut swap_cluster_info, ci_off: u32) {
    let mut swp_tb = __swap_table_get(ci, ci_off);
    swp_tb |= SWP_TB_ZERO_FLAG;
    __swap_table_set(ci, ci_off, swp_tb);
}

#[cfg(not(SWAP_TABLE_HAS_ZEROFLAG))]
#[inline]
pub unsafe fn __swap_table_set_zero(ci: *mut swap_cluster_info, ci_off: u32) { __set_bit(ci_off, (*ci).zero_bitmap); }

#[cfg(SWAP_TABLE_HAS_ZEROFLAG)]
#[inline]
pub unsafe fn __swap_table_test_zero(ci: *mut swap_cluster_info, ci_off: u32) -> bool { (__swap_table_get(ci, ci_off) & SWP_TB_ZERO_FLAG) != 0 }

#[cfg(not(SWAP_TABLE_HAS_ZEROFLAG))]
#[inline]
pub unsafe fn __swap_table_test_zero(ci: *mut swap_cluster_info, ci_off: u32) -> bool { test_bit(ci_off, (*ci).zero_bitmap) }

#[cfg(SWAP_TABLE_HAS_ZEROFLAG)]
#[inline]
pub unsafe fn __swap_table_clear_zero(ci: *mut swap_cluster_info, ci_off: u32) {
    let swp_tb = __swap_table_get(ci, ci_off) & !SWP_TB_ZERO_FLAG;
    __swap_table_set(ci, ci_off, swp_tb);
}

#[cfg(not(SWAP_TABLE_HAS_ZEROFLAG))]
#[inline]
pub unsafe fn __swap_table_clear_zero(ci: *mut swap_cluster_info, ci_off: u32) { __clear_bit(ci_off, (*ci).zero_bitmap); }

#[cfg(CONFIG_MEMCG)]
#[inline]
pub unsafe fn __swap_cgroup_set(ci: *mut swap_cluster_info, mut ci_off: u32, mut nr: usize, id: u16) {
    while { (*ci).memcg_table.as_mut().unwrap().id[ci_off as usize] = id; ci_off += 1; nr -= 1; nr != 0 } {}
}

#[cfg(CONFIG_MEMCG)]
#[inline]
pub unsafe fn __swap_cgroup_get(ci: *mut swap_cluster_info, ci_off: u32) -> u16 {
    if (*ci).memcg_table.is_null() { 0 } else { (*ci).memcg_table.as_ref().unwrap().id[ci_off as usize] }
}

#[cfg(CONFIG_MEMCG)]
#[inline]
pub unsafe fn __swap_cgroup_clear(ci: *mut swap_cluster_info, mut ci_off: u32, mut nr: usize) -> u16 {
    let old = __swap_cgroup_get(ci, ci_off);
    if old == 0 { return 0; }
    while { (*ci).memcg_table.as_mut().unwrap().id[ci_off as usize] = 0; ci_off += 1; nr -= 1; nr != 0 } {}
    old
}

#[cfg(not(CONFIG_MEMCG))]
#[inline]
pub fn __swap_cgroup_set(_ci: *mut swap_cluster_info, _ci_off: u32, _nr: usize, _id: u16) {}
#[cfg(not(CONFIG_MEMCG))]
#[inline]
pub fn __swap_cgroup_get(_ci: *mut swap_cluster_info, _ci_off: u32) -> u16 { 0 }
#[cfg(not(CONFIG_MEMCG))]
#[inline]
pub fn __swap_cgroup_clear(_ci: *mut swap_cluster_info, _ci_off: u32, _nr: usize) -> u16 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
