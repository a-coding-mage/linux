// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies are supplied by the surrounding translation unit.

extern "C" {
    static mut __start_static_call_sites: static_call_site;
    static mut __stop_static_call_sites: static_call_site;
    static mut __start_static_call_tramp_key: static_call_tramp_key;
    static mut __stop_static_call_tramp_key: static_call_tramp_key;
    static mut system_state: i32;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn arch_static_call_transform(site: *mut core::ffi::c_void, tramp: *mut core::ffi::c_void,
                                  func: *mut core::ffi::c_void, tail: bool);
    fn kernel_text_address(addr: usize) -> bool;
    fn within_module_init(addr: usize, module: *mut module) -> bool;
    fn init_section_contains(addr: *mut core::ffi::c_void, size: usize) -> bool;
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn pr_warn(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn register_module_notifier(nb: *mut notifier_block);
    fn notifier_from_errno(errno: i32) -> i32;
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn __module_text_address(addr: usize) -> *mut module;
    fn kfree(ptr: *mut core::ffi::c_void);
}

static mut static_call_initialized: i32 = 0;
static mut static_call_mutex: mutex = mutex::new();

pub unsafe fn static_call_force_reinit() {
    if static_call_initialized == 0 {
        return;
    }
    static_call_initialized += 1;
}

unsafe fn static_call_lock() { mutex_lock(&raw mut static_call_mutex); }
unsafe fn static_call_unlock() { mutex_unlock(&raw mut static_call_mutex); }

unsafe fn static_call_addr(site: *mut static_call_site) -> *mut core::ffi::c_void {
    (site as isize + (*site).addr as isize) as *mut core::ffi::c_void
}

unsafe fn __static_call_key(site: *const static_call_site) -> usize {
    (site as isize + (*site).key as isize) as usize
}

unsafe fn static_call_key(site: *const static_call_site) -> *mut static_call_key {
    (__static_call_key(site) & !STATIC_CALL_SITE_FLAGS) as *mut static_call_key
}

unsafe fn static_call_is_init(site: *mut static_call_site) -> bool { __static_call_key(site) & STATIC_CALL_SITE_INIT != 0 }
unsafe fn static_call_is_tail(site: *mut static_call_site) -> bool { __static_call_key(site) & STATIC_CALL_SITE_TAIL != 0 }
unsafe fn static_call_set_init(site: *mut static_call_site) {
    (*site).key = ((__static_call_key(site) | STATIC_CALL_SITE_INIT) as isize - site.offset(0).cast::<isize>() as isize) as _;
}

unsafe extern "C" fn static_call_site_cmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let key_a = static_call_key(a as *const static_call_site);
    let key_b = static_call_key(b as *const static_call_site);
    if key_a < key_b { -1 } else if key_a > key_b { 1 } else { 0 }
}

unsafe extern "C" fn static_call_site_swap(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, _size: i32) {
    let delta = b as isize - a as isize;
    let a = a as *mut static_call_site;
    let b = b as *mut static_call_site;
    let tmp = *a;
    (*a).addr = (*b).addr.wrapping_sub(delta as _);
    (*a).key = (*b).key.wrapping_sub(delta as _);
    (*b).addr = tmp.addr.wrapping_add(delta as _);
    (*b).key = tmp.key.wrapping_add(delta as _);
}

unsafe fn static_call_sort_entries(start: *mut static_call_site, stop: *mut static_call_site) {
    sort(start as *mut _, stop.offset_from(start) as usize, core::mem::size_of::<static_call_site>(),
         Some(static_call_site_cmp), Some(static_call_site_swap));
}

unsafe fn static_call_key_has_mods(key: *mut static_call_key) -> bool { (*key).type_ & 1 == 0 }
unsafe fn static_call_key_next(key: *mut static_call_key) -> *mut static_call_mod {
    if !static_call_key_has_mods(key) { core::ptr::null_mut() } else { (*key).mods }
}
unsafe fn static_call_key_sites(key: *mut static_call_key) -> *mut static_call_site {
    if static_call_key_has_mods(key) { core::ptr::null_mut() } else { ((*key).type_ & !1) as *mut static_call_site }
}

