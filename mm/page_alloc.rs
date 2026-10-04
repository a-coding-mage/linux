// SPDX-License-Identifier: GPL-2.0-only
//! Rust owner of the zoned buddy allocator. Layouts and constants come from the
//! configured native kernel headers; header/compiler primitives are ABI leaves.
#![cfg_attr(CONFIG_KMSAN, feature(no_sanitize))]
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    missing_docs,
    unreachable_pub,
    improper_ctypes,
    unsafe_op_in_unsafe_fn
)]
#[allow(clippy::all)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/page_alloc_generated.rs"
    ));
}
include!("page_alloc_diagnostics.rs");
use bindings as b;
use bindings::*;
use core::cmp::{max, min};
use core::mem::{align_of, offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char as CChar, c_long as Long, c_ulong as ULong, c_void as Void};
type fpi_t = i32;
const FPI_NONE: fpi_t = 0;
const FPI_SKIP_REPORT_NOTIFY: fpi_t = 1;
const FPI_TO_TAIL: fpi_t = 2;
const FPI_NOLOCK: fpi_t = 4;
const FPI_PREPARED: fpi_t = 8;
const MIN_PERCPU_PAGELIST_HIGH_FRACTION: i32 = 8;
const ALLOC_STALL_WARN_MSECS: ULong = 10 * 1000;

const _: () = {
    assert!(size_of::<page>() == RUST_PA_SIZEOF_PAGE as usize);
    assert!(align_of::<page>() == RUST_PA_ALIGNOF_PAGE as usize);
    assert!(size_of::<zone>() == RUST_PA_SIZEOF_ZONE as usize);
    assert!(size_of::<per_cpu_pages>() == RUST_PA_SIZEOF_PCP as usize);
    assert!(RUST_PA_NR_PAGEBLOCK_BITS == if cfg!(CONFIG_MEMORY_ISOLATION) { 8 } else { 4 });
};

#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut node_states: [nodemask_t; NR_NODE_STATES as usize] = {
    let mut states: [nodemask_t; NR_NODE_STATES as usize] = unsafe { zeroed() };
    let mut n = 0;
    while n < RUST_PA_MAX_NUMNODES as usize {
        states[N_POSSIBLE as usize].bits[n / ULong::BITS as usize] |=
            1 << (n % ULong::BITS as usize);
        n += 1;
    }
    states[N_ONLINE as usize].bits[0] = 1;
    #[cfg(not(CONFIG_NUMA))]
    {
        states[N_NORMAL_MEMORY as usize].bits[0] = 1;
        #[cfg(CONFIG_HIGHMEM)]
        {
            states[N_HIGH_MEMORY as usize].bits[0] = 1;
        }
        states[N_MEMORY as usize].bits[0] = 1;
        states[N_CPU as usize].bits[0] = 1;
    }
    states
};
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut gfp_allowed_mask: gfp_t = RUST_PA_GFP_BOOT_MASK as gfp_t;
#[cfg(CONFIG_HUGETLB_PAGE_SIZE_VARIABLE)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut pageblock_order: u32 = 0;
static mut sysctl_lowmem_reserve_ratio: [i32; RUST_PA_MAX_NR_ZONES as usize] = {
    let mut ratios = [0; RUST_PA_MAX_NR_ZONES as usize];
    #[cfg(CONFIG_ZONE_DMA)]
    {
        ratios[ZONE_DMA as usize] = 256;
    }
    #[cfg(CONFIG_ZONE_DMA32)]
    {
        ratios[ZONE_DMA32 as usize] = 256;
    }
    ratios[ZONE_NORMAL as usize] = 32;
    ratios
};
#[repr(transparent)]
pub struct PaNames<const N: usize>(pub [*const CChar; N]);
unsafe impl<const N: usize> Sync for PaNames<N> {}
#[no_mangle]
pub static zone_names: PaNames<{ RUST_PA_MAX_NR_ZONES as usize }> = PaNames([
    #[cfg(CONFIG_ZONE_DMA)]
    b"DMA\0".as_ptr().cast(),
    #[cfg(CONFIG_ZONE_DMA32)]
    b"DMA32\0".as_ptr().cast(),
    b"Normal\0".as_ptr().cast(),
    #[cfg(CONFIG_HIGHMEM)]
    b"HighMem\0".as_ptr().cast(),
    b"Movable\0".as_ptr().cast(),
    #[cfg(CONFIG_ZONE_DEVICE)]
    b"Device\0".as_ptr().cast(),
]);
#[no_mangle]
pub static migratetype_names: PaNames<{ MIGRATE_TYPES as usize }> = PaNames([
    b"Unmovable\0".as_ptr().cast(),
    b"Movable\0".as_ptr().cast(),
    b"Reclaimable\0".as_ptr().cast(),
    b"HighAtomic\0".as_ptr().cast(),
    #[cfg(CONFIG_CMA)]
    b"CMA\0".as_ptr().cast(),
    #[cfg(CONFIG_MEMORY_ISOLATION)]
    b"Isolate\0".as_ptr().cast(),
]);
#[no_mangle]
pub static mut min_free_kbytes: i32 = 1024;
#[no_mangle]
pub static mut user_min_free_kbytes: i32 = -1;
#[link_section = ".data..read_mostly"]
static mut watermark_boost_factor: i32 = 15000;
static mut watermark_scale_factor: i32 = 10;
#[no_mangle]
pub static mut defrag_mode: i32 = 0;
#[no_mangle]
pub static mut movable_zone: i32 = 0;
#[cfg(CONFIG_NUMA)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut nr_node_ids: u32 = RUST_PA_MAX_NUMNODES as u32;
#[cfg(CONFIG_NUMA)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut nr_online_nodes: u32 = 1;
static mut alloc_stall_warn_jiffies: ULong = RUST_PA_INITIAL_JIFFIES as ULong;
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut page_group_by_mobility_disabled: i32 = 0;

