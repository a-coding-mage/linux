// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Google LLC
 */

// Dependency declarations and the HASHTABLE_DEFINE/list macros are supplied by
// the surrounding translation unit.

const DIE_HASH_BITS: usize = 16;

/* {die->addr, state} -> struct die * */
// static HASHTABLE_DEFINE(die_map, 1 << DIE_HASH_BITS);
static mut DIE_MAP: *mut core::ffi::c_void = core::ptr::null_mut();

static mut MAP_HITS: ::core::ffi::c_uint = 0;
static mut MAP_MISSES: ::core::ffi::c_uint = 0;

#[inline]
unsafe fn die_hash(addr: usize, state: die_state) -> ::core::ffi::c_uint {
    hash_32(addr_hash(addr) ^ (state as ::core::ffi::c_uint))
}

unsafe fn init_die(cd: *mut die) {
    (*cd).state = DIE_INCOMPLETE;
    (*cd).mapped = false;
    (*cd).fqn = core::ptr::null_mut();
    (*cd).tag = -1;
    (*cd).addr = 0;
    INIT_LIST_HEAD(&mut (*cd).fragments);
}

unsafe fn create_die(die: *mut Dwarf_Die, state: die_state) -> *mut die {
    let cd: *mut die = xmalloc(core::mem::size_of::<die>()) as *mut die;
    init_die(cd);
    (*cd).addr = (*die).addr as usize;

    hash_add(
        DIE_MAP,
        &mut (*cd).hash,
        die_hash((*cd).addr, state),
    );
    cd
}

pub unsafe fn __die_map_get(
    addr: usize,
    state: die_state,
    res: *mut *mut die,
) -> ::core::ffi::c_int {
    let mut cd: *mut die;

    // hash_for_each_possible(die_map, cd, hash, die_hash(addr, state)) {
    for cd in hash_for_each_possible(DIE_MAP, die_hash(addr, state)) {
        if (*cd).addr == addr && (*cd).state == state {
            *res = cd;
            return 0;
        }
    }

    -1
}

pub unsafe fn die_map_get(die: *mut Dwarf_Die, state: die_state) -> *mut die {
    let mut cd: *mut die = core::ptr::null_mut();

    if __die_map_get((*die).addr as usize, state, &mut cd) == 0 {
        MAP_HITS = MAP_HITS.wrapping_add(1);
        return cd;
    }

    MAP_MISSES = MAP_MISSES.wrapping_add(1);
    create_die(die, state)
}

unsafe fn reset_die(cd: *mut die) {
    let mut tmp: *mut die_fragment;
    let mut df: *mut die_fragment;

    // list_for_each_entry_safe(df, tmp, &cd->fragments, list) {
    for (df, tmp) in list_for_each_entry_safe(&mut (*cd).fragments) {
        if (*df).type_ == FRAGMENT_STRING {
            free((*df).data.str_);
        }
        free(df as *mut core::ffi::c_void);
        let _ = tmp;
    }

    if !(*cd).fqn.is_null() && *(*cd).fqn != 0 {
        free((*cd).fqn as *mut core::ffi::c_void);
    }
    init_die(cd);
}

pub unsafe fn die_map_for_each(func: die_map_callback_t, arg: *mut core::ffi::c_void) {
    let mut tmp: *mut hlist_node;
    let mut cd: *mut die;

    // hash_for_each_safe(die_map, cd, tmp, hash) {
    for (cd, tmp) in hash_for_each_safe(DIE_MAP) {
        func(cd, arg);
        let _ = tmp;
    }
}

pub unsafe fn die_map_free() {
    let mut tmp: *mut hlist_node;
    let mut stats: [::core::ffi::c_uint; DIE_LAST as usize + 1] = [0; DIE_LAST as usize + 1];
    let mut cd: *mut die;
    let mut i: ::core::ffi::c_int;

    core::ptr::write_bytes(stats.as_mut_ptr(), 0, stats.len());

    // hash_for_each_safe(die_map, cd, tmp, hash) {
    for (cd, tmp) in hash_for_each_safe(DIE_MAP) {
        stats[(*cd).state as usize] = stats[(*cd).state as usize].wrapping_add(1);
        reset_die(cd);
        free(cd as *mut core::ffi::c_void);
        let _ = tmp;
    }
    hash_init(DIE_MAP);

    if MAP_HITS.wrapping_add(MAP_MISSES) > 0 {
        debug(
            b"hits %u, misses %u (hit rate %.02f%%)\0".as_ptr() as *const i8,
            MAP_HITS,
            MAP_MISSES,
            (100.0f32 * MAP_HITS as f32) / MAP_HITS.wrapping_add(MAP_MISSES) as f32,
        );
    }

    i = 0;
    while i <= DIE_LAST as ::core::ffi::c_int {
        debug(
            b"%s: %u entries\0".as_ptr() as *const i8,
            die_state_name(i),
            stats[i as usize],
        );
        i += 1;
    }
}

unsafe fn append_item(cd: *mut die) -> *mut die_fragment {
    let df: *mut die_fragment = xmalloc(core::mem::size_of::<die_fragment>()) as *mut die_fragment;
    (*df).type_ = FRAGMENT_EMPTY;
    list_add_tail(&mut (*df).list, &mut (*cd).fragments);
    df
}

pub unsafe fn die_map_add_string(cd: *mut die, str_: *const i8) {
    if cd.is_null() {
        return;
    }
    let df = append_item(cd);
    (*df).data.str_ = xstrdup(str_);
    (*df).type_ = FRAGMENT_STRING;
}

pub unsafe fn die_map_add_linebreak(cd: *mut die, linebreak: ::core::ffi::c_int) {
    if cd.is_null() {
        return;
    }
    let df = append_item(cd);
    (*df).data.linebreak = linebreak;
    (*df).type_ = FRAGMENT_LINEBREAK;
}

pub unsafe fn die_map_add_die(cd: *mut die, child: *mut die) {
    if cd.is_null() {
        return;
    }
    let df = append_item(cd);
    (*df).data.addr = (*child).addr;
    (*df).type_ = FRAGMENT_DIE;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
