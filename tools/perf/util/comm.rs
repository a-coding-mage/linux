// SPDX-License-Identifier: GPL-2.0
//
// Source-level translation of perf/util/comm.c.
// C include dependencies intentionally remain external to this isolated file.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

pub type u64 = u64;
pub type size_t = usize;
pub type bool_ = bool;

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_once_t {
    _private: [u8; 0],
}

// DECLARE_RC_STRUCT(comm_str)
#[repr(C)]
pub struct comm_str {
    pub refcnt: refcount_t,
    pub str_: [c_char; 0],
}

#[repr(C)]
pub struct comm {
    pub start: u64,
    pub comm_str: *mut comm_str,
    pub exec: bool,
}

#[repr(C)]
struct comm_strs {
    lock: rw_semaphore,
    strs: *mut *mut comm_str,
    num_strs: c_int,
    capacity: c_int,
}

extern "C" {
    fn init_rwsem(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);

    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn refcount_inc_not_zero(r: *mut refcount_t) -> bool;
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn refcount_read(r: *const refcount_t) -> c_int;

    fn pthread_once(once_control: *mut pthread_once_t, init_routine: unsafe extern "C" fn());

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    ) -> *mut c_void;
}

const ENOMEM: c_int = 12;
const PTHREAD_ONCE_INIT: pthread_once_t = pthread_once_t { _private: [] };

static mut _comm_strs: comm_strs = comm_strs {
    lock: rw_semaphore { _private: [] },
    strs: ptr::null_mut(),
    num_strs: 0,
    capacity: 0,
};

static mut COMM_STRS_TYPE_ONCE: pthread_once_t = PTHREAD_ONCE_INIT;

unsafe extern "C" fn comm_strs__remove_if_last(cs: *mut comm_str);

// NO_THREAD_SAFETY_ANALYSIS: Inherently single threaded due to pthread_once.
unsafe extern "C" fn comm_strs__init() {
    init_rwsem(ptr::addr_of_mut!(_comm_strs.lock));
    _comm_strs.capacity = 16;
    _comm_strs.num_strs = 0;
    _comm_strs.strs = calloc(16, size_of::<*mut comm_str>()) as *mut *mut comm_str;
}

unsafe fn comm_strs__get() -> *mut comm_strs {
    pthread_once(ptr::addr_of_mut!(COMM_STRS_TYPE_ONCE), comm_strs__init);

    ptr::addr_of_mut!(_comm_strs)
}

unsafe fn comm_str__refcnt(cs: *mut comm_str) -> *mut refcount_t {
    // RC_CHK_ACCESS(cs)->refcnt
    ptr::addr_of_mut!((*cs).refcnt)
}

unsafe fn comm_str__str(cs: *const comm_str) -> *const c_char {
    // RC_CHK_ACCESS(cs)->str[0]
    (*cs).str_.as_ptr()
}

unsafe fn comm_str__get(cs: *mut comm_str) -> *mut comm_str {
    let result: *mut comm_str = cs;

    // RC_CHK_GET(result, cs)
    if !result.is_null() {
        refcount_inc_not_zero(comm_str__refcnt(cs));
    }

    result
}

unsafe fn comm_str__put(cs: *mut comm_str) {
    if cs.is_null() {
        return;
    }

    if refcount_dec_and_test(comm_str__refcnt(cs)) {
        // RC_CHK_FREE(cs)
        free(cs as *mut c_void);
    } else {
        if refcount_read(comm_str__refcnt(cs)) == 1 {
            comm_strs__remove_if_last(cs);
        }

        // RC_CHK_PUT(cs)
    }
}

unsafe fn comm_str__new(str_: *const c_char) -> *mut comm_str {
    let mut result: *mut comm_str = ptr::null_mut();
    // RC_STRUCT(comm_str) *cs;
    let cs = malloc(size_of::<comm_str>() + strlen(str_) + 1) as *mut comm_str;

    // ADD_RC_CHK(result, cs)
    if !cs.is_null() {
        result = cs;
        refcount_set(comm_str__refcnt(result), 1);
        strcpy((*cs).str_.as_mut_ptr(), str_);
    }
    result
}

unsafe extern "C" fn comm_str__search(_key: *const c_void, _member: *const c_void) -> c_int {
    let key = _key as *const c_char;
    let member = *(_member as *const *const comm_str);

    strcmp(key, comm_str__str(member))
}