#[inline]
unsafe fn pb_order() -> u32 {
    rust_pa_pageblock_order()
}
#[inline]
unsafe fn pb_pages() -> ULong {
    1 << pb_order()
}
#[inline]
unsafe fn pb_start(pfn: ULong) -> ULong {
    pfn & !(pb_pages() - 1)
}
#[inline]
unsafe fn pb_end(pfn: ULong) -> ULong {
    pfn.wrapping_add(pb_pages()) & !(pb_pages() - 1)
}
#[inline]
unsafe fn zone_spans(z: *const zone, pfn: ULong) -> bool {
    pfn.wrapping_sub((*z).zone_start_pfn) < rust_pa_read_ulong(addr_of!((*z).spanned_pages))
}
#[inline]
unsafe fn is_migrate_cma(mt: i32) -> bool {
    #[cfg(CONFIG_CMA)]
    {
        return mt == MIGRATE_CMA as i32;
    }
    #[cfg(not(CONFIG_CMA))]
    {
        false
    }
}
#[inline]
unsafe fn is_migrate_isolate(mt: i32) -> bool {
    #[cfg(CONFIG_MEMORY_ISOLATION)]
    {
        return mt == MIGRATE_ISOLATE as i32;
    }
    #[cfg(not(CONFIG_MEMORY_ISOLATION))]
    {
        false
    }
}
#[inline]
fn migratetype_is_mergeable(mt: i32) -> bool {
    mt < MIGRATE_PCPTYPES as i32
}
#[cfg(CONFIG_DEFERRED_STRUCT_PAGE_INIT)]
#[link_section = ".ref.text"]
#[inline(never)]
unsafe fn _deferred_grow_zone(z: *mut zone, order: u32) -> bool {
    deferred_grow_zone(z, order)
}
#[cfg(not(CONFIG_DEFERRED_STRUCT_PAGE_INIT))]
unsafe fn _deferred_grow_zone(_z: *mut zone, _order: u32) -> bool {
    false
}
#[inline]
unsafe fn deferred_pages_enabled() -> bool {
    rust_pa_deferred_pages_enabled()
}

