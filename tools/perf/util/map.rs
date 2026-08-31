// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/map.c. C includes are represented by the extern
// declarations and type references below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;
type u32 = u32;
type size_t = usize;

const PATH_MAX: usize = 4096;
const MAP_HUGETLB: u32 = 0x40000;
const PROT_EXEC: u32 = 0x4;
const DSO__DELETED: &[u8] = b"(deleted)\0";

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    refs: c_int,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    rb_root: rb_root,
    rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct rb_node_member {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    start: u64,
    end: u64,
    rb_node: rb_node,
}

#[repr(C)]
pub struct build_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_id {
    build_id: build_id,
}

#[repr(C)]
pub struct dsos {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    dsos: dsos,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmap {
    name: [c_char; 0],
    kmaps: *mut maps,
}

#[repr(C)]
pub struct map {
    refcnt: refcount_t,
    dso: *mut dso,
    prot: u32,
    flags: u32,
    mapping_type: c_int,
}

#[repr(C)]
pub struct srccode_state {
    srcfile: *mut c_char,
    line: c_int,
}

#[repr(C)]
pub struct symbol_conf_t {
    show_kernel_path: bool,
    pad_output_len_dso: size_t,
}

extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut SRCLINE_UNKNOWN: *mut c_char;
    static SBUILD_ID_SIZE: size_t;
    static MAPPING_TYPE__DSO: c_int;
    static MAPPING_TYPE__IDENTITY: c_int;
    static DSO_BINARY_TYPE__BPF_PROG_INFO: c_int;
    static DSO_BINARY_TYPE__BPF_IMAGE: c_int;
    static DSO_BINARY_TYPE__OOL: c_int;
    static DSO_SPACE__USER: c_int;

    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn memdup(src: *const c_void, len: size_t) -> *mut c_void;

    fn is_anon_memory(filename: *const c_char) -> bool;
    fn is_no_dso_memory(filename: *const c_char) -> bool;
    fn is_vdso_map(filename: *const c_char) -> bool;
    fn is_bpf_image(name: *const c_char) -> bool;