pub unsafe fn __static_call_update(key: *mut static_call_key, tramp: *mut core::ffi::c_void, func: *mut core::ffi::c_void) {
    cpus_read_lock(); static_call_lock();
    if (*key).func == func { static_call_unlock(); cpus_read_unlock(); return; }
    (*key).func = func;
    arch_static_call_transform(core::ptr::null_mut(), tramp, func, false);
    if static_call_initialized == 0 { static_call_unlock(); cpus_read_unlock(); return; }
    let mut site_mod = static_call_key_next(key);
    let mut first = static_call_mod { next: site_mod, mod_: core::ptr::null_mut(), sites: static_call_key_sites(key) };
    let mut current: *mut static_call_mod = &mut first;
    while !current.is_null() {
        let init = system_state < SYSTEM_RUNNING;
        let module = (*current).mod_;
        if (*current).sites.is_null() { current = (*current).next; continue; }
        let mut stop = &raw mut __stop_static_call_sites;
        if !module.is_null() { stop = (*module).static_call_sites.add((*module).num_static_call_sites as usize); }
        let mut site = (*current).sites;
        while site < stop && static_call_key(site) == key {
            let addr = static_call_addr(site);
            if !init && static_call_is_init(site) { site = site.add(1); continue; }
            if !kernel_text_address(addr as usize) { site = site.add(1); continue; }
            arch_static_call_transform(addr, tramp, func, static_call_is_tail(site));
            site = site.add(1);
        }
        current = (*current).next;
    }
    static_call_unlock(); cpus_read_unlock();
}

unsafe fn addr_conflict(site: *mut static_call_site, start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    let addr = static_call_addr(site) as usize;
    if addr <= end as usize && addr + CALL_INSN_SIZE > start as usize { 1 } else { 0 }
}

unsafe fn __static_call_text_reserved(mut iter: *mut static_call_site, stop: *mut static_call_site, start: *mut core::ffi::c_void, end: *mut core::ffi::c_void, init: bool) -> i32 {
    while iter < stop { if (init || !static_call_is_init(iter)) && addr_conflict(iter, start, end) != 0 { return 1; } iter = iter.add(1); }
    0
}

pub unsafe fn static_call_text_reserved(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> i32 {
    __static_call_text_reserved(&raw mut __start_static_call_sites, &raw mut __stop_static_call_sites, start, end, system_state < SYSTEM_RUNNING)
}

unsafe fn __static_call_init(module: *mut module, mut start: *mut static_call_site, stop: *mut static_call_site) -> i32 {
    if start == stop { return 0; }
    static_call_sort_entries(start, stop);
    let mut prev_key: *mut static_call_key = core::ptr::null_mut();
    while start < stop {
        let site_addr = static_call_addr(start);
        if (!module.is_null() && within_module_init(site_addr as usize, module)) ||
           (module.is_null() && init_section_contains(site_addr, 1)) { static_call_set_init(start); }
        let key = static_call_key(start);
        if key != prev_key {
            prev_key = key;
            if module.is_null() {
                (*key).sites = start;
                (*key).type_ |= 1;
            } else {
                // C's kzalloc_obj() allocation and -ENOMEM paths are external kernel services.
                let site_mod = kzalloc_obj::<static_call_mod>();
                if site_mod.is_null() { return -12; }
                (*site_mod).mod_ = module;
                (*site_mod).sites = start;
                (*site_mod).next = static_call_key_next(key);
                (*key).mods = site_mod;
            }
        }
        arch_static_call_transform(site_addr, core::ptr::null_mut(), (*key).func, static_call_is_tail(start));
        start = start.add(1);
    }
    0
}

#[cfg(not(feature = "config_modules"))]
unsafe fn __static_call_mod_text_reserved(_start: *mut core::ffi::c_void, _end: *mut core::ffi::c_void) -> i32 { 0 }

pub unsafe fn static_call_init() -> i32 {
    // See static_call_force_reinit().
    if static_call_initialized == 1 { return 0; }
    cpus_read_lock(); static_call_lock();
    let ret = __static_call_init(core::ptr::null_mut(), &raw mut __start_static_call_sites, &raw mut __stop_static_call_sites);
    static_call_unlock(); cpus_read_unlock();
    if ret != 0 { pr_err(b"Failed to allocate memory for static_call!\0".as_ptr()); BUG(); }
    static_call_initialized = 1;
    0
}

#[cfg(feature = "config_static_call_selftest")]
unsafe fn func_a(x: i32) -> i32 { x + 1 }
#[cfg(feature = "config_static_call_selftest")]
unsafe fn func_b(x: i32) -> i32 { x + 2 }

#[cfg(feature = "config_static_call_selftest")]
#[repr(C)]
struct static_call_data { func: Option<unsafe fn(i32) -> i32>, val: i32, expect: i32 }

#[cfg(feature = "config_static_call_selftest")]
static mut STATIC_CALL_DATA: [static_call_data; 3] = [
    static_call_data { func: None, val: 2, expect: 3 },
    static_call_data { func: Some(func_b), val: 2, expect: 4 },
    static_call_data { func: Some(func_a), val: 2, expect: 3 },
];

// C-only registration macros (early_initcall and EXPORT_SYMBOL_GPL) are intentionally represented by Rust linkage items above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
