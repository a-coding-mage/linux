// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/dsos.c.
// Original C dependencies:
// debug.h, dsos.h, dso.h, util.h, vdso.h, namespaces.h, errno.h,
// stdlib.h, string.h, symbol.h, unistd.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::c_void;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

type size_t = usize;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;

const SBUILD_ID_SIZE: usize = 41;
const DSO_SPACE__KERNEL: c_int = 1;
const PERF_RECORD_MISC_CPUMODE_UNKNOWN: c_int = 0;

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsos {
    pub lock: rw_semaphore,
    pub cnt: c_uint,
    pub allocated: c_uint,
    pub dsos: *mut *mut dso,
    pub sorted: bool,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct build_id {
    pub size: u8,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmod_path {
    pub name: *const c_char,
}

unsafe extern "C" {
    static dso_id_empty: dso_id;

    fn init_rwsem(sem: *mut rw_semaphore);
    fn exit_rwsem(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);

    fn dso__set_dsos(dso: *mut dso, dsos: *mut dsos);
    fn dso__put(dso: *mut dso);
    fn dso__get(dso: *mut dso) -> *mut dso;
    fn dso__hit(dso: *mut dso) -> bool;
    fn dso__is_vdso(dso: *mut dso) -> bool;
    fn dso__has_build_id(dso: *mut dso) -> bool;
    fn dso__nsinfo(dso: *mut dso) -> *mut nsinfo;
    fn dso__long_name(dso: *const dso) -> *const c_char;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn dso__id_const(dso: *const dso) -> *const dso_id;
    fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
    fn dso__filename_with_chroot(dso: *mut dso, name: *const c_char) -> *mut c_char;
    fn dso__set_short_name(dso: *mut dso, name: *const c_char, name_allocated: bool);
    fn dso__new_id(name: *const c_char, id: *const dso_id) -> *mut dso;
    fn __dso__improve_id(dso: *mut dso, id: *const dso_id);
    fn dso__bid(dso: *mut dso) -> *const build_id;
    fn dso__fprintf(dso: *mut dso, fp: *mut FILE) -> size_t;
    fn dso__set_hit(dso: *mut dso);
    fn dso__set_module_info(dso: *mut dso, m: *mut kmod_path, machine: *mut machine);
    fn dso__set_long_name(dso: *mut dso, name: *mut c_char, name_allocated: bool);
    fn dso__set_kernel(dso: *mut dso, space: c_int);
    fn dso__kernel(dso: *mut dso) -> bool;

    fn dso_id__cmp(a: *const dso_id, b: *const dso_id) -> c_int;
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn perf_pid_map_tid(name: *const c_char, tid: *mut c_int) -> bool;
    fn perf_basename(path: *const c_char) -> *const c_char;
    fn build_id__snprintf(bid: *const build_id, sbuild_id: *mut c_char, size: size_t) -> c_int;
    fn is_kernel_module(name: *const c_char, cpumode: c_int) -> bool;

    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
}

unsafe fn zfree_dsos(pptr: *mut *mut *mut dso) {
    if !(*pptr).is_null() {
        free(*pptr as *mut c_void);
        *pptr = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__init(dsos: *mut dsos) {
    unsafe {
        init_rwsem(&mut (*dsos).lock);

        (*dsos).cnt = 0;
        (*dsos).allocated = 0;
        (*dsos).dsos = ptr::null_mut();
        (*dsos).sorted = true;
    }
}

unsafe extern "C" fn dsos__purge(dsos: *mut dsos) {
    unsafe {
        down_write(&mut (*dsos).lock);

        for i in 0..(*dsos).cnt {
            let dso = *(*dsos).dsos.add(i as usize);

            dso__set_dsos(dso, ptr::null_mut());
            dso__put(dso);
        }

        zfree_dsos(&mut (*dsos).dsos);
        (*dsos).cnt = 0;
        (*dsos).allocated = 0;
        (*dsos).sorted = true;

        up_write(&mut (*dsos).lock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__exit(dsos: *mut dsos) {
    unsafe {
        dsos__purge(dsos);
        exit_rwsem(&mut (*dsos).lock);
    }
}

unsafe extern "C" fn __dsos__for_each_dso(
    dsos: *mut dsos,
    cb: Option<unsafe extern "C" fn(*mut dso, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> c_int {
    unsafe {
        for i in 0..(*dsos).cnt {
            let dso = *(*dsos).dsos.add(i as usize);
            let err = cb.unwrap()(dso, data);
            if err != 0 {
                return err;
            }
        }
        0
    }
}

#[repr(C)]
struct dsos__read_build_ids_cb_args {
    with_hits: bool,
    have_build_id: bool,
}

unsafe extern "C" fn dsos__read_build_ids_cb(dso: *mut dso, data: *mut c_void) -> c_int {
    unsafe {
        let args = data as *mut dsos__read_build_ids_cb_args;
        let mut nsc = mem::MaybeUninit::<nscookie>::uninit();
        let mut bid = build_id { size: 0 };

        if (*args).with_hits && !dso__hit(dso) && !dso__is_vdso(dso) {
            return 0;
        }
        if dso__has_build_id(dso) {
            (*args).have_build_id = true;
            return 0;
        }
        nsinfo__mountns_enter(dso__nsinfo(dso), nsc.as_mut_ptr());
        if filename__read_build_id(dso__long_name(dso), &mut bid) > 0 {
            dso__set_build_id(dso, &bid);
            (*args).have_build_id = true;
        } else if *__errno_location() == ENOENT && !dso__nsinfo(dso).is_null() {
            let new_name = dso__filename_with_chroot(dso, dso__long_name(dso));

            if !new_name.is_null() && filename__read_build_id(new_name, &mut bid) > 0 {
                dso__set_build_id(dso, &bid);
                (*args).have_build_id = true;
            }
            free(new_name as *mut c_void);
        }
        nsinfo__mountns_exit(nsc.as_mut_ptr());
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__read_build_ids(dsos: *mut dsos, with_hits: bool) -> bool {
    unsafe {
        let mut args = dsos__read_build_ids_cb_args {
            with_hits,
            have_build_id: false,
        };

        dsos__for_each_dso(
            dsos,
            Some(dsos__read_build_ids_cb),
            &mut args as *mut _ as *mut c_void,
        );
        args.have_build_id
    }
}

unsafe extern "C" fn __dso__cmp_long_name(
    long_name: *const c_char,
    id: *const dso_id,
    b: *const dso,
) -> c_int {
    unsafe {
        let rc = strcmp(long_name, dso__long_name(b));
        if rc != 0 {
            rc
        } else {
            dso_id__cmp(id, dso__id_const(b))
        }
    }
}

unsafe extern "C" fn __dso__cmp_short_name(
    short_name: *const c_char,
    id: *const dso_id,
    b: *const dso,
) -> c_int {
    unsafe {
        let rc = strcmp(short_name, dso__short_name(b));
        if rc != 0 {
            rc
        } else {
            dso_id__cmp(id, dso__id_const(b))
        }
    }
}

unsafe extern "C" fn dsos__cmp_long_name_id_short_name(
    va: *const c_void,
    vb: *const c_void,
) -> c_int {
    unsafe {
        let a = *(va as *const *const dso);
        let b = *(vb as *const *const dso);
        let mut rc = strcmp(dso__long_name(a), dso__long_name(b));

        if rc == 0 {
            rc = dso_id__cmp(dso__id_const(a), dso__id_const(b));
            if rc == 0 {
                rc = strcmp(dso__short_name(a), dso__short_name(b));
            }
        }
        rc
    }
}

#[repr(C)]
struct dsos__key {
    long_name: *const c_char,
    id: *const dso_id,
}

unsafe extern "C" fn dsos__cmp_key_long_name_id(
    vkey: *const c_void,
    vdso: *const c_void,
) -> c_int {
    unsafe {
        let key = vkey as *const dsos__key;
        let dso = *(vdso as *const *const dso);

        __dso__cmp_long_name((*key).long_name, (*key).id, dso)
    }
}

/*
 * Find a matching entry and/or link current entry to RB tree.
 * Either one of the dso or name parameter must be non-NULL or the
 * function will not work.
 */
unsafe extern "C" fn __dsos__find_by_longname_id(
    dsos: *mut dsos,
    name: *const c_char,
    id: *const dso_id,
    write_locked: bool,
) -> *mut dso {
    unsafe {
        let key = dsos__key {
            long_name: name,
            id,
        };
        let mut res: *mut c_void;

        if (*dsos).dsos.is_null() {
            return ptr::null_mut();
        }

        if !(*dsos).sorted {
            if !write_locked {
                let dso: *mut dso;

                up_read(&mut (*dsos).lock);
                down_write(&mut (*dsos).lock);
                dso = __dsos__find_by_longname_id(dsos, name, id, true);
                up_write(&mut (*dsos).lock);
                down_read(&mut (*dsos).lock);
                return dso;
            }
            qsort(
                (*dsos).dsos as *mut c_void,
                (*dsos).cnt as size_t,
                mem::size_of::<*mut dso>(),
                Some(dsos__cmp_long_name_id_short_name),
            );
            (*dsos).sorted = true;
        }

        res = bsearch(
            &key as *const _ as *const c_void,
            (*dsos).dsos as *const c_void,
            (*dsos).cnt as size_t,
            mem::size_of::<*mut dso>(),
            Some(dsos__cmp_key_long_name_id),
        );
        if res.is_null() {
            return ptr::null_mut();
        }

        dso__get(*(res as *mut *mut dso))
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __dsos__add(dsos: *mut dsos, dso: *mut dso) -> c_int {
    unsafe {
        if dso.is_null() {
            return -EINVAL;
        }

        if (*dsos).cnt == (*dsos).allocated {
            let mut to_allocate: c_uint = 2;
            let temp: *mut *mut dso;

            if (*dsos).allocated > 0 {
                to_allocate = (*dsos).allocated.wrapping_mul(2);
            }
            temp = realloc(
                (*dsos).dsos as *mut c_void,
                mem::size_of::<*mut dso>() * to_allocate as usize,
            ) as *mut *mut dso;
            if temp.is_null() {
                return -ENOMEM;
            }
            (*dsos).dsos = temp;
            (*dsos).allocated = to_allocate;
        }
        if !(*dsos).sorted {
            let cnt = (*dsos).cnt as usize;
            *(*dsos).dsos.add(cnt) = dso__get(dso);
            (*dsos).cnt = (*dsos).cnt.wrapping_add(1);
        } else {
            let mut low: c_int = 0;
            let mut high: c_int = (*dsos).cnt as c_int - 1;
            let mut insert: c_int = (*dsos).cnt as c_int; /* Default to inserting at the end. */

            while low <= high {
                let mid = low + (high - low) / 2;
                let cmp = dsos__cmp_long_name_id_short_name(
                    &*(*dsos).dsos.add(mid as usize) as *const *mut dso as *const c_void,
                    &dso as *const *mut dso as *const c_void,
                );

                if cmp < 0 {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                    insert = mid;
                }
            }
            memmove(
                (*dsos).dsos.add((insert + 1) as usize) as *mut c_void,
                (*dsos).dsos.add(insert as usize) as *const c_void,
                ((*dsos).cnt as c_int - insert) as usize * mem::size_of::<*mut dso>(),
            );
            (*dsos).cnt = (*dsos).cnt.wrapping_add(1);
            *(*dsos).dsos.add(insert as usize) = dso__get(dso);
        }
        dso__set_dsos(dso, dsos);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__add(dsos: *mut dsos, dso: *mut dso) -> c_int {
    unsafe {
        let ret: c_int;

        down_write(&mut (*dsos).lock);
        ret = __dsos__add(dsos, dso);
        up_write(&mut (*dsos).lock);
        ret
    }
}

#[repr(C)]
struct dsos__find_id_cb_args {
    name: *const c_char,
    id: *const dso_id,
    res: *mut dso,
}

unsafe extern "C" fn dsos__find_id_cb(dso: *mut dso, data: *mut c_void) -> c_int {
    unsafe {
        let args = data as *mut dsos__find_id_cb_args;

        if __dso__cmp_short_name((*args).name, (*args).id, dso) == 0 {
            (*args).res = dso__get(dso);
            return 1;
        }
        0
    }
}

unsafe extern "C" fn __dsos__find_id(
    dsos: *mut dsos,
    name: *const c_char,
    id: *const dso_id,
    cmp_short: bool,
    write_locked: bool,
) -> *mut dso {
    unsafe {
        let res: *mut dso;

        if cmp_short {
            let mut args = dsos__find_id_cb_args {
                name,
                id,
                res: ptr::null_mut(),
            };

            __dsos__for_each_dso(
                dsos,
                Some(dsos__find_id_cb),
                &mut args as *mut _ as *mut c_void,
            );
            return args.res;
        }
        res = __dsos__find_by_longname_id(dsos, name, id, write_locked);
        res
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__find(
    dsos: *mut dsos,
    name: *const c_char,
    cmp_short: bool,
) -> *mut dso {
    unsafe {
        let res: *mut dso;

        down_read(&mut (*dsos).lock);
        res = __dsos__find_id(dsos, name, &dso_id_empty, cmp_short, false);
        up_read(&mut (*dsos).lock);
        res
    }
}

unsafe extern "C" fn dso__set_basename(dso: *mut dso) {
    unsafe {
        let mut allocated = false;
        let base: *const c_char;
        let mut tid: c_int = 0;

        if perf_pid_map_tid(dso__long_name(dso), &mut tid) {
            let mut jitname: *mut c_char = ptr::null_mut();

            if asprintf(&mut jitname, c"[JIT] tid %d".as_ptr(), tid) < 0 {
                return;
            }
            allocated = true;
            base = jitname;
        } else {
            base = perf_basename(dso__long_name(dso));
        }
        dso__set_short_name(dso, base, allocated);
    }
}

unsafe extern "C" fn __dsos__addnew_id(
    dsos: *mut dsos,
    name: *const c_char,
    id: *const dso_id,
) -> *mut dso {
    unsafe {
        let dso = dso__new_id(name, id);

        if !dso.is_null() {
            /*
             * The dsos lock is held on entry, so rename the dso before
             * adding it to avoid needing to take the dsos lock again to say
             * the array isn't sorted.
             */
            dso__set_basename(dso);
            __dsos__add(dsos, dso);
        }
        dso
    }
}

unsafe extern "C" fn __dsos__findnew_id(
    dsos: *mut dsos,
    name: *const c_char,
    id: *const dso_id,
) -> *mut dso {
    unsafe {
        let dso = __dsos__find_id(dsos, name, id, false, true);

        if !dso.is_null() {
            __dso__improve_id(dso, id);
        }

        if !dso.is_null() {
            dso
        } else {
            __dsos__addnew_id(dsos, name, id)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__findnew_id(
    dsos: *mut dsos,
    name: *const c_char,
    id: *const dso_id,
) -> *mut dso {
    unsafe {
        let dso: *mut dso;
        down_write(&mut (*dsos).lock);
        dso = __dsos__findnew_id(dsos, name, id);
        up_write(&mut (*dsos).lock);
        dso
    }
}

#[repr(C)]
struct dsos__fprintf_buildid_cb_args {
    fp: *mut FILE,
    skip: Option<unsafe extern "C" fn(*mut dso, c_int) -> bool>,
    parm: c_int,
    ret: size_t,
}

unsafe extern "C" fn dsos__fprintf_buildid_cb(dso: *mut dso, data: *mut c_void) -> c_int {
    unsafe {
        let args = data as *mut dsos__fprintf_buildid_cb_args;
        let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];

        if let Some(skip) = (*args).skip {
            if skip(dso, (*args).parm) {
                return 0;
            }
        }
        build_id__snprintf(dso__bid(dso), sbuild_id.as_mut_ptr(), sbuild_id.len());
        (*args).ret = (*args).ret.wrapping_add(fprintf(
            (*args).fp,
            c"%-40s %s\n".as_ptr(),
            sbuild_id.as_ptr(),
            dso__long_name(dso),
        ) as size_t);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__fprintf_buildid(
    dsos: *mut dsos,
    fp: *mut FILE,
    skip: Option<unsafe extern "C" fn(*mut dso, c_int) -> bool>,
    parm: c_int,
) -> size_t {
    unsafe {
        let mut args = dsos__fprintf_buildid_cb_args {
            fp,
            skip,
            parm,
            ret: 0,
        };

        dsos__for_each_dso(
            dsos,
            Some(dsos__fprintf_buildid_cb),
            &mut args as *mut _ as *mut c_void,
        );
        args.ret
    }
}

#[repr(C)]
struct dsos__fprintf_cb_args {
    fp: *mut FILE,
    ret: size_t,
}

unsafe extern "C" fn dsos__fprintf_cb(dso: *mut dso, data: *mut c_void) -> c_int {
    unsafe {
        let args = data as *mut dsos__fprintf_cb_args;

        (*args).ret = (*args).ret.wrapping_add(dso__fprintf(dso, (*args).fp));
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__fprintf(dsos: *mut dsos, fp: *mut FILE) -> size_t {
    unsafe {
        let mut args = dsos__fprintf_cb_args { fp, ret: 0 };

        dsos__for_each_dso(dsos, Some(dsos__fprintf_cb), &mut args as *mut _ as *mut c_void);
        args.ret
    }
}

unsafe extern "C" fn dsos__hit_all_cb(dso: *mut dso, _data: *mut c_void) -> c_int {
    unsafe {
        dso__set_hit(dso);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__hit_all(dsos: *mut dsos) -> c_int {
    unsafe { dsos__for_each_dso(dsos, Some(dsos__hit_all_cb), ptr::null_mut()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__findnew_module_dso(
    dsos: *mut dsos,
    machine: *mut machine,
    m: *mut kmod_path,
    filename: *const c_char,
) -> *mut dso {
    unsafe {
        let mut dso: *mut dso;

        down_write(&mut (*dsos).lock);

        dso = __dsos__find_id(dsos, (*m).name, &dso_id_empty, true, true);
        if !dso.is_null() {
            up_write(&mut (*dsos).lock);
            return dso;
        }
        /*
         * Failed to find the dso so create it. Change the name before adding it
         * to the array, to avoid unnecessary sorts and potential locking
         * issues.
         */
        dso = dso__new_id((*m).name, ptr::null());
        if dso.is_null() {
            up_write(&mut (*dsos).lock);
            return ptr::null_mut();
        }
        dso__set_basename(dso);
        dso__set_module_info(dso, m, machine);
        dso__set_long_name(dso, strdup(filename), true);
        dso__set_kernel(dso, DSO_SPACE__KERNEL);
        __dsos__add(dsos, dso);

        up_write(&mut (*dsos).lock);
        dso
    }
}

unsafe extern "C" fn dsos__find_kernel_dso_cb(dso: *mut dso, data: *mut c_void) -> c_int {
    unsafe {
        let res = data as *mut *mut dso;
        /*
         * The cpumode passed to is_kernel_module is not the cpumode of *this*
         * event. If we insist on passing correct cpumode to is_kernel_module,
         * we should record the cpumode when we adding this dso to the linked
         * list.
         *
         * However we don't really need passing correct cpumode.  We know the
         * correct cpumode must be kernel mode (if not, we should not link it
         * onto kernel_dsos list).
         *
         * Therefore, we pass PERF_RECORD_MISC_CPUMODE_UNKNOWN.
         * is_kernel_module() treats it as a kernel cpumode.
         */
        if !dso__kernel(dso)
            || is_kernel_module(dso__long_name(dso), PERF_RECORD_MISC_CPUMODE_UNKNOWN)
        {
            return 0;
        }

        *res = dso__get(dso);
        1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__find_kernel_dso(dsos: *mut dsos) -> *mut dso {
    unsafe {
        let mut res: *mut dso = ptr::null_mut();

        dsos__for_each_dso(
            dsos,
            Some(dsos__find_kernel_dso_cb),
            &mut res as *mut _ as *mut c_void,
        );
        res
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dsos__for_each_dso(
    dsos: *mut dsos,
    cb: Option<unsafe extern "C" fn(*mut dso, *mut c_void) -> c_int>,
    data: *mut c_void,
) -> c_int {
    unsafe {
        let err: c_int;

        down_read(&mut (*dsos).lock);
        err = __dsos__for_each_dso(dsos, cb, data);
        up_read(&mut (*dsos).lock);
        err
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
