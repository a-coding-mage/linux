/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A hash table (hashtab) maintains associations between
 * key values and datum values.  The type of the key values
 * and the type of the datum values is arbitrary.  The
 * functions for hash computation and key comparison are
 * provided by the creator of the table.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/* Depends on linux/types.h, linux/errno.h, and linux/sched.h. */

use core::ffi::c_void;

pub const HASHTAB_MAX_NODES: u32 = u32::MAX;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashtab_key_params {
    pub hash: Option<unsafe extern "C" fn(key: *const c_void) -> u32>, /* hash func */
    pub cmp: Option<
        unsafe extern "C" fn(key1: *const c_void, key2: *const c_void) -> core::ffi::c_int,
    >, /* comparison func */
}

#[repr(C)]
pub struct hashtab_node {
    pub key: *mut c_void,
    pub datum: *mut c_void,
    pub next: *mut hashtab_node,
}

#[repr(C)]
pub struct hashtab {
    pub htable: *mut *mut hashtab_node, /* hash table */
    pub size: u32,                      /* number of slots in hash table */
    pub nel: u32,                       /* number of elements in hash table */
}

#[repr(C)]
pub struct hashtab_info {
    pub slots_used: u32,
    pub max_chain_len: u32,
    pub chain2_len_sum: u64,
}

unsafe extern "C" {
    pub static EINVAL: core::ffi::c_int;
    pub static EEXIST: core::ffi::c_int;

    pub fn cond_resched();

    /*
     * Initializes a new hash table with the specified characteristics.
     *
     * Returns -ENOMEM if insufficient space is available or 0 otherwise.
     */
    pub fn hashtab_init(h: *mut hashtab, nel_hint: u32) -> core::ffi::c_int;

    pub fn __hashtab_insert(
        h: *mut hashtab,
        dst: *mut *mut hashtab_node,
        key: *mut c_void,
        datum: *mut c_void,
    ) -> core::ffi::c_int;

    /*
     * Destroys the specified hash table.
     */
    pub fn hashtab_destroy(h: *mut hashtab);

    /*
     * Applies the specified apply function to (key,datum,args)
     * for each entry in the specified hash table.
     *
     * The order in which the function is applied to the entries
     * is dependent upon the internal structure of the hash table.
     *
     * If apply returns a non-zero status, then hashtab_map will cease
     * iterating through the hash table and will propagate the error
     * return to its caller.
     */
    pub fn hashtab_map(
        h: *mut hashtab,
        apply: Option<
            unsafe extern "C" fn(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> core::ffi::c_int,
        >,
        args: *mut c_void,
    ) -> core::ffi::c_int;

    pub fn hashtab_duplicate(
        new: *mut hashtab,
        orig: *const hashtab,
        copy: Option<
            unsafe extern "C" fn(
                new: *mut hashtab_node,
                orig: *const hashtab_node,
                args: *mut c_void,
            ) -> core::ffi::c_int,
        >,
        destroy: Option<
            unsafe extern "C" fn(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> core::ffi::c_int,
        >,
        args: *mut c_void,
    ) -> core::ffi::c_int;
}

/*
 * Inserts the specified (key, datum) pair into the specified hash table.
 *
 * Returns -ENOMEM on memory allocation error,
 * -EEXIST if there is already an entry with the same key,
 * -EINVAL for general errors or
  0 otherwise.
 */
#[inline]
pub unsafe fn hashtab_insert(
    h: *mut hashtab,
    key: *mut c_void,
    datum: *mut c_void,
    key_params: hashtab_key_params,
) -> core::ffi::c_int {
    let hvalue: u32;
    let mut prev: *mut hashtab_node;
    let mut cur: *mut hashtab_node;

    unsafe {
        cond_resched();

        if (*h).size == 0 || (*h).nel == HASHTAB_MAX_NODES {
            return -EINVAL;
        }

        hvalue = key_params.hash.unwrap()(key as *const c_void) & ((*h).size - 1);
        prev = core::ptr::null_mut();
        cur = *(*h).htable.add(hvalue as usize);
        while !cur.is_null() {
            let cmp = key_params.cmp.unwrap()(key as *const c_void, (*cur).key as *const c_void);

            if cmp == 0 {
                return -EEXIST;
            }
            if cmp < 0 {
                break;
            }
            prev = cur;
            cur = (*cur).next;
        }

        __hashtab_insert(
            h,
            if !prev.is_null() {
                &mut (*prev).next
            } else {
                (*h).htable.add(hvalue as usize)
            },
            key,
            datum,
        )
    }
}

/*
 * Searches for the entry with the specified key in the hash table.
 *
 * Returns NULL if no entry has the specified key or
 * the datum of the entry otherwise.
 */
#[inline]
pub unsafe fn hashtab_search(
    h: *const hashtab,
    key: *const c_void,
    key_params: hashtab_key_params,
) -> *mut c_void {
    let hvalue: u32;
    let mut cur: *const hashtab_node;

    unsafe {
        if (*h).size == 0 {
            return core::ptr::null_mut();
        }

        hvalue = key_params.hash.unwrap()(key) & ((*h).size - 1);
        cur = *(*h).htable.add(hvalue as usize);
        while !cur.is_null() {
            let cmp = key_params.cmp.unwrap()(key, (*cur).key as *const c_void);

            if cmp == 0 {
                return (*cur).datum;
            }
            if cmp < 0 {
                break;
            }
            cur = (*cur).next;
        }
        core::ptr::null_mut()
    }
}

/* CONFIG_SECURITY_SELINUX_DEBUG:
 * Fill info with some hash table statistics.
 */
#[cfg(CONFIG_SECURITY_SELINUX_DEBUG)]
unsafe extern "C" {
    pub fn hashtab_stat(h: *mut hashtab, info: *mut hashtab_info);
}

#[cfg(not(CONFIG_SECURITY_SELINUX_DEBUG))]
#[inline]
pub unsafe fn hashtab_stat(_h: *mut hashtab, _info: *mut hashtab_info) {
    return;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
