// SPDX-License-Identifier: GPL-2.0-only
// External Linux kernel headers and symbols are supplied by other files.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
mod kernel {
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_ulong = usize;
    pub type c_ulonglong = u64;
    pub type u32 = u32;
    pub type cycles_t = u64;

    #[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
    #[repr(C)] pub struct interval_tree_node { pub start: c_ulong, pub last: c_ulong, _private: [u8; 0] }
    #[repr(C)] pub struct rnd_state { _private: [u8; 0] }
    #[repr(C)] pub struct ma_state { pub status: c_ulong, pub index: c_ulong, pub last: c_ulong }
    #[repr(C)] pub struct interval_tree_span_iter {
        pub first_index: c_ulong, pub last_index: c_ulong, pub is_hole: c_int,
        pub start_hole: c_ulong, pub last_hole: c_ulong,
        pub start_used: c_ulong, pub last_used: c_ulong,
    }
    #[repr(C)] pub struct maple_tree { _private: [u8; 0] }

    extern "C" {
        pub static mut root: rb_root_cached;
    }
}

use kernel::*;

static mut nnodes: c_int = 100;
static mut perf_loops: c_int = 1000;
static mut nsearches: c_int = 100;
static mut search_loops: c_int = 1000;
static mut search_all: bool = false;
static mut max_endpoint: c_uint = u32::MAX;
static mut seed: c_ulonglong = 3141592653589793238;

static mut root: rb_root_cached = rb_root_cached { _private: [] };
static mut nodes: *mut interval_tree_node = core::ptr::null_mut();
static mut queries: *mut u32 = core::ptr::null_mut();
static mut rnd: rnd_state = rnd_state { _private: [] };

extern "C" {
    fn interval_tree_iter_first(root: *mut rb_root_cached, start: c_ulong, last: c_ulong) -> *mut interval_tree_node;
    fn interval_tree_iter_next(node: *mut interval_tree_node, start: c_ulong, last: c_ulong) -> *mut interval_tree_node;
    fn interval_tree_insert(node: *mut interval_tree_node, root: *mut rb_root_cached);
    fn interval_tree_remove(node: *mut interval_tree_node, root: *mut rb_root_cached);
    fn prandom_u32_state(state: *mut rnd_state) -> u32;
    fn prandom_seed_state(state: *mut rnd_state, seed: c_ulonglong);
    fn get_cycles() -> cycles_t;
    fn div_u64(value: u64, divisor: u64) -> u64;
    fn printk(format: *const u8, ...);
    fn pr_warn(format: *const u8, ...);
    fn kmalloc_objs<T>(count: c_int) -> *mut T;
    fn kmalloc_array<T>(count: c_int, size: usize) -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn bitmap_alloc(bits: c_int, flags: c_ulong) -> *mut c_ulong;
    fn bitmap_free(bitmap: *mut c_ulong);
    fn bitmap_zero(bitmap: *mut c_ulong, bits: c_int);
    fn bitmap_set(bitmap: *mut c_ulong, start: c_int, len: c_int);
    fn bitmap_equal(a: *const c_ulong, b: *const c_ulong, bits: c_int) -> bool;
    fn WARN_ONCE(condition: bool, format: *const u8, ...);
    fn WARN_ON_ONCE(condition: bool);
}

unsafe fn search(root: *mut rb_root_cached, start: c_ulong, last: c_ulong) -> c_ulong {
    let mut node = interval_tree_iter_first(root, start, last);
    let mut results: c_ulong = 0;
    while !node.is_null() {
        results += 1;
        node = interval_tree_iter_next(node, start, last);
    }
    results
}

unsafe fn init() {
    for i in 0..nnodes {
        let b = (prandom_u32_state(&mut rnd) >> 4) % max_endpoint;
        let a = (prandom_u32_state(&mut rnd) >> 4) % b;
        (*nodes.add(i as usize)).start = a as c_ulong;
        (*nodes.add(i as usize)).last = b as c_ulong;
    }
    for i in 0..nsearches {
        *queries.add(i as usize) = (prandom_u32_state(&mut rnd) >> 4) % max_endpoint;
    }
}

