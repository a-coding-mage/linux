// SPDX-License-Identifier: GPL-2.0-or-later
//! Rust owner of the early logical-memory allocator.
//!
//! Region storage and all allocator decisions live here. The companion C file
//! only exposes architecture/header inlines, compiler diagnostics and registration
//! macros. Bindgen reads the configured native headers; no struct layout is
//! duplicated in this owner. Early boot callers serialize the region arrays as
//! in memblock.c. Named reservations retain their separate runtime mutex.
#![allow(
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_mut,
    missing_docs,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]

#[allow(
    clippy::all,
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unsafe_op_in_unsafe_fn,
    unreachable_pub
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/memblock_generated.rs"
    ));
}
use bindings::*;
use core::cmp::{max, min};
use core::mem::{align_of, offset_of, size_of, zeroed};
use core::ptr::{null, null_mut};
use kernel::ffi::{c_char as CChar, c_ulong as ULong, c_void as Void};
type Phys = phys_addr_t;
type Flags = memblock_flags;
const PAGE_BYTES: Phys = RUST_MEMBLOCK_PAGE_SIZE as Phys;
const PAGE_BITS: u32 = RUST_MEMBLOCK_PAGE_SHIFT as u32;
const NO_NODE: i32 = RUST_MEMBLOCK_NUMA_NO_NODE as i32;
const MAX_NODES: i32 = RUST_MEMBLOCK_MAX_NUMNODES as i32;
const ANYWHERE: Phys = Phys::MAX;
const ACCESSIBLE: Phys = 0;
const NOLEAKTRACE: Phys = 1;

// __init_memblock and __initdata_memblock are retained for memory hotplug.
macro_rules! mb_fn {
    ($($item:item)*) => { $(
        #[cfg_attr(all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)), link_section = ".init.text")]
        $item
    )* };
}
macro_rules! info { ($fmt:expr $(, $arg:expr)* $(,)?) => { #[cfg(CONFIG_PRINTK)] { _printk($fmt.as_ptr().cast::<CChar>() $(, $arg)*); } }; }
macro_rules! warn {
    ($condition:expr) => {
        rust_memblock_warn($condition)
    };
}
macro_rules! bug {
    ($condition:expr) => {
        rust_memblock_bug($condition)
    };
}

const _: () = {
    assert!(size_of::<memblock_region>() == RUST_MEMBLOCK_REGION_SIZE as usize);
    assert!(align_of::<memblock_region>() == RUST_MEMBLOCK_REGION_ALIGN as usize);
    assert!(size_of::<memblock_type>() == RUST_MEMBLOCK_TYPE_SIZE as usize);
    assert!(align_of::<memblock_type>() == RUST_MEMBLOCK_TYPE_ALIGN as usize);
    assert!(size_of::<bindings::memblock>() == RUST_MEMBLOCK_SIZE as usize);
    assert!(offset_of!(memblock_type, cnt) == RUST_MEMBLOCK_CNT_OFFSET as usize);
    assert!(offset_of!(memblock_type, regions) == RUST_MEMBLOCK_REGIONS_OFFSET as usize);
};

#[cfg(not(CONFIG_NUMA))]
#[no_mangle]
#[link_section = ".ref.data"]
pub static mut contig_page_data: pglist_data = unsafe { zeroed() };
#[no_mangle]
pub static mut max_low_pfn: ULong = 0;
#[no_mangle]
pub static mut min_low_pfn: ULong = 0;
#[no_mangle]
pub static mut max_pfn: ULong = 0;
#[no_mangle]
pub static mut max_possible_pfn: u64 = 0;

#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
static mut MEMORY_REGIONS: [memblock_region; RUST_MEMBLOCK_INIT_MEMORY_REGIONS as usize] =
    unsafe { zeroed() };
#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
static mut RESERVED_REGIONS: [memblock_region; RUST_MEMBLOCK_INIT_RESERVED_REGIONS as usize] =
    unsafe { zeroed() };
#[cfg(CONFIG_HAVE_MEMBLOCK_PHYS_MAP)]
static mut PHYSMEM_REGIONS: [memblock_region; 4] = unsafe { zeroed() };
#[no_mangle]
#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
pub static mut memblock: bindings::memblock = bindings::memblock {
    bottom_up: false,
    current_limit: ANYWHERE,
    memory: memblock_type {
        cnt: 0,
        max: RUST_MEMBLOCK_INIT_MEMORY_REGIONS as ULong,
        total_size: 0,
        regions: (&raw mut MEMORY_REGIONS).cast::<memblock_region>(),
        name: c"memory".as_ptr().cast::<CChar>().cast_mut(),
    },
    reserved: memblock_type {
        cnt: 0,
        max: RUST_MEMBLOCK_INIT_RESERVED_REGIONS as ULong,
        total_size: 0,
        regions: (&raw mut RESERVED_REGIONS).cast::<memblock_region>(),
        name: c"reserved".as_ptr().cast::<CChar>().cast_mut(),
    },
};
#[cfg(CONFIG_HAVE_MEMBLOCK_PHYS_MAP)]
#[no_mangle]
pub static mut physmem: memblock_type = memblock_type {
    cnt: 0,
    max: 4,
    total_size: 0,
    regions: (&raw mut PHYSMEM_REGIONS).cast::<memblock_region>(),
    name: c"physmem".as_ptr().cast::<CChar>().cast_mut(),
};
#[link_section = ".ref.data"]
static mut memblock_memory: *mut memblock_type = unsafe { &raw mut memblock.memory };
#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
static mut memblock_debug: i32 = 0;
#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
static mut system_has_some_mirror: bool = false;
#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
static mut memblock_can_resize: bool = false;
#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
static mut memblock_memory_in_slab: bool = false;
#[cfg_attr(
    all(not(CONFIG_ARCH_KEEP_MEMBLOCK), not(CONFIG_MEMORY_HOTPLUG)),
    link_section = ".init.data"
)]
static mut memblock_reserved_in_slab: bool = false;
#[cfg(CONFIG_MEMBLOCK_KHO_SCRATCH)]
static mut kho_scratch_only: bool = false;

#[inline(always)]
unsafe fn node(r: *const memblock_region) -> i32 {
    #[cfg(CONFIG_NUMA)]
    {
        (*r).nid
    }
    #[cfg(not(CONFIG_NUMA))]
    {
        let _ = r;
        0
    }
}
#[inline(always)]
unsafe fn set_node(r: *mut memblock_region, nid: i32) {
    #[cfg(CONFIG_NUMA)]
    {
        (*r).nid = nid;
    }
    #[cfg(not(CONFIG_NUMA))]
    {
        let _ = (r, nid);
    }
}
#[inline(always)]
fn valid_node(nid: i32) -> bool {
    nid >= 0 && nid < MAX_NODES
}
#[inline(always)]
fn cap_size(base: Phys, size: Phys) -> Phys {
    min(size, Phys::MAX - base)
}
#[inline(always)]
fn round_up(value: Phys, align: Phys) -> Phys {
    (value.wrapping_sub(1) | align.wrapping_sub(1)).wrapping_add(1)
}
#[inline(always)]
fn round_down(value: Phys, align: Phys) -> Phys {
    value & !align.wrapping_sub(1)
}
#[inline(always)]
fn pfn_up(value: Phys) -> ULong {
    (value.wrapping_add(PAGE_BYTES - 1) >> PAGE_BITS) as ULong
}
#[inline(always)]
fn pfn_down(value: Phys) -> ULong {
    (value >> PAGE_BITS) as ULong
}
#[inline(always)]
fn pfn_phys(value: ULong) -> Phys {
    (value as Phys).wrapping_shl(PAGE_BITS)
}
#[inline(always)]
unsafe fn memory() -> *mut memblock_type {
    &raw mut memblock.memory
}
#[inline(always)]
unsafe fn reserved() -> *mut memblock_type {
    &raw mut memblock.reserved
}
#[inline(always)]
unsafe fn region(ty: *mut memblock_type, i: ULong) -> *mut memblock_region {
    (*ty).regions.add(i as usize)
}
#[inline(always)]
unsafe fn region_end(r: *const memblock_region) -> Phys {
    (*r).base.wrapping_add((*r).size)
}

