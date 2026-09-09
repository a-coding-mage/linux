// SPDX-License-Identifier: GPL-2.0
/* KASAN quarantine. */

use core::ffi::c_void;

#[repr(C)]
pub struct qlist_node { pub next: *mut qlist_node }
#[repr(C)]
pub struct qlist_head {
    pub head: *mut qlist_node,
    pub tail: *mut qlist_node,
    pub bytes: usize,
    pub offline: bool,
}

#[repr(C)] pub struct kasan_info { pub free_meta_offset: usize }
#[repr(C)] pub struct kmem_cache { pub size: usize, pub kasan_info: kasan_info }
#[repr(C)] pub struct kasan_free_meta { pub quarantine_link: qlist_node }
#[repr(C)] pub struct raw_spinlock_t { _private: usize }
#[repr(C)] pub struct srcu_struct { _private: usize }
#[repr(C)] pub struct cpu_shrink_qlist { pub lock: raw_spinlock_t, pub qlist: qlist_head }

const QUARANTINE_PERCPU_SIZE: usize = 1 << 20;
const QUARANTINE_FRACTION: usize = 32;
const CONFIG_NR_CPUS: usize = 1;
const QUARANTINE_BATCHES: usize = if 1024 > 4 * CONFIG_NR_CPUS { 1024 } else { 4 * CONFIG_NR_CPUS };

static mut CPU_QUARANTINE: qlist_head = qlist_head { head: core::ptr::null_mut(), tail: core::ptr::null_mut(), bytes: 0, offline: false };
static mut GLOBAL_QUARANTINE: [qlist_head; QUARANTINE_BATCHES] = [qlist_head { head: core::ptr::null_mut(), tail: core::ptr::null_mut(), bytes: 0, offline: false }; QUARANTINE_BATCHES];
static mut QUARANTINE_HEAD: i32 = 0;
static mut QUARANTINE_TAIL: i32 = 0;
static mut QUARANTINE_SIZE: usize = 0;
static mut QUARANTINE_MAX_SIZE: usize = 0;
static mut QUARANTINE_BATCH_SIZE: usize = 0;
static mut SHRINK_QLIST: cpu_shrink_qlist = cpu_shrink_qlist { lock: raw_spinlock_t { _private: 0 }, qlist: qlist_head { head: core::ptr::null_mut(), tail: core::ptr::null_mut(), bytes: 0, offline: false } };
static mut QUARANTINE_LOCK: raw_spinlock_t = raw_spinlock_t { _private: 0 };
static mut REMOVE_CACHE_SRCU: srcu_struct = srcu_struct { _private: 0 };

extern "C" {
    fn virt_to_slab(q: *mut qlist_node) -> *mut kmem_cache;
    fn kasan_get_free_meta(c: *mut kmem_cache, object: *mut c_void) -> *mut kasan_free_meta;
    fn slab_want_init_on_free(c: *mut kmem_cache) -> bool;
    fn memzero_explicit(p: *mut c_void, n: usize);
    fn ___cache_free(c: *mut kmem_cache, object: *mut c_void, ip: *mut c_void);
    fn totalram_pages() -> usize; fn num_online_cpus() -> usize;
    fn srcu_read_lock(s: *mut srcu_struct) -> i32; fn srcu_read_unlock(s: *mut srcu_struct, i: i32);
    fn synchronize_srcu(s: *mut srcu_struct); fn cond_resched();
    fn on_each_cpu(f: unsafe extern "C" fn(*mut c_void), arg: *mut c_void, wait: i32);
    fn cpuhp_setup_state(state: i32, name: *const u8, on: unsafe extern "C" fn(u32) -> i32, off: unsafe extern "C" fn(u32) -> i32) -> i32;
}

unsafe fn qlist_empty(q: *mut qlist_head) -> bool { (*q).head.is_null() }
unsafe fn qlist_init(q: *mut qlist_head) { (*q).head = core::ptr::null_mut(); (*q).tail = core::ptr::null_mut(); (*q).bytes = 0; }
unsafe fn qlist_put(q: *mut qlist_head, link: *mut qlist_node, size: usize) {
    if qlist_empty(q) { (*q).head = link; } else { (*(*q).tail).next = link; }
    (*q).tail = link; (*link).next = core::ptr::null_mut(); (*q).bytes = (*q).bytes.wrapping_add(size);
}
unsafe fn qlist_move_all(from: *mut qlist_head, to: *mut qlist_head) {
    if qlist_empty(from) { return; }
    if qlist_empty(to) { core::ptr::write(to, core::ptr::read(from)); qlist_init(from); return; }
    (*(*to).tail).next = (*from).head; (*to).tail = (*from).tail; (*to).bytes += (*from).bytes; qlist_init(from);
}
unsafe fn qlink_to_cache(q: *mut qlist_node) -> *mut kmem_cache { virt_to_slab(q) }
unsafe fn qlink_to_object(q: *mut qlist_node, cache: *mut kmem_cache) -> *mut c_void {
    let free_info = q as *mut kasan_free_meta;
    (free_info as *mut u8).sub((*cache).kasan_info.free_meta_offset) as *mut c_void
}
unsafe fn qlink_free(q: *mut qlist_node, cache: *mut kmem_cache) {
    let object = qlink_to_object(q, cache); let free_meta = kasan_get_free_meta(cache, object);
    if slab_want_init_on_free(cache) && (*cache).kasan_info.free_meta_offset == 0 { memzero_explicit(free_meta as *mut c_void, core::mem::size_of::<kasan_free_meta>()); }
    ___cache_free(cache, object, core::ptr::null_mut());
}
unsafe fn qlist_free_all(q: *mut qlist_head, cache: *mut kmem_cache) {
    if qlist_empty(q) { return; } let mut link = (*q).head;
    while !link.is_null() { let next = (*link).next; let c = if cache.is_null() { qlink_to_cache(link) } else { cache }; qlink_free(link, c); link = next; }
    qlist_init(q);
}

