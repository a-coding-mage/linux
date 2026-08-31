// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/map.h. C include/header-guard dependencies are
// intentionally left as external Rust items.

use core::ffi::{c_char, c_int, c_void};

pub type u64 = u64;
pub type u32 = u32;
pub type size_t = usize;
pub type FILE = c_void;
pub type refcount_t = c_void;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_id {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mapping_type {
    /* map__map_ip/map__unmap_ip are given as offsets in the DSO. */
    MAPPING_TYPE__DSO = 0,
    /* map__map_ip/map__unmap_ip are just the given ip value. */
    MAPPING_TYPE__IDENTITY = 1,
}

// DECLARE_RC_STRUCT(map) { ... }
#[repr(C)]
pub struct map {
    pub start: u64,
    pub end: u64,
    pub pgoff: u64,
    pub reloc: u64,
    pub dso: *mut dso,
    pub refcnt: refcount_t,
    pub prot: u32,
    pub flags: u32,
    /* C declares mapping_type as an 8-bit bit-field. */
    pub mapping_type: mapping_type,
    pub erange_warned: bool,
    pub priv_: bool,
    pub hit: bool,
}

unsafe extern "C" {
    pub fn __map__kmap(map: *mut map) -> *mut kmap;
    pub fn map__kmap(map: *mut map) -> *mut kmap;
    pub fn map__kmaps(map: *mut map) -> *mut maps;

    /* rip/ip <-> addr suitable for passing to `objdump --start-address=` */
    pub fn map__rip_2objdump(map: *const map, rip: u64) -> u64;

    /* objdump address -> memory address */
    pub fn map__objdump_2mem(map: *const map, ip: u64) -> u64;

    /* objdump address -> rip */
    pub fn map__objdump_2rip(map: *const map, ip: u64) -> u64;

    pub fn map__new(
        machine: *mut machine,
        start: u64,
        len: u64,
        pgoff: u64,
        id: *const dso_id,
        prot: u32,
        flags: u32,
        filename: *mut c_char,
        thread: *mut thread,
    ) -> *mut map;
    pub fn map__new2(start: u64, dso: *mut dso) -> *mut map;
    pub fn map__delete(map: *mut map);
    pub fn map__clone(map: *mut map) -> *mut map;
    pub fn map__put(map: *mut map);

    pub fn map__fprintf(map: *mut map, fp: *mut FILE) -> size_t;
    pub fn map__fprintf_dsoname(map: *mut map, fp: *mut FILE) -> size_t;
    pub fn map__fprintf_dsoname_dsoff(
        map: *mut map,
        print_off: bool,
        addr: u64,
        fp: *mut FILE,
    ) -> size_t;
    pub fn map__srcline(map: *mut map, addr: u64, sym: *mut symbol) -> *mut c_char;
    pub fn map__fprintf_srcline(
        map: *mut map,
        addr: u64,
        prefix: *const c_char,
        fp: *mut FILE,
    ) -> c_int;

    pub fn map__load(map: *mut map) -> c_int;
    pub fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    pub fn map__find_symbol_by_name(map: *mut map, name: *const c_char) -> *mut symbol;
    pub fn map__find_symbol_by_name_idx(
        map: *mut map,
        name: *const c_char,
        idx: *mut size_t,
    ) -> *mut symbol;
    pub fn map__fixup_start(map: *mut map);
    pub fn map__fixup_end(map: *mut map);

    pub fn map__set_kallsyms_ref_reloc_sym(
        map: *mut map,
        symbol_name: *const c_char,
        addr: u64,
    ) -> c_int;

    pub fn __map__is_kernel(map: *const map) -> bool;
    pub fn __map__is_extra_kernel_map(map: *const map) -> bool;
    pub fn __map__is_bpf_prog(map: *const map) -> bool;
    pub fn __map__is_bpf_image(map: *const map) -> bool;
    pub fn __map__is_ool(map: *const map) -> bool;

    pub fn map__has_symbols(map: *const map) -> bool;
    pub fn map__contains_symbol(map: *const map, sym: *const symbol) -> bool;

    pub fn refcount_inc(r: *mut refcount_t);
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;

    pub fn dso__for_each_symbol(dso: *mut dso, pos: *mut symbol, n: *mut c_void);
    pub fn dso__next_symbol_by_name(dso: *mut dso, idx: *mut size_t) -> *mut symbol;
    pub fn symbol__match_symbol_name(
        name: *const c_char,
        sym_name: *const c_char,
        flags: c_int,
    ) -> bool;
}

unsafe fn rc_chk_access_const<'a>(map: *const map) -> &'a map {
    &*map
}

unsafe fn rc_chk_access_mut<'a>(map: *mut map) -> &'a mut map {
    &mut *map
}

unsafe fn rc_chk_get(result: &mut *mut map, map: *mut map) -> bool {
    *result = map;
    !result.is_null()
}

pub unsafe fn map__dso(map: *const map) -> *mut dso {
    rc_chk_access_const(map).dso
}

pub unsafe fn map__start(map: *const map) -> u64 {
    rc_chk_access_const(map).start
}

pub unsafe fn map__end(map: *const map) -> u64 {
    rc_chk_access_const(map).end
}

pub unsafe fn map__pgoff(map: *const map) -> u64 {
    rc_chk_access_const(map).pgoff
}

pub unsafe fn map__reloc(map: *const map) -> u64 {
    rc_chk_access_const(map).reloc
}

pub unsafe fn map__flags(map: *const map) -> u32 {
    rc_chk_access_const(map).flags
}

pub unsafe fn map__prot(map: *const map) -> u32 {
    rc_chk_access_const(map).prot
}

pub unsafe fn map__priv(map: *const map) -> bool {
    rc_chk_access_const(map).priv_
}

pub unsafe fn map__hit(map: *const map) -> bool {
    rc_chk_access_const(map).hit
}

pub unsafe fn map__refcnt(map: *mut map) -> *mut refcount_t {
    &mut rc_chk_access_mut(map).refcnt
}

pub unsafe fn map__erange_warned(map: *mut map) -> bool {
    rc_chk_access_const(map).erange_warned
}

pub unsafe fn map__size(map: *const map) -> size_t {
    (map__end(map)).wrapping_sub(map__start(map)) as size_t
}

/* ip -> dso rip */
pub unsafe fn map__dso_map_ip(map: *const map, ip: u64) -> u64 {
    ip.wrapping_sub(map__start(map)).wrapping_add(map__pgoff(map))
}

/* dso rip -> ip */
pub unsafe fn map__dso_unmap_ip(map: *const map, rip: u64) -> u64 {
    rip.wrapping_add(map__start(map)).wrapping_sub(map__pgoff(map))
}

pub unsafe fn map__map_ip(map: *const map, ip_or_rip: u64) -> u64 {
    if rc_chk_access_const(map).mapping_type == mapping_type::MAPPING_TYPE__DSO {
        map__dso_map_ip(map, ip_or_rip)
    } else {
        ip_or_rip
    }
}

pub unsafe fn map__unmap_ip(map: *const map, ip_or_rip: u64) -> u64 {
    if rc_chk_access_const(map).mapping_type == mapping_type::MAPPING_TYPE__DSO {
        map__dso_unmap_ip(map, ip_or_rip)
    } else {
        ip_or_rip
    }
}

/* map__for_each_symbol - iterate over the symbols in the given map
 *
 * @map: the 'struct map *' in which symbols are iterated
 * @pos: the 'struct symbol *' to use as a loop cursor
 * @n: the 'struct rb_node *' to use as a temporary storage
 * Note: caller must ensure map->dso is not NULL (map is loaded).
 */
#[macro_export]
macro_rules! map__for_each_symbol {
    ($map:expr, $pos:expr, $n:expr) => {
        dso__for_each_symbol(map__dso($map), $pos, $n)
    };
}

/* map__for_each_symbol_with_name - iterate over the symbols in the given map
 *                                  that have the given name
 *
 * @map: the 'struct map *' in which symbols are iterated
 * @sym_name: the symbol name
 * @pos: the 'struct symbol *' to use as a loop cursor
 * @idx: the cursor index in the symbol names array
 */
#[macro_export]
macro_rules! __map__for_each_symbol_by_name {
    ($map:expr, $sym_name:expr, $pos:ident, $idx:ident, $body:block) => {{
        $pos = map__find_symbol_by_name_idx($map, $sym_name, &mut $idx);
        while !$pos.is_null()
            && !symbol__match_symbol_name(
                (*$pos).name,
                $sym_name,
                SYMBOL_TAG_INCLUDE__DEFAULT_ONLY,
            )
        {
            $body
            $pos = dso__next_symbol_by_name(map__dso($map), &mut $idx);
        }
    }};
}

#[macro_export]
macro_rules! map__for_each_symbol_by_name {
    ($map:expr, $sym_name:expr, $pos:ident, $idx:ident, $body:block) => {
        __map__for_each_symbol_by_name!($map, $sym_name, $pos, $idx, $body)
    };
}

pub unsafe fn map__get(map: *mut map) -> *mut map {
    let mut result: *mut map = core::ptr::null_mut();

    if rc_chk_get(&mut result, map) {
        refcount_inc(map__refcnt(map));
    }

    result
}

pub unsafe fn __map__zput(map: *mut *mut map) {
    map__put(*map);
    *map = core::ptr::null_mut();
}

#[macro_export]
macro_rules! map__zput {
    ($map:expr) => {
        __map__zput(&mut $map)
    };
}

pub unsafe fn __map__is_kmodule(map: *const map) -> bool {
    !__map__is_kernel(map)
        && !__map__is_extra_kernel_map(map)
        && !__map__is_bpf_prog(map)
        && !__map__is_ool(map)
        && !__map__is_bpf_image(map)
}

pub const ENTRY_TRAMPOLINE_NAME: &[u8] = b"__entry_SYSCALL_64_trampoline\0";

pub unsafe fn is_entry_trampoline(name: *const c_char) -> bool {
    strcmp(name, ENTRY_TRAMPOLINE_NAME.as_ptr() as *const c_char) == 0
}

pub unsafe fn is_bpf_image(name: *const c_char) -> bool {
    strncmp(
        name,
        b"bpf_trampoline_\0".as_ptr() as *const c_char,
        b"bpf_trampoline_".len(),
    ) == 0
        || strncmp(
            name,
            b"bpf_dispatcher_\0".as_ptr() as *const c_char,
            b"bpf_dispatcher_".len(),
        ) == 0
}

pub unsafe fn is_anon_memory(filename: *const c_char) -> c_int {
    (strcmp(filename, b"//anon\0".as_ptr() as *const c_char) == 0
        || strncmp(
            filename,
            b"/dev/zero\0".as_ptr() as *const c_char,
            b"/dev/zero".len(),
        ) == 0
        || strncmp(
            filename,
            b"/anon_hugepage\0".as_ptr() as *const c_char,
            b"/anon_hugepage".len(),
        ) == 0) as c_int
}

pub unsafe fn is_no_dso_memory(filename: *const c_char) -> c_int {
    (strncmp(filename, b"[stack\0".as_ptr() as *const c_char, 6) == 0
        || strncmp(filename, b"/SYSV\0".as_ptr() as *const c_char, 5) == 0
        || strcmp(filename, b"[heap]\0".as_ptr() as *const c_char) == 0) as c_int
}

pub unsafe fn map__set_start(map: *mut map, start: u64) {
    rc_chk_access_mut(map).start = start;
}

pub unsafe fn map__set_end(map: *mut map, end: u64) {
    rc_chk_access_mut(map).end = end;
}

pub unsafe fn map__set_pgoff(map: *mut map, pgoff: u64) {
    rc_chk_access_mut(map).pgoff = pgoff;
}

pub unsafe fn map__add_pgoff(map: *mut map, inc: u64) {
    let access = rc_chk_access_mut(map);
    access.pgoff = access.pgoff.wrapping_add(inc);
}

pub unsafe fn map__set_reloc(map: *mut map, reloc: u64) {
    rc_chk_access_mut(map).reloc = reloc;
}

pub unsafe fn map__set_priv(map: *mut map) {
    rc_chk_access_mut(map).priv_ = true;
}

pub unsafe fn map__set_hit(map: *mut map) {
    rc_chk_access_mut(map).hit = true;
}

pub unsafe fn map__set_erange_warned(map: *mut map) {
    rc_chk_access_mut(map).erange_warned = true;
}

pub unsafe fn map__set_dso(map: *mut map, dso: *mut dso) {
    rc_chk_access_mut(map).dso = dso;
}

pub unsafe fn map__set_mapping_type(map: *mut map, type_: mapping_type) {
    rc_chk_access_mut(map).mapping_type = type_;
}

pub unsafe fn map__mapping_type(map: *mut map) -> mapping_type {
    rc_chk_access_const(map).mapping_type
}