#[inline]
unsafe fn pcpu_task_pin() {
    #[cfg(CONFIG_PREEMPT_RT)]
    {
        rust_pa_migrate_disable();
    }
    #[cfg(not(CONFIG_PREEMPT_RT))]
    {
        rust_pa_preempt_disable();
    }
}
#[inline]
unsafe fn pcpu_task_unpin() {
    #[cfg(CONFIG_PREEMPT_RT)]
    {
        rust_pa_migrate_enable();
    }
    #[cfg(not(CONFIG_PREEMPT_RT))]
    {
        rust_pa_preempt_enable();
    }
}
#[inline]
unsafe fn pcp_spin_trylock(ptr: *mut per_cpu_pages) -> *mut per_cpu_pages {
    #[cfg(CONFIG_SMP)]
    {
        pcpu_task_pin();
        let p = rust_pa_this_cpu_pcp(ptr);
        if !rust_pa_spin_trylock(addr_of_mut!((*p).lock)) {
            pcpu_task_unpin();
            return null_mut();
        }
        p
    }
    #[cfg(not(CONFIG_SMP))]
    {
        null_mut()
    }
}
#[inline]
unsafe fn pcp_spin_unlock(ptr: *mut per_cpu_pages) {
    #[cfg(CONFIG_SMP)]
    {
        rust_pa_spin_unlock(addr_of_mut!((*ptr).lock));
        pcpu_task_unpin();
    }
    #[cfg(not(CONFIG_SMP))]
    {
        rust_pa_bug(true);
    }
}
#[inline]
unsafe fn pcp_spin_lock_nopin(ptr: *mut per_cpu_pages) {
    rust_pa_spin_lock(addr_of_mut!((*ptr).lock));
}
#[inline]
unsafe fn pcp_spin_unlock_nopin(ptr: *mut per_cpu_pages) {
    rust_pa_spin_unlock(addr_of_mut!((*ptr).lock));
}

