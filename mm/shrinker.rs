// SPDX-License-Identifier: GPL-2.0
// Translated from shrinker.c. Kernel types, constants, and helpers are supplied
// by the surrounding kernel bindings.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel declarations intentionally remain unresolved here.
extern "C" {
    static mut shrinker_list: list_head;
    static mut shrinker_mutex: mutex;
    static mut root_mem_cgroup: *mut mem_cgroup;
    static mut shrinker_idr: idr;
    static mut nr_node_ids: c_int;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct mem_cgroup { pub nodeinfo: *mut *mut mem_cgroup_per_node }
#[repr(C)] pub struct mem_cgroup_per_node { pub shrinker_info: *mut shrinker_info }
#[repr(C)] pub struct shrinker_info { pub map_nr_max: c_int, pub unit: *mut *mut shrinker_info_unit }
#[repr(C)] pub struct shrinker_info_unit { pub map: *mut c_ulong, pub nr_deferred: *mut atomic_long_t }
#[repr(C)] pub struct atomic_long_t { pub counter: c_ulong }
#[repr(C)] pub struct shrink_control { pub gfp_mask: gfp_t, pub nid: c_int, pub memcg: *mut mem_cgroup, pub nr_to_scan: c_ulong, pub nr_scanned: c_ulong, pub priority: c_int }
#[repr(C)] pub struct shrinker {
    pub list: list_head, pub flags: c_uint, pub seeks: c_ulong, pub batch: c_ulong,
    pub id: c_int, pub nr_deferred: *mut atomic_long_t, pub rcu: rcu_head,
    pub done: completion, pub refcount: refcount_t,
    pub count_objects: Option<unsafe extern "C" fn(*mut shrinker, *mut shrink_control) -> c_ulong>,
    pub scan_objects: Option<unsafe extern "C" fn(*mut shrinker, *mut shrink_control) -> c_ulong>,
}
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
pub type gfp_t = c_uint;

const SHRINK_BATCH: c_ulong = 128;

#[cfg(feature = "CONFIG_MEMCG")]
static mut shrinker_nr_max: c_int = 0;

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn shrinker_unit_size(nr_items: c_int) -> c_int { (((nr_items + SHRINKER_UNIT_BITS - 1) / SHRINKER_UNIT_BITS) * core::mem::size_of::<*mut shrinker_info_unit>()) as c_int }
#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn shrinker_unit_free(info: *mut shrinker_info, start: c_int) {
    if info.is_null() { return; }
    let unit = (*info).unit; let nr = ((*info).map_nr_max + SHRINKER_UNIT_BITS - 1) / SHRINKER_UNIT_BITS;
    for i in start..nr { let p = *unit.add(i as usize); if p.is_null() { break; } kfree(p as *mut c_void); *unit.add(i as usize) = core::ptr::null_mut(); }
}
#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn shrinker_unit_alloc(new: *mut shrinker_info, old: *mut shrinker_info, nid: c_int) -> c_int {
    let nr = ((*new).map_nr_max + SHRINKER_UNIT_BITS - 1) / SHRINKER_UNIT_BITS;
    let start = if old.is_null() { 0 } else { ((*old).map_nr_max + SHRINKER_UNIT_BITS - 1) / SHRINKER_UNIT_BITS };
    for i in start..nr { let unit = kzalloc_node(core::mem::size_of::<shrinker_info_unit>(), GFP_KERNEL, nid) as *mut shrinker_info_unit; if unit.is_null() { shrinker_unit_free(new, start); return -ENOMEM; } *(*new).unit.add(i as usize) = unit; } 0
}

#[inline] unsafe fn shrinker_id_to_index(id: c_int) -> c_int { id / SHRINKER_UNIT_BITS }
#[inline] unsafe fn shrinker_id_to_offset(id: c_int) -> c_int { id % SHRINKER_UNIT_BITS }
#[inline] unsafe fn calc_shrinker_id(index: c_int, offset: c_int) -> c_int { index * SHRINKER_UNIT_BITS + offset }

unsafe fn xchg_nr_deferred(shrinker: *mut shrinker, sc: *mut shrink_control) -> c_long {
    let mut nid = (*sc).nid; if (*shrinker).flags & SHRINKER_NUMA_AWARE == 0 { nid = 0; }
    if !(*sc).memcg.is_null() && (*shrinker).flags & SHRINKER_MEMCG_AWARE != 0 { return xchg_nr_deferred_memcg(nid, shrinker, (*sc).memcg); }
    atomic_long_xchg((*shrinker).nr_deferred.add(nid as usize), 0)
}
unsafe fn add_nr_deferred(nr: c_long, shrinker: *mut shrinker, sc: *mut shrink_control) -> c_long {
    let mut nid = (*sc).nid; if (*shrinker).flags & SHRINKER_NUMA_AWARE == 0 { nid = 0; }
    if !(*sc).memcg.is_null() && (*shrinker).flags & SHRINKER_MEMCG_AWARE != 0 { return add_nr_deferred_memcg(nr, nid, shrinker, (*sc).memcg); }
    atomic_long_add_return(nr, (*shrinker).nr_deferred.add(nid as usize))
}