mb_fn! {
#[no_mangle]
pub unsafe extern "C" fn memblock_has_mirror() -> bool { system_has_some_mirror }
unsafe fn choose_memblock_flags() -> Flags {
    #[cfg(CONFIG_MEMBLOCK_KHO_SCRATCH)]
    if kho_scratch_only { return MEMBLOCK_KHO_SCRATCH; }
    if system_has_some_mirror { MEMBLOCK_MIRROR } else { MEMBLOCK_NONE }
}
#[no_mangle]
pub unsafe extern "C" fn memblock_addrs_overlap(base1: Phys, size1: Phys, base2: Phys, size2: Phys) -> ULong {
    ((base1 < base2.wrapping_add(size2)) && (base2 < base1.wrapping_add(size1))) as ULong
}
#[no_mangle]
pub unsafe extern "C" fn memblock_overlaps_region(ty: *mut memblock_type, base: Phys, size: Phys) -> bool {
    let size = cap_size(base, size);
    for i in 0..(*ty).cnt {
        let r = region(ty, i);
        if memblock_addrs_overlap(base, size, (*r).base, (*r).size) != 0 { return true; }
    }
    false
}
unsafe fn memblock_remove_region(ty: *mut memblock_type, i: ULong) {
    let r = region(ty, i);
    (*ty).total_size = (*ty).total_size.wrapping_sub((*r).size);
    core::ptr::copy(r.add(1), r, ((*ty).cnt - i - 1) as usize);
    (*ty).cnt -= 1;
    if (*ty).cnt == 0 {
        warn!((*ty).total_size != 0);
        let r = (*ty).regions;
        (*r).base = 0; (*r).size = 0; (*r).flags = MEMBLOCK_NONE;
        set_node(r, MAX_NODES);
    }
}
unsafe fn memblock_merge_regions(ty: *mut memblock_type, start: ULong, end: ULong) {
    let mut i = if start == 0 { 0 } else { start - 1 };
    let mut end = min(end, (*ty).cnt.wrapping_sub(1));
    while i < end {
        let r = region(ty, i);
        let next = r.add(1);
        if region_end(r) != (*next).base || node(r) != node(next) || (*r).flags != (*next).flags {
            bug!(region_end(r) > (*next).base);
            i += 1;
        } else {
            (*r).size = (*r).size.wrapping_add((*next).size);
            core::ptr::copy(next.add(1), next, ((*ty).cnt - i - 2) as usize);
            (*ty).cnt -= 1;
            end -= 1;
        }
    }
}
unsafe fn memblock_insert_region(ty: *mut memblock_type, i: ULong, base: Phys, size: Phys, nid: i32, flags: Flags) {
    bug!((*ty).cnt >= (*ty).max);
    let r = region(ty, i);
    core::ptr::copy(r, r.add(1), ((*ty).cnt - i) as usize);
    (*r).base = base; (*r).size = size; (*r).flags = flags;
    set_node(r, nid);
    (*ty).cnt += 1;
    (*ty).total_size = (*ty).total_size.wrapping_add(size);
}
unsafe fn memblock_add_range(ty: *mut memblock_type, obase: Phys, size: Phys, nid: i32, flags: Flags) -> i32 {
    let size = cap_size(obase, size);
    let end = obase + size;
    if size == 0 { return 0; }
    if (*(*ty).regions).size == 0 {
        warn!((*ty).cnt != 0 || (*ty).total_size != 0);
        let r = (*ty).regions;
        (*r).base = obase; (*r).size = size; (*r).flags = flags;
        set_node(r, nid);
        (*ty).total_size = size; (*ty).cnt = 1;
        return 0;
    }
    let mut insert = (*ty).cnt.wrapping_mul(2).wrapping_add(1) <= (*ty).max;
    loop {
        let mut base = obase;
        let mut nr_new: ULong = 0;
        let mut idx: ULong = 0;
        let mut start_rgn = ULong::MAX;
        let mut end_rgn: ULong = 0;
        while idx < (*ty).cnt {
            let r = region(ty, idx);
            let rbase = (*r).base;
            let rend = region_end(r);
            if rbase >= end { break; }
            if rend <= base { idx += 1; continue; }
            if rbase > base {
                #[cfg(CONFIG_NUMA)] warn!(nid != node(r));
                warn!(flags != MEMBLOCK_NONE && flags != (*r).flags);
                nr_new += 1;
                if insert {
                    if start_rgn == ULong::MAX { start_rgn = idx; }
                    end_rgn = idx + 1;
                    memblock_insert_region(ty, idx, base, rbase - base, nid, flags);
                    idx += 1;
                }
            }
            base = min(rend, end);
            idx += 1;
        }
        if base < end {
            nr_new += 1;
            if insert {
                if start_rgn == ULong::MAX { start_rgn = idx; }
                end_rgn = idx + 1;
                memblock_insert_region(ty, idx, base, end - base, nid, flags);
            }
        }
        if nr_new == 0 { return 0; }
        if insert { memblock_merge_regions(ty, start_rgn, end_rgn); return 0; }
        while (*ty).cnt + nr_new > (*ty).max {
            if memblock_double_array(ty, obase, size) < 0 { return -(ENOMEM as i32); }
        }
        insert = true;
    }
}
unsafe fn memblock_isolate_range(ty: *mut memblock_type, base: Phys, size: Phys, start: *mut i32, stop: *mut i32) -> i32 {
    let size = cap_size(base, size);
    let end = base + size;
    *start = 0; *stop = 0;
    if size == 0 { return 0; }
    while (*ty).cnt + 2 > (*ty).max {
        if memblock_double_array(ty, base, size) < 0 { return -(ENOMEM as i32); }
    }
    let mut i: ULong = 0;
    while i < (*ty).cnt {
        let r = region(ty, i);
        let rbase = (*r).base;
        let rend = region_end(r);
        if rbase >= end { break; }
        if rend <= base { i += 1; continue; }
        let nid = node(r);
        let flags = (*r).flags;
        if rbase < base {
            (*r).base = base;
            (*r).size -= base - rbase;
            (*ty).total_size -= base - rbase;
            memblock_insert_region(ty, i, rbase, base - rbase, nid, flags);
            i += 1;
        } else if rend > end {
            (*r).base = end;
            (*r).size -= end - rbase;
            (*ty).total_size -= end - rbase;
            memblock_insert_region(ty, i, rbase, end - rbase, nid, flags);
            // Reprocess the new lower half at the same index.
        } else {
            if *stop == 0 { *start = i as i32; }
            *stop = (i + 1) as i32;
            i += 1;
        }
    }
    0
}
unsafe fn memblock_remove_range(ty: *mut memblock_type, base: Phys, size: Phys) -> i32 {
    let (mut start, mut end) = (0, 0);
    let err = memblock_isolate_range(ty, base, size, &mut start, &mut end);
    if err != 0 { return err; }
    for i in (start..end).rev() { memblock_remove_region(ty, i as ULong); }
    0
}
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_add_node(base: Phys, size: Phys, nid: i32, flags: Flags, caller: *const Void) -> i32 {
    debug_range(c"memblock_add_node".as_ptr().cast::<CChar>(), base, size, nid, flags, caller, true);
    memblock_add_range(memory(), base, size, nid, flags)
}
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_add(base: Phys, size: Phys, caller: *const Void) -> i32 {
    debug_range(c"memblock_add".as_ptr().cast::<CChar>(), base, size, MAX_NODES, 0, caller, false);
    memblock_add_range(memory(), base, size, MAX_NODES, 0)
}
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_remove(base: Phys, size: Phys, caller: *const Void) -> i32 {
    debug_range(c"memblock_remove".as_ptr().cast::<CChar>(), base, size, NO_NODE, 0, caller, false);
    memblock_remove_range(memory(), base, size)
}
#[no_mangle]
pub unsafe extern "C" fn rust___memblock_reserve(base: Phys, size: Phys, nid: i32, flags: Flags, caller: *const Void) -> i32 {
    debug_range(c"__memblock_reserve".as_ptr().cast::<CChar>(), base, size, nid, flags, caller, true);
    memblock_add_range(reserved(), base, size, nid, flags)
}
#[cfg(CONFIG_HAVE_MEMBLOCK_PHYS_MAP)]
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_physmem_add(base: Phys, size: Phys, caller: *const Void) -> i32 {
    debug_range(c"memblock_physmem_add".as_ptr().cast::<CChar>(), base, size, MAX_NODES, 0, caller, false);
    memblock_add_range(&raw mut physmem, base, size, MAX_NODES, 0)
}
unsafe fn memblock_setclr_flag(ty: *mut memblock_type, base: Phys, size: Phys, set: bool, flag: Flags) -> i32 {
    let (mut start, mut end) = (0, 0);
    let err = memblock_isolate_range(ty, base, size, &mut start, &mut end);
    if err != 0 { return err; }
    for i in start..end {
        let r = region(ty, i as ULong);
        if set { (*r).flags |= flag; } else { (*r).flags &= !flag; }
    }
    memblock_merge_regions(ty, start as ULong, end as ULong);
    0
}
#[no_mangle]
pub unsafe extern "C" fn memblock_mark_mirror(base: Phys, size: Phys) -> i32 {
    if !mirrored_kernelcore { return 0; }
    system_has_some_mirror = true;
    memblock_setclr_flag(memory(), base, size, true, MEMBLOCK_MIRROR)
}
#[no_mangle]
pub unsafe extern "C" fn memblock_set_node(base: Phys, size: Phys, ty: *mut memblock_type, nid: i32) -> i32 {
    #[cfg(CONFIG_NUMA)] {
        let (mut start, mut end) = (0, 0);
        let err = memblock_isolate_range(ty, base, size, &mut start, &mut end);
        if err != 0 { return err; }
        for i in start..end { set_node(region(ty, i as ULong), nid); }
        memblock_merge_regions(ty, start as ULong, end as ULong);
    }
    #[cfg(not(CONFIG_NUMA))] let _ = (base, size, ty, nid);
    0
}
}
macro_rules! flag_api {
    ($name:ident, $which:ident, $set:expr, $flag:ident) => {
        mb_fn! {
            #[no_mangle]
            pub unsafe extern "C" fn $name(base: Phys, size: Phys) -> i32 {
                memblock_setclr_flag($which(), base, size, $set, $flag)
            }
        }
    };
}
flag_api!(memblock_mark_hotplug, memory, true, MEMBLOCK_HOTPLUG);
flag_api!(memblock_clear_hotplug, memory, false, MEMBLOCK_HOTPLUG);
flag_api!(memblock_mark_nomap, memory, true, MEMBLOCK_NOMAP);
flag_api!(memblock_clear_nomap, memory, false, MEMBLOCK_NOMAP);
flag_api!(
    memblock_reserved_mark_noinit,
    reserved,
    true,
    MEMBLOCK_RSRV_NOINIT
);
flag_api!(
    memblock_reserved_mark_kern,
    reserved,
    true,
    MEMBLOCK_RSRV_KERN
);
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_mark_kho_scratch(base: Phys, size: Phys) -> i32 {
    memblock_setclr_flag(memory(), base, size, true, MEMBLOCK_KHO_SCRATCH)
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_clear_kho_scratch(base: Phys, size: Phys) -> i32 {
    memblock_setclr_flag(memory(), base, size, false, MEMBLOCK_KHO_SCRATCH)
}

// Kept in ordinary text: __next_mem_range remains usable for physmem even
// after the boot-only memory/reserved arrays have been discarded.
unsafe fn should_skip_region(
    ty: *mut memblock_type,
    r: *const memblock_region,
    nid: i32,
    flags: Flags,
) -> bool {
    if ty != memblock_memory {
        return false;
    }
    let f = (*r).flags;
    (valid_node(nid) && nid != node(r))
        || (rust_memblock_movable_node()
            && f & MEMBLOCK_HOTPLUG != 0
            && flags & MEMBLOCK_HOTPLUG == 0)
        || (flags & MEMBLOCK_MIRROR != 0 && f & MEMBLOCK_MIRROR == 0)
        || (flags & MEMBLOCK_NOMAP == 0 && f & MEMBLOCK_NOMAP != 0)
        || (flags & MEMBLOCK_DRIVER_MANAGED == 0 && f & MEMBLOCK_DRIVER_MANAGED != 0)
        || (flags & MEMBLOCK_KHO_SCRATCH != 0 && f & MEMBLOCK_KHO_SCRATCH == 0)
}
#[inline(always)]
unsafe fn range_out(start: Phys, end: Phys, nid: i32, os: *mut Phys, oe: *mut Phys, on: *mut i32) {
    if !os.is_null() {
        *os = start;
    }
    if !oe.is_null() {
        *oe = end;
    }
    if !on.is_null() {
        *on = nid;
    }
}
#[inline(always)]
fn range_index(a: i32, b: i32) -> u64 {
    (a as u32 as u64) | ((b as u64) << 32)
}
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_next_mem_range(
    idx: *mut u64,
    nid: i32,
    flags: Flags,
    a: *mut memblock_type,
    b: *mut memblock_type,
    os: *mut Phys,
    oe: *mut Phys,
    on: *mut i32,
) {
    let mut ai = (*idx as u32) as i32;
    let mut bi = (*idx >> 32) as i32;
    while (ai as ULong) < (*a).cnt {
        let m = region(a, ai as ULong);
        let ms = (*m).base;
        let me = region_end(m);
        let mn = node(m);
        if should_skip_region(a, m, nid, flags) {
            ai += 1;
            continue;
        }
        if b.is_null() {
            range_out(ms, me, mn, os, oe, on);
            *idx = range_index(ai + 1, bi);
            return;
        }
        while (bi as ULong) < (*b).cnt + 1 {
            let rs = if bi != 0 {
                region_end(region(b, (bi - 1) as ULong))
            } else {
                0
            };
            let re = if (bi as ULong) < (*b).cnt {
                (*region(b, bi as ULong)).base
            } else {
                Phys::MAX
            };
            if rs >= me {
                break;
            }
            if ms < re {
                range_out(max(ms, rs), min(me, re), mn, os, oe, on);
                if me <= re {
                    ai += 1;
                } else {
                    bi += 1;
                }
                *idx = range_index(ai, bi);
                return;
            }
            bi += 1;
        }
        ai += 1;
    }
    *idx = u64::MAX;
}
mb_fn! {
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_next_mem_range_rev(idx: *mut u64, nid: i32, flags: Flags,
    a: *mut memblock_type, b: *mut memblock_type, os: *mut Phys, oe: *mut Phys, on: *mut i32) {
    let mut ai = (*idx as u32) as i32;
    let mut bi = (*idx >> 32) as i32;
    if *idx == u64::MAX {
        ai = (*a).cnt.wrapping_sub(1) as i32;
        bi = if b.is_null() { 0 } else { (*b).cnt as i32 };
    }
    while ai >= 0 {
        let m = region(a, ai as ULong);
        let ms = (*m).base; let me = region_end(m); let mn = node(m);
        if should_skip_region(a, m, nid, flags) { ai -= 1; continue; }
        if b.is_null() {
            range_out(ms, me, mn, os, oe, on);
            *idx = range_index(ai - 1, bi); return;
        }
        while bi >= 0 {
            let rs = if bi != 0 { region_end(region(b, (bi - 1) as ULong)) } else { 0 };
            let re = if (bi as ULong) < (*b).cnt { (*region(b, bi as ULong)).base } else { Phys::MAX };
            if re <= ms { break; }
            if me > rs {
                range_out(max(ms, rs), min(me, re), mn, os, oe, on);
                if ms >= rs { ai -= 1; } else { bi -= 1; }
                *idx = range_index(ai, bi); return;
            }
            bi -= 1;
        }
        ai -= 1;
    }
    *idx = u64::MAX;
}
#[no_mangle]
pub unsafe extern "C" fn __next_mem_pfn_range(idx: *mut i32, nid: i32, os: *mut ULong, oe: *mut ULong, on: *mut i32) {
    let ty = memory();
    *idx += 1;
    while (*idx as ULong) < (*ty).cnt {
        let r = region(ty, *idx as ULong);
        let start = pfn_up((*r).base);
        let end = pfn_down(region_end(r));
        let rn = node(r);
        if start < end && (!valid_node(nid) || nid == rn) {
            if !os.is_null() { *os = start; }
            if !oe.is_null() { *oe = end; }
            if !on.is_null() { *on = rn; }
            return;
        }
        *idx += 1;
    }
    *idx = -1;
}
}

mb_fn! {
unsafe fn __memblock_find_range_bottom_up(start: Phys, end: Phys, size: Phys, align: Phys, nid: i32, flags: Flags) -> Phys {
    let mut idx = 0;
    let (mut rs, mut re) = (0, 0);
    loop {
        __next_mem_range(&mut idx, nid, flags, memory(), reserved(), &mut rs, &mut re, null_mut());
        if idx == u64::MAX { return 0; }
        rs = min(max(rs, start), end);
        re = min(max(re, start), end);
        let candidate = round_up(rs, align);
        if candidate < re && re - candidate >= size { return candidate; }
    }
}
unsafe fn __memblock_find_range_top_down(start: Phys, end: Phys, size: Phys, align: Phys, nid: i32, flags: Flags) -> Phys {
    let mut idx = u64::MAX;
    let (mut rs, mut re) = (0, 0);
    loop {
        __next_mem_range_rev(&mut idx, nid, flags, memory(), reserved(), &mut rs, &mut re, null_mut());
        if idx == u64::MAX { return 0; }
        rs = min(max(rs, start), end);
        re = min(max(re, start), end);
        if re < size { continue; }
        let candidate = round_down(re - size, align);
        if candidate >= rs { return candidate; }
    }
}
unsafe fn memblock_find_in_range_node(size: Phys, align: Phys, mut start: Phys, mut end: Phys, nid: i32, flags: Flags) -> Phys {
    if end == ACCESSIBLE || end == NOLEAKTRACE { end = memblock.current_limit; }
    start = max(start, PAGE_BYTES);
    end = max(start, end);
    if memblock.bottom_up {
        __memblock_find_range_bottom_up(start, end, size, align, nid, flags)
    } else {
        __memblock_find_range_top_down(start, end, size, align, nid, flags)
    }
}
unsafe fn memblock_find_in_range(start: Phys, end: Phys, size: Phys, align: Phys) -> Phys {
    let mut flags = choose_memblock_flags();
    loop {
        let found = memblock_find_in_range_node(size, align, start, end, NO_NODE, flags);
        if found == 0 && flags & MEMBLOCK_MIRROR != 0 {
            rust_memblock_warn_mirror_find(size);
            flags &= !MEMBLOCK_MIRROR;
        } else { return found; }
    }
}
unsafe fn memblock_double_array(ty: *mut memblock_type, mut avoid_start: Phys, mut avoid_size: Phys) -> i32 {
    let use_slab = rust_memblock_slab_available();
    if !memblock_can_resize { panic(c"memblock: cannot resize %s array\n".as_ptr().cast::<CChar>(), (*ty).name); }
    // C evaluates max * sizeof(region) in size_t before widening to Phys.
    let old_size = ((*ty).max as usize).wrapping_mul(size_of::<memblock_region>()) as Phys;
    let new_size = old_size.wrapping_shl(1);
    let old_alloc_size = round_up(old_size, PAGE_BYTES);
    let new_alloc_size = round_up(new_size, PAGE_BYTES);
    let in_slab = if ty == memory() { &raw mut memblock_memory_in_slab } else { &raw mut memblock_reserved_in_slab };
    let addr: Phys;
    let new_array: *mut memblock_region;
    if use_slab {
        new_array = rust_memblock_kmalloc(new_size as usize).cast();
        addr = if new_array.is_null() { 0 } else { rust_memblock_pa(new_array.cast()) };
    } else {
        if ty != reserved() { avoid_start = 0; avoid_size = 0; }
        let mut found = memblock_find_in_range(avoid_start.wrapping_add(avoid_size), memblock.current_limit, new_alloc_size, PAGE_BYTES);
        if found == 0 && avoid_size != 0 {
            found = memblock_find_in_range(0, min(avoid_start, memblock.current_limit), new_alloc_size, PAGE_BYTES);
        }
        addr = found;
        if addr != 0 {
            rust_memblock_accept_memory(addr, new_alloc_size as ULong);
            new_array = rust_memblock_va(addr).cast();
        } else { new_array = null_mut(); }
    }
    if addr == 0 {
        info!(c"\x013memblock: Failed to double %s array from %ld to %ld entries !\n", (*ty).name, (*ty).max, (*ty).max.wrapping_mul(2));
        return -1;
    }
    if memblock_debug != 0 {
        let end = addr.wrapping_add(new_size).wrapping_sub(1);
        info!(c"\x016memblock: %s is doubled to %ld at [%pa-%pa]", (*ty).name, (*ty).max.wrapping_mul(2), &addr, &end);
    }
    let old_array = (*ty).regions;
    core::ptr::copy_nonoverlapping(old_array, new_array, (*ty).max as usize);
    core::ptr::write_bytes(new_array.add((*ty).max as usize), 0, (*ty).max as usize);
    (*ty).regions = new_array;
    (*ty).max = (*ty).max.wrapping_shl(1);
    // Install the new reserved array before any reserve/free can recurse.
    if *in_slab { kfree(old_array.cast()); }
    else if old_array != (&raw mut MEMORY_REGIONS).cast() && old_array != (&raw mut RESERVED_REGIONS).cast() {
        memblock_free(old_array.cast(), old_alloc_size as usize);
    }
    if !use_slab { bug!(__memblock_reserve(addr, new_alloc_size, NO_NODE, MEMBLOCK_RSRV_KERN) != 0); }
    *in_slab = use_slab;
    0
}
#[no_mangle]
pub unsafe extern "C" fn memblock_validate_numa_coverage(threshold_bytes: ULong) -> bool {
    let mut nr_pages: ULong = 0;
    let mut i = -1;
    let (mut start, mut end, mut nid) = (0, 0, 0);
    loop {
        __next_mem_pfn_range(&mut i, MAX_NODES, &mut start, &mut end, &mut nid);
        if i < 0 { break; }
        if !valid_node(nid) { nr_pages = nr_pages.wrapping_add(end - start); }
    }
    if nr_pages.wrapping_shl(PAGE_BITS) > threshold_bytes {
        let total_mb = (memblock_phys_mem_size() / (1 << 20)) as ULong;
        info!(c"\x013NUMA: no nodes coverage for %luMB of %luMB RAM\n", nr_pages.wrapping_shl(PAGE_BITS) / (1 << 20), total_mb);
        false
    } else { true }
}
#[no_mangle]
pub unsafe extern "C" fn memblock_free(ptr: *mut Void, size: usize) {
    if !ptr.is_null() { memblock_phys_free(rust_memblock_pa(ptr), size as Phys); }
}
#[no_mangle]
pub unsafe extern "C" fn rust_memblock_phys_free(base: Phys, size: Phys, caller: *const Void) -> i32 {
    debug_range(c"memblock_phys_free".as_ptr().cast::<CChar>(), base, size, NO_NODE, 0, caller, false);
    rust_memblock_kmemleak_free(base, size as usize);
    let mut ret = 0;
    if !rust_memblock_slab_available() || cfg!(CONFIG_ARCH_KEEP_MEMBLOCK) {
        ret = memblock_remove_range(reserved(), base, size);
    }
    if rust_memblock_slab_available() { __free_reserved_area(base, base.wrapping_add(size), -1); }
    ret
}
}

#[cfg(not(CONFIG_ARCH_KEEP_MEMBLOCK))]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_discard() {
    if memblock.reserved.regions != (&raw mut RESERVED_REGIONS).cast() {
        let ptr = memblock.reserved.regions.cast();
        let bytes = (memblock.reserved.max as usize).wrapping_mul(size_of::<memblock_region>());
        let size = bytes.wrapping_add(PAGE_BYTES as usize - 1) & !(PAGE_BYTES as usize - 1);
        if memblock_reserved_in_slab {
            kfree(ptr);
        } else {
            memblock_free(ptr, size);
        }
    }
    if memblock.memory.regions != (&raw mut MEMORY_REGIONS).cast() {
        let ptr = memblock.memory.regions.cast();
        let bytes = (memblock.memory.max as usize).wrapping_mul(size_of::<memblock_region>());
        let size = bytes.wrapping_add(PAGE_BYTES as usize - 1) & !(PAGE_BYTES as usize - 1);
        if memblock_memory_in_slab {
            kfree(ptr);
        } else {
            memblock_free(ptr, size);
        }
    }
    memblock_memory = null_mut();
}