unsafe extern "C" fn comm_strs__remove_if_last(cs: *mut comm_str) {
    let comm_strs = comm_strs__get();

    down_write(ptr::addr_of_mut!((*comm_strs).lock));
    /*
     * Are there only references from the array, if so remove the array
     * reference under the write lock so that we don't race with findnew.
     */
    if refcount_read(comm_str__refcnt(cs)) == 1 {
        let entry = bsearch(
            comm_str__str(cs) as *const c_void,
            (*comm_strs).strs as *const c_void,
            (*comm_strs).num_strs as size_t,
            size_of::<*mut comm_str>(),
            comm_str__search,
        ) as *mut *mut comm_str;
        comm_str__put(*entry);
        let mut i = entry.offset_from((*comm_strs).strs) as c_int;
        while i < (*comm_strs).num_strs - 1 {
            *(*comm_strs).strs.offset(i as isize) = *(*comm_strs).strs.offset((i + 1) as isize);
            i += 1;
        }
        (*comm_strs).num_strs -= 1;
    }
    up_write(ptr::addr_of_mut!((*comm_strs).lock));
}

// SHARED_LOCKS_REQUIRED(comm_strs->lock)
unsafe fn __comm_strs__find(comm_strs: *mut comm_strs, str_: *const c_char) -> *mut comm_str {
    let result = bsearch(
        str_ as *const c_void,
        (*comm_strs).strs as *const c_void,
        (*comm_strs).num_strs as size_t,
        size_of::<*mut comm_str>(),
        comm_str__search,
    ) as *mut *mut comm_str;

    if result.is_null() {
        return ptr::null_mut();
    }

    comm_str__get(*result)
}

unsafe fn comm_strs__findnew(str_: *const c_char) -> *mut comm_str {
    let comm_strs = comm_strs__get();
    let mut result: *mut comm_str;

    if comm_strs.is_null() {
        return ptr::null_mut();
    }

    down_read(ptr::addr_of_mut!((*comm_strs).lock));
    result = __comm_strs__find(comm_strs, str_);
    up_read(ptr::addr_of_mut!((*comm_strs).lock));
    if !result.is_null() {
        return result;
    }

    down_write(ptr::addr_of_mut!((*comm_strs).lock));
    result = __comm_strs__find(comm_strs, str_);
    if result.is_null() {
        if (*comm_strs).num_strs == (*comm_strs).capacity {
            let tmp = reallocarray(
                (*comm_strs).strs as *mut c_void,
                ((*comm_strs).capacity + 16) as size_t,
                size_of::<*mut comm_str>(),
            ) as *mut *mut comm_str;
            if tmp.is_null() {
                up_write(ptr::addr_of_mut!((*comm_strs).lock));
                return ptr::null_mut();
            }
            (*comm_strs).strs = tmp;
            (*comm_strs).capacity += 16;
        }
        result = comm_str__new(str_);
        if !result.is_null() {
            let mut low: c_int = 0;
            let mut high: c_int = (*comm_strs).num_strs - 1;
            let mut insert: c_int = (*comm_strs).num_strs; /* Default to inserting at the end. */

            while low <= high {
                let mid: c_int = low + (high - low) / 2;
                let cmp = strcmp(comm_str__str(*(*comm_strs).strs.offset(mid as isize)), str_);

                if cmp < 0 {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                    insert = mid;
                }
            }
            memmove(
                (*comm_strs).strs.offset((insert + 1) as isize) as *mut c_void,
                (*comm_strs).strs.offset(insert as isize) as *const c_void,
                (((*comm_strs).num_strs - insert) as size_t) * size_of::<*mut comm_str>(),
            );
            (*comm_strs).num_strs += 1;
            *(*comm_strs).strs.offset(insert as isize) = result;
        }
    }
    up_write(ptr::addr_of_mut!((*comm_strs).lock));
    comm_str__get(result)
}

#[no_mangle]
pub unsafe extern "C" fn comm__new(
    str_: *const c_char,
    timestamp: u64,
    exec: bool,
) -> *mut comm {
    let comm = calloc(1, size_of::<comm>()) as *mut comm;

    if comm.is_null() {
        return ptr::null_mut();
    }

    (*comm).start = timestamp;
    (*comm).exec = exec;

    (*comm).comm_str = comm_strs__findnew(str_);
    if (*comm).comm_str.is_null() {
        free(comm as *mut c_void);
        return ptr::null_mut();
    }

    comm
}

#[no_mangle]
pub unsafe extern "C" fn comm__override(
    comm: *mut comm,
    str_: *const c_char,
    timestamp: u64,
    exec: bool,
) -> c_int {
    let old = (*comm).comm_str;

    let new = comm_strs__findnew(str_);
    if new.is_null() {
        return -ENOMEM;
    }

    comm_str__put(old);
    (*comm).comm_str = new;
    (*comm).start = timestamp;
    if exec {
        (*comm).exec = true;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn comm__free(comm: *mut comm) {
    comm_str__put((*comm).comm_str);
    free(comm as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn comm__str(comm: *const comm) -> *const c_char {
    comm_str__str((*comm).comm_str)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
