/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/maps.h. */

pub type size_t = usize;
pub type u64 = u64;
pub type uint16_t = u16;
pub type c_int = i32;
pub type c_char = i8;
pub type c_void = core::ffi::c_void;

/* External C types supplied by included headers or other perf headers. */
pub enum ref_reloc_sym {}
pub enum machine {}
pub enum map {}
pub enum maps {}
pub enum symbol {}
pub enum addr_map_symbol {}
pub enum FILE {}
pub enum refcount_t {}

pub const KMAP_NAME_LEN: usize = 256;

#[repr(C)]
pub struct kmap {
    pub ref_reloc_sym: *mut ref_reloc_sym,
    pub kmaps: *mut maps,
    pub name: [c_char; KMAP_NAME_LEN],
}

extern "C" {
    pub fn maps__new(machine: *mut machine) -> *mut maps;
    pub fn maps__empty(maps: *mut maps) -> bool;
    pub fn maps__copy_from(maps: *mut maps, parent: *mut maps) -> c_int;

    pub fn maps__get(maps: *mut maps) -> *mut maps;
    pub fn maps__put(maps: *mut maps);
}

#[inline]
pub unsafe fn __maps__zput(map: *mut *mut maps) {
    maps__put(*map);
    *map = core::ptr::null_mut();
}

/* C macro: #define maps__zput(map) __maps__zput(&map) */
#[inline]
pub unsafe fn maps__zput(map: *mut *mut maps) {
    __maps__zput(map);
}

extern "C" {
    pub fn maps__equal(a: *mut maps, b: *mut maps) -> bool;

    /* Iterate over map calling cb for each entry. */
    pub fn maps__for_each_map(
        maps: *mut maps,
        cb: Option<unsafe extern "C" fn(map: *mut map, data: *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;

    /* Iterate over map removing an entry if cb returns true. */
    pub fn maps__remove_maps(
        maps: *mut maps,
        cb: Option<unsafe extern "C" fn(map: *mut map, data: *mut c_void) -> bool>,
        data: *mut c_void,
    );

    pub fn maps__machine(maps: *const maps) -> *mut machine;
    pub fn maps__nr_maps(maps: *const maps) -> ::core::primitive::u32; /* Test only. */
    pub fn maps__refcnt(maps: *mut maps) -> *mut refcount_t; /* Test only. */
}

/*
 * Conditional C declarations under HAVE_LIBUNWIND_SUPPORT:
 * void *maps__addr_space(const struct maps *maps);
 * void maps__set_addr_space(struct maps *maps, void *addr_space);
 * uint16_t maps__e_machine(const struct maps *maps);
 * void maps__set_e_machine(struct maps *maps, uint16_t e_machine);
 */
extern "C" {
    pub fn maps__addr_space(maps: *const maps) -> *mut c_void;
    pub fn maps__set_addr_space(maps: *mut maps, addr_space: *mut c_void);
    pub fn maps__e_machine(maps: *const maps) -> uint16_t;
    pub fn maps__set_e_machine(maps: *mut maps, e_machine: uint16_t);
}

/*
 * Conditional C declarations under HAVE_LIBDW_SUPPORT:
 * void *maps__libdw_addr_space_dwfl(const struct maps *maps);
 * void maps__set_libdw_addr_space_dwfl(struct maps *maps, void *dwfl);
 */
extern "C" {
    pub fn maps__libdw_addr_space_dwfl(maps: *const maps) -> *mut c_void;
    pub fn maps__set_libdw_addr_space_dwfl(maps: *mut maps, dwfl: *mut c_void);
}

extern "C" {
    pub fn maps__fprintf(maps: *mut maps, fp: *mut FILE) -> size_t;

    pub fn maps__load_maps(maps: *mut maps) -> c_int;
    pub fn maps__insert(maps: *mut maps, map: *mut map) -> c_int;
    pub fn maps__remove(maps: *mut maps, map: *mut map);
    pub fn maps__mutate_mapping(
        maps: *mut maps,
        map: *mut map,
        mutate_cb: Option<unsafe extern "C" fn(map: *mut map, data: *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;

    pub fn maps__find(maps: *mut maps, addr: u64) -> *mut map;
    pub fn maps__find_symbol(maps: *mut maps, addr: u64, mapp: *mut *mut map) -> *mut symbol;
    pub fn maps__find_symbol_by_name(
        maps: *mut maps,
        name: *const c_char,
        mapp: *mut *mut map,
    ) -> *mut symbol;

    pub fn maps__find_ams(maps: *mut maps, ams: *mut addr_map_symbol) -> c_int;

    pub fn maps__fixup_overlap_and_insert(maps: *mut maps, new: *mut map) -> c_int;

    pub fn maps__find_by_name(maps: *mut maps, name: *const c_char) -> *mut map;

    pub fn maps__find_next_entry(maps: *mut maps, map: *mut map) -> *mut map;

    pub fn maps__merge_in(kmaps: *mut maps, new_map: *mut map) -> c_int;

    pub fn maps__fixup_end(maps: *mut maps);

    pub fn maps__load_first(maps: *mut maps);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