// Header pfn_valid() is allowed to implement an architecture's map semantics;
// the iteration, page lifetime, poisoning and freeing remain Rust decisions.
unsafe fn __free_reserved_area(start: Phys, end: Phys, poison: i32) -> ULong {
    if rust_memblock_deferred_pages_enabled() {
        rust_memblock_warn_deferred_free();
        return 0;
    }
    let end_pfn = pfn_down(end);
    let mut pfn = first_valid_pfn(pfn_up(start), end_pfn);
    let mut pages: ULong = 0;
    while pfn < end_pfn {
        let page = rust_memblock_pfn_to_page(pfn);
        let direct = rust_memblock_kasan_reset_tag(rust_memblock_page_address(page));
        if poison as u32 <= 0xff {
            core::ptr::write_bytes(direct.cast::<u8>(), poison as u8, PAGE_BYTES as usize);
        }
        rust_memblock_free_reserved_page(page);
        pages += 1;
        pfn = next_valid_pfn(pfn, end_pfn);
    }
    pages
}
#[no_mangle]
pub unsafe extern "C" fn free_reserved_area(
    start: *mut Void,
    end: *mut Void,
    poison: i32,
    name: *const CChar,
) -> ULong {
    let last = end.cast::<u8>().wrapping_sub(1).cast();
    let (start_pa, end_pa) = if rust_memblock_is_kernel(start as ULong) {
        (
            rust_memblock_pa_symbol(start),
            rust_memblock_pa_symbol(last).wrapping_add(1),
        )
    } else {
        (
            rust_memblock_pa(start),
            rust_memblock_pa(last).wrapping_add(1),
        )
    };
    #[cfg(CONFIG_ARCH_KEEP_MEMBLOCK)]
    if start_pa < end_pa {
        memblock_remove_range(reserved(), start_pa, end_pa - start_pa);
    }
    let pages = __free_reserved_area(start_pa, end_pa, poison);
    if pages != 0 && !name.is_null() {
        info!(
            c"\x016Freeing %s memory: %ldK\n",
            name,
            pages.wrapping_shl(PAGE_BITS - 10)
        );
    }
    pages
}