#[inline]
unsafe fn get_pageblock_bitmap(p: *const page, pfn: ULong) -> *mut ULong {
    #[cfg(CONFIG_SPARSEMEM)]
    {
        rust_pa_section_usemap(pfn)
    }
    #[cfg(not(CONFIG_SPARSEMEM))]
    {
        (*rust_pa_page_zone(p)).pageblock_flags
    }
}
#[inline]
unsafe fn pfn_to_bitidx(p: *const page, mut pfn: ULong) -> i32 {
    #[cfg(CONFIG_SPARSEMEM)]
    {
        pfn &= RUST_PA_PAGES_PER_SECTION as ULong - 1;
    }
    #[cfg(not(CONFIG_SPARSEMEM))]
    {
        pfn = pfn.wrapping_sub(pb_start((*rust_pa_page_zone(p)).zone_start_pfn));
    }
    ((pfn >> pb_order()) * RUST_PA_NR_PAGEBLOCK_BITS as ULong) as i32
}
#[inline]
fn is_standalone_pb_bit(bit: pageblock_bits) -> bool {
    bit >= PB_compact_skip && bit < __NR_PAGEBLOCK_BITS
}
#[inline]
unsafe fn get_pfnblock_bitmap_bitidx(p: *const page, pfn: ULong) -> (*mut ULong, ULong) {
    pa_vm_bug_page!(!zone_spans(rust_pa_page_zone(p), pfn), p);
    let idx = pfn_to_bitidx(p, pfn) as ULong;
    (
        get_pageblock_bitmap(p, pfn).add((idx / ULong::BITS as ULong) as usize),
        idx & (ULong::BITS as ULong - 1),
    )
}
unsafe fn __get_pfnblock_flags_mask(p: *const page, pfn: ULong, mask: ULong) -> ULong {
    let (word, bit) = get_pfnblock_bitmap_bitidx(p, pfn);
    (rust_pa_read_ulong(word) >> bit) & mask
}
#[export_name = "rust_pa_impl_get_pfnblock_bit"]
pub unsafe extern "C" fn get_pfnblock_bit(p: *const page, pfn: ULong, bit: pageblock_bits) -> bool {
    if rust_pa_warn_get_pb_bit(!is_standalone_pb_bit(bit)) {
        return false;
    }
    let (word, idx) = get_pfnblock_bitmap_bitidx(p, pfn);
    rust_pa_test_bit(idx + bit as ULong, word)
}
#[export_name = "rust_pa_impl_get_pfnblock_migratetype"]
pub unsafe extern "C" fn get_pfnblock_migratetype(p: *const page, pfn: ULong) -> migratetype {
    let flags = __get_pfnblock_flags_mask(
        p,
        pfn,
        RUST_PA_PAGEBLOCK_MIGRATETYPE_MASK as ULong | RUST_PA_PAGEBLOCK_ISO_MASK as ULong,
    );
    #[cfg(CONFIG_MEMORY_ISOLATION)]
    if flags & (1 << PB_migrate_isolate) != 0 {
        return MIGRATE_ISOLATE;
    }
    (flags & RUST_PA_PAGEBLOCK_MIGRATETYPE_MASK as ULong) as migratetype
}
unsafe fn __set_pfnblock_flags_mask(p: *const page, pfn: ULong, mut flags: ULong, mut mask: ULong) {
    let (word, idx) = get_pfnblock_bitmap_bitidx(p, pfn);
    mask <<= idx;
    flags <<= idx;
    let mut old = rust_pa_read_ulong(word);
    loop {
        let new = (old & !mask) | flags;
        if rust_pa_try_cmpxchg_ulong(word, &mut old, new) {
            break;
        }
    }
}
#[export_name = "rust_pa_impl_set_pfnblock_bit"]
pub unsafe extern "C" fn set_pfnblock_bit(p: *const page, pfn: ULong, bit: pageblock_bits) {
    if rust_pa_warn_set_pb_bit(!is_standalone_pb_bit(bit)) {
        return;
    }
    let (word, idx) = get_pfnblock_bitmap_bitidx(p, pfn);
    rust_pa_set_bit(idx + bit as ULong, word);
}
#[export_name = "rust_pa_impl_clear_pfnblock_bit"]
pub unsafe extern "C" fn clear_pfnblock_bit(p: *const page, pfn: ULong, bit: pageblock_bits) {
    if rust_pa_warn_clear_pb_bit(!is_standalone_pb_bit(bit)) {
        return;
    }
    let (word, idx) = get_pfnblock_bitmap_bitidx(p, pfn);
    rust_pa_clear_bit(idx + bit as ULong, word);
}
unsafe fn set_pageblock_migratetype(p: *mut page, mut mt: i32) {
    if page_group_by_mobility_disabled != 0 && mt < MIGRATE_PCPTYPES as i32 {
        mt = MIGRATE_UNMOVABLE as i32;
    }
    #[cfg(CONFIG_MEMORY_ISOLATION)]
    {
        if mt == MIGRATE_ISOLATE as i32 {
            rust_pa_warn_isolation(0);
            return;
        }
        if rust_pa_get_pageblock_isolate(p) {
            rust_pa_warn_isolation(1);
        }
    }
    __set_pfnblock_flags_mask(
        p,
        rust_pa_page_to_pfn(p),
        mt as ULong,
        RUST_PA_PAGEBLOCK_MIGRATETYPE_MASK as ULong | RUST_PA_PAGEBLOCK_ISO_MASK as ULong,
    );
}
#[export_name = "rust_pa_impl_init_pageblock_migratetype"]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
pub unsafe extern "C" fn init_pageblock_migratetype(
    p: *mut page,
    mut mt: migratetype,
    isolate: bool,
) {
    if page_group_by_mobility_disabled != 0 && mt < MIGRATE_PCPTYPES {
        mt = MIGRATE_UNMOVABLE;
    }
    let mut flags = mt as ULong;
    #[cfg(CONFIG_MEMORY_ISOLATION)]
    {
        if mt == MIGRATE_ISOLATE {
            rust_pa_warn_isolation(2);
            return;
        }
        if isolate {
            flags |= 1 << PB_migrate_isolate;
        }
    }
    __set_pfnblock_flags_mask(
        p,
        rust_pa_page_to_pfn(p),
        flags,
        RUST_PA_PAGEBLOCK_MIGRATETYPE_MASK as ULong | RUST_PA_PAGEBLOCK_ISO_MASK as ULong,
    );
}
#[cfg(CONFIG_DEBUG_VM)]
unsafe fn page_outside_zone_boundaries(z: *mut zone, p: *mut page) -> bool {
    let pfn = rust_pa_page_to_pfn(p);
    loop {
        let seq = rust_pa_zone_span_seqbegin(z);
        let start = (*z).zone_start_pfn;
        let sp = (*z).spanned_pages;
        let outside = !zone_spans(z, pfn);
        if rust_pa_zone_span_seqretry(z, seq) {
            continue;
        }
        if outside {
            rust_pa_bad_zone(pfn, z, start, start.wrapping_add(sp));
        }
        return outside;
    }
}
unsafe fn bad_range(z: *mut zone, p: *mut page) -> bool {
    #[cfg(CONFIG_DEBUG_VM)]
    {
        page_outside_zone_boundaries(z, p) || z != rust_pa_page_zone(p)
    }
    #[cfg(not(CONFIG_DEBUG_VM))]
    {
        false
    }
}
unsafe fn bad_page(p: *mut page, reason: *const CChar) {
    static mut RESUME: ULong = 0;
    static mut SHOWN: ULong = 0;
    static mut UNSHOWN: ULong = 0;
    let mut show = true;
    if SHOWN == 60 {
        if (rust_pa_jiffies().wrapping_sub(RESUME) as Long) < 0 {
            UNSHOWN = UNSHOWN.wrapping_add(1);
            show = false;
        } else {
            if UNSHOWN != 0 {
                rust_pa_bad_suppressed(UNSHOWN);
                UNSHOWN = 0;
            }
            SHOWN = 0;
        }
    }
    if show {
        let old = SHOWN;
        SHOWN = SHOWN.wrapping_add(1);
        if old == 0 {
            RESUME = rust_pa_jiffies().wrapping_add(60 * RUST_PA_HZ as ULong);
        }
        rust_pa_bad_process(p);
        dump_page(p, reason);
        print_modules();
        rust_pa_dump_stack();
    }
    if rust_pa_page_buddy(p) {
        rust_pa_clear_page_buddy(p);
    }
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
}
#[inline]
fn order_to_pindex(mt: i32, order: i32) -> u32 {
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    if order > RUST_PA_PAGE_ALLOC_COSTLY_ORDER as i32 {
        return RUST_PA_NR_LOWORDER_PCP_LISTS as u32 + (mt == MIGRATE_MOVABLE as i32) as u32;
    }
    (MIGRATE_PCPTYPES as i32 * order + mt) as u32
}
#[inline]
fn pindex_to_order(pindex: u32) -> i32 {
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    if pindex >= RUST_PA_NR_LOWORDER_PCP_LISTS as u32 {
        return RUST_PA_HPAGE_PMD_ORDER as i32;
    }
    (pindex / MIGRATE_PCPTYPES as u32) as i32
}
#[inline]
unsafe fn pcp_allowed_order(order: u32) -> bool {
    if order <= RUST_PA_PAGE_ALLOC_COSTLY_ORDER as u32 {
        return true;
    }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    if rust_pa_is_pmd_order(order) {
        return true;
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn prep_compound_page(p: *mut page, order: u32) {
    rust_pa_set_page_head(p);
    for i in 1..(1usize << order) {
        rust_pa_prep_compound_tail(p.add(i), p, order);
    }
    rust_pa_prep_compound_head(p, order);
}
#[inline]
unsafe fn set_buddy_order(p: *mut page, order: u32) {
    rust_pa_set_page_private(p, order as ULong);
    rust_pa_set_page_buddy(p);
}

include!("page_alloc_primitives.rs");
include!("page_alloc_buddy.rs");
include!("page_alloc_fastpaths.rs");
include!("page_alloc_late.rs");
include!("page_alloc_zone_tail.rs");