    fn thread__nsinfo(thread: *mut thread) -> *mut nsinfo;
    fn nsinfo__get(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__put(nsi: *mut nsinfo);
    fn nsinfo__copy(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__clear_need_setns(nsi: *mut nsinfo);
    fn nsinfo__pid(nsi: *mut nsinfo) -> c_int;

    fn machine__findnew_vdso(machine: *mut machine, thread: *mut thread) -> *mut dso;
    fn machine__findnew_dso_id(machine: *mut machine, filename: *const c_char, id: *const dso_id) -> *mut dso;
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn maps__machine(maps: *mut maps) -> *mut machine;

    fn dso__get(dso: *mut dso) -> *mut dso;
    fn dso__put(dso: *mut dso);
    fn dso__zput(dso: *mut dso);
    fn dso__kernel(dso: *const dso) -> c_int;
    fn dso__binary_type(dso: *const dso) -> c_int;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn dso__has_symbols(dso: *const dso) -> bool;
    fn dso__symbols(dso: *mut dso) -> *mut rb_root_cached;
    fn dso__long_name(dso: *const dso) -> *const c_char;
    fn dso__loaded(dso: *const dso) -> bool;
    fn dso__load(dso: *mut dso, map: *mut map) -> c_int;
    fn dso__has_build_id(dso: *const dso) -> bool;
    fn dso__bid(dso: *const dso) -> *const build_id;
    fn dso__name(dso: *const dso) -> *const c_char;
    fn dso__is_kcore(dso: *const dso) -> bool;
    fn dso__is_object_file(dso: *const dso) -> bool;
    fn dso__find_symbol(dso: *mut dso, addr: u64) -> *mut symbol;
    fn dso__sort_by_name(dso: *mut dso);
    fn dso__find_symbol_by_name(dso: *mut dso, name: *const c_char, idx: *mut size_t) -> *mut symbol;
    fn dso__lock(dso: *mut dso) -> *mut c_void;
    fn dso__set_nsinfo(dso: *mut dso, nsi: *mut nsinfo);
    fn dso__set_loaded(dso: *mut dso);
    fn dso__header_build_id(dso: *const dso) -> bool;
    fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
    fn dso__set_header_build_id(dso: *mut dso, value: c_int);
    fn dso__adjust_symbols(dso: *const dso) -> bool;
    fn dso__rel(dso: *const dso) -> bool;
    fn dso__text_offset(dso: *const dso) -> u64;

    fn build_id__is_defined(bid: *const build_id) -> bool;
    fn build_id__snprintf(bid: *const build_id, s: *mut c_char, size: size_t) -> c_int;
    fn dsos__find(dsos: *mut dsos, name: *const c_char, cmp_short: bool) -> *mut dso;

    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_last(root: *mut rb_root) -> *mut rb_node;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn scnprintf_pad(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn get_srcline(dso: *mut dso, addr: u64, sym: *mut symbol, show_sym: bool, show_addr: bool, ip: u64) -> *mut c_char;
    fn zfree_srcline(srcline: *mut *mut c_char);
    fn zfree(ptr: *mut *mut c_void);
    fn is_entry_trampoline(name: *const c_char) -> bool;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn map__set_start(map: *mut map, start: u64);
    fn map__set_end(map: *mut map, end: u64);
    fn map__set_pgoff(map: *mut map, pgoff: u64);
    fn map__set_dso(map: *mut map, dso: *mut dso);
    fn map__refcnt(map: *mut map) -> *mut refcount_t;
    fn map__reloc(map: *const map) -> u64;
    fn map__erange_warned(map: *const map) -> bool;
    fn map__priv(map: *const map) -> bool;
    fn map__hit(map: *const map) -> bool;
    fn map__set_mapping_type(map: *mut map, ty: c_int);
    fn map__dso(map: *const map) -> *mut dso;
    fn map__start(map: *const map) -> u64;
    fn map__end(map: *const map) -> u64;
    fn map__pgoff(map: *const map) -> u64;
    fn map__unmap_ip(map: *const map, ip: u64) -> u64;
    fn map__map_ip(map: *const map, ip: u64) -> u64;
    fn refcount_set(r: *mut refcount_t, v: c_int);
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
}

unsafe fn RC_CHK_ACCESS<T>(p: *mut T) -> *mut T { p }
unsafe fn RC_CHK_FREE<T>(_p: *mut T) {}
unsafe fn RC_CHK_PUT<T>(_p: *mut T) {}
unsafe fn ADD_RC_CHK(result: &mut *mut map, map_ptr: *mut map) -> bool {
    *result = map_ptr;
    !map_ptr.is_null()
}
unsafe fn BUG_ON(cond: bool) {
    if cond {
        panic!("BUG_ON");
    }
}

unsafe fn rb_entry_symbol(ptr: *mut rb_node) -> *mut symbol {
    ptr as *mut symbol
}

#[inline]
unsafe fn is_android_lib(filename: *const c_char) -> c_int {
    (strstarts(filename, c"/data/app-lib/".as_ptr()) ||
     strstarts(filename, c"/system/lib/".as_ptr())) as c_int
}

#[inline]
unsafe fn replace_android_lib(filename: *const c_char, newfilename: *mut c_char) -> bool {
    let mut lib_length: size_t = 0;
    let libname = strrchr(filename, '/' as c_int);
    if !libname.is_null() {
        lib_length = strlen(libname);
    }

    let app_abi = getenv(c"APP_ABI".as_ptr());
    if app_abi.is_null() {
        return false;
    }

    let app_abi_length = strlen(app_abi);

    if strstarts(filename, c"/data/app-lib/".as_ptr()) {
        if app_abi_length == 0 {
            return false;
        }

        let mut new_length = 7 + app_abi_length + lib_length;
        let apk_path = getenv(c"APK_PATH".as_ptr());
        if !apk_path.is_null() {
            new_length += strlen(apk_path) + 1;
            if new_length > PATH_MAX {
                return false;
            }
            snprintf(newfilename, new_length, c"%s/libs/%s/%s".as_ptr(), apk_path, app_abi, libname);
        } else {
            if new_length > PATH_MAX {
                return false;
            }
            snprintf(newfilename, new_length, c"libs/%s/%s".as_ptr(), app_abi, libname);
        }
        return true;
    }

    if strstarts(filename, c"/system/lib/".as_ptr()) {
        let ndk = getenv(c"NDK_ROOT".as_ptr());
        let app = getenv(c"APP_PLATFORM".as_ptr());

        if ndk.is_null() || app.is_null() {
            return false;
        }

        let ndk_length = strlen(ndk) as c_int;
        let app_length = strlen(app) as c_int;

        if !(ndk_length != 0 && app_length != 0 && app_abi_length != 0) {
            return false;
        }

        let arch = if strncmp(app_abi, c"arm".as_ptr(), 3) == 0 {
            c"arm".as_ptr()
        } else if strncmp(app_abi, c"mips".as_ptr(), 4) == 0 {
            c"mips".as_ptr()
        } else if strncmp(app_abi, c"x86".as_ptr(), 3) == 0 {
            c"x86".as_ptr()
        } else {
            ptr::null()
        };

        if arch.is_null() {
            return false;
        }

        let new_length = 27 + ndk_length as size_t + app_length as size_t + lib_length + strlen(arch);
        if new_length > PATH_MAX {
            return false;
        }
        snprintf(
            newfilename,
            new_length,
            c"%.*s/platforms/%.*s/arch-%s/usr/lib/%s".as_ptr(),
            ndk_length,
            ndk,
            app_length,
            app,
            arch,
            libname,
        );

        return true;
    }
    false
}

unsafe fn map__init(map: *mut map, start: u64, end: u64, pgoff: u64, dso: *mut dso, prot: u32, flags: u32) {
    map__set_start(map, start);
    map__set_end(map, end);
    map__set_pgoff(map, pgoff);
    assert!(map__reloc(map) == 0);
    map__set_dso(map, dso__get(dso));
    refcount_set(map__refcnt(map), 1);
    (*RC_CHK_ACCESS(map)).prot = prot;
    (*RC_CHK_ACCESS(map)).flags = flags;
    map__set_mapping_type(map, MAPPING_TYPE__DSO);
    assert!(map__erange_warned(map) == false);
    assert!(map__priv(map) == false);
    assert!(map__hit(map) == false);
}

#[no_mangle]
pub unsafe extern "C" fn map__new(machine: *mut machine, start: u64, len: u64, mut pgoff: u64, id: *const dso_id, prot: u32, flags: u32, mut filename: *mut c_char, thread: *mut thread) -> *mut map {
    let mut result: *mut map = ptr::null_mut();
    let map_ptr = zalloc(size_of::<map>()) as *mut map;
    let mut nsi: *mut nsinfo = ptr::null_mut();

    if ADD_RC_CHK(&mut result, map_ptr) {
        let mut newfilename = [0 as c_char; PATH_MAX];
        let mut dso_ptr: *mut dso;

        let android = is_android_lib(filename);
        let anon = is_anon_memory(filename) || (flags & MAP_HUGETLB) != 0;
        let vdso = is_vdso_map(filename);
        let no_dso = is_no_dso_memory(filename);
        nsi = nsinfo__get(thread__nsinfo(thread));

        if (anon || no_dso) && !nsi.is_null() && (prot & PROT_EXEC) != 0 {
            snprintf(newfilename.as_mut_ptr(), newfilename.len(), c"/tmp/perf-%d.map".as_ptr(), nsinfo__pid(nsi));
            filename = newfilename.as_mut_ptr();
        }

        if android != 0 && replace_android_lib(filename, newfilename.as_mut_ptr()) {
            filename = newfilename.as_mut_ptr();
        }

        if vdso {
            /*
             * The vdso maps are always on the host and not the container.
             * Ensure that we don't use setns to look them up.
             */
            let nnsi = nsinfo__copy(nsi);
            if !nnsi.is_null() {
                nsinfo__put(nsi);
                nsinfo__clear_need_setns(nnsi);
                nsi = nnsi;
            }
            pgoff = 0;
            dso_ptr = machine__findnew_vdso(machine, thread);
        } else {
            dso_ptr = machine__findnew_dso_id(machine, filename, id);
        }

        if dso_ptr.is_null() {
            nsinfo__put(nsi);
            RC_CHK_FREE(result);
            return ptr::null_mut();
        }

        assert!(dso__kernel(dso_ptr) == 0);
        map__init(result, start, start.wrapping_add(len), pgoff, dso_ptr, prot, flags);

        if anon || no_dso {
            (*map_ptr).mapping_type = MAPPING_TYPE__IDENTITY;

            /*
             * Set memory without DSO as loaded. All map__find_* functions still
             * return NULL, and we avoid the unnecessary map__load warning.
             */
            if (prot & PROT_EXEC) == 0 {
                dso__set_loaded(dso_ptr);
            }
        }
        mutex_lock(dso__lock(dso_ptr));
        dso__set_nsinfo(dso_ptr, nsi);
        mutex_unlock(dso__lock(dso_ptr));

        if !build_id__is_defined(&(*id).build_id) {
            /*
             * If the mmap event had no build ID, search for an existing dso from
             * the build ID header by name. Otherwise only the dso loaded at the
             * time of reading the header will have the build ID set and all
             * future mmaps will have it missing.
             */
            let header_bid_dso = dsos__find(&mut (*machine).dsos, filename, false);

            if !header_bid_dso.is_null() && dso__header_build_id(header_bid_dso) {
                dso__set_build_id(dso_ptr, dso__bid(header_bid_dso));
                dso__set_header_build_id(dso_ptr, 1);
            }
            dso__put(header_bid_dso);
        }
        dso__put(dso_ptr);
    }
    result
}

/*
 * Constructor variant for modules (where we know from /proc/modules where
 * they are loaded) and for vmlinux, where only after we load all the
 * symbols we'll know where it starts and ends.
 */
#[no_mangle]
pub unsafe extern "C" fn map__new2(start: u64, dso: *mut dso) -> *mut map {
    let mut result: *mut map = ptr::null_mut();
    let mut size = size_of::<map>();

    if dso__kernel(dso) != 0 {
        size += size_of::<kmap>();
    }

    let map_ptr = calloc(1, size) as *mut map;
    if ADD_RC_CHK(&mut result, map_ptr) {
        /* ->end will be filled after we load all the symbols. */
        map__init(result, start, 0, 0, dso, 0, 0);
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn __map__is_kernel(map: *const map) -> bool {
    if dso__kernel(map__dso(map)) == 0 {
        return false;
    }
    machine__kernel_map(maps__machine(map__kmaps(map as *mut map))) == map as *mut map
}

#[no_mangle]
pub unsafe extern "C" fn __map__is_extra_kernel_map(map: *const map) -> bool {
    let kmap = __map__kmap(map as *mut map);
    !kmap.is_null() && (*kmap).name[0] != 0
}

#[no_mangle]
pub unsafe extern "C" fn __map__is_bpf_prog(map: *const map) -> bool {
    let dso = map__dso(map);

    if dso__binary_type(dso) == DSO_BINARY_TYPE__BPF_PROG_INFO {
        return true;
    }

    /*
     * If PERF_RECORD_BPF_EVENT is not included, the dso will not have type of
     * DSO_BINARY_TYPE__BPF_PROG_INFO. In such cases, we can guess the type
     * based on name.
     */
    let name = dso__short_name(dso);
    !name.is_null() && strstr(name, c"bpf_prog_".as_ptr()) == name as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn __map__is_bpf_image(map: *const map) -> bool {
    let dso = map__dso(map);

    if dso__binary_type(dso) == DSO_BINARY_TYPE__BPF_IMAGE {
        return true;
    }

    /*
     * If PERF_RECORD_KSYMBOL is not included, the dso will not have type of
     * DSO_BINARY_TYPE__BPF_IMAGE. In such cases, we can guess the type based on
     * name.
     */
    let name = dso__short_name(dso);
    !name.is_null() && is_bpf_image(name)
}

#[no_mangle]
pub unsafe extern "C" fn __map__is_ool(map: *const map) -> bool {
    let dso = map__dso(map);
    !dso.is_null() && dso__binary_type(dso) == DSO_BINARY_TYPE__OOL
}

#[no_mangle]
pub unsafe extern "C" fn map__has_symbols(map: *const map) -> bool {
    dso__has_symbols(map__dso(map))
}

unsafe fn map__exit(map: *mut map) {
    BUG_ON(refcount_read(map__refcnt(map)) != 0);
    dso__zput((*RC_CHK_ACCESS(map)).dso);
}

#[no_mangle]
pub unsafe extern "C" fn map__delete(map: *mut map) {
    map__exit(map);
    RC_CHK_FREE(map);
}

#[no_mangle]
pub unsafe extern "C" fn map__put(map: *mut map) {
    if !map.is_null() && refcount_dec_and_test(map__refcnt(map)) {
        map__delete(map);
    } else {
        RC_CHK_PUT(map);
    }
}

#[no_mangle]
pub unsafe extern "C" fn map__fixup_start(map: *mut map) {
    let dso = map__dso(map);
    let symbols = dso__symbols(dso);
    let nd = rb_first_cached(symbols);

    if !nd.is_null() {
        let sym = rb_entry_symbol(nd);
        map__set_start(map, (*sym).start);
    }
}

#[no_mangle]
pub unsafe extern "C" fn map__fixup_end(map: *mut map) {
    let dso = map__dso(map);
    let symbols = dso__symbols(dso);
    let nd = rb_last(&mut (*symbols).rb_root);

    if !nd.is_null() {
        let sym = rb_entry_symbol(nd);
        map__set_end(map, (*sym).end);
    }
}

#[no_mangle]
pub unsafe extern "C" fn map__load(map: *mut map) -> c_int {
    let dso = map__dso(map);
    let name = dso__long_name(dso);

    if dso__loaded(dso) {
        return 0;
    }

    let nr = dso__load(dso, map);
    if nr < 0 {
        if dso__has_build_id(dso) {
            let mut sbuild_id = [0 as c_char; 128];
            build_id__snprintf(dso__bid(dso), sbuild_id.as_mut_ptr(), sbuild_id.len());
            pr_debug(c"%s with build id %s not found".as_ptr(), name, sbuild_id.as_ptr());
        } else {
            pr_debug(c"Failed to open %s".as_ptr(), name);
        }

        pr_debug(c", continuing without symbols\n".as_ptr());
        return -1;
    } else if nr == 0 {
        // Original C condition: #ifdef HAVE_LIBELF_SUPPORT
        #[cfg(HAVE_LIBELF_SUPPORT)]
        {
            let len = strlen(name);
            let real_len = len - DSO__DELETED.len();

            if len > DSO__DELETED.len()
                && strcmp(name.add(real_len + 1), DSO__DELETED.as_ptr() as *const c_char) == 0
            {
                pr_debug(
                    c"%.*s was updated (is prelink enabled?). Restart the long running apps that use it!\n".as_ptr(),
                    real_len as c_int,
                    name,
                );
            } else {
                pr_debug(c"no symbols found in %s, maybe install a debug package?\n".as_ptr(), name);
            }
        }
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol {
    if map__load(map) < 0 {
        return ptr::null_mut();
    }

    dso__find_symbol(map__dso(map), addr)
}

#[no_mangle]
pub unsafe extern "C" fn map__find_symbol_by_name_idx(map: *mut map, name: *const c_char, idx: *mut size_t) -> *mut symbol {
    if map__load(map) < 0 {
        return ptr::null_mut();
    }

    let dso = map__dso(map);
    dso__sort_by_name(dso);

    dso__find_symbol_by_name(dso, name, idx)
}

#[no_mangle]
pub unsafe extern "C" fn map__find_symbol_by_name(map: *mut map, name: *const c_char) -> *mut symbol {
    let mut idx: size_t = 0;
    map__find_symbol_by_name_idx(map, name, &mut idx)
}

#[no_mangle]
pub unsafe extern "C" fn map__clone(from: *mut map) -> *mut map {
    let mut result: *mut map = ptr::null_mut();
    let mut size = size_of::<map>();
    let dso = map__dso(from);

    if !dso.is_null() && dso__kernel(dso) != 0 {
        size += size_of::<kmap>();
    }

    let map_ptr = memdup(RC_CHK_ACCESS(from) as *const c_void, size) as *mut map;
    if ADD_RC_CHK(&mut result, map_ptr) {
        refcount_set(&mut (*map_ptr).refcnt, 1);
        (*map_ptr).dso = dso__get(dso);
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn map__fprintf(map: *mut map, fp: *mut FILE) -> size_t {
    let dso = map__dso(map);

    fprintf(
        fp,
        c" %lx-%lx %lx %s\n".as_ptr(),
        map__start(map),
        map__end(map),
        map__pgoff(map),
        dso__name(dso),
    ) as size_t
}

unsafe fn prefer_dso_long_name(dso: *const dso, print_off: bool) -> bool {
    !dso__long_name(dso).is_null()
        && (symbol_conf.show_kernel_path
            || (print_off && (*dso__name(dso) == '[' as c_char || dso__is_kcore(dso))))
}

unsafe fn __map__fprintf_dsoname(map: *mut map, print_off: bool, fp: *mut FILE) -> size_t {
    let pad_len = symbol_conf.pad_output_len_dso;
    let mut buf = vec![0 as c_char; pad_len + 1];
    let mut dsoname = c"[unknown]".as_ptr();
    let dso = if !map.is_null() { map__dso(map) } else { ptr::null_mut() };

    if !dso.is_null() {
        if prefer_dso_long_name(dso, print_off) {
            dsoname = dso__long_name(dso);
        } else {
            dsoname = dso__name(dso);
        }
    }

    if pad_len != 0 {
        scnprintf_pad(buf.as_mut_ptr(), pad_len, c"%s".as_ptr(), dsoname);
        dsoname = buf.as_ptr();
    }

    fprintf(fp, c"%s".as_ptr(), dsoname) as size_t
}

#[no_mangle]
pub unsafe extern "C" fn map__fprintf_dsoname(map: *mut map, fp: *mut FILE) -> size_t {
    __map__fprintf_dsoname(map, false, fp)
}

#[no_mangle]
pub unsafe extern "C" fn map__fprintf_dsoname_dsoff(map: *mut map, mut print_off: bool, addr: u64, fp: *mut FILE) -> size_t {
    let dso = if !map.is_null() { map__dso(map) } else { ptr::null_mut() };
    let mut printed: c_int = 0;

    if print_off && (dso.is_null() || !dso__is_object_file(dso)) {
        print_off = false;
    }
    printed += fprintf(fp, c" (".as_ptr());
    printed += __map__fprintf_dsoname(map, print_off, fp) as c_int;
    if print_off {
        printed += fprintf(fp, c"+0x%lx".as_ptr(), addr);
    }
    printed += fprintf(fp, c")".as_ptr());

    printed as size_t
}

#[no_mangle]
pub unsafe extern "C" fn map__srcline(map: *mut map, addr: u64, sym: *mut symbol) -> *mut c_char {
    if map.is_null() {
        return SRCLINE_UNKNOWN;
    }

    get_srcline(map__dso(map), map__rip_2objdump(map, addr), sym, true, true, addr)
}

#[no_mangle]
pub unsafe extern "C" fn map__fprintf_srcline(map: *mut map, addr: u64, prefix: *const c_char, fp: *mut FILE) -> c_int {
    let dso = if !map.is_null() { map__dso(map) } else { ptr::null_mut() };
    let mut ret = 0;

    if !dso.is_null() {
        let mut srcline = map__srcline(map, addr, ptr::null_mut());
        if srcline != SRCLINE_UNKNOWN {
            ret = fprintf(fp, c"%s%s".as_ptr(), prefix, srcline);
        }
        zfree_srcline(&mut srcline);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn srccode_state_free(state: *mut srccode_state) {
    zfree(&mut (*state).srcfile as *mut *mut c_char as *mut *mut c_void);
    (*state).line = 0;
}

/**
 * map__rip_2objdump - convert symbol start address to objdump address.
 * @map: memory map
 * @rip: symbol start address
 *
 * objdump wants/reports absolute IPs for ET_EXEC, and RIPs for ET_DYN.
 * map->dso->adjust_symbols==1 for ET_EXEC-like cases except ET_REL which is
 * relative to section start.
 *
 * Return: Address suitable for passing to "objdump --start-address="
 */
#[no_mangle]
pub unsafe extern "C" fn map__rip_2objdump(mut map: *const map, rip: u64) -> u64 {
    let kmap = __map__const_kmap(map);
    let dso = map__dso(map);

    /*
     * vmlinux does not have program headers for PTI entry trampolines and kcore
     * may not either. However the trampoline object code is on the main kernel
     * map, so just use that instead.
     */
    if !kmap.is_null() && is_entry_trampoline((*kmap).name.as_ptr()) && !(*kmap).kmaps.is_null() {
        let machine = maps__machine((*kmap).kmaps);

        if !machine.is_null() {
            let kernel_map = machine__kernel_map(machine);

            if !kernel_map.is_null() {
                map = kernel_map;
            }
        }
    }

    if !dso__adjust_symbols(dso) {
        return rip;
    }

    if dso__rel(dso) {
        return rip.wrapping_sub(map__pgoff(map));
    }

    if dso__kernel(dso) == DSO_SPACE__USER {
        return rip.wrapping_add(dso__text_offset(dso));
    }

    map__unmap_ip(map, rip).wrapping_sub(map__reloc(map))
}

/**
 * map__objdump_2mem - convert objdump address to a memory address.
 * @map: memory map
 * @ip: objdump address
 *
 * Closely related to map__rip_2objdump(), this function takes an address from
 * objdump and converts it to a memory address.  Note this assumes that @map
 * contains the address.  To be sure the result is valid, check it forwards
 * e.g. map__rip_2objdump(map__map_ip(map, map__objdump_2mem(map, ip))) == ip
 *
 * Return: Memory address.
 */
#[no_mangle]
pub unsafe extern "C" fn map__objdump_2mem(map: *const map, ip: u64) -> u64 {
    let dso = map__dso(map);

    if !dso__adjust_symbols(dso) {
        return map__unmap_ip(map, ip);
    }

    if dso__rel(dso) {
        return map__unmap_ip(map, ip.wrapping_add(map__pgoff(map)));
    }

    if dso__kernel(dso) == DSO_SPACE__USER {
        return map__unmap_ip(map, ip.wrapping_sub(dso__text_offset(dso)));
    }

    ip.wrapping_add(map__reloc(map))
}

/* convert objdump address to relative address.  (To be removed) */
#[no_mangle]
pub unsafe extern "C" fn map__objdump_2rip(map: *const map, ip: u64) -> u64 {
    let dso = map__dso(map);

    if !dso__adjust_symbols(dso) {
        return ip;
    }

    if dso__rel(dso) {
        return ip.wrapping_add(map__pgoff(map));
    }

    if dso__kernel(dso) == DSO_SPACE__USER {
        return ip.wrapping_sub(dso__text_offset(dso));
    }

    map__map_ip(map, ip.wrapping_add(map__reloc(map)))
}

#[no_mangle]
pub unsafe extern "C" fn map__contains_symbol(map: *const map, sym: *const symbol) -> bool {
    let ip = map__unmap_ip(map, (*sym).start);
    ip >= map__start(map) && ip < map__end(map)
}

#[no_mangle]
pub unsafe extern "C" fn __map__kmap(map: *mut map) -> *mut kmap {
    let dso = map__dso(map);

    if dso.is_null() || dso__kernel(dso) == 0 {
        return ptr::null_mut();
    }
    RC_CHK_ACCESS(map).add(1) as *mut kmap
}

unsafe fn __map__const_kmap(map: *const map) -> *const kmap {
    let dso = map__dso(map);

    if dso.is_null() || dso__kernel(dso) == 0 {
        return ptr::null();
    }
    RC_CHK_ACCESS(map as *mut map).add(1) as *const kmap
}

#[no_mangle]
pub unsafe extern "C" fn map__kmap(map: *mut map) -> *mut kmap {
    let kmap = __map__kmap(map);

    if kmap.is_null() {
        pr_err(c"Internal error: map__kmap with a non-kernel map\n".as_ptr());
    }
    kmap
}

#[no_mangle]
pub unsafe extern "C" fn map__kmaps(map: *mut map) -> *mut maps {
    let kmap = map__kmap(map);

    if kmap.is_null() || (*kmap).kmaps.is_null() {
        pr_err(c"Internal error: map__kmaps with a non-kernel map\n".as_ptr());
        return ptr::null_mut();
    }
    (*kmap).kmaps
}