unsafe fn basic_check() -> c_int {
    printk(b"interval tree insert/remove\0".as_ptr());
    init();
    let time1 = get_cycles();
    for _ in 0..perf_loops {
        for j in 0..nnodes { interval_tree_insert(nodes.add(j as usize), &mut root); }
        for j in 0..nnodes { interval_tree_remove(nodes.add(j as usize), &mut root); }
    }
    let time = div_u64(get_cycles() - time1, perf_loops as u64);
    printk(b" -> %llu cycles\n\0".as_ptr(), time as c_ulonglong);
    0
}

unsafe fn search_check() -> c_int {
    printk(b"interval tree search\0".as_ptr());
    init();
    for j in 0..nnodes { interval_tree_insert(nodes.add(j as usize), &mut root); }
    let time1 = get_cycles();
    let mut results: c_ulong = 0;
    for _ in 0..search_loops { for j in 0..nsearches {
        let start = if search_all { 0 } else { *queries.add(j as usize) as c_ulong };
        let last = if search_all { max_endpoint as c_ulong } else { *queries.add(j as usize) as c_ulong };
        results += search(&mut root, start, last);
    }}
    let time = div_u64(get_cycles() - time1, search_loops as u64);
    results = div_u64(results as u64, search_loops as u64) as c_ulong;
    printk(b" -> %llu cycles (%lu results)\n\0".as_ptr(), time as c_ulonglong, results);
    for j in 0..nnodes { interval_tree_remove(nodes.add(j as usize), &mut root); }
    0
}

unsafe fn intersection_range_check() -> c_int {
    printk(b"interval tree iteration\n\0".as_ptr());
    let intxn1 = bitmap_alloc(nnodes, 0);
    if intxn1.is_null() { WARN_ONCE(true, b"Failed to allocate intxn1\n\0".as_ptr()); return -12; }
    let intxn2 = bitmap_alloc(nnodes, 0);
    if intxn2.is_null() { WARN_ONCE(true, b"Failed to allocate intxn2\n\0".as_ptr()); bitmap_free(intxn1); return -12; }
    for _ in 0..search_loops {
        init();
        for j in 0..nnodes { interval_tree_insert(nodes.add(j as usize), &mut root); }
        for k in 0..nsearches {
            let (start, last) = if k == 0 { (0, c_ulong::MAX) } else {
                let last = ((prandom_u32_state(&mut rnd) >> 4) % max_endpoint) as c_ulong;
                (((prandom_u32_state(&mut rnd) >> 4) as c_ulong) % last, last)
            };
            bitmap_zero(intxn1, nnodes);
            for j in 0..nnodes { let node = nodes.add(j as usize); if start <= (*node).last && last >= (*node).start { bitmap_set(intxn1, j, 1); } }
            bitmap_zero(intxn2, nnodes);
            let mut node = interval_tree_iter_first(&mut root, start, last);
            while !node.is_null() { bitmap_set(intxn2, node.offset_from(nodes) as c_int, 1); node = interval_tree_iter_next(node, start, last); }
            WARN_ON_ONCE(!bitmap_equal(intxn1, intxn2, nnodes));
        }
        for j in 0..nnodes { interval_tree_remove(nodes.add(j as usize), &mut root); }
    }
    bitmap_free(intxn1); bitmap_free(intxn2); 0
}

#[cfg(feature = "CONFIG_INTERVAL_TREE_SPAN_ITER")]
unsafe fn span_iteration_check() -> c_int { 0 }
#[cfg(not(feature = "CONFIG_INTERVAL_TREE_SPAN_ITER"))]
unsafe fn span_iteration_check() -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn interval_tree_test_init() -> c_int {
    if nnodes <= 0 || nsearches <= 0 || perf_loops <= 0 || search_loops <= 0 || max_endpoint < 2 { return -22; }
    nodes = kmalloc_objs::<interval_tree_node>(nnodes);
    if nodes.is_null() { return -12; }
    queries = kmalloc_array::<u32>(nsearches, core::mem::size_of::<u32>());
    if queries.is_null() { kfree(nodes.cast()); return -12; }
    prandom_seed_state(&mut rnd, seed);
    basic_check(); search_check(); intersection_range_check(); span_iteration_check();
    kfree(queries.cast()); kfree(nodes.cast()); -11
}

#[no_mangle]
pub unsafe extern "C" fn interval_tree_test_exit() { printk(b"test exit\n\0".as_ptr()); }

// module_init(interval_tree_test_init)
// module_exit(interval_tree_test_exit)
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Michel Lespinasse");
// MODULE_DESCRIPTION("Interval Tree test");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
