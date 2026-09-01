// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/maps.c. C include dependencies are intentionally
// represented as external declarations and opaque C-compatible types.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;
type size_t = usize;
type bool_ = bool;

const ENOMEM: c_int = 12;
const INT_MAX: c_uint = c_int::MAX as c_uint;

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmap {
    pub kmaps: *mut maps,
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub addr: u64,
    pub al_addr: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct unwind_libunwind_ops {
    _private: [u8; 0],
}

/*
 * Locking/sorting note:
 *
 * Sorting is done with the write lock, iteration and binary searching happens
 * under the read lock requiring being sorted. There is a race between sorting
 * releasing the write lock and acquiring the read lock for iteration/searching
 * where another thread could insert and break the sorting of the maps. In
 * practice inserting maps should be rare meaning that the race shouldn't lead
 * to live lock. Removal of maps doesn't break being sorted.
 */

#[repr(C)]
pub struct maps {
    pub lock: rw_semaphore,
    /**
     * @maps_by_address: array of maps sorted by their starting address if
     * maps_by_address_sorted is true.
     */
    pub maps_by_address: *mut *mut map,
    /**
     * @maps_by_name: optional array of maps sorted by their dso name if
     * maps_by_name_sorted is true.
     */
    pub maps_by_name: *mut *mut map,
    pub machine: *mut machine,
    /*
     * HAVE_LIBUNWIND_SUPPORT:
     * void *addr_space;
     * const struct unwind_libunwind_ops *unwind_libunwind_ops;
     * uint16_t e_machine;
     */
    pub addr_space: *mut c_void,
    pub unwind_libunwind_ops: *const unwind_libunwind_ops,
    pub e_machine: u16,
    /*
     * HAVE_LIBDW_SUPPORT:
     * void *libdw_addr_space_dwfl;
     */
    pub libdw_addr_space_dwfl: *mut c_void,
    pub refcnt: refcount_t,
    /**
     * @nr_maps: number of maps_by_address, and possibly maps_by_name,
     * entries that contain maps.
     */
    pub nr_maps: c_uint,
    /**
     * @nr_maps_allocated: number of entries in maps_by_address and possibly
     * maps_by_name.
     */
    pub nr_maps_allocated: c_uint,
    /**
     * @last_search_by_name_idx: cache of last found by name entry's index
     * as frequent searches for the same dso name are common.
     */
    pub last_search_by_name_idx: c_uint,
    /** @maps_by_address_sorted: is maps_by_address sorted. */
    pub maps_by_address_sorted: bool,
    /** @maps_by_name_sorted: is maps_by_name sorted. */
    pub maps_by_name_sorted: bool,
    /** @ends_broken: does the map contain a map where end values are unset/unsorted? */
    pub ends_broken: bool,
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn bsearch(key: *const c_void, base: *const c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    fn init_rwsem(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut c_void);

    fn refcount_set(r: *mut refcount_t, n: c_uint);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn refcount_read(r: *const refcount_t) -> c_uint;

    fn map__start(map: *const map) -> u64;
    fn map__end(map: *const map) -> u64;
    fn map__set_start(map: *mut map, start: u64);
    fn map__set_end(map: *mut map, end: u64);
    fn map__add_pgoff(map: *mut map, pgoff: u64);
    fn map__map_ip(map: *const map, ip: u64) -> u64;
    fn map__dso(map: *const map) -> *mut dso;
    fn map__kmap(map: *mut map) -> *mut kmap;
    fn map__refcnt(map: *mut map) -> *mut refcount_t;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn map__zput(map: *mut map);
    fn map__clone(map: *mut map) -> *mut map;
    fn map__load(map: *mut map) -> c_int;
    fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    fn map__find_symbol_by_name(map: *mut map, name: *const c_char) -> *mut symbol;
    fn map__contains_symbol(map: *mut map, sym: *mut symbol) -> bool;
    fn map__fprintf(map: *mut map, fp: *mut FILE) -> size_t;

    fn dso__kernel(dso: *mut dso) -> bool;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn dso__fprintf(dso: *mut dso, fp: *mut FILE) -> size_t;

    fn unwind__finish_access(maps: *mut maps);
    fn unwind__prepare_access(maps: *mut maps, e_machine: u16) -> c_int;
    fn libdw__invalidate_dwfl(maps: *mut maps, dwfl: *mut c_void);

    fn pr_err(format: *const c_char, ...);
    fn pr_warning(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
    fn debug_file() -> *mut FILE;

    static mut verbose: c_int;
    static mut use_browser: bool;
}

unsafe fn RC_CHK_ACCESS<T>(ptr: *const T) -> *mut T {
    ptr as *mut T
}

unsafe fn RC_CHK_EQUAL<T>(a: *const T, b: *const T) -> bool {
    a == b
}

unsafe fn ADD_RC_CHK(result: *mut *mut maps, raw: *mut maps) -> bool {
    *result = raw;
    !raw.is_null()
}

unsafe fn RC_CHK_GET(result: *mut *mut maps, raw: *mut maps) -> bool {
    *result = raw;
    !raw.is_null()
}

unsafe fn RC_CHK_PUT(_maps: *mut maps) {}

unsafe fn RC_CHK_FREE(maps: *mut maps) {
    free(maps as *mut c_void);
}

unsafe fn check_invariants(_maps: *const maps) {
    /*
     * NDEBUG conditional assertions from the C source are intentionally omitted
     * from executable Rust in this isolated translation.
     */
}

unsafe fn maps__maps_by_address(maps: *const maps) -> *mut *mut map {
    (*RC_CHK_ACCESS(maps)).maps_by_address
}

unsafe fn maps__set_maps_by_address(maps: *mut maps, new: *mut *mut map) {
    (*RC_CHK_ACCESS(maps)).maps_by_address = new;
}

unsafe fn maps__set_nr_maps_allocated(maps: *mut maps, nr_maps_allocated: c_uint) {
    (*RC_CHK_ACCESS(maps)).nr_maps_allocated = nr_maps_allocated;
}

unsafe fn maps__set_nr_maps(maps: *mut maps, nr_maps: c_uint) {
    (*RC_CHK_ACCESS(maps)).nr_maps = nr_maps;
}

/* Not in the header, to aid reference counting. */
unsafe fn maps__maps_by_name(maps: *const maps) -> *mut *mut map {
    (*RC_CHK_ACCESS(maps)).maps_by_name
}

unsafe fn maps__set_maps_by_name(maps: *mut maps, new: *mut *mut map) {
    (*RC_CHK_ACCESS(maps)).maps_by_name = new;
}

unsafe fn maps__maps_by_address_sorted(maps: *const maps) -> bool {
    (*RC_CHK_ACCESS(maps)).maps_by_address_sorted
}

unsafe fn maps__set_maps_by_address_sorted(maps: *mut maps, value: bool) {
    (*RC_CHK_ACCESS(maps)).maps_by_address_sorted = value;
}

unsafe fn maps__maps_by_name_sorted(maps: *const maps) -> bool {
    (*RC_CHK_ACCESS(maps)).maps_by_name_sorted
}

unsafe fn maps__set_maps_by_name_sorted(maps: *mut maps, value: bool) {
    (*RC_CHK_ACCESS(maps)).maps_by_name_sorted = value;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__machine(maps: *const maps) -> *mut machine {
    (*RC_CHK_ACCESS(maps)).machine
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__nr_maps(maps: *const maps) -> c_uint {
    (*RC_CHK_ACCESS(maps)).nr_maps
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__refcnt(maps: *mut maps) -> *mut refcount_t {
    &mut (*RC_CHK_ACCESS(maps)).refcnt
}

/*
 * HAVE_LIBUNWIND_SUPPORT accessors from C are translated unconditionally here;
 * in the original build they are gated by that feature macro.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__addr_space(maps: *const maps) -> *mut c_void {
    (*RC_CHK_ACCESS(maps)).addr_space
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__set_addr_space(maps: *mut maps, addr_space: *mut c_void) {
    (*RC_CHK_ACCESS(maps)).addr_space = addr_space;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__e_machine(maps: *const maps) -> u16 {
    (*RC_CHK_ACCESS(maps)).e_machine
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__set_e_machine(maps: *mut maps, e_machine: u16) {
    (*RC_CHK_ACCESS(maps)).e_machine = e_machine;
}

/*
 * HAVE_LIBDW_SUPPORT accessors from C are translated unconditionally here; in
 * the original build they are gated by that feature macro.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__libdw_addr_space_dwfl(maps: *const maps) -> *mut c_void {
    (*RC_CHK_ACCESS(maps)).libdw_addr_space_dwfl
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__set_libdw_addr_space_dwfl(maps: *mut maps, dwfl: *mut c_void) {
    (*RC_CHK_ACCESS(maps)).libdw_addr_space_dwfl = dwfl;
}

unsafe fn maps__lock(maps: *mut maps) -> *mut rw_semaphore {
    &mut (*RC_CHK_ACCESS(maps)).lock
}

unsafe fn maps__init(maps: *mut maps, machine: *mut machine) {
    init_rwsem(maps__lock(maps));
    (*RC_CHK_ACCESS(maps)).maps_by_address = ptr::null_mut();
    (*RC_CHK_ACCESS(maps)).maps_by_name = ptr::null_mut();
    (*RC_CHK_ACCESS(maps)).machine = machine;
    (*RC_CHK_ACCESS(maps)).addr_space = ptr::null_mut();
    (*RC_CHK_ACCESS(maps)).unwind_libunwind_ops = ptr::null();
    (*RC_CHK_ACCESS(maps)).libdw_addr_space_dwfl = ptr::null_mut();
    refcount_set(maps__refcnt(maps), 1);
    (*RC_CHK_ACCESS(maps)).nr_maps = 0;
    (*RC_CHK_ACCESS(maps)).nr_maps_allocated = 0;
    (*RC_CHK_ACCESS(maps)).last_search_by_name_idx = 0;
    (*RC_CHK_ACCESS(maps)).maps_by_address_sorted = true;
    (*RC_CHK_ACCESS(maps)).maps_by_name_sorted = false;
}

unsafe fn maps__exit(maps: *mut maps) {
    let mut maps_by_address = maps__maps_by_address(maps);
    let mut maps_by_name = maps__maps_by_name(maps);

    for i in 0..maps__nr_maps(maps) {
        map__zput(*maps_by_address.add(i as usize));
        if !maps_by_name.is_null() {
            map__zput(*maps_by_name.add(i as usize));
        }
    }
    zfree(&mut maps_by_address as *mut _ as *mut c_void);
    zfree(&mut maps_by_name as *mut _ as *mut c_void);
    unwind__finish_access(maps);
    libdw__invalidate_dwfl(maps, maps__libdw_addr_space_dwfl(maps));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__new(machine: *mut machine) -> *mut maps {
    let mut result: *mut maps = ptr::null_mut();
    let maps_raw = zalloc(size_of::<maps>()) as *mut maps;

    if ADD_RC_CHK(&mut result, maps_raw) {
        maps__init(result, machine);
    }

    result
}

unsafe fn maps__delete(maps: *mut maps) {
    maps__exit(maps);
    RC_CHK_FREE(maps);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__get(maps: *mut maps) -> *mut maps {
    let mut result: *mut maps = ptr::null_mut();

    if RC_CHK_GET(&mut result, maps) {
        refcount_inc(maps__refcnt(maps));
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__put(maps: *mut maps) {
    if !maps.is_null() && refcount_dec_and_test(maps__refcnt(maps)) {
        maps__delete(maps);
    } else {
        RC_CHK_PUT(maps);
    }
}

unsafe fn __maps__free_maps_by_name(maps: *mut maps) {
    if maps__maps_by_name(maps).is_null() {
        return;
    }

    /*
     * Free everything to try to do it from the rbtree in the next search
     */
    for i in 0..maps__nr_maps(maps) {
        map__put(*maps__maps_by_name(maps).add(i as usize));
    }

    zfree(&mut (*RC_CHK_ACCESS(maps)).maps_by_name as *mut _ as *mut c_void);

    /* Consistent with maps__init(). When maps_by_name == NULL, maps_by_name_sorted == false */
    maps__set_maps_by_name_sorted(maps, false);
}

unsafe extern "C" fn map__start_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let map_a = *(a as *const *const map);
    let map_b = *(b as *const *const map);
    let map_a_start = map__start(map_a);
    let map_b_start = map__start(map_b);

    if map_a_start == map_b_start {
        let map_a_end = map__end(map_a);
        let map_b_end = map__end(map_b);

        if map_a_end == map_b_end {
            /* Ensure maps with the same addresses have a fixed order. */
            if RC_CHK_ACCESS(map_a) == RC_CHK_ACCESS(map_b) {
                return 0;
            }
            return if (RC_CHK_ACCESS(map_a) as isize) > (RC_CHK_ACCESS(map_b) as isize) { 1 } else { -1 };
        }
        return if map_a_end > map_b_end { 1 } else { -1 };
    }
    if map_a_start > map_b_start { 1 } else { -1 }
}

unsafe fn __maps__sort_by_address(maps: *mut maps) {
    if maps__maps_by_address_sorted(maps) {
        return;
    }

    qsort(
        maps__maps_by_address(maps) as *mut c_void,
        maps__nr_maps(maps) as size_t,
        size_of::<*mut map>(),
        map__start_cmp,
    );
    maps__set_maps_by_address_sorted(maps, true);
}

unsafe fn maps__sort_by_address(maps: *mut maps) {
    down_write(maps__lock(maps));
    __maps__sort_by_address(maps);
    up_write(maps__lock(maps));
}

unsafe extern "C" fn map__strcmp(a: *const c_void, b: *const c_void) -> c_int {
    let map_a = *(a as *const *const map);
    let map_b = *(b as *const *const map);
    let dso_a = map__dso(map_a);
    let dso_b = map__dso(map_b);
    let ret = strcmp(dso__short_name(dso_a), dso__short_name(dso_b));

    if ret == 0 && RC_CHK_ACCESS(map_a) != RC_CHK_ACCESS(map_b) {
        /* Ensure distinct but name equal maps have an order. */
        return map__start_cmp(a, b);
    }
    ret
}

unsafe fn maps__sort_by_name(maps: *mut maps) -> c_int {
    let mut err = 0;

    down_write(maps__lock(maps));
    if !maps__maps_by_name_sorted(maps) {
        let mut maps_by_name = maps__maps_by_name(maps);

        if maps_by_name.is_null() {
            maps_by_name = malloc((*RC_CHK_ACCESS(maps)).nr_maps_allocated as usize * size_of::<*mut map>()) as *mut *mut map;
            if maps_by_name.is_null() {
                err = -ENOMEM;
            } else {
                let maps_by_address = maps__maps_by_address(maps);
                let n = maps__nr_maps(maps);

                maps__set_maps_by_name(maps, maps_by_name);
                for i in 0..n {
                    *maps_by_name.add(i as usize) = map__get(*maps_by_address.add(i as usize));
                }
            }
        }
        if err == 0 {
            qsort(
                maps_by_name as *mut c_void,
                maps__nr_maps(maps) as size_t,
                size_of::<*mut map>(),
                map__strcmp,
            );
            maps__set_maps_by_name_sorted(maps, true);
        }
    }
    check_invariants(maps);
    up_write(maps__lock(maps));
    err
}

unsafe fn maps__by_address_index(maps: *const maps, map: *const map) -> c_uint {
    let maps_by_address = maps__maps_by_address(maps);

    if maps__maps_by_address_sorted(maps) {
        let mapp = bsearch(
            &map as *const _ as *const c_void,
            maps__maps_by_address(maps) as *const c_void,
            maps__nr_maps(maps) as size_t,
            size_of::<*mut map>(),
            map__start_cmp,
        ) as *mut *mut map;

        if !mapp.is_null() {
            return mapp.offset_from(maps_by_address) as c_uint;
        }
    } else {
        for i in 0..maps__nr_maps(maps) {
            if RC_CHK_ACCESS(*maps_by_address.add(i as usize)) == RC_CHK_ACCESS(map) {
                return i;
            }
        }
    }
    pr_err(c"Map missing from maps".as_ptr());
    -1i32 as c_uint
}

unsafe fn maps__by_name_index(maps: *const maps, map: *const map) -> c_uint {
    let maps_by_name = maps__maps_by_name(maps);

    if maps__maps_by_name_sorted(maps) {
        let mapp = bsearch(
            &map as *const _ as *const c_void,
            maps_by_name as *const c_void,
            maps__nr_maps(maps) as size_t,
            size_of::<*mut map>(),
            map__strcmp,
        ) as *mut *mut map;

        if !mapp.is_null() {
            return mapp.offset_from(maps_by_name) as c_uint;
        }
    } else {
        for i in 0..maps__nr_maps(maps) {
            if RC_CHK_ACCESS(*maps_by_name.add(i as usize)) == RC_CHK_ACCESS(map) {
                return i;
            }
        }
    }
    pr_err(c"Map missing from maps".as_ptr());
    -1i32 as c_uint
}

unsafe fn map__set_kmap_maps(map: *mut map, maps: *mut maps) {
    let dso: *mut dso;

    if map.is_null() {
        return;
    }

    dso = map__dso(map);

    if !dso.is_null() && dso__kernel(dso) {
        let kmap = map__kmap(map);

        if !kmap.is_null() {
            (*kmap).kmaps = maps;
        } else {
            pr_err(c"Internal error: kernel dso with non kernel map\n".as_ptr());
        }
    }
}

unsafe fn __maps__insert(maps: *mut maps, new: *mut map) -> c_int {
    let mut maps_by_address = maps__maps_by_address(maps);
    let mut maps_by_name = maps__maps_by_name(maps);
    let mut nr_maps = maps__nr_maps(maps);
    let mut nr_allocate = (*RC_CHK_ACCESS(maps)).nr_maps_allocated;

    if nr_maps + 1 > nr_allocate {
        nr_allocate = if nr_allocate == 0 { 32 } else { nr_allocate * 2 };

        maps_by_address = realloc(maps_by_address as *mut c_void, nr_allocate as usize * size_of::<*mut map>()) as *mut *mut map;
        if maps_by_address.is_null() {
            return -ENOMEM;
        }

        maps__set_maps_by_address(maps, maps_by_address);
        if !maps_by_name.is_null() {
            maps_by_name = realloc(maps_by_name as *mut c_void, nr_allocate as usize * size_of::<*mut map>()) as *mut *mut map;
            if maps_by_name.is_null() {
                /*
                 * If by name fails, just disable by name and it will
                 * recompute next time it is required.
                 */
                __maps__free_maps_by_name(maps);
            }
            maps__set_maps_by_name(maps, maps_by_name);
        }
        (*RC_CHK_ACCESS(maps)).nr_maps_allocated = nr_allocate;
    }
    /* Insert the value at the end. */
    *maps_by_address.add(nr_maps as usize) = map__get(new);
    map__set_kmap_maps(new, maps);
    if !maps_by_name.is_null() {
        *maps_by_name.add(nr_maps as usize) = map__get(new);
    }

    nr_maps += 1;
    (*RC_CHK_ACCESS(maps)).nr_maps = nr_maps;

    /*
     * Recompute if things are sorted. If things are inserted in a sorted
     * manner, for example by processing /proc/pid/maps, then no
     * sorting/resorting will be necessary.
     */
    if nr_maps == 1 {
        /* If there's just 1 entry then maps are sorted. */
        maps__set_maps_by_address_sorted(maps, true);
        maps__set_maps_by_name_sorted(maps, !maps_by_name.is_null());
    } else {
        /* Sorted if maps were already sorted and this map starts after the last one. */
        maps__set_maps_by_address_sorted(
            maps,
            maps__maps_by_address_sorted(maps) && map__end(*maps_by_address.add((nr_maps - 2) as usize)) <= map__start(new),
        );
        maps__set_maps_by_name_sorted(maps, false);
    }
    if map__end(new) < map__start(new) {
        (*RC_CHK_ACCESS(maps)).ends_broken = true;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__insert(maps: *mut maps, map: *mut map) -> c_int {
    let ret: c_int;

    down_write(maps__lock(maps));
    ret = __maps__insert(maps, map);
    check_invariants(maps);
    up_write(maps__lock(maps));
    ret
}

unsafe fn __maps__remove(maps: *mut maps, map: *mut map) {
    let maps_by_address = maps__maps_by_address(maps);
    let maps_by_name = maps__maps_by_name(maps);
    let nr_maps = maps__nr_maps(maps);
    let address_idx: c_uint;

    /* Slide later mappings over the one to remove */
    address_idx = maps__by_address_index(maps, map);
    map__put(*maps_by_address.add(address_idx as usize));
    memmove(
        maps_by_address.add(address_idx as usize) as *mut c_void,
        maps_by_address.add(address_idx as usize + 1) as *const c_void,
        (nr_maps - address_idx - 1) as usize * size_of::<*mut map>(),
    );

    if !maps_by_name.is_null() {
        let name_idx = maps__by_name_index(maps, map);

        map__put(*maps_by_name.add(name_idx as usize));
        memmove(
            maps_by_name.add(name_idx as usize) as *mut c_void,
            maps_by_name.add(name_idx as usize + 1) as *const c_void,
            (nr_maps - name_idx - 1) as usize * size_of::<*mut map>(),
        );
    }

    (*RC_CHK_ACCESS(maps)).nr_maps -= 1;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__remove(maps: *mut maps, map: *mut map) {
    down_write(maps__lock(maps));
    __maps__remove(maps, map);
    check_invariants(maps);
    up_write(maps__lock(maps));
    libdw__invalidate_dwfl(maps, maps__libdw_addr_space_dwfl(maps));
}

/**
 * maps__mutate_mapping - Apply write-protected mutations to a map.
 * @maps: The maps collection containing the map.
 * @map: The map to mutate.
 * @mutate_cb: Callback function that performs the actual mutations.
 * @data: Private data passed to the callback.
 *
 * This acquires the write lock on the maps semaphore to safely protect
 * concurrent readers from seeing partially mutated or unsorted map boundaries.
 *
 * WARNING: Acquiring down_write() here can trigger a recursive self-deadlock if
 * the caller already holds the read lock (e.g., during maps__for_each_map() or
 * maps__find() iteration paths that trigger lazy symbol loading). To completely
 * avoid this deadlock, all kernel/module maps must be pre-loaded up-front (via
 * maps__load_maps()) under a clean, single-threaded context before entering
 * multi-threaded event processing loops.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__mutate_mapping(
    maps: *mut maps,
    map: *mut map,
    mutate_cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> c_int {
    let mut err = 0;

    if !maps.is_null() {
        down_write(maps__lock(maps));

        err = mutate_cb.unwrap()(map, data);

        (*RC_CHK_ACCESS(maps)).maps_by_address_sorted = false;
        (*RC_CHK_ACCESS(maps)).maps_by_name_sorted = false;

        up_write(maps__lock(maps));

        libdw__invalidate_dwfl(maps, maps__libdw_addr_space_dwfl(maps));
    } else {
        err = mutate_cb.unwrap()(map, data);
    }

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__empty(maps: *mut maps) -> bool {
    let res: bool;

    down_read(maps__lock(maps));
    res = maps__nr_maps(maps) == 0;
    up_read(maps__lock(maps));

    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__equal(a: *mut maps, b: *mut maps) -> bool {
    RC_CHK_EQUAL(a, b)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__for_each_map(
    maps: *mut maps,
    cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> c_int {
    let mut done = false;
    let mut ret = 0;

    /* See locking/sorting note. */
    while !done {
        down_read(maps__lock(maps));
        if maps__maps_by_address_sorted(maps) {
            /*
             * maps__for_each_map callbacks may buggily/unsafely
             * insert into maps_by_address. Deliberately reload
             * maps__nr_maps and maps_by_address on each iteration
             * to avoid using memory freed by maps__insert growing
             * the array - this may cause maps to be skipped or
             * repeated.
             */
            let mut i = 0;
            while i < maps__nr_maps(maps) {
                let maps_by_address = maps__maps_by_address(maps);
                let map = *maps_by_address.add(i as usize);

                ret = cb.unwrap()(map, data);
                if ret != 0 {
                    break;
                }
                i += 1;
            }
            done = true;
        }
        up_read(maps__lock(maps));
        if !done {
            maps__sort_by_address(maps);
        }
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__load_maps(maps: *mut maps) -> c_int {
    let maps_copy: *mut *mut map;
    let nr_maps: c_uint;
    let mut err = 0;

    if maps.is_null() {
        return 0;
    }

    down_read(maps__lock(maps));
    nr_maps = maps__nr_maps(maps);
    if nr_maps == 0 {
        up_read(maps__lock(maps));
        return 0;
    }
    maps_copy = calloc(nr_maps as usize, size_of::<*mut map>()) as *mut *mut map;
    if maps_copy.is_null() {
        up_read(maps__lock(maps));
        return -ENOMEM;
    }
    for i in 0..nr_maps {
        *maps_copy.add(i as usize) = map__get(*maps__maps_by_address(maps).add(i as usize));
    }
    up_read(maps__lock(maps));

    for i in 0..nr_maps {
        if map__load(*maps_copy.add(i as usize)) < 0 {
            pr_warning(c"Failed to load map %s\n".as_ptr(), dso__name(map__dso(*maps_copy.add(i as usize))));
            err = -1;
        }
        map__put(*maps_copy.add(i as usize));
    }
    free(maps_copy as *mut c_void);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__remove_maps(
    maps: *mut maps,
    cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> bool>,
    data: *mut c_void,
) {
    let maps_by_address: *mut *mut map;
    let mut removed = false;

    down_write(maps__lock(maps));

    maps_by_address = maps__maps_by_address(maps);
    let mut i = 0;
    while i < maps__nr_maps(maps) {
        if cb.unwrap()(*maps_by_address.add(i as usize), data) {
            __maps__remove(maps, *maps_by_address.add(i as usize));
            removed = true;
        } else {
            i += 1;
        }
    }
    check_invariants(maps);
    up_write(maps__lock(maps));
    if removed {
        libdw__invalidate_dwfl(maps, maps__libdw_addr_space_dwfl(maps));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__find_symbol(maps: *mut maps, addr: u64, mapp: *mut *mut map) -> *mut symbol {
    let map = maps__find(maps, addr);
    let mut result: *mut symbol = ptr::null_mut();

    /* Ensure map is loaded before using map->map_ip */
    if !map.is_null() && map__load(map) >= 0 {
        result = map__find_symbol(map, map__map_ip(map, addr));
    }

    if !mapp.is_null() {
        *mapp = map;
    } else {
        map__put(map);
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__find_symbol_by_name(maps: *mut maps, name: *const c_char, mapp: *mut *mut map) -> *mut symbol {
    let maps_copy: *mut *mut map;
    let nr_maps: c_uint;
    let mut sym: *mut symbol = ptr::null_mut();

    if maps.is_null() {
        return ptr::null_mut();
    }

    /*
     * First, ensure all maps are loaded. We pre-load them outside of any
     * read-to-write locks to avoid deadlocks. Even if some fail, we proceed.
     */
    maps__load_maps(maps);

    /*
     * Create a local snapshot of the maps while holding the read lock.
     * This prevents deadlocking if iteration triggers further map insertions.
     */
    down_read(maps__lock(maps));
    nr_maps = maps__nr_maps(maps);
    maps_copy = calloc(nr_maps as usize, size_of::<*mut map>()) as *mut *mut map;
    if !maps_copy.is_null() {
        for i in 0..nr_maps {
            let map = *maps__maps_by_address(maps).add(i as usize);

            *maps_copy.add(i as usize) = map__get(map);
        }
    }
    up_read(maps__lock(maps));

    if maps_copy.is_null() {
        return ptr::null_mut();
    }

    for i in 0..nr_maps {
        let map = *maps_copy.add(i as usize);

        sym = map__find_symbol_by_name(map, name);
        if !sym.is_null() && map__contains_symbol(map, sym) {
            if !mapp.is_null() {
                *mapp = map__get(map);
            }
            break;
        }
        sym = ptr::null_mut();
    }

    for i in 0..nr_maps {
        map__put(*maps_copy.add(i as usize));
    }

    free(maps_copy as *mut c_void);
    sym
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__find_ams(maps: *mut maps, ams: *mut addr_map_symbol) -> c_int {
    if (*ams).addr < map__start((*ams).ms.map) || (*ams).addr >= map__end((*ams).ms.map) {
        if maps.is_null() {
            return -1;
        }
        map__put((*ams).ms.map);
        (*ams).ms.map = maps__find(maps, (*ams).addr);
        if (*ams).ms.map.is_null() {
            return -1;
        }
    }

    (*ams).al_addr = map__map_ip((*ams).ms.map, (*ams).addr);
    (*ams).ms.sym = map__find_symbol((*ams).ms.map, (*ams).al_addr);

    if !(*ams).ms.sym.is_null() { 0 } else { -1 }
}

#[repr(C)]
pub struct maps__fprintf_args {
    fp: *mut FILE,
    printed: size_t,
}

unsafe extern "C" fn maps__fprintf_cb(map: *mut map, data: *mut c_void) -> c_int {
    let args = data as *mut maps__fprintf_args;

    (*args).printed += fprintf((*args).fp, c"Map:".as_ptr()) as size_t;
    (*args).printed += map__fprintf(map, (*args).fp);
    if verbose > 2 {
        (*args).printed += dso__fprintf(map__dso(map), (*args).fp);
        (*args).printed += fprintf((*args).fp, c"--\n".as_ptr()) as size_t;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__fprintf(maps: *mut maps, fp: *mut FILE) -> size_t {
    let mut args = maps__fprintf_args {
        fp,
        printed: 0,
    };

    maps__for_each_map(maps, Some(maps__fprintf_cb), &mut args as *mut _ as *mut c_void);

    args.printed
}

/*
 * Find first map where end > map->start.
 * Same as find_vma() in kernel.
 */
unsafe fn first_ending_after(maps: *mut maps, map: *const map) -> c_uint {
    let maps_by_address = maps__maps_by_address(maps);
    let mut low: c_int = 0;
    let mut high: c_int = maps__nr_maps(maps) as c_int - 1;
    let mut first: c_int = high + 1;

    debug_assert!(maps__maps_by_address_sorted(maps));
    if low <= high && map__end(*maps_by_address.add(0)) > map__start(map) {
        return 0;
    }

    while low <= high {
        let mid = (low + high) / 2;
        let pos = *maps_by_address.add(mid as usize);

        if map__end(pos) > map__start(map) {
            first = mid;
            if map__start(pos) <= map__start(map) {
                /* Entry overlaps map. */
                break;
            }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    first as c_uint
}

unsafe fn __maps__insert_sorted(maps: *mut maps, first_after_index: c_uint, new1: *mut map, new2: *mut map) -> c_int {
    let mut maps_by_address = maps__maps_by_address(maps);
    let mut maps_by_name = maps__maps_by_name(maps);
    let nr_maps = maps__nr_maps(maps);
    let mut nr_allocate = (*RC_CHK_ACCESS(maps)).nr_maps_allocated;
    let to_add = if !new2.is_null() { 2 } else { 1 };

    debug_assert!(maps__maps_by_address_sorted(maps));
    debug_assert!(first_after_index == nr_maps || map__end(new1) <= map__start(*maps_by_address.add(first_after_index as usize)));
    debug_assert!(new2.is_null() || map__end(new1) <= map__start(new2));
    debug_assert!(first_after_index == nr_maps || new2.is_null() || map__end(new2) <= map__start(*maps_by_address.add(first_after_index as usize)));

    if nr_maps + to_add > nr_allocate {
        nr_allocate = if nr_allocate == 0 { 32 } else { nr_allocate * 2 };

        maps_by_address = realloc(maps_by_address as *mut c_void, nr_allocate as usize * size_of::<*mut map>()) as *mut *mut map;
        if maps_by_address.is_null() {
            return -ENOMEM;
        }

        maps__set_maps_by_address(maps, maps_by_address);
        if !maps_by_name.is_null() {
            maps_by_name = realloc(maps_by_name as *mut c_void, nr_allocate as usize * size_of::<*mut map>()) as *mut *mut map;
            if maps_by_name.is_null() {
                /*
                 * If by name fails, just disable by name and it will
                 * recompute next time it is required.
                 */
                __maps__free_maps_by_name(maps);
            }
            maps__set_maps_by_name(maps, maps_by_name);
        }
        (*RC_CHK_ACCESS(maps)).nr_maps_allocated = nr_allocate;
    }
    memmove(
        maps_by_address.add((first_after_index + to_add) as usize) as *mut c_void,
        maps_by_address.add(first_after_index as usize) as *const c_void,
        (nr_maps - first_after_index) as usize * size_of::<*mut map>(),
    );
    *maps_by_address.add(first_after_index as usize) = map__get(new1);
    if !maps_by_name.is_null() {
        *maps_by_name.add(nr_maps as usize) = map__get(new1);
    }
    if !new2.is_null() {
        *maps_by_address.add(first_after_index as usize + 1) = map__get(new2);
        if !maps_by_name.is_null() {
            *maps_by_name.add(nr_maps as usize + 1) = map__get(new2);
        }
    }
    (*RC_CHK_ACCESS(maps)).nr_maps = nr_maps + to_add;
    maps__set_maps_by_name_sorted(maps, false);
    map__set_kmap_maps(new1, maps);
    map__set_kmap_maps(new2, maps);

    check_invariants(maps);
    0
}

/*
 * Adds new to maps, if new overlaps existing entries then the existing maps are
 * adjusted or removed so that new fits without overlapping any entries.
 */
unsafe fn __maps__fixup_overlap_and_insert(maps: *mut maps, new: *mut map) -> c_int {
    let mut err = 0;
    let mut ni: c_uint = INT_MAX; // Some gcc complain, but depends on maps_by_name...

    if !maps__maps_by_address_sorted(maps) {
        __maps__sort_by_address(maps);
    }

    /*
     * Iterate through entries where the end of the existing entry is
     * greater-than the new map's start.
     */
    let mut i = first_ending_after(maps, new);
    while i < maps__nr_maps(maps) {
        let maps_by_address = maps__maps_by_address(maps);
        let maps_by_name = maps__maps_by_name(maps);
        let pos = *maps_by_address.add(i as usize);
        let mut before: *mut map = ptr::null_mut();
        let mut after: *mut map = ptr::null_mut();

        /*
         * Stop if current map starts after map->end.
         * Maps are ordered by start: next will not overlap for sure.
         */
        if map__start(pos) >= map__end(new) {
            break;
        }

        if use_browser {
            pr_debug(c"overlapping maps in %s (disable tui for more info)\n".as_ptr(), dso__name(map__dso(new)));
        } else if verbose >= 2 {
            pr_debug(c"overlapping maps:\n".as_ptr());
            map__fprintf(new, debug_file());
            map__fprintf(pos, debug_file());
        }

        if !maps_by_name.is_null() {
            ni = maps__by_name_index(maps, pos);
        }

        /*
         * Now check if we need to create new maps for areas not
         * overlapped by the new map:
         */
        if map__start(new) > map__start(pos) {
            /* Map starts within existing map. Need to shorten the existing map. */
            before = map__clone(pos);

            if before.is_null() {
                err = -ENOMEM;
                return err;
            }
            map__set_end(before, map__start(new));

            if verbose >= 2 && !use_browser {
                map__fprintf(before, debug_file());
            }
        }
        if map__end(new) < map__end(pos) {
            /* The new map isn't as long as the existing map. */
            after = map__clone(pos);

            if after.is_null() {
                map__zput(before);
                err = -ENOMEM;
                return err;
            }

            map__set_start(after, map__end(new));
            map__add_pgoff(after, map__end(new) - map__start(pos));
            debug_assert!(map__map_ip(pos, map__end(new)) == map__map_ip(after, map__end(new)));

            if verbose >= 2 && !use_browser {
                map__fprintf(after, debug_file());
            }
        }
        /*
         * If adding one entry, for `before` or `after`, we can replace
         * the existing entry. If both `before` and `after` are
         * necessary than an insert is needed. If the existing entry
         * entirely overlaps the existing entry it can just be removed.
         */
        if !before.is_null() {
            map__put(*maps_by_address.add(i as usize));
            *maps_by_address.add(i as usize) = before;
            map__set_kmap_maps(before, maps);

            if !maps_by_name.is_null() {
                map__put(*maps_by_name.add(ni as usize));
                *maps_by_name.add(ni as usize) = map__get(before);
            }

            /* Maps are still ordered, go to next one. */
            i += 1;
            if !after.is_null() {
                /*
                 * 'before' and 'after' mean 'new' split the
                 * 'pos' mapping and therefore there are no
                 * later mappings.
                 */
                err = __maps__insert_sorted(maps, i, new, after);
                map__put(after);
                check_invariants(maps);
                return err;
            }
            check_invariants(maps);
        } else if !after.is_null() {
            /*
             * 'after' means 'new' split 'pos' and there are no
             * later mappings.
             */
            map__put(*maps_by_address.add(i as usize));
            *maps_by_address.add(i as usize) = map__get(new);
            map__set_kmap_maps(new, maps);

            if !maps_by_name.is_null() {
                map__put(*maps_by_name.add(ni as usize));
                *maps_by_name.add(ni as usize) = map__get(new);
                maps__set_maps_by_name_sorted(maps, false);
            }

            err = __maps__insert_sorted(maps, i + 1, after, ptr::null_mut());
            map__put(after);
            check_invariants(maps);
            return err;
        } else {
            let mut next: *mut map = ptr::null_mut();
            let nr_maps = maps__nr_maps(maps);

            if i + 1 < nr_maps {
                next = *maps_by_address.add(i as usize + 1);
            }

            if next.is_null() || map__start(next) >= map__end(new) {
                /*
                 * Replace existing mapping and end knowing
                 * there aren't later overlapping or any
                 * mappings.
                 */
                map__put(*maps_by_address.add(i as usize));
                *maps_by_address.add(i as usize) = map__get(new);
                map__set_kmap_maps(new, maps);

                if !maps_by_name.is_null() {
                    map__put(*maps_by_name.add(ni as usize));
                    *maps_by_name.add(ni as usize) = map__get(new);
                    maps__set_maps_by_name_sorted(maps, false);
                }

                check_invariants(maps);
                return err;
            }
            /*
             * pos fully covers the previous mapping so remove
             * it. The following is an inlined version of
             * maps__remove that reuses the already computed
             * indices.
             */
            map__put(*maps_by_address.add(i as usize));
            memmove(
                maps_by_address.add(i as usize) as *mut c_void,
                maps_by_address.add(i as usize + 1) as *const c_void,
                (nr_maps - i - 1) as usize * size_of::<*mut map>(),
            );

            if !maps_by_name.is_null() {
                map__put(*maps_by_name.add(ni as usize));
                memmove(
                    maps_by_name.add(ni as usize) as *mut c_void,
                    maps_by_name.add(ni as usize + 1) as *const c_void,
                    (nr_maps - ni - 1) as usize * size_of::<*mut map>(),
                );
            }
            (*RC_CHK_ACCESS(maps)).nr_maps -= 1;
            check_invariants(maps);
            /*
             * Maps are ordered but no need to increase `i` as the
             * later maps were moved down.
             */
        }
    }
    /* Add the map. */
    err = __maps__insert_sorted(maps, i, new, ptr::null_mut());
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__fixup_overlap_and_insert(maps: *mut maps, new: *mut map) -> c_int {
    let err: c_int;

    down_write(maps__lock(maps));
    err = __maps__fixup_overlap_and_insert(maps, new);
    up_write(maps__lock(maps));
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__copy_from(dest: *mut maps, parent: *mut maps) -> c_int {
    /* Note, if struct map were immutable then cloning could use ref counts. */
    let parent_maps_by_address: *mut *mut map;
    let mut err = 0;
    let n: c_uint;

    down_write(maps__lock(dest));
    down_read(maps__lock(parent));

    err = unwind__prepare_access(dest, maps__e_machine(parent));
    if err != 0 {
        up_read(maps__lock(parent));
        up_write(maps__lock(dest));
        return err;
    }

    parent_maps_by_address = maps__maps_by_address(parent);
    n = maps__nr_maps(parent);
    if maps__nr_maps(dest) == 0 {
        /* No existing mappings so just copy from parent to avoid reallocs in insert. */
        let nr_maps_allocated = (*RC_CHK_ACCESS(parent)).nr_maps_allocated;
        let dest_maps_by_address = malloc(nr_maps_allocated as usize * size_of::<*mut map>()) as *mut *mut map;
        let mut dest_maps_by_name: *mut *mut map = ptr::null_mut();

        if dest_maps_by_address.is_null() {
            err = -ENOMEM;
        } else {
            if !maps__maps_by_name(parent).is_null() {
                dest_maps_by_name = malloc(nr_maps_allocated as usize * size_of::<*mut map>()) as *mut *mut map;
            }

            (*RC_CHK_ACCESS(dest)).maps_by_address = dest_maps_by_address;
            (*RC_CHK_ACCESS(dest)).maps_by_name = dest_maps_by_name;
            (*RC_CHK_ACCESS(dest)).nr_maps_allocated = nr_maps_allocated;
        }

        let mut i = 0;
        while err == 0 && i < n {
            let pos = *parent_maps_by_address.add(i as usize);
            let new = map__clone(pos);

            if new.is_null() {
                err = -ENOMEM;
            } else {
                *dest_maps_by_address.add(i as usize) = new;
                map__set_kmap_maps(new, dest);
                if !dest_maps_by_name.is_null() {
                    *dest_maps_by_name.add(i as usize) = map__get(new);
                }
                (*RC_CHK_ACCESS(dest)).nr_maps = i + 1;
            }
            if err != 0 {
                map__put(new);
            }
            i += 1;
        }
        maps__set_maps_by_address_sorted(dest, maps__maps_by_address_sorted(parent));
        (*RC_CHK_ACCESS(dest)).last_search_by_name_idx = 0;
        /* Values were copied into the name array in address order. */
        maps__set_maps_by_name_sorted(dest, false);
    } else {
        /* Unexpected copying to a maps containing entries. */
        let mut i = 0;
        while err == 0 && i < n {
            let pos = *parent_maps_by_address.add(i as usize);
            let new = map__clone(pos);

            if new.is_null() {
                err = -ENOMEM;
            } else {
                err = __maps__insert(dest, new);
            }
            map__put(new);
            i += 1;
        }
    }
    check_invariants(dest);

    up_read(maps__lock(parent));
    up_write(maps__lock(dest));
    err
}

unsafe extern "C" fn map__addr_cmp(key: *const c_void, entry: *const c_void) -> c_int {
    let ip = *(key as *const u64);
    let map = *(entry as *const *const map);

    if ip < map__start(map) {
        return -1;
    }
    if ip >= map__end(map) {
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__find(maps: *mut maps, ip: u64) -> *mut map {
    let mut result: *mut map = ptr::null_mut();
    let mut done = false;

    /* See locking/sorting note. */
    while !done {
        down_read(maps__lock(maps));
        if maps__maps_by_address_sorted(maps) {
            let mut mapp: *mut *mut map = ptr::null_mut();
            let maps_by_address = maps__maps_by_address(maps);
            let nr_maps = maps__nr_maps(maps);

            if !maps_by_address.is_null() && nr_maps != 0 {
                mapp = bsearch(
                    &ip as *const _ as *const c_void,
                    maps_by_address as *const c_void,
                    nr_maps as size_t,
                    size_of::<*mut map>(),
                    map__addr_cmp,
                ) as *mut *mut map;
            }
            if !mapp.is_null() {
                result = map__get(*mapp);
            }
            done = true;
        }
        up_read(maps__lock(maps));
        if !done {
            maps__sort_by_address(maps);
        }
    }
    result
}

unsafe extern "C" fn map__strcmp_name(name: *const c_void, b: *const c_void) -> c_int {
    let dso = map__dso(*(b as *const *mut map));

    strcmp(name as *const c_char, dso__short_name(dso))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__find_by_name(maps: *mut maps, name: *const c_char) -> *mut map {
    let mut result: *mut map = ptr::null_mut();
    let mut done = false;

    /* See locking/sorting note. */
    while !done {
        let mut i: c_uint;

        down_read(maps__lock(maps));

        /* First check last found entry. */
        i = (*RC_CHK_ACCESS(maps)).last_search_by_name_idx;
        if i < maps__nr_maps(maps) && !maps__maps_by_name(maps).is_null() {
            let dso = map__dso(*maps__maps_by_name(maps).add(i as usize));

            if !dso.is_null() && strcmp(dso__short_name(dso), name) == 0 {
                result = map__get(*maps__maps_by_name(maps).add(i as usize));
                done = true;
            }
        }

        /* Second search sorted array. */
        if !done && maps__maps_by_name_sorted(maps) {
            let mapp = bsearch(
                name as *const c_void,
                maps__maps_by_name(maps) as *const c_void,
                maps__nr_maps(maps) as size_t,
                size_of::<*mut map>(),
                map__strcmp_name,
            ) as *mut *mut map;

            if !mapp.is_null() {
                result = map__get(*mapp);
                i = mapp.offset_from(maps__maps_by_name(maps)) as c_uint;
                (*RC_CHK_ACCESS(maps)).last_search_by_name_idx = i;
            }
            done = true;
        }
        up_read(maps__lock(maps));
        if !done {
            /* Sort and retry binary search. */
            if maps__sort_by_name(maps) != 0 {
                /*
                 * Memory allocation failed do linear search
                 * through address sorted maps.
                 */
                let maps_by_address: *mut *mut map;
                let n: c_uint;

                down_read(maps__lock(maps));
                maps_by_address = maps__maps_by_address(maps);
                n = maps__nr_maps(maps);
                i = 0;
                while i < n {
                    let pos = *maps_by_address.add(i as usize);
                    let dso = map__dso(pos);

                    if !dso.is_null() && strcmp(dso__short_name(dso), name) == 0 {
                        result = map__get(pos);
                        break;
                    }
                    i += 1;
                }
                up_read(maps__lock(maps));
                done = true;
            }
        }
    }
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__find_next_entry(maps: *mut maps, map: *mut map) -> *mut map {
    let mut i: c_uint;
    let mut result: *mut map = ptr::null_mut();

    down_read(maps__lock(maps));
    while !maps__maps_by_address_sorted(maps) {
        up_read(maps__lock(maps));
        maps__sort_by_address(maps);
        down_read(maps__lock(maps));
    }
    i = maps__by_address_index(maps, map);
    i += 1;
    if i < maps__nr_maps(maps) {
        result = map__get(*maps__maps_by_address(maps).add(i as usize));
    }

    up_read(maps__lock(maps));
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__fixup_end(maps: *mut maps) {
    let maps_by_address: *mut *mut map;
    let n: c_uint;

    down_write(maps__lock(maps));
    if !maps__maps_by_address_sorted(maps) {
        __maps__sort_by_address(maps);
    }

    maps_by_address = maps__maps_by_address(maps);
    n = maps__nr_maps(maps);
    for i in 1..n {
        let prev = *maps_by_address.add((i - 1) as usize);
        let curr = *maps_by_address.add(i as usize);

        if map__end(prev) == 0 || map__end(prev) > map__start(curr) {
            map__set_end(prev, map__start(curr));
        }
    }

    /*
     * We still haven't the actual symbols, so guess the
     * last map final address.
     */
    if n > 0 && map__end(*maps_by_address.add((n - 1) as usize)) == 0 {
        map__set_end(*maps_by_address.add((n - 1) as usize), !0u64);
    }

    (*RC_CHK_ACCESS(maps)).ends_broken = false;
    check_invariants(maps);

    up_write(maps__lock(maps));
}

/*
 * Merges map into maps by splitting the new map within the existing map
 * regions.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__merge_in(kmaps: *mut maps, new_map: *mut map) -> c_int {
    let mut first_after_: c_uint;
    let kmaps__nr_maps: c_uint;
    let mut kmaps_maps_by_address: *mut *mut map;
    let merged_maps_by_address: *mut *mut map;
    let mut merged_nr_maps_allocated: c_uint;

    /* First try under a read lock. */
    loop {
        down_read(maps__lock(kmaps));
        if maps__maps_by_address_sorted(kmaps) {
            break;
        }

        up_read(maps__lock(kmaps));

        /* First after binary search requires sorted maps. Sort and try again. */
        maps__sort_by_address(kmaps);
    }
    first_after_ = first_ending_after(kmaps, new_map);
    kmaps_maps_by_address = maps__maps_by_address(kmaps);

    if first_after_ >= maps__nr_maps(kmaps) || map__start(*kmaps_maps_by_address.add(first_after_ as usize)) >= map__end(new_map) {
        /* No overlap so regular insert suffices. */
        up_read(maps__lock(kmaps));
        return maps__insert(kmaps, new_map);
    }
    up_read(maps__lock(kmaps));

    /* Plain insert with a read-lock failed, try again now with the write lock. */
    down_write(maps__lock(kmaps));
    if !maps__maps_by_address_sorted(kmaps) {
        __maps__sort_by_address(kmaps);
    }

    first_after_ = first_ending_after(kmaps, new_map);
    kmaps_maps_by_address = maps__maps_by_address(kmaps);
    kmaps__nr_maps = maps__nr_maps(kmaps);

    if first_after_ >= kmaps__nr_maps || map__start(*kmaps_maps_by_address.add(first_after_ as usize)) >= map__end(new_map) {
        /* No overlap so regular insert suffices. */
        let ret = __maps__insert(kmaps, new_map);

        check_invariants(kmaps);
        up_write(maps__lock(kmaps));
        return ret;
    }
    /* Array to merge into, possibly 1 more for the sake of new_map. */
    merged_nr_maps_allocated = (*RC_CHK_ACCESS(kmaps)).nr_maps_allocated;
    if kmaps__nr_maps + 1 == merged_nr_maps_allocated {
        merged_nr_maps_allocated += 1;
    }

    merged_maps_by_address = malloc(merged_nr_maps_allocated as usize * size_of::<*mut map>()) as *mut *mut map;
    if merged_maps_by_address.is_null() {
        up_write(maps__lock(kmaps));
        return -ENOMEM;
    }
    maps__set_maps_by_address(kmaps, merged_maps_by_address);
    maps__set_maps_by_address_sorted(kmaps, true);
    __maps__free_maps_by_name(kmaps);
    maps__set_nr_maps_allocated(kmaps, merged_nr_maps_allocated);

    /* Copy entries before the new_map that can't overlap. */
    for i in 0..first_after_ {
        *merged_maps_by_address.add(i as usize) = map__get(*kmaps_maps_by_address.add(i as usize));
    }

    maps__set_nr_maps(kmaps, first_after_);

    /* Add the new map, it will be split when the later overlapping mappings are added. */
    __maps__insert(kmaps, new_map);

    /* Insert mappings after new_map, splitting new_map in the process. */
    for i in first_after_..kmaps__nr_maps {
        __maps__fixup_overlap_and_insert(kmaps, *kmaps_maps_by_address.add(i as usize));
    }

    /* Copy the maps from merged into kmaps. */
    for i in 0..kmaps__nr_maps {
        map__zput(*kmaps_maps_by_address.add(i as usize));
    }

    free(kmaps_maps_by_address as *mut c_void);
    check_invariants(kmaps);
    up_write(maps__lock(kmaps));
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn maps__load_first(maps: *mut maps) {
    down_read(maps__lock(maps));

    if maps__nr_maps(maps) > 0 {
        map__load(*maps__maps_by_address(maps).add(0));
    }

    up_read(maps__lock(maps));
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