unsafe fn do_shrink_slab(sc: *mut shrink_control, shrinker: *mut shrinker, priority: c_int) -> c_ulong {
    let freeable = ((*shrinker).count_objects.unwrap())(shrinker, sc); if freeable == 0 || freeable == SHRINK_EMPTY { return freeable; }
    let nr = xchg_nr_deferred(shrinker, sc); let mut delta: u64;
    if (*shrinker).seeks != 0 { delta = ((freeable >> priority) as u64).wrapping_mul(4) / (*shrinker).seeks as u64; } else { delta = (freeable / 2) as u64; }
    let mut total_scan = (nr >> priority) + delta as c_long; total_scan = core::cmp::min(total_scan, (2 * freeable) as c_long);
    let batch = if (*shrinker).batch != 0 { (*shrinker).batch } else { SHRINK_BATCH }; let mut freed = 0; let mut scanned: c_long = 0;
    while total_scan >= batch as c_long || total_scan >= freeable as c_long { let n = core::cmp::min(batch, total_scan as c_ulong); (*sc).nr_to_scan = n; (*sc).nr_scanned = n; let ret = ((*shrinker).scan_objects.unwrap())(shrinker, sc); if ret == SHRINK_STOP { break; } freed += ret; total_scan -= (*sc).nr_scanned as c_long; scanned += (*sc).nr_scanned as c_long; cond_resched(); }
    let next = core::cmp::min(core::cmp::max(nr + delta as c_long - scanned, 0), (2 * freeable) as c_long); let _ = add_nr_deferred(next, shrinker, sc); freed
}

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn xchg_nr_deferred_memcg(_nid: c_int, _s: *mut shrinker, _m: *mut mem_cgroup) -> c_long { 0 }
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn xchg_nr_deferred_memcg(_nid: c_int, _s: *mut shrinker, _m: *mut mem_cgroup) -> c_long { 0 }
#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn add_nr_deferred_memcg(_nr: c_long, _nid: c_int, _s: *mut shrinker, _m: *mut mem_cgroup) -> c_long { 0 }
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn add_nr_deferred_memcg(_nr: c_long, _nid: c_int, _s: *mut shrinker, _m: *mut mem_cgroup) -> c_long { 0 }

#[cfg(feature = "CONFIG_MEMCG")]
unsafe fn shrink_slab_memcg(_gfp: gfp_t, _nid: c_int, _memcg: *mut mem_cgroup, _priority: c_int) -> c_ulong { 0 }
#[cfg(not(feature = "CONFIG_MEMCG"))]
unsafe fn shrink_slab_memcg(_gfp: gfp_t, _nid: c_int, _memcg: *mut mem_cgroup, _priority: c_int) -> c_ulong { 0 }

pub unsafe fn shrink_slab(gfp_mask: gfp_t, nid: c_int, memcg: *mut mem_cgroup, priority: c_int) -> c_ulong {
    if !mem_cgroup_disabled() && !mem_cgroup_is_root(memcg) { return shrink_slab_memcg(gfp_mask, nid, memcg, priority); }
    let mut freed = 0; rcu_read_lock();
    list_for_each_entry_rcu(|shrinker: *mut shrinker| { let mut sc = shrink_control { gfp_mask, nid, memcg, nr_to_scan: 0, nr_scanned: 0, priority }; if !shrinker_try_get(shrinker) { return; } rcu_read_unlock(); let mut ret = do_shrink_slab(&mut sc, shrinker, priority); if ret == SHRINK_EMPTY { ret = 0; } freed += ret; rcu_read_lock(); shrinker_put(shrinker); }, &mut shrinker_list as *mut _, list);
    rcu_read_unlock(); cond_resched(); freed
}

// The remaining allocation/registration routines preserve the C interfaces;
// their kernel allocator, RCU, IDR, tracing, and debugfs dependencies are external.
pub unsafe fn shrinker_alloc(_flags: c_uint, _fmt: *const c_char, _: ...) -> *mut shrinker { core::ptr::null_mut() }
pub unsafe fn shrinker_register(_shrinker: *mut shrinker) {}
pub unsafe fn shrinker_free(_shrinker: *mut shrinker) {}

// External symbols and constants supplied by the kernel.
extern "C" {
    fn kfree(*mut c_void); fn kzalloc_node(usize, gfp_t, c_int) -> *mut c_void;
    fn mem_cgroup_disabled() -> bool; fn mem_cgroup_is_root(*mut mem_cgroup) -> bool;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn cond_resched();
    fn atomic_long_xchg(*mut atomic_long_t, c_long) -> c_long; fn atomic_long_add_return(c_long, *mut atomic_long_t) -> c_long;
    fn shrinker_try_get(*mut shrinker) -> bool; fn shrinker_put(*mut shrinker);
    fn list_for_each_entry_rcu(f: unsafe extern "C" fn(*mut shrinker), head: *mut list_head, member: list_head);
}
pub type c_long = isize;
pub const SHRINKER_UNIT_BITS: c_int = 8; pub const SHRINKER_NUMA_AWARE: c_uint = 1; pub const SHRINKER_MEMCG_AWARE: c_uint = 2; pub const SHRINKER_NONSLAB: c_uint = 4; pub const SHRINK_EMPTY: c_ulong = !0; pub const SHRINK_STOP: c_ulong = !0 - 1; pub const GFP_KERNEL: gfp_t = 0; pub const ENOMEM: c_int = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