#[no_mangle] pub unsafe extern "C" fn kasan_quarantine_put(cache: *mut kmem_cache, object: *mut c_void) -> bool {
    let meta = kasan_get_free_meta(cache, object); if meta.is_null() { return false; }
    if CPU_QUARANTINE.offline { return false; }
    qlist_put(&mut CPU_QUARANTINE, &mut (*meta).quarantine_link, (*cache).size);
    if CPU_QUARANTINE.bytes > QUARANTINE_PERCPU_SIZE { let mut temp = qlist_head { head: core::ptr::null_mut(), tail: core::ptr::null_mut(), bytes: 0, offline: false }; qlist_move_all(&mut CPU_QUARANTINE, &mut temp); QUARANTINE_SIZE += temp.bytes; qlist_move_all(&mut temp, &mut GLOBAL_QUARANTINE[QUARANTINE_TAIL as usize]); if GLOBAL_QUARANTINE[QUARANTINE_TAIL as usize].bytes >= QUARANTINE_BATCH_SIZE { let n = (QUARANTINE_TAIL + 1) % QUARANTINE_BATCHES as i32; if n != QUARANTINE_HEAD { QUARANTINE_TAIL = n; } } }
    true
}

#[no_mangle] pub unsafe extern "C" fn kasan_quarantine_reduce() {
    if QUARANTINE_SIZE <= QUARANTINE_MAX_SIZE { return; }
    let total = (totalram_pages() << 12) / QUARANTINE_FRACTION; let percpu = QUARANTINE_PERCPU_SIZE * num_online_cpus();
    QUARANTINE_MAX_SIZE = total.saturating_sub(percpu); QUARANTINE_BATCH_SIZE = core::cmp::max(QUARANTINE_PERCPU_SIZE, 2 * total / QUARANTINE_BATCHES);
    if QUARANTINE_SIZE > QUARANTINE_MAX_SIZE { let mut free = qlist_head { head: core::ptr::null_mut(), tail: core::ptr::null_mut(), bytes: 0, offline: false }; qlist_move_all(&mut GLOBAL_QUARANTINE[QUARANTINE_HEAD as usize], &mut free); QUARANTINE_SIZE -= free.bytes; QUARANTINE_HEAD = (QUARANTINE_HEAD + 1) % QUARANTINE_BATCHES as i32; qlist_free_all(&mut free, core::ptr::null_mut()); }
}

#[no_mangle] pub unsafe extern "C" fn kasan_quarantine_remove_cache(cache: *mut kmem_cache) {
    let mut free = qlist_head { head: core::ptr::null_mut(), tail: core::ptr::null_mut(), bytes: 0, offline: false };
    for i in 0..QUARANTINE_BATCHES { if !qlist_empty(&mut GLOBAL_QUARANTINE[i]) { let old = GLOBAL_QUARANTINE[i].bytes; let mut cur = GLOBAL_QUARANTINE[i].head; qlist_init(&mut GLOBAL_QUARANTINE[i]); while !cur.is_null() { let n = (*cur).next; if qlink_to_cache(cur) == cache { qlist_put(&mut free, cur, old); } else { qlist_put(&mut GLOBAL_QUARANTINE[i], cur, old); } cur = n; } QUARANTINE_SIZE -= old - GLOBAL_QUARANTINE[i].bytes; } }
    qlist_free_all(&mut free, cache);
}

unsafe fn qlist_move_cache(from: *mut qlist_head, to: *mut qlist_head, cache: *mut kmem_cache) {
    if qlist_empty(from) { return; }
    let mut cur = (*from).head; qlist_init(from);
    while !cur.is_null() { let next = (*cur).next; let obj_cache = qlink_to_cache(cur); if obj_cache == cache { qlist_put(to, cur, (*obj_cache).size); } else { qlist_put(from, cur, (*obj_cache).size); } cur = next; }
}
unsafe extern "C" fn __per_cpu_remove_cache(q: *mut qlist_head, arg: *mut c_void) { qlist_move_cache(q, &mut SHRINK_QLIST.qlist, arg as *mut kmem_cache); }
unsafe extern "C" fn per_cpu_remove_cache(arg: *mut c_void) { if !CPU_QUARANTINE.offline { __per_cpu_remove_cache(&mut CPU_QUARANTINE, arg); } }
unsafe extern "C" fn kasan_cpu_online(_cpu: u32) -> i32 { CPU_QUARANTINE.offline = false; 0 }
unsafe extern "C" fn kasan_cpu_offline(_cpu: u32) -> i32 { CPU_QUARANTINE.offline = true; qlist_free_all(&mut CPU_QUARANTINE, core::ptr::null_mut()); 0 }
unsafe extern "C" fn kasan_cpu_quarantine_init() -> i32 {
    let ret = cpuhp_setup_state(0, b"mm/kasan:online\0".as_ptr(), kasan_cpu_online, kasan_cpu_offline);
    if ret < 0 { /* pr_err("cpu quarantine register failed [%d]", ret); */ } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