unsafe fn memblock_prep_allocation(start: Phys, size: Phys, trace: bool) {
    if trace {
        rust_memblock_kmemleak_alloc(start, size as usize);
    }
    rust_memblock_accept_memory(start, size as ULong);
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_alloc_range_nid(
    size: Phys,
    mut align: Phys,
    start: Phys,
    end: Phys,
    nid: i32,
    exact_nid: bool,
) -> Phys {
    let mut flags = choose_memblock_flags();
    if rust_memblock_warn_slab_once(rust_memblock_slab_available()) {
        let ptr = rust_memblock_kzalloc_node(size as usize, nid);
        return if ptr.is_null() {
            0
        } else {
            rust_memblock_virt_to_phys(ptr)
        };
    }
    if align == 0 {
        rust_memblock_dump_stack();
        align = RUST_MEMBLOCK_SMP_CACHE_BYTES as Phys;
    }
    loop {
        let found = memblock_find_in_range_node(size, align, start, end, nid, flags);
        if found != 0 && __memblock_reserve(found, size, nid, MEMBLOCK_RSRV_KERN) == 0 {
            memblock_prep_allocation(found, size, end != NOLEAKTRACE);
            return found;
        }
        if valid_node(nid) && !exact_nid {
            let found = memblock_find_in_range_node(size, align, start, end, NO_NODE, flags);
            if found != 0 && __memblock_reserve(found, size, NO_NODE, MEMBLOCK_RSRV_KERN) == 0 {
                memblock_prep_allocation(found, size, end != NOLEAKTRACE);
                return found;
            }
        }
        if flags & MEMBLOCK_MIRROR == 0 {
            return 0;
        }
        flags &= !MEMBLOCK_MIRROR;
        rust_memblock_warn_mirror_alloc(size);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_phys_alloc_range(
    size: Phys,
    align: Phys,
    start: Phys,
    end: Phys,
    caller: *const Void,
) -> Phys {
    debug_alloc(
        c"memblock_phys_alloc_range".as_ptr().cast::<CChar>(),
        size,
        align,
        start,
        end,
        NO_NODE,
        caller,
        false,
    );
    memblock_alloc_range_nid(size, align, start, end, NO_NODE, false)
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_phys_alloc_try_nid(size: Phys, align: Phys, nid: i32) -> Phys {
    memblock_alloc_range_nid(size, align, 0, ACCESSIBLE, nid, false)
}
#[link_section = ".init.text"]
unsafe fn memblock_alloc_internal(
    size: Phys,
    align: Phys,
    min_addr: Phys,
    mut max_addr: Phys,
    nid: i32,
    exact_nid: bool,
) -> *mut Void {
    if max_addr > memblock.current_limit {
        max_addr = memblock.current_limit;
    }
    let mut addr = memblock_alloc_range_nid(size, align, min_addr, max_addr, nid, exact_nid);
    if addr == 0 && min_addr != 0 {
        addr = memblock_alloc_range_nid(size, align, 0, max_addr, nid, exact_nid);
    }
    if addr == 0 {
        null_mut()
    } else {
        rust_memblock_phys_to_virt(addr)
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_alloc_exact_nid_raw(
    size: Phys,
    align: Phys,
    min_addr: Phys,
    max_addr: Phys,
    nid: i32,
    caller: *const Void,
) -> *mut Void {
    debug_alloc(
        c"memblock_alloc_exact_nid_raw".as_ptr().cast::<CChar>(),
        size,
        align,
        min_addr,
        max_addr,
        nid,
        caller,
        true,
    );
    memblock_alloc_internal(size, align, min_addr, max_addr, nid, true)
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_alloc_try_nid_raw(
    size: Phys,
    align: Phys,
    min_addr: Phys,
    max_addr: Phys,
    nid: i32,
    caller: *const Void,
) -> *mut Void {
    debug_alloc(
        c"memblock_alloc_try_nid_raw".as_ptr().cast::<CChar>(),
        size,
        align,
        min_addr,
        max_addr,
        nid,
        caller,
        true,
    );
    memblock_alloc_internal(size, align, min_addr, max_addr, nid, false)
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_alloc_hugetlb(
    size: Phys,
    mut nid: i32,
    exact_nid: bool,
    caller: *const Void,
) -> *mut Void {
    let flags = choose_memblock_flags() & !MEMBLOCK_MIRROR;
    let (mut start, mut end) = (0, ACCESSIBLE);
    if memblock_debug != 0 {
        info!(
            c"\x016memblock_alloc_hugetlb: %llu bytes, nid=%d, exact_nid=%d %pS\n",
            size as u64, nid, exact_nid as i32, caller
        );
    }
    loop {
        let addr = memblock_find_in_range_node(size, size, start, end, nid, flags);
        if addr == 0 {
            if valid_node(nid) && !exact_nid {
                nid = NO_NODE;
                start = 0;
                end = ACCESSIBLE;
                continue;
            }
            return null_mut();
        }
        if rust_memblock_kho_scratch_overlap(addr, size as usize) {
            if memblock.bottom_up {
                start = addr.wrapping_add(size);
            } else {
                end = addr;
            }
            continue;
        }
        if __memblock_reserve(addr, size, nid, MEMBLOCK_RSRV_KERN | MEMBLOCK_RSRV_HUGETLB) != 0 {
            return null_mut();
        }
        memblock_prep_allocation(addr, size, true);
        return rust_memblock_phys_to_virt(addr);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_alloc_try_nid(
    size: Phys,
    align: Phys,
    min_addr: Phys,
    max_addr: Phys,
    nid: i32,
    caller: *const Void,
) -> *mut Void {
    debug_alloc(
        c"memblock_alloc_try_nid".as_ptr().cast::<CChar>(),
        size,
        align,
        min_addr,
        max_addr,
        nid,
        caller,
        true,
    );
    let ptr = memblock_alloc_internal(size, align, min_addr, max_addr, nid, false);
    if !ptr.is_null() {
        core::ptr::write_bytes(ptr.cast::<u8>(), 0, size as usize);
    }
    ptr
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn __memblock_alloc_or_panic(
    size: Phys,
    align: Phys,
    func: *const CChar,
) -> *mut Void {
    let ptr = memblock_alloc_try_nid(size, align, 0, ACCESSIBLE, NO_NODE);
    if ptr.is_null() {
        panic(
            c"%s: Failed to allocate %pap bytes\n"
                .as_ptr()
                .cast::<CChar>(),
            func,
            &size,
        );
    }
    ptr
}

mb_fn! {
#[no_mangle]
pub unsafe extern "C" fn memblock_phys_mem_size() -> Phys { memblock.memory.total_size }
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_size() -> Phys { memblock.reserved.total_size }
unsafe fn reserved_flag_size(limit: Phys, nid: i32, flag: Flags) -> Phys {
    let mut total: Phys = 0;
    for i in 0..memblock.reserved.cnt {
        let r = region(reserved(), i);
        if (*r).base > limit { break; }
        let size = if region_end(r) > limit { limit - (*r).base } else { (*r).size };
        if (nid == node(r) || !valid_node(nid)) && (*r).flags & flag != 0 { total = total.wrapping_add(size); }
    }
    total
}
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_hugetlb_size(limit: Phys, nid: i32) -> Phys { reserved_flag_size(limit, nid, MEMBLOCK_RSRV_HUGETLB) }
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_kern_size(limit: Phys, nid: i32) -> Phys { reserved_flag_size(limit, nid, MEMBLOCK_RSRV_KERN) }
#[no_mangle]
pub unsafe extern "C" fn memblock_start_of_DRAM() -> Phys { (*memblock.memory.regions).base }
#[no_mangle]
pub unsafe extern "C" fn memblock_end_of_DRAM() -> Phys { region_end(region(memory(), memblock.memory.cnt.wrapping_sub(1))) }
unsafe fn __find_max_addr(mut limit: Phys) -> Phys {
    for i in 0..memblock.memory.cnt {
        let r = region(memory(), i);
        if limit <= (*r).size { return (*r).base.wrapping_add(limit); }
        limit -= (*r).size;
    }
    Phys::MAX
}
unsafe fn memblock_search(ty: *mut memblock_type, addr: Phys) -> i32 {
    let (mut left, mut right) = (0u32, (*ty).cnt as u32);
    loop {
        let mid = right.wrapping_add(left) / 2;
        let r = region(ty, mid as ULong);
        if addr < (*r).base { right = mid; }
        else if addr >= region_end(r) { left = mid + 1; }
        else { return mid as i32; }
        if left >= right { return -1; }
    }
}
#[no_mangle]
pub unsafe extern "C" fn memblock_is_reserved(addr: Phys) -> bool { memblock_search(reserved(), addr) != -1 }
#[no_mangle]
pub unsafe extern "C" fn memblock_is_memory(addr: Phys) -> bool { memblock_search(memory(), addr) != -1 }
#[no_mangle]
pub unsafe extern "C" fn memblock_is_map_memory(addr: Phys) -> bool {
    let i = memblock_search(memory(), addr);
    i != -1 && (*region(memory(), i as ULong)).flags & MEMBLOCK_NOMAP == 0
}
#[no_mangle]
pub unsafe extern "C" fn memblock_search_pfn_nid(pfn: ULong, start: *mut ULong, end: *mut ULong) -> i32 {
    let i = memblock_search(memory(), pfn_phys(pfn));
    if i == -1 { return NO_NODE; }
    let r = region(memory(), i as ULong);
    *start = pfn_down((*r).base); *end = pfn_down(region_end(r));
    node(r)
}
#[no_mangle]
pub unsafe extern "C" fn memblock_is_region_memory(base: Phys, size: Phys) -> bool {
    let i = memblock_search(memory(), base);
    let end = base + cap_size(base, size);
    i != -1 && region_end(region(memory(), i as ULong)) >= end
}
#[no_mangle]
pub unsafe extern "C" fn memblock_is_region_reserved(base: Phys, size: Phys) -> bool { memblock_overlaps_region(reserved(), base, size) }
#[no_mangle]
pub unsafe extern "C" fn memblock_trim_memory(align: Phys) {
    let mut i = 0;
    while i < memblock.memory.cnt {
        let r = region(memory(), i);
        let start = round_up((*r).base, align);
        let end = round_down(region_end(r), align);
        if start == (*r).base && end == region_end(r) { i += 1; continue; }
        if start < end {
            (*r).base = start; (*r).size = end - start;
            // Original memblock_trim_memory intentionally does not update total_size here.
            i += 1;
        } else { memblock_remove_region(memory(), i); }
    }
}
#[no_mangle]
pub unsafe extern "C" fn memblock_set_current_limit(limit: Phys) { memblock.current_limit = limit; }
#[no_mangle]
pub unsafe extern "C" fn memblock_get_current_limit() -> Phys { memblock.current_limit }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_estimated_nr_free_pages() -> ULong {
    pfn_down(memblock_phys_mem_size().wrapping_sub(memblock_reserved_kern_size(ANYWHERE, NO_NODE)))
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_enforce_memory_limit(limit: Phys) {
    if limit == 0 {
        return;
    }
    let addr = __find_max_addr(limit);
    if addr == Phys::MAX {
        return;
    }
    memblock_remove_range(memory(), addr, Phys::MAX);
    memblock_remove_range(reserved(), addr, Phys::MAX);
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_cap_memory_range(base: Phys, size: Phys) {
    if size == 0 {
        return;
    }
    if (*memblock_memory).total_size == 0 {
        info!(c"\x014memblock_cap_memory_range: No memory registered yet\n");
        return;
    }
    let (mut start, mut end) = (0, 0);
    if memblock_isolate_range(memory(), base, size, &mut start, &mut end) != 0 {
        return;
    }
    let mut i = memblock.memory.cnt as i32 - 1;
    while i >= end {
        if (*region(memory(), i as ULong)).flags & MEMBLOCK_NOMAP == 0 {
            memblock_remove_region(memory(), i as ULong);
        }
        i -= 1;
    }
    let mut i = start - 1;
    while i >= 0 {
        if (*region(memory(), i as ULong)).flags & MEMBLOCK_NOMAP == 0 {
            memblock_remove_region(memory(), i as ULong);
        }
        i -= 1;
    }
    memblock_remove_range(reserved(), 0, base);
    memblock_remove_range(reserved(), base.wrapping_add(size), Phys::MAX);
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_mem_limit_remove_map(limit: Phys) {
    if limit == 0 {
        return;
    }
    let addr = __find_max_addr(limit);
    if addr != Phys::MAX {
        memblock_cap_memory_range(0, addr);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_allow_resize() {
    memblock_can_resize = true;
}

mb_fn! {
unsafe fn debug_range(name: *const CChar, base: Phys, size: Phys, nid: i32, flags: Flags, caller: *const Void, with_node: bool) {
    if memblock_debug != 0 {
        let end = base.wrapping_add(size).wrapping_sub(1);
        if with_node {
            info!(c"\x016%s: [%pa-%pa] nid=%d flags=%x %pS\n", name, &base, &end, nid, flags, caller);
        } else {
            info!(c"\x016%s: [%pa-%pa] %pS\n", name, &base, &end, caller);
        }
    }
}
unsafe fn debug_alloc(name: *const CChar, size: Phys, align: Phys, start: Phys, end: Phys, nid: i32, caller: *const Void, with_node: bool) {
    if memblock_debug != 0 {
        if with_node {
            info!(c"\x016%s: %llu bytes align=0x%llx nid=%d from=%pa max_addr=%pa %pS\n", name, size as u64, align as u64, nid, &start, &end, caller);
        } else {
            info!(c"\x016%s: %llu bytes align=0x%llx from=%pa max_addr=%pa %pS\n", name, size as u64, align as u64, &start, &end, caller);
        }
    }
}
unsafe fn memblock_dump(ty: *mut memblock_type) {
    info!(c"\x016 %s.cnt  = 0x%lx\n", (*ty).name, (*ty).cnt);
    for i in 0..(*ty).cnt {
        let r = region(ty, i);
        let base = (*r).base; let size = (*r).size;
        let end = base.wrapping_add(size).wrapping_sub(1);
        let mut nid_buf = [0 as CChar; 32];
        #[cfg(CONFIG_NUMA)]
        if valid_node(node(r)) { snprintf(nid_buf.as_mut_ptr(), nid_buf.len(), c" on node %d".as_ptr().cast::<CChar>(), node(r)); }
        info!(c"\x016 %s[%#x]\t[%pa-%pa], %pa bytes%s flags: %#x\n", (*ty).name, i as u32, &base, &end, &size, nid_buf.as_ptr(), (*r).flags);
    }
}
unsafe fn __memblock_dump_all() {
    info!(c"\x016MEMBLOCK configuration:\n");
    info!(c"\x016 memory size = %pa reserved size = %pa\n", &raw const memblock.memory.total_size, &raw const memblock.reserved.total_size);
    memblock_dump(memory()); memblock_dump(reserved());
    #[cfg(CONFIG_HAVE_MEMBLOCK_PHYS_MAP)] memblock_dump(&raw mut physmem);
}
#[no_mangle]
pub unsafe extern "C" fn memblock_dump_all() { if memblock_debug != 0 { __memblock_dump_all(); } }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_memblock_early_param(p: *mut CChar) -> i32 {
    if !p.is_null() && !strstr(p, c"debug".as_ptr().cast::<CChar>()).is_null() {
        memblock_debug = 1;
    }
    0
}

#[inline]
unsafe fn first_valid_pfn(mut pfn: ULong, end: ULong) -> ULong {
    #[cfg(all(CONFIG_SPARSEMEM, not(CONFIG_HAVE_ARCH_PFN_VALID)))]
    {
        rust_memblock_first_valid_pfn(pfn, end)
    }
    #[cfg(not(all(CONFIG_SPARSEMEM, not(CONFIG_HAVE_ARCH_PFN_VALID))))]
    {
        while pfn < end && !rust_memblock_pfn_valid(pfn) {
            pfn += 1;
        }
        pfn
    }
}
#[inline]
unsafe fn next_valid_pfn(pfn: ULong, end: ULong) -> ULong {
    #[cfg(all(CONFIG_SPARSEMEM, not(CONFIG_HAVE_ARCH_PFN_VALID)))]
    {
        rust_memblock_next_valid_pfn(pfn, end)
    }
    #[cfg(not(all(CONFIG_SPARSEMEM, not(CONFIG_HAVE_ARCH_PFN_VALID))))]
    {
        first_valid_pfn(pfn.wrapping_add(1), end)
    }
}
#[link_section = ".init.text"]
unsafe fn free_memmap(start_pfn: ULong, end_pfn: ULong) {
    let start_pg = rust_memblock_pfn_to_page(start_pfn.wrapping_sub(1)).wrapping_add(1);
    let end_pg = rust_memblock_pfn_to_page(end_pfn.wrapping_sub(1)).wrapping_add(1);
    let start = round_up(rust_memblock_pa(start_pg.cast()), PAGE_BYTES);
    let end = round_down(rust_memblock_pa(end_pg.cast()), PAGE_BYTES);
    if start < end {
        memblock_phys_free(start, end - start);
    }
}
#[link_section = ".init.text"]
unsafe fn free_unused_memmap() {
    if !cfg!(CONFIG_HAVE_ARCH_PFN_VALID) || cfg!(CONFIG_SPARSEMEM_VMEMMAP) {
        return;
    }
    let (mut start, mut end, mut prev_end) = (0, 0, 0);
    let mut idx = -1;
    loop {
        __next_mem_pfn_range(&mut idx, MAX_NODES, &mut start, &mut end, null_mut());
        if idx < 0 {
            break;
        }
        #[cfg(CONFIG_SPARSEMEM)]
        {
            start = min(
                start,
                round_up(prev_end as Phys, RUST_MEMBLOCK_PAGES_PER_SECTION as Phys) as ULong,
            );
        }
        start = rust_memblock_pageblock_start(start);
        if prev_end != 0 && prev_end < start {
            free_memmap(prev_end, start);
        }
        prev_end = rust_memblock_pageblock_align(end);
    }
    #[cfg(CONFIG_SPARSEMEM)]
    if prev_end & (RUST_MEMBLOCK_PAGES_PER_SECTION as ULong - 1) != 0 {
        free_memmap(
            prev_end,
            round_up(prev_end as Phys, RUST_MEMBLOCK_PAGES_PER_SECTION as Phys) as ULong,
        );
    }
}
#[link_section = ".init.text"]
unsafe fn __free_pages_memory(mut start: ULong, end: ULong) {
    while start < end {
        let mut order = if start == 0 {
            RUST_MEMBLOCK_MAX_PAGE_ORDER as u32
        } else {
            min(RUST_MEMBLOCK_MAX_PAGE_ORDER as u32, start.trailing_zeros())
        };
        while start.wrapping_add((1 as ULong) << order) > end {
            order -= 1;
        }
        memblock_free_pages(start, order);
        start = start.wrapping_add((1 as ULong) << order);
    }
}
#[link_section = ".init.text"]
unsafe fn __free_memory_core(start: Phys, end: Phys) -> ULong {
    let start_pfn = pfn_up(start);
    let mut end_pfn = pfn_down(end);
    if !cfg!(CONFIG_HIGHMEM) && end_pfn > max_low_pfn {
        end_pfn = max_low_pfn;
    }
    if start_pfn >= end_pfn {
        return 0;
    }
    __free_pages_memory(start_pfn, end_pfn);
    end_pfn - start_pfn
}
#[link_section = ".init.text"]
unsafe fn memmap_init_reserved_range(start: Phys, end: Phys, nid: i32) {
    let end_pfn = pfn_up(end);
    let mut pfn = first_valid_pfn(pfn_down(start), end_pfn);
    while pfn < end_pfn {
        let page = rust_memblock_pfn_to_page(pfn);
        init_deferred_page(pfn, nid);
        rust_memblock_set_page_reserved(page);
        pfn = next_valid_pfn(pfn, end_pfn);
    }
}
#[link_section = ".init.text"]
unsafe fn memmap_init_reserved_pages() {
    loop {
        let old_max = memblock.reserved.max;
        for i in 0..memblock.memory.cnt {
            let r = region(memory(), i);
            let nid = node(r);
            let start = (*r).base;
            let size = (*r).size;
            if (*r).flags & MEMBLOCK_NOMAP != 0 {
                memmap_init_reserved_range(start, start.wrapping_add(size), nid);
            }
            memblock_set_node(start, size, reserved(), nid);
        }
        if old_max == memblock.reserved.max {
            break;
        }
    }
    for i in 0..memblock.reserved.cnt {
        let r = region(reserved(), i);
        if (*r).flags & MEMBLOCK_RSRV_NOINIT == 0 {
            let mut nid = node(r);
            let start = (*r).base;
            let end = region_end(r);
            if !valid_node(nid) {
                nid = rust_memblock_early_pfn_to_nid(pfn_down(start));
            }
            memmap_init_reserved_range(start, end, nid);
        }
    }
}
#[link_section = ".init.text"]
unsafe fn free_low_memory_core_early() -> ULong {
    memblock_clear_hotplug(0, Phys::MAX);
    memmap_init_reserved_pages();
    let mut count: ULong = 0;
    let mut idx = 0;
    let (mut start, mut end) = (0, 0);
    loop {
        __next_mem_range(
            &mut idx,
            NO_NODE,
            MEMBLOCK_NONE,
            memory(),
            reserved(),
            &mut start,
            &mut end,
            null_mut(),
        );
        if idx == u64::MAX {
            break;
        }
        count = count.wrapping_add(__free_memory_core(start, end));
    }
    count
}
#[link_section = ".init.data"]
static mut reset_managed_pages_done: bool = false;
#[link_section = ".init.text"]
unsafe fn reset_node_managed_pages(pgdat: *mut pglist_data) {
    let zones = (&raw mut (*pgdat).node_zones).cast::<zone>();
    for i in 0..RUST_MEMBLOCK_MAX_NR_ZONES as usize {
        rust_memblock_atomic_long_set(&raw mut (*zones.add(i)).managed_pages, 0);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn reset_all_zones_managed_pages() {
    if reset_managed_pages_done {
        return;
    }
    let mut pgdat = first_online_pgdat();
    while !pgdat.is_null() {
        reset_node_managed_pages(pgdat);
        pgdat = next_online_pgdat(pgdat);
    }
    reset_managed_pages_done = true;
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_free_all() {
    free_unused_memmap();
    reset_all_zones_managed_pages();
    #[cfg(CONFIG_MEMBLOCK_KHO_SCRATCH)]
    memblock_clear_kho_scratch_only();
    let pages = free_low_memory_core_early();
    rust_memblock_totalram_pages_add(pages);
}
#[cfg(CONFIG_MEMBLOCK_KHO_SCRATCH)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_set_kho_scratch_only() {
    kho_scratch_only = true;
}
#[cfg(CONFIG_MEMBLOCK_KHO_SCRATCH)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn memblock_clear_kho_scratch_only() {
    kho_scratch_only = false;
}

include!("memblock_reserved.rs");
